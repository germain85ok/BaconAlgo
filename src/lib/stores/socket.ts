import { sentimentRaw } from '$lib/stores/sentiment';

export function connectStream() {
  const ws = new WebSocket('ws://localhost:3000');

  ws.onmessage = (e) => {
    const msg = JSON.parse(e.data);

    if (msg.type === 'snapshot') {
      sentimentRaw.set(msg.payload);
    }

    if (msg.type === 'sentiment.update') {
      sentimentRaw.update((s) => ({
        ...s,
        [msg.payload.key]: msg.payload
      }));
    }
  };
}
