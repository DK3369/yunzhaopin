/** EP 3: radio/checkbox `label` as the selected value is deprecated. Not el-form-item / el-table-column. */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const here = dirname(fileURLToPath(import.meta.url))
const phpAdmin = (...p: string[]) => join(here, 'public/php-admin', ...p)

function rewriteChoiceLabelToValue(code: string) {
  return code.replace(
    /<(el-(?:radio|checkbox)(?:-button)?)(\s[\s\S]*?)(\/?)>/g,
    (_full, tag: string, attrs: string, slash: string) => {
      const hasValue = /\s(?:v-bind:|:)?value\s*=/.test(attrs)
      if (hasValue) {
        attrs = attrs
          .replace(/\s(?:v-bind:|:)?label\s*=\s*(?:"[^"]*"|'[^']*'|[^\s/>]+)/g, '')
        return `<${tag}${attrs}${slash}>`
      }
      attrs = attrs.replace(/(?<=\s)v-bind:label(\s*=)/g, 'v-bind:value$1')
      attrs = attrs.replace(/(?<=\s):label(\s*=)/g, ':value$1')
      attrs = attrs.replace(/(?<=\s)label(\s*=)/g, 'value$1')
      return `<${tag}${attrs}${slash}>`
    },
  )
}

/**
 * PHP Element UI 2 used empty `<el-checkbox :label="lc('…')">` as display text.
 * EP 3 only paints the default slot, so those boxes were blank. Fill the slot
 * from value/label. Standalone `v-model` stays boolean (drop value).
 */
function fillEmptyChoiceText(code: string) {
  const inject = (tag: string, attrs: string) => {
    const boundM = attrs.match(/\s(?:v-bind:value|:value)\s*=\s*(?:"([^"]*)"|'([^']*)')/)
    const staticM = boundM ? null : attrs.match(/\svalue\s*=\s*(?:"([^"]*)"|'([^']*)')/)
    const expr = boundM?.[1] ?? boundM?.[2] ?? staticM?.[1] ?? staticM?.[2]
    if (expr == null || expr === '') return `<${tag}${attrs}></${tag}>`
    const hasVModel = /\sv-model(?:[.=:]|\s*=)/.test(attrs)
    const isCheckbox = tag === 'el-checkbox' || tag === 'el-checkbox-button'
    let nextAttrs = attrs
    if (isCheckbox && hasVModel) {
      nextAttrs = attrs.replace(/\s(?:v-bind:value|:value|value)\s*=\s*(?:"[^"]*"|'[^']*')/g, '')
    }
    const inner = boundM ? `{{ ${expr} }}` : expr
    return `<${tag}${nextAttrs}>${inner}</${tag}>`
  }
  let out = code.replace(
    /<(el-(?:radio|checkbox)(?:-button)?)(\s[^>]*?)>\s*<\/\1>/g,
    (_full, tag: string, attrs: string) => inject(tag, attrs),
  )
  out = out.replace(
    /<(el-(?:radio|checkbox)(?:-button)?)(\s[^>]*?)\/>/g,
    (_full, tag: string, attrs: string) => {
      if (!/\s(?:v-bind:value|:value|value)\s*=/.test(attrs)) return `<${tag}${attrs}/>`
      return inject(tag, attrs)
    },
  )
  return out
}

/** EP 3: el-button `type="text"` is deprecated; use `link`. Do not touch `<input type="text">`. */
function rewriteButtonTypeText(code: string) {
  return code.replace(
    /<(el-button)(\s[\s\S]*?)(\/?)>/g,
    (_full, tag: string, attrs: string, slash: string) => {
      attrs = attrs.replace(/(?<=\s)type(\s*=\s*)(["'])text\2/g, 'type$1$2link$2')
      attrs = attrs.replace(/(?<=\s):type(\s*=\s*)(["'])text\2/g, ':type$1$2link$2')
      attrs = attrs.replace(/(?<=\s):type(\s*=\s*)"'text'"/g, ':type$1"\'link\'"')
      attrs = attrs.replace(/(?<=\s):type(\s*=\s*)'"text"'/g, ':type$1\'"link"\'')
      return `<${tag}${attrs}${slash}>`
    },
  )
}

/** Concat php-admin CSS (PostCSS cannot parse the minified Element UI 2 sheet). */
function rewriteCssUrls(css: string, cssHref: string): string {
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

/**
 * PHP 登录后只引 phpyun.css（overall…crm）+ element 图标。
 * wangeditor 不进全站包。admin.css 拼在最后，再用 html:has(.adminDomeAll) 把
 * body 登录蓝限制在登录页。
 *
 * public 里的 element-icons.css 实际是截断的 EU2 表，末尾停在 `.el-dropdown`。
 * 直接拼接会把下一份的 `:root` 吃成 `.el-dropdown :root`，皮肤变量全部失效。
 */
const PHP_ADMIN_CSS: { disk: string; href: string; note: string }[] = [
  { disk: 'js/element-icons.css', href: '/admin/php-admin/js/element-icons.css', note: 'element-icons.css 图标字体' },
  { disk: 'adstyle/allcss/overall.css', href: '/admin/php-admin/adstyle/allcss/overall.css', note: 'overall.css 全局' },
  { disk: 'adstyle/allcss/system.css', href: '/admin/php-admin/adstyle/allcss/system.css', note: 'system.css 系统' },
  { disk: 'adstyle/allcss/yunying.css', href: '/admin/php-admin/adstyle/allcss/yunying.css', note: 'yunying.css 运营' },
  { disk: 'adstyle/allcss/neirong.css', href: '/admin/php-admin/adstyle/allcss/neirong.css', note: 'neirong.css 内容' },
  { disk: 'adstyle/allcss/tool.css', href: '/admin/php-admin/adstyle/allcss/tool.css', note: 'tool.css 工具' },
  { disk: 'adstyle/allcss/huiyuan.css', href: '/admin/php-admin/adstyle/allcss/huiyuan.css', note: 'huiyuan.css 会员' },
  { disk: 'adstyle/allcss/index.css', href: '/admin/php-admin/adstyle/allcss/index.css', note: 'index.css 首页' },
  { disk: 'adstyle/allcss/gongju.css', href: '/admin/php-admin/adstyle/allcss/gongju.css', note: 'gongju.css 工具页' },
  { disk: 'adstyle/allcss/crm.css', href: '/admin/php-admin/adstyle/allcss/crm.css', note: 'crm.css CRM' },
  { disk: 'images/admin.css', href: '/admin/php-admin/images/admin.css', note: 'admin.css 登录' },
]

function takeIconFontCss(css: string): string {
  const cut = css.search(/\.el-pagination\b/)
  return cut > 0 ? css.slice(0, cut) : css
}

/** Close a dangling last selector so the next file's `:root` stays `:root`. */
function closeCssModule(): string {
  return '\nhtml.php-admin-css-sep{}\n'
}

/**
 * Element Plus 默认 `--el-color-primary:#409eff`；贴到 PHP 顶栏主色。
 * admin.css 的 `body #145dff` 只给登录页；登录后 body 回到 --bg-color7。
 * 不覆盖 .subHeader，overall.css 的 --bg-color3 自己画顶栏。
 */
function phpAdminEpPrimaryCss(): string {
  return `html:root {
  --el-color-primary: var(--bg-color3);
  --el-color-primary-rgb: 45, 87, 229;
  --el-color-primary-light-3: #6C89ED;
  --el-color-primary-light-5: #96ABF2;
  --el-color-primary-light-7: #C0CDF7;
  --el-color-primary-light-8: #D5DDFA;
  --el-color-primary-light-9: #EAEEFC;
  --el-color-primary-dark-2: #2446B7;
}
body {
  background: var(--bg-color7);
}
html:has(.adminDomeAll),
html:has(.adminDomeAll) body {
  background: #145dff;
}
`
}

function bundlePhpAdminCss(): string {
  const parts: string[] = [
    '/* php-admin CSS modules — section comments keep overall/system/yunying boundaries; class prefixes unchanged */',
  ]
  for (const file of PHP_ADMIN_CSS) {
    const abs = phpAdmin(file.disk)
    if (!existsSync(abs)) continue
    let css = readFileSync(abs, 'utf8')
    css = css.replace(/@charset\s+[^;]+;/gi, '')
    if (file.disk.endsWith('element-icons.css')) css = takeIconFontCss(css)
    parts.push(`/* ==== ${file.note} ==== */\n${rewriteCssUrls(css, file.href)}${closeCssModule()}`)
  }
  parts.push(`/* ==== element-plus 主色对齐 PHP 皮肤 --bg-color3 ==== */\n${phpAdminEpPrimaryCss()}`)
  return parts.join('\n')
}

/** Compile-time EP 2.10 compat: do not batch-edit dozens of PHP Vue templates. */
function phpAdminEpCompat() {
  return {
    name: 'php-admin-ep-compat',
    enforce: 'pre' as const,
    transform(code: string, id: string) {
      if (!id.includes('admin-php') || !id.includes('.vue')) return
      let out = code
      out = out.replaceAll(':underline="false"', 'underline="never"')
      out = out.replaceAll(":underline='false'", 'underline="never"')
      out = out.replaceAll(/(?<!v-model):current-page=/g, 'v-model:current-page=')
      out = out.replaceAll(/(?<!v-model):page-size=/g, 'v-model:page-size=')
      out = rewriteChoiceLabelToValue(out)
      out = fillEmptyChoiceText(out)
      out = rewriteButtonTypeText(out)
      // Auto-import bypasses vueApp.component('ElSwitch'/'ElTooltip'); rename so php-compat wrap applies.
      out = out.replace(/<(\/?)el-switch\b/gi, '<$1PhpElSwitch')
      out = out.replace(/<(\/?)el-tooltip\b/gi, '<$1PhpElTooltip')
      // Do not eat el-checkbox-group / el-checkbox-button.
      out = out.replace(/<(\/?)el-checkbox(?!-group|-button)\b/gi, '<$1PhpElCheckbox')
      out = out.replace(/^\s*console\.log\(\s*tab\s*,\s*event\s*\)\s*;?\s*$/gm, '')
      if (out === code) return
      return { code: out, map: null }
    },
  }
}

/** New folder each deploy so Cloudflare cannot mix an old entry with deleted chunks. */
const adminAssetTag = (process.env.ADMIN_ASSET_TAG || 'dev').replace(/[^a-zA-Z0-9_-]/g, '') || 'dev'

export default defineNuxtConfig({
  extends: ['../../layers/base', '../../layers/ui'],
  ssr: false,
  compatibilityDate: '2026-08-25',
  vite: {
    define: {
      endLoading: '(globalThis.endLoading||function(){})',
      startLoading: '(globalThis.startLoading||function(){})',
      baseUrl: '(globalThis.baseUrl||"/admin/api/php-admin?")',
      getUrlParams: '(globalThis.getUrlParams)',
      AMap: '(globalThis.AMap)',
      custoapp: '(globalThis.custoapp)',
    },
    plugins: [phpAdminEpCompat()],
    server: {
      allowedHosts: true,
    },
    vue: {
      template: {
        transformAssetUrls: {
          includeAbsolute: false,
        },
      },
    },
  },
  modules: ['@element-plus/nuxt', '@nuxtjs/i18n'],
  runtimeConfig: {
    public: {
      adminAssetTag,
      localeCookieKey: 'admin_lang',
      localeFallback: 'en',
    },
  },
  routeRules: {
    '/favicon.v1.ico': { headers: { 'cache-control': 'public, max-age=31536000, immutable' } },
    '/_nuxt/**': { headers: { 'cache-control': 'public, max-age=60, must-revalidate' } },
    '/_n/**': { headers: { 'cache-control': 'public, max-age=31536000, immutable' } },
    '/**': {
      headers: {
        'cache-control': 'no-store, no-cache, must-revalidate',
        'cdn-cache-control': 'no-store',
        'cloudflare-cdn-cache-control': 'no-store',
      },
    },
  },
  i18n: {
    locales: [
      { code: 'zh', language: 'zh-CN', file: 'zh.json', name: '中文' },
      { code: 'en', language: 'en-US', file: 'en.json', name: 'English' },
    ],
    defaultLocale: 'en',
    lazy: true,
    langDir: 'locales',
    strategy: 'no_prefix',
    // PHP `global.php` never sniffs the browser for admin: it reads `?lang=`,
    // then the `admin_lang` cookie, then falls back to English. Browser
    // detection is off so `php-compat.client.ts` owns that resolution, and the
    // shared front-end `lang` cookie can never leak in here.
    detectBrowserLanguage: false,
    compilation: {
      strictMessage: false,
    },
  },
  app: {
    baseURL: '/admin/',
    buildAssetsDir: `/_n/${adminAssetTag}/`,
    head: {
      meta: [{ name: 'admin-build', content: adminAssetTag }],
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/admin/favicon.v1.ico' },
        {
          rel: 'stylesheet',
          href: `/admin/php-admin/adstyle/admin-bundle.css?b=${adminAssetTag}`,
          tagPriority: 10000,
        },
      ],
      script: [
        {
          innerHTML:
            'globalThis.startLoading=globalThis.startLoading||function(){};globalThis.endLoading=globalThis.endLoading||function(){};globalThis.baseUrl=globalThis.baseUrl||"/admin/api/php-admin?";globalThis.getUrlParams=globalThis.getUrlParams||function(l){l=l||window.location;var qs="",a={},p,n;if(l.search)qs=l.search.slice(1);else if(l.hash&&l.hash.indexOf("?")>=0)qs=l.hash.slice(l.hash.indexOf("?")+1);qs.split("&").forEach(function(s){if(!s)return;p=s.split("=");n=decodeURIComponent(p[0]||"");if(n)a[n]=decodeURIComponent(p[1]||"");});return a;};globalThis.wangEditor=globalThis.wangEditor||{createEditor:function(){return{getHtml:function(){return""},getText:function(){return""},setHtml:function(){},destroy:function(){},on:function(){}}},createToolbar:function(){return{destroy:function(){}}}};',
        },
        { src: '/admin/php-admin/js/jquery.min.js', tagPosition: 'bodyClose' },
        { src: '/admin/php-admin/js/echarts.min.js', tagPosition: 'bodyClose' },
        { src: '/admin/php-admin/js/clipboard.min.js', tagPosition: 'bodyClose' },
      ],
    },
  },
  hooks: {
    'render:html'(html: { head: string[] }) {
      const re = /<link[^>]+admin-bundle\.css[^>]*>/i
      const tag = `<link rel="stylesheet" href="/admin/php-admin/adstyle/admin-bundle.css?b=${adminAssetTag}">`
      html.head = html.head.filter((s) => !re.test(s))
      const lastCss = html.head.reduce((acc, s, i) => {
        if (/rel=["']stylesheet["']/i.test(s) || /\.css["']/i.test(s)) return i
        return acc
      }, -1)
      if (lastCss >= 0) html.head.splice(lastCss + 1, 0, tag)
      else html.head.push(tag)
    },
    'nitro:build:public-assets'(nitro: { options: { output: { publicDir: string } } }) {
      const pub = nitro.options.output.publicDir
      writeFileSync(join(pub, 'admin-asset-tag'), `${adminAssetTag}\n`)
      const cssDir = join(pub, 'php-admin/adstyle')
      mkdirSync(cssDir, { recursive: true })
      writeFileSync(join(cssDir, 'admin-bundle.css'), bundlePhpAdminCss())
    },
  },
})
