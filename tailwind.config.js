/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ["./index.html","src/**/*.{rs,html}"],
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
