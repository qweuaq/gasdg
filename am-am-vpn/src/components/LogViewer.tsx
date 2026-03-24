import { useState, useEffect, useCallback } from "react";
import type { LogEntry } from "../types";
import * as api from "../services/api";

export default function LogViewer() {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [open, setOpen] = useState(false);

  const refresh = useCallback(async () => {
    try {
      const entries = await api.getLogs(200);
      setLogs(entries);
    } catch {
      /* backend not ready */
    }
  }, []);

  useEffect(() => {
    if (open) {
      refresh();
      const id = setInterval(refresh, 3000);
      return () => clearInterval(id);
    }
  }, [open, refresh]);

  return (
    <div className="log-viewer">
      <button className="btn-sm" onClick={() => setOpen((o) => !o)}>
        {open ? "Hide Logs" : "Show Logs"}
      </button>
      {open && (
        <div className="log-entries">
          {logs.map((l, i) => (
            <div key={i} className={`log-entry ${l.level}`}>
              <span className="log-time">{l.timestamp.slice(11, 19)}</span>
              <span className="log-level">{l.level}</span>
              <span className="log-msg">{l.message}</span>
            </div>
          ))}
          {logs.length === 0 && <div className="log-empty">No logs yet</div>}
        </div>
      )}
    </div>
  );
}
