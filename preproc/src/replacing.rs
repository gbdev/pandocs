use std::ops::RangeBounds;

/// Allows replacing sub-ranges of a string in a single pass.
///
/// ```
/// # use pandocs_preproc::replacing::Replacer;
/// # use std::fmt::Write;
/// let mut replacer = Replacer::new("My tailor is rich.");
/// write!(replacer.replace(3..=8), "neighbour").unwrap();
/// write!(replacer.replace(13..17), "nice").unwrap();
/// assert_eq!(replacer.finish().as_deref(), Some("My neighbour is nice."));
/// ```
///
/// # Panics
///
/// Replace operations must be sorted, otherwise a panic will occur:
///
// Only doctests support checking for panics.
/// ```should_panic
/// # use pandocs_preproc::replacing::Replacer;
/// # use std::fmt::Write;
/// let mut replacer = Replacer::new("My tailor is rich.");
/// write!(replacer.replace(13..17), "nice").unwrap();
/// write!(replacer.replace(3..=8), "neighbour").unwrap(); // Panics
/// ```
#[must_use = "Call `.finish()` to obtain the replaced text"]
pub struct Replacer<'input> {
    replaced: Option<(usize, String)>,
    original: &'input str,
}

impl<'input> Replacer<'input> {
    pub fn new(original: &'input str) -> Self {
        Self {
            replaced: None,
            original,
        }
    }

    #[must_use = "Append to the returned string to write the replacing text"]
    pub fn replace(&mut self, range_in_original: impl RangeBounds<usize>) -> &mut String {
        use std::ops::Bound;

        let range_start = range_in_original.start_bound().map(|idx| *idx);
        let range_end = range_in_original.end_bound().map(|idx| *idx);
        self.original
            .get((range_start, range_end))
            .expect("invalid range within original string");
        let (unprocessed_start, replaced) = self
            .replaced
            // TODO: is this capacity a good estimate?
            .get_or_insert_with(|| (0, String::with_capacity(self.original.len())));

        let end_of_kept = match range_start {
            Bound::Unbounded => 0,
            Bound::Excluded(idx) => idx + 1,
            Bound::Included(idx) => idx,
        };
        replaced.push_str(&self.original[*unprocessed_start..end_of_kept]);
        *unprocessed_start = match range_end {
            Bound::Unbounded => self.original.len(),
            Bound::Excluded(idx) => idx,
            Bound::Included(idx) => idx + 1,
        };

        replaced
    }

    pub fn finish(self) -> Option<String> {
        self.replaced.map(|(unprocessed_start, mut replaced)| {
            replaced.push_str(&self.original[unprocessed_start..]);
            replaced
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Write;

    #[test]
    fn multiple_work() {
        let mut replacer = Replacer::new("My tailor is rich.");
        write!(replacer.replace(3..=8), "neighbour").unwrap();
        write!(replacer.replace(13..17), "nice").unwrap();
        assert_eq!(replacer.finish().as_deref(), Some("My neighbour is nice."));
    }

    #[test]
    fn replace_at_start() {
        let mut replacer = Replacer::new("Arch is the best!");
        write!(replacer.replace(..4), "Rust").unwrap();
        assert_eq!(replacer.finish().as_deref(), Some("Rust is the best!"));
    }

    #[test]
    fn insertion() {
        let mut replacer = Replacer::new("My tailor is rich.");
        write!(replacer.replace(13..13), "very ").unwrap();
        assert_eq!(
            replacer.finish().as_deref(),
            Some("My tailor is very rich."),
        );
    }

    #[test]
    fn deletion() {
        let mut replacer = Replacer::new("My tailor is very rich.");
        let _ = replacer.replace(13..18);
        assert_eq!(replacer.finish().as_deref(), Some("My tailor is rich."));
    }

    #[test]
    fn no_op() {
        let replacer = Replacer::new("My tailor is rich.");
        assert_eq!(replacer.finish(), None);
    }

    #[test]
    fn replace_everything() {
        let mut replacer = Replacer::new("My tailor is rich.");
        let _ = replacer.replace(..);
        assert_eq!(replacer.finish().as_deref(), Some(""));
    }

    #[test]
    fn insert_at_end() {
        let string = "My tailor is rich.";
        let mut replacer = Replacer::new(string);
        replacer.replace(string.len()..).push('.');
        replacer.replace(string.len()..string.len()).push('!');
        assert_eq!(replacer.finish().as_deref(), Some("My tailor is rich..!"));
    }

    #[test]
    fn insert_at_beginning() {
        let string = "My tailor is rich.";
        let mut replacer = Replacer::new(string);
        write!(replacer.replace(0..0), "Narrator").unwrap();
        write!(replacer.replace(..0), ": ").unwrap();
        assert_eq!(
            replacer.finish().as_deref(),
            Some("Narrator: My tailor is rich.")
        );
    }
}
