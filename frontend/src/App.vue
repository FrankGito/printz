<script setup lang="ts">
import * as THREE from "three";
import { useControls, TresLeches } from "@tresjs/leches";
import FileInput from "./components/FileInput.vue";
import Navbar from "./components/Navbar.vue";
import Card from "./components/Card.vue";
import { TresCanvas } from "@tresjs/core";

const { debugMessage } = useControls({
  debugMessage: "I`m a Debug UI",
  debugBtn: {
    label: "Debug Button",
    type: "button",
    size: "lg",
    onClick: () => {},
  },
});
</script>

<template>
  <TresLeches />
  <Navbar />
  <div class="container text-center">
    <div class="row row-cols-3">
      <Card class="mt-3">
        <FileInput />
      </Card>
      <Card class="mt-3">
        <TresCanvas clear-color="#00000000">
          <TresPerspectiveCamera :position="[5, 5, 5]" :look-at="[0, 0, 0]" />
          <TresMesh>
            <TresTorusGeometry :args="[1, 0.5, 16, 32]" />
            <TresMeshBasicMaterial color="orange" />
          </TresMesh>
          <TresAmbientLight :intensity="1" />
        </TresCanvas>
      </Card>
    </div>
  </div>
</template>
