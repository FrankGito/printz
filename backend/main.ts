import { Application } from "jsr:@oak/oak/application";
import { Router } from "jsr:@oak/oak/router";
import { oakCors } from "https://deno.land/x/cors@v1.2.2/mod.ts";
import { generateIpfsHash } from "./ipfs.ts";
import { get_ipfs_data, insert_ipfs_data } from "./postgres.ts";
import { generateUUID } from "./uuid.ts";

const app = new Application();
const router = new Router();

router.post("/upload", async (ctx) => {
  const formData = await ctx.request.body.formData();
  const file = formData.get("file") as File;

  const uploadsDir = "./uploads";
  const files = Deno.readDirSync(uploadsDir);
  const fileNames = Array.from(files).map((file) => file.name);
  const fileNumbers = fileNames.map((name) => parseInt(name.split("_")[0]));
  const maxFileNumber = Math.max(...fileNumbers);
  const nextCount = isNaN(maxFileNumber) ? 1 : maxFileNumber + 1;

  const filePath = `${uploadsDir}/${nextCount}_printz.glb`;
  //@ts-ignore it work that way ^^ usually I want the arrayBuffer
  // const content = await Deno.readFile(file);
  const content = await file.arrayBuffer();
  const array = new Uint8Array(content);
  await Deno.writeFile(
    filePath,
    array,
  );
  const ipfsHash = await generateIpfsHash(filePath) || "";

  // TODO! Pass tokenId to postCall
  const tokenId = nextCount.toString();
  await insert_ipfs_data(
    tokenId,
    `${nextCount}_printz.glb`,
    ipfsHash,
  );

  ctx.response.body = { message: "File uploaded successfully" };
});

router.get("/getAllUploads", async (ctx) => {
  const data = await get_ipfs_data();
  const uploads = data.rows.map((row) => {
    return {
      id: row[0],
      filename: row[1],
      uri: row[2],
    };
  });
  ctx.response.body = { uploads };
});

router.post("/addGlb", async (ctx) => {
  const formData = await ctx.request.body.formData();
  const file = formData.get("file") as File;
  const filePath = `./uploads/cube.glb`;
  const content = await file.arrayBuffer();
  const array = new Uint8Array(content);
  await Deno.writeFile(
    filePath,
    array,
  );
  ctx.response.body = { message: "File uploaded successfully" };
});

router.get("/getGlb", async (ctx) => {
  const filePath = "./uploads/cube.glb";
  const fileContent = await Deno.readFile(filePath);
  ctx.response.headers.set("Content-Type", "model/gltf-binary");
  ctx.response.body = fileContent;
});

router.get("/getGlbHash", async (ctx) => {
  const ipfsHash = await generateIpfsHash("./uploads/0_printz.glb");
  ctx.response.headers.set("Content-Type", "text/plain");
  ctx.response.body = ipfsHash;
});

router.get("/getLatestGlbHash", async (ctx) => {
  const uploadsDir = "./uploads";
  const files = Deno.readDirSync(uploadsDir);
  const fileNames = Array.from(files).map((file) => file.name);
  const fileNumbers = fileNames.map((name) => parseInt(name.split("_")[0]));
  const maxFileNumber = Math.max(...fileNumbers);
  const count = isNaN(maxFileNumber) ? 1 : maxFileNumber;
  const filePath = `${uploadsDir}/${count}_printz.glb`;
  //!TODO It generates a new one ^^ Have to call db
  const ipfsHash = await generateIpfsHash(filePath);
  ctx.response.headers.set("Content-Type", "text/plain");
  ctx.response.body = ipfsHash;
});

router.post("/postIpfsData", async (ctx) => {
  const ipfsHash = await generateIpfsHash("./uploads/0_printz.glb") || "";
  const mock_uuid = generateUUID();
  await insert_ipfs_data(mock_uuid, "0_printz.glb", ipfsHash);
});

app.use(oakCors({ origin: "http://localhost:5173" }));
app.use(router.routes());
app.use(router.allowedMethods());

await app.listen({ port: 8000 });
