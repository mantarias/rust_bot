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
                    "primary": "#00dbe6",
                    "secondary": "#00a500",
                    "accent": "#efb500",
                    "neutral": "#141414",
                    "base-100": "#fef9ff",
                    "info": "#00abd6",
                    "success": "#00a85f",
                    "warning": "#ffaf00",
                    "error": "#d00014",
                },
                dark: {
                    "primary": "#f59e0b",
                    "secondary": "#facc15",
                    "accent": "#4ade80",
                    "neutral": "#3f2d29",
                    "base-100": "#342c23",
                    "info": "#7dd3fc",
                    "success": "#a3e635",
                    "warning": "#fc6000",
                    "error": "#f43f5e",
                },
            },
        ],
    },
};
