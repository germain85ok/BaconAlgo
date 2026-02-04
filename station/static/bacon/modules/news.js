mount(app) {
  const dash = document.querySelector('#bacon-dashboard');
  if (!dash) return; // sécurité

  dash.appendChild(el('h2', {}, ['Live News']));
  dash.appendChild(el('div', { id: 'bacon-news' }));

  fetchNews(app);
  setInterval(() => fetchNews(app), 15000);
}
