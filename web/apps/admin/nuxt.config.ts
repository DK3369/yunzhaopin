export default defineNuxtConfig({
  extends: ['../../layers/base'],
  ssr: false,
  compatibilityDate: '2026-08-25',
  modules: ['@pinia/nuxt', '@element-plus/nuxt'],
  app: {
    baseURL: '/admin/',
  },
  elementPlus: { importStyle: 'css' },
})
