// const { createThemes } = require("tw-colors");

module.exports = {
  darkMode: "class", // or 'media' or 'false'
  content: [
    "./main_app./components/**/*.{js,vue,ts}",
    "./main_app./layouts/**/*.vue",
    "./main_app./pages/**/*.vue",
    "./main_app./plugins/**/*.{js,ts}",
    "./main_app./nuxt.config.{js,ts}",
    "./main_app./app.vue",
  ],
  variants: {
    extend: {
      display: ["group-hover"],
    },
  },

  theme: {
    extend: {
      colors: {
        themeBackground: "var(--background)",
        themeBackground2: "var(--background2)",
        themeBackground3: "var(--background3",
        themeText: "var(--text)",
        themePrimary: "var(--primary)",
        themeSecondary: "var(--secondary)",
        themeHint: "var(--hint)",
        themeText2: "var(--text2)",
        themeBorder: "var(--border)",
        themeBorder2: "var(--border2)",
        themeInvert: "var(--invert)",
        themeButton: "var(--button)",
      },
      keyframes: {
        showUp: {
          "0%": { transform: "translateY(50px)" },
          "100%": { transform: "translateY(0px)" },
        },
        hideDown: {
          "0%": { transform: "translateY(0px)" },
          "100%": { transform: "translateY(50px)" },
        },
        hideDownNavbar: {
          "0%": { transform: "translateY(0rem)" },
          "100%": { transform: "translateY(25rem)" },
        },
        spin: {
          "0%": { transform: "rotate(0deg)" },
          "100%": { transform: "rotate(180deg)" },
        },
        spinBack: {
          "0%": { transform: "rotate(180deg)" },
          "100%": { transform: "rotate(0deg)" },
        },
      },
      animation: {
        showUp: "showUp 0.2s linear",
        hideDownNavbar: "hideDownNavbar 0.4s linear",
        showUpNavbar: "hideDownNavbar 0.4s linear",
        spinOnce: "spin 1s forwards",
        spinOnceBack: "spinBack 1s forwards",
      },
    },
    fontFamily: {
      brown: ["BROWN", "sans-serif"],
      itim: ["ITIM", "sans-serif"],
      intern: ["INTERN", "sans-serif"],
      roboto: ["ROBOTO", "sans-serif"],
      robotolight: ["ROBOTOLIGHT", "sans-serif"],
      montserrat: ["MONTSERRAT", "sans-serif"],
      elmessiri: ["ELMESSIRI", "sans-serif"],
      publicsans: ["PublicSans", "sans-serif"],
      righteous: ["RIGHTEOUS", "regular"],
      flamabook: ["FLAMABOOK", "regular"],
      mulish: ["Mulish", "regular"],
    },
    screens: {
      sm: "640px",
      // => @media (min-width: 640px) { ... }

      md: "768px",
      // => @media (min-width: 768px) { ... }

      lg: "1024px",
      // => @media (min-width: 1024px) { ... }

      xl: "1330px",
      // => @media (min-width: 1330px) { ... }

      "2xl": "1536px",
      // => @media (min-width: 1536px) { ... }
      "3xl": "1920px",
      "4xl": "2560px",
    },
  },
  plugins: [
    // createThemes({
    //   "default-light": {},
    //   "default-dark": {},
    // }),
  ],
};
