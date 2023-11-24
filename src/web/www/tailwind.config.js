/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./index.html",
        "./static/javascript.js",
        // Add other paths as necessary
    ],
    plugins: [require("daisyui")], // Add DaisyUI as a plugin


    daisyui: {
        themes: [
            {
                light: {
                    primary: "#de691b",
                    secondary: "#e8f4fc",
                    accent: "#e67a33",
                    neutral: "#072f45",
                    "base-100": "#e3f3fc",
                },
                dark: {
                    primary: "#e46f21",
                    secondary: "#030f17",
                    accent: "#cc6119",
                    neutral: "#082d45",
                    "base-100": "#03121b",
                },
            },
        ],
    },
};