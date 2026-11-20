use std::{fs::File, ops::Range};

use ariadne::{Cache, Color, Config, Label, Report, ReportBuilder, ReportKind, Source, Span};

const CONFIG: Config = Config::new().with_index_type(ariadne::IndexType::Byte);
pub fn make_error<S: Span>(span: S, msg: impl ToString) -> ReportBuilder<'static, S> {
    Report::build(ReportKind::Error, span)
        .with_config(CONFIG)
        .with_message(msg)
}

pub fn error_label<S: Span>(span: S, msg: impl ToString) -> Label<S> {
    Label::new(span).with_color(Color::Red).with_message(msg)
}
pub fn secondary_label<S: Span>(span: S, msg: impl ToString) -> Label<S> {
    Label::new(span).with_color(Color::Blue).with_message(msg)
}

/// Helps render diagnostics about a file whose in-memory representation may not match its on-disk contents anymore.
pub struct FileDiags<'a> {
    file_name: &'a str,
    contents: Result<Source<&'a str>, &'a str>,
}
impl<'a> FileDiags<'a> {
    pub fn new(file_name: &'a str, contents: &'a str) -> Self {
        Self {
            file_name,
            contents: Err(contents),
        }
    }

    pub fn make_span(&self, byte_range: Range<usize>) -> (&'a str, Range<usize>) {
        (self.file_name, byte_range)
    }
    pub fn render(&mut self, mut diag: ReportBuilder<'_, (&str, Range<usize>)>) {
        let source = match self.contents {
            Ok(ref source) => source,
            Err(contents) => {
                self.contents = Ok(Source::from(contents));
                let source = self.contents.as_ref().unwrap();

                if let Some(first_neq_idx) = compare_against_disk(contents, self.file_name) {
                    let (_line, line_no, _col_no) = source.get_byte_line(first_neq_idx).unwrap();
                    diag.set_note(format!(
                        "Due to limitations, line numbers above {line_no} may be misreported",
                    ));
                }

                source
            }
        };

        diag.finish().eprint((self.file_name, source)).unwrap();
    }
}

// This function is meant to be run on chapter files, which should exist as regular files and everything;
// so, I/O errors are *not* expected here, and panicking is appropriate.
fn compare_against_disk(in_memory: &str, path: &str) -> Option<usize> {
    use std::io::Read;
    let mut file = File::open(path).expect("Chapter source not loadable!?");
    let mut buf = [0; 8 * 1024]; // Arbitrary batch size for fast I/O; aped from `std::io::DEFAULT_BUF_SIZE`.

    let mut nb_checked = 0;
    loop {
        let nb_to_check = match file.read(&mut buf) {
            Ok(0) => break, // Reached EOF.
            Ok(n) => n,
            Err(error) if matches!(error.kind(), std::io::ErrorKind::Interrupted) => continue, // Retry.
            Err(error) => panic!("Error while reading {path}: {error}"),
        };
        let from_disk = &buf[..nb_to_check];
        let from_memory = &in_memory.as_bytes()[nb_checked..nb_checked + nb_to_check];
        if let Some(mismatch_ofs) =
            std::iter::zip(from_disk, from_memory).position(|(&disk, &mem)| disk != mem)
        {
            return Some(nb_checked + mismatch_ofs);
        }
        nb_checked += nb_to_check;
    }
    None // Even if the on-disk file is longer than the in-memory one, we will never witness the modified parts.
}

#[derive(Debug, Clone)]
pub(crate) struct GlobalDiag(Source<&'static str>);
#[derive(Debug, Clone, Copy)]
pub(crate) struct DummySpan;
impl Span for DummySpan {
    type SourceId = ();
    fn source(&self) -> &Self::SourceId {
        &()
    }
    fn start(&self) -> usize {
        0
    }
    fn end(&self) -> usize {
        0
    }
}
impl Cache<()> for GlobalDiag {
    type Storage = &'static str;
    #[allow(refining_impl_trait)]
    fn fetch(&mut self, &(): &()) -> Result<&Source<Self::Storage>, !> {
        Ok(&self.0)
    }
    #[allow(refining_impl_trait)]
    fn display(&self, &(): &()) -> Option<!> {
        None
    }
}
impl Default for GlobalDiag {
    fn default() -> Self {
        Self(Source::from(""))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ChapterError {
    pub(crate) err_msg: String,
    pub(crate) label_byte_range: Range<usize>,
    pub(crate) label_msg: String,
}
pub(crate) type ChapterResult<T = ()> = Result<T, ChapterError>;
impl ChapterError {
    pub fn new(
        err_msg: impl ToString,
        label_byte_range: impl Into<Range<usize>>,
        label_msg: impl ToString,
    ) -> Self {
        Self {
            err_msg: err_msg.to_string(),
            label_byte_range: label_byte_range.into(),
            label_msg: label_msg.to_string(),
        }
    }
    pub fn substr_adjust(&mut self, string: &str, substring: &str) {
        #[cfg(debug_assertions)]
        substring
            .get(self.label_byte_range.clone())
            .expect("byte range does not match substring");

        let substr_range = string
            .as_bytes()
            .subslice_range(substring.as_bytes())
            .expect("Didn't pass a proper sub-string!");
        self.label_byte_range.start += substr_range.start;
        self.label_byte_range.end += substr_range.start;
    }
}
impl<EM: ToString, LBR: Into<Range<usize>>, LM: ToString> From<(EM, LBR, LM)> for ChapterError {
    fn from((err_msg, label_byte_range, label_msg): (EM, LBR, LM)) -> Self {
        Self::new(err_msg, label_byte_range, label_msg)
    }
}
pub fn substr_adjust<T>(res: ChapterResult<T>, string: &str, substring: &str) -> ChapterResult<T> {
    res.map_err(|mut error| {
        error.substr_adjust(string, substring);
        error
    })
}

#[macro_export]
macro_rules! bail {
    ($err_msg:expr, $label_byte_range:expr, $label_msg:expr) => {
        return Err($crate::diagnostics::ChapterError::from((
            $err_msg,
            $label_byte_range,
            $label_msg,
        )))
    };
}
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $tuple:tt) => {
        if !($cond) {
            $crate::bail! $tuple;
        }
    };
}
