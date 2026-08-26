<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('follows', () =>
  api.post('/v1/mcenter/follows/list', { page: 1, page_size: 20 }),
)
async function toggle(row: { target_uid?: number; uid?: number; target_kind?: number }) {
  await api.post('/v1/mcenter/follows', {
    target_kind: row.target_kind || 2,
    target_uid: row.target_uid || row.uid,
  })
  refresh()
}
useSeoMeta({ title: t('wap_00385') })
</script>

<template>
  <MemberPanel :title="$t('wap_00385')" :error="error" :empty="!error && !(data?.list || []).length">
    <article v-for="row in data?.list || []" :key="row.target_uid || row.uid" class="look_resume_list">
      <p>{{ row.name || row.com_name || row.target_uid }}</p>
      <button type="button" @click="toggle(row)">{{ $t('common.delete') }}</button>
    </article>
  </MemberPanel>
</template>
