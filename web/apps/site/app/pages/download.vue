<script setup lang="ts">
import { mediaUrl } from '~/utils/site'

const { t } = useI18n()
const { settings } = useSiteChrome()
const api = useApi()
const { data: android } = await useAsyncData('app-android', () =>
  api.post<{ download_url?: string; version?: string; changelog?: string }>('/v1/wap/app-version', { platform: 'android' }).catch(() => null),
)
const { data: ios } = await useAsyncData('app-ios', () =>
  api.post<{ download_url?: string; version?: string; changelog?: string }>('/v1/wap/app-version', { platform: 'ios' }).catch(() => null),
)
const inWeixin = computed(() => import.meta.client && /MicroMessenger/i.test(navigator.userAgent))
const isIos = computed(() => import.meta.client && /iphone|ipad|ipod/i.test(navigator.userAgent))
const qr = computed(() => {
  if (!inWeixin.value) return ''
  const key = isIos.value ? 'sy_iosu_qcode' : 'sy_androidu_qcode'
  return mediaUrl(settings.value[key] || settings.value.sy_wap_qcode)
})
useSeoMeta({ title: t('ui.app_download') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.app_download') }}</h1>
    <p v-if="settings.sy_app_winchat">{{ settings.sy_app_winchat }}</p>
    <p v-if="qr">
      <img :src="qr" alt="qr" width="180" height="180" />
    </p>
    <p v-else-if="android?.download_url">
      <a :href="android.download_url">Android {{ android.version }}</a>
    </p>
    <p v-if="!inWeixin && ios?.download_url">
      <a :href="ios.download_url">iOS {{ ios.version }}</a>
    </p>
    <p v-if="!qr && !android?.download_url && !ios?.download_url" class="muted">{{ $t('common_02409') }}</p>
  </section>
</template>
