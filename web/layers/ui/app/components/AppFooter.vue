<template>
  <!-- PC 深色页脚，对齐原版 footer.htm -->
  <div class="site-pc">
    <div class="hp_foot fl">
      <div class="w1000">
        <div class="hp_foot_wt fl">
          <div class="hp_foot_pho fl">
            <dl>
              <dt />
              <dd>{{ $t('default_00129') }}</dd>
              <dd class="hp_foot_pho_nmb">{{ phone || '—' }}</dd>
              <dd>{{ worktime }}</dd>
            </dl>
          </div>
          <div class="hp_foot_wh fl">
            <i class="hp_foot_wh_lline" />
            <i class="hp_foot_wh_rline" />
            <template v-if="footerNav.length">
              <dl v-for="col in footerNav" :key="col.id">
                <dt>{{ col.name }}</dt>
                <dd>
                  <ul>
                    <li v-for="item in col.list" :key="item.id">
                      <NuxtLink :to="item.to" :title="item.title">{{ item.title }}</NuxtLink>
                    </li>
                  </ul>
                </dd>
              </dl>
            </template>
            <template v-else>
              <dl>
                <dt>{{ $t('common.website_nav') }}</dt>
                <dd>
                  <ul>
                    <li><NuxtLink to="/">{{ $t('common.home') }}</NuxtLink></li>
                    <li>{{ $t('common.phone') }}<template v-if="phone"> {{ phone }}</template></li>
                    <li><NuxtLink to="/announcements">{{ $t('common.site_notice') }}</NuxtLink></li>
                  </ul>
                </dd>
              </dl>
              <dl>
                <dt>{{ $t('common.search') }}</dt>
                <dd>
                  <ul>
                    <li><NuxtLink to="/jobs">{{ $t('default_00246') }}</NuxtLink></li>
                    <li><NuxtLink to="/resumes">{{ $t('common.resume') }}</NuxtLink></li>
                    <li><NuxtLink to="/companies">{{ $t('common.company') }}</NuxtLink></li>
                    <li><NuxtLink to="/fairs">{{ $t('member_com_00293') }}</NuxtLink></li>
                    <li><NuxtLink to="/articles">{{ $t('common.article') }}</NuxtLink></li>
                  </ul>
                </dd>
              </dl>
            </template>
          </div>
        </div>
        <div v-if="wxQr" class="hp_foot_wx fr">
          <dl>
            <dt><img :src="wxQr" width="105" height="105" alt="" /></dt>
            <dd>{{ $t('default_00130') }}</dd>
          </dl>
        </div>
        <div v-if="wapQr" class="hp_foot_wx fr">
          <dl>
            <dt><img :src="wapQr" width="105" height="105" alt="" /></dt>
            <dd>{{ $t('common_02413') }}</dd>
          </dl>
        </div>
      </div>
      <div class="clear" />
      <div class="hp_foot_bt">
        <div class="hp_foot_bt_c">
          <p>
            {{ copyright || `© ${new Date().getFullYear()} ${siteName}` }}
            <i class="hp_foot_bt_cr">
              <a v-if="record" href="https://beian.miit.gov.cn" target="_blank" rel="nofollow">{{ record }}</a>
              <a v-if="secord" href="https://www.beian.gov.cn" target="_blank" rel="nofollow">{{ secord }}</a>
            </i>
          </p>
          <p v-if="address || email">
            {{ $t('wap_js_00082') }}：{{ address }} <span v-if="email">EMAIL：{{ email }}</span>
          </p>
          <p v-if="perfor || hrlicense">
            <a v-if="perfor" href="/pages/jyxkz" target="_blank">{{ $t('admin_01020') }} {{ perfor }}</a>
            <a v-if="hrlicense" href="/pages/rlzy" target="_blank">{{ $t('admin_system_00321') }} {{ hrlicense }}</a>
          </p>
          <p><LangSwitch /></p>
          <p>Powered by OVSIX.</p>
        </div>
      </div>
    </div>
    <div class="clear" />
  </div>

  <!-- H5 底栏，对齐原版 wap/footer.htm -->
  <div class="site-h5">
    <p class="muted" style="text-align: center; padding: 0.16rem 0 1.2rem"><LangSwitch /></p>
    <div class="wap_footer">
      <div class="wap_footerfixd">
        <div class="wap_footerbox">
          <NuxtLink class="wap_footernav" to="/">
            <div class="wap_footericon">
              <img :src="tabIcon('home')" alt="" style="width: 100%" />
            </div>
            <div class="wap_footer_name">{{ $t('common.home') }}</div>
          </NuxtLink>
          <NuxtLink v-if="!isCompany" class="wap_footernav" to="/jobs">
            <div class="wap_footericon">
              <img :src="tabIcon('job')" alt="" style="width: 100%" />
            </div>
            <div class="wap_footer_name">{{ $t('common.job') }}</div>
          </NuxtLink>
          <NuxtLink v-else class="wap_footernav" to="/resumes">
            <div class="wap_footericon">
              <img :src="tabIcon('resume')" alt="" style="width: 100%" />
            </div>
            <div class="wap_footer_name">{{ $t('common.resume') }}</div>
          </NuxtLink>
          <NuxtLink class="wap_footernav" :to="me ? (isCompany ? '/com/jobs/new' : '/user/resume') : '/login'">
            <div class="wap_footer_fb">
              <img src="/legacy/h5/images/home_icon_release_default.png" alt="" style="width: 100%" />
            </div>
            <div class="wap_footer_name">{{ isCompany ? $t('common.publish_job') : me ? $t('common.publish_resume') : $t('common.publish') }}</div>
          </NuxtLink>
          <NuxtLink class="wap_footernav" :to="me ? messageTo : '/login'">
            <div class="wap_footericon" style="position: relative">
              <img :src="tabIcon('news')" alt="" style="width: 100%" />
              <em
                v-if="unreadTotal > 0"
                style="position: absolute; top: -4px; right: -6px; min-width: 16px; height: 16px; line-height: 16px; border-radius: 8px; background: #f33; color: #fff; font-size: 11px; text-align: center; padding: 0 4px"
              >{{ unreadTotal > 99 ? '99+' : unreadTotal }}</em>
            </div>
            <div class="wap_footer_name">{{ $t('common.message') }}</div>
          </NuxtLink>
          <NuxtLink class="wap_footernav" :to="me ? memberHome : '/login'">
            <div class="wap_footericon">
              <img :src="tabIcon('me')" alt="" style="width: 100%" />
            </div>
            <div class="wap_footer_name">{{ $t('common.mine') }}</div>
          </NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const route = useRoute()
const { siteName, phone, worktime, copyright, record, email, address, me, memberHome, footerNav, wxQr, wapQr, perfor, hrlicense, secord } = useSiteChrome()
const api = useApi()
const isCompany = computed(() => Number(me.value?.usertype) === 2)
const messageTo = computed(() => (isCompany.value ? '/com/messages' : '/user/messages'))
const { data: unread } = await useAsyncData(
  () => `footer-unread-${me.value?.uid || 0}`,
  () =>
    me.value
      ? api
          .post<{ total?: number }>('/v1/mcenter/messages/unread-summary', {})
          .catch(() => ({ total: 0 }))
      : Promise.resolve({ total: 0 }),
)
const unreadTotal = computed(() => Number(unread.value?.total || 0))

function tabIcon(kind: 'home' | 'job' | 'resume' | 'news' | 'me') {
  const on =
    kind === 'home'
      ? route.path === '/'
      : kind === 'job'
        ? route.path.startsWith('/jobs')
        : kind === 'resume'
          ? route.path.startsWith('/resumes')
          : kind === 'news'
            ? route.path.endsWith('/messages')
            : kind === 'me'
              ? route.path.startsWith('/user') || route.path.startsWith('/com') || route.path === '/login'
              : false
  const map = {
    home: on ? 'tab_icon_home_s.png' : 'tab_icon_home_n.png',
    job: on ? 'tab_icon_position_s.png' : 'tab_icon_position_n.png',
    resume: on ? 'tab_icon_jl_n.png' : 'tab_icon_jl.png',
    news: on ? 'tab_icon_news_s.png' : 'tab_icon_news_n.png',
    me: on ? 'tab_icon_me_s.png' : 'tab_icon_me_n.png',
  }
  return `/legacy/h5/images/${map[kind]}`
}
</script>
