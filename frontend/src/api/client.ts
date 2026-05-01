import { NotFoundError } from './errors';

export type PaginationParams = {
  page?: number;
  limit?: number;
};

const API_BASE_PATH = process.env.REACT_APP_API_URL ?? '/api';

function apiUrl(path: string, params?: Record<string, string | undefined>): URL {
  const url = new URL(API_BASE_PATH + path, window.location.origin);
  for (const [k, v] of Object.entries(params ?? {})) {
    if (v != null) url.searchParams.set(k, v);
  }
  return url;
}

function assertOk(response: Response, method: string, path: string): void {
  if (response.status === 404) throw new NotFoundError();
  if (!response.ok)
    throw new Error(`${method} ${path} failed: ${response.status} ${response.statusText}`);
}

export async function get(
  path: string,
  params?: Record<string, string | undefined>,
): Promise<Response> {
  const response = await fetch(apiUrl(path, params), { credentials: 'include' });
  assertOk(response, 'GET', path);
  return response;
}

export async function postBinary(
  path: string,
  body: ArrayBuffer,
  params?: Record<string, string | undefined>,
): Promise<Response> {
  const response = await fetch(apiUrl(path, params), {
    method: 'POST',
    headers: { 'Content-Type': 'application/octet-stream' },
    body,
    credentials: 'include',
  });
  assertOk(response, 'POST', path);
  return response;
}

export async function putBinary(
  path: string,
  body: ArrayBuffer,
  params?: Record<string, string | undefined>,
): Promise<Response> {
  const response = await fetch(apiUrl(path, params), {
    method: 'PUT',
    headers: { 'Content-Type': 'application/octet-stream' },
    body,
    credentials: 'include',
  });
  assertOk(response, 'PUT', path);
  return response;
}

export async function del(
  path: string,
  params?: Record<string, string | undefined>,
): Promise<Response> {
  const response = await fetch(apiUrl(path, params), { method: 'DELETE', credentials: 'include' });
  assertOk(response, 'DELETE', path);
  return response;
}

export async function post(path: string, body?: unknown): Promise<Response> {
  const headers: Record<string, string> = {};
  if (body !== undefined) {
    headers['Content-Type'] = 'application/json; charset=utf-8';
  }
  const response = await fetch(apiUrl(path), {
    method: 'POST',
    headers,
    body: body !== undefined ? JSON.stringify(body) : undefined,
    credentials: 'include',
  });
  assertOk(response, 'POST', path);
  return response;
}

export async function patch(path: string, body?: unknown): Promise<Response> {
  const headers: Record<string, string> = {};
  if (body !== undefined) {
    headers['Content-Type'] = 'application/json; charset=utf-8';
  }
  const response = await fetch(apiUrl(path), {
    method: 'PATCH',
    headers,
    body: body !== undefined ? JSON.stringify(body) : undefined,
    credentials: 'include',
  });
  assertOk(response, 'PATCH', path);
  return response;
}
