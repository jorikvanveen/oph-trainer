import type { ApiErrorDto } from '#lib/dto/ApiErrorDto.ts';

export async function api<T>(path: string, accountKey = '', method = 'GET', body?: unknown): Promise<T> {
  const headers: Record<string, string> = {};
  if (accountKey) headers.Authorization = `Bearer ${accountKey}`;
  if (body !== undefined) headers['Content-Type'] = 'application/json';
  const response = await fetch(`/api${path}`, {
    method,
    headers,
    body: body === undefined ? undefined : JSON.stringify(body)
  });
  if (!response.ok) {
    const error: ApiErrorDto | null = await response.json().catch(() => null);
    throw new Error(error?.message ?? `Request failed with status ${response.status}. Try again.`);
  }
  return response.status === 204 ? undefined as T : await response.json() as T;
}

export function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : 'Could not reach the server. Try again.';
}
