import * as api from "./api";

type ListItem = {
  type: "list";
  platform: api.v1.Platform;
  id: string;
  songs: api.v1.Song[];
  cursor: number;
  count: number;
};
type SongItem = {
  type: "song";
  platform: api.v1.Platform;
  id: string;
};
type Item = ListItem | SongItem;

export class Playlist {
  items: Item[] = [];
  current: Item | null = null;

  next(): SongItem | null {
    this.current ??= this.items.shift() ?? null;
    const current = this.current;
    if (!current) return null;
    if (current.type === "song") {
      this.current = null;
      return current;
    } else {
      const id = current.songs[current.cursor]?.id ?? null;
      const item = id ? { type: "song" as const, platform: current.platform, id } : null;

      current.cursor += 1;
      const remainingLocal = current.songs.length - current.cursor;
      const remainingRemote = current.count - current.cursor;

      if (remainingRemote > 0) {
        if (remainingLocal <= 1) {
          const count = remainingRemote >= 10 ? 10 : remainingRemote;
          api.v1.playlist(current.platform, current.id, current.songs.length, count).then(({ data }) => {
            current.songs.push(...data);
          });
        }
      } else {
        this.current = null;
      }
      return item;
    }
  }

  skip(full: boolean = false): SongItem | null {
    if (!full) return this.next();
    else {
      if (this.current && this.current.type === "list") {
        this.current = null;
      }
      return this.next();
    }
  }

  async add(type: "list" | "song", platform: api.v1.Platform, id: string, count: number = 10): Promise<void> {
    if (type === "song") {
      await api.v1.memo(platform, id);
      this.items.push({ type, platform, id });
    } else {
      // get the backend to pre-fetch the playlist + fetch first few songs
      const response = await api.v1.playlist(platform, id, 0, count! < 10 ? count! : 10);
      this.items.push({ type, platform, id, songs: response.data, cursor: 0, count: count! });
    }
  }
}
