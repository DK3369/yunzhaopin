<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-profile', () =>
  api.post('/v1/mcenter/company/list', {}),
)
const form = reactive({
  name: '',
  shortname: '',
  content: '',
  linkman: '',
  linkphone: '',
  linkmail: '',
  hy: 0,
  pr: 0,
  mun: 0,
  provinceid: 0,
  cityid: 0,
  three_cityid: 0,
  logo: '',
  x: '',
  y: '',
})
watch(
  data,
  (row) => {
    if (!row) return
    form.name = String(row.name || '')
    form.shortname = String(row.shortname || '')
    form.content = String(row.content || '')
    form.linkman = String(row.linkman || '')
    form.linkphone = String(row.linkphone || '')
    form.linkmail = String(row.linkmail || '')
    form.hy = Number(row.hy || 0)
    form.pr = Number(row.pr || 0)
    form.mun = Number(row.mun || 0)
    form.provinceid = Number(row.provinceid || 0)
    form.cityid = Number(row.cityid || 0)
    form.three_cityid = Number(row.three_cityid || 0)
    form.logo = String(row.logo || '')
    form.x = String(row.x || '')
    form.y = String(row.y || '')
  },
  { immediate: true },
)
const { data: dicts } = await usePublicDicts()
const industries = computed(() => dicts.value?.industries ?? [])
const natures = computed(() => dicts.value?.company_natures ?? [])
const sizes = computed(() => dicts.value?.company_sizes ?? [])
const msg = ref('')
async function onLogo(ev: Event) {
  const file = (ev.target as HTMLInputElement).files?.[0]
  if (!file) return
  try {
    const r = await $fetch<{ key: string; url: string }>('/api/upload/company-logo', {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    form.logo = r.key || r.url
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function save() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company', { ...form })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_com_00378') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00378')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form v-else class="com_release_box site-pc" @submit.prevent="save">
      <ul>
        <MemberReleaseRow :label="$t('wap_com_00157')" required>
          <input v-model="form.name" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.shortname')">
          <input v-model="form.shortname" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('common.all')">
          <span class="com_release_selectbox">
            <select v-model.number="form.hy">
              <option :value="0">{{ $t('common.all') }}</option>
              <option v-for="h in industries || []" :key="h.id" :value="h.id">{{ h.name }}</option>
            </select>
          </span>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00159')">
          <span class="com_release_selectbox">
            <select v-model.number="form.pr">
              <option :value="0">{{ $t('wap_com_00159') }}</option>
              <option v-for="n in natures || []" :key="n.id" :value="n.id">{{ n.name }}</option>
            </select>
          </span>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_com_00196')">
          <span class="com_release_selectbox">
            <select v-model.number="form.mun">
              <option :value="0">{{ $t('member_com_00196') }}</option>
              <option v-for="s in sizes || []" :key="s.id" :value="s.id">{{ s.name }}</option>
            </select>
          </span>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00243')">
          <LocationFields
            v-model:province-id="form.provinceid"
            v-model:city-id="form.cityid"
            v-model:district-id="form.three_cityid"
          />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00157')">
          <input type="file" accept="image/jpeg,image/png,image/webp" @change="onLogo" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.desc')" area>
          <textarea v-model="form.content" rows="6" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_01431')" required>
          <input v-model="form.linkman" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.linkphone')" required>
          <input v-model="form.linkphone" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_user_00282')">
          <input v-model="form.linkmail" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00243')">
          <MapPick v-model:x="form.x" v-model:y="form.y" />
        </MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('common.save') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
    <div v-if="!error" class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="save">
        <MemberField wap :label="$t('wap_com_00157')">
          <input v-model="form.name" required />
        </MemberField>
        <MemberField wap :label="$t('ui.shortname')">
          <input v-model="form.shortname" />
        </MemberField>
        <MemberField wap :label="$t('common.all')">
          <select v-model.number="form.hy">
            <option :value="0">{{ $t('common.all') }}</option>
            <option v-for="h in industries || []" :key="'h5hy-' + h.id" :value="h.id">{{ h.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('wap_com_00159')">
          <select v-model.number="form.pr">
            <option :value="0">{{ $t('wap_com_00159') }}</option>
            <option v-for="n in natures || []" :key="'h5pr-' + n.id" :value="n.id">{{ n.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('member_com_00196')">
          <select v-model.number="form.mun">
            <option :value="0">{{ $t('member_com_00196') }}</option>
            <option v-for="s in sizes || []" :key="'h5mun-' + s.id" :value="s.id">{{ s.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('wap_user_00243')">
          <LocationFields
            v-model:province-id="form.provinceid"
            v-model:city-id="form.cityid"
            v-model:district-id="form.three_cityid"
          />
        </MemberField>
        <MemberField wap :label="$t('wap_com_00157')">
          <input type="file" accept="image/jpeg,image/png,image/webp" @change="onLogo" />
        </MemberField>
        <MemberField wap area :label="$t('ui.desc')">
          <textarea v-model="form.content" rows="6" />
        </MemberField>
        <MemberField wap :label="$t('wap_01431')">
          <input v-model="form.linkman" required />
        </MemberField>
        <MemberField wap :label="$t('ui.linkphone')">
          <input v-model="form.linkphone" required />
        </MemberField>
        <MemberField wap :label="$t('member_user_00282')">
          <input v-model="form.linkmail" />
        </MemberField>
        <MemberField wap :label="$t('wap_user_00243')">
          <MapPick v-model:x="form.x" v-model:y="form.y" />
        </MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.save') }}</button>
        <p v-if="msg">{{ msg }}</p>
      </form>
    </div>
  </MemberPanel>
</template>
