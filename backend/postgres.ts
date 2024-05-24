import { Client } from "jsr:@bartlomieju/postgres";

const client = new Client({
  user: "frank",
  database: "testo",
  hostname: "localhost",
  port: 5432,
  password: "postgres",
});

const result = await client.queryArray("SELECT * FROM ipfs_data");
console.log(result.rows); // [[1, 'Carlos'], [2, 'John'], ...]

await client.connect();
