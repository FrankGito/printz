<script setup>
import { ref } from "vue";
import { useCommitText } from "./../composables/useIpfs.ts";

const { cid, commitText, commitedText, fetchCommitedText } = useCommitText();
const textToCommit = ref();

const handleCommitText = async () => {
  await commitText(textToCommit.value);
};
</script>

<template>
  <div class="flex">
    <input
      id="commitText"
      type="text"
      v-model="textToCommit"
      class="mint-text"
    />
    <button class="mint-button" @click="handleCommitText">
      <div class="mint-text">Add text</div>
    </button>
    <p id="commitTextCidOutput" class="mint-text">cid:</p>
    <p class="cid-text">{{ cid }}</p>
  </div>
  <div v-if="cid">
    <button
      id="fetchCommitedTextButton"
      class="mint-button"
      @click="fetchCommitedText"
    >
      <div class="mint-text">Fetch</div>
    </button>
    <div>
      <p id="fetchedCommitedTextOutput" class="mint-text">added text:</p>
      <p class="cid-text">{{ commitedText }}</p>
    </div>
  </div>
</template>

<style scoped>
@import url("https://fonts.googleapis.com/css2?family=Text+Me+One&display=swap");
body {
  font-family: "Text Me One", sans-serif;
}
.flex {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
button {
  width: 150px;
  height: 50px;
  padding: 10px 15px;
  background: #ffffff;
  border-radius: 5px;
  border: 1px black solid;
  justify-content: center;
  align-items: center;
}

button:hover {
  background: #ffe5f3;
}
button:active {
  background: #ffcce7;
}

input {
  width: 300px;
  height: 50px;
  padding: 0px 5px;
  background: #ffffff;
  border-radius: 5px;
  border: 1px black solid;
  justify-content: center;
  align-items: center;
}

.mint-text {
  color: black;
  font-size: 24px;
  font-family: Text Me One;
  font-weight: 400;
  letter-spacing: 2.4px;
  word-wrap: break-word;
}
.cid-text {
  color: black;
  font-size: 10px;
  font-family: Text Me One;
  font-weight: 400;
  letter-spacing: 2.4px;
  word-wrap: break-word;
}
</style>
