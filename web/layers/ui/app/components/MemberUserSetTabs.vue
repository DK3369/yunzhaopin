<template>
  <div class="site-pc job_list_tit">
    <ul>
      <li :class="{ job_list_tit_cur: on('/user/ident') }">
        <NuxtLink to="/user/ident">{{ $t('wap_00456') }}</NuxtLink>
      </li>
      <li :class="{ job_list_tit_cur: on('/user/password') }">
        <NuxtLink to="/user/password">{{ $t('member_user_00058') }}</NuxtLink>
      </li>
      <li :class="{ job_list_tit_cur: on('/user/binding') }">
        <NuxtLink to="/user/binding">{{ $t('member_user_00057') }}</NuxtLink>
      </li>
      <li :class="{ job_list_tit_cur: on('/user/resume') }">
        <NuxtLink to="/user/resume">{{ $t('member_user_00053') }}</NuxtLink>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
const route = useRoute()
function on(to: string) {
  return route.path === to
}
</script>
