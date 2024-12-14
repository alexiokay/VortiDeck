// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  extends: [["../base", { install: true }]],
  ssr: false,
  routeRules: {
    "/": { prerender: true },
    "*": { prerender: true },
    /* do the same for all routes used */
  },

  ogImage: { enabled: false },
});
