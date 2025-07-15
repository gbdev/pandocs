import xmltodict
import pypandoc
import os
import subprocess
import sys
import getopt


def main(argv):
    try:
        # -i requires a value, -f is just a flag
        opts, args = getopt.getopt(sys.argv[1:], "i:f")
    except getopt.GetoptError as err:
        # print help information and exit:
        print(err)  # will print something like "option -a not recognized"
        usage()
        sys.exit(2)
    fake = False
    input_file = ""
    if "-i" not in opts[0]:
        print("-i <filename> is required")
        usage()
        sys.exit(2)
    for option, value in opts:
        if option == "-i":
            input_file = value
        elif option == "-f":
            print("Running in fake mode - git repo won't be touched")
            fake = True
    parseExport(input_file,fake)


def usage():
    print("Usage: python parseExport -i <filename> [-f]")
    print("-f simulates the execution")


def parseExport(input_file, fake=False):
    with open(input_file) as fd:
        doc = xmltodict.parse(fd.read())

    print(len(doc["mediawiki"]["page"]))
    if (len(doc["mediawiki"]["page"]) == 1):
        pages = [doc["mediawiki"]["page"]]
    else:
        pages = doc["mediawiki"]["page"]
    for page in pages:
        print("\n\n##### Parsing page", page["title"])
        print(len(page["revision"]), "revisions found\n")
        # if len(page["revision"] == 1):
    #		revisions = [page["revision"]]
    #	else:
    #		revisions = page["revision"]
        revisions = page["revision"]
        for revision in revisions:
            print("\n")
            #print(revision)
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
            # print(revision["text"]["#text"])

            # x-wiki/mediawiki to markdown
            s = pypandoc.convert_text(
                revision["text"]["#text"], 'md', format='mediawiki')

            # Create the markdown file
            filename = page["title"].replace(" ", "_") + ".md"
            with open("../content/" + filename, "w") as text_file:
                print("{}".format(s), file=text_file)

            # Forge the commit

            # Copy the current environment
            d = dict(os.environ)
            # Set the date to the revision timestamp we are committing
            d['GIT_COMMITTER_DATE'] = revision["timestamp"]
            d['GIT_AUTHOR_DATE'] = revision["timestamp"]

            # git add the file
            gitcmd = ["git", "add", "../content/" + filename]
            if not fake:
                subprocess.call(gitcmd)
            print(gitcmd)

            # set the author email
            author = revision["contributor"]["username"]

            #	 Source: https://api.github.com/users/USERNAME/events/public
            emails = {'ISSOtm': 'eldredhabert0@gmail.com',
                      'endrift': 'vi@endrift.com',
                      'LIJI32': 'LIJI32@gmail.com',
                      'mattcurrie': 'me@mattcurrie.com',
                      'AntonioND': 'antonio_nd@outlook.com',
                      'PinoBatch': 'git@pineight.com',
                      'Nitro2k01': 'nitro2k01@gmail.com',
                      'T4g1': 'thomasvangysegem@gmail.com',
                              'Elizafox': 'elizabeth@interlinked.me',
                              'Furrtek': 'furrtek@gmail.com'}

            # Different username spellings
            usernames = {'LIJI': 'LIJI32',
                         'Endrift': 'endrift',
                         'Mattcurrie': 'mattcurrie'}

            if author in usernames:
                author = usernames[author]

            if author in emails:
                authoremail = emails[author]
            else:
                authoremail = ''

            # print("###", author, authoremail)
            # prepare the --author=username <email> to pass to git commit
            authorstring = "--author='" + author + " <" + authoremail + ">'"
            # assemble the git commit command, using :
            # the wiki comment as commit message
            # the wiki contributor as commit author
            # the revision timestamp as commit date/time
            gitcmd = ["git", "commit", authorstring, "-m", commitmessage]
            # execute git commit with the modified environment
            if not fake:
                subprocess.call(gitcmd, env=d)
            print(gitcmd)

if __name__ == "__main__":
    main(sys.argv[1:])
