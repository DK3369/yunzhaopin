export default defineNitroPlugin(() => {
  const rustApi = useRuntimeConfig().rustApi
  console.log(`[admin] rustApi=${rustApi}`)
})
