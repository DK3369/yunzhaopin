<script setup lang="ts">
import { isUnauthErr, mediaUrl } from '~/utils/site'

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
  linkjob: '',
  linkphone: '',
  linktel: '',
  linkmail: '',
  address: '',
  website: '',
  busstops: '',
  linkqq: '',
  sdate: '',
  money: 0,
  moneytype: 1,
  infostatus: 1,
  welfare: '',
  not_disturb: '0',
  hy: 0,
  pr: 0,
  mun: 0,
  provinceid: 0,
  cityid: 0,
  three_cityid: 0,
  logo: '',
  comqcode: '',
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
    form.linkjob = String(row.linkjob || '')
    form.linkphone = String(row.linkphone || '')
    form.linktel = String(row.linktel || '')
    form.linkmail = String(row.linkmail || '')
    form.address = String(row.address || '')
    form.website = String(row.website || '')
    form.busstops = String(row.busstops || '')
    form.linkqq = String(row.linkqq || '')
    form.sdate = String(row.sdate || '')
    form.money = Number(row.money || 0)
    form.moneytype = Number(row.moneytype || 1)
    form.infostatus = Number(row.infostatus || 1)
    form.welfare = String(row.welfare || '')
    form.not_disturb = String(row.not_disturb || '0')
    form.hy = Number(row.hy || 0)
    form.pr = Number(row.pr || 0)
    form.mun = Number(row.mun || 0)
    form.provinceid = Number(row.provinceid || 0)
    form.cityid = Number(row.cityid || 0)
    form.three_cityid = Number(row.three_cityid || 0)
    form.logo = String(row.logo || '')
    form.comqcode = String(row.comqcode || '')
    form.x = String(row.x || '')
    form.y = String(row.y || '')
  },
  { immediate: true },
)
const nameLocked = computed(() => Number(data.value?.yyzz_status) === 1)
const telLocked = computed(() => Number(data.value?.moblie_status) === 1)
const mailLocked = computed(() => Number(data.value?.email_status) === 1)
const { data: dicts } = await usePublicDicts()
const { settings } = useSiteChrome()
const industries = computed(() => dicts.value?.industries ?? [])
const natures = computed(() => dicts.value?.company_natures ?? [])
const sizes = computed(() => dicts.value?.company_sizes ?? [])
const welfares = computed(() => dicts.value?.welfares ?? [])
const welNames = ref<string[]>([])
const extraWel = ref('')
const disturbOn = ref(false)
const disturbStart = ref('22:00')
const disturbEnd = ref('08:00')
const msg = ref('')

function parseDisturb(raw: string) {
  const s = String(raw || '').trim()
  if (!s || s === '0') return { on: false, start: '22:00', end: '08:00' }
  const parts = s.split('-').map((x) => x.trim())
  if (parts.length === 2 && parts[0].includes(':')) {
    return { on: true, start: parts[0].slice(0, 5), end: parts[1].slice(0, 5) }
  }
  if (s === '1') return { on: true, start: '22:00', end: '08:00' }
  return { on: false, start: '22:00', end: '08:00' }
}
function applyWels(raw: string) {
  welNames.value = String(raw || '')
    .split(/[,，]/)
    .map((s) => s.trim())
    .filter(Boolean)
}
function addWelfare() {
  const s = extraWel.value.trim()
  if (s.length < 2 || s.length > 8) return
  if (!welNames.value.includes(s)) welNames.value = [...welNames.value, s]
  extraWel.value = ''
}
watch(
  data,
  (row) => {
    if (!row) return
    applyWels(String(row.welfare || ''))
    const d = parseDisturb(String(row.not_disturb || '0'))
    disturbOn.value = d.on
    disturbStart.value = d.start
    disturbEnd.value = d.end
  },
  { immediate: true },
)
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
async function onQcode(ev: Event) {
  const file = (ev.target as HTMLInputElement).files?.[0]
  if (!file) return
  try {
    const r = await $fetch<{ key: string; url: string }>('/api/upload/company-logo', {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    form.comqcode = r.key || r.url
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
const checkHint = ref('')
async function checkField(typeStr: 'name' | 'linktel', value: string) {
  const s = value.trim()
  if (!s) return true
  try {
    const r = await api.post<{ used?: boolean }>('/v1/mcenter/company/check', {
      type_str: typeStr,
      check_str: s,
    })
    if (r.used) {
      checkHint.value = typeStr === 'name' ? t('common_01222') : t('wap_js_00049')
      return false
    }
    if (checkHint.value) checkHint.value = ''
    return true
  } catch (e: unknown) {
    checkHint.value = e instanceof Error ? e.message : t('ui.failed')
    return false
  }
}
const guide = ref<{ msg: string; to: string; act: string } | null>(null)
function settingOn(key: string) {
  return String(settings.value[key] || '') === '1'
}
function afterSave() {
  const row = data.value as Record<string, unknown> | null
  const yyzz = Number(row?.yyzz_status) === 1
  const mob = Number(row?.moblie_status) === 1
  const email = Number(row?.email_status) === 1
  const mapped = String(form.x || '').trim() !== '' && String(form.y || '').trim() !== ''
  if (settingOn('com_enforce_licensecert') && !yyzz) {
    guide.value = { msg: t('member_com_00162'), to: '/com/cert', act: t('member_com_00191') }
    return
  }
  if (settingOn('com_enforce_mobilecert') && !mob) {
    guide.value = { msg: t('member_com_00161'), to: '/com/binding', act: t('member_com_00190') }
    return
  }
  if (settingOn('com_enforce_emailcert') && !email) {
    guide.value = { msg: t('member_com_00164'), to: '/com/binding', act: t('member_com_00192') }
    return
  }
  if (settingOn('com_enforce_setposition') && !mapped) {
    guide.value = { msg: t('member_com_00160'), to: '/com/map', act: t('member_com_00204') }
    return
  }
  if (!yyzz) {
    guide.value = { msg: t('member_com_00187'), to: '/com/cert', act: t('member_com_00191') }
    return
  }
  if (!mob) {
    guide.value = { msg: t('member_com_00168'), to: '/com/binding', act: t('member_com_00190') }
    return
  }
  if (!email) {
    guide.value = { msg: t('member_com_00170'), to: '/com/binding', act: t('member_com_00192') }
    return
  }
  guide.value = null
}
async function save() {
  msg.value = ''
  guide.value = null
  try {
    if (!(await checkField('name', form.name))) return
    if (!(await checkField('linktel', form.linktel))) return
    form.welfare = welNames.value.join(',')
    form.not_disturb = disturbOn.value ? `${disturbStart.value}-${disturbEnd.value}` : '0'
    await api.post('/v1/mcenter/company', { ...form })
    msg.value = t('common.success')
    await refresh()
    afterSave()
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
          <input v-model="form.name" class="com_release_textnew_text" :disabled="nameLocked" @blur="checkField('name', form.name)" />
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
          <img v-if="form.logo" :src="mediaUrl(form.logo)" width="40" height="40" alt="" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_com_00197')">
          <input type="file" accept="image/jpeg,image/png,image/webp" @change="onQcode" />
          <img v-if="form.comqcode" :src="mediaUrl(form.comqcode)" width="40" height="40" alt="" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.desc')" area>
          <textarea v-model="form.content" rows="6" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_01431')" required>
          <input v-model="form.linkman" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00288')">
          <input v-model="form.linkjob" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00142')" required>
          <input v-model="form.linktel" class="com_release_textnew_text" :disabled="telLocked" @blur="checkField('linktel', form.linktel)" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.linkphone')">
          <input v-model="form.linkphone" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_user_00282')">
          <input v-model="form.linkmail" class="com_release_textnew_text" :disabled="mailLocked" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.map_addr')">
          <input v-model="form.address" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00160')">
          <input v-model="form.website" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00162')">
          <input v-model="form.busstops" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00091')">
          <input v-model="form.linkqq" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_user_00106')">
          <input v-model="form.sdate" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00158')">
          <input v-model.number="form.money" type="number" min="0" class="com_release_textnew_text" />
          <select v-model.number="form.moneytype">
            <option :value="1">{{ $t('common_02067') }}</option>
            <option :value="2">USD</option>
          </select>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_js_00005')">
          <label><input v-model.number="form.infostatus" type="radio" :value="1" /> {{ $t('wap_js_00005') }}</label>
          <label><input v-model.number="form.infostatus" type="radio" :value="2" /> {{ $t('wap_js_00003') }}</label>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('common_02017')">
          <label v-for="w in welfares" :key="w.id">
            <input v-model="welNames" type="checkbox" :value="w.name" /> {{ w.name }}
          </label>
          <label v-for="n in welNames.filter((x) => !welfares.some((w) => w.name === x))" :key="'ex-' + n">
            <input v-model="welNames" type="checkbox" :value="n" /> {{ n }}
          </label>
          <input v-model="extraWel" maxlength="8" class="com_release_textnew_text" style="width: 8em" />
          <a href="javascript:;" class="cblue" @click.prevent="addWelfare">{{ $t('member_com_00202') }}</a>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_com_00695')">
          <label><input v-model="disturbOn" type="checkbox" /> {{ $t('member_com_00695') }}</label>
        </MemberReleaseRow>
        <MemberReleaseRow v-if="disturbOn" :label="$t('wap_com_00150')">
          <input v-model="disturbStart" type="time" />
          {{ $t('common_02098') }}
          <input v-model="disturbEnd" type="time" />
          {{ $t('wap_com_00151') }}
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00243')">
          <MapPick v-model:x="form.x" v-model:y="form.y" :preset="form.address" />
          <p><NuxtLink to="/com/map">{{ $t('ui.map_addr') }}</NuxtLink></p>
        </MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('common.save') }}</button>
      <p v-if="checkHint" class="muted">{{ checkHint }}</p>
      <p v-if="msg">{{ msg }}</p>
      <p v-if="guide" class="yun_prompt_cont">
        {{ guide.msg }}
        <NuxtLink :to="guide.to" class="yun_m_job_r_l">{{ guide.act }}</NuxtLink>
      </p>
    </form>
    <div v-if="!error" class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="save">
        <MemberField wap :label="$t('wap_com_00157')">
          <input v-model="form.name" required :disabled="nameLocked" @blur="checkField('name', form.name)" />
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
          <img v-if="form.logo" :src="mediaUrl(form.logo)" width="40" height="40" alt="" />
        </MemberField>
        <MemberField wap :label="$t('member_com_00197')">
          <input type="file" accept="image/jpeg,image/png,image/webp" @change="onQcode" />
          <img v-if="form.comqcode" :src="mediaUrl(form.comqcode)" width="40" height="40" alt="" />
        </MemberField>
        <MemberField wap area :label="$t('ui.desc')">
          <textarea v-model="form.content" rows="6" />
        </MemberField>
        <MemberField wap :label="$t('wap_01431')">
          <input v-model="form.linkman" required />
        </MemberField>
        <MemberField wap :label="$t('wap_com_00288')">
          <input v-model="form.linkjob" />
        </MemberField>
        <MemberField wap :label="$t('wap_com_00142')">
          <input v-model="form.linktel" required :disabled="telLocked" @blur="checkField('linktel', form.linktel)" />
        </MemberField>
        <MemberField wap :label="$t('ui.linkphone')">
          <input v-model="form.linkphone" />
        </MemberField>
        <MemberField wap :label="$t('member_user_00282')">
          <input v-model="form.linkmail" :disabled="mailLocked" />
        </MemberField>
        <MemberField wap :label="$t('ui.map_addr')">
          <input v-model="form.address" />
        </MemberField>
        <MemberField wap :label="$t('wap_com_00160')">
          <input v-model="form.website" />
        </MemberField>
        <MemberField wap :label="$t('wap_com_00162')">
          <input v-model="form.busstops" />
        </MemberField>
        <MemberField wap :label="$t('wap_user_00091')">
          <input v-model="form.linkqq" />
        </MemberField>
        <MemberField wap :label="$t('member_user_00106')">
          <input v-model="form.sdate" />
        </MemberField>
        <MemberField wap :label="$t('wap_com_00158')">
          <input v-model.number="form.money" type="number" min="0" />
          <select v-model.number="form.moneytype">
            <option :value="1">{{ $t('common_02067') }}</option>
            <option :value="2">USD</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('wap_js_00005')">
          <label><input v-model.number="form.infostatus" type="radio" :value="1" /> {{ $t('wap_js_00005') }}</label>
          <label><input v-model.number="form.infostatus" type="radio" :value="2" /> {{ $t('wap_js_00003') }}</label>
        </MemberField>
        <MemberField wap :label="$t('common_02017')">
          <label v-for="w in welfares" :key="'h5wel-' + w.id">
            <input v-model="welNames" type="checkbox" :value="w.name" /> {{ w.name }}
          </label>
          <label v-for="n in welNames.filter((x) => !welfares.some((w) => w.name === x))" :key="'h5ex-' + n">
            <input v-model="welNames" type="checkbox" :value="n" /> {{ n }}
          </label>
          <input v-model="extraWel" maxlength="8" />
          <a href="javascript:;" @click.prevent="addWelfare">{{ $t('member_com_00202') }}</a>
        </MemberField>
        <MemberField wap :label="$t('member_com_00695')">
          <label><input v-model="disturbOn" type="checkbox" /> {{ $t('member_com_00695') }}</label>
        </MemberField>
        <MemberField v-if="disturbOn" wap :label="$t('wap_com_00150')">
          <input v-model="disturbStart" type="time" />
          {{ $t('common_02098') }}
          <input v-model="disturbEnd" type="time" />
        </MemberField>
        <MemberField wap :label="$t('wap_user_00243')">
          <MapPick v-model:x="form.x" v-model:y="form.y" :preset="form.address" />
          <p><NuxtLink to="/com/map">{{ $t('ui.map_addr') }}</NuxtLink></p>
        </MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.save') }}</button>
        <p v-if="checkHint" class="muted">{{ checkHint }}</p>
        <p v-if="msg">{{ msg }}</p>
        <p v-if="guide" class="yun_prompt_cont">
          {{ guide.msg }}
          <NuxtLink :to="guide.to">{{ guide.act }}</NuxtLink>
        </p>
      </form>
    </div>
  </MemberPanel>
</template>
