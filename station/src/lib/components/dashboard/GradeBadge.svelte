<script lang="ts">
  type Grade = 'S' | 'A' | 'B' | 'C';

  interface Props {
    grade: Grade;
  }

  let { grade }: Props = $props();

  const gradeMap: Record<string, { name: string; icon: string; class: string }> = {
    'S': { name: 'Legendary', icon: '👑', class: 'legendary' },
    'A': { name: 'Epic', icon: '💎', class: 'epic' },
    'B': { name: 'Rare', icon: '💠', class: 'rare' },
    'C': { name: 'Common', icon: '⚪', class: 'common' }
  };

  const config = $derived(gradeMap[grade] || gradeMap['C']);
  const shouldShimmer = $derived(config.class === 'legendary' || config.class === 'epic');
</script>

<div class="grade-badge {config.class}" class:shimmer={shouldShimmer}>
  <span class="icon">{config.icon}</span>
  <span class="name">{config.name}</span>
</div>

<style>
  .grade-badge {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.75rem;
    border-radius: 9999px;
    font-size: 0.875rem;
    font-weight: 600;
    position: relative;
    overflow: hidden;
    transition: all 0.3s ease;
  }

  .icon {
    font-size: 1rem;
    line-height: 1;
  }

  .name {
    line-height: 1;
  }

  /* Legendary - Gold gradient with glow */
  .legendary {
    background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 50%, #d97706 100%);
    color: #1f2937;
    box-shadow: 0 0 20px rgba(251, 191, 36, 0.6),
                0 0 40px rgba(251, 191, 36, 0.3);
  }

  /* Epic - Purple-pink gradient with glow */
  .epic {
    background: linear-gradient(135deg, #a855f7 0%, #ec4899 100%);
    color: #fff;
    box-shadow: 0 0 20px rgba(168, 85, 247, 0.6),
                0 0 40px rgba(236, 72, 153, 0.3);
  }

  /* Rare - Blue-cyan gradient with glow */
  .rare {
    background: linear-gradient(135deg, #3b82f6 0%, #06b6d4 100%);
    color: #fff;
    box-shadow: 0 0 15px rgba(59, 130, 246, 0.4);
  }

  /* Common - Gray gradient */
  .common {
    background: linear-gradient(135deg, #6b7280 0%, #4b5563 100%);
    color: #e5e7eb;
    box-shadow: 0 0 10px rgba(107, 114, 128, 0.2);
  }

  /* Shimmer animation for Legendary and Epic */
  .shimmer::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.4),
      transparent
    );
    animation: shimmer 2s infinite;
  }

  @keyframes shimmer {
    0% {
      left: -100%;
    }
    100% {
      left: 100%;
    }
  }

  /* Hover effects */
  .legendary:hover {
    transform: scale(1.05);
    box-shadow: 0 0 30px rgba(251, 191, 36, 0.8),
                0 0 60px rgba(251, 191, 36, 0.4);
  }

  .epic:hover {
    transform: scale(1.05);
    box-shadow: 0 0 30px rgba(168, 85, 247, 0.8),
                0 0 60px rgba(236, 72, 153, 0.4);
  }

  .rare:hover {
    transform: scale(1.05);
    box-shadow: 0 0 25px rgba(59, 130, 246, 0.6);
  }

  .common:hover {
    transform: scale(1.05);
  }

  /* Responsive */
  @media (max-width: 640px) {
    .grade-badge {
      font-size: 0.75rem;
      padding: 0.25rem 0.5rem;
      gap: 0.25rem;
    }

    .icon {
      font-size: 0.875rem;
    }
  }
</style>
