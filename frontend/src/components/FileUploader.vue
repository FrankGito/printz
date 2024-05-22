<script setup>
import {ref} from 'vue'
const file = ref(null)

function handleFileUpload(event){
  file.value = event.target.files[0]
}
function logMe(event){
  console.log(file.value)
}
async function sendFileToServer(){
  const formData = new FormData()
  formData.append('file', file.value)

  try {
    const response = await fetch('http://localhost:8080/upload', {
      method: 'POST',
      body: formData
    })
    console.log('File sent successfully')
  } catch (error) {
    console.error('Error sending file:', error)
  }
}
</script>
<template>
  <div>
    <input type="file" @change="handleFileUpload" />
    <button @click="logMe">Log me</button>
     <button @click="sendFileToServer">Send file to server</button>
  </div>
</template>


