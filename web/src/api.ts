export interface Show {
  name: string;
  opens_on: string;
}

export interface Stand {
  id: number;
  label: string;
  area_m2: number;
  price_cents: number;
  booked_by: string | null;
}

export interface Booking {
  id: number;
  stand_id: number;
  exhibitor: string;
  deposit_cents: number;
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(path, { headers: { "content-type": "application/json" }, ...init });
  const body = await response.json().catch(() => null);
  if (!response.ok) throw new Error(body?.error ?? `Erreur ${response.status}`);
  return body as T;
}

export const getShow = () => request<Show>("/api/show");
export const getStands = () => request<Stand[]>("/api/stands");
export const book = (standId: number, exhibitor: string) =>
  request<Booking>("/api/bookings", { method: "POST", body: JSON.stringify({ stand_id: standId, exhibitor }) });
