<script setup lang="ts">
import { marked } from "marked";
import {ref} from "vue"
const html = ref("")
fetch('./About.md')
.then(response => response.text())
.then((data) => {
  html.value = marked(data);
});
</script>
<template>
  <div class="container text-left">
    <div class="row row-cols-1 mt-3" v-html="html">
    </div>
  </div>
</template>
