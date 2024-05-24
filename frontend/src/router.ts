import { createMemoryHistory, createRouter } from "vue-router";

import AboutView from "./components/AboutView.vue";
import HomeView from "./components/HomeView.vue";

const routes = [
  { path: "/", component: HomeView },
  { path: "/about", component: AboutView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
