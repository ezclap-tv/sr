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

  videoEnd: void;
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
              this.emit("state", data);
              const info = (target as any).playerInfo;
              if (data === YT.PlayerState.ENDED || (data == null && info.duration - info.currentTime < 0.5)) {
                this.emit("videoEnd");
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

  playing() {
    return [/* buffering */ 3, /* paused */ 2, /* playing */ 1].includes(this.instance.getPlayerState());
  }

  play(id?: string) {
    if (!this.loaded()) return;
    if (id) {
      this.instance.loadVideoById(id);
    } else {
      this.instance.playVideo();
    }
  }

  pause() {
    if (!this.loaded()) return;
    this.instance.pauseVideo();
  }

  stop() {
    if (!this.loaded()) return;
    this.instance.stopVideo();
  }
}
