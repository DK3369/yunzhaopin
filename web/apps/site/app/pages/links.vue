<script setup lang="ts">
import { mediaUrl } from '~/utils/site'

type LinkRow = { id: number; name: string; url: string; logo?: string; category?: string }

const api = useApi()
const { t } = useI18n()
const { data } = await useAsyncData('links', () =>
  api.get<LinkRow[]>('/v1/wap/friend-links').catch(() => [] as LinkRow[]),
)
const list = computed(() => (Array.isArray(data.value) ? data.value : []) as LinkRow[])
const logoLinks = computed(() =>
  list.value.filter((r) => String(r.logo || '').trim() || String(r.category || '') === '2'),
)
const textLinks = computed(() => {
  const logos = new Set(logoLinks.value.map((r) => r.id))
  return list.value.filter((r) => !logos.has(r.id))
})
const form = reactive({ name: '', url: '', captcha_cid: '', captcha_input: '' })
const captcha = ref<{ cid: string; image: string } | null>(null)
const msg = ref('')
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
  form.captcha_input = ''
}
onMounted(loadCaptcha)
async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/wap/friend-links/apply', { ...form })
    msg.value = t('common.success')
    form.name = ''
    form.url = ''
    await loadCaptcha()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
    await loadCaptcha()
  }
}
useSeoMeta({ title: t('default_00256') })
useHead({ link: [{ rel: 'canonical', href: '/links' }] })
</script>

<template>
  <div class="yun_content">
    <div class="yun_link_content">
      <div class="yun_link_content_tit">
        <i class="yun_link_content_tit_icon" />{{ $t('default_00256') }}
      </div>
      <p v-if="!list.length" class="muted">{{ $t('ui.no_links') }}</p>
      <ul v-else class="yun_link_content_list">
        <li v-for="row in logoLinks" :key="`logo-${row.id}`">
          <a :href="row.url" rel="nofollow noopener" target="_blank">
            <img
              v-if="row.logo"
              :src="mediaUrl(row.logo)"
              :alt="row.name"
              style="width: 120px; height: 38px"
            />
            <span v-else>{{ row.name }}</span>
          </a>
        </li>
      </ul>
      <div class="yun_link_content_linkp">
        <a v-for="row in textLinks" :key="`txt-${row.id}`" :href="row.url" rel="nofollow noopener" target="_blank">
          {{ row.name }}
        </a>
      </div>
    </div>
    <div class="yun_link_content">
      <div class="yun_link_content_tit">
        <i class="yun_link_content_tit_icon" />{{ $t('common_02354') }}
      </div>
      <div class="clear" />
      <form class="form" @submit.prevent="submit">
        <input v-model="form.name" required :placeholder="$t('admin_system_00292')" />
        <input v-model="form.url" required :placeholder="$t('admin_00101')" />
        <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
        <input v-model="form.captcha_input" :placeholder="$t('wap_00110')" />
        <button type="submit">{{ $t('common.submit') }}</button>
      </form>
      <p v-if="msg">{{ msg }}</p>
    </div>
    <div class="clear" />
  </div>
</template>
