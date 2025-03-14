/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
    "./.storybook/**/*.stories.@(js|jsx|ts|tsx|mdx)",
    "./public/index.html",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
