/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        base: {
          DEFAULT: "rgb(var(--color-base) / <alpha-value>)",
          light: "rgb(var(--color-base-light) / <alpha-value>)",
        },
        background: "rgb(var(--color-background) / <alpha-value>)",
        primary: {
          DEFAULT: "rgb(var(--color-primary) / <alpha-value>)",
          light: "rgb(var(--color-primary-light) / <alpha-value>)",
        },
        secondary: "rgb(var(--color-secondary) / <alpha-value>)",
      },
    },
  },
  plugins: [],
};
