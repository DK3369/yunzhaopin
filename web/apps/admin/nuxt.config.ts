export default defineNuxtConfig({
  extends: ['../../layers/base', '../../layers/ui'],
  ssr: false,
  compatibilityDate: '2026-08-25',
  vite: {
    server: {
      allowedHosts: true,
    },
  },
  modules: ['@pinia/nuxt', '@element-plus/nuxt', '@nuxtjs/i18n'],
  i18n: {
    locales: [
      { code: 'zh', language: 'zh-CN', file: 'zh.json', name: '中文' },
      { code: 'en', language: 'en-US', file: 'en.json', name: 'English' },
    ],
    defaultLocale: 'zh',
    lazy: true,
    langDir: 'locales',
    strategy: 'no_prefix',
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'admin_lang',
      fallbackLocale: 'zh',
      redirectOn: 'no prefix',
    },
    compilation: {
      strictMessage: false,
    },
  },
  app: {
    baseURL: '/admin/',
  },
  elementPlus: { importStyle: 'css' },
})
