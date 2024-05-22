<script setup lang="ts">
import * as THREE from "three";
import { useControls, TresLeches } from "@tresjs/leches";
import { onMounted, ref } from "vue";
import { watchEffect } from "vue";
import { getOwnerOf } from "./composables/usePsp34.ts";
import { BN } from "@polkadot/util";
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
  <div class="container text-center">
    <div class="row row-cols-3">
      <Card class="mt-3">
        <div>
          <canvas
            id="canvas_three"
            ref="canvasRef"
            width="300px"
            height="300px"
          ></canvas>
        </div>
      </Card>
      <Card class="mt-3">
      <FileInput />
      </Card>
        <Card class="mt-3">
          <TresCanvas clear-color="#82DBC5">
            <TresPerspectiveCamera :position="[3, 3, 3]" :look-at="[0, 0, 0]" />
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
<style>
#canvas {
  height: 50%;
  width: 50%;
}
</style>
