const defaultTheme = require("tailwindcss/defaultTheme");

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./web/**/*.{hbs,html,js}"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
}
