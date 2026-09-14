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
    <MemberHrResumeRows
      :rows="list.map((row) => ({
        key: row.uid,
        name: String(row.display_name || row.uid),
        to: `/resumes/${row.uid}`,
      }))"
    />
  </MemberPanel>
</template>
