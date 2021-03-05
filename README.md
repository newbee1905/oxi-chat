# oxi-chat

## Table of contents
* [General info](#general-info)
* [Current functions](#current-functions)
* [Todo](#todo)

## General Info
This project is a simple chat application written with warp-rs, svelte and postgres.

## Requirement
* rust
* llvm/clang
* nodejs
* postgres
* ngrok <span style="color: grey; font-style: italic">(optional)</span>

## Setup
* Set up your postgresql configuration in ```.env```
* Add your admin's username and password, default it will be admin and 1234

Example of using ngrok as host
* Run ```ngrok http 3030``` then copy the ngrok link
* Copy the ngrok link then paste it into ```client/src/config.ts```
* Example: ```https://6108c614fd28.ngrok.io``` to ```export let url = new URL("https://6108c614fd28.ngrok.io")``` inside ```client/src/config.ts```
* Run ```npm install && npm run build``` inside ```client```
* Then run ```cargo run``` inside ```server```


## Current functions
* Encrypted user's info with argon2
* Register and Login
* Send message through websocket

## Todo
- [ ] Improve frontend
- [ ] Add Dockerfile
- [ ] End-to-end encryption messaging
