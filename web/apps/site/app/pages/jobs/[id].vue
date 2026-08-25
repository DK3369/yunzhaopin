<script setup lang="ts">
const route = useRoute()
const id = Number(route.params.id)
const api = useApi()
const { data, error } = await useAsyncData(`job-${id}`, () => api.get('/v1/wap/jobs/detail', { id }))
const job = computed(() => ((data.value as { job?: Record<string, unknown> } | null)?.job || {}) as Record<string, unknown>)
const dict = computed(() => ((data.value as { dict?: Record<string, unknown> } | null)?.dict || {}) as Record<string, unknown>)
const applyMsg = ref('')
async function apply() {
  applyMsg.value = ''
  try {
    await api.post('/v1/mcenter/apply', { job_id: id })
    applyMsg.value = '已投递'
  } catch (e: unknown) {
    applyMsg.value = e instanceof Error ? e.message : '投递失败'
  }
}
const description = computed(() =>
  stripHtml(job.value.description || job.value.content || job.value.name || job.value.com_name),
)
const datePosted = computed(() => unixToIso(job.value.lastupdate || job.value.sdate))
const employmentType = computed(() => {
  const t = Number(job.value.type)
  if (t === 58) return 'PART_TIME'
  if (t === 59) return 'INTERN'
  if (t === 60) return 'TEMPORARY'
  return 'FULL_TIME'
})
useSeoMeta({
  title: () => String(job.value.name || '职位详情'),
  description: () => description.value,
})
useHead({
  link: [{ rel: 'canonical', href: `/jobs/${id}` }],
  script: job.value.name
    ? [
        {
          type: 'application/ld+json',
          innerHTML: JSON.stringify({
            '@context': 'https://schema.org',
            '@type': 'JobPosting',
            title: job.value.name,
            description: description.value || String(job.value.name),
            datePosted: datePosted.value,
            hiringOrganization: {
              '@type': 'Organization',
              name: job.value.com_name || job.value.name,
            },
            jobLocation: dict.value.city_two
              ? {
                  '@type': 'Place',
                  address: {
                    '@type': 'PostalAddress',
                    addressLocality: dict.value.city_two,
                    addressRegion: dict.value.city_one,
                    addressCountry: 'CN',
                  },
                }
              : undefined,
            employmentType: employmentType.value,
            identifier: String(id),
          }),
        },
      ]
    : [],
})
</script>

<template>
  <article v-if="job.name">
    <h1>{{ job.name }}</h1>
    <p class="muted">{{ job.com_name }} · {{ dict.city_two }}</p>
    <p v-if="dict.jobname">{{ dict.jobname }}</p>
    <div v-html="job.description || job.content" />
    <NuxtLink v-if="job.uid" :to="`/companies/${job.uid}`">查看企业</NuxtLink>
    <button type="button" @click="apply">投递简历</button>
    <p v-if="applyMsg">{{ applyMsg }}</p>
  </article>
  <article v-else>
    <h1>职位不存在</h1>
    <p class="muted">{{ error ? '暂时无法加载职位' : '没有这条职位' }}</p>
  </article>
</template>
