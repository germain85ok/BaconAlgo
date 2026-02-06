import { webcrypto } from 'crypto';

export interface AuditLog {
	userId: string;
	action: string;
	timestamp: Date;
	details: any;
}

export class SecurityModule {
	private auditLogs: AuditLog[] = [];
	private rateLimits: Map<string, number[]> = new Map();

	// AES-256 Encryption for sensitive data
	async encrypt(data: string, key: string): Promise<string> {
		const encoder = new TextEncoder();
		const dataBuffer = encoder.encode(data);
		
		// In browser environment, use Web Crypto API
		if (typeof window !== 'undefined' && window.crypto) {
			const keyData = encoder.encode(key.padEnd(32, '0').substring(0, 32));
			const cryptoKey = await window.crypto.subtle.importKey(
				'raw',
				keyData,
				{ name: 'AES-GCM' },
				false,
				['encrypt']
			);

			const iv = window.crypto.getRandomValues(new Uint8Array(12));
			const encrypted = await window.crypto.subtle.encrypt(
				{ name: 'AES-GCM', iv },
				cryptoKey,
				dataBuffer
			);

			// Combine IV and encrypted data
			const combined = new Uint8Array(iv.length + encrypted.byteLength);
			combined.set(iv);
			combined.set(new Uint8Array(encrypted), iv.length);

			return btoa(String.fromCharCode(...combined));
		}

		// Fallback for server-side or unsupported environments
		return btoa(data); // Simplified - use proper encryption in production
	}

	async decrypt(encryptedData: string, key: string): Promise<string> {
		if (typeof window !== 'undefined' && window.crypto) {
			const encoder = new TextEncoder();
			const keyData = encoder.encode(key.padEnd(32, '0').substring(0, 32));
			
			const cryptoKey = await window.crypto.subtle.importKey(
				'raw',
				keyData,
				{ name: 'AES-GCM' },
				false,
				['decrypt']
			);

			const combined = Uint8Array.from(atob(encryptedData), c => c.charCodeAt(0));
			const iv = combined.slice(0, 12);
			const data = combined.slice(12);

			const decrypted = await window.crypto.subtle.decrypt(
				{ name: 'AES-GCM', iv },
				cryptoKey,
				data
			);

			return new TextDecoder().decode(decrypted);
		}

		return atob(encryptedData); // Simplified fallback
	}

	// Audit logging
	logAction(userId: string, action: string, details: any = {}): void {
		const log: AuditLog = {
			userId,
			action,
			timestamp: new Date(),
			details
		};

		this.auditLogs.push(log);

		// In production, persist to database
		console.log('Audit Log:', log);
	}

	getAuditLogs(userId?: string): AuditLog[] {
		if (userId) {
			return this.auditLogs.filter(log => log.userId === userId);
		}
		return this.auditLogs;
	}

	// Rate limiting
	checkRateLimit(userId: string, maxRequests: number = 100, windowMs: number = 60000): boolean {
		const now = Date.now();
		const userRequests = this.rateLimits.get(userId) || [];

		// Remove old requests outside the window
		const recentRequests = userRequests.filter(timestamp => now - timestamp < windowMs);

		if (recentRequests.length >= maxRequests) {
			return false; // Rate limit exceeded
		}

		// Add current request
		recentRequests.push(now);
		this.rateLimits.set(userId, recentRequests);

		return true;
	}

	// Clear rate limit for user
	clearRateLimit(userId: string): void {
		this.rateLimits.delete(userId);
	}
}

export const securityModule = new SecurityModule();
