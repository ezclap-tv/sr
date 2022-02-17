type Method = "GET" | "POST" | "PUT" | "DELETE" | "PATCH";
type Headers = Record<string, string>;
type Body = Record<string, any>;
type Params = Record<string, any>;

export type Response<T> = T extends void ? { type: "sucess" } : { type: "success"; data: T };

/**
 * Helper function to send a request
 *
 * - Any search params in `endpoint` are discarded (use `params` instead)
 * - In case of a GET request, `body` is ignored
 * - The request `body` is stringified using `JSON.stringify`
 * - Default timeout is 10s
 * - keys with values equal to `null` or `undefined` are filtered from the object (deeply)
 */
export async function send<T = void>(
  method: Method,
  uri: string,
  params: Params | null = null,
  headers: Headers | null = null,
  body: Body | null = null,
  timeout: number = 10000 /* ms */
): Promise<Response<T>> {
  const controller = new AbortController();
  setTimeout(() => controller.abort(), timeout);

  const url = new URL(uri);
  url.search = "";
  if (params) url.search = "?" + new URLSearchParams(params).toString();

  const init: RequestInit = {
    method: method,
    signal: controller.signal,
    mode: "cors",
  };
  if (headers) init.headers = headers;
  if (method !== "GET" && body) {
    init.body = JSON.stringify(body);
    init.headers = { ...init.headers, "Content-Type": "application/json" };
  }

  try {
    const response = await fetch(url.toString(), init);
    const body = await response.text();
    const data = body.length > 0 ? JSON.parse(body) : null;
    if (response.ok) {
      return {
        type: "success",
        ...(data && { data }),
      };
    } else {
      throw {
        type: "error",
        status: response.status,
        ...(data && { message: data.message ?? "Unknown error" }),
      };
    }
  } catch (error) {
    throw {
      type: "error",
      status: 500,
      message: "Could not reach server",
    };
  }
}

export async function get<T = void>(
  uri: string,
  params: Params | null = null,
  headers: Headers | null = null,
  timeout: number = 10000 /* ms */
): Promise<Response<T>> {
  return await send("GET", uri, params, headers, null, timeout);
}

export async function post<T = void>(
  uri: string,
  params: Params | null = null,
  headers: Headers | null = null,
  body: Body | null = null,
  timeout: number = 10000 /* ms */
): Promise<Response<T>> {
  return await send("POST", uri, params, headers, body, timeout);
}

export namespace v1 {
  export const base = import.meta.env.VITE_API_URL + "/v1";

  const platforms = ["youtube"] as const;
  export type Platform = typeof platforms[number];
  export function isPlatform(v: string): v is Platform {
    return platforms.includes(v as any);
  }

  export async function memo(platform: Platform, id: string) {
    return await post(base + "/memo", null, null, { platform, id });
  }

  export type Song = { id: string; title: string };
  export async function playlist(
    platform: Platform,
    id: string,
    offset: number,
    limit: number
  ): Promise<Response<Song[]>> {
    return await get(base + "/playlist", { platform, id, offset, limit }, null);
  }

  //export async function random() {}
}
