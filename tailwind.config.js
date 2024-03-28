/** @type {import('tailwindcss').Config} */
const plugin = require("tailwindcss/plugin");

module.exports = {
  content: {
    files: ["*.html", "./src/main.rs", "./src/**/*.rs"],
  },
  theme: {
    container: {
      center: true,
    },

    // colors: {
    //     text: 'var(--clr-text)',
    //     bg: 'var(--clr-bg)',
    //     accent: 'var(--clr-accent)',
    //     success: 'var(--clr-success)',
    //     danger: 'var(--clr-danger)',
    // },
    extend: {},
  },
  plugins: [
    plugin(function ({ addVariant }) {
      // addVariant("hocus", ["&:hover", "&:focus"]);
      addVariant("hocus", ["&:is(:hover, :focus):not(disabled)"]);
    }),
  ],
};
