/**
 * BaconAlgo 2040 - Animated Background
 * Fond animé avec particules et grille holographique
 */

<script lang="ts">
	import { onMount } from 'svelte';

	interface Props {
		variant?: 'particles' | 'grid' | 'gradient';
		intensity?: 'low' | 'medium' | 'high';
	}

	let { variant = 'particles', intensity = 'medium' }: Props = $props();

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null;
	let particles: Particle[] = [];
	let animationId: number;

	class Particle {
		x: number;
		y: number;
		size: number;
		speedX: number;
		speedY: number;
		opacity: number;

		constructor(width: number, height: number) {
			this.x = Math.random() * width;
			this.y = Math.random() * height;
			this.size = Math.random() * 2 + 1;
			this.speedX = (Math.random() - 0.5) * 0.5;
			this.speedY = (Math.random() - 0.5) * 0.5;
			this.opacity = Math.random() * 0.5 + 0.2;
		}

		update(width: number, height: number) {
			this.x += this.speedX;
			this.y += this.speedY;

			if (this.x < 0 || this.x > width) this.speedX *= -1;
			if (this.y < 0 || this.y > height) this.speedY *= -1;
		}

		draw(ctx: CanvasRenderingContext2D) {
			ctx.fillStyle = `rgba(255, 107, 53, ${this.opacity})`;
			ctx.beginPath();
			ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
			ctx.fill();
		}
	}

	function initParticles() {
		if (!canvas || !ctx) return;

		const particleCount = {
			low: 30,
			medium: 50,
			high: 100
		}[intensity];

		particles = [];
		for (let i = 0; i < particleCount; i++) {
			particles.push(new Particle(canvas.width, canvas.height));
		}
	}

	function drawGrid() {
		if (!ctx || !canvas) return;

		const gridSize = 40;
		ctx.strokeStyle = 'rgba(255, 107, 53, 0.1)';
		ctx.lineWidth = 1;

		// Lignes verticales
		for (let x = 0; x < canvas.width; x += gridSize) {
			ctx.beginPath();
			ctx.moveTo(x, 0);
			ctx.lineTo(x, canvas.height);
			ctx.stroke();
		}

		// Lignes horizontales
		for (let y = 0; y < canvas.height; y += gridSize) {
			ctx.beginPath();
			ctx.moveTo(0, y);
			ctx.lineTo(canvas.width, y);
			ctx.stroke();
		}
	}

	function animate() {
		if (!ctx || !canvas) return;

		ctx.clearRect(0, 0, canvas.width, canvas.height);

		if (variant === 'grid') {
			drawGrid();
		} else if (variant === 'particles') {
			particles.forEach((particle) => {
				particle.update(canvas.width, canvas.height);
				particle.draw(ctx!);
			});

			// Connexions entre particules
			particles.forEach((p1, i) => {
				particles.slice(i + 1).forEach((p2) => {
					const dx = p1.x - p2.x;
					const dy = p1.y - p2.y;
					const distance = Math.sqrt(dx * dx + dy * dy);

					if (distance < 150) {
						ctx!.strokeStyle = `rgba(255, 107, 53, ${0.2 * (1 - distance / 150)})`;
						ctx!.lineWidth = 1;
						ctx!.beginPath();
						ctx!.moveTo(p1.x, p1.y);
						ctx!.lineTo(p2.x, p2.y);
						ctx!.stroke();
					}
				});
			});
		}

		animationId = requestAnimationFrame(animate);
	}

	function handleResize() {
		if (!canvas) return;
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;
		initParticles();
	}

	onMount(() => {
		ctx = canvas.getContext('2d');
		handleResize();

		if (variant !== 'gradient') {
			animate();
		}

		window.addEventListener('resize', handleResize);

		return () => {
			window.removeEventListener('resize', handleResize);
			if (animationId) {
				cancelAnimationFrame(animationId);
			}
		};
	});
</script>

<div class="animated-background {variant}">
	{#if variant === 'gradient'}
		<div class="gradient-background"></div>
	{:else}
		<canvas bind:this={canvas} class="background-canvas"></canvas>
	{/if}

	<!-- Overlay gradient pour améliorer la lisibilité -->
	<div class="overlay"></div>
</div>

<style>
	.animated-background {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		z-index: -1;
		background: #0a0a0f;
		overflow: hidden;
	}

	.background-canvas {
		width: 100%;
		height: 100%;
		display: block;
	}

	.gradient-background {
		width: 100%;
		height: 100%;
		background: linear-gradient(
			135deg,
			#0a0a0f 0%,
			#1a0f1a 25%,
			#0a0a0f 50%,
			#0f1a1a 75%,
			#0a0a0f 100%
		);
		background-size: 400% 400%;
		animation: gradientShift 15s ease infinite;
	}

	@keyframes gradientShift {
		0% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
		100% {
			background-position: 0% 50%;
		}
	}

	.overlay {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: radial-gradient(
			ellipse at center,
			transparent 0%,
			rgba(10, 10, 15, 0.5) 100%
		);
		pointer-events: none;
	}

	/* Blobs d'ambiance */
	.animated-background::before,
	.animated-background::after {
		content: '';
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.3;
		animation: float 20s ease-in-out infinite;
	}

	.animated-background::before {
		top: 10%;
		left: 20%;
		width: 500px;
		height: 500px;
		background: radial-gradient(circle, #ff6b35 0%, transparent 70%);
		animation-delay: 0s;
	}

	.animated-background::after {
		bottom: 10%;
		right: 20%;
		width: 400px;
		height: 400px;
		background: radial-gradient(circle, #00d9ff 0%, transparent 70%);
		animation-delay: 10s;
	}

	@keyframes float {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(30px, -30px) scale(1.1);
		}
		66% {
			transform: translate(-20px, 20px) scale(0.9);
		}
	}
</style>
