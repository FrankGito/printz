# Backend

This is the Backend Folder.  

## Tech 
The Backend uses Oak, Deno, Ipfs and Postgres.  

## Files
`./main.ts` - contains the api service. Route `/upload` can be seen as the main.  
`./postgres.ts` - contains Database Write and Read Functionality.  
`./ipfs.ts` - contains Ipfs Functionality.  
`./uuid.ts` - tiny utility  

## Working  
```
title Sequence Diagram

Client -> Api(/upload): sends File
Api(/upload) -> Folder(./uploads): safes file
Api(/upload) -> IpfsNode: ipfs add <file>
Api(/upload) -> Database: safes tokenId, name, cid
Api(/upload) -> Client: its done
```

## Story of Creation
```
$ git log --oneline --reverse

---Start with deno, jsr and oak---
df9e95a :zap: (backend) gotcha, post cube.glb sucessfully
e1f0e51 :zap: (backend) add getGlb route to api
22abdc5 :zap: (backend) intialise ipfs script
---Interact with ipfs daemon---
0a426aa :zap: (backend) write 'ipfs add cube.glb' in runtime equivelent
5018353 :zap: (backend) impl generateIpfsHash in api
---Interact with Postgres daemon---
a80097b :zap: (backend) intialise postrgres
a66e875 :zap: (backend) add postIpfsData to safe to postgres
84bdc48 :zap: (backend) add upload route, add incremational saving
---Add Postgres, Ipfs and File in one Api Call---
ad905b0 :zap: (backend) add & adjust routes
4ddf09d :zap: (backend) craft getAllUploads response
b8da1f8 :zap: (backend) adjust main.ts for recordable video
19c58c0 :abc: (backend) add README.md
```
