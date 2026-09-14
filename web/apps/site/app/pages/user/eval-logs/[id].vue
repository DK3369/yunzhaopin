<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const route = useRoute()
const id = computed(() => Number(route.params.id))
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData(
  () => `eval-log-${id.value}`,
  () =>
    api.post<{
      id: number
      paper_id?: number
      paper_name?: string
      score?: number
      comment?: string
      created_at_n?: string
      answers?: Record<string, string> | unknown[]
    }>('/v1/mcenter/eval-logs/detail', { id: id.value }),
)
const answers = computed(() => {
  const raw = data.value?.answers
  if (!raw || Array.isArray(raw)) return [] as Array<{ k: string; v: string }>
  return Object.entries(raw as Record<string, unknown>).map(([k, v]) => ({ k, v: String(v) }))
})
useSeoMeta({ title: t('wap_00194') })
</script>

<template>
  <MemberPanel :title="data?.paper_name || $t('wap_00194')" :error="error && !isUnauthErr(error) ? error : undefined">
    <div class="resume_Prompt_box">
      <div class="resume_Prompt">
        <i class="resume_Prompt_icon" />
        <NuxtLink to="/user/eval-logs" class="cblue">{{ $t('wap_00194') }}</NuxtLink>
      </div>
    </div>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <template v-else-if="data">
      <div class="wxts_box">
        <div class="wxts">{{ data.paper_name || $t('wap_00194') }} · {{ data.score }}</div>
        {{ data.created_at_n }}
        <p v-if="data.comment">{{ data.comment }}</p>
      </div>
      <div v-if="data.paper_id" class="site-pc">
        <NuxtLink :to="`/eval/${data.paper_id}`" class="uesr_submit">{{ $t('wap_00194') }}</NuxtLink>
      </div>
      <div v-for="row in answers" :key="row.k" class="yun_send_resume_list site-pc">
        <div class="yun_send_resume_list_name">{{ row.k }}</div>
        <div class="yun_send_resume_list_right">{{ row.v }}</div>
      </div>
      <div class="site-h5 mag_show">
        <div v-for="row in answers" :key="'h5-' + row.k" class="com_member_hr_p1">
          <span class="member_c9">{{ row.k }}</span>{{ row.v }}
        </div>
      </div>
    </template>
  </MemberPanel>
</template>
