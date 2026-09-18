<template>
  <div
    class="member-field yun_createlist"
    :class="[pr ? 'yun_createlist_pr' : '', area ? 'member-field--area' : '']"
  >
    <template v-if="wap">
      <div v-if="label" class="yun_create_name">{{ label }}</div>
      <div
        ref="box"
        :class="['yun_create_text', area ? 'verification_form_code--area' : '']"
      >
        <slot />
      </div>
    </template>
    <template v-else>
      <div v-if="label" class="site-pc verification_formname">{{ label }}</div>
      <div v-if="label" class="site-h5 yun_create_name">{{ label }}</div>
      <div
        ref="box"
        class="verification_form_code yun_create_text"
        :class="area ? 'verification_form_code--area' : ''"
      >
        <slot />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  label?: string
  area?: boolean
  /** H5 空简历创建条：只要 WAP `yun_createlist`。小节表单不传，PC/H5 各吃一套 class。 */
  wap?: boolean
  pr?: boolean
}>()

const box = ref<HTMLElement | null>(null)

function tagInputs() {
  if (props.wap) return
  box.value?.querySelectorAll('input, select, textarea').forEach((el) => {
    const type = (el as HTMLInputElement).type
    if (['file', 'checkbox', 'radio', 'hidden', 'button', 'submit'].includes(type)) return
    el.classList.add('verification_text')
  })
}

onMounted(tagInputs)
onUpdated(tagInputs)
</script>
