// Import reactive and ref from Vue
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { ContractPromise } from "@polkadot/api-contract";
import type { WeightV2 } from "@polkadot/types/interfaces";
import { BN } from "@polkadot/util";

interface Id {
  u8?: number | string | BN;
  u16?: number | string | BN;
  u32?: number | string | BN;
  u64?: number | string | BN;
  u128?: string | number | BN;
  bytes?: Array<(number | string | BN)>;
}

export class IdBuilder {
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

const CONTRACT = "5E3UaHkVu1KuJMc9w4pkqSTa6Ae42yVvxSMPFjo7ANfaVdeF";

// !TODO
// const getOwnerOf = async (mintNumber: BN) => {
//   const wsProvider = new WsProvider("ws://127.0.0.1:9944");
//   const api = await ApiPromise.create({ provider: wsProvider });
//   const keyring = new Keyring({ type: "sr25519" });
//   const alicePair = keyring.addFromUri("//Alice");
//   const res = await fetch("./item.json");
//   const abi = await res.json();
//
//   const contract = new ContractPromise(api, abi, CONTRACT);
//
//   const gasLimit: WeightV2 = api.registry.createType("WeightV2", {
//     refTime: new BN("2000000000"),
//     proofSize: new BN("200000"),
//   }) as unknown as WeightV2;
//   const storageDepositLimit = null;
//
//   const { output } = await contract.query["psp34::ownerOf"](
//     alicePair.address,
//     {
//       gasLimit,
//       storageDepositLimit,
//     },
//     mintNumber,
//   );

// const jsonOutput: any = output?.toJSON()!;
// console.log(`Item (${mintNumber.toString()}) is owned by ${jsonOutput.ok}`);
// };

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

  const idToMint = IdBuilder.U8(2);
  await contract.tx["psp34Mintable::mint"]({
    gasLimit,
    storageDepositLimit,
  }, idToMint).signAndSend(alicePair, (res) => {
    if (res.isInBlock) {
      console.log("Mint");
    }
    if (res.isFinalized) {
      console.log(res);
      console.log("Done");
    }
  });
};

export { /*getOwnerOf,*/ getTotalSupply, mint };
