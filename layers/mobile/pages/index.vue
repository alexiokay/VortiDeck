<template lang="pug">
   
    div(v-if="$isWebSocketConnected" class="flex items-center gap-x-2 ml-auto relative group")
        div(class="w-3 h-3 bg-green-600 rounded-full group")
        p(class="text-green-600") Connected
        div(class="absolute group-hover:flex hidden top-[2rem] left-[0.1rem] bg-themeBackground2 px-3 py-2 shadow-md text-nowrap rounded-lg	") OnePlus 11
        

    div(v-else class="flex items-center gap-x-2 ml-auto relative group")
        div( class="w-3 h-3 bg-red-600 rounded-full group")
        p(class="text-red-600") Disconnected
    p sass s ad s XD
    button(@click="sendMessage()" class=" px-3 py-2 bg-white rounded-md") send message to socket
</template>

<script setup lang="ts">
const { $socket, $isWebSocketConnected } = useNuxtApp();

$socket.value.onmessage = (event) => {
  const message = JSON.parse(event.data);
  alert("test");

  //   if (message.type === "component") {
  //     console.log("Received component:", message.payload);
  //     // Use the component data (e.g., render it in the UI)
  //     renderComponent(message.payload);
  //   }
};

function renderComponent(componentData) {
  // Example: Update the UI with the received data
  const container = document.getElementById("component-container");
  container.innerHTML = `<div id="${componentData.id}">${componentData.content}</div>`;
}

const sendMessage = () => {
  console.log("Sending message");

  // Check if the socket is ready before sending the message
  if ($socket.value && $socket.value.readyState === WebSocket.OPEN) {
    $socket.value.send(
      JSON.stringify({
        action: "component",
        payload: "test key event",
      })
    );
    alert("Message sent to server");
  } else {
    alert("WebSocket is not open, cannot send message.");
  }
};

$socket.value.onmessage = (data) => {
  alert(`Server response: ${data}`);
};
</script>

<style lang="scss"></style>
