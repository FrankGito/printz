# Frontend

This is the Frontend Folder.

## Tech 
The Frontend uses PolkadotJS for the contract calls.   
The Frontend uses Vue & vue-router for reactivity & client side routing.
The Frontend uses TresJs for 3D Experience.
The Frontend uses Leches for DebugUi.

## Files
We are using hooks to manage api, debugUi and contracts.  
`./src/composables/useApi.ts` - contains the Api Handling
`./src/composables/usePsp34.ts` - contains the Contract Handling
`./src/composables/useLeches.ts` - contains the DebugUi.

We have 2 main Pages. HomeView and AboutView.
`./src/components/HomeView.vue` Contains 3D Object, Upload Interaction and the mint functionality.
`./src/components/AboutView.vue` Contains some parsed Markdown.

## Working
```
title Sequence Diagram

User -> WebApp: uploads .glb Model
WebApp -> Api: sends file to server
WebApp -> SmartContract: mints & setAttribute(id, ipfsHash)
SmartContract -> User: declares Ownership
WebApp -> WebApp: rerender
WebApp -> User: displays newly minted Model
```

## Story of Creation

```
$ git log --oneline --reverse

---Start with pnpm---
098fc72 :zap: (frontend) pnpm create vite@latest --template vue
a039fad :zap: (frontend) clean up
---Do Simple ThreeJS Scene---
a9fd9bd :zap: (frontend) add three, add button, add card, does the job right now
c8e78c1 :zap: (frontend) add leches, setup leches
b9faa24 :zap: (frontend) initialise usePsp34
---Switch to Deno---
84ea27e :zap: (frontend) transition to deno
ba3ed40 :zap: (frontend) modify imports for deno, make call work
---Able to send String to Ipfs---
ccd6851 :zap: (frontend) get ipfs helia running :dance:
---Not able to send Blob to Ipfs---
46c6fb5 :zap: (frontend) get rid of ipfs helia, change stragedy
c172f35 :zap: (frontend) make deno work :neutral_face:
---Switch to CSS Framework---
0e92428 :zap: (frontend) add bootswatch-sketchy, add nav, add card, remove old css
c0ac20b :zap: (frontend) adjust frontend
6f7d246 :zap: (frontend) nice card with 2 buttons, glorious :party:
---Switch to ThreeJS Wrapper---
878d282 :zap: (frontend) add tresJS create scene, throw things in cards
7f3e233 :zap: (frontend) add getIpfsHash Button
e32c1eb :zap: (frontend) add getTotalSupply Button
40fff5f :zap: (frontend) add mint button, make mint work
---Struggle with Types and PolkadotJS---
6d74697 :zap: (frontend) call mint correct via Id::U8(x) type
---Doing Hex Value Transformation, super wild call ^^'---
25bf991 :zap: (frontend) make set_attribute callable, send hex over the wire :sweat_smiley:
7fea844 :zap: (frontend) make set_attribute work with 'uri' key
58f247d :zap: (frontend) tiny cleanup, add useApi.ts
bbd00d0 :zap: (frontend) add cientos
---Refactor---
b7a0ed6 :zap: (frontend) create Canvas.vue & extract Leches to useLeches.t
2d024bb :zap: (frontend) unbroke it ^^ :hands_up:
85d595e :zap: (frontend) throw everyting in one mint o.O :thumbs_up:
6338034 :zap: (frontend) add router for navigation, implent it
77e076f :zap: (frontend) add marked, add about.md in public, make it render
32fc845 :zap: (frontend) add model select and change icon to crown
eeadd53 :zap: (frontend) adjust select Models statement
435d054 :zap: (frontend) write About.md
1f4c39b :zap: (frontend) add OffersView, adjust useApi, make Upload/Mint logic working
c2ab8b8 :zap: (frontend) adjust Navbar
---Do a record of Pitch Video---
5462e66 :zap: (frontend) adjust calls for recordable video
```
