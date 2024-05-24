import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { ContractPromise } from "@polkadot/api-contract";
// @ts-ignore It has ^¹
import type { WeightV2 } from "@polkadot/types/interfaces";
import { BN } from "@polkadot/util";

const CONTRACT = "5FH5UKqDCUwWh8QJu9DMFrwFtRPogRfpDYa8udM4vpBmyKVc";

interface Id {
  u8?: number | string | BN;
  u16?: number | string | BN;
  u32?: number | string | BN;
  u64?: number | string | BN;
  u128?: string | number | BN;
  bytes?: Array<(number | string | BN)>;
}

const getTotalSupply = async () => {
  const wsProvider = new WsProvider("ws://127.0.0.1:9944");
  const api = await ApiPromise.create({ provider: wsProvider });
  const keyring = new Keyring({ type: "sr25519" });
  const alicePair = keyring.addFromUri("//Alice");
  const res = await fetch("./item.json");
  const abi = await res.json();

  const contract = new ContractPromise(api, abi, CONTRACT);

  const gasLimit: WeightV2 = api.registry.createType("WeightV2", {
    refTime: new BN("2000000000"),
    proofSize: new BN("200000"),
  }) as unknown as WeightV2;
  const storageDepositLimit = null;

  // @ts-ignore It does exists
  const { output } = await contract.query["psp34::totalSupply"](
    alicePair.address,
    {
      gasLimit,
      storageDepositLimit,
    },
  );

  const jsonOutput: any = output?.toJSON()!;
  console.log(`Item has a total Supply of ${jsonOutput.ok}`);
  return jsonOutput.ok;
};

const mint = async (id: number) => {
  const wsProvider = new WsProvider("ws://127.0.0.1:9944");
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: {
      Id: {
        _enum: {
          U8: "u8",
          U16: "u16",
          U32: "u32",
          U64: "u64",
          U128: "u128",
          Bytes: "Vec<u8>",
        },
      },
    },
  });
  const keyring = new Keyring({ type: "sr25519" });
  const alicePair = keyring.addFromUri("//Alice");
  const res = await fetch("./item.json");
  const abi = await res.json();

  const contract = new ContractPromise(api, abi, CONTRACT);

  const gasLimit: WeightV2 = api.registry.createType("WeightV2", {
    refTime: new BN("20000000000"),
    proofSize: new BN("20000000"),
  }) as unknown as WeightV2;
  const storageDepositLimit = null;

  const id_convert = api.createType("Id", {
    "U8": id, // use 1 for Id::U8(1)
  });

  // @ts-ignore It does exists
  await contract.tx["psp34Mintable::mint"]({
    gasLimit,
    storageDepositLimit,
  }, id_convert).signAndSend(alicePair, (res: any) => {
    if (res.isInBlock) {
      console.log("Mint");
    }
    if (res.isFinalized) {
      console.log(res);
      console.log("Done");
    }
  });
};

const setAttribute = async (id: number, key: string, value: string) => {
  const wsProvider = new WsProvider("ws://127.0.0.1:9944");
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: {
      Id: {
        _enum: {
          U8: "u8",
          U16: "u16",
          U32: "u32",
          U64: "u64",
          U128: "u128",
          Bytes: "Vec<u8>",
        },
      },
    },
  });
  const keyring = new Keyring({ type: "sr25519" });
  const alicePair = keyring.addFromUri("//Alice");
  const res = await fetch("./item.json");
  const abi = await res.json();

  const contract = new ContractPromise(api, abi, CONTRACT);

  const gasLimit: WeightV2 = api.registry.createType("WeightV2", {
    refTime: new BN("20000000000"),
    proofSize: new BN("20000000"),
  }) as unknown as WeightV2;
  const storageDepositLimit = null;

  const id_convert = api.createType("Id", {
    "U8": id,
  });

  // const key = "uri";
  const keyHex = [];
  for (let i = 0; i < key.length; i++) {
    keyHex.push("0x" + key.charCodeAt(i).toString(16));
  }
  // const value = "QmQdrFfgHrBBSNZB9C5Wjaa7vzxxccfEcj47RhZSwsymZ2";
  const valueHex = [];
  for (let i = 0; i < value.length; i++) {
    valueHex.push("0x" + value.charCodeAt(i).toString(16));
  }

  // @ts-ignore It does exists
  await contract.tx["setAttribute"](
    {
      gasLimit,
      storageDepositLimit,
    },
    id_convert,
    keyHex,
    valueHex,
  ).signAndSend(alicePair, (res: any) => {
    if (res.isInBlock) {
      console.log("is in block");
    }
    if (res.isFinalized) {
      console.log("is isFinalized");
    }
  });
};

export { getTotalSupply, mint, setAttribute };
