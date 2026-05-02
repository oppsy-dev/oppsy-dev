import { UnprocessableContent, NotFoundError } from './errors';

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

async function assertOk(response: Response, method: string, path: string): Promise<void> {
  if (response.status === 404) throw new NotFoundError();
  if (response.status === 422) {
    let message = 'Unprocessable content';
    try {
      const body: unknown = await response.json();
      if (typeof body === 'string' && body.length > 0) message = body;
    } catch {}
    throw new UnprocessableContent(message);
  }
  if (!response.ok)
    throw new Error(`${method} ${path} failed: ${response.status} ${response.statusText}`);
}

export async function get(
  path: string,
  params?: Record<string, string | undefined>,
): Promise<Response> {
  const response = await fetch(apiUrl(path, params), { credentials: 'include' });
  await assertOk(response, 'GET', path);
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
  await assertOk(response, 'POST', path);
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
  await assertOk(response, 'PUT', path);
  return response;
}

export async function del(
  path: string,
  params?: Record<string, string | undefined>,
): Promise<Response> {
  const response = await fetch(apiUrl(path, params), { method: 'DELETE', credentials: 'include' });
  await assertOk(response, 'DELETE', path);
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
  await assertOk(response, 'POST', path);
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
  await assertOk(response, 'PATCH', path);
  return response;
}
