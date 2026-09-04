<template>
  <div
    class="new_jobshow_telbox"
    style="position: fixed; left: 0.4rem; right: 0.4rem; bottom: 1.4rem; z-index: 90; background: #fff; border-radius: 0.16rem; padding: 0.32rem; box-shadow: 0 4px 16px rgba(0,0,0,.12)"
  >
    <div class="new_jobshow_leftname">{{ $t('wap_00266') }}</div>
    <div>
      <button
        v-for="r in reasons"
        :key="r.code"
        type="button"
        class="job_tckxz"
        :class="{ job_tckxz_cur: selected === r.code }"
        @click="selected = r.code"
      >
        {{ r.name }}
      </button>
    </div>
    <textarea v-model="detail" class="mt10" :placeholder="$t('wap_00245')" />
    <div class="mt10">
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="captchaInput" :placeholder="$t('wap_00110')" />
    </div>
    <p v-if="msg" class="muted">{{ msg }}</p>
    <a href="javascript:;" class="new_jobshow_telbth" @click.prevent="submit">{{ $t('wap_00290') }}</a>
    <a href="javascript:;" class="new_jobshow_telbth" @click.prevent="emit('close')">{{ $t('common.close') }}</a>
  </div>
</template>

<script setup lang="ts">
type Reason = { id?: number; code: string; name: string }

const props = defineProps<{ targetKind: number; targetId: number }>()
const emit = defineEmits<{ close: []; done: [] }>()
const { t } = useI18n()
const api = useApi()
const reasons = ref<Reason[]>([])
const selected = ref('')
const detail = ref('')
const msg = ref('')
const captcha = ref<{ cid: string; image: string } | null>(null)
const captchaInput = ref('')
const { me } = useSiteChrome()

onMounted(async () => {
  if (!me.value || Number(me.value.usertype) !== 1) {
    await navigateTo('/login')
    return
  }
  try {
    const r = await api.post<Reason[]>('/v1/wap/site/settings', { key: 'report_reasons' })
    reasons.value = Array.isArray(r) ? r.filter((x) => x && x.code && x.name) : []
  } catch {
    reasons.value = []
  }
  await loadCaptcha()
})

async function loadCaptcha() {
  try {
    captcha.value = await api.post('/v1/wap/captcha')
    captchaInput.value = ''
  } catch {
    captcha.value = null
  }
}

async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/reports', {
      target_kind: props.targetKind,
      target_id: props.targetId,
      reason_code: selected.value || 'other',
      detail: detail.value.trim(),
      captcha_cid: captcha.value?.cid,
      captcha_input: captchaInput.value,
    })
    msg.value = t('common.confirm')
    emit('done')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
    await loadCaptcha()
  }
}
</script>
