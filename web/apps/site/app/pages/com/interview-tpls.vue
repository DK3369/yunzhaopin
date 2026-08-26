<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('interview-tpls', () =>
  api.post('/v1/mcenter/interview-templates/list', {}),
)
const form = reactive({
  name: '',
  content: '',
  address: '',
  linkman: '',
  linktel: '',
  intertime: 0,
})
const msg = ref('')
const list = computed(() => (Array.isArray(data.value) ? data.value : []) as Array<Record<string, unknown>>)
async function create() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/interview-templates', { ...form })
    msg.value = t('ui.added')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.interview_tpl') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.interview_tpl') }}</h1>
    <p v-if="error" class="muted">{{ $t('ui.please_login_com') }}</p>
    <form class="form" @submit.prevent="create">
      <input v-model="form.name" :placeholder="$t('ui.name')" />
      <textarea v-model="form.content" rows="4" :placeholder="$t('ui.content')" />
      <input v-model="form.address" :placeholder="$t('ui.interview_place')" />
      <input v-model="form.linkman" :placeholder="$t('ui.linkman')" />
      <input v-model="form.linktel" :placeholder="$t('ui.linkphone')" />
      <button type="submit">{{ $t('ui.add') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <p v-if="!list.length" class="muted">{{ $t('ui.no_tpl') }}</p>
    <div class="stack">
      <article v-for="row in list" :key="String(row.id)" class="job-card">
        <h3>{{ row.name }}</h3>
        <p class="muted">{{ row.address }} · {{ row.linkman }} {{ row.linktel }}</p>
      </article>
    </div>
  </section>
</template>
