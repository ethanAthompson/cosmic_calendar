/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ["*.html","src/**/*.rs"],
  theme: {
    screens: {
      'watch' : '320px',
      'phone' : '480px',
      'tablet': '640px',
      'laptop': '1024px',
      'desktop': '1280px',
    },
    extend: {},
  },
  plugins: [
  ],
}

