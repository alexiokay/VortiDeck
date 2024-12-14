import { defineNuxtConfig } from "nuxt/config";
import Icons from "unplugin-icons/vite";
import ViteYaml from "@modyfi/vite-plugin-yaml";
import { createResolver } from "@nuxt/kit";

const { resolve } = createResolver(import.meta.url);

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
  // ssr: true,
  modules: [
    "@nuxtjs/tailwindcss",
    // "@nuxt/content",
    "@nuxtjs/i18n",
    "@formkit/auto-animate/nuxt",
    "nuxt-og-image",
    "@nuxt/image",
    [
      "@pinia/nuxt",
      {
        autoImports: ["defineStore", "acceptHMRUpdate", "useStore"],
      },
    ],
    "@nuxtjs/robots",
    "@nuxtjs/sitemap",
    "pinia-plugin-persistedstate/nuxt",
    "unplugin-icons/nuxt",
  ],

  future: {
    compatibilityVersion: 4,
  },
  alias: { "@": resolve("./") },

  i18n: {
    vueI18n: "./i18n.config.ts", // if you are using custom path, default
    locales: [
      { code: "en", language: "en-US" },
      { code: "nl", language: "nl-NL" },
    ],
    defaultLocale: "en",
  },

  build: {
    transpile: ["@headlessui/vue", "@fawmi/vue-google-maps"],
  },

  devServer: {
    port: 3001,
  },

  content: {
    // https://content.nuxtjs.org/api/configuration
  },

  nitro: {
    prerender: {
      failOnError: false,
      crawlLinks: true,
      routes: ["/", "/sitemap.xml"],
    },
  },

  routeRules: {
    // Static page generated on-demand, revalidates in background (ISG)

    "/**": { ssr: true },

    // Static page generated on-demand once (SSG - or at least mighty close) { static: true },

    // Render these routes on the client (SPA) { ssr: false },
  },

  image: {
    domains: [process.env.FRONTEND_URL as string, "localhost:3001"],
    format: ["webp"],
    provider: "ipx",
    ipx: {
      dir: "public",
    },
    storyblok: {
      baseURL: "https://a.storyblok.com",
    },
    twicpics: {
      baseURL: "https://freshandtidy.twic.pics/partners/",
      // Feel free to use our demo domain to try the following examples.
      // baseUrl: 'https://demo.twic.pics/'
    },
  },

  app: {
    head: {
      charset: "utf-8",
      viewport: "width=device-width, initial-scale=1",
      title: "",
      meta: [
        // <meta name="description" content="My amazing site">
        {
          hid: `description`,
          name: "description",
          content: " ",
        },
      ],
      link: [
        { rel: "icon", type: "image/x-icon", href: "/images/favicon.ico" },
      ],
    },
  },

  css: ["./assets/css/styles.css", "./assets/scss/main.scss"],

  vite: {
    plugins: [
      Icons({
        // the feature below is experimental ⬇️
        autoInstall: true,
      }),
      ViteYaml(),
    ],
    css: {
      preprocessorOptions: {
        scss: {
          api: "modern-compiler", // or "modern"
        },
      },
    },
  },

  compatibilityDate: "2024-11-23",

  runtimeConfig: {
    public: {
      WEBSOCKET_SERVER_URL: process.env.WEBSOCKET_SERVER_URL,
    },
  },
});
