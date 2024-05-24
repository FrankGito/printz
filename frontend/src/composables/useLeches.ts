import { useControls } from "@tresjs/leches";
//@ts-ignore it its there
import { ref } from "vue";
import { getTotalSupply, mint, setAttribute } from "../composables/usePsp34.ts";
import {
  getFileFromServer,
  getIpfsHashFromServer,
} from "../composables/useApi.ts";

const useLeches = () => {
  const totalSupplyRef = ref("-");
  const cidRef = ref("-");
  useControls({
    totalSupply: totalSupplyRef,
    cid: cidRef,
    getTotalSupply: {
      label: "getTotalSupply",
      type: "button",
      size: "lg",
      onClick: async () => {
        const supply = await getTotalSupply();
        totalSupplyRef.valeu = supply;
      },
    },
    mint: {
      label: "mint",
      type: "button",
      size: "lg",
      onClick: async () => {
        const currentSupply = await getTotalSupply();
        await mint(currentSupply.toNumber() + 1);
        const newSupply = await getTotalSupply();
        totalSupplyRef.value = newSupply;
      },
    },
    setAttribute: {
      label: "setAttribute",
      type: "button",
      size: "lg",
      onClick: async () => {
        const currentSupply = await getTotalSupply();
        const cid = await getIpfsHashFromServer();
        await setAttribute(currentSupply, "uri", cid!);
      },
    },
    getIpfsHashFromServer: {
      label: "getIpfsHashFromServer",
      type: "button",
      size: "lg",
      onClick: async () => {
        const cid = await getIpfsHashFromServer();
        cidRef.value = cid;
      },
    },
    getFileFromServer: {
      label: "getFileFromServer",
      type: "button",
      size: "lg",
      onClick: async () => {
        await getFileFromServer();
      },
    },
  });
};

export { useLeches };
