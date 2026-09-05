<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type TplRow = {
  id: number
  name?: string
  content?: string
  address?: string
  linkman?: string
  linktel?: string
  intertime?: number | string
}

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('interview-tpls', () =>
  api.post('/v1/mcenter/interview-templates/list', {}),
)
const blank = () => ({
  id: 0,
  name: '',
  content: '',
  address: '',
  linkman: '',
  linktel: '',
  intertime: 0,
})
const form = reactive(blank())
const msg = ref('')
const list = computed(() => (Array.isArray(data.value) ? data.value : []) as TplRow[])
const editing = computed(() => form.id > 0)

function fill(row: TplRow) {
  form.id = row.id
  form.name = String(row.name || '')
  form.content = String(row.content || '')
  form.address = String(row.address || '')
  form.linkman = String(row.linkman || '')
  form.linktel = String(row.linktel || '')
  form.intertime = Number(row.intertime || 0)
}
function reset() {
  Object.assign(form, blank())
}
function fail(e: unknown) {
  return e instanceof Error ? e.message : t('ui.failed')
}
async function save() {
  msg.value = ''
  try {
    if (editing.value) {
      await api.post('/v1/mcenter/interview-templates/update', { ...form })
    } else {
      await api.post('/v1/mcenter/interview-templates', { ...form })
    }
    reset()
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function remove(row: TplRow) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/interview-templates/update', { id: row.id, status: 2 })
    if (form.id === row.id) reset()
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
useSeoMeta({ title: t('wap_com_00404') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_com_00404') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form class="form" @submit.prevent="save">
      <input v-model="form.name" required :placeholder="$t('wap_00529')" />
      <textarea v-model="form.content" rows="4" required :placeholder="$t('wap_user_00102')" />
      <input v-model="form.address" required :placeholder="$t('ui.interview_place')" />
      <input v-model="form.linkman" required :placeholder="$t('wap_01431')" />
      <input v-model="form.linktel" required :placeholder="$t('ui.linkphone')" />
      <div class="row">
        <button type="submit">{{ editing ? $t('common.save') : $t('ui.add') }}</button>
        <button v-if="editing" type="button" @click="reset">{{ $t('common.cancel') }}</button>
      </div>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <p v-if="!list.length" class="muted">{{ $t('ui.no_tpl') }}</p>
    <div class="stack">
      <article v-for="row in list" :key="row.id" class="job-card">
        <h3>{{ row.name }}</h3>
        <p class="muted">{{ row.address }} · {{ row.linkman }} {{ row.linktel }}</p>
        <div class="row">
          <button type="button" @click="fill(row)">{{ $t('common.edit') }}</button>
          <button type="button" @click="remove(row)">{{ $t('common.delete') }}</button>
        </div>
      </article>
    </div>
  </section>
</template>

<style scoped>
.row {
  display: flex;
  gap: 0.5rem;
  align-items: center;
  flex-wrap: wrap;
}
</style>
