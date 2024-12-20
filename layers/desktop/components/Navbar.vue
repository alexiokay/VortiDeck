<template lang="pug">

div(class="flex w-full items-center gap-x-5 pb-3 px-6 themeBackground justify-end fixed z-50 select-none" )
  
    div(v-if="connectionStore.getStatus === 'connected'" class=" text-themeConnected flex items-center gap-x-2 ml-auto relative group")
        div(class="w-3 h-3 bg-themeConnected   rounded-full group")
        p(class="") Connected 
   
        

    div(v-else-if="connectionStore.getStatus === 'disconnected'" class="  flex items-center gap-x-2 ml-auto relative group")
        div( class="w-3 h-3 bg-themeDisconnected rounded-full group")
        p(class="text-themeDisconnected ") Disconnected
        
    div(v-else-if="connectionStore.getStatus === 'paired'" class="flex items-center gap-x-2 ml-auto relative group")
        div( class="w-3 h-3  bg-themePaired rounded-full group")
        p(class="text-themePaired ") Paired
        div(class="absolute group-hover:flex hidden top-[2rem] left-[0.1rem] bg-themeBackground2 px-3 py-2 shadow-md text-nowrap rounded-lg	") 
          p(v-for="device in connectionStore.getPaired") {{device.device}}
   
    div(v-else-if="connectionStore.getStatus === 'reconnecting'" class="flex items-center gap-x-2 ml-auto relative group")
        div.status.reconnecting( class="w-3 h-3 bg-themeConnecting  rounded-full group")
        p(class="text-themeConnecting ") Reconnecting

      
        
    NuxtLink(to="pair") Connect
    div(@click="isProfileSearch = !isProfileSearch" class="  bg-themeBackground2 rounded-lg flex px-3 py-2 gap-x-4  items-center border-[1px] min-w-[20rem] hover:bg-themeBackground2 hover:cursor-pointer relative")
      MaterialSymbolsLightScreenshotMonitorOutline(class="h-7 w-7")
      p Code: Domyslny
      HugeiconsArrowDown01(class="ml-auto")
      <!-- Profile search dropdown -->
      div(@click.stop="" v-if="isProfileSearch" class="absolute w-full top-[2.8rem] left-0 h-auto flex flex-col bg-white rounded-lg px-4 pt-4 hover:cursor-default  ")
        div(class="flex w-full px-3 py-2 gap-x-3 rounded-lg border-[1px] items-center")
          PixelarticonsSearch(class="w-5 h-5")
          input(placeholder="Search profiles " class="focus:outline-none focus:shadow-outline")
        div(class="h-[10rem] w-full overflow-y-scroll	 ")
        div(class="mt-auto h-[5rem] items-center justify-center flex border-t-[1px]")
          button(class="font-light rounded-md py-3 px-3 md:px-4  text-black hover:bg-blue-100") Manage Profiles
    PixelarticonsUser(class="hover:cursor-pointer")
GgMenuLeft(class="w-6 h-6 hover:cursor-pointer fixed top-[3rem] left-[2rem] z-[10000] " @click="desktopUtils.toggleSidebar()")
</template>

<script setup lang="ts">
import GgMenuLeft from "~icons/gg/menu-left?width=24px&height=24px";
import MaterialSymbolsLightScreenshotMonitorOutline from "~icons/material-symbols-light/screenshot-monitor-outline?width=24px&height=24px";
import HugeiconsArrowDown01 from "~icons/hugeicons/arrow-down-01?width=24px&height=24px";
import PixelarticonsSearch from "~icons/pixelarticons/search?width=24px&height=24px";
import PixelarticonsUser from "~icons/pixelarticons/user?width=24px&height=24px";

import { useConnectionStore } from "@/stores/Connection";

const connectionStore = useConnectionStore();
import { useDesktopUtils } from "../stores/desktopUtils";

const desktopUtils = useDesktopUtils();
const isProfileSearch = ref(false);
const { $isWebSocketConnected } = useNuxtApp();
</script>

<style lang="scss">
/* Base styles for the reconnecting status */
.status.reconnecting {
  animation: pulse 1.5s infinite ease-in-out; /* Apply the pulse animation */
}

/* Pulse animation keyframes */
@keyframes pulse {
  0% {
    transform: scale(0.8);
    opacity: 1;
  }
  50% {
    transform: scale(1.3);
    opacity: 0.8;
  }
  100% {
    transform: scale(0.8);
    opacity: 1;
  }
}
</style>
