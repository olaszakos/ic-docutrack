import type { AuthClient } from "@dfinity/auth-client";
import { writable } from "svelte/store";
import { createActor } from "../../../../../declarations/backend";
import type { ActorType } from "../actor";

const canisterId = import.meta.env.VITE_BACKEND_CANISTER_ID;
const host = import.meta.env.VITE_HOST;

export const firstName = writable(null);
export const lastName = writable(null);
export const actor = writable<ActorType>(
  createActor(canisterId, { agentOptions: { host } })
);
export const authClient = writable<AuthClient | null>(null);
export const isAuthenticated = writable<boolean | null>(null);
