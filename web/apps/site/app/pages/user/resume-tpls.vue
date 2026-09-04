<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('resume-tpls', () => api.post('/v1/mcenter/resume-tpls', {}))
const msg = ref('')
const list = computed(() => (Array.isArray(data.value) ? data.value : data.value?.list || []) as Array<{
  id: number
  name?: string
  price?: number
  price_yuan?: number
}>)
async function apply(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-tpls/apply', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function buy(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-tpls/buy', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_00328') })
</script>

<template>
  <MemberPanel :title="$t('wap_00328')" :error="error" :empty="!error && !list.length">
    <article v-for="row in list" :key="row.id" class="look_resume_list">
      <h3>{{ row.name }}</h3>
      <p v-if="row.price_yuan || row.price" class="muted">{{ row.price_yuan || row.price }}</p>
      <button type="button" @click="apply(row.id)">{{ $t('common.confirm') }}</button>
      <button v-if="Number(row.price || row.price_yuan || 0) > 0" type="button" @click="buy(row.id)">{{ $t('common_01946') }}</button>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
