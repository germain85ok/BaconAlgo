<script lang="ts">
	import { onMount } from 'svelte';

	export let trigger = false;
	export let colors: string[] = ['#FFD700', '#FF6B35', '#F7931E', '#FFA500', '#FF1493'];

	interface Confetti {
		x: number;
		y: number;
		vx: number;
		vy: number;
		rotation: number;
		rotationSpeed: number;
		color: string;
		size: number;
	}

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null;
	let confettiPieces: Confetti[] = [];
	let animationFrame: number;

	const gravity = 0.5;
	const confettiCount = 50;

	function createConfettiPiece(): Confetti {
		return {
			x: Math.random() * (canvas?.width || 800),
			y: -20,
			vx: (Math.random() - 0.5) * 10,
			vy: Math.random() * -15 - 5,
			rotation: Math.random() * 360,
			rotationSpeed: (Math.random() - 0.5) * 10,
			color: colors[Math.floor(Math.random() * colors.length)],
			size: Math.random() * 10 + 5
		};
	}

	function animate() {
		if (!ctx || !canvas) return;

		ctx.clearRect(0, 0, canvas.width, canvas.height);

		confettiPieces = confettiPieces.filter((confetti) => {
			confetti.vy += gravity;
			confetti.x += confetti.vx;
			confetti.y += confetti.vy;
			confetti.rotation += confetti.rotationSpeed;

			if (confetti.y > canvas.height) return false;

			ctx!.save();
			ctx!.translate(confetti.x, confetti.y);
			ctx!.rotate((confetti.rotation * Math.PI) / 180);
			ctx!.fillStyle = confetti.color;
			ctx!.fillRect(-confetti.size / 2, -confetti.size / 2, confetti.size, confetti.size);
			ctx!.restore();

			return true;
		});

		if (confettiPieces.length > 0) {
			animationFrame = requestAnimationFrame(animate);
		}
	}

	function startConfetti() {
		if (!canvas) return;

		for (let i = 0; i < confettiCount; i++) {
			confettiPieces.push(createConfettiPiece());
		}

		if (animationFrame) cancelAnimationFrame(animationFrame);
		animate();
	}

	$: if (trigger && canvas) {
		startConfetti();
	}

	onMount(() => {
		if (canvas) {
			ctx = canvas.getContext('2d');
			canvas.width = canvas.offsetWidth;
			canvas.height = canvas.offsetHeight;
		}

		return () => {
			if (animationFrame) cancelAnimationFrame(animationFrame);
		};
	});
</script>

<canvas bind:this={canvas} class="confetti-canvas" />

<style>
	.confetti-canvas {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		pointer-events: none;
		z-index: 9999;
	}
</style>
