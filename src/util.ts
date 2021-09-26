declare global {
  interface ObjectConstructor {
    filter<T, Keys extends (keyof T)[]>(
      object: T,
      keys: Keys
    ): Omit<T, Keys[number]>;
  }
}

Object.filter = function (object, keys) {
  const out = { ...object };
  for (let i = 0; i < keys.length; ++i) {
    delete out[keys[i]];
  }
  return out;
};

export {};
