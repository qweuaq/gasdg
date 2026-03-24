import type { ConnectionState, ServerNode } from "../types";

interface Props {
  state: ConnectionState;
  server: ServerNode | undefined;
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function elapsed(since: string | null): string {
  if (!since) return "—";
  const diff = Math.floor((Date.now() - new Date(since).getTime()) / 1000);
  const h = Math.floor(diff / 3600);
  const m = Math.floor((diff % 3600) / 60);
  const s = diff % 60;
  return `${String(h).padStart(2, "0")}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}

export default function StatusBar({ state, server }: Props) {
  return (
    <div className={`status-bar ${state.status}`}>
      <div className="status-row">
        <span className="status-dot" />
        <span className="status-text">{state.status.toUpperCase()}</span>
      </div>
      {server && (
        <div className="status-detail">
          <span>{server.name}</span>
          <span>{server.protocol.toUpperCase()}</span>
        </div>
      )}
      {state.status === "connected" && (
        <div className="status-stats">
          <span>↓ {formatBytes(state.download_bytes)}</span>
          <span>↑ {formatBytes(state.upload_bytes)}</span>
          <span>{elapsed(state.connected_since)}</span>
        </div>
      )}
    </div>
  );
}
