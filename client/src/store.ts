import { writable } from "svelte/store";
import type { Message } from "./utils/message";
import ReconnectingWebSocket from "reconnecting-websocket";

const user = writable(null);
const messageStore = writable({
  message: "",
});

const url = "ws://localhost:3030/ws";

const socket = new ReconnectingWebSocket(url);

socket.addEventListener("open", (event) => console.log("It's open!!!"));

socket.addEventListener("message", ({ data }) => {
  let message: Message = JSON.parse(data);
  messageStore.set(message);
});

const sendMessage = (message) => {
  if (socket.readyState <= 1) {
    socket.send(message);
  }
};

export { user };
export default {
  subscribe: messageStore.subscribe,
  sendMessage,
};
