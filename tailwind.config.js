const defaultTheme = require("tailwindcss/defaultTheme")

/** @type {import('tailwindcss').Config} */
module.exports = {
        darkMode: "media",
        content: ["./src/web/components/**/*.rs"],
        theme: {
                fontFamily: {
                        atkinson: ["Atkinson", "sans-serif"],
                },
                extend: {},
        },
        plugins: [require("@tailwindcss/forms")],
}
