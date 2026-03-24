import { useState, type FormEvent } from "react";

interface Props {
  onAdd: (url: string) => Promise<void>;
  loading: boolean;
}

export default function SubscriptionInput({ onAdd, loading }: Props) {
  const [url, setUrl] = useState("");

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    const trimmed = url.trim();
    if (!trimmed) return;
    await onAdd(trimmed);
    setUrl("");
  };

  return (
    <form className="subscription-input" onSubmit={handleSubmit}>
      <input
        type="url"
        placeholder="Paste subscription URL…"
        value={url}
        onChange={(e) => setUrl(e.target.value)}
        disabled={loading}
      />
      <button type="submit" disabled={loading || !url.trim()}>
        {loading ? "…" : "+"}
      </button>
    </form>
  );
}
