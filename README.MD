# Pan Docs

> ### The single, most comprehensive technical reference to Game Boy available to the public.


**WIP NOTICE**: This is a community effort to give this document a new form. Everything is still work in progress. You're welcome to [contribute](#contributing).

## History

Pan Docs - also known as `GAMEBOY.TXT` or `GBSPEC.TXT` - is an old document dating back to early 1995 originally written by *Pan of Anthrox*. It has been one of the most important references for Game Boy hackers, emulators and homebrew developers during the last 25 years.

<p align="center">
<img src="historical/1995-Jan-28-ATX-GBI/ADDRESS1.png">
<br>
  <i>ADDRESS1.PCX, one of the diagrams attached to the first version, released January 28th, 1995</i>
</p>

After its release (1995-2008), it received a number of revisions, corrections and updates, maintaining its **TXT format**. This [folder](historical/) provides a historical archive of those versions.

In 2008, a **wikified** version (using Martin Korth's 2001 revision as a baseline) has been published. The document was split into different articles and it continued being maintained and updated in that form.

In 2020, we want to make this repository the new home of this resource, where it can receive new public discussions and contributions, maintain its legacy and historical relevance, while making use of recent tools to be visualized and distributed.

Everything is in plain Markdown and easily editable by anyone with a GitHub account by clicking the "Edit" button on the web UI. We accept contributions, patches, suggestions and feedback via email, too (`pandocs (at) gbdev.io`).

## Contributing 

[This](https://github.com/gbdev/awesome-gbdev/issues/153) is the RFC proposing the change. You're welcome to discuss the development of this project in our various community channels found [here](https://gbdev.io/chat.html).

Contributing is really easy, fork this repo and edit the files in the **src** directory. Then, you can send your PR.

To deploy Pandocs locally:

1. Install [Rust](https://www.rust-lang.org/tools/install), [mdBook](https://github.com/rust-lang/mdBook#readme), and [Python 3](https://www.python.org/downloads).
  mdBook powers the book itself, Rust is used for some custom plugins, and Python for some image generation.
2. [Install](https://packaging.python.org/tutorials/installing-packages/#requirements-files) the Python prerequisites (`pip3 install -r requirements.txt`)
3. Within a terminal pointed at the directory `book.toml` is in, run mdBook (`mdbook build` / `mdbook watch` / `mdbook serve`).
4. The HTML files are in `docs/pandocs/`.

  ⚠️ `docs/html/` contains only partially processed files.
  This folder is what gets served when running `mdbook serve`, so you may see some custom markup missing if using that.
  As a workaround, you can manually open the files in the `docs/pandocs/` folder in your browser, they just won't auto-refresh on changes.

### Special markup

Pan Docs uses a custom mdBook preprocessor & renderer to enable some special markup:

- ```markdown
  ::: type HEADING

  Content

  :::
  ```

  These are rendered as "info boxes".
  `type` must be `tip`, or `warning`.
  `HEADING` can contain spaces, and be omitted entirely.

  ⚠️ Both `:::` lines **must** be surrounded by empty lines, otherwise they won't be processed!

- ```markdown
  [VRAM Sprite Attribute Table (OAM)](<#VRAM Sprite Attribute Table (OAM)>)
  ```

  Since Pan Docs contains a lot of internal references, and getting the actual anchor is somewhat tedious, internal links are given special treatment.
  Links whose URL simply begins with a hash are eligible; the rest of the (pseudo-)URL is treated as a section name (as plain text), and the link made to point to that section.

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

  ⚠️ `mdbook watch` and `mdbook serve` do *not* watch for changes to files in the `theme/` directory, you must trigger the build by either restarting the command, or manually changing one of the watched files.

Example:

```console
$ git clone git@github.com:highlightjs/highlight.js.git
$ cd highlight.js
$ git checkout 10.7.2
$ npm install
$ git clone git@github.com:gbdev/highlightjs-rgbasm.git extras/rgbasm
$ node tools/build.js -t browser rgbasm c
$ cp build/highlight.min.js ../pandocs/theme/highlight.js
```

## License

We assume the content to be in the [public domain](LICENSE).
