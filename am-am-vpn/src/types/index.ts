// ─── Protocol types shared between UI and backend ───

export type Protocol = "vmess" | "vless" | "trojan" | "shadowsocks";

export type ConnectionStatus = "disconnected" | "connecting" | "connected" | "error";

export interface ServerNode {
  id: string;
  name: string;
  address: string;
  port: number;
  protocol: Protocol;
  latency: number | null;
  /** Raw protocol-specific settings serialised as JSON string */
  settings: string;
}

export interface Subscription {
  id: string;
  url: string;
  name: string;
  updated_at: string;
  servers: ServerNode[];
}

export interface ConnectionState {
  status: ConnectionStatus;
  server_id: string | null;
  connected_since: string | null;
  download_bytes: number;
  upload_bytes: number;
}

export interface AppSettings {
  proxy_mode: "system" | "tun";
  socks_port: number;
  http_port: number;
  dns_servers: string[];
  auto_connect: boolean;
  auto_select_fastest: boolean;
  subscription_auto_update: boolean;
  update_interval_hours: number;
}

export interface LogEntry {
  timestamp: string;
  level: "info" | "warn" | "error" | "debug";
  message: string;
}
