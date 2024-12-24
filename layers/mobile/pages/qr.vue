<template lang="pug">
div(class="flex flex-col w-full h-full justify-between")
  div(class="w-full h-[10vh] bg-black bg-opacity-50 justify-center items-center flex")
    p(class="text-white font-2xl") Scan Qr Code to Pair
  div(class="w-full h-[20vh] bg-black bg-opacity-50 justify-center items-center flex ")
    button(@click="exit" class=" px-4 py-2 text-black bg-white shadow-lg border-[1px] border-gray-200 rounded-xl text-2xl") Cancel


//- div(class="w-full h-full flex flex-col items-center justify-start relative")
//-   <!-Transparent box -->
//-   div(class="absolute top-1/2 left-1/2 w-[20rem] h-[20rem] bg-transparent shadow-[0_-1px_0_30vmax_rgba(0,0,0,0.5)] pointer-events-auto transform -translate-x-1/2 -translate-y-1/2")
//-     <!-Content inside the transparent box -->
//-     p(class="text-white text-center") Align the QR code
</template>

<script setup lang="ts">
import { scan, Format, cancel } from "@tauri-apps/plugin-barcode-scanner";
import DeviceDetector from "device-detector-js";
import { useConnectionStore } from "@/stores/Connection";

const deviceDetector = new DeviceDetector();
const connectionStore = useConnectionStore();

// interface ScannedData {
//   key: String;
// }
const scanningInProgress = ref(false);
// const scannedData = ref<ScannedData | null>(null);

const { $socket, $isWebSocketConnected } = useNuxtApp();
const router = useRouter();
definePageMeta({
  layout: "qr",
});

const exit = () => {
  cancel();
  router.push("/");
};
// Function to establish a WebSocket connection
const connectToWebsocket = (wsUrl: string, key: string) => {
  // if ($isWebSocketConnected) {
  //   console.warn("WebSocket already connected.");
  //   return;
  // }
  $socket.value = null;
  $socket.value = new WebSocket(wsUrl);

  $socket.value.onopen = () => {
    $isWebSocketConnected.value = true;
    connectionStore.setStatus("paired");
    console.log("Connected to WebSocket:", wsUrl);

    let clientId = localStorage.getItem("clientId");
    if (!clientId) {
      clientId = crypto.randomUUID(); // Generate a unique ID
      localStorage.setItem("clientId", clientId);
    }
    const userAgent = navigator.userAgent;
    const device = deviceDetector.parse(userAgent);

    // Send authentication or handshake message with secret key
    const authMessage = JSON.stringify({
      device: device,
      action: "authenticate",
      clientId,
      secret: key,
    });
    $socket.value?.send(authMessage);
  };

  $socket.value.onmessage = (event) => {
    console.log("WebSocket message received:", event.data);

    // Handle specific actions based on received message
    const message = JSON.parse(event.data);
    if (message.action === "component") {
      // Handle component update
      console.log("Component action received:", message.payload);
    }
    if (message.action === "server_info") {
      // paired.value?.push(message)
      // alert(JSON.stringify(message.server));
      connectionStore.setPeers(message.server);
    }

    if (message.action === "new_token") {
      localStorage.setItem("token", message.token);
    }
  };
};

// Function to start scanning
const startScan = async () => {
  try {
    scanningInProgress.value = true;
    const result = await scan({
      windowed: true, // This will display the camera feed
      formats: [Format.QRCode], // Scan for QR codes
    });

    if (result) {
      scanningInProgress.value = false;
      // alert("Scanned QR Code: " + result.content);

      // Parse scanned QR code content
      const parsedData = JSON.parse(result.content);
      const { ip, key, ws } = parsedData;
      // scannedData.value = {
      //   key: key,
      // };

      if (ip && key && ws) {
        connectToWebsocket(ws, key);
      } else {
        alert("Invalid QR code format.");
      }

      cancel(); // Stop the scanner after a successful scan
      router.push("/");
    }
  } catch (error) {
    console.error("Error scanning:", error);
    scanningInProgress.value = false;
    alert("Error during scanning: " + (error as Error).message);
  }
};

// Function to stop scanning
const stopScan = async () => {
  try {
    scanningInProgress.value = false;
    await cancel(); // Cancel the scanning process
  } catch (error) {
    console.error("Error stopping scan:", error);
    alert("Error stopping the scan: " + (error as Error).message);
  }
};

// Optionally, you can trigger the scan automatically when the component mounts
startScan();
</script>

<style lang="scss"></style>
