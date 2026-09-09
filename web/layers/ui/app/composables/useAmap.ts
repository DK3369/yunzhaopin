type AMapWin = Window & {
  AMap?: {
    Map: new (el: HTMLElement, opts: Record<string, unknown>) => {
      on: (ev: string, fn: (e: { lnglat: { getLng: () => number; getLat: () => number } }) => void) => void
      setCenter: (pos: [number, number]) => void
    }
    Marker: new (opts: Record<string, unknown>) => { setPosition: (pos: [number, number]) => void }
  }
  _AMapSecurityConfig?: { securityJsCode: string }
}

let loading: Promise<boolean> | null = null

export async function loadAmap(key: string, secret?: string): Promise<boolean> {
  if (!import.meta.client || !key) return false
  const w = window as AMapWin
  if (w.AMap) return true
  if (!loading) {
    loading = new Promise((resolve) => {
      if (secret) w._AMapSecurityConfig = { securityJsCode: secret }
      const s = document.createElement('script')
      s.src = `https://webapi.amap.com/maps?v=2.0&key=${encodeURIComponent(key)}`
      s.onload = () => resolve(true)
      s.onerror = () => resolve(false)
      document.head.appendChild(s)
    })
  }
  return loading
}

export function mapCenter(settings: Record<string, string>, x?: string, y?: string): [number, number] {
  const lng = Number(x || settings.map_x || 116.397428)
  const lat = Number(y || settings.map_y || 39.90923)
  return [Number.isFinite(lng) ? lng : 116.397428, Number.isFinite(lat) ? lat : 39.90923]
}
