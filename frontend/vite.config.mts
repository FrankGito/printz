import { defineConfig } from "npm:vite@latest";
import vue from "npm:@vitejs/plugin-vue@latest";

import "npm:vue@latest";
import "npm:three@latest";
// import "npm:@types/three@latest";
import "npm:@tresjs/leches@latest";
import "npm:@polkadot/api@latest";
import "npm:@polkadot/api-contract@latest";
import "npm:@polkadot/util@latest";
import "npm:@polkadot/types/"


// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
});
