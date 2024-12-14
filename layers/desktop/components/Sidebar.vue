<template lang="pug">


div#sidebar(:class="is_sidebar_open ? 'hide-left' : 'show-right'" class="z-[100] fixed w-[20rem] lg:w-[23rem] bottom-0 left-0 z-50 h-[100lvh] bg-[#Fbfafa]  pb-4 shadow-lg rounded-br-xl text-black flex flex-col") 
   
  div#sidebar-content(class="flex-1 overflow-y-auto relative w-full flex flex-col items-start rounded-md px-2 p-6 mt-[5rem]")
    ButtonSidebar(
      v-for="(button, index) in sidebarButtons"
      :key="button.to"
      :text="button.text"
      :to="button.to"
      :notifications_count="button.notifications_count"
      @click="desktopUtils.toggleSidebar()"
    )
        template(v-slot:icon)
          component(:is="button.icon" class="w-[1.4rem] h-[1.4rem]")
  
</template>

<script setup lang="ts">
import DashboardIcon from "~icons/ant-design/home-outlined";
import SubscriptionIcon from "~icons/fluent-mdl2/recurring-event";
import ClaritySettingsLine from "~icons/clarity/settings-line?width=36px&height=36px";
import FluentCommunication24Regular from "~icons/fluent/communication-24-regular?width=24px&height=24px";
import RiApps2Line from "~icons/ri/apps-2-line?width=24px&height=24px";
import FluentPremium28Regular from "~icons/fluent/premium-28-regular?width=28px&height=28px";
import PepiconsPencilKeyboard from "~icons/pepicons-pencil/keyboard?width=20px&height=20px";

import { useDesktopUtils } from "../stores/desktopUtils";

const desktopUtils = useDesktopUtils();

const is_sidebar_open = computed(() => {
  return desktopUtils.getIsSidebar;
});

const sidebarButtons = ref([
  { text: "Home", to: "/", icon: DashboardIcon },
  { text: "Deck", to: "/deck", icon: PepiconsPencilKeyboard },

  { text: "Games and Applications", to: "/", icon: RiApps2Line },
  { text: "Community", to: "/", icon: FluentCommunication24Regular },
  {
    text: "Premium",
    to: "/premium",
    icon: FluentPremium28Regular,
    // notifications_count: 0,
  },
  { text: "Settings", to: "/", icon: ClaritySettingsLine },
]);

function enter(el, done) {
  // Transition in
  el.style.opacity = 0;
  el.style.height = "0";

  requestAnimationFrame(() => {
    el.style.transition = "opacity 0.2s, height 0.2s";
    el.style.opacity = "1";
    el.style.height = "190px"; // Adjust the height you want the submenu to expand to

    // Wait for the transition to finish
    el.addEventListener("transitionend", done);
  });
}

function leave(el, done) {
  // Transition out
  el.style.transition = "opacity 0.2s, height 0.2s";
  el.style.opacity = "0";
  el.style.height = "0";

  // Wait for the transition to finish
  el.addEventListener("transitionend", done);
}
</script>

<style lang="scss">
.hide-left {
  animation: hide-left 0.5s ease-in-out forwards;

  @media (min-width: 1536px) {
    animation: none;
  }
}
.fade-enter {
}
.fade-enter-active {
  animation: fade-in 0.3s;
}
.fade-leave {
}
.fade-leave-active {
  animation: fade-out 0.3s;
}
@keyframes fade-in {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}
@keyframes fade-out {
  from {
    opacity: 1;
  }
  to {
    opacity: 0;
  }
}

@keyframes hide-left {
  from {
    transform: translateX(0rem);
  }

  to {
    transform: translateX(-100%);
  }
}
.show-right {
  animation: show-right 0.5s ease-in-out forwards;

  @media (min-width: 1536px) {
    animation: none;
  }
}

@keyframes show-right {
  from {
    transform: translateX(-25rem);
  }
  to {
    transform: translateX(0rem);
  }
}
</style>
