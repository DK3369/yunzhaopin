/** Byte-mode QR (versions 1–8, ECC M) → SVG data URI. */

const ECC_PER_BLOCK = [-1, 10, 16, 26, 18, 24, 16, 18, 22]
const ECC_BLOCKS = [-1, 1, 1, 1, 2, 2, 4, 4, 4]
const ALIGN_POS = [
  [],
  [],
  [18],
  [22],
  [26],
  [30],
  [34],
  [22, 38],
  [24, 42],
]
const TOTAL_CW = [-1, 26, 44, 70, 100, 134, 172, 196, 242]

function gfExp(): Uint8Array {
  const exp = new Uint8Array(512)
  let x = 1
  for (let i = 0; i < 255; i++) {
    exp[i] = x
    x <<= 1
    if (x & 0x100) x ^= 0x11d
  }
  for (let i = 255; i < 512; i++) exp[i] = exp[i - 255]
  return exp
}

function gfLog(exp: Uint8Array): Uint8Array {
  const log = new Uint8Array(256)
  for (let i = 0; i < 255; i++) log[exp[i]] = i
  return log
}

const EXP = gfExp()
const LOG = gfLog(EXP)

function gfMul(a: number, b: number): number {
  if (!a || !b) return 0
  return EXP[LOG[a] + LOG[b]]
}

function rsRemainder(data: Uint8Array, degree: number): Uint8Array {
  let poly = new Uint8Array([1])
  for (let i = 0; i < degree; i++) {
    const next = new Uint8Array(poly.length + 1)
    for (let j = 0; j < poly.length; j++) {
      next[j] ^= poly[j]
      next[j + 1] ^= gfMul(poly[j], EXP[i])
    }
    poly = next
  }
  const r = new Uint8Array(degree)
  for (const b of data) {
    const factor = b ^ r[0]
    r.copyWithin(0, 1)
    r[degree - 1] = 0
    if (!factor) continue
    for (let i = 0; i < degree; i++) r[i] ^= gfMul(poly[i + 1], factor)
  }
  return r
}

function bitsToBytes(bits: number[]): Uint8Array {
  const out = new Uint8Array(Math.ceil(bits.length / 8))
  for (let i = 0; i < bits.length; i++) {
    if (bits[i]) out[i >> 3] |= 1 << (7 - (i & 7))
  }
  return out
}

function pushBits(bits: number[], val: number, n: number) {
  for (let i = n - 1; i >= 0; i--) bits.push((val >> i) & 1)
}

function dataBits(text: string, version: number): number[] {
  const bytes = new TextEncoder().encode(text)
  const bits: number[] = []
  pushBits(bits, 0b0100, 4)
  pushBits(bits, bytes.length, version >= 10 ? 16 : 8)
  for (const b of bytes) pushBits(bits, b, 8)
  const dataCw = TOTAL_CW[version] - ECC_PER_BLOCK[version] * ECC_BLOCKS[version]
  const capacity = dataCw * 8
  const remain = capacity - bits.length
  pushBits(bits, 0, Math.min(4, Math.max(0, remain)))
  while (bits.length % 8) bits.push(0)
  let pad = 0xec
  while (bits.length < capacity) {
    pushBits(bits, pad, 8)
    pad = pad === 0xec ? 0x11 : 0xec
  }
  return bits
}

function interleave(version: number, data: Uint8Array): Uint8Array {
  const nBlocks = ECC_BLOCKS[version]
  const ecn = ECC_PER_BLOCK[version]
  const dataCw = TOTAL_CW[version] - ecn * nBlocks
  const shortBlocks = nBlocks - (dataCw % nBlocks)
  const shortLen = Math.floor(dataCw / nBlocks)
  const blocks: Uint8Array[] = []
  const eccs: Uint8Array[] = []
  let off = 0
  for (let i = 0; i < nBlocks; i++) {
    const len = shortLen + (i < shortBlocks ? 0 : 1)
    const block = data.slice(off, off + len)
    off += len
    blocks.push(block)
    eccs.push(rsRemainder(block, ecn))
  }
  const out = new Uint8Array(TOTAL_CW[version])
  let p = 0
  const maxData = shortLen + 1
  for (let i = 0; i < maxData; i++) {
    for (const b of blocks) {
      if (i < b.length) out[p++] = b[i]
    }
  }
  for (let i = 0; i < ecn; i++) {
    for (const e of eccs) out[p++] = e[i]
  }
  return out
}

function sizeOf(version: number): number {
  return 21 + 4 * (version - 1)
}

function inFinder(x: number, y: number, n: number): boolean {
  return (x < 9 && y < 9) || (x >= n - 8 && y < 9) || (x < 9 && y >= n - 8)
}

function placeFinders(mod: number[][], n: number) {
  const draw = (ox: number, oy: number) => {
    for (let y = -1; y <= 7; y++) {
      for (let x = -1; x <= 7; x++) {
        const xx = ox + x
        const yy = oy + y
        if (xx < 0 || yy < 0 || xx >= n || yy >= n) continue
        const ring = x >= 0 && x <= 6 && y >= 0 && y <= 6 && (x === 0 || x === 6 || y === 0 || y === 6)
        const core = x >= 2 && x <= 4 && y >= 2 && y <= 4
        const sep = x === -1 || x === 7 || y === -1 || y === 7
        if (sep) mod[yy][xx] = 0
        else if (ring || core) mod[yy][xx] = 1
        else if (x >= 0 && x <= 6 && y >= 0 && y <= 6) mod[yy][xx] = 0
      }
    }
  }
  draw(0, 0)
  draw(n - 7, 0)
  draw(0, n - 7)
}

function placeTiming(mod: number[][], n: number) {
  for (let i = 8; i < n - 8; i++) {
    mod[6][i] = i % 2 === 0 ? 1 : 0
    mod[i][6] = i % 2 === 0 ? 1 : 0
  }
}

function placeAlign(mod: number[][], version: number, n: number) {
  const pos = ALIGN_POS[version]
  for (const oy of pos) {
    for (const ox of pos) {
      if (inFinder(ox, oy, n)) continue
      for (let y = -2; y <= 2; y++) {
        for (let x = -2; x <= 2; x++) {
          const xx = ox + x
          const yy = oy + y
          const ring = Math.max(Math.abs(x), Math.abs(y)) === 2
          const core = x === 0 && y === 0
          mod[yy][xx] = ring || core ? 1 : 0
        }
      }
    }
  }
}

function reserved(x: number, y: number, n: number, version: number): boolean {
  if (inFinder(x, y, n)) return true
  if (x === 6 || y === 6) return true
  if (y === 8 && (x <= 8 || x >= n - 8)) return true
  if (x === 8 && (y <= 8 || y >= n - 8)) return true
  const pos = ALIGN_POS[version]
  for (const oy of pos) {
    for (const ox of pos) {
      if (inFinder(ox, oy, n)) continue
      if (Math.abs(x - ox) <= 2 && Math.abs(y - oy) <= 2) return true
    }
  }
  return false
}

function maskBit(mask: number, x: number, y: number): boolean {
  switch (mask) {
    case 0:
      return (x + y) % 2 === 0
    case 1:
      return y % 2 === 0
    case 2:
      return x % 3 === 0
    case 3:
      return (x + y) % 3 === 0
    case 4:
      return (Math.floor(y / 2) + Math.floor(x / 3)) % 2 === 0
    case 5:
      return ((x * y) % 2) + ((x * y) % 3) === 0
    case 6:
      return (((x * y) % 2) + ((x * y) % 3)) % 2 === 0
    default:
      return (((x + y) % 2) + ((x * y) % 3)) % 2 === 0
  }
}

function bchFormat(eclMask: number): number {
  let d = eclMask << 10
  const gen = 0b10100110111
  for (let i = 14; i >= 10; i--) {
    if ((d >>> i) & 1) d ^= gen << (i - 10)
  }
  return (eclMask << 10 | d) ^ 0b101010000010010
}

function placeFormat(mod: number[][], n: number, mask: number) {
  const bits = bchFormat((0b00 << 3) | mask)
  const horiz: Array<[number, number]> = [
    [0, 8],
    [1, 8],
    [2, 8],
    [3, 8],
    [4, 8],
    [5, 8],
    [7, 8],
    [8, 8],
    [8, 7],
    [8, 5],
    [8, 4],
    [8, 3],
    [8, 2],
    [8, 1],
    [8, 0],
  ]
  const vert: Array<[number, number]> = [
    [8, n - 1],
    [8, n - 2],
    [8, n - 3],
    [8, n - 4],
    [8, n - 5],
    [8, n - 6],
    [8, n - 7],
    [n - 8, 8],
    [n - 7, 8],
    [n - 6, 8],
    [n - 5, 8],
    [n - 4, 8],
    [n - 3, 8],
    [n - 2, 8],
    [n - 1, 8],
  ]
  for (let i = 0; i < 15; i++) {
    const bit = (bits >> i) & 1
    const [x1, y1] = horiz[i]
    const [x2, y2] = vert[i]
    mod[y1][x1] = bit
    mod[y2][x2] = bit
  }
  mod[n - 8][8] = 1
}

function placeData(mod: number[][], n: number, version: number, code: Uint8Array, mask: number) {
  const bits: number[] = []
  for (const b of code) {
    for (let i = 7; i >= 0; i--) bits.push((b >> i) & 1)
  }
  let bi = 0
  let up = true
  for (let col = n - 1; col > 0; col -= 2) {
    if (col === 6) col--
    for (let i = 0; i < n; i++) {
      const y = up ? n - 1 - i : i
      for (let dx = 0; dx < 2; dx++) {
        const x = col - dx
        if (reserved(x, y, n, version)) continue
        let bit = bi < bits.length ? bits[bi++] : 0
        if (maskBit(mask, x, y)) bit ^= 1
        mod[y][x] = bit
      }
    }
    up = !up
  }
}

function penalty(mod: number[][], n: number): number {
  let s = 0
  for (let y = 0; y < n; y++) {
    let run = 1
    for (let x = 1; x < n; x++) {
      if (mod[y][x] === mod[y][x - 1]) run++
      else {
        if (run >= 5) s += run - 2
        run = 1
      }
    }
    if (run >= 5) s += run - 2
  }
  for (let x = 0; x < n; x++) {
    let run = 1
    for (let y = 1; y < n; y++) {
      if (mod[y][x] === mod[y - 1][x]) run++
      else {
        if (run >= 5) s += run - 2
        run = 1
      }
    }
    if (run >= 5) s += run - 2
  }
  for (let y = 0; y < n - 1; y++) {
    for (let x = 0; x < n - 1; x++) {
      if (mod[y][x] === mod[y][x + 1] && mod[y][x] === mod[y + 1][x] && mod[y][x] === mod[y + 1][x + 1]) s += 3
    }
  }
  let dark = 0
  for (let y = 0; y < n; y++) for (let x = 0; x < n; x++) dark += mod[y][x]
  s += Math.abs(Math.floor((dark * 100) / (n * n) / 5) * 5 - 50) / 5 * 10
  return s
}

function pickVersion(text: string): number {
  const bytes = new TextEncoder().encode(text).length
  for (let v = 1; v <= 8; v++) {
    const dataCw = TOTAL_CW[v] - ECC_PER_BLOCK[v] * ECC_BLOCKS[v]
    const cap = dataCw * 8
    const need = 4 + (v >= 10 ? 16 : 8) + bytes * 8 + 4
    if (need <= cap) return v
  }
  throw new Error('qr too long')
}

function matrix(text: string): number[][] {
  const version = pickVersion(text)
  const n = sizeOf(version)
  const raw = bitsToBytes(dataBits(text, version))
  const code = interleave(version, raw)
  let best: number[][] | null = null
  let bestScore = Infinity
  for (let mask = 0; mask < 8; mask++) {
    const mod = Array.from({ length: n }, () => Array(n).fill(0))
    placeFinders(mod, n)
    placeTiming(mod, n)
    placeAlign(mod, version, n)
    placeData(mod, n, version, code, mask)
    placeFormat(mod, n, mask)
    const sc = penalty(mod, n)
    if (sc < bestScore) {
      bestScore = sc
      best = mod
    }
  }
  return best as number[][]
}

export function qrSvgDataUri(text: string, size = 180): string {
  const mod = matrix(text)
  const n = mod.length
  const quiet = 2
  const dim = n + quiet * 2
  const parts = [`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${dim} ${dim}" width="${size}" height="${size}" shape-rendering="crispEdges">`, `<rect width="${dim}" height="${dim}" fill="#fff"/>`]
  for (let y = 0; y < n; y++) {
    for (let x = 0; x < n; x++) {
      if (mod[y][x]) parts.push(`<rect x="${x + quiet}" y="${y + quiet}" width="1" height="1" fill="#111"/>`)
    }
  }
  parts.push('</svg>')
  return `data:image/svg+xml;charset=utf-8,${encodeURIComponent(parts.join(''))}`
}
