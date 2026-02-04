import { el } from '../core/bacon-inject.js';

export function PortfolioModule() {
  const key = 'bacon_portfolio';
  const load = () => {
    try { return JSON.parse(localStorage.getItem(key) || '[]'); } catch { return []; }
  };
  const save = (arr) => localStorage.setItem(key, JSON.stringify(arr));

  function open(app) {
    const modal = el('div', { style: `
      position: fixed; inset:0; z-index:10003;
      background: rgba(0,0,0,.75);
      display:flex; align-items:center; justify-content:center;
    `});

    const box = el('div', { style: `
      width: min(820px, 94vw);
      max-height: 82vh;
      overflow:auto;
      background: rgba(12,12,20,.92);
      border: 1px solid rgba(255,107,53,.35);
      border-radius: 16px;
      padding: 16px;
    `});

    const items = load();

    box.innerHTML = `
      <div style="display:flex; justify-content:space-between; align-items:center; gap:10px;">
        <div class="bacon-sectionTitle">Portfolio</div>
        <button class="bacon-navBtn" id="close">✕</button>
      </div>
      <div style="margin-top: 12px; color: rgba(255,255,255,.75); font-family: var(--mono);">
        ${items.length ? items.map(x => `• ${x.symbol} (${x.score}/100)`).join('<br/>') : 'Empty'}
      </div>
      <div style="margin-top: 12px; display:flex; gap:10px;">
        <button class="bacon-navBtn" id="clear">Clear</button>
      </div>
    `;

    modal.appendChild(box);
    document.body.appendChild(modal);

    box.querySelector('#close')?.addEventListener('click', () => document.body.removeChild(modal));
    modal.addEventListener('click', (e) => { if (e.target === modal) document.body.removeChild(modal); });

    box.querySelector('#clear')?.addEventListener('click', () => {
      save([]);
      document.body.removeChild(modal);
    });
  }

  return {
    name: 'portfolio',
    mount(app) {
      const btn = el('button', { class: 'bacon-corner bacon-br', title: 'Portfolio' }, ['💼']);
      btn.addEventListener('click', () => open(app));
      document.body.appendChild(btn);

      app.on('portfolio:add', (s) => {
        const items = load();
        if (!items.find(x => x.symbol === s.symbol)) items.push({ symbol: s.symbol, score: s.score, ts: Date.now() });
        save(items);
      });
    }
  };
}
