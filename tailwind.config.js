const defaultTheme = require("tailwindcss/defaultTheme")

module.exports = {
  mode: "jit",
  purge: [
    "./index.html",
    "./src/**/*.rs",
  ],
  darkMode: false, // or "media" or "class"
  theme: {
    fontFamily: {
      mono: ["Proggy Clean", ...defaultTheme.fontFamily.mono],
    },
    colors: {
      black: "#000000",
      white: "#FFFFFF",
      default: {
        bg: "#0D0D0D",
        border: "#333333",
      },
      'menu-bar': "#242424",
      table: {
        odd: "#121212",
        even: "#1A1A1A",
      },
      popup: "#121212",
      me1: {
        bg: "#1C526E",
        default: "#296B94",
        active: "#478CAB",
        hover: "#D46E2B",
      },
      me2: {
        bg: "#A3521F",
        default: "#B35E29",
        active: "#D97D40",
        hover: "#38853B",
      },
      me3: {
        bg: "#660000",
        default: "#870000",
        active: "#B30000",
        hover: "#05476E",
      },
      theme: {
        bg: "var(--bg)",
        default: "var(--default)",
        active: "var(--active)",
        hover: "var(--hover)",
      },
    },
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
