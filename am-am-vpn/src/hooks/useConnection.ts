import { useState, useCallback, useEffect, useRef } from "react";
import type { ConnectionState, ConnectionStatus } from "../types";
import * as api from "../services/api";

const INITIAL_STATE: ConnectionState = {
  status: "disconnected",
  server_id: null,
  connected_since: null,
  download_bytes: 0,
  upload_bytes: 0,
};

export function useConnection() {
  const [state, setState] = useState<ConnectionState>(INITIAL_STATE);
  const polling = useRef<ReturnType<typeof setInterval> | null>(null);

  const poll = useCallback(async () => {
    try {
      const s = await api.getConnectionState();
      setState(s);
    } catch {
      /* backend not ready */
    }
  }, []);

  useEffect(() => {
    poll();
    polling.current = setInterval(poll, 2000);
    return () => {
      if (polling.current) clearInterval(polling.current);
    };
  }, [poll]);

  const connectTo = useCallback(async (serverId: string) => {
    setState((prev) => ({ ...prev, status: "connecting" }));
    try {
      await api.connect(serverId);
      await poll();
    } catch {
      setState((prev) => ({ ...prev, status: "error" }));
    }
  }, [poll]);

  const disconnectVpn = useCallback(async () => {
    try {
      await api.disconnect();
      setState(INITIAL_STATE);
    } catch {
      /* ignore */
    }
  }, []);

  const selectFastest = useCallback(async () => {
    const id = await api.selectFastestServer();
    await connectTo(id);
  }, [connectTo]);

  return { state, connectTo, disconnectVpn, selectFastest };
}
