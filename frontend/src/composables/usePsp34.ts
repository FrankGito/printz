// Import reactive and ref from Vue
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { ContractPromise } from "@polkadot/api-contract";
import type { WeightV2 } from "@polkadot/types/interfaces";
import { BN } from "@polkadot/util";

const CONTRACT = "5E3UaHkVu1KuJMc9w4pkqSTa6Ae42yVvxSMPFjo7ANfaVdeF";

const getOwnerOf = async (mintNumber: BN) => {
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

  const { output } = await contract.query["psp34::ownerOf"](
    alicePair.address,
    {
      gasLimit,
      storageDepositLimit,
    },
    new BN(mintNumber),
  );

  const jsonOutput: any = output?.toJSON()!;
  console.log(`Item (${mintNumber.toString()}) is owned by ${jsonOutput.ok}`);
};

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

export { getOwnerOf, getTotalSupply };
