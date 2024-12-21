// Import necessary dependencies
// import { useStorage } from "@vueuse/core";
import { createPinia } from "pinia";
import { type PeerInfo } from "@/types/Peer";
import { defineStore } from "pinia"; // Import defineStore for Pinia
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

type ConnectionStatus = "connected" | "connecting" | "disconnected" | "paired";

// Define the Pinia store
export const useConnectionStore = defineStore("connectionStore", {
  state: () => ({
    paired: [] as PeerInfo[],
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
    addPeer(peer: PeerInfo) {
      this.paired.push(peer); // Add the new peer to the paired array
    },
    setPeers(peers: PeerInfo[] | null) {
      if (peers) this.paired = peers;
    },
    setStatus(status: ConnectionStatus) {
      this.status = status;
    },
  },
  persist: {
    storage: sessionStorage,
  },
});
