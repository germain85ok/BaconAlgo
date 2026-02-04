import { el, q } from '../core/bacon-inject.js';

export function ScannerModule() {
  return {
    mount(app) {
      app.on('news', n => {
        n.symbols.forEach(sym => {
          const card = q(`[data-symbol="${sym}"]`);
          if (card) {
            card.classList.add('news-hit');
            setTimeout(() => card.classList.remove('news-hit'), 2000);
          }
        });
      });
    }
  };
}
