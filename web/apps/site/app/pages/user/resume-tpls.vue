<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('resume-tpls', () => api.post('/v1/mcenter/resume-tpls', {}))
async function apply(id: number) {
  await api.post('/v1/mcenter/resume-tpls/apply', { id })
}
useSeoMeta({ title: t('wap_00328') })
</script>

<template>
  <MemberPanel :title="$t('wap_00328')" :error="error" :empty="!error && !(data?.list || data || []).length">
    <article v-for="row in data?.list || data || []" :key="row.id" class="look_resume_list">
      <h3>{{ row.name }}</h3>
      <button type="button" @click="apply(row.id)">{{ $t('common.confirm') }}</button>
    </article>
  </MemberPanel>
</template>
