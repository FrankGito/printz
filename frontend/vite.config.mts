import { defineConfig } from "npm:vite@^5.2.10";
import vue from "npm:@vitejs/plugin-vue@^5.0.4";

import "npm:vue@^3.4.23";
import "npm:three@latest";
// import "npm:@types/three@latest";
import "npm:@tresjs/leches@latest";
import "npm:@polkadot/api@latest";
import "npm:@polkadot/api-contract@latest";
import "npm:@polkadot/util@latest";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
});
