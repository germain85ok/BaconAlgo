<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { createChart, ColorType, LineStyle, type IChartApi, type ISeriesApi } from 'lightweight-charts';
  
  export let symbol = 'BTCUSDT';
  export let timeframe = '15';
  export let showFVG = true;
  export let showOB = true;
  export let showBOS = true;
  export let showLiquidity = true;
  export let showVWAP = true;
  export let showVolumeProfile = false;
  export let showPrevDayLevels = true;
  export let showFibonacci = true;
  export let showSignals = true;
  
  let chartContainer: HTMLDivElement;
  let chart: IChartApi;
  let candlestickSeries: any;
  let volumeSeries: any;
  let vwapSeries: any;
  
  // SMC Drawing arrays
  let fvgBoxes: any[] = [];
  let obBoxes: any[] = [];
  let bosLines: any[] = [];
  let liquidityLines: any[] = [];
  let priceLines: any[] = [];
  
  // Use environment variable for API URL with fallback
  const API_BASE = import.meta.env.VITE_API_URL || 'http://localhost:8080';
  
  onMount(() => {
    // Create chart
    chart = createChart(chartContainer, {
      layout: {
        background: { type: ColorType.Solid, color: '#0a0a0f' },
        textColor: '#d1d4dc',
      },
      grid: {
        vertLines: { color: '#1a1a2e' },
        horzLines: { color: '#1a1a2e' },
      },
      crosshair: {
        mode: 1,
      },
      rightPriceScale: {
        borderColor: '#2a2a3e',
      },
      timeScale: {
        borderColor: '#2a2a3e',
        timeVisible: true,
        secondsVisible: false,
      },
      width: chartContainer.clientWidth,
      height: 600,
    });
    
    // Add candlestick series
    candlestickSeries = chart.addCandlestickSeries({
      upColor: '#00ff88',
      downColor: '#ff4466',
      borderUpColor: '#00ff88',
      borderDownColor: '#ff4466',
      wickUpColor: '#00ff88',
      wickDownColor: '#ff4466',
    });
    
    // Add volume series
    volumeSeries = chart.addHistogramSeries({
      color: '#26a69a',
      priceFormat: { type: 'volume' },
      priceScaleId: '',
      scaleMargins: { top: 0.8, bottom: 0 },
    });
    
    // Load initial data
    loadChartData();
    
    // Handle resize
    const handleResize = () => {
      chart.applyOptions({ width: chartContainer.clientWidth });
    };
    window.addEventListener('resize', handleResize);
    
    return () => {
      window.removeEventListener('resize', handleResize);
    };
  });
  
  async function loadChartData() {
    try {
      // Fetch candle data from API
      const response = await fetch(`${API_BASE}/api/candles/${symbol}?timeframe=${timeframe}`);
      
      if (!response.ok) {
        // Use mock data if API is not available
        useMockData();
        return;
      }
      
      const data = await response.json();
      
      if (data.candles && data.candles.length > 0) {
        candlestickSeries.setData(data.candles);
        volumeSeries.setData(data.volume || []);
        
        // Draw SMC levels
        if (data.smc) {
          drawSMCLevels(data.smc);
        }
      } else {
        useMockData();
      }
    } catch (error) {
      console.error('Failed to load chart data:', error);
      useMockData();
    }
  }
  
  function useMockData() {
    // Generate mock candle data for demonstration
    const now = Math.floor(Date.now() / 1000);
    const candleData = [];
    const volumeData = [];
    let basePrice = 50000;
    
    for (let i = 300; i >= 0; i--) {
      const time = now - i * 900; // 15 min intervals
      const volatility = Math.random() * 200 - 100;
      const open = basePrice + volatility;
      const close = open + (Math.random() * 400 - 200);
      const high = Math.max(open, close) + Math.random() * 150;
      const low = Math.min(open, close) - Math.random() * 150;
      
      candleData.push({
        time,
        open,
        high,
        low,
        close,
      });
      
      volumeData.push({
        time,
        value: Math.random() * 1000000,
        color: close >= open ? '#26a69a' : '#ef5350',
      });
      
      basePrice = close;
    }
    
    candlestickSeries.setData(candleData);
    volumeSeries.setData(volumeData);
    
    // Mock SMC data
    const mockSMC = {
      fvg: [
        {
          type: 'bullish',
          low: basePrice - 500,
          high: basePrice - 300,
          startTime: now - 50 * 900,
          endTime: now,
        },
      ],
      orderBlocks: [
        {
          type: 'bullish',
          low: basePrice - 800,
          high: basePrice - 600,
          startTime: now - 100 * 900,
          endTime: now,
        },
      ],
      bos: [
        {
          type: 'bullish',
          price: basePrice - 400,
          time: now - 30 * 900,
        },
      ],
      liquidity: [
        {
          type: 'bsl',
          price: basePrice + 600,
          time: now - 80 * 900,
        },
      ],
      vwap: candleData.map((c) => ({
        time: c.time,
        value: basePrice * 0.998,
      })),
      prevDay: {
        high: basePrice + 800,
        low: basePrice - 800,
        close: basePrice - 100,
      },
    };
    
    drawSMCLevels(mockSMC);
  }
  
  function drawSMCLevels(smc: any) {
    // Clear existing drawings
    clearDrawings();
    
    // Draw FVGs
    if (showFVG && smc.fvg) {
      smc.fvg.forEach((fvg: any) => {
        // Note: Lightweight Charts doesn't support boxes natively
        // We use horizontal lines instead
        const color = fvg.type === 'bullish' ? '#00ff88' : '#ff4466';
        
        const highLine = candlestickSeries.createPriceLine({
          price: fvg.high,
          color: color,
          lineWidth: 1,
          lineStyle: LineStyle.Dashed,
          axisLabelVisible: false,
          title: `FVG ${fvg.type}`,
        });
        
        const lowLine = candlestickSeries.createPriceLine({
          price: fvg.low,
          color: color,
          lineWidth: 1,
          lineStyle: LineStyle.Dashed,
          axisLabelVisible: false,
        });
        
        fvgBoxes.push(highLine, lowLine);
      });
    }
    
    // Draw Order Blocks
    if (showOB && smc.orderBlocks) {
      smc.orderBlocks.forEach((ob: any) => {
        const color = ob.type === 'bullish' ? '#4488ff' : '#ff8844';
        
        const highLine = candlestickSeries.createPriceLine({
          price: ob.high,
          color: color,
          lineWidth: 2,
          lineStyle: LineStyle.Solid,
          axisLabelVisible: true,
          title: `OB ${ob.type}`,
        });
        
        const lowLine = candlestickSeries.createPriceLine({
          price: ob.low,
          color: color,
          lineWidth: 2,
          lineStyle: LineStyle.Solid,
          axisLabelVisible: false,
        });
        
        obBoxes.push(highLine, lowLine);
      });
    }
    
    // Draw BOS lines
    if (showBOS && smc.bos) {
      smc.bos.forEach((bos: any) => {
        const line = candlestickSeries.createPriceLine({
          price: bos.price,
          color: bos.type === 'bullish' ? '#00ff88' : '#ff4466',
          lineWidth: 2,
          lineStyle: LineStyle.Dashed,
          axisLabelVisible: true,
          title: `BOS ${bos.type === 'bullish' ? '↑' : '↓'}`,
        });
        bosLines.push(line);
      });
    }
    
    // Draw Liquidity zones
    if (showLiquidity && smc.liquidity) {
      smc.liquidity.forEach((liq: any) => {
        const line = candlestickSeries.createPriceLine({
          price: liq.price,
          color: liq.type === 'bsl' ? '#ffcc00' : '#ff6600',
          lineWidth: 1,
          lineStyle: LineStyle.Dotted,
          axisLabelVisible: true,
          title: `${liq.type.toUpperCase()} $$$`,
        });
        liquidityLines.push(line);
      });
    }
    
    // Draw VWAP
    if (showVWAP && smc.vwap && smc.vwap.length > 0) {
      vwapSeries = chart.addLineSeries({
        color: '#ffcc00',
        lineWidth: 2,
        title: 'VWAP',
      });
      vwapSeries.setData(smc.vwap);
    }
    
    // Draw Previous Day Levels
    if (showPrevDayLevels && smc.prevDay) {
      const pdhLine = candlestickSeries.createPriceLine({
        price: smc.prevDay.high,
        color: '#ff4466',
        lineWidth: 1,
        lineStyle: LineStyle.Solid,
        axisLabelVisible: true,
        title: 'PDH',
      });
      
      const pdlLine = candlestickSeries.createPriceLine({
        price: smc.prevDay.low,
        color: '#00ff88',
        lineWidth: 1,
        lineStyle: LineStyle.Solid,
        axisLabelVisible: true,
        title: 'PDL',
      });
      
      const pdcLine = candlestickSeries.createPriceLine({
        price: smc.prevDay.close,
        color: '#ffcc00',
        lineWidth: 1,
        lineStyle: LineStyle.Dashed,
        axisLabelVisible: true,
        title: 'PDC',
      });
      
      priceLines.push(pdhLine, pdlLine, pdcLine);
    }
  }
  
  function clearDrawings() {
    // Remove all drawings
    fvgBoxes.forEach((line) => candlestickSeries.removePriceLine(line));
    obBoxes.forEach((line) => candlestickSeries.removePriceLine(line));
    bosLines.forEach((line) => candlestickSeries.removePriceLine(line));
    liquidityLines.forEach((line) => candlestickSeries.removePriceLine(line));
    priceLines.forEach((line) => candlestickSeries.removePriceLine(line));
    
    if (vwapSeries) {
      chart.removeSeries(vwapSeries);
      vwapSeries = null;
    }
    
    fvgBoxes = [];
    obBoxes = [];
    bosLines = [];
    liquidityLines = [];
    priceLines = [];
  }
  
  export function updateSymbol(newSymbol: string) {
    symbol = newSymbol;
    loadChartData();
  }
  
  export function updateTimeframe(newTimeframe: string) {
    timeframe = newTimeframe;
    loadChartData();
  }
  
  onDestroy(() => {
    if (chart) {
      chart.remove();
    }
  });
</script>

<div class="chart-wrapper">
  <div class="chart-controls">
    <div class="control-group">
      <label for="symbol-input">Symbol:</label>
      <input
        id="symbol-input"
        type="text"
        bind:value={symbol}
        on:change={() => loadChartData()}
        placeholder="BTCUSDT"
      />
    </div>
    
    <div class="control-group">
      <label for="timeframe-select">Timeframe:</label>
      <select id="timeframe-select" bind:value={timeframe} on:change={() => loadChartData()}>
        <option value="1">1m</option>
        <option value="5">5m</option>
        <option value="15">15m</option>
        <option value="60">1H</option>
        <option value="240">4H</option>
        <option value="D">1D</option>
      </select>
    </div>
    
    <div class="control-group toggles">
      <label>
        <input type="checkbox" bind:checked={showFVG} on:change={() => loadChartData()} />
        FVG
      </label>
      <label>
        <input type="checkbox" bind:checked={showOB} on:change={() => loadChartData()} />
        OB
      </label>
      <label>
        <input type="checkbox" bind:checked={showBOS} on:change={() => loadChartData()} />
        BOS
      </label>
      <label>
        <input type="checkbox" bind:checked={showLiquidity} on:change={() => loadChartData()} />
        Liquidity
      </label>
      <label>
        <input type="checkbox" bind:checked={showVWAP} on:change={() => loadChartData()} />
        VWAP
      </label>
      <label>
        <input type="checkbox" bind:checked={showPrevDayLevels} on:change={() => loadChartData()} />
        Prev Day
      </label>
    </div>
  </div>
  
  <div bind:this={chartContainer} class="chart-container"></div>
  
  <div class="chart-legend">
    <span class="legend-item fvg" style="opacity: {showFVG ? 1 : 0.3}">FVG</span>
    <span class="legend-item ob" style="opacity: {showOB ? 1 : 0.3}">OB</span>
    <span class="legend-item bos" style="opacity: {showBOS ? 1 : 0.3}">BOS</span>
    <span class="legend-item vwap" style="opacity: {showVWAP ? 1 : 0.3}">VWAP</span>
    <span class="legend-item liquidity" style="opacity: {showLiquidity ? 1 : 0.3}">Liquidity</span>
  </div>
</div>

<style>
  .chart-wrapper {
    width: 100%;
    height: 700px;
    background: #0a0a0f;
    border-radius: 12px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  
  .chart-container {
    width: 100%;
    flex: 1;
  }
  
  .chart-controls {
    display: flex;
    gap: 15px;
    padding: 12px 15px;
    background: #12121a;
    border-bottom: 1px solid #1a1a2e;
    flex-wrap: wrap;
    align-items: center;
  }
  
  .control-group {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  .control-group label {
    color: #d1d4dc;
    font-size: 13px;
    font-weight: 500;
  }
  
  .control-group input[type="text"],
  .control-group select {
    padding: 6px 10px;
    background: #1a1a2e;
    border: 1px solid #2a2a3e;
    border-radius: 6px;
    color: #d1d4dc;
    font-size: 13px;
    min-width: 120px;
  }
  
  .control-group input[type="text"]:focus,
  .control-group select:focus {
    outline: none;
    border-color: #00ff88;
  }
  
  .control-group.toggles {
    display: flex;
    gap: 12px;
    margin-left: auto;
  }
  
  .control-group.toggles label {
    display: flex;
    align-items: center;
    gap: 5px;
    cursor: pointer;
    font-size: 12px;
  }
  
  .control-group.toggles input[type="checkbox"] {
    cursor: pointer;
  }
  
  .chart-legend {
    display: flex;
    gap: 15px;
    padding: 10px 15px;
    background: #12121a;
    border-top: 1px solid #1a1a2e;
  }
  
  .legend-item {
    font-size: 11px;
    font-weight: 600;
    padding: 5px 10px;
    border-radius: 4px;
    letter-spacing: 0.5px;
    transition: opacity 0.2s;
  }
  
  .legend-item.fvg {
    background: rgba(0, 255, 136, 0.15);
    color: #00ff88;
    border: 1px solid rgba(0, 255, 136, 0.3);
  }
  
  .legend-item.ob {
    background: rgba(68, 136, 255, 0.15);
    color: #4488ff;
    border: 1px solid rgba(68, 136, 255, 0.3);
  }
  
  .legend-item.bos {
    background: rgba(255, 204, 0, 0.15);
    color: #ffcc00;
    border: 1px solid rgba(255, 204, 0, 0.3);
  }
  
  .legend-item.vwap {
    background: rgba(255, 136, 68, 0.15);
    color: #ff8844;
    border: 1px solid rgba(255, 136, 68, 0.3);
  }
  
  .legend-item.liquidity {
    background: rgba(255, 102, 0, 0.15);
    color: #ff6600;
    border: 1px solid rgba(255, 102, 0, 0.3);
  }
</style>
