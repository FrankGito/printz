<script setup lang="ts">
import Navbar from "./components/Navbar.vue";

import { TresLeches } from "@tresjs/leches";
import { useLeches } from "./composables/useLeches.ts";
useLeches();
</script>

<template>
  <TresLeches />
  <Navbar />
  <RouterView />
</template>
