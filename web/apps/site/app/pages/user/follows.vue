<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const kind = ref(2)
const { data, error, refresh } = await useAsyncData(
  () => `follows-${kind.value}`,
  () => api.post('/v1/mcenter/follows/list', { kind: kind.value, page: 1, page_size: 20 }),
)
watch(kind, () => refresh())
async function toggle(row: { target_uid?: number; uid?: number; target_kind?: number }) {
  await api.post('/v1/mcenter/follows', {
    target_kind: row.target_kind || kind.value,
    target_uid: row.target_uid || row.uid,
  })
  refresh()
}
useSeoMeta({ title: t('wap_01142') })
</script>

<template>
  <MemberPanel :title="$t('wap_01142')" :error="error" :empty="!error && !(data?.list || []).length">
    <p>
      <button type="button" @click="kind = 2">{{ $t('common.company') }}</button>
      <button type="button" @click="kind = 1">{{ $t('ui.user_kind') }}</button>
    </p>
    <article v-for="row in data?.list || []" :key="row.target_uid || row.uid" class="look_resume_list">
      <p>
        <NuxtLink v-if="kind === 2" :to="`/companies/${row.target_uid || row.uid}`">{{ row.name || row.com_name || row.target_uid }}</NuxtLink>
        <span v-else>{{ row.name || row.com_name || row.target_uid }}</span>
      </p>
      <button type="button" @click="toggle(row)">{{ $t('common.delete') }}</button>
    </article>
  </MemberPanel>
</template>
