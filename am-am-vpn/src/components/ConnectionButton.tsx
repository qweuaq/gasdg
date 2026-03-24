import type { ConnectionStatus } from "../types";

interface Props {
  status: ConnectionStatus;
  onConnect: () => void;
  onDisconnect: () => void;
  disabled: boolean;
}

export default function ConnectionButton({
  status,
  onConnect,
  onDisconnect,
  disabled,
}: Props) {
  const isConnected = status === "connected";
  const isConnecting = status === "connecting";

  const handleClick = () => {
    if (isConnected) {
      onDisconnect();
    } else if (!isConnecting) {
      onConnect();
    }
  };

  const label = isConnecting
    ? "Connecting…"
    : isConnected
      ? "Disconnect"
      : "Connect";

  return (
    <button
      className={`connection-btn ${status}`}
      onClick={handleClick}
      disabled={disabled || isConnecting}
    >
      <span className="pulse-ring" />
      <span className="btn-label">{label}</span>
    </button>
  );
}
