/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx,css}"],
  theme: {
    extend: {
      colors: {
        primary: "#f2f2f2",
        secondary: "#c6c7de",
      },
      boxShadow: {
        s1: "0px 0px 30px 0px rgba(0,0,0,0.1)",
      },
    },
  },
  plugins: [],
  darkMode: ["class", '[data-mode="dark"]'],
};
