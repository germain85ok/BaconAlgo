#!/bin/bash
set -e
echo "🥓 BaconAlgo Deployment"
cd station && pnpm install && pnpm build
cd ../execution && cargo build --release
echo "✅ Build complete!"
