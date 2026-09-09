<script setup lang="ts">
import { isUnauthErr, mediaUrl } from '~/utils/site'

type CertMine = {
  uid?: number
  status?: number
  status_n?: string
  statusbody?: string
  company_name?: string
  social_credit?: string
  check?: string
  check_n?: string
  owner_cert?: string
  owner_cert_n?: string
  wt_cert?: string
  wt_cert_n?: string
  other_cert?: string
  other_cert_n?: string
  yyzz_status?: number
  com_social_credit?: number
  com_cert_owner?: number
  com_cert_wt?: number
  com_cert_other?: number
  com_cert_status?: number
  exa_cert_wt?: string
  pic_type?: string
  file_maxsize?: string
  review_tel?: string
}

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-cert', () =>
  api.post<CertMine>('/v1/mcenter/company/cert/list', {}),
)

const form = reactive({
  company_name: '',
  social_credit: '',
  check: '',
  owner_cert: '',
  wt_cert: '',
  other_cert: '',
})
const preview = reactive({
  check: '',
  owner_cert: '',
  wt_cert: '',
  other_cert: '',
})
const msg = ref('')

watch(
  data,
  (row) => {
    if (!row) return
    form.company_name = String(row.company_name || '')
    form.social_credit = String(row.social_credit || '')
    form.check = String(row.check || '')
    form.owner_cert = String(row.owner_cert || '')
    form.wt_cert = String(row.wt_cert || '')
    form.other_cert = String(row.other_cert || '')
    preview.check = String(row.check_n || '')
    preview.owner_cert = String(row.owner_cert_n || '')
    preview.wt_cert = String(row.wt_cert_n || '')
    preview.other_cert = String(row.other_cert_n || '')
  },
  { immediate: true },
)

function fail(e: unknown) {
  return e instanceof Error ? e.message : t('ui.failed')
}

type PicField = 'check' | 'owner_cert' | 'wt_cert' | 'other_cert'

async function upload(kind: PicField, ev: Event) {
  const input = ev.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  msg.value = ''
  try {
    const r = await $fetch<{ key: string; url: string }>('/api/upload/cert', {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    form[kind] = r.key || r.url
    preview[kind] = r.url || mediaUrl(r.key)
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

function statusText(row: CertMine | null) {
  const st = row?.status
  if (st === 1) return t('wap_com_00052')
  if (st === 0) return `${t('wap_00844')}${row?.review_tel || ''}`
  if (st === 2) {
    const reason = row?.statusbody
      ? `${t('admin_system_00134')}${row.statusbody}，`
      : ''
    return `${t('wap_user_00325')}，${reason}${t('wap_00845')}${row?.review_tel || ''}`
  }
  return t('wap_com_00053')
}

async function submit() {
  msg.value = ''
  const row = data.value
  if (!form.company_name.trim()) {
    msg.value = t('wap_00835')
    return
  }
  if (row?.com_social_credit === 1 && !form.social_credit.trim()) {
    msg.value = t('wap_00836')
    return
  }
  if (!form.check && !preview.check) {
    msg.value = t('wap_com_00059')
    return
  }
  if (row?.com_cert_owner === 1 && !form.owner_cert && !preview.owner_cert) {
    msg.value = t('wap_com_00058')
    return
  }
  if (row?.com_cert_wt === 1 && !form.wt_cert && !preview.wt_cert) {
    msg.value = t('wap_com_00056')
    return
  }
  try {
    await api.post('/v1/mcenter/company/cert', { ...form })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

useSeoMeta({ title: t('wap_com_00075') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00075')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <template v-else>
      <p class="cert-tip">{{ statusText(data) }}</p>
      <form class="form" @submit.prevent="submit">
        <label>
          <i class="req">*</i>{{ $t('wap_user_00080') }}
          <input v-model="form.company_name" type="text" />
        </label>
        <label v-if="data?.com_social_credit === 1">
          <i class="req">*</i>{{ $t('admin_user_company_00063') }}
          <input
            v-model="form.social_credit"
            type="text"
            maxlength="18"
            :placeholder="$t('wap_00837')"
          />
        </label>
        <label>
          <i class="req">*</i>{{ $t('wap_com_00054') }}
          <span class="muted">{{ $t('wap_com_00034') }}</span>
          <img v-if="preview.check" :src="mediaUrl(preview.check)" alt="" width="160" />
          <input type="file" accept="image/jpeg,image/png,image/webp,image/gif" @change="upload('check', $event)" />
        </label>
        <label v-if="data?.com_cert_owner === 1">
          <i class="req">*</i>{{ $t('member_com_00067') }}
          <span class="muted">{{ $t('wap_00842') }}</span>
          <img v-if="preview.owner_cert" :src="mediaUrl(preview.owner_cert)" alt="" width="160" />
          <input type="file" accept="image/jpeg,image/png,image/webp,image/gif" @change="upload('owner_cert', $event)" />
        </label>
        <label v-if="data?.com_cert_wt === 1">
          <i class="req">*</i>{{ $t('common_01902') }}/{{ $t('admin_user_company_00302') }}
          <span class="muted">{{ $t('wap_00843') }}</span>
          <p v-if="data?.exa_cert_wt" class="muted">
            {{ $t('wap_com_00057') }}
            <a :href="data.exa_cert_wt" target="_blank" rel="noopener">{{ $t('wap_com_00063') }}</a>
          </p>
          <img v-if="preview.wt_cert" :src="mediaUrl(preview.wt_cert)" alt="" width="160" />
          <input type="file" accept="image/jpeg,image/png,image/webp,image/gif" @change="upload('wt_cert', $event)" />
        </label>
        <label v-if="data?.com_cert_other === 1">
          {{ $t('member_com_00189') }}
          <span class="muted">{{ $t('wap_com_00055') }}</span>
          <img v-if="preview.other_cert" :src="mediaUrl(preview.other_cert)" alt="" width="160" />
          <input type="file" accept="image/jpeg,image/png,image/webp,image/gif" @change="upload('other_cert', $event)" />
        </label>
        <p v-if="data?.pic_type || data?.file_maxsize" class="muted">
          {{ $t('wap_com_00050') }} {{ data?.pic_type }} {{ data?.file_maxsize }}M
        </p>
        <button type="submit">{{ $t('common.submit') }}</button>
      </form>
      <p v-if="msg">{{ msg }}</p>
    </template>
  </MemberPanel>
</template>

<style scoped>
.cert-tip {
  margin: 0 0 16px;
  padding: 10px 12px;
  background: #f6f8fb;
  color: #555;
  border-radius: 4px;
}
.req {
  color: #e23c3c;
  font-style: normal;
  margin-right: 4px;
}
.form label {
  display: block;
  margin: 12px 0;
}
.form img {
  display: block;
  margin: 8px 0;
  max-width: 160px;
  height: auto;
}
</style>
