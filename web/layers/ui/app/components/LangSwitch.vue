<template>
  <span class="lang-switch">
    <a href="javascript:;" :class="{ on: locale === 'zh' }" @click.prevent="go('zh')">{{ $t('ui.lang_zh') }}</a>
    <span>|</span>
    <a href="javascript:;" :class="{ on: locale === 'en' }" @click.prevent="go('en')">{{ $t('ui.lang_en') }}</a>
  </span>
</template>

<script setup lang="ts">
import { persistWebLocale } from '../../../base/app/utils/locale'

const props = withDefaults(defineProps<{ reload?: boolean }>(), { reload: false })
const { locale, setLocale } = useI18n()
async function go(code: 'zh' | 'en') {
  if (locale.value === code) return
  persistWebLocale(code)
  await setLocale(code)
  if (props.reload && import.meta.client) location.reload()
}
</script>
