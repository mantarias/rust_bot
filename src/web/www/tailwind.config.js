/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./static/index.html",
        "./static/javascript.js",
        "./static/manageCommands.html",
        // Add other paths as necessary
    ],
    plugins: [require("daisyui")], // Add DaisyUI as a plugin

    daisyui: {
        themes: [
            "dark"
        ],
    },
};
