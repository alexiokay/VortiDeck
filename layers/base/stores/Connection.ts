// Import necessary dependencies
// import { useStorage } from "@vueuse/core";
import { createPinia } from "pinia";

import { defineStore } from "pinia"; // Import defineStore for Pinia
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

type Device = {
  clientId: string;
  deviceType: string;
  device: string;
};

type ConnectionStatus = "connected" | "connecting" | "disconnected" | "paired";

// Define the Pinia store
export const useConnectionStore = defineStore("connectionStore", {
  state: () => ({
    paired: [] as Device[],
    status: "disconnected" as ConnectionStatus,
  }),
  getters: {
    getPaired(state) {
      return state.paired;
    },
    getStatus(state) {
      if (state.paired.length >= 1) {
        if (state.status === "connected") return "paired";
        else return state.status;
      } else return state.status;
    },
  },
  actions: {
    addPeer(peer: Device) {
      this.paired.push(peer); // Add the new peer to the paired array
    },
    setStatus(status: ConnectionStatus) {
      this.status = status;
    },
  },
  persist: {
    storage: sessionStorage,
  },
});
