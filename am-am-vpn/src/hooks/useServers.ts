import { useState, useCallback, useEffect } from "react";
import type { ServerNode, Subscription } from "../types";
import * as api from "../services/api";

export function useServers() {
  const [servers, setServers] = useState<ServerNode[]>([]);
  const [subscriptions, setSubscriptions] = useState<Subscription[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const refresh = useCallback(async () => {
    setLoading(true);
    try {
      const [subs, srvs] = await Promise.all([
        api.listSubscriptions(),
        api.listServers(),
      ]);
      setSubscriptions(subs);
      setServers(srvs);
    } catch {
      /* backend not ready */
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const addSub = useCallback(async (url: string) => {
    setLoading(true);
    try {
      await api.addSubscription(url);
      await refresh();
    } finally {
      setLoading(false);
    }
  }, [refresh]);

  const removeSub = useCallback(async (id: string) => {
    await api.removeSubscription(id);
    await refresh();
  }, [refresh]);

  const testAll = useCallback(async () => {
    const latencies = await api.testAllLatencies();
    setServers((prev) =>
      prev.map((s) => ({
        ...s,
        latency: latencies[s.id] ?? s.latency,
      }))
    );
  }, []);

  return {
    servers,
    subscriptions,
    selectedId,
    setSelectedId,
    loading,
    addSub,
    removeSub,
    testAll,
    refresh,
  };
}
