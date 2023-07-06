import got, { OptionsOfJSONResponseBody, RequestError, Response } from "got";
import { CookieJar } from "./cookieJar";
import { AptosClientRequest, AptosClientResponse } from "./types";

const cookieJar = new CookieJar();

export default async function aptosClient<Res>(requestOptions: AptosClientRequest): Promise<AptosClientResponse<Res>> {
  const { params, method, url, headers, body } = requestOptions;

  const request: OptionsOfJSONResponseBody = {
    http2: true,
    searchParams: convertBigIntToNumber(params),
    method,
    url,
    responseType: "json",
    headers,
    hooks: {
      beforeRequest: [
        (options) => {
          const cookies = cookieJar.getCookies(new URL(options.url!));

          if (cookies?.length > 0 && options.headers) {
            /* eslint-disable no-param-reassign */
            options.headers.cookie = cookies.map((cookie: any) => `${cookie.name}=${cookie.value}`).join("; ");
          }
        },
      ],
      afterResponse: [
        (response) => {
          if (Array.isArray(response.headers["set-cookie"])) {
            response.headers["set-cookie"].forEach((c) => {
              cookieJar.setCookie(new URL(response.url!), c);
            });
          }
          return response;
        },
      ],
    },
  };

  if (body) {
    if (body instanceof Uint8Array) {
      request.body = Buffer.from(body);
    } else {
      request.body = Buffer.from(JSON.stringify(body));
    }
  }

  try {
    const response = await got<Res>(request);
    parseHeaders<Res>(response);
    return parseResponse<Res>(response);
  } catch (error) {
    const gotError = error as RequestError;
    if (gotError.response) {
      return parseResponse<Res>(gotError.response as Response<Res>);
    }
    throw error;
  }
}

function parseResponse<Res>(response: Response<Res>): AptosClientResponse<Res> {
  return {
    status: response.statusCode,
    statusText: response.statusMessage || "",
    data: response.body,
    config: response.request.options,
    request: response.request,
    response,
    headers: response.headers,
  };
}

function parseHeaders<Res>(response: Response<Res>) {
  // capitalize authorization key to be compatible with axios response
  if (response.request.options.headers.authorization) {
    response.request.options.headers.Authorization = response.request.options.headers.authorization;
    delete response.request.options.headers.authorization;
  }
  // capitalize content-type key to be compatible with axios response
  if (response.request.options.headers["content-type"]) {
    response.request.options.headers["Content-Type"] = response.request.options.headers["content-type"];
    delete response.request.options.headers["content-type"];
  }
}

/**
 * got supports only - string | number | boolean | null | undefined
 * in the searchParam props,
 * so if we have bigint type, convert it to Number
 */
function convertBigIntToNumber(obj: any): any {
  const result: any = {};
  if (!obj) return result;

  Object.entries(obj).forEach(([key, value]) => {
    if (Object.prototype.hasOwnProperty.call(obj, key)) {
      if (typeof value === "bigint") {
        result[key] = Number(value);
      } else {
        result[key] = value;
      }
    }
  });

  return result;
}
