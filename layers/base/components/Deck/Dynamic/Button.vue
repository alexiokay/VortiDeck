<template lang="pug">
div(class="flex flex-col gap-y-2")
    div( class="border-[1px] rounded-md border-gray-400 aspect-square w-full button ") 
    p {{ props.label }}
</template>

<script setup lang="ts">
const props = defineProps({
  id: Number,
  label: String,
  action: String,
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
