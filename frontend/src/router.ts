import { createMemoryHistory, createRouter } from "vue-router";

import AboutView from "./components/AboutView.vue";
import HomeView from "./components/HomeView.vue";
import OffersView from "./components/OffersView.vue";

const routes = [
  { path: "/", component: HomeView },
  { path: "/about", component: AboutView },
  { path: "/offers", component: OffersView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
