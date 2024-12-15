<template lang="pug">
  div(class="grid grid-cols-3 gap-x-4 lg:gap-x-8 ")
    div(class="flex flex-col p-6 rounded-xl bg-white gap-y-5 shadow-sm")
        div(class="flex w-full justify-between items-center")
          p(class="font-semibold") Connection Status
          div(class=" flex px-3 py-1 items-center justify-center bg-green-100  text-green-700 text-sm  rounded-full")
            span Connected
        div(class="flex w-full gap-x-2")
          BiPhone(class="w-6 h-6 ")
          span Iphone 14 Pro
    div(class="flex flex-col p-6 rounded-xl bg-white gap-y-5 shadow-sm")
        div(class="flex w-full justify-between items-center")
          p(class="font-semibold") Premium Status
          div(class=" flex px-3 py-1 items-center justify-center bg-blue-100  text-blue-700 text-sm  rounded-full")
            span Pro
        div(class="flex w-full gap-x-2")
          Fa6SolidCrown(class="w-6 h-6 ")
          span Valid until March 2025

       
  p ssas
  button( @click="sendMessage()") send Message
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import BiPhone from "~icons/bi/phone?width=16px&height=16px";
import Fa6SolidCrown from "~icons/fa6-solid/crown?width=648px&height=576px";
const { $socket } = useNuxtApp();

const sendMessage = () => {
  console.log("sending message");
  $socket.value.send("Hello, Tauri");
  // console.log("Message sent to server");
};
</script>

<style lang="scss"></style>
