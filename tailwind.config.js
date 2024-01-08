/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    "./index.html",
    "./src/**/*.rs",
    "./src/**/*.html"
  ],
  theme: {
    screens: {
      'watch': '320px',
      'phone': '480px',
      'tablet': '640px',
      'laptop': '1024px',
      'desktop': '1280px',
    },
    extend: {
      fontFamily: {
        "rubik": ["Rubik Doodle Shadow"],
        "monda": ["Monda"]
      },
      colors: {
        accent: {
          1: "lch(var(--color-primary) / <alpha-value>)",
          2: "lch(var(--color-secondary) / <alpha-value>)",
        },
        content: {
          1: "lch(var(--color-non-highlight) / <alpha-value>)",
          2: "lch(var(--color-highlight) / <alpha-value>)",
        }
      }
    },
  },
  plugins: []
}

