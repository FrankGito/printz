import { Application } from "jsr:@oak/oak/application";
import { Router } from "jsr:@oak/oak/router";
import { oakCors } from "https://deno.land/x/cors/mod.ts";

const app = new Application();
const router = new Router();

router.post("/upload", async (ctx) => {
  const formData = await ctx.request.body.formData();
  const file = formData.get("file") as File;
  const filePath = `./uploads/${file.name}`;
  const content = await file.arrayBuffer();
  const array = new Uint8Array(content);
  await Deno.writeFile(
    filePath,
    array,
  );
  ctx.response.body = { message: "File uploaded successfully" };
});

app.use(oakCors({ origin: "http://localhost:5173" }));
app.use(router.routes());
app.use(router.allowedMethods());

await app.listen({ port: 8080 });
