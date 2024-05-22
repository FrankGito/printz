<script setup lang="ts">
import * as THREE from "three";
import { useControls, TresLeches } from "@tresjs/leches";
import { onMounted, ref } from "vue";
import { watchEffect } from "vue";
import { getOwnerOf } from "./composables/usePsp34.ts";
import { BN } from "@polkadot/util";

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
  <div class="card">
    <canvas id="canvas" ref="canvasRef" width="300px" height="300px"></canvas>
    <button class="mint-button">
      <div class="mint-text">Mint</div>
    </button>
  </div>
</template>

<style scoped>
@import url("https://fonts.googleapis.com/css2?family=Text+Me+One&display=swap");
body {
  font-family: "Text Me One", sans-serif;
}
.mint-button {
  width: 150px;
  height: 50px;
  padding: 10px 15px;
  background: #ffffff;
  border-radius: 5px;
  border: 1px black solid;
  justify-content: center;
  align-items: center;
}

.mint-button:hover {
  background: #ffe5f3;
}
.mint-button:active {
  background: #ffcce7;
}

.mint-text {
  color: black;
  font-size: 24px;
  font-family: Text Me One;
  font-weight: 400;
  letter-spacing: 2.4px;
  word-wrap: break-word;
}

.card {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 400px;
  height: 400px;
  padding: 25px;
  background: white;
  border-radius: 5px;
  border: 1px black solid;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
  gap: 25px;
  display: inline-flex;
}
</style>
