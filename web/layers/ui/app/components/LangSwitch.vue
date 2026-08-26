<template>
  <span class="lang-switch">
    <a href="javascript:;" :class="{ on: locale === 'zh' }" @click.prevent="go('zh')">中文</a>
    <span>|</span>
    <a href="javascript:;" :class="{ on: locale === 'en' }" @click.prevent="go('en')">English</a>
  </span>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{ cookieKey?: string }>(), { cookieKey: 'lang' })
const { locale, setLocale } = useI18n()
async function go(code: 'zh' | 'en') {
  await setLocale(code)
  if (!import.meta.client) return
  document.cookie = `${props.cookieKey}=${code}; path=/; max-age=31536000`
}
</script>
