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
  <MemberPanel :title="$t('wap_com_00404')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form class="com_release_box" @submit.prevent="save">
      <ul>
        <MemberReleaseRow :label="$t('wap_00529')" required><input v-model="form.name" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00102')" area required><textarea v-model="form.content" rows="4" required /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.interview_place')" required><input v-model="form.address" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_01431')" required><input v-model="form.linkman" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.linkphone')" required><input v-model="form.linktel" required class="com_release_textnew_text" /></MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ editing ? $t('common.save') : $t('ui.add') }}</button>
      <button v-if="editing" type="button" class="btn_01" @click="reset">{{ $t('common.cancel') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <table v-if="list.length" class="com_table mt20 site-pc">
      <tr>
        <th>{{ $t('wap_com_00413') }}</th>
        <th>{{ $t('ui.interview_place') }}</th>
        <th>{{ $t('member_user_00048') }}</th>
      </tr>
      <tr v-for="row in list" :key="row.id">
        <td>{{ row.name }}</td>
        <td>{{ row.address }} · {{ row.linkman }}</td>
        <td>
          <a href="javascript:;" class="com_bth cblue" @click="fill(row)">{{ $t('common.edit') }}</a>
          <a href="javascript:;" class="com_bth cblue" @click="remove(row)">{{ $t('common.delete') }}</a>
        </td>
      </tr>
    </table>
    <div class="site-h5">
      <div v-for="row in list" :key="'h5-' + row.id" class="com_cardlist">
        <div class="com_cardlist_tit">{{ row.name }}</div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('ui.interview_place') }}</span>
          {{ row.address }} · {{ row.linkman }}
        </div>
        <div class="com_card_cz">
          <a href="javascript:;" class="com_bth cblue" @click="fill(row)">{{ $t('common.edit') }}</a>
          <span class="com_card_delete" @click="remove(row)" />
        </div>
      </div>
    </div>
  </MemberPanel>
</template>
