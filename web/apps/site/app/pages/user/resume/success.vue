<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const bonus = computed(() => String(settings.value.integral_add_resume || '').trim())
const { data: completion } = await useAsyncData('resume-success-completion', () =>
  api.post<{ score?: number; missing?: string[] }>('/v1/mcenter/resume/completion', {}).catch(() => null),
)
const { data: recJobs } = await useAsyncData('resume-success-jobs', () =>
  api.post('/v1/mcenter/recommend/jobs', { limit: 8 }).catch(() => []),
)
type RecJob = { id: number; name?: string; com_name?: string }
const jobs = computed((): RecJob[] => {
  const raw = recJobs.value
  if (Array.isArray(raw)) return raw as RecJob[]
  if (raw && typeof raw === 'object' && 'list' in raw) return ((raw as { list?: RecJob[] }).list || []) as RecJob[]
  return []
})
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
useSeoMeta({ title: t('wap_01131') })
</script>

<template>
  <MemberPanel :title="$t('wap_01131')">
    <p>{{ $t('wap_01131') }}</p>
    <p v-if="bonus">{{ $t('wap_user_00008') }} +{{ bonus }}</p>
    <ul>
      <li v-for="k in completion?.missing || []" :key="k">
        <NuxtLink :to="hrefOf(k)">{{ missingLabel(k) }}</NuxtLink>
      </li>
    </ul>
    <p><NuxtLink to="/user/resume/optimize">{{ $t('member_user_00485') }}</NuxtLink></p>
    <div v-for="job in jobs" :key="job.id" class="job_search_box site-pc">
      <div class="job_search_box_left">
        <div class="job_search_box_jobmane">
          <NuxtLink :to="`/jobs/${job.id}`">{{ job.name }}</NuxtLink>
        </div>
        <div class="job_search_box_tj">{{ job.com_name }}</div>
      </div>
    </div>
    <div class="site-h5">
      <MemberPostedCard v-for="job in jobs" :key="'h5-' + job.id" :title="job.name || ''" :sub="job.com_name" :to="`/jobs/${job.id}`" />
    </div>
  </MemberPanel>
</template>
