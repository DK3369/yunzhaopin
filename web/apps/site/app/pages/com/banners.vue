<script setup lang="ts">
import { isUnauthErr, mediaUrl } from '~/utils/site'

type Row = {
  id: number
  pic: string
  pic_n?: string
  link?: string | null
  sort: number
  addtime_n?: string
}

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-banners', () =>
  api.post<Row[]>('/v1/mcenter/company-banners/list', {}),
)

const blank = () => ({ id: 0, pic: '', link: '', sort: 0 })
const form = reactive(blank())
const editing = computed(() => form.id > 0)
const msg = ref('')
const list = computed<Row[]>(() => data.value || [])

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
      await api.post('/v1/mcenter/company-banners/update', { ...form })
    } else {
      await api.post('/v1/mcenter/company-banners', {
        pic: form.pic,
        link: form.link,
        sort: form.sort,
      })
    }
    reset()
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

function edit(row: Row) {
  form.id = row.id
  form.pic = row.pic || ''
  form.link = row.link || ''
  form.sort = Number(row.sort || 0)
}

async function remove(row: Row) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company-banners/delete', { ids: [row.id] })
    if (form.id === row.id) reset()
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

useSeoMeta({ title: t('ui.com_banner') })
</script>

<template>
  <MemberPanel :title="$t('ui.com_banner')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <form class="form verification_form" @submit.prevent="save">
        <MemberField :label="$t('ui.image')"><input v-model="form.pic" required /></MemberField>
        <MemberField :label="$t('ui.link')"><input v-model="form.link" /></MemberField>
        <MemberField :label="$t('ui.sort')"><input v-model.number="form.sort" type="number" min="0" /></MemberField>
        <button type="submit" class="verification_form_btn">{{ editing ? $t('common.save') : $t('ui.add') }}</button>
        <button v-if="editing" type="button" class="verification_form_btn" @click="reset">{{ $t('common.cancel') }}</button>
      </form>
      <p v-if="msg">{{ msg }}</p>
      <div v-for="row in list" :key="row.id" class="user_resume_box">
        <img v-if="row.pic" :src="row.pic_n || mediaUrl(row.pic)" alt="" width="240" />
        <p class="muted">{{ row.link }}</p>
        <a href="javascript:;" class="cblue" @click="edit(row)">{{ $t('common.edit') }}</a>
        <a href="javascript:;" class="List_dete cblue" @click="remove(row)">{{ $t('common.delete') }}</a>
      </div>
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
