import { writable } from 'svelte/store';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

interface ScannerResult {
  symbol: string;
  setup: string;
  score: number;
}

export const scannerResults = writable<ScannerResult[]>([]);

// Fetch scanner results from API
export async function fetchScannerResults() {
  try {
    const response = await fetch(`${API_URL}/api/signals`);
    if (response.ok) {
      const data = await response.json();
      if (data.signals && Array.isArray(data.signals)) {
        // Transform API signals to scanner results
        const results = data.signals.map((signal: any) => ({
          symbol: signal.symbol,
          setup: signal.reason || 'Unknown',
          score: 85 // Default score - can be enhanced later
        }));
        scannerResults.set(results);
      }
    }
  } catch (error) {
    console.error('Failed to fetch scanner results:', error);
  }
}

// Connect to SSE stream for real-time updates
export function connectToScannerStream() {
  const eventSource = new EventSource(`${API_URL}/api/signals/stream`);
  
  eventSource.onmessage = (event) => {
    try {
      const signal = JSON.parse(event.data);
      scannerResults.update((results) => {
        // Add new signal or update existing
        const existingIndex = results.findIndex(r => r.symbol === signal.symbol);
        const newResult = {
          symbol: signal.symbol,
          setup: signal.reason || 'Unknown',
          score: 85
        };
        
        if (existingIndex >= 0) {
          results[existingIndex] = newResult;
          return [...results];
        } else {
          return [newResult, ...results].slice(0, 50); // Keep last 50
        }
      });
    } catch (error) {
      console.error('Failed to parse signal:', error);
    }
  };
  
  eventSource.onerror = (error) => {
    console.error('SSE connection error:', error);
    eventSource.close();
    // Retry connection after 5 seconds
    setTimeout(connectToScannerStream, 5000);
  };
  
  return () => eventSource.close();
}

// Initialize scanner data
fetchScannerResults();

