<template lang="pug">
    div(class="w-full flex flex-col rounded-xl header-gradient w-full  text-white p-8 gap-y-4")
        h1(class="text-2xl font-semibold") Unlock Premium Features
        p Take your VortiDect experience to the next level with exclusive featuresand content
        Button(class="mt-3 w-max text-themePrimary bg-white" text="Start Free Trial") 

    div(class="grid grid-cols-3 gap-x-8 ")
        div(class="flex flex-col p-6 rounded-lg bg-white gap-y-1 shadow-sm")
            BxsLayer(class="w-10 h-10 text-themePrimary")
            p(class="text-xl font-semibold") Advanced Layers
            p Create complex multi-layer buttons with advanced macro support
        div(class="flex flex-col p-6 rounded-lg bg-white gap-y-1 shadow-sm")
            MdiCloud(class="w-10 h-10 text-themePrimary")
            p(class="text-xl font-semibold") Cloud Sync
            p Seamlessly sync your configurations across all devices
        div(class="flex flex-col p-6 rounded-lg bg-white gap-y-1 shadow-sm")
            HugeiconsPaintBoard(class="w-10 h-10 text-themePrimary")
            p(class="text-xl font-semibold") Premium Themes
            p Across exclusive icon packs and design themes

    div(class="flex flex-col w-full gap-y-6")
        p(class="text-2xl text-black font-semibold") Shop
        div(class="grid grid-cols-4  gap-x-5 lg:gap-x-8")
            div(class="flex flex-col rounded-lg p-4 gap-y-4 bg-white" v-for="i in 4")
                div(class="w-full rounded-lg  flex items-center justify-center bg-themeBackground p-8")
                    FluentEmojiHighContrastPackage(class="w-10 h-10 text-themePrimary")
                p(class="text-lg font-semibold") Gaming Icons Pack
                p 50+ gaming-specific icons
                Button(text="$4.99" class="w-full bg-themePrimary text-white  ")
            
    div(class="flex flex-col w-full p-6 gap-y-2 rounded-lg bg-white ")
        div(class="flex w-full justify-between text-themePrimary")
            p(class="text-lg font-semibold") Rewards Progress
            div(class="flex w-auto gap-x-2 items-center font-semibold")
                MaterialSymbolsLightTrophy(class="w-6 h-6")
                p 250 points
        div.progress-bar(class="w-full h-[0.35rem] bg-white mt-3"  )
                div.progress-bar__bar(:style="{ width: 50 }" class=" h-full bg-themePrimary rounded-lg")
        p Earn 50 more points to unlock next free pack!

        
</template>

<script setup lang="ts">
import BxsLayer from "~icons/bxs/layer";
import MdiCloud from "~icons/mdi/cloud";
import HugeiconsPaintBoard from "~icons/hugeicons/paint-board";

import FluentEmojiHighContrastPackage from "~icons/fluent-emoji-high-contrast/package?width=32px&height=32px";
import MaterialSymbolsLightTrophy from "~icons/material-symbols-light/trophy?width=24px&height=24px";
</script>

<style lang="scss">
.header-gradient {
  background: rgb(80, 70, 229, 1);
  background: linear-gradient(
    90deg,
    rgba(80, 70, 229, 1) 0%,
    rgba(124, 58, 235, 1) 100%
  );
}
</style>
