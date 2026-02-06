/**
 * V3: Volume Profile Analysis
 * Point of Control, Value Area, HVN/LVN detection
 */

export interface VolumeNode {
  price: number;
  volume: number;
  percentage: number;
}

export interface VolumeProfileAnalysis {
  symbol: string;
  poc: number; // Point of Control
  vah: number; // Value Area High
  val: number; // Value Area Low
  hvn: number[]; // High Volume Nodes
  lvn: number[]; // Low Volume Nodes
  nakedPOC: boolean;
  score: number; // 0-7
}

export class VolumeProfileAnalyzer {
  analyze(symbol: string, priceVolume: Array<{price: number; volume: number}>): VolumeProfileAnalysis {
    // Find POC (price with highest volume)
    let maxVol = 0;
    let poc = 0;
    
    for (const pv of priceVolume) {
      if (pv.volume > maxVol) {
        maxVol = pv.volume;
        poc = pv.price;
      }
    }
    
    // Calculate total volume
    const totalVol = priceVolume.reduce((sum, pv) => sum + pv.volume, 0);
    
    // Find value area (70% of volume)
    const sorted = [...priceVolume].sort((a, b) => b.volume - a.volume);
    let cumVol = 0;
    const valueArea: number[] = [];
    
    for (const pv of sorted) {
      cumVol += pv.volume;
      valueArea.push(pv.price);
      if (cumVol / totalVol >= 0.7) break;
    }
    
    const vah = Math.max(...valueArea);
    const val = Math.min(...valueArea);
    
    // Find HVN (volume > 150% average)
    const avgVol = totalVol / priceVolume.length;
    const hvn = priceVolume.filter(pv => pv.volume > avgVol * 1.5).map(pv => pv.price);
    
    // Find LVN (volume < 50% average)
    const lvn = priceVolume.filter(pv => pv.volume < avgVol * 0.5).map(pv => pv.price);
    
    const score = Math.min(7, Math.floor((maxVol / avgVol)));
    
    return { symbol, poc, vah, val, hvn, lvn, nakedPOC: false, score };
  }
}
