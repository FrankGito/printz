import VueVitePlugin from "npm:@vitejs/plugin-vue@latest";
import { defineConfig } from "npm:vite@latest";
import { templateCompilerOptions } from "npm:@tresjs/core";

import "npm:vue@latest";
import "npm:three@latest";
import "npm:@tresjs/leches@latest";
import "npm:@polkadot/api@latest";
import "npm:@polkadot/api-contract@latest";
import "npm:@polkadot/util@latest";
import "npm:@polkadot/types/interfaces";
import "npm:@tresjs/core";
import "npm:@tresjs/cientos";
import "npm:vue-router@4";
import "npm:marked@latest";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [VueVitePlugin({
    ...templateCompilerOptions,
  })],
});
