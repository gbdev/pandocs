# Undocumented SGB commands

The following information has been extracted from disassembling a SGB1v2
firmware; it should be verified on other SGB revisions.

The SGB firmware explicitly ignores all commands with ID >= $1E. This
leaves undocumented commands $1A to $1D inclusive.

## Stubbed commands

Commands $1A to $1F (inclusive)'s handlers are stubs (only contain an
`RTS`). This is interesting, since the command-processing function
explicitly ignores commands $1E and $1F.
