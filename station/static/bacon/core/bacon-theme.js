export function applyBaconTheme() {
  const s = document.createElement('style');
  s.textContent = `
    body { font-family: Inter, system-ui; color: white; }
    .bacon-bg {
      position: fixed; inset: 0;
      background: radial-gradient(circle at 20% 10%, rgba(255,107,53,.15), transparent 40%);
      z-index: -1;
    }
    .news-hit {
      box-shadow: 0 0 30px gold;
      animation: pulse 1s ease-out;
    }
    @keyframes pulse {
      0% { transform: scale(1); }
      50% { transform: scale(1.03); }
      100% { transform: scale(1); }
    }
  `;
  document.head.appendChild(s);
}
