import { describe, expect, it } from "vitest";
import { depositCents, formatDate, formatPrice } from "./format";

// Les espaces insécables de Intl sont remplacés par des espaces simples pour comparer.
const plain = (s: string) => s.replace(/[  ]/g, " ");

describe("affichage", () => {
  it("formate un prix en euros", () => {
    expect(plain(formatPrice(120_000))).toBe("1 200,00 €");
  });

  it("calcule un acompte de 30 %", () => {
    expect(depositCents(120_000)).toBe(36_000);
    expect(depositCents(165_001)).toBe(49_500);
  });

  it("écrit la date d'ouverture en toutes lettres", () => {
    expect(formatDate("2027-01-22")).toBe("vendredi 22 janvier 2027");
  });
});
