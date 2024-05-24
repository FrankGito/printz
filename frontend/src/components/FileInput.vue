<script setup>
import { getTotalSupply, mint, setAttribute } from "../composables/usePsp34.ts";
import {
  handleFileUpload,
  sendFileToServer,
  getFileFromServer,
  getIpfsHashFromServer,
} from "../composables/useApi.ts";
</script>
<template>
  <div>
    <label for="formFile" class="form-label mt-4">Upload your .glb File</label>
    <input
      class="form-control"
      id="formFile"
      type="file"
      @change="handleFileUpload"
    />
    <div class="d-grid gap-2 mt-4">
      <button
        @click="sendFileToServer"
        class="btn btn-lg btn-outline-primary"
        type="button"
      >
        Send file to server
      </button>
      <button
        @click="getFileFromServer"
        class="btn btn-lg btn-outline-primary"
        type="button"
      >
        Get file from server
      </button>
      <button
        @click="getIpfsHashFromServer"
        class="btn btn-lg btn-outline-primary"
        type="button"
      >
        Get Ipfs Hash from server
      </button>
      <button
        @click="getTotalSupply"
        class="btn btn-lg btn-outline-primary"
        type="button"
      >
        Get total supply from contract
      </button>
      <button
        @click="mint(12)"
        class="btn btn-lg btn-outline-primary"
        type="button"
      >
        mint to contract
      </button>
      <button
        @click="setAttribute(12, 'uri', 'asdfasdf')"
        class="btn btn-lg btn-outline-primary"
        type="button"
      >
        setAttribute to contract
      </button>
    </div>
  </div>
</template>
