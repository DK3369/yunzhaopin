import { existsSync } from 'node:fs'
import { readFile } from 'node:fs/promises'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { transform } from 'esbuild'

type Kind = 'pc' | 'h5'
type CssFile = { disk: string; href: string; note: string }

const PC_FILES: CssFile[] = [
  { disk: 'uploads/app/template/default/style/index.css', href: '/legacy/pc/style/index.css', note: 'index.css 首页' },
  { disk: 'uploads/app/template/default/style/style.css', href: '/legacy/pc/style/style.css', note: 'style.css 全局' },
  { disk: 'uploads/app/template/default/style/css.css', href: '/legacy/pc/style/css.css', note: 'css.css 公共' },
  { disk: 'uploads/app/template/default/style/yun_seach.css', href: '/legacy/pc/style/yun_seach.css', note: 'yun_seach.css 搜索' },
  { disk: 'uploads/app/template/default/style/comapply.css', href: '/legacy/pc/style/comapply.css', note: 'comapply.css 企业申请' },
  { disk: 'uploads/app/template/default/style/login.css', href: '/legacy/pc/style/login.css', note: 'login.css 登录' },
  { disk: 'uploads/app/template/default/style/news.css', href: '/legacy/pc/style/news.css', note: 'news.css 资讯' },
  { disk: 'uploads/app/template/default/style/job.css', href: '/legacy/pc/style/job.css', note: 'job.css 职位' },
  { disk: 'uploads/app/template/default/style/class.public.css', href: '/legacy/pc/style/class.public.css', note: 'class.public.css 分类' },
  { disk: 'uploads/app/template/default/style/yun_job_fairs.css', href: '/legacy/pc/style/yun_job_fairs.css', note: 'yun_job_fairs.css 招聘会' },
  { disk: 'uploads/app/template/default/style/part.css', href: '/legacy/pc/style/part.css', note: 'part.css 兼职' },
  { disk: 'uploads/app/template/default/style/map.css', href: '/legacy/pc/style/map.css', note: 'map.css 地图' },
  { disk: 'uploads/app/template/default/style/evaluate.css', href: '/legacy/pc/style/evaluate.css', note: 'evaluate.css 评价' },
  { disk: 'uploads/app/template/default/style/integral.css', href: '/legacy/pc/style/integral.css', note: 'integral.css 积分' },
  { disk: 'uploads/app/template/member/user/images/m_css.css', href: '/legacy/member/user/m_css.css', note: 'm_css.css 个人会员' },
  { disk: 'uploads/app/template/member/com/images/m_style.css', href: '/legacy/member/com/m_style.css', note: 'm_style.css 企业会员' },
]

const H5_FILES: CssFile[] = [
  { disk: 'uploads/app/template/wap/css/base.css', href: '/legacy/h5/css/base.css', note: 'base.css 基础' },
  { disk: 'uploads/app/template/wap/css/yunwap.css', href: '/legacy/h5/css/yunwap.css', note: 'yunwap.css 全局' },
  { disk: 'uploads/app/template/wap/css/css.css', href: '/legacy/h5/css/css.css', note: 'css.css 公共' },
  { disk: 'uploads/app/template/wap/css/job.css', href: '/legacy/h5/css/job.css', note: 'job.css 职位' },
  { disk: 'uploads/app/template/wap/css/member/memberwap.css', href: '/legacy/h5/css/member/memberwap.css', note: 'memberwap.css 会员' },
]

const mem = new Map<string, string>()

function repoRoot(): string {
  if (process.env.REPO_ROOT) return process.env.REPO_ROOT
  const fromSrc = resolve(dirname(fileURLToPath(import.meta.url)), '../../../../..')
  if (existsSync(join(fromSrc, 'uploads/app/template/default/style/index.css'))) return fromSrc
  return '/www/wwwroot/zzzz.com'
}

function rewriteUrls(css: string, cssHref: string): string {
  const dir = cssHref.replace(/[^/]+$/, '')
  return css.replace(/url\(\s*(['"]?)([^'")]+)\1\s*\)/gi, (full, quote: string, raw: string) => {
    const src = String(raw || '').trim()
    if (!src || /^(data:|https?:|\/\/|#)/i.test(src) || src.startsWith('/')) return full
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

async function build(kind: Kind): Promise<string> {
  const root = repoRoot()
  const files = kind === 'pc' ? PC_FILES : H5_FILES
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

export async function bundledLegacyCss(kind: Kind, ver: string): Promise<string> {
  const key = `${kind}:${ver || '0'}`
  const hit = mem.get(key)
  if (hit) return hit
  const body = await build(kind)
  mem.set(key, body)
  if (mem.size > 8) {
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
