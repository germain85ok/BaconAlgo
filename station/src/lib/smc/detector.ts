export interface PriceBar {
	time: number;
	open: number;
	high: number;
	low: number;
	close: number;
	volume: number;
}

export interface FairValueGap {
	type: 'bullish' | 'bearish';
	start: number;
	end: number;
	top: number;
	bottom: number;
	isFilled: boolean;
}

export interface OrderBlock {
	type: 'bullish' | 'bearish';
	time: number;
	high: number;
	low: number;
	volume: number;
	strength: number;
}

export interface StructureBreak {
	type: 'BOS' | 'CHoCH';
	direction: 'bullish' | 'bearish';
	time: number;
	price: number;
	previousHigh?: number;
	previousLow?: number;
}

export class SMCDetector {
	// Detect Fair Value Gaps
	detectFVGs(bars: PriceBar[]): FairValueGap[] {
		const fvgs: FairValueGap[] = [];
		
		for (let i = 2; i < bars.length; i++) {
			const prev = bars[i - 2];
			const current = bars[i - 1];
			const next = bars[i];

			// Bullish FVG: gap between prev high and next low
			if (prev.high < next.low) {
				fvgs.push({
					type: 'bullish',
					start: current.time,
					end: next.time,
					top: next.low,
					bottom: prev.high,
					isFilled: false
				});
			}

			// Bearish FVG: gap between prev low and next high
			if (prev.low > next.high) {
				fvgs.push({
					type: 'bearish',
					start: current.time,
					end: next.time,
					top: prev.low,
					bottom: next.high,
					isFilled: false
				});
			}
		}

		return fvgs;
	}

	// Detect Order Blocks
	detectOrderBlocks(bars: PriceBar[]): OrderBlock[] {
		const orderBlocks: OrderBlock[] = [];
		
		for (let i = 1; i < bars.length - 1; i++) {
			const prev = bars[i - 1];
			const current = bars[i];
			const next = bars[i + 1];

			// Bullish Order Block: strong up move after consolidation
			if (current.close > current.open && 
				next.close > next.open && 
				next.close > current.high) {
				
				const strength = (next.close - current.open) / current.open;
				orderBlocks.push({
					type: 'bullish',
					time: current.time,
					high: current.high,
					low: current.low,
					volume: current.volume,
					strength
				});
			}

			// Bearish Order Block: strong down move after consolidation
			if (current.close < current.open && 
				next.close < next.open && 
				next.close < current.low) {
				
				const strength = (current.open - next.close) / current.open;
				orderBlocks.push({
					type: 'bearish',
					time: current.time,
					high: current.high,
					low: current.low,
					volume: current.volume,
					strength
				});
			}
		}

		return orderBlocks;
	}

	// Detect Break of Structure (BOS) and Change of Character (CHoCH)
	detectStructureBreaks(bars: PriceBar[]): StructureBreak[] {
		const breaks: StructureBreak[] = [];
		
		// Find swing highs and lows
		const swingHighs: { time: number, price: number }[] = [];
		const swingLows: { time: number, price: number }[] = [];

		for (let i = 2; i < bars.length - 2; i++) {
			const bar = bars[i];
			
			// Swing high
			if (bar.high > bars[i-1].high && bar.high > bars[i-2].high &&
				bar.high > bars[i+1].high && bar.high > bars[i+2].high) {
				swingHighs.push({ time: bar.time, price: bar.high });
			}

			// Swing low
			if (bar.low < bars[i-1].low && bar.low < bars[i-2].low &&
				bar.low < bars[i+1].low && bar.low < bars[i+2].low) {
				swingLows.push({ time: bar.time, price: bar.low });
			}
		}

		// Detect breaks
		for (let i = 1; i < swingHighs.length; i++) {
			const prev = swingHighs[i - 1];
			const current = swingHighs[i];
			
			if (current.price > prev.price) {
				breaks.push({
					type: 'BOS',
					direction: 'bullish',
					time: current.time,
					price: current.price,
					previousHigh: prev.price
				});
			}
		}

		for (let i = 1; i < swingLows.length; i++) {
			const prev = swingLows[i - 1];
			const current = swingLows[i];
			
			if (current.price < prev.price) {
				breaks.push({
					type: 'BOS',
					direction: 'bearish',
					time: current.time,
					price: current.price,
					previousLow: prev.price
				});
			}
		}

		return breaks;
	}

	// Comprehensive analysis
	analyze(bars: PriceBar[]) {
		return {
			fvgs: this.detectFVGs(bars),
			orderBlocks: this.detectOrderBlocks(bars),
			structureBreaks: this.detectStructureBreaks(bars)
		};
	}
}

export const smcDetector = new SMCDetector();
