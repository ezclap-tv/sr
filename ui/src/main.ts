import "./style.css";
import "./util";
import * as api from "./api";
import { Player } from "./player";
import { Playlist } from "./playlist";
import { Channel, Role } from "./twitch";
import { CommandRegistry } from "./command";

// TODO: support multiple players dynamically based on playlist specified platform
const youtube = new Player();
const playlist = new Playlist();

youtube.on("videoEnd", () => {
  console.log("go next (end)");
  const next = playlist.next();
  if (!next) {
    youtube.stop();
  } else {
    youtube.play(next.id);
  }
});
youtube.on("error", () => {
  console.log("go next (error)");
  const next = playlist.next();
  if (!next) {
    youtube.stop();
  } else {
    youtube.play(next.id);
  }
});

function getListId(id: string) {
  console.log(id);
  try {
    const url = new URL(id);
    return url.pathname === "/playlist" ? url.searchParams.get("list") : null;
  } catch (e) {
    console.error(e);
    return id;
  }
}
function getSongId(id: string) {
  try {
    const url = new URL(id);
    return (url.pathname === "/watch" ? url.searchParams.get("v") : url.pathname) || null;
  } catch (e) {
    console.error(e);
    return id;
  }
}

const registry = new CommandRegistry("$sr");

registry.add("default", {
  allow: Role.Broadcaster,
  async run(_user: string, rawId: string) {
    const id = getSongId(rawId);
    console.log("parsed id", id);
    if (!id) return;

    await playlist.add("song", "youtube", id);
    if (!youtube.playing()) {
      youtube.play(playlist.next()!.id);
    }
  },
});

async function list(platform: api.v1.Platform, rawId: string, rawCount: string) {
  const count = parseInt(rawCount, 10);
  if (Number.isNaN(count)) return;

  const id = getListId(rawId);
  console.log("parsed id", id);
  if (!id) return;
  if (!api.v1.isPlatform(platform)) return;

  await playlist.add("list", platform, id, count);
  if (!youtube.playing()) {
    youtube.play(playlist.next()!.id);
  }
}
registry.add("list", {
  allow: Role.Broadcaster,
  run: async (_user, rawId, rawCount: string = "10") => list("youtube", rawId, rawCount),
});
registry.add("list:youtube", {
  allow: Role.Broadcaster,
  run: async (_user, rawId, rawCount: string = "10") => list("youtube", rawId, rawCount),
});

registry.add("skip", {
  allow: Role.Moderator,
  run: async (_user) => {
    const next = playlist.skip();
    if (!next) return;
    youtube.play(next.id);
  },
});
registry.add("skip:list", {
  allow: Role.Moderator,
  run: async (_user) => {
    const next = playlist.skip(true);
    if (!next) {
      youtube.stop();
    } else {
      youtube.play(next.id);
    }
  },
});

const channel = new Channel("moscowwbish");
channel.onopen = () => console.log("connected");
channel.onmessage = (m) => registry.handle(m);

// @ts-ignore
window._app = { youtube, playlist, registry, channel };
