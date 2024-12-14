<template lang="pug">
div(class="flex flex-row w-full justify-center")
    button(@click="rotateComponent")
        SolarSmartphoneRotate2Linear(class="")
div(class="flex justify-center")
    div(:class="isLandscape? 'aspect-[9/16] h-[calc(100vh-12rem)]': 'aspect-[16/9] w-[calc(100vh-12rem)]'" class="rotatable-container  bg-white rounded-2xl") 
        component(:is="AsyncComp")
        
    //- div(:class="isLandscape? 'rotate-90': '	 '" class="aspect-[9/16] h-[calc(100vh-12rem)] rotatable-container  bg-white rounded-2xl") 
    //-     component(:is="AsyncComp")
button(@click="sendExampleComponent()") send example component     
    
</template>

<script setup lang="ts">
import SolarSmartphoneRotate2Linear from "~icons/solar/smartphone-rotate-2-linear?width=24px&height=24px";
const { $socket } = useNuxtApp();

const AsyncComp = defineAsyncComponent(
  () => import("../components/Deck/Example.vue")
);

const sendExampleComponent = () => {
  if ($socket.value.readyState === WebSocket.OPEN) {
    $socket.value.send(
      JSON.stringify({
        action: "component",
        payload: "test component",
      })
    );
  }
};

$socket.value.onopen = () => {
  console.log("Socket.IO connected");
};

$socket.value.onclose = () => {
  console.log("Socket.IO disconnected");
};

$socket.value.onmessage = (data) => {
  console.log("Server response:", data);
};

const isLandscape = ref(false);
const rotateComponent = () => {
  isLandscape.value = !isLandscape.value;
};
</script>

<style lang="scss" scoped>
/* Rotation container simulating the phone screen */
</style>
