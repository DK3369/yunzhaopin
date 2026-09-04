<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const { me } = useSiteChrome()
const api = useApi()
const { data } = await useAsyncData(`reward-${id}`, () =>
  api.get('/v1/wap/redeem/rewards/detail', { id }),
)
const form = reactive({ linkman: '', linktel: '', address: '', num: 1 })
const msg = ref('')
async function submit() {
  msg.value = ''
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  try {
    await api.post('/v1/mcenter/redeem/rewards/redeem', { id, ...form })
    msg.value = t('common.confirm')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
useSeoMeta({ title: () => String(data.value?.name || t('ui.redeem')) })
useHead({ link: [{ rel: 'canonical', href: `/redeem/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.name || $t('wap_00611') }}</h1>
    <p v-if="data?.integral" class="muted">{{ data.integral }} {{ $t('ui.integral') }} · {{ $t('ui.stock') }} {{ data.remaining }}</p>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else-if="!data?.name" class="muted">{{ $t('wap_00611') }}</p>
    <form v-if="data?.name" class="form" @submit.prevent="submit">
      <input v-model="form.linkman" required :placeholder="$t('wap_01619')" />
      <input v-model="form.linktel" required />
      <input v-model="form.address" />
      <input v-model.number="form.num" type="number" min="1" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </article>
</template>
