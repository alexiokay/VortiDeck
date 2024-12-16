<template lang="pug">
div(class="w-full h-auto grid grid-cols-4 gap-x-4 p-2 px-4 border-t-[1px] shadow-t-xl items-center   rounded-t-md")
    NuxtLink(v-for="(item, index) in menu" :key="index" :to="item.to" class="flex flex-col   h-auto w-auto rounded-md border-[1px] border-gray-500 p-1  items-center justify-center sample menu-item__active")
        component(:is="item.icon" class="h-[2.4rem] ")
        p(class="text-sm") {{item.title}}
    
</template>

<script setup lang="ts">
import ArcticonsKeysCafeRgbKeyboard from "~icons/arcticons/keys-cafe-rgb-keyboard?width=48px&height=48px";

const menu = ref({
  deck: {
    title: "deck",
    icon: ArcticonsKeysCafeRgbKeyboard,
    to: "/",
    active: true,
  },
  test1: {
    title: "QR",
    icon: ArcticonsKeysCafeRgbKeyboard,
    to: "/qr",
    active: true,
  },
  test2: {
    title: "test2",
    icon: ArcticonsKeysCafeRgbKeyboard,
    to: "/",
    active: true,
  },
  test3: {
    title: "test3",
    icon: ArcticonsKeysCafeRgbKeyboard,
    to: "/",
    active: true,
  },
});
</script>

<style lang="scss">
.menu-item__active {
  background-color: white;
  color: black;
}

.menu-item__unactive {
  background-color: black;
  color: white;
}
.sample {
  transition: all 0.2s ease-in-out;
}

.sample:active {
  transform: scale(1.1);
  transition: 0.1s;
}
</style>
