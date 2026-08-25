export function stripHtml(value: unknown): string {
  return String(value ?? '')
    .replace(/<[^>]+>/g, ' ')
    .replace(/\s+/g, ' ')
    .trim()
}

export function unixToIso(ts: unknown): string | undefined {
  const n = Number(ts)
  if (!Number.isFinite(n) || n <= 0) return undefined
  const ms = n > 1e12 ? n : n * 1000
  const d = new Date(ms)
  if (Number.isNaN(d.getTime())) return undefined
  return d.toISOString()
}
