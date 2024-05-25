<script setup lang="ts">
import {ref, onMounted, watch} from 'vue'
import * as THREE from "three";
import { TresCanvas } from "@tresjs/core";
import { OrbitControls, useGLTF } from "@tresjs/cientos";

import Card from "./Card.vue";
import FileInput from "./FileInput.vue";
import Canvas from "./Canvas.vue";
import { handleFileUpload,  uploaded} from '../composables/useApi.ts'
import { mint } from '../composables/usePsp34.ts'

</script>
<template>
  <div class="container text-center">
    <div class="row row-cols-3">
      <Card class="mt-3">
        <div>
          <div style="height: 300px">
            <Suspense>
              <Canvas></Canvas>
            </Suspense>
          </div>
          <label for="formFile" class="form-label mt-4"
            >Upload your .glb File</label
          >
          <input
            class="form-control"
            id="formFile"
            type="file"
            @change="handleFileUpload"
            :disabled="!uploaded"
          />
          <button
            @click="mint"
            class="btn btn-lg btn-outline-primary mt-3"
            type="button"
            :disabled="uploaded"
          >
            mint
          </button>
        </div>
      </Card>
    </div>
  </div>
</template>
