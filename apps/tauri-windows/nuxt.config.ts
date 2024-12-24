// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-11-01",
  devtools: { enabled: true },
  extends: [["../../layers/desktop", { install: true }]],
  future: {
    compatibilityVersion: 4,
  },
  ssr: false,

  build: {
    transpile: ["@tauri-apps/api"],
  },

  // ogImage: { enabled: false },

  //!tauri
  // build: {
  //   distDir: "../.output/public",
  // },
  devServer: {
    host: "localhost",
    port: 3000,
  },

  hooks: {
    "vite:extend": function ({ config }) {
      if (config.server && config.server.hmr && config.server.hmr !== true) {
        config.server.hmr.protocol = "ws";
        // config.server.hmr.host = "0.0.0.0";
        config.server.hmr.port = 3000;
        config.server.strictPort = true;
      }
    },
  },

  vite: {
    clearScreen: false,
    envPrefix: ["VITE_", "TAURI_"],
  },

  // !tauri

  runtimeConfig: {
    GOOGLE_MAPS_API_KEY: process.env.GOOGLE_MAPS_API_KEY,
    TWILIO_ACCOUNT_SID: process.env.TWILIO_ACCOUNT_SID,
    TWILIO_AUTH_TOKEN: process.env.TWILIO_AUTH_TOKEN,
    TWILIO_SHORT_NAME: process.env.TWILIO_SHORT_NAME,
    TWILIO_PHONE_NUMBER: process.env.TWILIO_PHONE_NUMBER,

    public: {
      cwd: process.cwd(),
      BASE_URL: process.env.BASE_URL,
      API_TOKEN: process.env.API_TOKEN,
      API_URL: process.env.API_URL,
      HOST: process.env.HOST,
      FETCH_HOST: process.env.FETCH_HOST,
      gtm_id: process.env.GOOGLE_TAG_MANAGER_ID,
      gtm_enabled: process.env.GOOGLE_TAG_MANAGER_ENABLED,
      gtm_debug: process.env.GOOGLE_TAG_MANAGER_DEBUG,
      test: process.env.TEST,
    },
  },
});
