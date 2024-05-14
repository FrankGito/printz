// /* eslint-disable no-console */
import { ref } from "vue";
import type { App } from "vue";

export const HeliaProviderPlugin = {
  install: async (app: App) => {
    const loading = ref(true);
    const error = ref("");
    const helia = ref();
    const fs = ref();
    app.provide("HeliaProvider", {
      loading,
      error,
      helia,
      fs,
    });
    try {
      //@ts-ignore loading via cdn
      const instance = await window.Helia.createHelia();
      loading.value = false;
      helia.value = instance;
      //@ts-ignore loading via cdn
      fs.value = window.HeliaUnixfs.unixfs(instance);
    } catch (e) {
      console.error(e);
      error.value = e.toString();
      loading.value = false;
    }
  },
};
