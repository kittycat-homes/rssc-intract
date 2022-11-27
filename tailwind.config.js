const defaultTheme = require("tailwindcss/defaultTheme")

/** @type {import('tailwindcss').Config} */
module.exports = {
        darkMode: "media",
        content: ["./src/web/components/**/*.rs"],
        theme: {
                extend: {},
        },
        plugins: [require("@tailwindcss/forms")],
}
