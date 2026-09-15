<template>
  <div class="member-field" :class="wap ? ['yun_createlist', pr ? 'yun_createlist_pr' : ''] : ''">
    <div v-if="label" :class="wap ? 'yun_create_name' : 'verification_formname'">{{ label }}</div>
    <div
      ref="box"
      :class="wap ? ['yun_create_text', area ? 'verification_form_code--area' : ''] : ['verification_form_code', area && 'verification_form_code--area']"
    >
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  label?: string
  area?: boolean
  /** H5 走 WAP `yun_createlist`；PC 仍用 verification_*（yun_* 只打进 member-user-h5） */
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
