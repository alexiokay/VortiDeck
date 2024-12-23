<!-- eslint-disable @typescript-eslint/no-unused-vars -->
<!-- eslint-disable @typescript-eslint/no-unused-vars -->
<!-- eslint-disable @typescript-eslint/no-unused-vars -->
<template lang="pug">
    div(class="w-full overflow-y-hidden h-full grid portrait:grid-cols-3 gap-5 landscape:grid-cols-5 portrait:px-[calc(41vw-16.3vh)] landscape:px-[1%]   place-content-center	")
        DeckDynamicButton(
        @click="useSendCommand('backspace')"
        v-for="button in buttons"
        :key="button.id"
        :label="button.label"
        :action="button.command"
        :icon="button.icon"
        )

</template>

<script setup lang="ts">
import FxemojiVideogame from "~icons/fxemoji/videogame?width=70px&height=70px";
import IconParkMessage from "~icons/icon-park/message?width=48px&height=48px";
import {
  vibrate,
  impactFeedback,
  notificationFeedback,
  selectionFeedback,
} from "@tauri-apps/plugin-haptics";

const test = async () => {
  await vibrate(105);
};
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const buttons = ref([
  {
    id: 1,
    label: "backspace",
    command: "backspace",
    icon: FxemojiVideogame,
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
    icon: IconParkMessage,
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
  {
    id: 1,
    label: "backspace",
    command: "backspace",
  },
]); // Store the fetched button configs

const fetchbuttons = () => {};
</script>

<style lang="scss">
.grid-container--fill {
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
}

.grid-container--fit {
  grid-template-columns: repeat(auto-fit, minmax(5rem, 1fr));
}
</style>
