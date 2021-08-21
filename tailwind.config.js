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
      default: [defaultTheme.fontFamily.mono],
    },
    colors: {
      white: "#FFFFFF",
      default: {
        bg: "#0D0D0D",
        border: "#333333",
      },
      "title-bar": {
        bg: "#191919",
        close: "#AA0000",
      },
      "menu-bar": "#242424",
      "scroll-bar": {
        bg: "#090909",
        fg: "#4F4F4F",
      },
      table: {
        odd: "#121212",
        even: "#1A1A1A",
      },
      popup: "#121212",
      me1: {
        bg: "#1C526E",
        tab: "#296B94",
        active: "#478CAB",
        hover: "#D46E2B",
      },
      me2: {
        bg: "#A3521F",
        tab: "#B35E29",
        active: "#D97D40",
        hover: "#38853B",
      },
      me3: {
        bg: "#660000",
        tab: "#870000",
        active: "#B30000",
        hover: "#05476E",
      },
      theme: {
        bg: "var(--bg)",
        tab: "var(--tab)",
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
