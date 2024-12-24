<template>
  <Html>
    <NuxtLayout>
      <NuxtLoadingIndicator :height="5" :duration="3000" :throttle="400" />
      <NuxtPage class="" />
    </NuxtLayout>
  </Html>
</template>

<script lang="ts" setup>
const config = useRuntimeConfig();
console.log(config.public.test);
// import { listen } from "@tauri-apps/api/event";
// import { useMobileStore } from "@/stores/Mobile";

// type Device = {
//   client_id: string;
//   device_type: string;
//   device: string;
// };

// const mobileStore = useMobileStore();
// listen<Device>("new_peer_added", (event) => {
//   console.log(`peer ${event.payload.deviceType} added`);
//   console.log(event.payload);
//   mobileStore.addPeer(event.payload);
//   console.log(mobileStore.paired);
// });
</script>

<style lang="scss">
.page-enter-active,
.page-leave-active {
  transition: all 0.4s;
}
.page-enter-from,
.page-leave-to {
  opacity: 0;
  filter: blur(1rem);
}
</style>
