<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'
import { runPackedTranslate } from '../../../../../layers/base/app/utils/packedText'

const api = useApi()
const { t, te, locale, getLocaleMessage } = useI18n()
function packed(text: unknown) {
  const loc = String(locale.value).toLowerCase().startsWith('en') ? 'en' : 'zh'
  const zhRoot = getLocaleMessage('zh') as Record<string, unknown>
  return runPackedTranslate(text, {
    locale: loc,
    lc: (k) => (te(k) ? String(t(k)) : k),
    zhRoot,
  })
}
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data: bal, error } = await useAsyncData('user-finance-bal', () =>
  api.post('/v1/mcenter/integral/balance', {}),
)
const { data: pays } = await useAsyncData(
  () => `user-finance-pay-${page.value}`,
  () => api.post('/v1/mcenter/integral/consumes', { page: page.value, page_size: pageSize }).catch(() => ({ list: [] })),
)
const { data: rewards } = await useAsyncData('user-finance-ex', () =>
  api.post('/v1/mcenter/integral/history', { page: 1, page_size: 20 }).catch(() => ({ list: [] })),
)
const { data: signSt, refresh: refreshSign } = await useAsyncData('user-finance-sign', () =>
  api.post<{ signed_today?: boolean }>('/v1/mcenter/sign/status', {}).catch(() => null),
)
const msg = ref('')
async function sign() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/sign', {})
    msg.value = t('common.success')
    await refreshSign()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00213') })
const payTotal = computed(() => inferTotal(pays.value))
</script>

<template>
  <MemberPanel :title="$t('wap_user_00213')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <template v-else>
      <div class="financial_management_header">
        <div class="management_header_card">
          <div class="management_card_data">
            <div class="management_data_top">{{ $t('ui.balance') }}</div>
            <div class="management_data_cen">{{ bal?.balance ?? 0 }}</div>
            <div class="management_data_bom">
              <NuxtLink to="/user/integral">{{ $t('wap_user_00008') }}</NuxtLink>
            </div>
          </div>
          <div class="management_card_data">
            <div class="management_data_top">{{ $t('member_user_00190') }}</div>
            <div class="management_data_cen">{{ (pays?.list || []).length }}</div>
            <NuxtLink to="/user/pay" class="management_data_bom">{{ $t('common_01946') }}</NuxtLink>
          </div>
        </div>
        <NuxtLink to="/user/pay" class="management_card_btn">{{ $t('common_01946') }}</NuxtLink>
      </div>
      <p>
        <button type="button" class="integral_list_bth_a site-pc" :disabled="signSt?.signed_today" @click="sign">{{ $t('wap_01023') }}</button>
        <button type="button" class="verification_form_btn site-h5" :disabled="signSt?.signed_today" @click="sign">{{ $t('wap_01023') }}</button>
      </p>
      <div v-if="(pays?.list || []).length" class="paylist_tit site-pc">
        <span class="paylist_span paylist_span_dh">{{ $t('ui.detail') }}</span>
        <span class="paylist_span paylist_span_money">{{ $t('wap_00925') }}</span>
        <span class="paylist_span paylist_span_time">{{ $t('member_user_00106') }}</span>
      </div>
      <div v-for="row in pays?.list || []" :key="row.id" class="paylist_list site-pc">
        <span class="paylist_span paylist_span_dh">{{ packed(row.detail) }}</span>
        <span class="paylist_span paylist_span_money">{{ row.delta }}</span>
        <span class="paylist_span paylist_span_time">{{ row.ctime_n || row.ctime }}</span>
      </div>
      <div class="site-h5 m_cardbox">
        <MemberSxNewsCard
          v-for="row in pays?.list || []"
          :key="'h5-' + row.id"
          :title="String(packed(row.detail) || row.delta)"
          :time="row.ctime_n || row.ctime"
        />
      </div>
      <MemberPager :page="page" :page-size="pageSize" :total="payTotal" @update:page="go" />
      <p v-if="msg">{{ msg }}</p>
    </template>
  </MemberPanel>
</template>
