import { Application } from "jsr:@oak/oak/application";
import { Router } from "jsr:@oak/oak/router";
import { oakCors } from "https://deno.land/x/cors/mod.ts";

const app = new Application();
const router = new Router();

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

app.use(oakCors({ origin: "http://localhost:5173" }));
app.use(router.routes());
app.use(router.allowedMethods());

await app.listen({ port: 8080 });
