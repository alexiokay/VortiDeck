<template lang="pug">
div(class="flex  flex-col w-full h-full")
    div(v-if="qrCodeImage" class="flex flex-col w-full h-full  items-center justify-center mb-[2.1rem]")
        
        img(:src="qrCodeImage" alt="QR Code" class="")
    div(v-else)
        p "No QR code generated yet."

</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface ServiceInfo {
  ip: string;
  key: string;
  ws: string;
  host: string;
  data: string;
}

// Reactive variables to hold the QR code image and decoded data
const qrCodeImage = ref<string | null>(null);
const qrData = ref<ServiceInfo | null>(null);

async function getQRCode(data: string | null = null): Promise<void> {
  try {
    // Safely invoke the Tauri command and pass the optional data parameter
    const response = await invoke("generate_qr_code", { data });

    // console.log(response);

    // Set the base64 image source for the QR code
    qrCodeImage.value = `data:image/png;base64,${response}`;
  } catch (error) {
    // console.error("Failed to generate QR code:", error);
    qrCodeImage.value = null;
    qrData.value = null;
  }
}

getQRCode();
</script>

<style lang="scss"></style>
