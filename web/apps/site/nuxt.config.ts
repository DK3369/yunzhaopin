export default defineNuxtConfig({
  extends: ['../../layers/base', '../../layers/ui'],
  compatibilityDate: '2026-08-25',
  css: ['~/assets/main.css'],
  modules: ['@nuxtjs/i18n'],
  i18n: {
    locales: [
      { code: 'zh', language: 'zh', file: 'zh.json', name: '中文' },
      { code: 'en', language: 'en', file: 'en.json', name: 'English' },
    ],
    defaultLocale: 'en',
    lazy: true,
    langDir: 'locales',
    strategy: 'no_prefix',
    detectBrowserLanguage: false,
    compilation: {
      strictMessage: false,
    },
  },
  runtimeConfig: {
    public: {
      siteUrl: process.env.NUXT_PUBLIC_SITE_URL || 'http://127.0.0.1:3001',
    },
  },
  features: {
    inlineStyles: true,
  },
  routeRules: {
    '/user/**': { ssr: false },
    '/com/**': { ssr: false },
    '/favicon.ico': { headers: { 'cache-control': 'public, max-age=31536000, immutable' } },
    '/_nuxt/**': { headers: { 'cache-control': 'public, max-age=31536000, immutable' } },
  },
  app: {
    pageTransition: false,
    layoutTransition: false,
    head: {
      meta: [
        {
          name: 'viewport',
          content: 'width=device-width, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no',
        },
      ],
      link: [{ rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }],
    },
  },
  nitro: {
    externals: { external: ['esbuild'] },
    compressPublicAssets: true,
    prerender: { crawlLinks: false, routes: [] },
    // 皮肤/站标在 public/legacy、public/data/logo。用户文件由
    // server/routes/data/upload 运行时读 storage/upload，不在构建时拷贝。
    // 仅 `nuxt dev`：把 /admin 转到本机 admin 进程。现网由 site Nitro :3001 直接出 /admin，不再另开端口。
    devProxy: {
      '/admin': { target: 'http://127.0.0.1:3002', changeOrigin: true },
    },
  },
})
