#!/bin/bash

echo "⚡ Initialisation du stream WebSocket..."

mkdir -p src/lib/stream

cat << 'EOF' > src/lib/stream/socket.ts
import { sentimentRaw } from '$lib/stores/sentiment';

let socket: WebSocket;

export function connectStream() {
  socket = new WebSocket('ws://localhost:3000/ws');

  socket.onmessage = (event) => {
    const msg = JSON.parse(event.data);

    if (msg.type === 'sentiment.update') {
      sentimentRaw.update((s) => ({
        ...s,
        [msg.payload.key]: msg.payload
      }));
    }
  };

  socket.onopen = () => console.log('🟢 Stream connecté');
  socket.onclose = () => console.log('🔴 Stream fermé');
}
EOF

echo "✅ Stream WebSocket prêt."
