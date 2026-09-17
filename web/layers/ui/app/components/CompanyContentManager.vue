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
  <MemberPanel :title="props.title" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <form class="com_release_box site-pc" @submit.prevent="save">
        <ul>
          <MemberReleaseRow :label="$t('wap_user_00103')" required><input v-model="form.title" required class="com_release_textnew_text" /></MemberReleaseRow>
          <MemberReleaseRow :label="$t('ui.image')"><input v-model="form.file" class="com_release_textnew_text" /></MemberReleaseRow>
          <MemberReleaseRow :label="$t('ui.body')" area>
            <RichEditor v-model="form.body" />
          </MemberReleaseRow>
        </ul>
        <button type="submit" class="btn_01">{{ editing ? $t('common.save') : $t('common.publish') }}</button>
        <button v-if="editing" type="button" class="btn_01" @click="reset">{{ $t('common.cancel') }}</button>
      </form>
      <div class="site-h5 issue_post_body">
        <form class="yun_createbox" @submit.prevent="save">
          <MemberField wap :label="$t('wap_user_00103')">
            <input v-model="form.title" required />
          </MemberField>
          <MemberField wap :label="$t('ui.image')">
            <input v-model="form.file" />
          </MemberField>
          <MemberField wap area :label="$t('ui.body')">
            <RichEditor v-model="form.body" />
          </MemberField>
          <button type="submit" class="issue_post_body_btn">{{ editing ? $t('common.save') : $t('common.publish') }}</button>
          <button v-if="editing" type="button" class="issue_post_body_btn" @click="reset">{{ $t('common.cancel') }}</button>
        </form>
      </div>
      <p v-if="msg">{{ msg }}</p>
      <div v-for="row in list" :key="row.id" class="sysynews_list site-pc">
        <div class="sysynews_span sysynews_name">{{ row.title }}</div>
        <div class="sysynews_span sysynews_time">{{ row.status_n }} · {{ row.ctime_n }}</div>
        <div class="sysynews_span sysynews_cz">
          <a href="javascript:;" class="cblue" @click="edit(row)">{{ $t('common.edit') }}</a>
          <a href="javascript:;" class="List_dete cblue" @click="remove(row)">{{ $t('common.delete') }}</a>
        </div>
      </div>
      <div class="site-h5 m_cardbox">
        <MemberSxNewsCard
          v-for="row in list"
          :key="'h5-' + row.id"
          :title="row.title"
          :time="`${row.status_n} · ${row.ctime_n}`"
          :on-delete="() => remove(row)"
        >
          <a href="javascript:;" class="sx_new_edit" @click.prevent="edit(row)">{{ $t('common.edit') }}</a>
        </MemberSxNewsCard>
      </div>
      <MemberPager :page="page" :page-size="PAGE_SIZE" :total="total" @update:page="(p) => (page = p)" />
    </template>
    <p>
      <NuxtLink to="/com">{{ $t('ui.back_com') }}</NuxtLink>
    </p>
  </MemberPanel>
</template>

<style scoped>
.row {
  display: flex;
  gap: 0.5rem;
  align-items: center;
  flex-wrap: wrap;
}
</style>
