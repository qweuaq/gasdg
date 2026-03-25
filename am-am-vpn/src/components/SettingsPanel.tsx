import { useState, useEffect, useCallback } from "react";
import type { AppSettings } from "../types";
import * as api from "../services/api";

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function SettingsPanel({ open, onClose }: Props) {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [saving, setSaving] = useState(false);

  const load = useCallback(async () => {
    try {
      const s = await api.getSettings();
      setSettings(s);
    } catch {
      /* backend not ready */
    }
  }, []);

  useEffect(() => {
    if (open) load();
  }, [open, load]);

  if (!open || !settings) return null;

  const update = (patch: Partial<AppSettings>) => {
    setSettings((prev) => (prev ? { ...prev, ...patch } : prev));
  };

  const handleSave = async () => {
    if (!settings) return;
    setSaving(true);
    try {
      await api.updateSettings(settings);
      onClose();
    } catch {
      /* keep panel open so user can retry */
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="settings-overlay" onClick={onClose}>
      <div className="settings-panel" onClick={(e) => e.stopPropagation()}>
        <div className="settings-header">
          <h2>Settings</h2>
          <button className="btn-sm" onClick={onClose}>
            ✕
          </button>
        </div>

        <div className="settings-body">
          <label className="settings-field">
            <span>Proxy Mode</span>
            <select
              value={settings.proxy_mode}
              onChange={(e) =>
                update({
                  proxy_mode: e.target.value as "system" | "tun",
                })
              }
            >
              <option value="system">System Proxy</option>
              <option value="tun">TUN Mode</option>
            </select>
          </label>

          <label className="settings-field">
            <span>HTTP Port</span>
            <input
              type="number"
              value={settings.http_port}
              min={1024}
              max={65535}
              onChange={(e) =>
                update({ http_port: Number(e.target.value) })
              }
            />
          </label>

          <label className="settings-field">
            <span>SOCKS5 Port</span>
            <input
              type="number"
              value={settings.socks_port}
              min={1024}
              max={65535}
              onChange={(e) =>
                update({ socks_port: Number(e.target.value) })
              }
            />
          </label>

          <label className="settings-field">
            <span>DNS Servers</span>
            <input
              type="text"
              value={settings.dns_servers.join(", ")}
              onChange={(e) =>
                update({
                  dns_servers: e.target.value
                    .split(",")
                    .map((s) => s.trim())
                    .filter(Boolean),
                })
              }
              placeholder="1.1.1.1, 8.8.8.8"
            />
          </label>

          <label className="settings-toggle">
            <input
              type="checkbox"
              checked={settings.auto_connect}
              onChange={(e) =>
                update({ auto_connect: e.target.checked })
              }
            />
            <span>Auto-connect on startup</span>
          </label>

          <label className="settings-toggle">
            <input
              type="checkbox"
              checked={settings.auto_select_fastest}
              onChange={(e) =>
                update({ auto_select_fastest: e.target.checked })
              }
            />
            <span>Auto-select fastest server</span>
          </label>

          <label className="settings-toggle">
            <input
              type="checkbox"
              checked={settings.subscription_auto_update}
              onChange={(e) =>
                update({
                  subscription_auto_update: e.target.checked,
                })
              }
            />
            <span>Auto-update subscriptions</span>
          </label>

          {settings.subscription_auto_update && (
            <label className="settings-field">
              <span>Update interval (hours)</span>
              <input
                type="number"
                value={settings.update_interval_hours}
                min={1}
                max={168}
                onChange={(e) =>
                  update({
                    update_interval_hours: Number(e.target.value),
                  })
                }
              />
            </label>
          )}
        </div>

        <div className="settings-footer">
          <button className="btn-sm" onClick={onClose}>
            Cancel
          </button>
          <button
            className="btn-sm btn-primary"
            onClick={handleSave}
            disabled={saving}
          >
            {saving ? "Saving…" : "Save"}
          </button>
        </div>
      </div>
    </div>
  );
}
