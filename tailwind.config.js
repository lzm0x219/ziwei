/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{astro,html,md,mdx,ts}"],
  theme: {
    extend: {
      colors: {
        primary: "#d1d1e9",
        secondary: "",
      },
      boxShadow: {
        s1: "0px 0px 30px -6px rgba(0, 0, 0, 0.15)",
      },
    },
  },
  darkMode: ["class", '[data-mode="dark"]'],
};
