import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const here = dirname(fileURLToPath(import.meta.url))
const repoRoot = resolve(here, '../../..')

export default defineNuxtConfig({
  extends: ['../../layers/base', '../../layers/ui'],
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
  hooks: {
    'vite:extendConfig'(config, { isClient }) {
      if (!isClient) return
      const build = config.build ?? {}
      config.build = build
      const prev = build.rolldownOptions ?? {}
      const prevOut = prev.output
      const extra = {
        codeSplitting: {
          minSize: 20_000,
          groups: [{ name: 'vendor', test: /[\\/]node_modules[\\/]/ }],
        },
      }
      build.rolldownOptions = {
        ...prev,
        output: Array.isArray(prevOut)
          ? prevOut.map((o) => ({ ...o, ...extra }))
          : { ...(prevOut ?? {}), ...extra },
      }
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
    '/favicon.ico': { headers: { 'cache-control': 'public, max-age=31536000, immutable' } },
    '/_nuxt/**': { headers: { 'cache-control': 'public, max-age=31536000, immutable' } },
  },
  app: {
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
    compressPublicAssets: true,
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
