<template lang="pug">
NuxtLink.menu-button(:to="props.to" :class="isActive? 'font-bold': ''" class="relative whitespace-nowrap text-xl text-ellipsis flex w-full group justify-between rounded-md  hover:bg-slate-200  md:h-[2.7rem] px-4 py-7 items-center hover:cursor-pointer")
    div(class="flex gap-x-4 items-center")
        div(class="w-6 md:w-7 h-6 md:h-7 flex justify-center items-center")
          slot(name="icon")
        p {{ props.text }}
    div(v-if="props.notifications_count >= 0" :class="props.notifications_count > 0? 'moving-flag-bg': 'bg-slate-50'" class="h-full w-[2rem]  flex items-center justify-center rounded-md")
        p() {{props.notifications_count}}

    div(class="w-5 md:w-7 h-5 md:h-7 flex justify-center items-center")
      slot(name="icon-2")
    .active-pointer(v-if="isActive" class="absolute -left-[3.5rem]  w-[0.65rem] h-11 bg-violet-600 rounded-r-md")

</template>

<script setup lang="ts">
const props = defineProps({
  text: {
    type: String,
    required: true,
  },
  to: {
    type: String,
    required: false,
  },

  notifications_count: {
    type: Number,
    required: false,
  },
});

const route = useRoute();
const isActive = computed(() => {
  console.log("route");
  console.log(route.path);
  const path = route.path.split("/");
  return route.path.split("/")[path.length - 1] === props.text.toLowerCase()
    ? true
    : false;
});
</script>

<style lang="scss">
.menu-button {
  .active-pointer {
    animation: slideIn 0.6s forwards;
  }
}
@keyframes slideIn {
  0% {
    transform: translateX(-2rem);
  }
  100% {
    transform: translateX(-0.6rem);
  }
}

.moving-flag-bg {
  animation: colorChange 2s infinite;
}

@keyframes colorChange {
  0% {
    background: rgb(241 245 249);
  }
  10% {
    background: #7c3aed;
    color: white;
  }
  30% {
    background: #7c3aed;
    color: white;
  }
  40% {
    background: rgb(241 245 249);
  }
  100% {
    background: rgb(241 245 249);
  }
}
</style>
