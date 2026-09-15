<script setup lang="ts">
import { catTree } from '~/utils/site'

type SubscribeMeta = { jionly?: number; cionly?: number; cycles?: number[] }

const { t } = useI18n()
const api = useApi()
const { me, siteName } = useSiteChrome()

const { data: meta } = await useAsyncData(
  localeAsyncKey('subscribe-meta'),
  () =>
    api.get<SubscribeMeta>('/v1/wap/subscribe/meta').catch(() => ({
      jionly: 0,
      cionly: 0,
      cycles: [3, 7, 14, 26],
    })),
)
const { data: jobCats } = await useJobCats()

const jionly = computed(() => Number(meta.value?.jionly || 0) === 1)
const cionly = computed(() => Number(meta.value?.cionly || 0) === 1)
const cycles = computed(() => (meta.value?.cycles?.length ? meta.value.cycles : [3, 7, 14, 26]))
const jobRoots = computed(() => catTree(jobCats.value || [], 80))
const jobLevel2 = computed(() => jobRoots.value.find((c) => c.id === form.job1)?.children || [])
const jobLevel3 = computed(() => jobLevel2.value.find((c) => c.id === form.job1_son)?.children || [])

const form = reactive({
  job1: 0,
  job1_son: 0,
  job_post: 0,
  country: '',
  provinceid: 0,
  cityid: 0,
  three_cityid: 0,
  minsalary: 0,
  maxsalary: 0,
  time: 0,
  email: '',
  captcha_cid: '',
  captcha_input: '',
})
const captcha = ref<{ cid: string; image: string } | null>(null)
const msg = ref('')
const doneEmail = ref('')

watch(
  () => form.job1,
  () => {
    form.job1_son = 0
    form.job_post = 0
  },
)
watch(
  () => form.job1_son,
  () => {
    form.job_post = 0
  },
)

async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
  form.captcha_input = ''
}

onMounted(async () => {
  await loadCaptcha()
  const em = String(me.value?.email || '').trim()
  if (em) form.email = em
})

function clearForm() {
  form.job1 = 0
  form.job1_son = 0
  form.job_post = 0
  form.country = ''
  form.provinceid = 0
  form.cityid = 0
  form.three_cityid = 0
  form.minsalary = 0
  form.maxsalary = 0
  form.time = 0
  form.email = ''
  form.captcha_input = ''
  msg.value = ''
}

function checkForm(): boolean {
  if (jionly.value) {
    if (!form.job1) {
      msg.value = t('admin_user_company_00023')
      return false
    }
  } else if (!form.job1_son) {
    msg.value = t('admin_user_company_00023')
    return false
  }
  if (cionly.value) {
    if (!form.provinceid) {
      msg.value = t('wap_00901')
      return false
    }
  } else if (!form.cityid) {
    msg.value = t('wap_00901')
    return false
  }
  if (!form.minsalary || form.minsalary <= 0) {
    msg.value = t('wap_01484')
    return false
  }
  if (form.maxsalary > 0 && form.maxsalary < form.minsalary) {
    msg.value = t('wap_01484')
    return false
  }
  if (!form.time) {
    msg.value = t('admin_tool_00230')
    return false
  }
  if (!me.value && !form.email.trim()) {
    msg.value = t('member_user_00282')
    return false
  }
  if (!form.captcha_input.trim()) {
    msg.value = t('wap_js_00129')
    return false
  }
  return true
}

async function submit() {
  msg.value = ''
  if (!checkForm()) return
  try {
    await api.post('/v1/wap/subscribe', {
      job1: form.job1,
      job1_son: form.job1_son,
      job_post: form.job_post,
      provinceid: form.provinceid,
      cityid: form.cityid,
      three_cityid: form.three_cityid,
      minsalary: Number(form.minsalary) || 0,
      maxsalary: Number(form.maxsalary) || 0,
      time: form.time,
      email: form.email.trim(),
      type: 1,
      captcha_cid: form.captcha_cid,
      captcha_input: form.captcha_input,
    })
    doneEmail.value = form.email.trim()
    await loadCaptcha()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
    await loadCaptcha()
  }
}

async function sendNotice() {
  msg.value = ''
  if (!doneEmail.value) return
  if (!form.captcha_input.trim()) {
    msg.value = t('wap_js_00129')
    return
  }
  try {
    await api.post('/v1/wap/subscribe/send-email', {
      email: doneEmail.value,
      captcha_cid: form.captcha_cid,
      captcha_input: form.captcha_input,
    })
    msg.value = t('common.success')
    await loadCaptcha()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
    await loadCaptcha()
  }
}

useSeoMeta({ title: () => t('ui.subscribe_svc') })
useHead({ link: [{ rel: 'canonical', href: '/subscribe' }] })
</script>

<template>
  <div class="container">
    <div v-if="doneEmail" class="container_cont">
      <div class="container_cont_fr">
        <div class="post_read_h1">
          <span class="post_read_title">{{ $t('default_00303') }}</span>
        </div>
        <p class="post_read_p">
          {{ $t('common_01852') }}
          <span class="post_email">{{ doneEmail }}</span>
        </p>
        <p class="post_read_body_c">{{ $t('default_00300') }}</p>
        <div class="subscribe_cont_list">
          <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
          <input
            v-model="form.captcha_input"
            class="post_read_valid"
            :placeholder="$t('wap_00110')"
          />
          <button type="button" class="post_read_bth1" @click="sendNotice">{{ $t('admin_user_00167') }}</button>
        </div>
        <p v-if="msg" class="subscribe_cont_tips">{{ msg }}</p>
      </div>
    </div>
    <div v-else class="subscribe_cont">
      <div class="subscribe_cont_h1">
        <span class="subscribe_cont_h1_p">{{ $t('ui.subscribe_svc') }}</span>
        {{ siteName }}
      </div>
      <form @submit.prevent="submit">
        <div class="subscribe_cont_list">
          <span class="subscribe_cont_list_name">{{ $t('admin_user_company_00377') }}</span>
          <select v-model.number="form.job1" class="sub_xz_text">
            <option :value="0">{{ $t('wap_user_00100') }}</option>
            <option v-for="row in jobRoots" :key="row.id" :value="row.id">{{ row.name }}</option>
          </select>
          <select v-if="!jionly" v-model.number="form.job1_son" class="sub_xz_text">
            <option :value="0">{{ $t('wap_user_00100') }}</option>
            <option v-for="row in jobLevel2" :key="row.id" :value="row.id">{{ row.name }}</option>
          </select>
          <select v-if="!jionly && jobLevel3.length" v-model.number="form.job_post" class="sub_xz_text">
            <option :value="0">{{ $t('wap_user_00100') }}</option>
            <option v-for="row in jobLevel3" :key="row.id" :value="row.id">{{ row.name }}</option>
          </select>
        </div>
        <div class="subscribe_cont_list">
          <span class="subscribe_cont_list_name">{{ $t('member_user_00198') }}</span>
          <LocationFields
            v-model:country="form.country"
            v-model:province-id="form.provinceid"
            v-model:city-id="form.cityid"
            v-model:district-id="form.three_cityid"
            :required="!cionly"
          />
        </div>
        <div class="subscribe_cont_list">
          <span class="subscribe_cont_list_name">{{ $t('wap_01484') }}</span>
          <input
            v-model.number="form.minsalary"
            class="sub_xz_text"
            type="number"
            min="1"
            :placeholder="$t('ui.min_salary')"
          />
          <span class="sub_xz_text_line">-</span>
          <input
            v-model.number="form.maxsalary"
            class="sub_xz_text"
            type="number"
            min="0"
            :placeholder="$t('ui.max_salary')"
          />
          <span class="subscribe_cont_dw">{{ $t('common.salary_yuan') }}</span>
        </div>
        <div class="subscribe_cont_list">
          <span class="subscribe_cont_list_name">{{ $t('admin_tool_00230') }}</span>
          <select v-model.number="form.time" class="sub_xz_text">
            <option :value="0">{{ $t('wap_user_00100') }}</option>
            <option v-for="n in cycles" :key="n" :value="n">{{ n }}{{ $t('common_02067') }}</option>
          </select>
        </div>
        <div class="subscribe_cont_list">
          <span class="subscribe_cont_list_name">{{ $t('member_user_00282') }}</span>
          <input
            v-model="form.email"
            class="post_read_valid"
            type="email"
            :placeholder="$t('ui.email_addr')"
            :required="!me"
          />
        </div>
        <div class="subscribe_cont_list">
          <span class="subscribe_cont_list_name">{{ $t('wap_00110') }}</span>
          <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
          <input v-model="form.captcha_input" class="post_read_valid" :placeholder="$t('wap_00110')" />
        </div>
        <div class="subscribe_cont_bth">
          <span class="subscribe_cont_list_name">&nbsp;</span>
          <button type="submit" class="post_read_bth1">{{ $t('common.submit') }}</button>
          <button type="button" class="post_read_bth2" @click="clearForm">{{ $t('member_user_00259') }}</button>
        </div>
        <p v-if="msg" class="subscribe_cont_tips">{{ msg }}</p>
      </form>
    </div>
    <div class="clear" />
  </div>
</template>
