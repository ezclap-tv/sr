/**
 * Generates a random string of `length`.
 */
export const nonce = (length = 32) =>
  [...crypto.getRandomValues(new Uint8Array(length / 2))].map((v) => v.toString(16).padStart(2, "0")).join("");

declare global {
  interface FlatPromise<T> {
    promise: Promise<T>;
    resolve(value: T | PromiseLike<T>): void;
    reject(reason?: any): void;
  }
  interface PromiseConstructor {
    flat<T = unknown>(): FlatPromise<T>;
  }
}

Promise.flat = function <T = unknown>() {
  let resolve!: (value: T | PromiseLike<T>) => void;
  let reject!: (reason?: any) => void;
  let promise = new Promise<T>((_resolve, _reject) => {
    resolve = _resolve;
    reject = _reject;
  });
  return { promise, resolve, reject };
};

type Remove<From, What> = From extends Record<string, any>
  ? { [P in keyof From as From[P] extends What ? never : P]: From[P] }
  : From;

export class EventEmitter<
  Events extends Record<string, any>,
  WithValue = Remove<Events, void>,
  NoValue = Omit<Events, keyof WithValue>
> {
  callbacks: { [P in keyof WithValue]?: (v: WithValue[P]) => void } & { [P in keyof NoValue]?: () => void } = {};
  emit<E extends keyof WithValue>(event: E, value: WithValue[E]): void;
  emit<E extends keyof NoValue>(event: E): void;
  emit(event: string, value?: any) {
    const callbacks = this.callbacks as any;
    if (value) callbacks[event]?.forEach((f: any) => f(value));
    else callbacks[event]?.forEach((f: any) => f());
  }

  on<E extends keyof WithValue>(event: E, callback: (value: WithValue[E]) => void, options?: { once?: boolean }): void;
  on<E extends keyof NoValue>(event: E, callback: () => void, options?: { once?: boolean }): void;
  on(event: string, callback: any, options = { once: false }) {
    if (options.once) {
      let orig = callback;
      const fn = () => {
        orig();
        this.off(event as any, fn);
      };
      callback = fn;
    }
    const callbacks = this.callbacks as any;
    const set = callbacks[event] ?? new Set();
    set.add(callback);
    callbacks[event] = set;
  }

  off<E extends keyof WithValue>(event: E, callback: (value: WithValue[E]) => void): void;
  off<E extends keyof NoValue>(event: E, callback: () => void): void;
  off(event: string, callback: any) {
    const callbacks = this.callbacks as any;
    callbacks[event]?.delete(callback);
  }
}
