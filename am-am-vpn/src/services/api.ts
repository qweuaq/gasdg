import { invoke } from "@tauri-apps/api/core";
import type {
  ServerNode,
  Subscription,
  ConnectionState,
  AppSettings,
  LogEntry,
} from "../types";

// ─── Subscription ───

export async function addSubscription(url: string): Promise<Subscription> {
  return invoke<Subscription>("add_subscription", { url });
}

export async function refreshSubscription(id: string): Promise<Subscription> {
  return invoke<Subscription>("refresh_subscription", { id });
}

export async function removeSubscription(id: string): Promise<void> {
  return invoke<void>("remove_subscription", { id });
}

export async function listSubscriptions(): Promise<Subscription[]> {
  return invoke<Subscription[]>("list_subscriptions");
}

// ─── Servers ───

export async function listServers(): Promise<ServerNode[]> {
  return invoke<ServerNode[]>("list_servers");
}

export async function testLatency(serverId: string): Promise<number> {
  return invoke<number>("test_latency", { serverId });
}

export async function testAllLatencies(): Promise<Record<string, number>> {
  return invoke<Record<string, number>>("test_all_latencies");
}

// ─── Connection ───

export async function connect(serverId: string): Promise<void> {
  return invoke<void>("connect", { serverId });
}

export async function disconnect(): Promise<void> {
  return invoke<void>("disconnect");
}

export async function getConnectionState(): Promise<ConnectionState> {
  return invoke<ConnectionState>("get_connection_state");
}

export async function selectFastestServer(): Promise<string> {
  return invoke<string>("select_fastest_server");
}

// ─── Settings ───

export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

export async function updateSettings(settings: AppSettings): Promise<void> {
  return invoke<void>("update_settings", { settings });
}

// ─── Logs ───

export async function getLogs(count: number): Promise<LogEntry[]> {
  return invoke<LogEntry[]>("get_logs", { count });
}
