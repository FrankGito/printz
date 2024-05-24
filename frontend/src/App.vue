<script setup lang="ts">
import Navbar from "./components/Navbar.vue";
import {onMounted} from 'vue'
import { TresLeches } from "@tresjs/leches";
import { useLeches } from "./composables/useLeches.ts";
useLeches();

onMounted(()=>{
  const el = document.querySelector("#app > div > div > header > i")
  el.innerText = "👑"
})
</script>

<template>
  <TresLeches />
  <Navbar />
  <RouterView />
</template>
