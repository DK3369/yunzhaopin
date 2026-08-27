<script setup lang="ts">
import type { ApiEnvelope } from '~/utils/envelope'
import { lc, readStoredLocale, setAdminLocale } from '~/utils/phpLc'

definePageMeta({ layout: 'blank' })
const { setLocale } = useI18n()
const username = ref('')
const password = ref('')
const err = ref('')
const showDiv1 = ref(true)
const islook = ref(true)
const uiLang = ref(readStoredLocale())
type LoginData = { uid: number; usertype: number; path?: string; name?: string }

async function switchLang(next: 'zh' | 'en') {
  if (next === uiLang.value) return
  await setAdminLocale(next)
  await setLocale(next)
  uiLang.value = next
  location.reload()
}

async function login() {
  err.value = ''
  const body = await $fetch<ApiEnvelope<LoginData>>(bffUrl('/api/auth/admin-login'), {
    method: 'POST',
    credentials: 'include',
    body: { username: username.value, password: password.value },
  })
  if (body.code !== 200 || !body.data || typeof body.data !== 'object') {
    err.value = body.msg || lc('ui.login_failed', null, '登录失败')
    return
  }
  const me = body.data
  if (me.usertype !== 3) {
    err.value = lc('ui.need_admin', null, '需要管理员账号')
    await $fetch(bffUrl('/api/auth/logout'), { method: 'POST', credentials: 'include' })
    return
  }
  const path = me.path && me.path.startsWith('/') ? me.path : '/index'
  if (import.meta.client) localStorage.setItem('indexPath', path)
  await navigateTo(path)
}
function toggleDiv() {
  showDiv1.value = !showDiv1.value
}
</script>

<template>
  <div class="adminDomeAll">
    <div class="logoinLogo">
      <img src="/admin/php-admin/images/admin_new_logo.png" alt="" />
    </div>
    <div class="logoinBlock1"><img src="/admin/php-admin/images/lo_fk2.png" alt="" /></div>
    <div class="logoinBlock2"><img src="/admin/php-admin/images/lo_fk.png" alt="" /></div>
    <div class="logoinBlock3"><img src="/admin/php-admin/images/lo_fk3.png" alt="" /></div>
    <div class="loginCont">
      <div class="logoinContImg">
        <div class="logoinBacimg1"><img src="/admin/php-admin/images/lo_phpyun.png" alt="" /></div>
        <div class="logoinBacimg2"><img src="/admin/php-admin/images/lo_dp.png" alt="" /></div>
        <div class="logoinBacimg3"><img src="/admin/php-admin/images/lo_zzj.png" alt="" /></div>
        <div class="logoinBacimg4"><img src="/admin/php-admin/images/lo_sj.png" alt="" /></div>
        <div class="logoinBacimg5"><img src="/admin/php-admin/images/lo_ht.png" alt="" /></div>
        <div class="logoinBacimg6"><img src="/admin/php-admin/images/lo_sjdp.png" alt="" /></div>
        <div class="logoinBacimg7"><img src="/admin/php-admin/images/lo_znj.png" alt="" /></div>
        <div class="logoinBacimg8"><img src="/admin/php-admin/images/lo_app.png" alt="" /></div>
        <div class="logoinBacimg9"><img src="/admin/php-admin/images/lo_sj2.png" alt="" /></div>
      </div>
      <div class="loginBoxs">
        <div class="logoinRight">
          <div class="logoinName"><span>{{ lc('admin_01277') }}</span></div>
          <div class="loginIptbox" id="loginapp">
            <template v-if="islook">
              <div v-if="showDiv1" class="loginTabse">
                <ul class="logoinList">
                  <li>
                    <div class="adminLogins">
                      <input v-model="username" type="text" class="ipt" :placeholder="lc('wap_00208')" />
                    </div>
                  </li>
                  <li>
                    <div class="adminLogins adminLoginTwo">
                      <input v-model="password" type="password" class="ipt" :placeholder="lc('wap_js_00139')" @keyup.enter="login" />
                    </div>
                  </li>
                  <li>
                    <div class="adminLoginsButton">
                      <span id="submit_bt" class="adminLogiSub" @click="login">{{ $t('common.login') }}</span>
                    </div>
                  </li>
                </ul>
                <p v-if="err" style="color: #e34848; text-align: center">{{ err }}</p>
                <p style="text-align: center; margin-top: 12px; font-size: 13px">
                  <a href="javascript:;" :style="{ fontWeight: uiLang === 'zh' ? '700' : '400' }" @click="switchLang('zh')">中文</a>
                  <span> | </span>
                  <a href="javascript:;" :style="{ fontWeight: uiLang === 'en' ? '700' : '400' }" @click="switchLang('en')">English</a>
                </p>
                <div class="weixnLogins">
                  <div @click="toggleDiv">
                    <img src="/admin/php-admin/images/chyasw.png" alt="" />
                    <span>{{ lc('admin_01278') }}</span>
                  </div>
                </div>
              </div>
              <div v-else class="loginTabse">
                <div class="weixinsapoyis">
                  <div class="weixinsapTxte"><span>{{ lc('default_00060') }}</span></div>
                </div>
                <div class="weixnLogins">
                  <div @click="toggleDiv">
                    <img src="/admin/php-admin/images/zhanghao.png" alt="" />
                    <span>{{ lc('admin_01279') }}</span>
                  </div>
                </div>
              </div>
            </template>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
