<script setup lang="ts">
import { ensurePublicFound } from '~/utils/site'

const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(`tiny-${id}`, () => api.get('/v1/wap/tiny-resumes/show', { id }))
const password = ref('')
const msg = ref('')
const owned = ref<Record<string, unknown> | null>(null)
const edit = reactive({
  username: '',
  sex: 1,
  exp: 1,
  job: '',
  mobile: '',
  production: '',
})

async function verify() {
  msg.value = ''
  try {
    const r = await api.post<Record<string, unknown>>('/v1/wap/tiny-resumes/verify', { id, password: password.value })
    owned.value = r
    edit.username = String(r.username || '')
    edit.sex = Number(r.sex || 1)
    edit.exp = Number(r.exp || 1)
    edit.job = String(r.job || '')
    edit.mobile = String(r.mobile || '')
    edit.production = String(r.production || '')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function refresh() {
  msg.value = ''
  try {
    await api.post('/v1/wap/tiny-resumes/refresh', { id, password: password.value })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function save() {
  msg.value = ''
  try {
    await api.post('/v1/wap/tiny-resumes/update', { id, password: password.value, ...edit })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function remove() {
  msg.value = ''
  try {
    await api.post('/v1/wap/tiny-resumes/delete', { id, password: password.value })
    msg.value = t('common.success')
    await navigateTo('/tiny')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
ensurePublicFound(Boolean(data.value?.username || data.value?.id), error.value)
useSeoMeta({ title: () => String(data.value?.username || t('wap_js_00066')) })
useHead({ link: [{ rel: 'canonical', href: `/tiny/${id}` }] })
</script>

<template>
  <article class="site-pc">
    <h1>{{ data?.username || $t('common_02409') }}</h1>
    <p v-if="data?.job" class="muted">{{ data.job }} · {{ data.exp_n || data.exp }}</p>
    <p v-if="data?.sex_n || Number(data?.sex)" class="muted">
      {{ data.sex_n || (Number(data?.sex) === 2 ? $t('common_02069') : Number(data?.sex) === 1 ? $t('common_02092') : '') }}
    </p>
    <p
      v-if="data?.province_name || data?.city_name"
      class="muted"
    >{{ [data?.province_name, data?.city_name, data?.three_city_name].filter(Boolean).join('-') }}</p>
    <p v-if="Number(data?.hits)" class="muted">{{ $t('member_com_00268') }}：{{ data.hits }} {{ $t('common_02089') }}</p>
    <p v-if="data?.mobile_masked" class="muted">{{ data.mobile_masked }}</p>
    <p v-if="data?.production">{{ data.production }}</p>
    <p v-else-if="!data?.username" class="muted">{{ $t('common_02409') }}</p>
    <form class="form" @submit.prevent="verify">
      <input v-model="password" type="password" :placeholder="$t('wap_user_00371')" required />
      <button type="submit">{{ $t('common.confirm') }}</button>
    </form>
    <form v-if="owned" class="form" @submit.prevent="save">
      <input v-model="edit.username" required />
      <select v-model.number="edit.sex">
        <option :value="1">{{ $t('common_02092') }}</option>
        <option :value="2">{{ $t('common_02069') }}</option>
      </select>
      <input v-model="edit.job" required />
      <input v-model="edit.mobile" required />
      <textarea v-model="edit.production" rows="5" required />
      <button type="submit">{{ $t('common.save') }}</button>
      <button type="button" @click="refresh">{{ $t('common.latest') }}</button>
      <button type="button" @click="remove">{{ $t('common.delete') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </article>
  <div class="site-h5">
    <div class="tiny_bg" />
    <div class="tiny_bg_t">
      <div class="com_new_contnet_box">
        <div class="com_show_t1">
          <h2>{{ data?.username || $t('common_02409') }}</h2>
        </div>
        <div class="com_show_t2">{{ $t('wap_com_00353') }}：{{ data?.job }}</div>
        <div v-if="Number(data?.hits)" class="com_show_t2">{{ $t('wap_user_00221') }}：{{ data?.hits }}</div>
      </div>
    </div>
    <div class="mt10">
      <div class="com_new_contnet_box">
        <div class="wap_title">{{ $t('wap_00456') }}</div>
        <ul class="user_contnet_ul">
          <li class="com_show_li">
            <span class="user_contnet_info_n">{{ $t('wap_com_00303') }}：</span>
            {{ data?.sex_n || (Number(data?.sex) === 2 ? $t('common_02069') : Number(data?.sex) === 1 ? $t('common_02092') : '') }}
          </li>
          <li class="com_show_li">
            <span class="user_contnet_info_n">{{ $t('wap_00526') }}：</span>{{ data?.exp_n || data?.exp }}
          </li>
          <li v-if="data?.province_name || data?.city_name" class="com_show_li">
            <span class="user_contnet_info_n">{{ $t('wap_00349') }}：</span>
            {{ [data?.province_name, data?.city_name, data?.three_city_name].filter(Boolean).join('-') }}
          </li>
          <li v-if="data?.mobile_masked" class="com_show_li">
            <span class="user_contnet_info_n">{{ $t('wap_user_00265') }}：</span>{{ data.mobile_masked }}
          </li>
        </ul>
      </div>
    </div>
    <div class="mt10">
      <div class="com_new_contnet_box">
        <div class="wap_title">{{ $t('wap_00527') }}</div>
        <div class="tiny_show_content">
          <p v-if="data?.production">{{ data.production }}</p>
          <p v-else-if="!data?.username" class="muted">{{ $t('common_02409') }}</p>
        </div>
      </div>
    </div>
    <div class="mt10">
      <div class="com_new_contnet_box">
        <div class="wap_title">{{ $t('wap_00363') }}</div>
        <div class="tiny_tag">{{ $t('wap_00642') }}</div>
        <form class="form" @submit.prevent="verify">
          <input v-model="password" type="password" :placeholder="$t('wap_user_00371')" required />
          <button type="submit">{{ $t('common.confirm') }}</button>
        </form>
        <form v-if="owned" class="form" @submit.prevent="save">
          <input v-model="edit.username" required />
          <select v-model.number="edit.sex">
            <option :value="1">{{ $t('common_02092') }}</option>
            <option :value="2">{{ $t('common_02069') }}</option>
          </select>
          <input v-model="edit.job" required />
          <input v-model="edit.mobile" required />
          <textarea v-model="edit.production" rows="5" required />
          <a href="javascript:;" class="tiny_cz_sx" @click.prevent="refresh">{{ $t('wap_user_00334') }}</a>
          <button type="submit" class="tiny_cz_sx">{{ $t('wap_js_00073') }}</button>
          <a href="javascript:;" class="tiny_cz" @click.prevent="remove">{{ $t('common.delete') }}</a>
        </form>
        <p v-if="msg">{{ msg }}</p>
      </div>
    </div>
  </div>
</template>
