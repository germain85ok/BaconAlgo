import { el } from '../core/bacon-inject.js';

export function DiscordModule() {
  const key = 'bacon_discord_webhook';

  async function send(webhook, payload) {
    if (!webhook) return { ok: false };
    try {
      await fetch(webhook, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
      return { ok: true };
    } catch {
      return { ok: false };
    }
  }

  function open(app) {
    const modal = el('div', { style: `
      position: fixed; inset:0; z-index:10004;
      background: rgba(0,0,0,.75);
      display:flex; align-items:center; justify-content:center;
    `});

    const box = el('div', { style: `
      width: min(700px, 94vw);
      background: rgba(12,12,20,.92);
      border: 1px solid rgba(255,107,53,.35);
      border-radius: 16px;
      padding: 16px;
    `});

    const existing = localStorage.getItem(key) || '';

    box.innerHTML = `
      <div style="display:flex; justify-content:space-between; align-items:center; gap:10px;">
        <div class="bacon-sectionTitle">Discord</div>
        <button class="bacon-navBtn" id="close">✕</button>
      </div>
      <div style="margin-top: 10px; color: rgba(255,255,255,.75); font-size: 12px; line-height: 1.45;">
        Colle ton webhook ici (évite de le committer). Si tu l’avais déjà posté quelque part, régénère-le.
      </div>
      <input id="wh" value="${existing.replaceAll('"','&quot;')}" placeholder="Discord webhook URL"
        style="margin-top:10px; width:100%; padding:12px;border-radius:12px;border:1px solid rgba(255,107,53,.22);background: rgba(0,0,0,.22);color: rgba(255,255,255,.9); font-family: var(--mono);" />
      <div style="margin-top: 10px; display:flex; gap:10px;">
        <button class="bacon-navBtn" id="save">Save</button>
        <button class="bacon-navBtn" id="test">Test ping</button>
      </div>
    `;

    modal.appendChild(box);
    document.body.appendChild(modal);

    box.querySelector('#close')?.addEventListener('click', () => document.body.removeChild(modal));
    modal.addEventListener('click', (e) => { if (e.target === modal) document.body.removeChild(modal); });

    box.querySelector('#save')?.addEventListener('click', () => {
      const v = box.querySelector('#wh')?.value?.trim() || '';
      localStorage.setItem(key, v);
      document.body.removeChild(modal);
    });

    box.querySelector('#test')?.addEventListener('click', async () => {
      const v = box.querySelector('#wh')?.value?.trim() || '';
      const res = await send(v, { content: '🥓 BaconAlgo test ping' });
      if (!res.ok) alert('Discord ping failed (webhook invalid / blocked).');
      else alert('Discord ping sent.');
    });
  }

  return {
    name: 'discord',
    mount(app) {
      const btn = el('button', { class: 'bacon-corner bacon-tl', title: 'Discord' }, ['💬']);
      btn.addEventListener('click', () => open(app));
      document.body.appendChild(btn);

      // Example: notify on new READY flips (basic)
      let lastReady = new Map();
      app.on('state', async () => {
        const webhook = localStorage.getItem(key) || '';
        if (!webhook) return;

        for (const s of app.state.signals.values()) {
          const prev = lastReady.get(s.symbol);
          if (prev === undefined) { lastReady.set(s.symbol, s.ready); continue; }
          if (prev !== s.ready) {
            lastReady.set(s.symbol, s.ready);
            await send(webhook, {
              username: '🥓 BaconAlgo',
              content: `${s.ready ? '✅ READY' : '⛔ NOT READY'} — ${s.symbol} (${s.score}/100)\n${s.reason || ''}`
            });
          }
        }
      });
    }
  };
}
