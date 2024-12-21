<template lang="pug">
div(class=" flex flex-col gap-y-4")
  div(class="flex flex-col p-6 rounded-xl bg-white gap-y-3 shadow-sm"  @click="isShowMore = !isShowMore")
    div(class="flex w-full justify-between items-center gap-x-2")
      p(class="font-semibold") Pairing Status
      div(:class="useConnectionStatus" class=" flex px-3 py-1 items-center justify-center text-sm  rounded-full")
        span {{connectionStore.getStatus}}
    div(class="flex flex-col gap-y-4" v-if="isShowMore && connectionStore.getStatus === 'paired'")
      hr(class="w-full h-[1px] my-1")
      div(class="flex w-full gap-x-2 " v-for="(info, idx) in connectionStore.getPaired" :key="idx")
        p(class="w-[10rem]") {{idx}}: 
        div(class="bg-themeBackground w-full h-auto flex px-3 py-1")
          p {{ info }} 
            //- div(class="absolute group-hover:flex hidden top-[2rem] left-[0.1rem] bg-themeBackground2 px-3 py-2 shadow-md text-nowrap rounded-lg	") OnePlus 11
          
  button(@click="sendMessage()" class=" px-3 py-2 bg-white rounded-md") send message to desktop

  DeckDynamic
</template>

<script setup lang="ts">
import { useConnectionStore } from "@/stores/Connection";
const { $socket, $isWebSocketConnected } = useNuxtApp();
const connectionStore = useConnectionStore();

const isShowMore = ref(false);
// $socket.value.onmessage = (event) => {
//   const message = JSON.parse(event.data);
//   alert("test");

//   //   if (message.type === "component") {
//   //     console.log("Received component:", message.payload);
//   //     // Use the component data (e.g., render it in the UI)
//   //     renderComponent(message.payload);
//   //   }
// };

function renderComponent(componentData) {
  // Example: Update the UI with the received data
  const container = document.getElementById("component-container");
  container.innerHTML = `<div id="${componentData.id}">${componentData.content}</div>`;
}

const sendMessage = async () => {
  console.log("Sending message");

  // Check if the socket is ready before sending the message
  if ($socket.value && $socket.value.readyState === WebSocket.OPEN) {
    $socket.value.send(
      JSON.stringify({
        action: "component",
        payload: "test key event",
      })
    ).await;
    alert("Message sent to server");
  } else {
    alert("WebSocket is not open, cannot send message.");
  }
};

// $socket.value.onmessage = (data) => {
//   alert(`Server response: ${data}`);
// };
</script>

<style lang="scss"></style>
