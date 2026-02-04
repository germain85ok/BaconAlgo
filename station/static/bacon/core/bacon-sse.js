export function startSse({ url, onOpen, onError, onMessage }) {
  const es = new EventSource(url);

  es.onopen = () => onOpen?.();
  es.onerror = () => onError?.();
  es.onmessage = (ev) => onMessage?.(ev.data);

  return () => es.close();
}
