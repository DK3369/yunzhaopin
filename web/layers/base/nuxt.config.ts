function skipCloudflareRocketLoader(html: { head: string[]; bodyPrepend: string[]; body: string[]; bodyAppend: string[] }) {
  const patch = (s: string) =>
    s.replace(/<script(?![^>]*\bdata-cfasync=)/gi, '<script data-cfasync="false"')
  html.head = html.head.map(patch)
  html.bodyPrepend = html.bodyPrepend.map(patch)
  html.body = html.body.map(patch)
  html.bodyAppend = html.bodyAppend.map(patch)
}

// 只把 node_modules 合成 vendor。不要把 apps/pages/admin-php 打进同一 chunk：
// 后台 120+ 页 + i18n 会在登录时整包求值，Nuxt 直接进 500。
const clientChunkGroups = {
  codeSplitting: {
    groups: [{ name: 'vendor', test: /[\\/]node_modules[\\/]/ }],
  },
}

export default defineNuxtConfig({
  compatibilityDate: '2026-08-25',
  vite: {
    build: {
      cssCodeSplit: false,
      minify: 'oxc',
      cssMinify: true,
    },
    server: {
      allowedHosts: true,
    },
  },
  runtimeConfig: {
    rustApi: process.env.RUST_API_URL || process.env.NUXT_RUST_API || 'http://127.0.0.1:3003',
    cookieSecure: process.env.COOKIE_SECURE === '1',
    public: {
      siteName: process.env.NUXT_PUBLIC_SITE_NAME || '',
      localeCookieKey: 'lang',
      localeFallback: 'zh',
    },
  },
  hooks: {
    // Cloudflare Rocket Loader rewrites type="module"；nosniff 下会把 CSS 当模块加载失败
    'render:html': skipCloudflareRocketLoader,
    'vite:extendConfig'(config, { isClient }) {
      if (!isClient) return
      const build = config.build ?? {}
      config.build = build
      const prev = build.rolldownOptions ?? {}
      const prevOut = prev.output
      build.rolldownOptions = {
        ...prev,
        output: Array.isArray(prevOut)
          ? prevOut.map((o) => ({ ...o, ...clientChunkGroups }))
          : { ...(prevOut ?? {}), ...clientChunkGroups },
      }
    },
    // cssCodeSplit:false 后入口已含全部 CSS；剥掉异步 chunk 上的 css preload，避免引用已不存在的碎文件。
    'build:manifest'(manifest) {
      for (const item of Object.values(manifest)) {
        if (!item || typeof item !== 'object') continue
        const row = item as { isEntry?: boolean; css?: string[] }
        if (row.isEntry) continue
        row.css = []
      }
    },
  },
})
