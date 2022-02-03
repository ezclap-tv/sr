import { EventEmitter, nonce } from "./util";

declare global {
  interface Window {
    onYouTubeIframeAPIReady(): void;
  }
}

let ready = Promise.flat<void>();
window.onYouTubeIframeAPIReady = () => ready.resolve();

interface PlayerEvents {
  load: void;
  state: YT.PlayerState;
  quality: string;
  rate: number;
  error: YT.PlayerError;
  apichanged: void;
}

export class Player extends EventEmitter<PlayerEvents> {
  playlist: string[];
  instance!: YT.Player;

  private _loaded: boolean = false;

  constructor(parent: HTMLElement = document.body) {
    super();
    this.playlist = [];

    const el = document.createElement("div");
    el.id = nonce();
    parent.appendChild(el);

    (async () => {
      await ready.promise;
      this.instance = new YT.Player(el.id, {
        width: "100%",
        height: "100%",
        playerVars: {
          controls: 1,
          enablejsapi: 1,
          origin: window.location.origin,
        },
        events: {
          onReady: ({ target }) => {
            if (this._loaded) return;
            target.addEventListener<YT.OnStateChangeEvent>("onStateChange", ({ target, data }) => {
              if (data == null) {
                // video stopped working due to buffering failing
                // or something along those lines, just restart it 4Head
                const id = new URL(target.getVideoUrl()).searchParams.get("v");
                if (id) {
                  console.log("reloading", id);
                  target.loadVideoById(id);
                }
              } else {
                this.emit("state", data);
              }
            });
            target.addEventListener<YT.OnPlaybackQualityChangeEvent>("onPlaybackQualityChange", ({ data }) =>
              this.emit("quality", data)
            );
            target.addEventListener<YT.OnPlaybackRateChangeEvent>("onPlaybackRateChange", ({ data }) =>
              this.emit("rate", data)
            );
            target.addEventListener<YT.OnErrorEvent>("onError", ({ data }) => this.emit("error", data));
            target.addEventListener("onApiChange", () => this.emit("apichanged"));
            this._loaded = true;
            setTimeout(() => this.emit("load"), 0);
          },
        },
      });
    })();
  }

  loaded(): boolean {
    return this._loaded;
  }

  remove() {
    if (!this.loaded()) return;
    const el = this.instance.getIframe();
    el.remove();
  }

  appendTo(node: Node) {
    if (!this.loaded()) return;
    const el = this.instance.getIframe();
    el.remove();
    node.appendChild(this.instance.getIframe());
  }

  load(id: string) {
    if (!this.loaded()) return;
    this.instance.loadVideoById(id);
  }

  play() {
    if (!this.loaded()) return;
    this.instance.playVideo();
  }

  pause() {
    if (!this.loaded()) return;
    this.instance.pauseVideo();
  }
}
