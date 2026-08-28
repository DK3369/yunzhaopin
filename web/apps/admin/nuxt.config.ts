export default defineNuxtConfig({
  extends: ['../../layers/base', '../../layers/ui'],
  ssr: false,
  compatibilityDate: '2026-08-25',
  vite: {
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
      cookieKey: 'lang',
      fallbackLocale: 'zh',
      alwaysRedirect: false,
      redirectOn: 'no prefix',
    },
    compilation: {
      strictMessage: false,
    },
  },
  app: {
    baseURL: '/admin/',
    head: {
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/admin/favicon.ico' },
        { rel: 'stylesheet', href: '/admin/php-admin/js/element-icons.css' },
        { rel: 'stylesheet', href: '/admin/php-admin/adstyle/phpyun.css' },
        { rel: 'stylesheet', href: '/admin/php-admin/images/admin.css' },
      ],
      script: [
        { src: '/admin/php-admin/js/jquery.min.js' },
        { src: '/admin/php-admin/js/echarts.min.js' },
      ],
    },
  },
  elementPlus: { importStyle: 'css' },
})
