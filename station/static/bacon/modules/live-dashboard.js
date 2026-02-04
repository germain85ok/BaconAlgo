import { q, el } from '../core/bacon-inject.js';

export function LiveDashboardModule() {
  function renderTop(app) {
    const dash = q('#bacon-dashboard');
    if (!dash) return;

    const top = Array.from(app.state.signals.values())
      .sort((a, b) => (b.score - a.score) || a.symbol.localeCompare(b.symbol))
      .slice(0, 2);

    let host = q('#bacon-topSignals', dash);
    if (!host) {
      host = el('div', { id: 'bacon-topSignals', style: 'margin-top: 14px; display:grid; grid-template-columns: 1fr 1fr; gap: 10px;' });
      dash.appendChild(host);
    }

    const mk = (rank, s) => el('div', {
      style: `
        position: relative;
        background: rgba(0,0,0,.22);
        border: 1px solid rgba(255,107,53,.22);
        border-radius: 16px;
        padding: 14px;
        overflow: hidden;
      `
    }, [
      el('div', { style: 'display:flex; justify-content:space-between; align-items:center; gap:10px;' }, [
        el('div', { style: 'font-family: var(--orb); font-weight: 900; letter-spacing:.06em;' }, [`#${rank} ${s?.symbol || '—'}`]),
        el('div', { style: `
          font-family: var(--mono); font-weight: 900;
          padding: 6px 10px; border-radius: 12px;
          border: 1px solid rgba(255,255,255,.10);
          background: rgba(255,179,71,.12);
        ` }, [s ? `${s.score}/100` : '—'])
      ]),
      el('div', { style: 'margin-top: 10px; color: rgba(255,255,255,.70); min-height: 36px; font-size: 12px; line-height: 1.35;' }, [s?.reason || 'Waiting…']),
      el('div', { style: 'margin-top: 10px; display:grid; grid-template-columns: 1fr 1fr; gap: 8px;' }, [
        el('button', { class: 'bacon-navBtn', style: 'width:100%;' }, ['+ Portfolio']),
        el('button', { class: 'bacon-navBtn', style: 'width:100%;' }, ['📊 Chart'])
      ])
    ]);

    host.innerHTML = '';
    host.appendChild(mk(1, top[0]));
    host.appendChild(mk(2, top[1]));

    const cols = window.innerWidth < 900 ? '1fr' : '1fr 1fr';
    host.style.gridTemplateColumns = cols;
  }

  return {
    name: 'live-dashboard',
    mount(app) {
      const dash = q('#bacon-dashboard');
      dash?.appendChild(el('div', { class: 'bacon-sectionTitle', style: 'margin-top: 14px;' }, ['Top signals']));
      app.on('state', () => renderTop(app));
      window.addEventListener('resize', () => renderTop(app));
    }
  };
}
