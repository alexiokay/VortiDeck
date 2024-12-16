<template lang="pug">
div(class="flex flex-col  h-auto gap-y-3 py-4 ")
  
    button(@click="startScan" class="px-3 py-2 border-[1px] rounded-full bg-white") Start Scanning
    button(@click="stopScan" class="px-3 py-2 border-[1px] rounded-full bg-white") Stop Scanning
</template>

<script setup lang="ts">
import { ref } from "vue";
import { scan, Format, cancel } from "@tauri-apps/plugin-barcode-scanner";

const scanningInProgress = ref(false);
const scannedData = ref<string | null>(null);

definePageMeta({
  layout: "qr",
});
const startScan = async () => {
  try {
    scanningInProgress.value = true;
    const result = await scan({
      windowed: true, // This will display the camera feed

      formats: [Format.QRCode], // Scan for QR codes
    });

    if (result) {
      scannedData.value = result.content;
      scanningInProgress.value = false;
      alert("Scanned QR Code: " + result.content);
      cancel();
    }
  } catch (error) {
    console.error("Error scanning:", error);
    scanningInProgress.value = false;
    alert("Error during scanning: " + (error as Error).message);
  }
};

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
