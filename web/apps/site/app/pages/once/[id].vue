<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(`once-${id}`, () =>
  api.get('/v1/wap/once-jobs/show', { id }).catch((e: unknown) => {
    const key = (e as { key?: string; data?: { key?: string } })?.key || (e as { data?: { key?: string } })?.data?.key || ''
    if (key === 'once_unpaid' || key === 'once_not_approved') return null
    throw e
  }),
)
const password = ref('')
const msg = ref('')
const owned = ref<Record<string, unknown> | null>(null)
const gears = ref<Array<{ id: number; days: number; price: number }>>([])
const paytype = ref('alipay')
const gearId = ref(0)
const edit = reactive({
  title: '',
  companyname: '',
  linkman: '',
  linktel: '',
  address: '',
  require: '',
  salary: '',
  mans: '',
})

async function verify() {
  msg.value = ''
  try {
    const r = await api.post<Record<string, unknown>>('/v1/wap/once-jobs/verify', { id, password: password.value })
    owned.value = r
    edit.title = String(r.title || '')
    edit.companyname = String(r.companyname || '')
    edit.linkman = String(r.linkman || '')
    edit.linktel = String(r.linktel || '')
    edit.address = String(r.address || '')
    edit.require = String(r.require || '')
    edit.salary = String(r.salary_text || r.salary || '')
    edit.mans = String(r.mans || '')
    if (Number(r.pay) === 1 && !gears.value.length) {
      gears.value = await api.get('/v1/wap/once-jobs/gears').catch(() => [])
      if (gears.value.length) gearId.value = gears.value[0].id
    }
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function refresh() {
  msg.value = ''
  try {
    await api.post('/v1/wap/once-jobs/refresh', { id, password: password.value })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function save() {
  msg.value = ''
  try {
    await api.post('/v1/wap/once-jobs/update', { id, password: password.value, ...edit })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function remove() {
  msg.value = ''
  try {
    await api.post('/v1/wap/once-jobs/delete', { id, password: password.value })
    msg.value = t('common.success')
    await navigateTo('/once')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function pay() {
  msg.value = ''
  try {
    const r = await api.post<{ order_id?: string; state?: number; fast?: string; pay_url?: string; msg?: string }>('/v1/wap/once-jobs/pay', {
      id,
      password: password.value,
      paytype: paytype.value,
      oncepricegear: gearId.value,
    })
    if (r.fast) {
      localStorage.setItem('once_fast', r.fast)
      document.cookie = `fast=${encodeURIComponent(r.fast)};path=/;max-age=${60 * 60 * 24 * 30}`
    }
    if (r.pay_url) {
      window.location.assign(r.pay_url)
      return
    }
    msg.value = Number(r.state) === 2 ? t('common.success') : String(r.msg || r.order_id || t('common.success'))
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
useSeoMeta({ title: () => String(data.value?.title || data.value?.companyname || t('wap_js_00130')) })
useHead({ link: [{ rel: 'canonical', href: `/once/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || data?.companyname || $t('wap_00630') }}</h1>
    <p v-if="data?.companyname" class="muted">{{ data.companyname }}</p>
    <p v-if="data?.address" class="muted">{{ data.address }}</p>
    <p v-if="data?.linkman_masked" class="muted">{{ $t('wap_01431') }} {{ data.linkman_masked }} · {{ data.linktel_masked }}</p>
    <p v-if="data?.require">{{ data.require }}</p>
    <p v-else-if="!data && error" class="muted">{{ $t('wap_00630') }}</p>
    <form class="form" @submit.prevent="verify">
      <input v-model="password" type="password" :placeholder="$t('wap_01353')" required />
      <button type="submit">{{ $t('common.confirm') }}</button>
    </form>
    <template v-if="owned">
      <p v-if="Number(owned.pay) === 1" class="muted">{{ $t('wap_01381') }}</p>
      <p>{{ $t('wap_01431') }}：{{ owned.linkman }} · {{ owned.linktel }}</p>
      <form v-if="Number(owned.pay) === 1" class="form" @submit.prevent="pay">
        <select v-model.number="gearId">
          <option v-for="g in gears" :key="g.id" :value="g.id">{{ g.days }}{{ $t('wap_01375') }} · {{ g.price }}</option>
        </select>
        <select v-model="paytype">
          <option value="alipay">alipay</option>
          <option value="wxpay">wxpay</option>
        </select>
        <button type="submit">{{ $t('common.submit') }}</button>
      </form>
      <form v-else class="form" @submit.prevent="save">
        <input v-model="edit.title" required />
        <input v-model="edit.companyname" required />
        <input v-model="edit.linkman" required />
        <input v-model="edit.linktel" required />
        <input v-model="edit.address" required />
        <input v-model="edit.salary" />
        <input v-model="edit.mans" />
        <textarea v-model="edit.require" rows="4" required />
        <button type="submit">{{ $t('common.save') }}</button>
        <button type="button" @click="refresh">{{ $t('common.latest') }}</button>
        <button type="button" @click="remove">{{ $t('common.delete') }}</button>
      </form>
    </template>
    <p v-if="msg">{{ msg }}</p>
  </article>
</template>
