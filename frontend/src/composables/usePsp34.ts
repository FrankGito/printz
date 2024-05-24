// Import reactive and ref from Vue
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { ContractPromise } from "@polkadot/api-contract";
import type { WeightV2 } from "@polkadot/types/interfaces";
import { BN } from "@polkadot/util";

const CONTRACT = "5CjF5TNJpYQYhHjiYtVDSaaXnEtqfL6DryUAWayRs3hWXZpU";

interface Id {
  u8?: number | string | BN;
  u16?: number | string | BN;
  u32?: number | string | BN;
  u64?: number | string | BN;
  u128?: string | number | BN;
  bytes?: Array<(number | string | BN)>;
}

class IdBuilder {
  static U8(value: number | string | BN): Id {
    return {
      u8: value,
    };
  }
  static U16(value: number | string | BN): Id {
    return {
      u16: value,
    };
  }
  static U32(value: number | string | BN): Id {
    return {
      u32: value,
    };
  }
  static U64(value: number | string | BN): Id {
    return {
      u64: value,
    };
  }
  static U128(value: string | number | BN): Id {
    return {
      u128: value,
    };
  }
  static Bytes(value: Array<(number | string | BN)>): Id {
    return {
      bytes: value,
    };
  }
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

  const { output } = await contract.query["psp34::totalSupply"](
    alicePair.address,
    {
      gasLimit,
      storageDepositLimit,
    },
  );

  const jsonOutput: any = output?.toJSON()!;
  console.log(`Item has a total Supply of ${jsonOutput.ok}`);
};

const mint = async () => {
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

  let id = api.createType("Id", {
    "U8": 5, // use 1 for Id::U8(1)
  });

  await contract.tx["psp34Mintable::mint"]({
    gasLimit,
    storageDepositLimit,
  }, id).signAndSend(alicePair, (res) => {
    if (res.isInBlock) {
      console.log("Mint");
    }
    if (res.isFinalized) {
      console.log(res);
      console.log("Done");
    }
  });
};

const setAttribute = async () => {
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

  let id = api.createType("Id", {
    "U8": 5, // use 1 for Id::U8(1)
  });

  const key = "a".charCodeAt(0).toString(16);
  const value = "a".charCodeAt(0).toString(16);
  await contract.tx["setAttribute"](
    {
      gasLimit,
      storageDepositLimit,
    },
    id,
    [key],
    [value],
  ).signAndSend(alicePair, (res) => {
    if (res.isInBlock) {
      console.log("is in block");
    }
    if (res.isFinalized) {
      console.log("is isFinalized");
    }
  });
};

export { getTotalSupply, mint, setAttribute };
