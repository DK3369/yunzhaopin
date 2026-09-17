<script setup lang="ts">
import { ApiError } from '~/utils/envelope'
import { ensureLogin } from '~/utils/site'

const route = useRoute()
const id = Number(route.params.id)
const { t } = useI18n()
const { me } = useSiteChrome()
const api = useApi()
const { data } = await useAsyncData(`reward-${id}`, () =>
  api.get('/v1/wap/redeem/rewards/detail', { id }),
)
const form = reactive({
  password: '',
  linkman: '',
  linktel: '',
  address: '',
  provinceid: 0,
  cityid: 0,
  three_cityid: 0,
  num: 1,
})
const msg = ref('')
const needPay = ref(false)
const payTo = computed(() => (Number(me.value?.usertype) === 2 ? '/com/pay' : '/user/pay'))
async function submit() {
  msg.value = ''
  needPay.value = false
  if (!(await ensureLogin(me.value, route.fullPath))) return
  try {
    await api.post('/v1/mcenter/redeem/rewards/redeem', { id, ...form })
    msg.value = t('model_00051')
  } catch (e: unknown) {
    const key = e instanceof ApiError ? e.key : ''
    if (key === 'insufficient_balance') {
      needPay.value = true
      msg.value = e instanceof Error ? e.message : t('ui.failed')
      return
    }
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
useSeoMeta({ title: () => String(data.value?.name || t('ui.redeem')) })
useHead({ link: [{ rel: 'canonical', href: `/redeem/${id}` }] })
</script>

<template>
  <article class="site-pc">
    <h1>{{ data?.name || $t('wap_00611') }}</h1>
    <p v-if="data?.integral" class="muted">{{ data.integral }} {{ $t('wap_user_00008') }} · {{ $t('admin_yunying_00118') }} {{ data.remaining }}</p>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else-if="!data?.name" class="muted">{{ $t('wap_00611') }}</p>
    <form v-if="data?.name" class="form" @submit.prevent="submit">
      <input v-model="form.password" type="password" required :placeholder="$t('wap_01273')" />
      <input v-model="form.linkman" required :placeholder="$t('wap_01619')" />
      <input v-model="form.linktel" required />
      <LocationFields
        v-model:province-id="form.provinceid"
        v-model:city-id="form.cityid"
        v-model:district-id="form.three_cityid"
      />
      <input v-model="form.address" />
      <input v-model.number="form.num" type="number" min="1" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">
      {{ msg }}
      <NuxtLink v-if="needPay" :to="payTo" class="cblue">{{ $t('common_01946') }}</NuxtLink>
    </p>
  </article>
  <div class="site-h5 news_cont_box">
    <div class="news_cont_box_tit"><h1>{{ data?.name || $t('wap_00611') }}</h1></div>
    <div class="wap_news_cont">
      <div v-if="data?.content" class="wap_txt" v-html="data.content" />
      <form v-if="data?.name" class="yun_createbox" @submit.prevent="submit">
        <MemberField wap :label="$t('wap_user_00371')">
          <input v-model="form.password" type="password" required :placeholder="$t('wap_01273')" />
        </MemberField>
        <MemberField wap :label="$t('ui.linkman')">
          <input v-model="form.linkman" required :placeholder="$t('wap_01619')" />
        </MemberField>
        <MemberField wap :label="$t('common.phone')">
          <input v-model="form.linktel" required />
        </MemberField>
        <MemberField wap :label="$t('ui.map_addr')">
          <LocationFields
            v-model:province-id="form.provinceid"
            v-model:city-id="form.cityid"
            v-model:district-id="form.three_cityid"
          />
        </MemberField>
        <MemberField wap :label="$t('ui.map_addr')">
          <input v-model="form.address" />
        </MemberField>
        <MemberField wap :label="$t('ui.qty')">
          <input v-model.number="form.num" type="number" min="1" />
        </MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.submit') }}</button>
      </form>
      <p v-if="msg">
        {{ msg }}
        <NuxtLink v-if="needPay" :to="payTo" class="cblue">{{ $t('common_01946') }}</NuxtLink>
      </p>
    </div>
  </div>
</template>
