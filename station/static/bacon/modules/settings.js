import { el } from '../core/bacon-inject.js';

export function SettingsModule() {
  function open(app) {
    const modal = el('div', { style: `
      position: fixed; inset:0; z-index:10001;
      background: rgba(0,0,0,.75);
      display:flex; align-items:center; justify-content:center;
    `});

    const box = el('div', { style: `
      width: min(680px, 92vw);
      background: rgba(12,12,20,.92);
      border: 1px solid rgba(255,107,53,.35);
      border-radius: 16px;
      padding: 16px;
    `}, [
      el('div', { style: 'display:flex; justify-content:space-between; align-items:center; gap:10px;' }, [
        el('div', { class: 'bacon-sectionTitle' }, ['Settings']),
        el('button', { class: 'bacon-navBtn', id: 'close' }, ['✕'])
      ]),
      el('div', { style: 'margin-top: 10px; display:grid; gap: 10px; color: rgba(255,255,255,.8); font-weight: 700;' }, [
        el('label', {}, ['SSE URL']),
        el('input', { id: 'sse', value: app.state.sseUrl, style: 'padding:12px;border-radius:12px;border:1px solid rgba(255,107,53,.22);background: rgba(0,0,0,.22);color: rgba(255,255,255,.9); font-family: var(--mono);' }),
        el('button', { class: 'bacon-navBtn', id: 'save' }, ['Save'])
      ])
    ]);

    modal.appendChild(box);
    document.body.appendChild(modal);

    box.querySelector('#close')?.addEventListener('click', () => document.body.removeChild(modal));
    modal.addEventListener('click', (e) => { if (e.target === modal) document.body.removeChild(modal); });

    box.querySelector('#save')?.addEventListener('click', () => {
      const v = box.querySelector('#sse')?.value?.trim();
      if (v) {
        localStorage.setItem('bacon_sse_url', v);
        app.set({ sseUrl: v });
      }
      document.body.removeChild(modal);
      location.reload();
    });
  }

  return {
    name: 'settings',
    mount(app) {
      const saved = localStorage.getItem('bacon_sse_url');
      if (saved) app.set({ sseUrl: saved });

      const btn = el('button', { class: 'bacon-corner bacon-bl', title: 'Settings' }, ['⚙️']);
      btn.addEventListener('click', () => open(app));
      document.body.appendChild(btn);
    }
  };
}
