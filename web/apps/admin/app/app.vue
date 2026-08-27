<script setup lang="ts">
import zhCn from 'element-plus/es/locale/lang/zh-cn'
import en from 'element-plus/es/locale/lang/en'
import { adminI18n, parseAdminLocale, setAdminLocale } from '~/utils/phpLc'

const route = useRoute()
const { locale, setLocale } = useI18n()

async function applyQueryLang() {
  const mapped = mapPhpLang(String(route.query.lang || ''))
  if (mapped && mapped !== locale.value) {
    await setAdminLocale(mapped)
    await setLocale(mapped)
  }
}
if (import.meta.client && parseAdminLocale(locale.value) !== adminI18n.locale) {
  await setLocale(adminI18n.locale)
}
await applyQueryLang()
watch(() => route.query.lang, () => applyQueryLang())
const epLocale = computed(() => (locale.value === 'en' || adminI18n.locale === 'en' ? en : zhCn))
</script>

<template>
  <el-config-provider :locale="epLocale">
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </el-config-provider>
</template>
