export type SoundType = 'alert' | 'success' | 'warning' | 'error' | 'signal';

class AudioSystem {
	private lofiAudio: HTMLAudioElement | null = null;
	private soundEffects: Map<SoundType, HTMLAudioElement> = new Map();
	private isInitialized = false;
	private volume = 0.5;

	async initialize() {
		if (this.isInitialized) return;

		// Initialize lofi background music
		// In production, this would load actual audio files
		this.lofiAudio = new Audio();
		this.lofiAudio.loop = true;
		this.lofiAudio.volume = 0.3;

		// Initialize sound effects
		const soundTypes: SoundType[] = ['alert', 'success', 'warning', 'error', 'signal'];
		soundTypes.forEach(type => {
			const audio = new Audio();
			audio.volume = this.volume;
			this.soundEffects.set(type, audio);
		});

		this.isInitialized = true;
	}

	playLofi() {
		if (!this.lofiAudio) return;
		this.lofiAudio.play().catch(e => console.warn('Could not play lofi music:', e));
	}

	stopLofi() {
		if (!this.lofiAudio) return;
		this.lofiAudio.pause();
		this.lofiAudio.currentTime = 0;
	}

	playSound(type: SoundType) {
		const audio = this.soundEffects.get(type);
		if (!audio) return;
		
		audio.currentTime = 0;
		audio.play().catch(e => console.warn(`Could not play ${type} sound:`, e));
	}

	setVolume(volume: number) {
		this.volume = Math.max(0, Math.min(1, volume));
		
		if (this.lofiAudio) {
			this.lofiAudio.volume = this.volume * 0.3;
		}
		
		this.soundEffects.forEach(audio => {
			audio.volume = this.volume;
		});
	}

	getVolume(): number {
		return this.volume;
	}
}

export const audioSystem = new AudioSystem();
