<div align="center">
<img src="https://i.ibb.co/RbwMzcv/lucide-crown.png" alt="logo" width="120" height="120" />
</div>

<h3 align="center">Printz</h3>
  <p align="center">
  <br />
    <a href="http://printz.surge.sh">Demo v0.0.1</a>
    ·
    <a href="https://youtu.be/QBlTSIQg1gU">Youtube</a>
    ·
    <a href="https://github.com/frankgito/printz">Repo</a>
  </p>
</div>

## Screenshot

![Screenshot](https://i.ibb.co/6mQ1jyz/image.png)

## Summary 

We are Mark and Frank.  
We created **Printz**. 
Printz is a decentralised 3D Printing Service.  

### Deep Dive

- [About Print](./frontend/public/About.md)
- [UiUx README.md](./uiux/README.md)

- [Backend README.md](./Backend/README.md)
- [Contracts README.md](./contracts/README.md)
- [Frontend README.md](./frontend/README.md)

## Quickstart

- `cd backend && sudo systemctl start postgresql && ipfs daemon && deno run -A main.ts`
- `cd contracts/item && cargo contract build && substrate-contracts-node --dev && cargo contract instantiate --suri //Alice --args 0 --execute`
- copy contract address and over-paste it in `./frontend/src/composables/usePsp34.ts`
- `cd frontend && deno task dev`

## Contacts

| Name  | Telegram  | Discord    |
| :---- | :-------- | :--------- |
| Frank | frankbevr | frankbevr  |
| Mark  | proxy720  | proxy_0441 |
