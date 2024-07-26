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
        background: {
          DEFAULT: "rgb(var(--color-background) / <alpha-value>)",
          dark: "rgb(var(--color-background-dark) / <alpha-value>)",
          light: "rgb(var(--color-background-light) / <alpha-value>)",
        },
        primary: "rgb(var(--color-primary) / <alpha-value>)",
        secondary: "rgb(var(--color-secondary) / <alpha-value>)",
      },
    },
  },
  plugins: [],
};
