<script setup lang="ts">
const props = defineProps<{ kind: string; id: number; href?: string }>()
const api = useApi()
const { t } = useI18n()
const qr = ref('')
const copied = ref('')

onMounted(async () => {
  if (!props.id) return
  try {
    const r = await api.post<{ show_url?: string }>('/v1/wap/wechat/qr', {
      kind: props.kind,
      id: props.id,
    })
    qr.value = r.show_url || ''
  } catch {
    qr.value = ''
  }
})

async function copy() {
  copied.value = ''
  const url = props.href || ''
  if (!import.meta.client || !url) return
  try {
    await navigator.clipboard.writeText(url)
    copied.value = t('common.success')
  } catch (e: unknown) {
    copied.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
</script>

<template>
  <div>
    <img v-if="qr" :src="qr" alt="" width="160" height="160" />
    <p v-if="href" class="muted">{{ href }}</p>
    <button v-if="href" type="button" @click="copy">{{ $t('common.share') }}</button>
    <p v-if="copied" class="muted">{{ copied }}</p>
  </div>
</template>
