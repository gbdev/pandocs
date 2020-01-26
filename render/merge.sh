echo "Producing the merged markdown file.."
cd ../content
markdown-pp index.mdpp -o ../render/index.md -e latexrender
cd -

