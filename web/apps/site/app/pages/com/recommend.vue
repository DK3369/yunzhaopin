<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = { uid: number; display_name: string; sex?: number; education?: number; lastupdate?: number }

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-rec-resumes', () =>
  api.post<Row[]>('/v1/mcenter/recommend/resumes', { limit: 40 }).catch(() => [] as Row[]),
)
const list = computed(() => (Array.isArray(data.value) ? data.value : []) as Row[])
useSeoMeta({ title: t('wap_user_00211') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00211')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !list.length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <article v-for="row in list" :key="row.uid" class="look_resume_list">
      <NuxtLink :to="`/resumes/${row.uid}`">{{ row.display_name || row.uid }}</NuxtLink>
    </article>
  </MemberPanel>
</template>
