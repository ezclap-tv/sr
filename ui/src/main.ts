import "./style.css";
import "./util";
import { Player } from "./player";

const player = new Player();
player.on("load", () => {
  if (!player.loaded()) return;
  player.load("lqW3k6UwyDQ");
  player.on("state", (s) => s === 5 && player.play(), { once: true });
});
player.on("state", (d) => console.log("state", d));
player.on("quality", (d) => console.log("quality", d));
player.on("rate", (d) => console.log("rate", d));
player.on("error", (d) => console.log("error", d));
player.on("apichanged", () => console.log("apichanged"));

// @ts-ignore
window._player = player;
