<template lang="pug">
div(@click="haptics" class="border-[1px] rounded-xl border-[#1a1a1a35]  h-min w-[100%] aspect-square button bg-[#1a1a1af7]  shadow-md p-2")
  component(:is="icon" class="w-full h-full")

</template>

<script setup lang="ts">
import {
  vibrate,
  impactFeedback,
  notificationFeedback,
  selectionFeedback,
} from "@tauri-apps/plugin-haptics";

const haptics = async () => {
  await impactFeedback("soft");
};

const props = defineProps({
  id: Number,
  label: String,
  action: String,
  icon: Object,
});
</script>

<style lang="scss" scoped>
.button {
  transition: all 0.2s ease-in-out;
}

.button:active {
  transform: scale(1.1);
  transition: 0.1s;
}
</style>
