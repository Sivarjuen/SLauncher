/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "jit",
    content: {
        files: ["*.html", "./css/**/*.css", "./src/**/*.rs"],
    },
    plugins: [require("daisyui")],
};
