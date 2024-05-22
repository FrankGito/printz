<script setup>
import { ref } from "vue";
const file = ref(null);

function handleFileUpload(event) {
  file.value = event.target.files[0];
}

async function sendFileToServer() {
  const formData = new FormData();
  formData.append("file", file.value);

  try {
    const response = await fetch("http://localhost:8000/addGlb", {
      method: "POST",
      body: formData,
    });
    console.log("File sent successfully");
  } catch (error) {
    console.error("Error sending file:", error);
  }
}

async function getFileFromServer() {
  try {
    const response = await fetch("http://localhost:8000/getGlb");
    if (response.ok) {
      const blob = await response.blob();
      const url = URL.createObjectURL(blob);
      const link = document.createElement("a");
      link.href = url;
      link.setAttribute("download", "cube.glb");
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
    } else {
      console.error("Failed to fetch file:", response.status);
    }
  } catch (error) {
    console.error("Error fetching file:", error);
  }
}

async function getIpfsHashFromServer() {
  try {
    const response = await fetch("http://localhost:8000/getGlbHash");
    if (response.ok) {
      const res = await response.text()
      console.log(res)
    } else {
      console.error("Failed to fetch Ipfs Hash", response.status);
    }
  } catch (error) {
    console.error("Error fetching Ipfs Hash:", error);
  }
}
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
    </div>
  </div>
</template>
