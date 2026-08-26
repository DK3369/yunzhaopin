export default defineNuxtConfig({
  extends: ['../../layers/base', '../../layers/ui'],
  ssr: false,
  compatibilityDate: '2026-08-25',
  vite: {
    server: {
      allowedHosts: true,
    },
  },
  modules: ['@pinia/nuxt', '@element-plus/nuxt'],
  app: {
    baseURL: '/admin/',
  },
  elementPlus: { importStyle: 'css' },
})
