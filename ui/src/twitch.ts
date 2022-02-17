async function async_send(ws: WebSocket, msg: string): Promise<MessageEvent<any>> {
  return new Promise((resolve, reject) => {
    let onmsg = (e: MessageEvent<any>): void => (ws.removeEventListener("message", onmsg), resolve(e));
    let onerr = (e: Event): void => (ws.removeEventListener("error", onerr), reject(e));
    ws.addEventListener("message", onmsg);
    ws.addEventListener("error", onerr);
    ws.send(msg);
  });
}

export enum Role {
  User = 0,
  Subscriber = 1,
  VIP = 2,
  Moderator = 3,
  Broadcaster = 4,
}

export class Message {
  constructor(
    readonly user: string,
    readonly command: string,
    readonly channel: string,
    readonly text: string,
    readonly tags?: Record<string, string>
  ) {}

  displayName() {
    if (!this.tags) return null;
    if (!("user-id" in this.tags)) return null;
    return this.tags["display-name"].replaceAll("\\s", " ");
  }

  uid() {
    if (!this.tags) return null;
    if (!("user-id" in this.tags)) return null;
    return this.tags["user-id"];
  }

  time() {
    if (!this.tags) return null;
    if (!("tmi-sent-ts" in this.tags)) return null;
    return new Date(this.tags["tmi-sent-ts"]);
  }

  role() {
    if (!this.tags) return Role.User;
    switch (true) {
      case this.tags.badges.includes("broadcaster"):
        return Role.Broadcaster;
      case this.tags.badges.includes("moderator"):
        return Role.Moderator;
      case this.tags.badges.includes("vip"):
        return Role.VIP;
      case this.tags.badges.includes("founder") || this.tags.badges.includes("subscriber"):
        return Role.Subscriber;
      default:
        return Role.User;
    }
  }
}

function parse(msg: string): Message | null {
  let tags;
  let fullMsg = msg;
  if (msg.startsWith("@")) {
    const [tagsRaw, ...msgRaw] = msg.slice(1).split(" :");
    fullMsg = `:${msgRaw.join(" :")}`;
    tags = {} as Record<string, string>;
    for (const entry of tagsRaw.split(";")) {
      const [key, value] = entry.split("=");
      tags[key] = value;
    }
  }
  // TODO: better parsing for the message part
  const parts = /:((.*)!.*@.*.)?tmi.twitch.tv (.*) #(.*) :(.*)/.exec(fullMsg);
  if (!parts) return null;
  const [user, command, channel, text] = parts.slice(2) ?? [];
  return new Message(user, command, channel, text, tags);
}

export class Channel {
  socket!: WebSocket;
  constructor(readonly channel: string) {
    const onmessage = ({ data }: { data: string }) => {
      const message = parse(data);
      console.log("raw", data);
      console.log("parsed", message);
      if (!message) return;
      if (message.command === "PING") return this.socket.send("PONG :tmi.twitch.tv");
      this.onmessage(message);
    };
    const onclose = async () => {
      console.log("Disconnected, reconnecting...");
      this.socket = await connect(channel);
      this.socket.onclose = onclose;
      this.socket.onmessage = onmessage;
    };

    connect(channel).then((s) => {
      this.socket = s;
      this.socket.onclose = onclose;
      this.socket.onmessage = onmessage;
      this.onopen();
    });
  }

  onopen = () => {};
  onmessage = (message: Message) => {};
}

function connect(channel: string): Promise<WebSocket> {
  return new Promise<WebSocket>((resolve, reject) => {
    const url = `${window.location.protocol.startsWith("https") ? "wss" : "ws"}://irc-ws.chat.twitch.tv`;

    function retry(e: any) {
      if (e) console.error(e);
      console.warn("Failed to connect, retrying...");
      ws = new WebSocket(url);
      ws.onerror = retry;
      ws.onclose = retry;
      ws.onopen = onopen;
    }
    async function onopen() {
      await async_send(ws, "CAP REQ :twitch.tv/tags");
      let res = await async_send(ws, "NICK justinfan37982");
      if (res.data.startsWith(":tmi.twitch.tv 001")) {
        ws.send(`JOIN #${channel}`);
        ws.onerror = function () {};
        ws.onclose = function () {};
        ws.onopen = function () {};
        resolve(ws);
      } else {
        if (ws.readyState === WebSocket.OPEN) ws.close();
        reject(new Error("Failed to join"));
      }
    }

    let ws = new WebSocket(url);
    ws.onerror = retry;
    ws.onclose = retry;
    ws.onopen = onopen;
  });
}
