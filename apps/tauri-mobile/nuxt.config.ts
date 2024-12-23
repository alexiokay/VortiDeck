// https://nuxt.com/docs/api/configuration/nuxt-config
import { internalIpV6, internalIpV4 } from "internal-ip";
export default defineNuxtConfig({
  compatibilityDate: "2024-11-01",
  devtools: { enabled: true },
  extends: [["../../layers/mobile", { install: true }]],

  routeRules: {
    "/": { prerender: true },
    "*": { prerender: true },
    /* do the same for all routes used */
  },

  ogImage: { enabled: false },

  // ogImage: { enabled: false },

  //!tauri
  // build: {
  //   distDir: "../.output/public",
  // },
  devServer: {
    host: "localhost",
    port: 3001,
  },

  hooks: {
    "vite:extend": function ({ config }) {
      if (config.server && config.server.hmr && config.server.hmr !== true) {
        config.server.hmr.protocol = "ws";
        config.server.hmr.host = "192.168.178.129";
        config.server.hmr.clientPort = 3001;
        config.server.strictPort = true;
      }
    },
  },

  // hooks: {
  //   "vite:extend": async (ctx) => {
  //     const ip = await internalIpV4(); // Fetch internal IP asynchronously
  //     if (
  //       ctx.config.server &&
  //       ctx.config.server.hmr &&
  //       ctx.config.server.hmr !== true
  //     ) {
  //       ctx.config.server.hmr.host = ip; // Use the fetched IP
  //       ctx.config.server.hmr.protocol = "ws";
  //       ctx.config.server.strictPort = true;
  //       ctx.config.server.hmr.port = 443;
  //     }
  //   },
  // },

  // !tauri

  // runtimeConfig: {
  //   GOOGLE_MAPS_API_KEY: process.env.GOOGLE_MAPS_API_KEY,
  //   TWILIO_ACCOUNT_SID: process.env.TWILIO_ACCOUNT_SID,
  //   TWILIO_AUTH_TOKEN: process.env.TWILIO_AUTH_TOKEN,
  //   TWILIO_SHORT_NAME: process.env.TWILIO_SHORT_NAME,
  //   TWILIO_PHONE_NUMBER: process.env.TWILIO_PHONE_NUMBER,

  //   public: {
  //     test: "test",
  //     cwd: process.cwd(),
  //     BASE_URL: process.env.BASE_URL,
  //     API_TOKEN: process.env.API_TOKEN,
  //     API_URL: process.env.API_URL,
  //     HOST: process.env.HOST,
  //     FETCH_HOST: process.env.FETCH_HOST,
  //     gtm_id: process.env.GOOGLE_TAG_MANAGER_ID,
  //     gtm_enabled: process.env.GOOGLE_TAG_MANAGER_ENABLED,
  //     gtm_debug: process.env.GOOGLE_TAG_MANAGER_DEBUG,
  //   },
  // },
});
