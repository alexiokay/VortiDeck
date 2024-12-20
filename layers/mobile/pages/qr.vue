<template lang="pug">
  div(class="flex flex-col h-auto gap-y-3 py-4")
    button(@click="startScan" class="px-3 py-2 border-[1px] rounded-full bg-white") Start Scanning
    button(@click="stopScan" class="px-3 py-2 border-[1px] rounded-full bg-white") Stop Scanning
  </template>

<script setup lang="ts">
import { scan, Format, cancel } from "@tauri-apps/plugin-barcode-scanner";
import DeviceDetector from "device-detector-js";

const deviceDetector = new DeviceDetector();

// interface ScannedData {
//   key: String;
// }
const scanningInProgress = ref(false);
// const scannedData = ref<ScannedData | null>(null);

const { $socket, $isWebSocketConnected } = useNuxtApp();

definePageMeta({
  layout: "qr",
});

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
      alert(JSON.stringify(message.server));
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
// startScan();
</script>

<style lang="scss"></style>
