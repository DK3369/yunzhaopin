<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `com-gallery-${page.value}`,
  () => api.post('/v1/mcenter/galleries/list', { kind: 'company', page: page.value, page_size: pageSize }),
)
const title = ref('')
const msg = ref('')
async function onFile(ev: Event) {
  const file = (ev.target as HTMLInputElement).files?.[0]
  if (!file) return
  msg.value = ''
  try {
    const r = await $fetch<{ key: string; url: string }>('/api/upload/attachment', {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    await api.post('/v1/mcenter/galleries/create', {
      kind: 'company',
      title: title.value,
      picurl: r.key || r.url,
    })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/galleries/delete', { kind: 'company', ids: [id] })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00157') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00157')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form class="com_release_box site-pc" @submit.prevent>
      <ul>
        <MemberReleaseRow :label="$t('wap_user_00103')"><input v-model="title" class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.image')"><input type="file" accept="image/jpeg,image/png,image/webp" @change="onFile" /></MemberReleaseRow>
      </ul>
    </form>
    <div class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent>
        <MemberField wap :label="$t('wap_user_00103')"><input v-model="title" /></MemberField>
        <MemberField wap :label="$t('ui.image')"><input type="file" accept="image/jpeg,image/png,image/webp" @change="onFile" /></MemberField>
      </form>
    </div>
    <div class="com_banner_show_box site-pc">
      <div v-for="row in data?.list || []" :key="row.id" class="com_add_show_box">
        <p>{{ row.title || row.id }}</p>
        <img v-if="row.picurl" :src="row.picurl" alt="" width="120" />
        <a href="javascript:;" class="List_dete cblue" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 issue_post_body">
      <div class="issue_post_body_card">
        <div class="comshowtip">{{ $t('wap_01201') }}</div>
        <div class="company_photo_box">
          <ul>
            <li v-for="row in data?.list || []" :key="'h5-' + row.id">
              <img v-if="row.picurl" :src="row.picurl" alt="" />
              <span class="com_card_delete" @click="remove(row.id)" />
            </li>
          </ul>
        </div>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
    <MemberPager :page="page" :page-size="pageSize" :total="inferTotal(data)" @update:page="go" />
  </MemberPanel>
</template>
