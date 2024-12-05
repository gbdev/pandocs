# Deploy

This document will explain you how to set up a local copy of Pan Docs.

```sh
# Start by cloning the repository
git clone https://github.com/gbdev/pandocs.git
# and moving to the pandocs directory
cd pandocs
```

## Docker

If you have [Docker installed](https://docs.docker.com/engine/install/), you can pull and use the provided image by running:

```sh
docker run -p 8001:8000 \
  --mount "type=bind,source=$(pwd)/custom,target=/code/custom" \
  --mount "type=bind,source=$(pwd)/preproc,target=/code/preproc" \
  --mount "type=bind,source=$(pwd)/renderer,target=/code/renderer" \
  --mount "type=bind,source=$(pwd)/src,target=/code/src" \
  --mount "type=bind,source=$(pwd)/theme,target=/code/theme" \
  -it ghcr.io/gbdev/pandocs
```

That's it! Pan Docs is live at [localhost:8001](https://localhost:8001).

Be aware of the following caveat:

- The locally running site will not update from changes to files in the `theme/` or `custom/` directories (e.g. highlight.js builds, CSS style overrides). You must trigger the build by manually changing a file in the `src/` directory.

### Building the image

If you prefer to build the image yourself:

```sh
docker build -t pandocs .
```

## Local

If you prefer to install every dependency locally:

1. Install [Rust](https://www.rust-lang.org/tools/install), [mdBook](https://github.com/rust-lang/mdBook#readme), and [Python 3](https://www.python.org/downloads) (version 3.9 or newer).

  mdBook is the tool rendering the documentation, Rust is used for some custom plugins and Python scripts are used to render some images. E.g.:
  ```sh
  # Install Rust using rustup
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  # Install mdbook using cargo
  cargo install mdbook
  # Remember to add cargo bin directory to your path
  # Install Python; e.g. on Debian-based systems:
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

## Syntax highlighting

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

## Folder structure

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
├── README.md
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
