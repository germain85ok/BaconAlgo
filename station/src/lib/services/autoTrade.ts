// Auto-trade settings service
import { supabase } from './supabase';

export interface AutoTradeSettings {
	id: string;
	user_id: string;
	enabled: boolean;
	mode: 'paper' | 'semi-auto' | 'full-auto';
	broker_connection_id: string | null;
	
	// Risk Management
	max_daily_loss: number;
	max_position_size_percent: number;
	max_concurrent_positions: number;
	
	// Trading Rules
	auto_execute_grades: string[];
	symbol_blacklist: string[];
	trading_hours_start: string | null;
	trading_hours_end: string | null;
	
	// Notification Settings
	discord_webhook_url: string | null;
	telegram_chat_id: string | null;
	email_notifications: boolean;
	
	// Kill Switch
	kill_switch_activated: boolean;
	kill_switch_activated_at: string | null;
	
	created_at: string;
	updated_at: string;
}

export interface BrokerConnection {
	id: string;
	user_id: string;
	broker_name: 'alpaca' | 'ib' | 'questrade' | 'bitget';
	api_key_encrypted: string | null;
	api_secret_encrypted: string | null;
	additional_config: any;
	is_paper: boolean;
	is_active: boolean;
	last_connected_at: string | null;
	created_at: string;
	updated_at: string;
}

export interface TradeExecution {
	id: string;
	user_id: string;
	signal_id: string | null;
	broker_connection_id: string | null;
	symbol: string;
	side: 'BUY' | 'SELL' | 'LONG' | 'SHORT';
	quantity: number;
	entry_price: number | null;
	filled_price: number | null;
	stop_loss: number | null;
	take_profit: number | null;
	status: 'pending' | 'filled' | 'partial' | 'cancelled' | 'failed';
	broker_order_id: string | null;
	mode: 'paper' | 'semi-auto' | 'full-auto' | null;
	pnl: number | null;
	pnl_percent: number | null;
	error_message: string | null;
	executed_at: string | null;
	closed_at: string | null;
	created_at: string;
}

/**
 * Get user's auto-trade settings
 */
export async function getAutoTradeSettings(
	userId: string
): Promise<{ data: AutoTradeSettings | null; error: any }> {
	const { data, error } = await supabase
		.from('auto_trade_settings')
		.select('*')
		.eq('user_id', userId)
		.single();

	return { data, error };
}

export interface UpdateAutoTradeSettings {
	enabled?: boolean;
	mode?: 'paper' | 'semi-auto' | 'full-auto';
	broker_connection_id?: string | null;
	max_daily_loss?: number;
	max_position_size_percent?: number;
	max_concurrent_positions?: number;
	auto_execute_grades?: string[];
	symbol_blacklist?: string[];
	trading_hours_start?: string | null;
	trading_hours_end?: string | null;
	discord_webhook_url?: string | null;
	telegram_chat_id?: string | null;
	email_notifications?: boolean;
}

/**
 * Create or update auto-trade settings
 */
export async function upsertAutoTradeSettings(
	userId: string,
	settings: UpdateAutoTradeSettings
): Promise<{ data: AutoTradeSettings | null; error: any }> {
	const { data, error } = await supabase
		.from('auto_trade_settings')
		.upsert(
			{
				user_id: userId,
				...settings
			},
			{
				onConflict: 'user_id'
			}
		)
		.select()
		.single();

	return { data, error };
}

/**
 * Activate kill switch - stops all auto-trading immediately
 */
export async function activateKillSwitch(userId: string): Promise<{ data: any; error: any }> {
	const { data, error } = await supabase
		.from('auto_trade_settings')
		.update({
			kill_switch_activated: true,
			kill_switch_activated_at: new Date().toISOString(),
			enabled: false
		})
		.eq('user_id', userId)
		.select()
		.single();

	return { data, error };
}

/**
 * Deactivate kill switch
 */
export async function deactivateKillSwitch(userId: string): Promise<{ data: any; error: any }> {
	const { data, error } = await supabase
		.from('auto_trade_settings')
		.update({
			kill_switch_activated: false,
			kill_switch_activated_at: null
		})
		.eq('user_id', userId)
		.select()
		.single();

	return { data, error };
}

/**
 * Get user's broker connections
 */
export async function getBrokerConnections(
	userId: string
): Promise<{ data: BrokerConnection[] | null; error: any }> {
	const { data, error } = await supabase
		.from('broker_connections')
		.select('*')
		.eq('user_id', userId)
		.order('created_at', { ascending: false });

	return { data, error };
}

/**
 * Create broker connection
 */
export async function createBrokerConnection(
	connection: Partial<BrokerConnection>
): Promise<{ data: BrokerConnection | null; error: any }> {
	const { data, error } = await supabase
		.from('broker_connections')
		.insert(connection)
		.select()
		.single();

	return { data, error };
}

/**
 * Update broker connection
 */
export async function updateBrokerConnection(
	id: string,
	updates: Partial<BrokerConnection>
): Promise<{ data: BrokerConnection | null; error: any }> {
	const { data, error } = await supabase
		.from('broker_connections')
		.update(updates)
		.eq('id', id)
		.select()
		.single();

	return { data, error };
}

/**
 * Delete broker connection
 */
export async function deleteBrokerConnection(id: string): Promise<{ data: any; error: any }> {
	const { data, error } = await supabase.from('broker_connections').delete().eq('id', id);

	return { data, error };
}

/**
 * Get user's trade executions
 */
export async function getTradeExecutions(
	userId: string,
	limit: number = 50
): Promise<{ data: TradeExecution[] | null; error: any }> {
	const { data, error } = await supabase
		.from('trade_executions')
		.select('*')
		.eq('user_id', userId)
		.order('created_at', { ascending: false })
		.limit(limit);

	return { data, error };
}

/**
 * Create trade execution
 */
export async function createTradeExecution(
	execution: Partial<TradeExecution>
): Promise<{ data: TradeExecution | null; error: any }> {
	const { data, error } = await supabase
		.from('trade_executions')
		.insert(execution)
		.select()
		.single();

	return { data, error };
}

/**
 * Update trade execution
 */
export async function updateTradeExecution(
	id: string,
	updates: Partial<TradeExecution>
): Promise<{ data: TradeExecution | null; error: any }> {
	const { data, error } = await supabase
		.from('trade_executions')
		.update(updates)
		.eq('id', id)
		.select()
		.single();

	return { data, error };
}

/**
 * Get trade statistics for user
 */
export async function getTradeStats(userId: string): Promise<{
	total_trades: number;
	winning_trades: number;
	losing_trades: number;
	total_pnl: number;
	win_rate: number;
}> {
	const { data } = await supabase
		.from('trade_executions')
		.select('pnl, status')
		.eq('user_id', userId)
		.in('status', ['filled', 'partial']);

	if (!data || data.length === 0) {
		return {
			total_trades: 0,
			winning_trades: 0,
			losing_trades: 0,
			total_pnl: 0,
			win_rate: 0
		};
	}

	const total_trades = data.length;
	const winning_trades = data.filter(t => (t.pnl || 0) > 0).length;
	const losing_trades = data.filter(t => (t.pnl || 0) < 0).length;
	const total_pnl = data.reduce((sum, t) => sum + (t.pnl || 0), 0);
	const win_rate = total_trades > 0 ? (winning_trades / total_trades) * 100 : 0;

	return {
		total_trades,
		winning_trades,
		losing_trades,
		total_pnl,
		win_rate
	};
}
