<script setup lang="ts">
import * as THREE from "three";
import { useControls, TresLeches } from "@tresjs/leches";
import { onMounted, ref } from "vue";
import { watchEffect } from "vue";
import { getOwnerOf } from "./composables/usePsp34.ts";
import { BN } from "@polkadot/util";
import FileUploader from "./components/FileUploader.vue";
import Navbar from "./components/Navbar.vue";
import Card from "./components/Card.vue";

const { debugMessage } = useControls({
  debugMessage: "I`m a Debug UI",
  debugBtn: {
    label: "Debug Button",
    type: "button",
    size: "lg",
    onClick: () => {},
  },
});

watchEffect(() => {
  console.log(debugMessage.value.value);
});

const canvasRef = ref<HTMLCanvasElement | null>(null);

onMounted(() => {
  getOwnerOf(new BN(0));

  const canvas = canvasRef.value;

  if (canvas) {
    const renderer = new THREE.WebGLRenderer({ canvas, alpha: true });
    renderer.setSize(300, 300);

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, 1, 0.1, 1000);

    const geometry = new THREE.BoxGeometry();
    const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
    const cube = new THREE.Mesh(geometry, material);

    scene.add(cube);

    camera.position.z = 5;

    const animate = () => {
      requestAnimationFrame(animate);

      cube.rotation.x += 0.01;
      cube.rotation.y += 0.01;

      renderer.render(scene, camera);
    };

    animate();
  }
});
</script>

<template>
  <TresLeches />
  <Navbar />
  <Card>
    <canvas id="canvas" ref="canvasRef" width="300px" height="300px"></canvas>
  </Card>
  <FileUploader />
</template>
