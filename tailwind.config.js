const defaultTheme = require("tailwindcss/defaultTheme")

/** @type {import('tailwindcss').Config} */
module.exports = {
        content: ["./src/web/components/**/*.rs"],
        theme: {
                fontFamily: {
                        atkinson: ["Atkinson", "sans-serif"],
                },
                extend: {},
        },
        plugins: [require("@tailwindcss/forms")],
}
