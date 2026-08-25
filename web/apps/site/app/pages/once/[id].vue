<script setup lang="ts">
const id = Number(useRoute().params.id)
const api = useApi()
const { data } = await useAsyncData(`once-${id}`, () => api.get('/v1/wap/once-jobs/show', { id }))
useSeoMeta({ title: () => String(data.value?.companyname || '店铺招聘详情') })
useHead({ link: [{ rel: 'canonical', href: `/once/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.companyname || '店铺招聘不存在' }}</h1>
    <p v-if="data?.linkman_masked" class="muted">联系人 {{ data.linkman_masked }} · {{ data.linktel_masked }}</p>
    <p v-if="data?.require">{{ data.require }}</p>
    <p v-else-if="!data?.companyname" class="muted">没有这条店铺招聘，或暂时无法加载。</p>
  </article>
</template>
