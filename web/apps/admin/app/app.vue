<script setup lang="ts">
import zhCn from 'element-plus/es/locale/lang/zh-cn'
import en from 'element-plus/es/locale/lang/en'

const route = useRoute()
const { locale } = useI18n()

async function dropLangQuery() {
  if (route.query.lang == null) return
  const query = { ...route.query }
  delete query.lang
  await navigateTo({ path: route.path, query, hash: route.hash }, { replace: true })
}
await dropLangQuery()
watch(() => route.query.lang, () => {
  dropLangQuery()
})
const epLocale = computed(() => (locale.value === 'zh' ? zhCn : en))
</script>

<template>
  <el-config-provider :locale="epLocale">
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </el-config-provider>
</template>
