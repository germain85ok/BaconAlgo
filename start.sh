#!/bin/bash

# BaconAlgo Startup Script
# Launches the complete trading platform

echo "🥓 Starting BaconAlgo Ultimate Trading Platform..."
echo ""

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js 18+ first."
    exit 1
fi

echo "✅ Node.js found: $(node --version)"
echo ""

# Navigate to dashboard directory
cd "$(dirname "$0")/dashboard" || exit 1

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
    echo ""
fi

echo "🚀 Starting BaconAlgo Dashboard..."
echo ""
echo "Dashboard will be available at: http://localhost:5173"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

# Start the development server
npm run dev
