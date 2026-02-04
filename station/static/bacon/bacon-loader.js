import { Bacon } from './core/bacon-core.js';

Bacon.init({
  rootId: 'bacon-root',
  sseUrl: '/signals/live'
});

Bacon.start();
