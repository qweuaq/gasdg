import type { ServerNode } from "../types";

interface Props {
  servers: ServerNode[];
  selectedId: string | null;
  onSelect: (id: string) => void;
  onTestAll: () => void;
  onRemove?: (subscriptionId: string) => void;
}

function protocolBadge(proto: string) {
  const colors: Record<string, string> = {
    vmess: "#3b82f6",
    vless: "#8b5cf6",
    trojan: "#ef4444",
    shadowsocks: "#10b981",
  };
  return (
    <span className="badge" style={{ background: colors[proto] ?? "#666" }}>
      {proto.toUpperCase()}
    </span>
  );
}

function latencyLabel(ms: number | null) {
  if (ms === null) return <span className="latency unknown">—</span>;
  const cls = ms < 150 ? "good" : ms < 300 ? "medium" : "bad";
  return <span className={`latency ${cls}`}>{ms} ms</span>;
}

export default function ServerList({
  servers,
  selectedId,
  onSelect,
  onTestAll,
}: Props) {
  if (servers.length === 0) {
    return (
      <div className="server-list empty">
        <p>No servers yet. Add a subscription above.</p>
      </div>
    );
  }

  return (
    <div className="server-list">
      <div className="server-list-header">
        <span>{servers.length} servers</span>
        <button className="btn-sm" onClick={onTestAll}>
          Test All
        </button>
      </div>
      <ul>
        {servers.map((s) => (
          <li
            key={s.id}
            className={`server-item${s.id === selectedId ? " selected" : ""}`}
            onClick={() => onSelect(s.id)}
          >
            <div className="server-info">
              {protocolBadge(s.protocol)}
              <span className="server-name">{s.name}</span>
            </div>
            <div className="server-meta">
              <span className="server-addr">
                {s.address}:{s.port}
              </span>
              {latencyLabel(s.latency)}
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}
