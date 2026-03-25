import { useState } from "react";
import SubscriptionInput from "./components/SubscriptionInput";
import ServerList from "./components/ServerList";
import ConnectionButton from "./components/ConnectionButton";
import StatusBar from "./components/StatusBar";
import LogViewer from "./components/LogViewer";
import SettingsPanel from "./components/SettingsPanel";
import ToastContainer, { showToast } from "./components/Toast";
import { useConnection } from "./hooks/useConnection";
import { useServers } from "./hooks/useServers";

export default function App() {
  const [settingsOpen, setSettingsOpen] = useState(false);
  const {
    servers,
    selectedId,
    setSelectedId,
    loading,
    addSub,
    removeSub,
    testAll,
  } = useServers();
  const { state, connectTo, disconnectVpn, selectFastest } = useConnection();
  const currentServer = servers.find(
    (s) => s.id === (state.server_id ?? selectedId),
  );

  const handleAddSub = async (url: string) => {
    try {
      await addSub(url);
      showToast("success", "Subscription added");
    } catch {
      showToast("error", "Failed to add subscription");
    }
  };

  const handleRemoveSub = async (id: string) => {
    try {
      await removeSub(id);
      showToast("info", "Subscription removed");
    } catch {
      showToast("error", "Failed to remove subscription");
    }
  };

  const handleConnect = async () => {
    try {
      if (selectedId) {
        await connectTo(selectedId);
      } else {
        await selectFastest();
      }
    } catch {
      showToast("error", "Connection failed");
    }
  };

  return (
    <div className="app">
      <header className="app-header">
        <div className="header-row">
          <h1>Am-Am VPN</h1>
          <button
            className="btn-sm"
            onClick={() => setSettingsOpen(true)}
          >
            ⚙
          </button>
        </div>
        <StatusBar state={state} server={currentServer} />
      </header>

      <main className="app-main">
        <SubscriptionInput onAdd={handleAddSub} loading={loading} />

        <ConnectionButton
          status={state.status}
          onConnect={handleConnect}
          onDisconnect={disconnectVpn}
          disabled={servers.length === 0 && !selectedId}
        />

        <ServerList
          servers={servers}
          selectedId={selectedId}
          onSelect={setSelectedId}
          onTestAll={testAll}
          onRemove={handleRemoveSub}
        />
      </main>

      <footer className="app-footer">
        <LogViewer />
      </footer>

      <SettingsPanel
        open={settingsOpen}
        onClose={() => setSettingsOpen(false)}
      />
      <ToastContainer />
    </div>
  );
}
