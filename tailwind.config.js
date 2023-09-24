/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{astro,html,md,mdx,ts}"],
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
  darkMode: ["class", '[data-mode="dark"]'],
};
