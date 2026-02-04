export function q(sel, root = document) {
  return root.querySelector(sel);
}

export function el(tag, attrs = {}, children = []) {
  const n = document.createElement(tag);
  Object.entries(attrs).forEach(([k, v]) => {
    if (k === 'class') n.className = v;
    else if (k === 'style') n.setAttribute('style', v);
    else n.setAttribute(k, v);
  });
  children.forEach(c => {
    if (typeof c === 'string') n.appendChild(document.createTextNode(c));
    else if (c) n.appendChild(c);
  });
  return n;
}
