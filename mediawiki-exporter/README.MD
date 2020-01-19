# mediawiki-to-markdown

Reconstructs the mediawiki history of every Pan Docs article, on git.

This script takes a XML Mediawiki export (including revisions) and for every revision of every page:

- Converts the document to Markdown, using `pandoc` producing a separate file for every article
- Forges a git commit using the author and the timestamp from the revision.
- Adds the email and reformats the username to match GitHub accounts
- git add and commit, reconstructing the history and keeping the correct contributors


```
python3 -m venv .
source bin/activate
pip3 install -r requirements.txt
python3 parseExport.py
```
