/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  plugins: [],
  safelist: [
    {
      pattern:
        /(from|to)-(lime|amber|cyan|teal|rose|fuchsia|red|yellow|indigo)-(200|300|400|500|600)/,
    },
  ],
  theme: {
    extend: {},
  },
};
