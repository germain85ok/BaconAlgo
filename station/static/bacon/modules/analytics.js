import { el } from '../core/bacon-inject.js';

async function loadChartJs() {
  if (window.Chart) return;
  const s = document.createElement('script');
  s.src = 'https://cdn.jsdelivr.net/npm/chart.js@4.4.0/dist/chart.umd.js';
  document.head.appendChild(s);
  await new Promise((res) => { s.onload = res; });
}

export function AnalyticsModule() {
  function open(app) {
    const modal = el('div', { style: `
      position: fixed; inset:0; z-index:10002;
      background: rgba(0,0,0,.75);
      display:flex; align-items:center; justify-content:center;
    `});

    const box = el('div', { style: `
      width: min(980px, 94vw);
      max-height: 82vh;
      overflow:auto;
      background: rgba(12,12,20,.92);
      border: 1px solid rgba(255,107,53,.35);
      border-radius: 16px;
      padding: 16px;
    `});

    box.innerHTML = `
      <div style="display:flex; justify-content:space-between; align-items:center; gap:10px;">
        <div class="bacon-sectionTitle">Analytics</div>
        <button class="bacon-navBtn" id="close">✕</button>
      </div>
      <div style="margin-top: 12px; display:grid; grid-template-columns: 1fr 1fr; gap: 12px;">
        <div style="background: rgba(0,0,0,.22); border:1px solid rgba(255,107,53,.18); border-radius:14px; padding:12px;">
          <div style="color: rgba(255,255,255,.7); font-weight: 800; margin-bottom: 8px;">Signals per grade</div>
          <canvas id="chart-grade"></canvas>
        </div>
        <div style="background: rgba(0,0,0,.22); border:1px solid rgba(255,107,53,.18); border-radius:14px; padding:12px;">
          <div style="color: rgba(255,255,255,.7); font-weight: 800; margin-bottom: 8px;">Scores distribution</div>
          <canvas id="chart-score"></canvas>
        </div>
      </div>
    `;

    modal.appendChild(box);
    document.body.appendChild(modal);

    box.querySelector('#close')?.addEventListener('click', () => document.body.removeChild(modal));
    modal.addEventListener('click', (e) => { if (e.target === modal) document.body.removeChild(modal); });

    setTimeout(() => {
      if (!window.Chart) return;

      const arr = Array.from(app.state.signals.values());
      const grades = { LEGENDARY: 0, EPIC: 0, RARE: 0 };
      for (const s of arr) {
        const g = s.score >= 85 ? 'LEGENDARY' : s.score >= 70 ? 'EPIC' : 'RARE';
        grades[g]++;
      }

      new Chart(document.getElementById('chart-grade'), {
        type: 'doughnut',
        data: {
          labels: Object.keys(grades),
          datasets: [{ data: Object.values(grades), backgroundColor: ['#ffd700', '#ffb347', '#00d9ff'] }]
        },
        options: { plugins: { legend: { labels: { color: '#94a3b8' } } } }
      });

      const buckets = new Array(10).fill(0);
      for (const s of arr) {
        const i = Math.min(9, Math.floor((s.score || 0) / 10));
        buckets[i]++;
      }

      new Chart(document.getElementById('chart-score'), {
        type: 'bar',
        data: {
          labels: ['0-9','10-19','20-29','30-39','40-49','50-59','60-69','70-79','80-89','90-100'],
          datasets: [{ data: buckets, backgroundColor: '#ff6b35' }]
        },
        options: {
          plugins: { legend: { display: false } },
          scales: {
            x: { ticks: { color: '#94a3b8' }, grid: { color: '#334155' } },
            y: { ticks: { color: '#94a3b8' }, grid: { color: '#334155' } }
          }
        }
      });
    }, 50);
  }

  return {
    name: 'analytics',
    mount() {
      const btn = el('button', { class: 'bacon-corner bacon-tr', title: 'Analytics' }, ['📊']);
      btn.addEventListener('click', async () => {
        await loadChartJs();
        // app passed via closure isn’t here; we bind in start:
      });
      document.body.appendChild(btn);
      btn._baconOpen = open; // stash
    },
    start(app) {
      const btn = document.querySelector('.bacon-tr');
      if (btn && btn._baconOpen) {
        btn.addEventListener('click', async () => {
          await loadChartJs();
          btn._baconOpen(app);
        });
      }
    }
  };
}
