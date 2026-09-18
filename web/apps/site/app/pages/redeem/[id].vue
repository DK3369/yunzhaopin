<script setup lang="ts">
import { ApiError } from '~/utils/envelope'
import { ensureLogin } from '~/utils/site'

const route = useRoute()
const id = Number(route.params.id)
const { t } = useI18n()
const { me } = useSiteChrome()
const api = useApi()
const { data } = await useAsyncData(`reward-${id}`, () =>
  api.get<{
    id?: number
    name?: string
    integral?: number
    remaining?: number
    content?: string
    kind?: string
  }>('/v1/wap/redeem/rewards/detail', { id }),
)
const isRefresh = computed(() => String(data.value?.kind || 'goods') === 'resume_refresh')
const isEmployer = computed(() => Number(me.value?.usertype) === 2)
const dest = ref<'self' | 'gift'>('self')
watch(
  [isRefresh, isEmployer],
  () => {
    if (isRefresh.value && isEmployer.value) dest.value = 'gift'
  },
  { immediate: true },
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
const giftName = ref('')
const peer = ref<{ uid: number; username_mask: string; usertype: number } | null>(null)
const peerMsg = ref('')
const msg = ref('')
const needPay = ref(false)
const payTo = computed(() => (Number(me.value?.usertype) === 2 ? '/com/pay' : '/user/pay'))
const rewardsTo = computed(() => (Number(me.value?.usertype) === 2 ? '/com/rewards' : '/user/rewards'))

async function lookupPeer() {
  peerMsg.value = ''
  peer.value = null
  const name = giftName.value.trim()
  if (!name) {
    peerMsg.value = t('ui.gift_username')
    return
  }
  try {
    peer.value = await api.post('/v1/mcenter/redeem/gifts/lookup', { username: name })
  } catch (e: unknown) {
    peerMsg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

async function submit() {
  msg.value = ''
  needPay.value = false
  if (!(await ensureLogin(me.value, route.fullPath))) return
  if (isRefresh.value && isEmployer.value) dest.value = 'gift'
  let toUid = 0
  if (dest.value === 'gift') {
    if (!peer.value?.uid) {
      msg.value = t('ui.gift_need_peer')
      return
    }
    toUid = peer.value.uid
  }
  try {
    await api.post('/v1/mcenter/redeem/rewards/redeem', {
      id,
      password: form.password,
      num: isRefresh.value ? 1 : form.num,
      to_uid: toUid,
      linkman: isRefresh.value ? '' : form.linkman,
      linktel: isRefresh.value ? '' : form.linktel,
      address: isRefresh.value ? '' : form.address,
      provinceid: isRefresh.value ? 0 : form.provinceid,
      cityid: isRefresh.value ? 0 : form.cityid,
      three_cityid: isRefresh.value ? 0 : form.three_cityid,
    })
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
    <p v-if="data?.integral" class="muted">
      {{ data.integral }} {{ $t('wap_user_00008') }} · {{ $t('admin_yunying_00118') }} {{ data.remaining }}
      <span v-if="isRefresh"> · {{ $t('ui.resume_refresh') }}</span>
    </p>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else-if="!data?.name" class="muted">{{ $t('wap_00611') }}</p>
    <form v-if="data?.name" class="form" @submit.prevent="submit">
      <input v-model="form.password" type="password" required :placeholder="$t('wap_01273')" />
      <div>
        <label v-if="!(isRefresh && isEmployer)">
          <input v-model="dest" type="radio" value="self" /> {{ $t('ui.gift_to_self') }}
        </label>
        <label>
          <input v-model="dest" type="radio" value="gift" /> {{ $t('ui.gift_to_other') }}
        </label>
      </div>
      <template v-if="dest === 'gift'">
        <input v-model="giftName" :placeholder="$t('ui.gift_username')" @change="lookupPeer" />
        <button type="button" @click="lookupPeer">{{ $t('common.search') }}</button>
        <p v-if="peer" class="muted">{{ peer.username_mask }} ({{ peer.uid }})</p>
        <p v-if="peerMsg" class="muted">{{ peerMsg }}</p>
      </template>
      <template v-if="!isRefresh">
        <input v-model="form.linkman" :required="dest === 'gift'" :placeholder="$t('wap_01619')" />
        <input v-model="form.linktel" :required="dest === 'gift'" />
        <LocationFields
          v-model:province-id="form.provinceid"
          v-model:city-id="form.cityid"
          v-model:district-id="form.three_cityid"
        />
        <input v-model="form.address" :required="dest === 'gift'" />
        <input v-model.number="form.num" type="number" min="1" />
      </template>
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">
      {{ msg }}
      <NuxtLink v-if="needPay" :to="payTo" class="cblue">{{ $t('common_01946') }}</NuxtLink>
      <NuxtLink v-else :to="rewardsTo" class="cblue">{{ $t('wap_user_00170') }}</NuxtLink>
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
        <MemberField wap :label="$t('ui.gift_to_other')">
          <label v-if="!(isRefresh && isEmployer)">
            <input v-model="dest" type="radio" value="self" /> {{ $t('ui.gift_to_self') }}
          </label>
          <label>
            <input v-model="dest" type="radio" value="gift" /> {{ $t('ui.gift_to_other') }}
          </label>
        </MemberField>
        <MemberField v-if="dest === 'gift'" wap :label="$t('ui.gift_username')">
          <input v-model="giftName" :placeholder="$t('ui.gift_username')" />
          <button type="button" class="issue_post_body_btn" @click="lookupPeer">{{ $t('common.search') }}</button>
          <p v-if="peer" class="muted">{{ peer.username_mask }} ({{ peer.uid }})</p>
          <p v-if="peerMsg" class="muted">{{ peerMsg }}</p>
        </MemberField>
        <template v-if="!isRefresh">
          <MemberField wap :label="$t('ui.linkman')">
            <input v-model="form.linkman" :required="dest === 'gift'" :placeholder="$t('wap_01619')" />
          </MemberField>
          <MemberField wap :label="$t('common.phone')">
            <input v-model="form.linktel" :required="dest === 'gift'" />
          </MemberField>
          <MemberField wap :label="$t('ui.map_addr')">
            <LocationFields
              v-model:province-id="form.provinceid"
              v-model:city-id="form.cityid"
              v-model:district-id="form.three_cityid"
            />
          </MemberField>
          <MemberField wap :label="$t('ui.map_addr')">
            <input v-model="form.address" :required="dest === 'gift'" />
          </MemberField>
          <MemberField wap :label="$t('ui.qty')">
            <input v-model.number="form.num" type="number" min="1" />
          </MemberField>
        </template>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.submit') }}</button>
      </form>
      <p v-if="msg">
        {{ msg }}
        <NuxtLink v-if="needPay" :to="payTo" class="cblue">{{ $t('common_01946') }}</NuxtLink>
        <NuxtLink v-else :to="rewardsTo" class="cblue">{{ $t('wap_user_00170') }}</NuxtLink>
      </p>
    </div>
  </div>
</template>
