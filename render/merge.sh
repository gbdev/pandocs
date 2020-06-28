commithash=$(git rev-parse HEAD)
shortcommithash=$(git rev-parse --short HEAD)
timestamp=$(git show -s --format=%ci HEAD)

cd ../content
cp About.md About2.md

echo "This document version was produced from git commit ["$shortcommithash"](https://github.com/gbdev/pandocs/tree/"$commithash") ("$timestamp")." >> About.md

echo "Producing the merged markdown file.."

markdown-pp index.mdpp -o ../render/index.md -e latexrender

echo "Restoring untemplated version of About.md"
mv About2.md About.md

cd -

echo "Copying single non-templated articles.."
cp ../content/Timer_Obscure_Behaviour.md . --verbose 

echo "Copying image assets.."
mkdir -p .vuepress/public/imgs/
cp -R ../content/imgs/* .vuepress/public/imgs/ --verbose
