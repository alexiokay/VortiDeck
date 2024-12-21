import { invoke } from "@tauri-apps/api/core";
import { useConnectionStore } from "@/stores/Connection";
import { type PeerInfo } from "@/types/Peer";

export default defineNuxtPlugin(async () => {
  const connectionStore = useConnectionStore();

  async function retrievePeers(): Promise<PeerInfo[] | null> {
    try {
      // Safely invoke the Tauri command
      const result = await invoke<PeerInfo[]>("retrieve_peers");
      console.log("retrieved peers:", result); // Log the result

      return result; // Return the list of services
    } catch (error) {
      console.error("Error discovering services:", error);
      // alert("Error discovering services");
      return null; // Return null in case of error
    }
  }

  const peers = await retrievePeers();

  connectionStore.setPeers(peers);
});
