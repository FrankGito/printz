// @ts-ignore it has
import { ref } from "vue";
const file = ref(null);
const uploaded = ref(true);

async function handleFileUpload(event: any) {
  file.value = event.target.files[0];
  uploaded.value = false;
  await sendFileToServer();
}

async function sendFileToServer() {
  const formData = new FormData();
  formData.append("file", file.value);

  try {
    const response = await fetch("http://localhost:8000/upload", {
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
      console.log(url);
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
    const response = await fetch("http://localhost:8000/getLatestGlbHash");
    if (response.ok) {
      const res = await response.text();
      console.log(res);
      return res;
    } else {
      console.error("Failed to fetch Ipfs Hash", response.status);
    }
  } catch (error) {
    console.error("Error fetching Ipfs Hash:", error);
  }
}
export {
  getFileFromServer,
  getIpfsHashFromServer,
  handleFileUpload,
  sendFileToServer,
  uploaded,
};
