import L from "leaflet";

export default defineNuxtPlugin(async (nuxtApp) => {
  nuxtApp.L = L;
});
