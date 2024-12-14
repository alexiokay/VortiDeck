export const useDesktopUtils = defineStore("desktopUtils", {
  state: () => {
    return {
      // cookies settings
      isSidebar: false,
    };
  },
  getters: {
    getIsSidebar: (state) => {
      return state.isSidebar;
    },
  },
  actions: {
    toggleSidebar() {
      this.isSidebar = !this.isSidebar;
    },
    persist: {
      storage: piniaPluginPersistedstate.cookies(),
      // only save isCookiesSet as cookie
    },
    // other options...
  },
});
