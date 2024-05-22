<script setup>
import { ref } from "vue";
const file = ref(null);

function handleFileUpload(event) {
  file.value = event.target.files[0];
}
function logMe(event) {
  console.log(file.value);
}
async function sendFileToServer() {
  const formData = new FormData();
  formData.append("file", file.value);

  try {
    const response = await fetch("http://localhost:8080/upload", {
      method: "POST",
      body: formData,
    });
    console.log("File sent successfully");
  } catch (error) {
    console.error("Error sending file:", error);
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
        @click="logMe"
        class="btn btn-lg btn-outline-primary"
        type="button"
      >
        Log me
      </button>
      <button
        @click="sendFileToServer"
        class="btn btn-lg btn-outline-primary"
        type="button"
      >
        Send file to server
      </button>
    </div>
  </div>
</template>
