# Backend

This is the Backend Folder.  
The Backend uses Oak, Deno, Ipfs and Postgres.  

`./main.ts` - contains the api service. Route `/upload` can be seen as the main.  

`./postgres.ts` - contains Database Write and Read Functionality.

`./ipfs.ts` - contains Ipfs Functionality.

`./uuid.ts` - tiny utility

**Sequence Diagram:**  
```
Client -> Api(/upload): sends File
Api(/upload) -> Folder(./uploads): safes file
Api(/upload) -> IpfsNode: ipfs add <file>
Api(/upload) -> Database: safes tokenId, name, cid
Api(/upload) -> Client: its done
```
