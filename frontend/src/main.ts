import { createApp } from "vue";
import "@tresjs/leches/dist/style.css";
import App from "./App.vue";
import { HeliaProviderPlugin } from "./plugins/HeliaProviderPlugin.ts";

const app = createApp(App);
app.use(HeliaProviderPlugin);
app.mount("#app");
