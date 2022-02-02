import { EventEmitter, nonce } from "./util";

declare global {
  interface Window {
    onYouTubeIframeAPIReady(): void;
  }
}

let ready = Promise.flat<void>();
window.onYouTubeIframeAPIReady = () => ready.resolve();

const stringifyState = (s: YT.PlayerState) => {
  switch (s) {
    case YT.PlayerState.UNSTARTED:
      return "unstarted";
    case YT.PlayerState.ENDED:
      return "ended";
    case YT.PlayerState.PLAYING:
      return "playing";
    case YT.PlayerState.PAUSED:
      return "paused";
    case YT.PlayerState.BUFFERING:
      return "buffering";
    case YT.PlayerState.CUED:
      return "cued";
  }
};

const stringifyError = (e: YT.PlayerError) => {
  switch (e) {
    case YT.PlayerError.InvalidParam:
      return "invalid param";
    case YT.PlayerError.Html5Error:
      return "html5 error";
    case YT.PlayerError.VideoNotFound:
      return "video not found";
    case YT.PlayerError.EmbeddingNotAllowed:
      return "embedding not allowed";
    case YT.PlayerError.EmbeddingNotAllowed2:
      return "embedding not allowed";
  }
};

interface PlayerEvents {
  ready: void;
  state: ReturnType<typeof stringifyState>;
  quality: string;
  rate: number;
  error: string;
  api: void;
}

const temp = () => {
  const el = document.createElement("div");
  //el.style.display = "none";
  el.id = nonce();
  return el;
};

const createPlayer = async (
  extra?: { type: "list"; list: string } | { type: "video"; id: string }
): Promise<YT.Player> => {
  const el = temp();
  const id = el.id;
  document.body.appendChild(el);

  let ready = Promise.flat<void>();
  const instance = new YT.Player(id, {
    ...(extra && extra.type === "video" ? { videoId: extra.id } : {}),
    playerVars: {
      controls: 1,
      enablejsapi: 1,
      ...(extra && extra.type === "list" ? { listType: "playlist", list: extra.list } : {}),
    },
    events: {
      onReady: () => ready.resolve(),
    },
  });

  await ready.promise;
  return instance;
};

export class Player extends EventEmitter<PlayerEvents> {
  playlist: string[];

  private constructor(readonly display: YT.Player, readonly scraper: YT.Player) {
    super();
    this.playlist = [];
  }

  static async create() {
    await ready.promise;
    const display = await createPlayer();
    const scraper = await createPlayer();
    document.body.appendChild(scraper.getIframe());

    return new Player(display, scraper);
  }

  appendTo(node: Node) {
    this.display.getIframe().remove();
    node.appendChild(this.display.getIframe());
  }

  play() {
    this.display.playVideo();
  }

  pause() {
    this.display.pauseVideo();
  }

  async getPlaylistIds(id: string): Promise<string[]> {
    let ready = Promise.flat<void>();
    {
      let onStateChange: (e: YT.OnStateChangeEvent) => void, onError: (e: YT.OnErrorEvent) => void;
      this.scraper.addEventListener(
        "onStateChange",
        (onStateChange = ({ data }: YT.OnStateChangeEvent) => {
          console.log("yo");
          if (data === YT.PlayerState.CUED) {
            this.scraper.removeEventListener("onStateChange", onStateChange);
            ready.resolve();
          }
        })
      );
      this.scraper.addEventListener(
        "onError",
        (onError = ({ data }: YT.OnErrorEvent) => {
          console.log("error");
          this.scraper.removeEventListener("onError", onError);
          ready.reject(data);
        })
      );
    }
    this.scraper.cuePlaylist({ listType: "playlist", list: id });
    await ready.promise;
    return this.scraper.getPlaylist();
  }
}
