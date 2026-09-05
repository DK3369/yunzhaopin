<script setup lang="ts">
import { isUnauthErr } from '../utils/site'

type Row = {
  id: number
  title: string
  body?: string | null
  file?: string | null
  status: number
  status_n: string
  statusbody?: string | null
  ctime_n: string
}

const props = defineProps<{ kind: 'news' | 'product'; title: string }>()

const api = useApi()
const { t } = useI18n()

const PAGE_SIZE = 20
const page = ref(1)
const msg = ref('')

const { data, error, refresh } = await useAsyncData(
  `com-content-${props.kind}`,
  () =>
    api.post<{ list: Row[]; total: number }>('/v1/mcenter/company-contents/list', {
      kind: props.kind,
      page: page.value,
      page_size: PAGE_SIZE,
    }),
  { watch: [page] },
)

const list = computed<Row[]>(() => data.value?.list || [])
const total = computed(() => Number(data.value?.total || 0))

const form = reactive({ id: 0, title: '', body: '', file: '' })
const editing = computed(() => form.id > 0)

function reset() {
  form.id = 0
  form.title = ''
  form.body = ''
  form.file = ''
}

function fail(e: unknown) {
  return e instanceof Error ? e.message : t('ui.failed')
}

async function save() {
  msg.value = ''
  const body = {
    kind: props.kind,
    title: form.title,
    body: form.body,
    file: form.file || undefined,
  }
  try {
    if (editing.value) {
      await api.post('/v1/mcenter/company-contents/update', { ...body, id: form.id })
    } else {
      await api.post('/v1/mcenter/company-contents/create', body)
    }
    reset()
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

async function edit(row: Row) {
  msg.value = ''
  try {
    const detail = await api.post<Row>('/v1/mcenter/company-contents/detail', {
      kind: props.kind,
      id: row.id,
    })
    form.id = detail.id
    form.title = detail.title || ''
    form.body = detail.body || ''
    form.file = detail.file || ''
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

async function remove(row: Row) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company-contents/delete', { kind: props.kind, ids: [row.id] })
    if (form.id === row.id) reset()
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
</script>

<template>
  <section>
    <h1>{{ props.title }}</h1>
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <form class="form" @submit.prevent="save">
        <input v-model="form.title" required :placeholder="$t('wap_user_00103')" />
        <input v-model="form.file" :placeholder="$t('ui.image')" />
        <textarea v-model="form.body" required rows="6" :placeholder="$t('ui.body')" />
        <div class="row">
          <button type="submit">{{ editing ? $t('common.save') : $t('common.publish') }}</button>
          <button v-if="editing" type="button" @click="reset">{{ $t('common.cancel') }}</button>
        </div>
      </form>
      <p v-if="msg">{{ msg }}</p>

      <p v-if="!list.length" class="muted">{{ $t('ui.no_data') }}</p>
      <div class="stack">
        <article v-for="row in list" :key="row.id" class="job-card">
          <h3>{{ row.title }}</h3>
          <p class="muted">
            {{ $t('wap_com_00406') }}: {{ row.status_n }} · {{ row.ctime_n }}
          </p>
          <p v-if="row.statusbody" class="muted">
            {{ $t('common_02158') }}: {{ row.statusbody }}
          </p>
          <div class="row">
            <button type="button" @click="edit(row)">{{ $t('common.edit') }}</button>
            <button type="button" @click="remove(row)">{{ $t('common.delete') }}</button>
          </div>
        </article>
      </div>
      <Pager v-model:page="page" :page-size="PAGE_SIZE" :total="total" />
    </template>
    <p>
      <NuxtLink to="/com">{{ $t('ui.back_com') }}</NuxtLink>
    </p>
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
