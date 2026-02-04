<script lang="ts">
	import { onMount } from 'svelte';
	
	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D;
	let animationFrameId: number;
	
	interface Particle {
		x: number;
		y: number;
		vx: number;
		vy: number;
		size: number;
	}
	
	let particles: Particle[] = [];
	const particleCount = 80;
	
	onMount(() => {
		ctx = canvas.getContext('2d')!;
		resizeCanvas();
		initParticles();
		animate();
		
		window.addEventListener('resize', resizeCanvas);
		
		return () => {
			if (animationFrameId) cancelAnimationFrame(animationFrameId);
			window.removeEventListener('resize', resizeCanvas);
		};
	});
	
	function resizeCanvas() {
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;
	}
	
	function initParticles() {
		particles = [];
		for (let i = 0; i < particleCount; i++) {
			particles.push({
				x: Math.random() * canvas.width,
				y: Math.random() * canvas.height,
				vx: (Math.random() - 0.5) * 0.5,
				vy: (Math.random() - 0.5) * 0.5,
				size: Math.random() * 2 + 1
			});
		}
	}
	
	function animate() {
		ctx.fillStyle = 'rgba(10, 14, 39, 0.1)';
		ctx.fillRect(0, 0, canvas.width, canvas.height);
		
		// Draw grid
		drawGrid();
		
		// Update and draw particles
		particles.forEach((particle, i) => {
			particle.x += particle.vx;
			particle.y += particle.vy;
			
			if (particle.x < 0 || particle.x > canvas.width) particle.vx *= -1;
			if (particle.y < 0 || particle.y > canvas.height) particle.vy *= -1;
			
			// Draw particle
			ctx.beginPath();
			ctx.arc(particle.x, particle.y, particle.size, 0, Math.PI * 2);
			ctx.fillStyle = 'rgba(255, 107, 53, 0.6)';
			ctx.fill();
			
			// Draw connections
			for (let j = i + 1; j < particles.length; j++) {
				const dx = particles[j].x - particle.x;
				const dy = particles[j].y - particle.y;
				const distance = Math.sqrt(dx * dx + dy * dy);
				
				if (distance < 150) {
					ctx.beginPath();
					ctx.strokeStyle = `rgba(255, 107, 53, ${0.2 * (1 - distance / 150)})`;
					ctx.lineWidth = 0.5;
					ctx.moveTo(particle.x, particle.y);
					ctx.lineTo(particles[j].x, particles[j].y);
					ctx.stroke();
				}
			}
		});
		
		animationFrameId = requestAnimationFrame(animate);
	}
	
	function drawGrid() {
		const gridSize = 50;
		ctx.strokeStyle = 'rgba(255, 107, 53, 0.05)';
		ctx.lineWidth = 1;
		
		for (let x = 0; x < canvas.width; x += gridSize) {
			ctx.beginPath();
			ctx.moveTo(x, 0);
			ctx.lineTo(x, canvas.height);
			ctx.stroke();
		}
		
		for (let y = 0; y < canvas.height; y += gridSize) {
			ctx.beginPath();
			ctx.moveTo(0, y);
			ctx.lineTo(canvas.width, y);
			ctx.stroke();
		}
	}
</script>

<canvas bind:this={canvas} class="animated-background"></canvas>

<style>
	.animated-background {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		z-index: 0;
		background: linear-gradient(135deg, #0a0e27 0%, #1a1f3a 50%, #0a0e27 100%);
	}
</style>
