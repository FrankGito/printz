import { Client } from "jsr:@bartlomieju/postgres";

const client = new Client({
  user: "frank",
  database: "testo",
  hostname: "database", // docker-compose change
  port: 5432,
  password: "postgres",
});

async function insert_ipfs_data(tokenId: string, name: string, uri: string) {
  await client.connect();
  await client.queryArray(
    `INSERT INTO ipfs_data (tokenId, name, uri) VALUES ('${tokenId}','${name}', '${uri}');`, // TODO: fix sql injection vulnerability!!
  );
  client.end();
}
async function get_ipfs_data() {
  await client.connect();
  const result = await client.queryArray("SELECT * FROM ipfs_data");
  client.end();
  return result;
}

export { get_ipfs_data, insert_ipfs_data };
