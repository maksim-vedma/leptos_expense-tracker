/** @type {import('tailwindcss').Config} */
const plugin = require("tailwindcss/plugin");

module.exports = {
  content: {
    files: ["*.html", "./src/main.rs", "./src/**/*.rs"],
  },
  theme: {
    container: {
      center: true,
      padding: "1rem",
    },

    // colors: {
    //     text: 'var(--clr-text)',
    //     bg: 'var(--clr-bg)',
    //     accent: 'var(--clr-accent)',
    //     success: 'var(--clr-success)',
    //     danger: 'var(--clr-danger)',
    // },
    //
    extend: {
      animation: {
        wiggle: "wiggle 1s ease-in-out infinite",
        ripple: "ripple 0.6s linear",
      },
      keyframes: {
        wiggle: {
          "0%, 100%": { transform: "rotate(-3deg)" },
          "50%": { transform: "rotate(3deg)" },
        },
        ripple: {
          "100%": { transform: "scale(4)", opacity: 0 },
        },
      },
    },
  },
  plugins: [
    plugin(function ({ addVariant }) {
      // addVariant("hocus", ["&:hover", "&:focus"]);
      addVariant("hocus", ["&:is(:hover, :focus):not(disabled)"]);
    }),
    plugin(function ({ addVariant }) {
      // addVariant("hocus", ["&:hover", "&:focus"]);
      addVariant("active-page", ["&[aria-current='page']"]);
    }),
  ],
};
