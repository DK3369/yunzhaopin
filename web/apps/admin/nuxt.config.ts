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
  modules: ['@pinia/nuxt', '@element-plus/nuxt', '@nuxtjs/i18n'],
  runtimeConfig: {
    public: {
      adminAssetTag,
    },
  },
  routeRules: {
    '/favicon.v1.ico': { headers: { 'cache-control': 'public, max-age=31536000, immutable' } },
    '/_nuxt/**': { headers: { 'cache-control': 'public, max-age=60, must-revalidate' } },
    '/_n/**': { headers: { 'cache-control': 'public, max-age=60, must-revalidate' } },
    '/**': { headers: { 'cache-control': 'no-store' } },
  },
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
      cookieCrossOrigin: false,
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
    buildAssetsDir: `/_n/${adminAssetTag}/`,
    head: {
      meta: [{ name: 'admin-build', content: adminAssetTag }],
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/admin/favicon.v1.ico' },
        { rel: 'stylesheet', href: '/admin/php-admin/js/element-icons.css' },
        { rel: 'stylesheet', href: '/admin/php-admin/adstyle/phpyun.css' },
        { rel: 'stylesheet', href: '/admin/php-admin/images/admin.css' },
        { rel: 'stylesheet', href: '/admin/php-admin/js/wangeditor/index.css' },
      ],
      script: [
        {
          innerHTML:
            'globalThis.startLoading=globalThis.startLoading||function(){};globalThis.endLoading=globalThis.endLoading||function(){};globalThis.baseUrl=globalThis.baseUrl||"/admin/api/php-admin?";globalThis.wangEditor=globalThis.wangEditor||{createEditor:function(){return{getHtml:function(){return""},getText:function(){return""},setHtml:function(){},destroy:function(){},on:function(){}}},createToolbar:function(){return{destroy:function(){}}}};',
        },
        { src: '/admin/php-admin/js/jquery.min.js' },
        { src: '/admin/php-admin/js/echarts.min.js' },
        {
          src: '/admin/php-admin/js/wangeditor/index.js',
          tagPosition: 'head',
          defer: false,
          async: false,
        },
      ],
    },
  },
  elementPlus: { importStyle: 'css' },
})
