<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data: completion, error } = await useAsyncData('resume-opt-completion', () =>
  api.post<{ score?: number; missing?: string[] }>('/v1/mcenter/resume/completion', {}).catch(() => null),
)
const { data: bundle } = await useAsyncData('resume-opt-bundle', () =>
  api.post('/v1/mcenter/resume/bundle', {}).catch(() => null),
)

function missingLabel(k: string) {
  const map: Record<string, string> = {
    basic_info: t('wap_00269'),
    photo: t('wap_user_00204'),
    expect: t('wap_00460'),
    education: t('wap_00459'),
    work: t('wap_00457'),
    skill_or_language_or_project: t('wap_00450'),
  }
  return map[k] || k
}
function hrefOf(k: string) {
  const map: Record<string, string> = {
    basic_info: '/user/resume#basic',
    photo: '/user/resume#basic',
    expect: '/user/resume#expect',
    education: '/user/resume#edu',
    work: '/user/resume#work',
    skill_or_language_or_project: '/user/resume#skill',
  }
  return map[k] || '/user/resume'
}

useSeoMeta({ title: t('member_user_00485') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00485')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p>{{ $t('wap_01016') }} {{ completion?.score || 0 }}%</p>
    <ul class="stack">
      <li v-for="k in completion?.missing || []" :key="k">
        <NuxtLink :to="hrefOf(k)">{{ missingLabel(k) }}</NuxtLink>
      </li>
    </ul>
    <p v-if="!(completion?.missing || []).length">{{ $t('common.success') }}</p>
    <p v-if="bundle"><NuxtLink to="/user/resume">{{ $t('wap_user_00204') }}</NuxtLink></p>
  </MemberPanel>
</template>
