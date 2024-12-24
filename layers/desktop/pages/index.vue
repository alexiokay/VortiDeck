<template lang="pug">
  div(class="grid grid-cols-3 gap-x-4 lg:gap-x-8 ")
    div(class="flex flex-col p-6 rounded-xl bg-white gap-y-5 shadow-sm")
        div(class="flex w-full justify-between items-center gap-x-2")
          p(class="font-semibold") Paired Devices
          div(:class="statusClasses" class=" flex px-3 py-1 items-center justify-center text-sm  rounded-full")
            span {{connectionStore.getStatus}}
        div(class="flex w-full gap-x-2" v-for="(device, idx) in connectionStore.getPaired" :key="idx")
          BiPhone(class="w-6 h-6 ")
          span {{device.device}}
    div(class="flex flex-col p-6 rounded-xl bg-white gap-y-5 shadow-sm")
        div(class="flex w-full justify-between items-center")
          p(class="font-semibold") Premium Status
          div(class=" flex px-3 py-1 items-center justify-center bg-blue-100  text-blue-700 text-sm  rounded-full")
            span Pro
        div(class="flex w-full gap-x-2")
          Fa6SolidCrown(class="w-6 h-6 ")
          span Valid until March 2025

       

  button( @click="sendMessage()") send Message
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import BiPhone from "~icons/bi/phone?width=16px&height=16px";
import Fa6SolidCrown from "~icons/fa6-solid/crown?width=648px&height=576px";
import { useConnectionStore } from "@/stores/Connection";

const config = useRuntimeConfig();
console.log(config.public.test);

const connectionStore = useConnectionStore();

const { $socket } = useNuxtApp();

const sendMessage = () => {
  console.log("sending message");
  $socket.value.send("Hello, Tauri");
  // console.log("Message sent to server");
};

const statusClasses = useConnectionStatus;
</script>

<style lang="scss"></style>
