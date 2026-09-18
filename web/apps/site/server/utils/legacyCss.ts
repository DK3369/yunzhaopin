import { existsSync } from 'node:fs'
import { readFile } from 'node:fs/promises'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { transform } from 'esbuild'

export type CssPack = 'pc' | 'h5' | 'pc-user' | 'pc-com' | 'h5-user' | 'h5-com'
export type CssEnd = 'pc' | 'h5'
export type CssMember = 'none' | 'user' | 'com'
type CssFile = { disk: string; href: string; note: string }

const PC_PUBLIC: CssFile[] = [
  { disk: 'web/apps/site/public/legacy/pc/style/index.css', href: '/legacy/pc/style/index.css', note: 'index.css 首页' },
  { disk: 'web/apps/site/public/legacy/pc/style/style.css', href: '/legacy/pc/style/style.css', note: 'style.css 全局' },
  { disk: 'web/apps/site/public/legacy/pc/style/css.css', href: '/legacy/pc/style/css.css', note: 'css.css 公共' },
  { disk: 'web/apps/site/public/legacy/pc/style/yun_seach.css', href: '/legacy/pc/style/yun_seach.css', note: 'yun_seach.css 搜索' },
  { disk: 'web/apps/site/public/legacy/pc/style/comapply.css', href: '/legacy/pc/style/comapply.css', note: 'comapply.css 企业申请' },
  { disk: 'web/apps/site/public/legacy/pc/style/login.css', href: '/legacy/pc/style/login.css', note: 'login.css 登录' },
  { disk: 'web/apps/site/public/legacy/pc/style/news.css', href: '/legacy/pc/style/news.css', note: 'news.css 资讯' },
  { disk: 'web/apps/site/public/legacy/pc/style/job.css', href: '/legacy/pc/style/job.css', note: 'job.css 职位' },
  { disk: 'web/apps/site/public/legacy/pc/style/class.public.css', href: '/legacy/pc/style/class.public.css', note: 'class.public.css 分类' },
  { disk: 'web/apps/site/public/legacy/pc/style/yun_job_fairs.css', href: '/legacy/pc/style/yun_job_fairs.css', note: 'yun_job_fairs.css 招聘会' },
  { disk: 'web/apps/site/public/legacy/pc/style/part.css', href: '/legacy/pc/style/part.css', note: 'part.css 兼职' },
  { disk: 'web/apps/site/public/legacy/pc/style/map.css', href: '/legacy/pc/style/map.css', note: 'map.css 地图' },
  { disk: 'web/apps/site/public/legacy/pc/style/evaluate.css', href: '/legacy/pc/style/evaluate.css', note: 'evaluate.css 评价' },
  { disk: 'web/apps/site/public/legacy/pc/style/integral.css', href: '/legacy/pc/style/integral.css', note: 'integral.css 积分' },
  { disk: 'web/apps/site/public/legacy/pc/style/top.css', href: '/legacy/pc/style/top.css', note: 'top.css 排行榜' },
]

const PC_USER: CssFile[] = [
  { disk: 'web/apps/site/public/legacy/member/user/m_css.css', href: '/legacy/member/user/m_css.css', note: 'm_css.css 个人会员' },
  { disk: 'web/apps/site/public/legacy/member/user/m_resume.css', href: '/legacy/member/user/m_resume.css', note: 'm_resume.css 简历编辑' },
]

const PC_COM: CssFile[] = [
  { disk: 'web/apps/site/public/legacy/member/com/m_style.css', href: '/legacy/member/com/m_style.css', note: 'm_style.css 企业会员' },
  { disk: 'web/apps/site/public/legacy/member/com/two_style.css', href: '/legacy/member/com/two_style.css', note: 'two_style.css 企业中心首页' },
]

const H5_PUBLIC: CssFile[] = [
  { disk: 'web/apps/site/public/legacy/h5/css/base.css', href: '/legacy/h5/css/base.css', note: 'base.css 基础' },
  { disk: 'web/apps/site/public/legacy/h5/css/yunwap.css', href: '/legacy/h5/css/yunwap.css', note: 'yunwap.css 全局' },
  { disk: 'web/apps/site/public/legacy/h5/css/css.css', href: '/legacy/h5/css/css.css', note: 'css.css 公共' },
  { disk: 'web/apps/site/public/legacy/h5/css/job.css', href: '/legacy/h5/css/job.css', note: 'job.css 职位' },
]

const H5_USER: CssFile[] = [
  { disk: 'web/apps/site/public/legacy/h5/css/member/memberwap.css', href: '/legacy/h5/css/member/memberwap.css', note: 'memberwap.css 会员' },
  { disk: 'web/apps/site/public/legacy/h5/css/member/memberuserwap.css', href: '/legacy/h5/css/member/memberuserwap.css', note: 'memberuserwap.css 求职会员' },
  { disk: 'web/apps/site/public/legacy/h5/css/yun_wap_member.css', href: '/legacy/h5/css/yun_wap_member.css', note: 'yun_wap_member.css 会员图标' },
]

const H5_COM: CssFile[] = [
  { disk: 'web/apps/site/public/legacy/h5/css/member/memberwap.css', href: '/legacy/h5/css/member/memberwap.css', note: 'memberwap.css 会员' },
  { disk: 'web/apps/site/public/legacy/h5/css/combase.css', href: '/legacy/h5/css/combase.css', note: 'combase.css 企业会员' },
  { disk: 'web/apps/site/public/legacy/h5/css/yun_wap_member.css', href: '/legacy/h5/css/yun_wap_member.css', note: 'yun_wap_member.css 会员图标' },
  { disk: 'web/apps/site/public/legacy/h5/css/member_style.css', href: '/legacy/h5/css/member_style.css', note: 'member_style.css 会员补充' },
]

const PACKS: Record<CssPack, CssFile[]> = {
  pc: PC_PUBLIC,
  h5: H5_PUBLIC,
  'pc-user': PC_USER,
  'pc-com': PC_COM,
  'h5-user': H5_USER,
  'h5-com': H5_COM,
}

const mem = new Map<string, string>()

function repoRoot(): string {
  if (process.env.REPO_ROOT) return process.env.REPO_ROOT
  const fromSrc = resolve(dirname(fileURLToPath(import.meta.url)), '../../../../..')
  if (existsSync(join(fromSrc, 'web/apps/site/public/legacy/pc/style/index.css'))) return fromSrc
  return '/www/wwwroot/zzzz.com'
}

function rewriteUrls(css: string, cssHref: string): string {
  const dir = cssHref.replace(/[^/]+$/, '')
  return css.replace(/url\(\s*(['"]?)([^'")]+)\1\s*\)/gi, (full, quote: string, raw: string) => {
    let src = String(raw || '').trim()
    if (!src || /^(data:|https?:|\/\/|#)/i.test(src) || src.startsWith('/')) return full
    // PHP 会员 CSS 放在 images/ 目录里，`../images/foo.png` 实际就是同目录 foo.png。
    if (/^\.\.\/images\//i.test(src) && /\/legacy\/member\//.test(cssHref)) {
      src = src.replace(/^\.\.\/images\//i, '')
    }
    try {
      const abs = new URL(src, `https://dummy.local${dir}`).pathname
      return `url(${quote}${abs}${quote})`
    } catch {
      return full
    }
  })
}

async function minifyCss(css: string): Promise<string> {
  try {
    const out = await transform(css, { loader: 'css', minify: true })
    return out.code || css
  } catch {
    return css
  }
}

async function build(kind: CssPack): Promise<string> {
  const root = repoRoot()
  const files = PACKS[kind]
  const parts: string[] = [
    `/* ${kind} CSS modules — section comments keep original file boundaries; class prefixes unchanged */`,
  ]
  for (const file of files) {
    const abs = join(root, file.disk)
    try {
      let css = await readFile(abs, 'utf8')
      css = css.replace(/@charset\s+[^;]+;/gi, '')
      const min = await minifyCss(rewriteUrls(css, file.href))
      parts.push(`/* ==== ${file.note} ==== */\n${min}`)
    } catch {
      /* skip missing file */
    }
  }
  return parts.join('\n\n')
}

export async function bundledLegacyCss(kind: CssPack, ver: string): Promise<string> {
  const key = `${kind}:${ver || '0'}`
  const hit = mem.get(key)
  if (hit) return hit
  const body = await build(kind)
  mem.set(key, body)
  if (mem.size > 48) {
    const first = mem.keys().next().value
    if (first) mem.delete(first)
  }
  return body
}

export function cssMember(raw: unknown): CssMember {
  const s = String(raw || 'none').trim().toLowerCase()
  if (s === 'user' || s === 'com') return s
  return 'none'
}

export function cssSkin(raw: unknown): string {
  const s = String(raw || '').trim()
  return /^[a-zA-Z0-9_]{1,64}$/.test(s) ? s : ''
}

async function loadSkin(name: string): Promise<string> {
  const disk = join(repoRoot(), 'web/apps/site/public/skins', name, 'skin.css')
  try {
    let css = await readFile(disk, 'utf8')
    css = css.replace(/@charset\s+[^;]+;/gi, '')
    return `/* ==== skin ${name} ==== */\n${await minifyCss(rewriteUrls(css, `/skins/${name}/skin.css`))}`
  } catch {
    return ''
  }
}

export async function bundledSiteCss(
  end: CssEnd,
  member: CssMember,
  skin: string,
  ver: string,
): Promise<string> {
  const key = `site:${end}:${member}:${skin || '_'}:${ver || '0'}`
  const hit = mem.get(key)
  if (hit) return hit
  const parts = [await bundledLegacyCss(end, ver)]
  if (member === 'user' || member === 'com') {
    parts.push(await bundledLegacyCss(`${end}-${member}`, ver))
  }
  if (skin) {
    const skinCss = await loadSkin(skin)
    if (skinCss) parts.push(skinCss)
  }
  const body = parts.filter(Boolean).join('\n\n')
  mem.set(key, body)
  if (mem.size > 48) {
    const first = mem.keys().next().value
    if (first) mem.delete(first)
  }
  return body
}

export function cssCacheVer(raw: unknown): string {
  return String(raw || '')
    .replace(/[^0-9a-zA-Z_-]/g, '')
    .slice(0, 32)
}
