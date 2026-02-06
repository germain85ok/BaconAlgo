import type { Position } from '../brokers/types';

export interface VaRResult {
	var95: number;
	var99: number;
	portfolioValue: number;
	calculatedAt: Date;
}

export interface StressTestScenario {
	name: string;
	description: string;
	marketDrop: number; // percentage
	estimatedLoss: number;
}

export interface ExposureLimit {
	type: 'position' | 'sector' | 'total';
	limit: number;
	current: number;
	exceeded: boolean;
}

export interface DrawdownMetrics {
	current: number;
	max: number;
	recovery: number; // percentage to recover to high water mark
}

export class RiskEngine {
	// Value at Risk calculation (Historical method)
	calculateVaR(positions: Position[], confidence: 0.95 | 0.99): VaRResult {
		const portfolioValue = positions.reduce((sum, p) => 
			sum + (p.quantity * p.currentPrice), 0
		);

		// Simplified VaR calculation - in production, use historical returns
		const volatility = 0.02; // 2% daily volatility assumption
		const zScore = confidence === 0.95 ? 1.645 : 2.326;
		
		const varAmount = portfolioValue * volatility * zScore;

		return {
			var95: confidence === 0.95 ? varAmount : 0,
			var99: confidence === 0.99 ? varAmount : 0,
			portfolioValue,
			calculatedAt: new Date()
		};
	}

	// Stress Testing
	runStressTests(positions: Position[]): StressTestScenario[] {
		const portfolioValue = positions.reduce((sum, p) => 
			sum + (p.quantity * p.currentPrice), 0
		);

		const scenarios: StressTestScenario[] = [
			{
				name: 'Market Correction',
				description: '10% market drop',
				marketDrop: 0.10,
				estimatedLoss: portfolioValue * 0.10
			},
			{
				name: 'Bear Market',
				description: '20% market drop',
				marketDrop: 0.20,
				estimatedLoss: portfolioValue * 0.20
			},
			{
				name: 'Black Monday',
				description: '30% market crash',
				marketDrop: 0.30,
				estimatedLoss: portfolioValue * 0.30
			},
			{
				name: 'Flash Crash',
				description: '15% intraday drop',
				marketDrop: 0.15,
				estimatedLoss: portfolioValue * 0.15
			},
			{
				name: 'Sector Rotation',
				description: '25% sector-specific drop',
				marketDrop: 0.25,
				estimatedLoss: portfolioValue * 0.25 * 0.4 // Assume 40% sector exposure
			},
			{
				name: 'Interest Rate Shock',
				description: '12% market drop',
				marketDrop: 0.12,
				estimatedLoss: portfolioValue * 0.12
			}
		];

		return scenarios;
	}

	// Check exposure limits
	checkExposureLimits(
		positions: Position[],
		limits: { maxPositions: number, maxLeverage: number }
	): ExposureLimit[] {
		const totalValue = positions.reduce((sum, p) => 
			sum + Math.abs(p.quantity * p.currentPrice), 0
		);

		return [
			{
				type: 'position',
				limit: limits.maxPositions,
				current: positions.length,
				exceeded: positions.length > limits.maxPositions
			},
			{
				type: 'total',
				limit: totalValue * limits.maxLeverage,
				current: totalValue,
				exceeded: false // Simplified
			}
		];
	}

	// Calculate drawdown
	calculateDrawdown(
		currentValue: number,
		highWaterMark: number
	): DrawdownMetrics {
		const drawdown = (highWaterMark - currentValue) / highWaterMark;
		const recovery = highWaterMark > currentValue 
			? ((highWaterMark - currentValue) / currentValue) * 100
			: 0;

		return {
			current: drawdown * 100,
			max: drawdown * 100,
			recovery
		};
	}

	// Kill Switch - emergency position close
	async executeKillSwitch(broker: any): Promise<boolean> {
		try {
			await broker.cancelAllOrders();
			await broker.closeAllPositions();
			return true;
		} catch (error) {
			console.error('Kill switch execution failed:', error);
			return false;
		}
	}
}

export const riskEngine = new RiskEngine();
