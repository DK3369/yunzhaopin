import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const here = dirname(fileURLToPath(import.meta.url))
const repoRoot = resolve(here, '../../..')

export default defineNuxtConfig({
  extends: ['../../layers/base', '../../layers/ui'],
  compatibilityDate: '2026-08-25',
  vite: {
    server: {
      allowedHosts: true,
    },
  },
  css: ['~/assets/main.css'],
  modules: ['@nuxtjs/i18n'],
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
  runtimeConfig: {
    public: {
      siteUrl: process.env.NUXT_PUBLIC_SITE_URL || 'http://127.0.0.1:3001',
    },
  },
  routeRules: {
    '/user/**': { ssr: false },
    '/com/**': { ssr: false },
    '/wap': { redirect: { to: '/', statusCode: 301 } },
    '/wap/**': { redirect: { to: '/', statusCode: 301 } },
    '/favicon.ico': { headers: { 'cache-control': 'public, max-age=31536000, immutable' } },
  },
  app: {
    head: {
      meta: [
        {
          name: 'viewport',
          content: 'width=device-width, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no',
        },
      ],
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' },
        { rel: 'stylesheet', href: '/legacy/pc/style/index.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/style.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/css.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/yun_seach.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/comapply.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/login.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/news.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/job.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/class.public.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/yun_job_fairs.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/part.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/map.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/evaluate.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/pc/style/integral.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/member/user/m_css.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/member/com/m_style.css', media: 'screen and (min-width: 1200px)' },
        { rel: 'stylesheet', href: '/legacy/h5/css/base.css', media: 'screen and (max-width: 1199px)' },
        { rel: 'stylesheet', href: '/legacy/h5/css/yunwap.css', media: 'screen and (max-width: 1199px)' },
        { rel: 'stylesheet', href: '/legacy/h5/css/css.css', media: 'screen and (max-width: 1199px)' },
        { rel: 'stylesheet', href: '/legacy/h5/css/job.css', media: 'screen and (max-width: 1199px)' },
      ],
    },
  },
  nitro: {
    prerender: { crawlLinks: false, routes: [] },
    // 仅 `nuxt dev`：把 /admin 转到本机 admin 进程。现网由 site Nitro :3001 直接出 /admin，不再另开端口。
    devProxy: {
      '/admin': { target: 'http://127.0.0.1:3002', changeOrigin: true },
    },
    publicAssets: [
      {
        baseURL: 'legacy/pc/style',
        dir: resolve(repoRoot, 'uploads/app/template/default/style'),
        maxAge: 60 * 60 * 24 * 7,
      },
      {
        baseURL: 'legacy/pc/images',
        dir: resolve(repoRoot, 'uploads/app/template/default/images'),
        maxAge: 60 * 60 * 24 * 7,
      },
      {
        baseURL: 'legacy/h5/css',
        dir: resolve(repoRoot, 'uploads/app/template/wap/css'),
        maxAge: 60 * 60 * 24 * 7,
      },
      {
        baseURL: 'legacy/h5/images',
        dir: resolve(repoRoot, 'uploads/app/template/wap/images'),
        maxAge: 60 * 60 * 24 * 7,
      },
      {
        baseURL: 'data/logo',
        dir: resolve(repoRoot, 'uploads/data/logo'),
        maxAge: 60 * 60 * 24 * 7,
      },
      {
        baseURL: 'data/upload',
        dir: resolve(repoRoot, 'uploads/data/upload'),
        maxAge: 60 * 60 * 24 * 7,
      },
      {
        baseURL: 'legacy/member/user',
        dir: resolve(repoRoot, 'uploads/app/template/member/user/images'),
        maxAge: 60 * 60 * 24 * 7,
      },
      {
        baseURL: 'legacy/member/com',
        dir: resolve(repoRoot, 'uploads/app/template/member/com/images'),
        maxAge: 60 * 60 * 24 * 7,
      },
    ],
  },
})
