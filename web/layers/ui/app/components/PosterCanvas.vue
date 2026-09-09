<script setup lang="ts">
import { mediaUrl } from '~/utils/site'

type Pos = {
  left?: number
  top?: number
  size?: number
  color?: string
  width?: number
}

const props = defineProps<{
  pic?: string | null
  configPos?: string | null
  fields?: Record<string, string> | null
}>()

const canvasRef = ref<HTMLCanvasElement | null>(null)
const failed = ref(false)

function num(v: unknown, fallback = 0): number {
  const n = Number(v)
  return Number.isFinite(n) ? n : fallback
}

function parsePos(raw?: string | null): Record<string, Pos> {
  const out: Record<string, Pos> = {}
  const text = String(raw || '').trim()
  if (!text) return out
  try {
    const parsed = JSON.parse(text) as unknown
    const walk = (node: unknown) => {
      if (!node) return
      if (Array.isArray(node)) {
        for (const item of node) walk(item)
        return
      }
      if (typeof node !== 'object') return
      const row = node as Record<string, unknown>
      if (row.text && typeof row.text === 'object') walk(row.text)
      if (row.font && typeof row.font === 'object') walk(row.font)
      for (const [k, v] of Object.entries(row)) {
        if (v && typeof v === 'object' && !Array.isArray(v)) {
          const p = v as Record<string, unknown>
          if (p.left != null || p.top != null || p.x != null || p.y != null) {
            out[k] = {
              left: num(p.left ?? p.x, 24),
              top: num(p.top ?? p.y, 24),
              size: num(p.size ?? p.fontSize ?? p['font-size'], 22),
              color: String(p.color || '#111'),
              width: num(p.width, 0),
            }
          }
        }
      }
    }
    walk(parsed)
  } catch {
    /* ignore malformed template json */
  }
  return out
}

async function draw() {
  failed.value = false
  const canvas = canvasRef.value
  if (!canvas) return
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  const src = mediaUrl(props.pic || '', '')
  const fields = props.fields || {}
  const pos = parsePos(props.configPos)
  const keys = Object.keys(fields)
  const img = src
    ? await new Promise<HTMLImageElement | null>((resolve) => {
        const el = new Image()
        el.crossOrigin = 'anonymous'
        el.onload = () => resolve(el)
        el.onerror = () => resolve(null)
        el.src = src
      })
    : null
  const width = img?.naturalWidth || 600
  const height = img?.naturalHeight || 800
  canvas.width = width
  canvas.height = height
  if (img) {
    ctx.drawImage(img, 0, 0, width, height)
  } else {
    ctx.fillStyle = '#f4f4f4'
    ctx.fillRect(0, 0, width, height)
    if (src) failed.value = true
  }
  ctx.textBaseline = 'top'
  if (Object.keys(pos).length) {
    for (const [key, p] of Object.entries(pos)) {
      const text = String(fields[key] || '')
      if (!text) continue
      ctx.fillStyle = p.color || '#111'
      ctx.font = `bold ${p.size || 22}px sans-serif`
      if (p.width && p.width > 0) {
        ctx.fillText(text, p.left || 24, p.top || 24, p.width)
      } else {
        ctx.fillText(text, p.left || 24, p.top || 24)
      }
    }
    return
  }
  let y = 32
  ctx.fillStyle = '#111'
  ctx.font = 'bold 22px sans-serif'
  for (const key of keys) {
    const text = String(fields[key] || '').trim()
    if (!text) continue
    ctx.fillText(text, 24, y, width - 48)
    y += 36
  }
}

watch(
  () => [props.pic, props.configPos, JSON.stringify(props.fields || {})],
  () => {
    nextTick(() => {
      void draw()
    })
  },
  { immediate: true },
)
</script>

<template>
  <div class="poster-canvas">
    <canvas ref="canvasRef" />
    <p v-if="failed" class="muted">{{ $t('common_02409') }}</p>
  </div>
</template>

<style scoped>
.poster-canvas {
  max-width: 100%;
}
canvas {
  display: block;
  max-width: 100%;
  height: auto;
}
</style>
