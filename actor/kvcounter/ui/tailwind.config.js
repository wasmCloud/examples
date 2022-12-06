/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{js,jsx,ts,tsx}', './public/index.html'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        wasmcloudGreen: {
          light: '#00bc8e',
          dark: '#007559'
        },
        wasmcloudGray: '#778591',
        goldenCream: 'rgba(246, 215, 116, 0.4)'
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
