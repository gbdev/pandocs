# content

This folder contains the content source of the Pan Doc document.

Prerequisites:
```
npm install
pip3 install -r requirements.txt
```

To produce a single file and push it in the "render" branch:

```
markdown-pp index.mdpp -o render/index.md -e latexrender
npx gh-pages -d render -b render
```
