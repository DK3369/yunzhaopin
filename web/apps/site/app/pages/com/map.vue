<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-map', () =>
  api.post<{ x?: string; y?: string; address?: string }>('/v1/mcenter/company/list', {}),
)
const form = reactive({
  x: '',
  y: '',
})
watch(
  data,
  (row) => {
    if (!row) return
    form.x = String(row.x || '')
    form.y = String(row.y || '')
  },
  { immediate: true },
)
const msg = ref('')
async function save() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/map', { x: form.x, y: form.y })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.map_addr') })
</script>

<template>
  <MemberPanel :title="$t('ui.map_addr')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form v-else class="com_release_box site-pc" @submit.prevent="save">
      <ul>
        <MemberReleaseRow :label="$t('wap_user_00243')" area>
          <MapPick v-model:x="form.x" v-model:y="form.y" :preset="String(data?.address || '')" />
        </MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('common.save') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
    <div v-if="!error" class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="save">
        <MemberField wap :label="$t('wap_user_00243')">
          <MapPick v-model:x="form.x" v-model:y="form.y" :preset="String(data?.address || '')" />
        </MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.save') }}</button>
        <p v-if="msg">{{ msg }}</p>
      </form>
    </div>
  </MemberPanel>
</template>
