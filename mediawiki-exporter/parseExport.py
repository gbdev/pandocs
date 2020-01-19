import xmltodict
import pypandoc
import os, subprocess

with open('GbdevWiki-20200119114605.xml') as fd:
    doc = xmltodict.parse(fd.read())

# Skip the first page, (the Template)
for page in doc["mediawiki"]["page"][1:]:
	print("## Parsing page",page["title"])
	print(len(page["revision"]), "revisions found")
	for revision in page["revision"]:
		print("\n\n\n")
		print("ID", revision["id"])
		
		# Timestamp
		print("Timestamp", revision["timestamp"])
		# Commit message
		if "comment" in revision.keys():
			commitmessage = revision["comment"]
		else:
			commitmessage = "revision " + revision["timestamp"]

		print("Comment", commitmessage)
		# Author
		print("Contributor", revision["contributor"]["username"])
		# x-wiki source
		#print(revision["text"]["#text"])
		
		# x-wiki/mediawiki to markdown
		s = pypandoc.convert_text(revision["text"]["#text"], 'md', format='mediawiki')

		# Create the markdown file
		filename = page["title"].replace(" ", "_")+revision["id"]+".md"
		with open("../content/" + filename, "w") as text_file:
			print("{}".format(s), file=text_file)

		## Forge the commit

		# Copy the current environment
		d = dict(os.environ)
		# Set the date to the revision timestamp we are committing
		d['GIT_COMMITTER_DATE'] = revision["timestamp"]
		d['GIT_AUTHOR_DATE'] = revision["timestamp"]

		# git add the file
		#subprocess.call(["git","add", "../content/"+filename])
		print(["git","add", "../content/"+filename])
		# set the author email
		author = revision["contributor"]["username"]
		
	#	 Source: https://api.github.com/users/USERNAME/events/public
		emails = {'ISSOtm':'eldredhabert0@gmail.com',
				  'endrift':'vi@endrift.com',
				  'LIJI32': 'LIJI32@gmail.com',
				  'mattcurrie': 'me@mattcurrie.com',
				  'AntonioND': 'antonio_nd@outlook.com',
				  'PinoBatch': 'git@pineight.com',
				  'Nitro2k01': 'nitro2k01@gmail.com',
				  'T4g1':'thomasvangysegem@gmail.com',
				  'Elizafox':'elizabeth@interlinked.me'}
		
		# Different username spellings
		usernames = {'LIJI':'LIJI32',
					 'Endrift':'endrift',
					 'Mattcurrie':'mattcurrie'}

		if author in usernames:
			author = usernames[author]

		if author in emails:
			authoremail = emails[author]
		else:
			authoremail = ''

		#print("###", author, authoremail)
		# prepare the --author=username <email> to pass to git commit
		authorstring = "--author='"+author+" <" + authoremail + ">'"
		# assemble the git commit command, using :
		# the wiki comment as commit message
		# the wiki contributor as commit author
		# the revision timestamp as commit date/time
		gitcmd = ["git", "commit", authorstring, "-m", commitmessage]
		# execute git commit with the modified environment
		#subprocess.call(gitcmd, env=d)