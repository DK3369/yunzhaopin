<script setup lang="ts">
import { mediaUrl, PLACEHOLDER_LOGO } from '../utils/site'
import type { AdBanner } from '../composables/useAdsBundle'

type Profile = {
  uid?: number
  name?: string
  logo?: string
  hits?: number
  yyzz_status?: number
}
type Vip = {
  rating_name?: string
}

const props = defineProps<{
  profile?: Profile | null
  vip?: Vip | null
  vipLive: boolean
  vipOpened: boolean
  vipRange: string
  integral: number
  priceName: string
  posterOn: boolean
  gzhNeed: boolean
  wxQr?: string
  webtel: string
  webmoblie: string
  guwenPic?: string
  kfqq: string
  ads: AdBanner[]
  yearOn: boolean
  msg: string
}>()

defineEmits<{
  publish: []
  refresh: []
  year: []
  'kefu-qr': []
}>()

const uid = computed(() => Number(props.profile?.uid || 0))
const logo = computed(() => mediaUrl(props.profile?.logo, PLACEHOLDER_LOGO))
const certOk = computed(() => Number(props.profile?.yyzz_status) === 1)
const hasName = computed(() => Boolean(String(props.profile?.name || '').trim()))
const kefuImg = computed(() => mediaUrl(props.guwenPic) || '')
</script>

<template>
  <div class="memberSubLeft site-pc">
    <div class="membSubLeComs">
      <div class="membLeComData">
        <div class="membLeComLogo">
          <NuxtLink to="/com/profile">
            <img :src="logo" alt="" />
          </NuxtLink>
        </div>
        <div class="membLeComNam">
          <template v-if="hasName">
            <div class="membLeComText">
              <span>{{ profile?.name }}</span>
            </div>
            <div class="membLeComslink">
              <NuxtLink to="/com/profile">
                <img src="/legacy/member/com/qiye2.png" alt="" />
                <span>{{ $t('common_02150') }}></span>
              </NuxtLink>
            </div>
          </template>
          <div v-else class="membLeComslink">
            <NuxtLink to="/com/profile">
              <img src="/legacy/member/com/qiye2.png" alt="" />
              <span style="color: #f60">{{ $t('member_com_00158') }}></span>
            </NuxtLink>
          </div>
        </div>
      </div>
      <div class="com_n_data">
        <div class="com_n_data_a">
          <div class="com_n_data_n">{{ profile?.hits ?? 0 }}</div>
          {{ $t('wap_com_00112') }}
        </div>
        <div class="com_n_data_a">
          <div class="com_n_data_n">{{ integral }}</div>
          {{ priceName }}
        </div>
      </div>
      <div class="new_com_vip">
        <div class="new_com_vip_t">
          <div class="new_com_vip_icon" />
          <div class="new_com_vip_name">{{ vip?.rating_name || $t('wap_com_00087') }}</div>
          <div>{{ vipOpened ? vipRange : $t('wap_com_00083') }}</div>
        </div>
        <div class="new_vip">
          <div class="new_vip_a">
            <NuxtLink to="/com/orders">{{ $t('wap_com_00064') }}</NuxtLink>
          </div>
          <div class="new_vip_a">
            <NuxtLink to="/com/integral">{{ priceName }}{{ $t('wap_com_00304') }}</NuxtLink>
          </div>
          <div class="new_vip_a">
            <NuxtLink to="/com/pay">{{ $t('common_01946') }}</NuxtLink>
          </div>
        </div>
        <NuxtLink to="/com/member-right" class="new_com_vip_bth">{{ $t('wap_com_00066') }}</NuxtLink>
        <NuxtLink v-if="vipLive" to="/com/member-right" class="new_com_vip_bth">{{ $t('member_com_00145') }}</NuxtLink>
      </div>
      <div class="con_new_mininav">
        <div class="con_new_mininav_a">
          <NuxtLink to="/com/stats">
            <img src="/legacy/member/com/nav1.png" alt="" />{{ $t('wap_com_00103') }}
          </NuxtLink>
        </div>
        <div class="con_new_mininav_a">
          <NuxtLink to="/com/cert">
            <img src="/legacy/member/com/nav2.png" alt="" />{{ $t('member_com_00144') }}
          </NuxtLink>
        </div>
        <div class="con_new_mininav_a">
          <NuxtLink to="/com/binding">
            <img src="/legacy/member/com/nav3.png" alt="" />{{ $t('member_user_00059') }}
          </NuxtLink>
        </div>
      </div>
      <div class="con_new_mininav">
        <div class="con_new_mininav_a">
          <NuxtLink to="/com/views">
            <img src="/legacy/member/com/nav4.png" alt="" />{{ $t('member_com_00151') }}
          </NuxtLink>
        </div>
        <div v-if="posterOn && uid" class="con_new_mininav_a">
          <NuxtLink :to="`/poster/company/${uid}`">
            <img src="/legacy/member/com/nav5.png" alt="" />{{ $t('member_com_00149') }}
          </NuxtLink>
        </div>
        <div class="con_new_mininav_a">
          <NuxtLink v-if="uid" :to="`/companies/${uid}`" target="_blank">
            <img src="/legacy/member/com/nav6.png" alt="" />{{ $t('member_com_00143') }}
          </NuxtLink>
        </div>
      </div>
      <div class="membLeCoNavs">
        <div class="fbjob">
          <a href="javascript:;" @click.prevent="$emit('publish')">
            <img src="/legacy/member/com/fbi.png" alt="" />
            <span>{{ $t('common.publish_job') }}</span>
          </a>
        </div>
        <div class="sxjob">
          <a href="javascript:;" @click.prevent="$emit('refresh')">
            <img src="/legacy/member/com/sxi.png" alt="" />
            <span>{{ $t('wap_com_00029') }}</span>
          </a>
        </div>
      </div>
    </div>
    <div v-if="ads.length" class="yun_combanner">
      <a v-for="(ad, i) in ads" :key="'511-' + i" :href="ad.link || undefined" :target="ad.link ? '_blank' : undefined">
        <img v-if="ad.image_n || ad.image" :src="ad.image_n || ad.image" :alt="ad.title || ''" />
      </a>
    </div>
    <div v-if="yearOn" class="comIndexButn" @click="$emit('year')">
      <span>{{ $t('admin_yunying_00167') }}</span>
    </div>
    <div class="membSubLeKrfu">
      <div class="membLeKfuBand">
        <div v-if="gzhNeed" class="membLeKfuBaGren membLeKfusuz">
          <div class="membLeKfuBaIcon">
            <img src="/legacy/member/com/wechat1.png" alt="" />
            <span>{{ $t('wap_user_00181') }}</span>
          </div>
          <div class="membLeKfuBaLink">
            <span>{{ $t('member_com_00132') }}></span>
            <img v-if="wxQr" :src="wxQr" alt="" width="80" height="80" />
          </div>
        </div>
        <div v-if="!certOk" class="membLeKfuBaBlue membLeKfusuz">
          <div class="membLeKfuBaIcon">
            <img src="/legacy/member/com/wechat2.png" alt="" />
            <span>{{ $t('wap_user_00175') }}</span>
          </div>
          <div class="membLeKfuBaLink">
            <NuxtLink to="/com/cert">{{ $t('common_02151') }}>></NuxtLink>
          </div>
        </div>
      </div>
      <div class="membLeCoTitel">
        <span>{{ $t('wap_com_00374') }}</span>
      </div>
      <div class="membLeCoTips">
        <p>
          {{ $t('wap_com_00100') }}
          <a v-if="kfqq" :href="`tencent://message/?uin=${kfqq}`" target="_blank">
            <img src="/legacy/member/com/qqimg.png" alt="" />
          </a>
        </p>
      </div>
      <div class="membLeCoUser">
        <div class="membLeCoUsData">
          <div v-if="kefuImg" class="membLeCoUsImg">
            <img :src="kefuImg" style="width: 40px; height: 40px" alt="" />
          </div>
          <div class="membLeCoUsTell">
            <span v-if="webtel">{{ $t('common.phone') }}：{{ webtel }}</span>
            <span v-if="webmoblie">{{ $t('member_user_00163') }}：{{ webmoblie }}</span>
          </div>
        </div>
        <div v-if="wxQr" class="membLeCoUsButn">
          <a href="javascript:;" @click.prevent="$emit('kefu-qr')">{{ $t('member_com_00146') }}</a>
        </div>
      </div>
      <div class="membLeCoTitel">
        <span>{{ $t('member_com_00141') }}</span>
      </div>
      <div class="membLeCofkui">
        <p>{{ $t('member_com_00119') }}</p>
        <NuxtLink to="/com/report">{{ $t('member_com_00147') }}</NuxtLink>
      </div>
    </div>
    <p v-if="msg" class="muted">{{ msg }}</p>
  </div>
</template>
