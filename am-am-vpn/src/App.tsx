import SubscriptionInput from "./components/SubscriptionInput";
import ServerList from "./components/ServerList";
import ConnectionButton from "./components/ConnectionButton";
import StatusBar from "./components/StatusBar";
import LogViewer from "./components/LogViewer";
import { useConnection } from "./hooks/useConnection";
import { useServers } from "./hooks/useServers";

export default function App() {
  const { servers, selectedId, setSelectedId, loading, addSub, testAll } =
    useServers();
  const { state, connectTo, disconnectVpn, selectFastest } = useConnection();
  const currentServer = servers.find((s) => s.id === (state.server_id ?? selectedId));

  const handleConnect = () => {
    if (selectedId) {
      connectTo(selectedId);
    } else {
      selectFastest();
    }
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>Am-Am VPN</h1>
        <StatusBar state={state} server={currentServer} />
      </header>

      <main className="app-main">
        <SubscriptionInput onAdd={addSub} loading={loading} />

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
        />
      </main>

      <footer className="app-footer">
        <LogViewer />
      </footer>
    </div>
  );
}
