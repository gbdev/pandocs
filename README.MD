# Pan Docs

"Pan Docs" is a document started in early 1995, considered the single most comprehensive technical reference to Game Boy available to the public.

This git repository hosts a renewed version of it, mantained in the Markdown format and enjoying renewed community attention.

Go to [gbdev.io/pandocs](https://gbdev.io/pandocs) to read the document.

## Contributing 

Everyone is welcome to contribute opening issues, expressing feedback, adding and improving content or share new findings.

Here are some resources you should take a look at:

- [The wiki](https://github.com/gbdev/pandocs/wiki), containing various rules and contribution guidelines you will have to follow when proposing changes. Some RFCs and discussions detailing the document scope are linked there as well.
- [The project board](https://github.com/gbdev/pandocs/projects/1), for a general picture of the roadmap and the ongoing efforts.
- [The issues](https://github.com/gbdev/pandocs/issues), where we discuss what needs to be worked on, and how. If you are looking into investigating Game Boy unknown behaviours, check the Issues labeled with ["research"](https://github.com/gbdev/pandocs/issues?q=is%3Aissue+is%3Aopen+label%3Aresearch).
- [The various community channels](https://gbdev.io/chat.html) where we you can chat directly with maintainers and other contributors.

Once you feel comfortable, fork this repository, make your modifications, and [send a Pull Request](https://docs.github.com/en/github/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests).

### Deploy

To deploy Pan Docs locally:

1. Install [Rust](https://www.rust-lang.org/tools/install), [mdBook](https://github.com/rust-lang/mdBook#readme), and [Python 3](https://www.python.org/downloads).
  mdBook is the tool rendering the documentation, Rust is used for some custom plugins and Python scripts are used to render some images. E.g.:
  ```sh
  # Install Rust using rustup
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  # Install mdbook using cargo
  cargo install mdbook
  # Remember to add cargo bin directory to your path
  # Install python3 (on Debian systems)
  apt install python3
  ```
2. [Install the Python prerequisites](https://packaging.python.org/tutorials/installing-packages/#requirements-files).
```sh
# Create and activate a virtualenv
python -m venv env
source env/bin/activate
# Install python dependencies
pip install -r requirements.txt
# Be sure to keep this virtual env activated for the following steps
```
3. Clone this repository and run `mdBook` in the root folder with one of:
```bash
# Produce a build
mdbook build
# Watch your files and trigger a build automatically whenever you modify a file.
mdbook watch
# Watches the book's src directory for changes, rebuild the book, serve it on localhost:3000
#  and refresh clients for each change.
mdbook serve
```
4. The final HTML files are in `docs/pandocs/`.

Be aware of the following caveats:

- `docs/html/` contains only partially processed files and it's also the folder that gets served by `mdbook serve`, so you will see some unprocessed custom markup if you visit the endpoint exposed by mdbook's development web server (:3000).

As a workaround, you can manually serve the file in the `docs/pandocs/` with the web server of your choice (e.g. you can run `python3 -m http.server` from the `docs/pandocs` folder).

- `mdbook watch` and `mdbook serve` do *not* watch for changes to files in the `theme/` or `custom/` directories (e.g. highlight.js builds, CSS style overrides). You must trigger the build by either restarting the command, or manually changing one of the watched files.

### Special markup

Pan Docs uses a custom mdBook preprocessor & renderer to enable some special markup:

#### Custom Containers 

Those mimick Vuepress' [custom containers](https://vuepress.vuejs.org/guide/markdown.html#custom-containers) functionality.

```markdown
  ::: type HEADING

  Content

  :::
```

These are rendered as "info boxes".

- `type` must be `tip`, or `warning`.
- `HEADING` can contain spaces and will be the title of the box. It can be "Warning", "Tip" or something illustrative of the phenomenon described in the box.
- Both `:::` lines **must** be surrounded by empty lines, otherwise they won't be processed.

E.g.

```markdown
::: tip SCOPE

The information here is targeted at homebrew development.
Emulator developers may be also interested in the [Game Boy: Complete Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf) document.

:::
```

will render as

<img src=".github/example_container.png"></img>

#### Internal links

```markdown
[VRAM Sprite Attribute Table (OAM)](<#VRAM Sprite Attribute Table (OAM)>)
```

Since Pan Docs contains a lot of internal references, and getting the actual anchor is somewhat tedious, internal links are given special treatment. Links whose URL simply begins with a hash are eligible; the rest of the (pseudo-)URL is treated as a section name (as plain text), and the link made to point to that section.

Note that the angle brackets [are only required if there are spaces in the URL](https://spec.commonmark.org/0.29/#example-485).

In effect, this means that linking to a section is as simple as copy-pasting its name in the URL field, prepending a `#`, and wrapping everything in `<>` if the name contains a space.

### Syntax highlighting

Syntax highlighting is provided within the browser, courtesy of [`highlight.js`](https://github.com/highlightjs/highlight.js).
[RGBASM syntax](https://rgbds.gbdev.io/docs/rgbasm.5) is highlighted via [a plugin](https://github.com/gbdev/highlightjs-rgbasm), but this requires a custom build of `highlight.js`.

Steps:

1. [Clone](https://docs.github.com/en/github/getting-started-with-github/getting-started-with-git/about-remote-repositories) `highlight.js` anywhere, and go into that directory.

   You will probably want to target a specific version by checking out its tag.
2. Run `npm install` to install its dependencies.
3. Within the `extras/` directory, clone `highlightjs-rgbasm`; ensure the directory is called `rgbasm`, otherwise the build tool won't pick it up.
4. You can work on and make modifications to `highlightjs-rgbasm`!
5. To make the custom build of `highlight.js`, within the `highlight.js` directory, run `node tools/build.js -t browser <languages>...`, with `<languages>...` being the list of languages to enable support for.
  The languages identifiers are the same that you would use for highlighting (` ```rgbasm `, for example).
6. Copy `build/highlight.min.js` as `theme/highlight.js` in Pan Docs' source.
  Alternatively, for debugging, you can use `build/highlight.js` for a non-minified version, but please don't commit that.

E.g.

```bash
git clone git@github.com:highlightjs/highlight.js.git
cd highlight.js
git checkout 10.7.2
npm install
git clone git@github.com:gbdev/highlightjs-rgbasm.git extra/rgbasm
node tools/build.js -t browser rgbasm c
cp build/highlight.min.js ../pandocs/theme/highlight.js
```

### Folder structure

```
.
├── .github/
│   ├── worflows/
│   │   └── ...
│   └── ...
├── custom/
│   └── ...
├── historical/
│   └── ...
├── mediawiki-exporter/
│   └── ...
├── preproc/
│   └── ...
├── renderer/
│   └── ...
├── src/
│   ├── imgs/
│   │   └── ...
│   ├── SUMMARY.md
│   ├── *.md
│   └── ...
├── theme/
│   └── ...
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.MD
├── book.toml
└── requirements.txt
```

- `.github/` - Metadata files related to the GitHub repository.
  - `workflows/` - [CI workflow description](https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions) files.
- `custom/` - Custom files added to the build.
- `historical/` - Archive of legacy Pan Docs versions.
- `mediawiki-exporter/` - A script (and support files) to generate this repo's Git history from a MediaWiki export.
- `preproc/`, `renderer/` - Our custom mdBook [preprocessor](https://rust-lang.github.io/mdBook/for_developers/preprocessors) and [back-end](https://rust-lang.github.io/mdBook/for_developers/backends), respectively. Both are standard Rust project folders (though see `Cargo.toml` below).
- `src/` - Markdown text sources for the document. **You are probably interested in this folder.**
  - `imgs/` - Images should go in this folder, for organization purposes.
  - Any `.md` file mentioned in `SUMMARY.md` will be rendered to HTML to the output directory.
  - All other files are output verbatim, at the same relative location to `src/` (so, for example, images will be output in `docs/custom/imgs/`).
- `theme/` - Files overriding [mdBook's default `theme/` files](https://github.com/rust-lang/mdBook/tree/master/src/theme).
- `Cargo.lock`, `Cargo.toml` - Since `preproc/` and `renderer/` share most dependencies (transitively through `mdbook`), this folder is set up as a Cargo workspace. This creates a single `target/` directory in the repo's root, containing both crates' dependencies.
- `book.toml` - The [mdBook configuration file](https://rust-lang.github.io/mdBook/format/configuration).
- `requirements.txt` - The Python package requirements; [see above](#contributing).

## License

We assume the content to be in the [public domain](LICENSE).
