// Fonctions pures d'affichage : pas d'accès au réseau ni à la page, donc faciles à tester.

/** Part du prix demandée en acompte à la réservation (identique à l'API). */
export const DEPOSIT_PERCENT = 30;

/** 120000 → « 1 200,00 € » */
export function formatPrice(cents: number): string {
  return new Intl.NumberFormat("fr-FR", { style: "currency", currency: "EUR" }).format(cents / 100);
}

/** Acompte en centimes, arrondi à l'entier inférieur comme côté API. */
export function depositCents(priceCents: number): number {
  return Math.floor((priceCents * DEPOSIT_PERCENT) / 100);
}

/** « 2027-01-22 » → « vendredi 22 janvier 2027 » */
export function formatDate(isoDate: string): string {
  const [year, month, day] = isoDate.split("-").map(Number);
  return new Intl.DateTimeFormat("fr-FR", { weekday: "long", day: "numeric", month: "long", year: "numeric", timeZone: "UTC" })
    .format(new Date(Date.UTC(year, month - 1, day)));
}
