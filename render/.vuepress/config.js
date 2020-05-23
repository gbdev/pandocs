module.exports = {
  "title": "Pan Docs",
  // We're serving this on gbdev.github.io/pandocs
  base: "/pandocs/",
  plugins: [
    ['git-log', {
      onlyFirstAndLastCommit: true,
      extendGitLog: (git) => {
        const spawn = require("cross-spawn").sync

        const res = spawn("git", ["log", "-1", "--format=%H/%h/%ci", "HEAD"])
        if (res.stderr.toString().trim()) return

        const [hash, shorthash, timestamp ] = res.stdout.toString().trim().split("/")

        git.HEAD = {
          hash,
          shorthash,
          timestamp,
        }
      },
    }],
  ],
  themeConfig: {
    navbar: false
    /*
    sidebar: [
      '/',
      'Timer_Obscure_Behaviour'
    ]
    */
  }
}
