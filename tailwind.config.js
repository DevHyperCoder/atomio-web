/** @type {import('tailwindcss').Config} */
module.exports = {
content: [
    './src/**/*.rs',
  ],
  theme: {
    extend: {
        colors: {
            lightPurple: '#E4D8E9'
        }
    },
  },
  plugins: [require("@tailwindcss/forms")]
}
