module.exports = {
  title: "Pan Docs",
  // We're serving this on gbdev.github.io/pandocs
  base: "/pandocs/",
  head: [
  // https://github.com/gbdev/pandocs/issues/38
    [
      "script",
      {},
      `
    window.addEventListener('load', function () {
        var isChrome = /Chrome/.test(navigator.userAgent) && /Google Inc/.test(navigator.vendor);
        if (window.location.hash && isChrome) {
          var hash = window.location.hash;
          window.location.hash = "";
          window.location.hash = hash;
        }
    });
  `
    ]
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
};
