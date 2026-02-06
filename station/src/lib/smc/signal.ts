import type { FairValueGap, OrderBlock, StructureBreak } from './detector';

export type SignalDirection = 'LONG' | 'SHORT';
export type SignalStrength = 'WEAK' | 'MODERATE' | 'STRONG';

export interface TradingSignal {
	id: string;
	symbol: string;
	timeframe: string;
	direction: SignalDirection;
	strength: SignalStrength;
	score: number; // 0-100
	entry: number;
	stopLoss: number;
	takeProfit: number;
	targets: number[];
	riskRewardRatio: number;
	confidence: number;
	reasons: string[];
	smcData: {
		fvgs: FairValueGap[];
		orderBlocks: OrderBlock[];
		structureBreaks: StructureBreak[];
	};
	createdAt: Date;
	expiresAt: Date;
}

export class SignalGenerator {
	generateSignal(
		symbol: string,
		timeframe: string,
		fvgs: FairValueGap[],
		orderBlocks: OrderBlock[],
		structureBreaks: StructureBreak[],
		currentPrice: number
	): TradingSignal | null {
		
		// Calculate signal direction based on SMC patterns
		const bullishCount = [
			...fvgs.filter(f => f.type === 'bullish'),
			...orderBlocks.filter(ob => ob.type === 'bullish'),
			...structureBreaks.filter(sb => sb.direction === 'bullish')
		].length;

		const bearishCount = [
			...fvgs.filter(f => f.type === 'bearish'),
			...orderBlocks.filter(ob => ob.type === 'bearish'),
			...structureBreaks.filter(sb => sb.direction === 'bearish')
		].length;

		if (bullishCount === 0 && bearishCount === 0) {
			return null; // No signal
		}

		const direction: SignalDirection = bullishCount > bearishCount ? 'LONG' : 'SHORT';
		const reasons: string[] = [];

		// Calculate entry, stop loss, and take profit
		let entry = currentPrice;
		let stopLoss: number;
		let takeProfit: number;

		if (direction === 'LONG') {
			// Find nearest bullish order block for entry refinement
			const nearestOB = orderBlocks
				.filter(ob => ob.type === 'bullish' && ob.high < currentPrice)
				.sort((a, b) => b.time - a.time)[0];

			if (nearestOB) {
				entry = nearestOB.high;
				reasons.push(`Bullish Order Block at ${entry.toFixed(2)}`);
			}

			stopLoss = entry * 0.98; // 2% below entry
			takeProfit = entry * 1.06; // 6% above entry (3:1 R:R)

			if (fvgs.some(f => f.type === 'bullish')) {
				reasons.push('Bullish Fair Value Gap detected');
			}
			if (structureBreaks.some(sb => sb.type === 'BOS' && sb.direction === 'bullish')) {
				reasons.push('Break of Structure (Bullish)');
			}
		} else {
			// Find nearest bearish order block for entry refinement
			const nearestOB = orderBlocks
				.filter(ob => ob.type === 'bearish' && ob.low > currentPrice)
				.sort((a, b) => b.time - a.time)[0];

			if (nearestOB) {
				entry = nearestOB.low;
				reasons.push(`Bearish Order Block at ${entry.toFixed(2)}`);
			}

			stopLoss = entry * 1.02; // 2% above entry
			takeProfit = entry * 0.94; // 6% below entry (3:1 R:R)

			if (fvgs.some(f => f.type === 'bearish')) {
				reasons.push('Bearish Fair Value Gap detected');
			}
			if (structureBreaks.some(sb => sb.type === 'BOS' && sb.direction === 'bearish')) {
				reasons.push('Break of Structure (Bearish)');
			}
		}

		// Calculate targets (partial profit levels)
		const targets = direction === 'LONG' 
			? [entry * 1.02, entry * 1.04, entry * 1.06]
			: [entry * 0.98, entry * 0.96, entry * 0.94];

		// Calculate risk/reward ratio
		const risk = Math.abs(entry - stopLoss);
		const reward = Math.abs(takeProfit - entry);
		const riskRewardRatio = reward / risk;

		// Calculate score (0-100)
		let score = 50; // Base score
		score += bullishCount > bearishCount ? bullishCount * 10 : bearishCount * 10;
		score += Math.min(riskRewardRatio * 10, 30); // Bonus for good R:R
		score = Math.min(score, 100);

		// Determine strength
		let strength: SignalStrength;
		if (score >= 80) strength = 'STRONG';
		else if (score >= 60) strength = 'MODERATE';
		else strength = 'WEAK';

		// Calculate confidence
		const confidence = Math.min(score / 100, 1);

		return {
			id: `${symbol}-${timeframe}-${Date.now()}`,
			symbol,
			timeframe,
			direction,
			strength,
			score,
			entry,
			stopLoss,
			takeProfit,
			targets,
			riskRewardRatio,
			confidence,
			reasons,
			smcData: {
				fvgs,
				orderBlocks,
				structureBreaks
			},
			createdAt: new Date(),
			expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000) // 24 hours
		};
	}
}

export const signalGenerator = new SignalGenerator();
