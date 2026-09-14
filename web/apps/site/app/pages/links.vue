<script setup lang="ts">
import { mediaUrl } from '~/utils/site'

type LinkRow = { id: number; name: string; url: string; logo?: string; category?: string }

const api = useApi()
const { t } = useI18n()
const runtime = useRuntimeConfig()
const { settings, siteName, phone, email } = useSiteChrome()
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
const applyOpen = computed(() => String(settings.value.sy_linksq || '') === '1')
const weburl = computed(
  () => String(settings.value.sy_weburl || '').trim() || String(runtime.public.siteUrl || '').replace(/\/$/, ''),
)
const webtel = computed(() => String(settings.value.sy_webtel || '').trim())
const form = reactive({ name: '', url: '', link_type: '1', pic: '', captcha_cid: '', captcha_input: '' })
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
  if (!form.link_type) {
    msg.value = t('admin_system_00287')
    return
  }
  if (form.link_type === '2' && !form.pic.trim()) {
    msg.value = t('common_02362')
    return
  }
  try {
    await api.post('/v1/wap/friend-links/apply', { ...form })
    msg.value = t('common.success')
    form.name = ''
    form.url = ''
    form.pic = ''
    form.link_type = '1'
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
    <div v-if="applyOpen" class="yun_link_content">
      <div class="yun_link_content_tit">
        <i class="yun_link_content_tit_icon" />{{ $t('common_02354') }}
      </div>
      <div class="clear" />
      <div class="fri_left">
        <p>
          <strong>{{ $t('member_com_00207') }}:</strong><br />
          {{ $t('common_02355') }}<br />
          {{ $t('common_02356') }}<br />
          {{ $t('common_02357') }}<br />
          {{ $t('admin_user_company_00160') }}：
          <span style="color: #006697">
            <a :href="weburl">{{ weburl }}</a>
          </span>
          <br />
          {{ $t('member_com_00021') }}：{{ siteName }}<br />
          {{ $t('common_02358') }}<br />
          <span style="color: #ff5003; display: block; font-size: 18px; font-weight: bold">{{ phone }}</span>
          {{ $t('default_00255') }}
          <span style="color: #006697">{{ webtel }}</span>
          <br />
          {{ $t('member_user_00282') }}：
          <span style="color: #006697">{{ email }}</span>
        </p>
      </div>
      <div class="fri_right">
        <form @submit.prevent="submit">
          <ul>
            <li>
              <strong>{{ $t('admin_system_00688') }}：</strong>
              <select v-model="form.link_type" class="bot">
                <option value="1">{{ $t('admin_01013') }}</option>
                <option value="2">{{ $t('admin_00100') }}</option>
              </select>
            </li>
            <li>
              <strong>{{ $t('admin_system_00292') }}：</strong>
              <input v-model="form.name" class="bot" required :placeholder="$t('admin_system_00292')" />
              {{ $t('common_02359') }}
            </li>
            <li>
              <strong>{{ $t('admin_00101') }}：</strong>
              <input v-model="form.url" class="bot" required :placeholder="$t('admin_01009')" />
            </li>
            <li v-if="form.link_type === '2'">
              <strong>{{ $t('admin_01010') }}：</strong>
              <input v-model="form.pic" class="bot" :placeholder="$t('common_02360')" />
            </li>
            <li>
              <strong>{{ $t('wap_00110') }}：</strong>
              <input v-model="form.captcha_input" class="bot" :placeholder="$t('wap_00110')" />
              <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
            </li>
            <li>
              <strong>&nbsp;</strong>
              <button class="login_button2" type="submit">{{ $t('common.submit') }}</button>
            </li>
          </ul>
        </form>
        <p v-if="msg">{{ msg }}</p>
        <div class="clear" />
      </div>
    </div>
    <div class="clear" />
  </div>
</template>
