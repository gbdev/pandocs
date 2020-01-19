import xmltodict

with open('GbdevWiki-20200119114605.xml') as fd:
    doc = xmltodict.parse(fd.read())

# Skip the first page, (the Template)
for page in doc["mediawiki"]["page"][1:]:
	print("## Parsing page",page["title"])
	print(len(page["revision"]), "revisions found")
	for revision in page["revision"]:
		print("ID", revision["id"])
		# Timestamp
		print("Timestamp", revision["timestamp"])
		# Commit message
		if "comment" in revision.keys():
			print("Comment", revision["comment"])
		# Author
		print("Contributor", revision["contributor"]["username"])
		# x-wiki source
		print(revision["text"]["#text"])
		
		# x-wiki to markdown
		# TODO
		# create the markdown file
		# TODO
		# Forge the git commit
		# TODO