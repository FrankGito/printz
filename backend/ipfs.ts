const command = new Deno.Command("ipfs", {
  args: ["add", "./cube.glb"],
  stdin: "piped",
  stdout: "piped",
});
console.log("1");
const process = command.spawn();
console.log("2");

await process.stdin.close();
//!Info process output logs the process
const result = await process.output();
const output = new TextDecoder().decode(result.stdout);

//Excluding l 0 i o and these kind of sorts
const regex = /([1-9A-HJ-NP-Za-km-z]{46})\s/;
const match = output.match(regex);
const ipfsHash = match ? match[1] : null;

// console.log(ipfsHash);
