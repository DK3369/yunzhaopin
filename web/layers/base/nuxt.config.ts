function skipCloudflareRocketLoader(html: { head: string[]; bodyPrepend: string[]; body: string[]; bodyAppend: string[] }) {
  const patch = (s: string) =>
    s.replace(/<script(?![^>]*\bdata-cfasync=)/gi, '<script data-cfasync="false"')
  html.head = html.head.map(patch)
  html.bodyPrepend = html.bodyPrepend.map(patch)
  html.body = html.body.map(patch)
  html.bodyAppend = html.bodyAppend.map(patch)
}

export default defineNuxtConfig({
  compatibilityDate: '2026-08-25',
  vite: {
    server: {
      allowedHosts: true,
    },
  },
  runtimeConfig: {
    rustApi: process.env.RUST_API_URL || process.env.NUXT_RUST_API || 'http://127.0.0.1:3003',
    cookieSecure: process.env.COOKIE_SECURE === '1',
    public: {
      siteName: process.env.NUXT_PUBLIC_SITE_NAME || '招聘',
    },
  },
  hooks: {
    // Cloudflare Rocket Loader rewrites type="module"；nosniff 下会把 CSS 当模块加载失败
    'render:html': skipCloudflareRocketLoader,
  },
})
