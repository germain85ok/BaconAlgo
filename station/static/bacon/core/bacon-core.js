import { q, el } from './bacon-inject.js';

export const Bacon = {
  root: null,
  modules: [],
  handlers: {},

  state: {
    connected: false,
    sseUrl: '',
    signals: new Map(),
    news: []
  },

  init({ rootId, sseUrl }) {
    this.root = q(`#${rootId}`);
    this.state.sseUrl = sseUrl;
  },

  on(event, fn) {
    (this.handlers[event] ||= []).push(fn);
  },

  emit(event, payload) {
    (this.handlers[event] || []).forEach(fn => {
      try { fn(payload); } catch {}
    });
  },

  use(module) {
    this.modules.push(module);
  },

  start() {
    if (!this.root) {
      console.error('Bacon root not found');
      return;
    }

    this.root.innerHTML = '';

    const app = el('div', { class: 'bacon-app' }, [
      el('div', { class: 'bacon-bg' }),
      el('header', { class: 'bacon-top' }, [
        el('div', { class: 'bacon-brand' }, ['🥓 BaconAlgo']),
        el('div', { id: 'bacon-status' }, ['DISCONNECTED'])
      ]),
      el('main', { id: 'bacon-dashboard' })
    ]);

    this.root.appendChild(app);

    // Monter les modules APRÈS que le DOM existe
    this.modules.forEach(m => m.mount?.(this));
    this.modules.forEach(m => m.start?.(this));
  }
};
