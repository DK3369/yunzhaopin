export default defineNuxtConfig({
  compatibilityDate: '2026-08-25',
  runtimeConfig: {
    rustApi: process.env.RUST_API_URL || 'http://127.0.0.1:3000',
    cookieSecure: process.env.COOKIE_SECURE === '1',
    public: {
      siteName: process.env.NUXT_PUBLIC_SITE_NAME || '招聘',
    },
  },
})
