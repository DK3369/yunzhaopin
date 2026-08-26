export default defineNuxtConfig({
  extends: ['../../layers/base', '../../layers/ui'],
  compatibilityDate: '2026-08-25',
  vite: {
    server: {
      allowedHosts: true,
    },
  },
  css: ['~/assets/main.css'],
  modules: ['@pinia/nuxt', '@nuxtjs/i18n', '@nuxtjs/sitemap'],
  i18n: {
    locales: [
      { code: 'zh', language: 'zh-CN', file: 'zh.json', name: '中文' },
      { code: 'en', language: 'en-US', file: 'en.json', name: 'English' },
    ],
    defaultLocale: 'zh',
    lazy: true,
    langDir: 'locales',
    strategy: 'no_prefix',
  },
  runtimeConfig: {
    public: {
      siteUrl: process.env.NUXT_PUBLIC_SITE_URL || 'http://127.0.0.1:3001',
    },
  },
  site: {
    url: process.env.NUXT_PUBLIC_SITE_URL || 'http://127.0.0.1:3001',
  },
  sitemap: {
    sources: ['/api/__sitemap__/urls'],
  },
  routeRules: {
    '/user/**': { ssr: false },
    '/com/**': { ssr: false },
  },
  nitro: {
    prerender: { crawlLinks: false, routes: [] },
  },
})
