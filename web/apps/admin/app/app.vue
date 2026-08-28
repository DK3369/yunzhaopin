<script setup lang="ts">
import zhCn from 'element-plus/es/locale/lang/zh-cn'
import en from 'element-plus/es/locale/lang/en'

const route = useRoute()
const { locale, setLocale } = useI18n()

async function applyQueryLang() {
  const mapped = mapPhpLang(String(route.query.lang || ''))
  if (mapped && mapped !== locale.value) {
    persistWebLocale(mapped)
    await setLocale(mapped)
  }
}
await applyQueryLang()
watch(() => route.query.lang, () => applyQueryLang())
const epLocale = computed(() => (locale.value === 'en' ? en : zhCn))
</script>

<template>
  <el-config-provider :locale="epLocale">
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </el-config-provider>
</template>
