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
          </div>
        </div>
      </div>
      <div class="clear" />
      <div class="hp_foot_bt">
        <div class="hp_foot_bt_c">
          <p>
            {{ copyright || `© ${new Date().getFullYear()} ${siteName}` }}
            <i class="hp_foot_bt_cr">
              <a v-if="record" href="https://beian.miit.gov.cn" target="_blank" rel="nofollow">{{ record }}</a>
            </i>
          </p>
          <p v-if="address || email">
            {{ $t('wap_js_00082') }}：{{ address }} <span v-if="email">EMAIL：{{ email }}</span>
          </p>
          <p>Powered by PHPYun.</p>
        </div>
      </div>
    </div>
    <div class="clear" />
  </div>

  <!-- H5 底栏，对齐原版 wap/footer.htm -->
  <div class="site-h5">
    <div class="wap_footer">
      <div class="wap_footerfixd">
        <div class="wap_footerbox">
          <NuxtLink class="wap_footernav" to="/">
            <div class="wap_footericon">
              <img :src="tabIcon('home')" alt="" style="width: 100%" />
            </div>
            <div class="wap_footer_name">{{ $t('common.home') }}</div>
          </NuxtLink>
          <NuxtLink class="wap_footernav" to="/jobs">
            <div class="wap_footericon">
              <img :src="tabIcon('job')" alt="" style="width: 100%" />
            </div>
            <div class="wap_footer_name">{{ $t('common.job') }}</div>
          </NuxtLink>
          <NuxtLink class="wap_footernav" :to="me ? (me.usertype === 2 ? '/com/jobs/new' : '/user/resume') : '/login'">
            <div class="wap_footer_fb">
              <img src="/legacy/h5/images/home_icon_release_default.png" alt="" style="width: 100%" />
            </div>
            <div class="wap_footer_name">{{ me?.usertype === 2 ? $t('common.publish_job') : me ? $t('common.publish_resume') : $t('common.publish') }}</div>
          </NuxtLink>
          <NuxtLink class="wap_footernav" :to="me ? memberHome : '/login'">
            <div class="wap_footericon">
              <img :src="tabIcon('news')" alt="" style="width: 100%" />
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
const { siteName, phone, worktime, copyright, record, email, address, me, memberHome } = useSiteChrome()

function tabIcon(kind: 'home' | 'job' | 'news' | 'me') {
  const on =
    kind === 'home'
      ? route.path === '/'
      : kind === 'job'
        ? route.path.startsWith('/jobs')
        : kind === 'me'
          ? route.path.startsWith('/user') || route.path.startsWith('/com') || route.path === '/login'
          : false
  const map = {
    home: on ? 'tab_icon_home_s.png' : 'tab_icon_home_n.png',
    job: on ? 'tab_icon_position_s.png' : 'tab_icon_position_n.png',
    news: on ? 'tab_icon_news_s.png' : 'tab_icon_news_n.png',
    me: on ? 'tab_icon_me_s.png' : 'tab_icon_me_n.png',
  }
  return `/legacy/h5/images/${map[kind]}`
}
</script>
