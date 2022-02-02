import "./style.css";
import "./util";
import { Player } from "./player";

const player = await Player.create();
console.log("player loaded");
// @ts-ignore
window._player = player;
