<template>
  <Html>
    <WindowsTitlebar />
    <div class="app-content">
      <NuxtLayout>
        <NuxtLoadingIndicator :height="5" :duration="3000" :throttle="400" />
        <NuxtPage class="" />
      </NuxtLayout>
    </div>
  </Html>
</template>

<script lang="ts" setup></script>

<style lang="scss">
.page-enter-active,
.page-leave-active {
  transition: all 0.4s;
}
.page-enter-from,
.page-leave-to {
  opacity: 0;
  filter: blur(1rem);
}

.app-content {
  padding-top: 30px; /* Same height as the titlebar */
}
</style>
