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
      <form class="com_release_box site-pc" @submit.prevent="save">
        <ul>
          <MemberReleaseRow :label="$t('ui.image')" required><input v-model="form.pic" required class="com_release_textnew_text" /></MemberReleaseRow>
          <MemberReleaseRow :label="$t('ui.link')"><input v-model="form.link" class="com_release_textnew_text" /></MemberReleaseRow>
          <MemberReleaseRow :label="$t('ui.sort')"><input v-model.number="form.sort" type="number" min="0" /></MemberReleaseRow>
        </ul>
        <button type="submit" class="btn_01">{{ editing ? $t('common.save') : $t('ui.add') }}</button>
        <button v-if="editing" type="button" class="btn_01" @click="reset">{{ $t('common.cancel') }}</button>
      </form>
      <div class="site-h5 issue_post_body">
        <form class="yun_createbox" @submit.prevent="save">
          <MemberField wap :label="$t('ui.image')"><input v-model="form.pic" required /></MemberField>
          <MemberField wap :label="$t('ui.link')"><input v-model="form.link" /></MemberField>
          <MemberField wap :label="$t('ui.sort')"><input v-model.number="form.sort" type="number" min="0" /></MemberField>
          <button type="submit" class="issue_post_body_btn">{{ editing ? $t('common.save') : $t('ui.add') }}</button>
          <button v-if="editing" type="button" class="issue_post_body_btn" @click="reset">{{ $t('common.cancel') }}</button>
        </form>
      </div>
      <p v-if="msg">{{ msg }}</p>
      <div v-for="row in list" :key="row.id" class="combanner_box site-pc">
        <img v-if="row.pic" :src="row.pic_n || mediaUrl(row.pic)" alt="" width="300" />
        <div class="combanner_box_tip">{{ row.link }}</div>
        <div class="combanner_box_b">
          <a href="javascript:;" class="combanner_box_bth" @click="edit(row)">{{ $t('common.edit') }}</a>
          <a href="javascript:;" class="combanner_box_bth" @click="remove(row)">{{ $t('common.delete') }}</a>
        </div>
      </div>
      <div class="site-h5">
        <div v-for="row in list" :key="'h5-' + row.id" class="com_cardlist">
          <div class="com_cardlist_tit">
            <img v-if="row.pic" :src="row.pic_n || mediaUrl(row.pic)" alt="" />
          </div>
          <div class="com_cardlist_p">
            <span class="com_cardlist_p_name">{{ $t('ui.link') }}</span>
            {{ row.link }}
          </div>
          <div class="com_card_cz">
            <a href="javascript:;" class="com_bth cblue" @click="edit(row)">{{ $t('common.edit') }}</a>
            <span class="com_card_delete" @click="remove(row)" />
          </div>
        </div>
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
