import { useControls } from "@tresjs/leches";
//@ts-ignore it its there
import { ref, watch } from "vue";
import { getTotalSupply, mint, setAttribute } from "../composables/usePsp34.ts";
import {
  getFileFromServer,
  getIpfsHashFromServer,
} from "../composables/useApi.ts";

const useLeches = () => {
  const totalSupplyRef = ref("-");
  const cidRef = ref("-");
  const modelsRef = ref([]);

  const { models } = useControls({
    models: {
      value: "EmptyCube",
      options: [],
    },
    getAllUploads: {
      label: "getAllUploads",
      type: "button",
      size: "lg",
      onClick: async () => {
        const res = await fetch("http://localhost:8000/getAllUploads");
        const json = await res.json();
        let lechesA: { text: string; value: string }[] = [];
        json.uploads.map((el: any) => {
          lechesA.push({ text: el.filename, value: el.filename });
        });
        modelsRef.value = lechesA;
        update();
      },
    },
    totalSupply: totalSupplyRef,
    cid: cidRef,
    getTotalSupply: {
      label: "getTotalSupply",
      type: "button",
      size: "lg",
      onClick: async () => {
        const supply = await getTotalSupply();
        totalSupplyRef.value = supply;
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
    mint: {
      label: "mint",
      type: "button",
      size: "lg",
      onClick: async () => {
        const currentSupply = await getTotalSupply();
        await mint(currentSupply + 1);
        const newSupply = await getTotalSupply();
        const cid = await getIpfsHashFromServer();
        await setAttribute(newSupply, "uri", cid!);
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
    getFileFromServer: {
      label: "getFileFromServer",
      type: "button",
      size: "lg",
      onClick: async () => {
        await getFileFromServer();
      },
    },
  });

  function update() {
    models.value.options = modelsRef.value;
  }
};

export { useLeches };
