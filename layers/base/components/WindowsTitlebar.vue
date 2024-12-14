<template lang="pug">

  
  .titlebar(class="pl-4 bg-themeBackground  relative")

    // Titlebar title
    .titlebar-title.mr-auto#titlebar-title VortiDeck
  
    // Titlebar buttons (minimize, maximize, close)
    .titlebar-button#titlebar-minimize(@click="appWindow.minimize()")
      img(src="https://api.iconify.design/mdi:window-minimize.svg", alt="minimize")
    .titlebar-button#titlebar-maximize(@click="appWindow.toggleMaximize()")
      img(src="https://api.iconify.design/mdi:window-maximize.svg", alt="maximize")
    .titlebar-button#titlebar-close(@click="appWindow.close()")
      img(src="https://api.iconify.design/mdi:close.svg", alt="close")
  
    // Transparent overlay with excluded button area
  .titlebar-overlay(class="fixed top-0 left-0 w-[calc(100vw-90px)] h-[30px] z-[10000] " data-tauri-drag-region)
</template>

<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";

// when using `"withGlobalTauri": true`, you may use
// const { getCurrentWindow } = window.__TAURI__.window;

const appWindow = getCurrentWindow();
</script>

<style lang="scss">
// Windows Titlebar configuration
.titlebar {
  height: 30px;

  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  user-select: none;
  -webkit-user-select: none;
}
.titlebar-button:hover {
  background: #000000;
  & * {
    filter: invert(1);
  }
}
</style>
