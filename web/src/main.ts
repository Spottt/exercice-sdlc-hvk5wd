import { book, getShow, getStands, type Stand } from "./api";
import { depositCents, formatDate, formatPrice } from "./format";

const $ = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
const message = $("message");
const exhibitor = $<HTMLInputElement>("exhibitor");

function row(stand: Stand): HTMLTableRowElement {
  const tr = document.createElement("tr");
  const cell = (text: string, className = "") => {
    const td = document.createElement("td");
    td.textContent = text;
    td.className = className;
    tr.append(td);
    return td;
  };
  cell(stand.label);
  cell(`${stand.area_m2} m²`, "num");
  cell(formatPrice(stand.price_cents), "num");
  cell(formatPrice(depositCents(stand.price_cents)), "num");
  const action = cell("");
  if (stand.booked_by) {
    action.textContent = `Réservé par ${stand.booked_by}`;
    action.className = "muted";
  } else {
    const button = document.createElement("button");
    button.textContent = "Réserver";
    button.addEventListener("click", () => reserve(stand));
    action.append(button);
  }
  return tr;
}

async function reserve(stand: Stand) {
  message.textContent = "";
  try {
    await book(stand.id, exhibitor.value);
    await refresh();
  } catch (e) {
    message.textContent = (e as Error).message;
  }
}

async function refresh() {
  $("stands").replaceChildren(...(await getStands()).map(row));
}

async function start() {
  try {
    const show = await getShow();
    $("show-name").textContent = show.name;
    $("show-date").textContent = `Ouverture le ${formatDate(show.opens_on)}`;
    await refresh();
  } catch (e) {
    message.textContent = `API injoignable : ${(e as Error).message}`;
  }
}

start();
