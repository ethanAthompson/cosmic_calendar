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
      boxShadow : {
        'pink': '0 1px 2px 0 rgba(255, 164, 211, 0.8)',
        'amber': '0 35px 60px 15px rgba(255, 164, 65, 0.8)',
      },
    },
  },
  plugins: []
}

