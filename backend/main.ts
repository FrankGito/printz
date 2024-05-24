import { Application } from "jsr:@oak/oak/application";
import { Router } from "jsr:@oak/oak/router";
import { oakCors } from "https://deno.land/x/cors@v1.2.2/mod.ts";
import { generateIpfsHash } from "./ipfs.ts";
import { insert_ipfs_data } from "./postgres.ts";

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
  const content = await Deno.readFile(file);
  const array = new Uint8Array(content);
  await Deno.writeFile(
    filePath,
    array,
  );

  ctx.response.body = { message: "File uploaded successfully" };
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
  const ipfsHash = await generateIpfsHash();
  ctx.response.headers.set("Content-Type", "text/plain");
  ctx.response.body = ipfsHash;
});

router.post("/postIpfsData", async (ctx) => {
  const ipfsHash = await generateIpfsHash() || "";
  await insert_ipfs_data("cube3.glb", ipfsHash);
});

app.use(oakCors({ origin: "http://localhost:5173" }));
app.use(router.routes());
app.use(router.allowedMethods());

await app.listen({ port: 8000 });
