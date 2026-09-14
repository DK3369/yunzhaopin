<template>
  <div class="member-field">
    <div v-if="label" class="verification_formname">{{ label }}</div>
    <div ref="box" class="verification_form_code" :class="{ 'verification_form_code--area': area }">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  label?: string
  area?: boolean
}>()

const box = ref<HTMLElement | null>(null)

function tagInputs() {
  box.value?.querySelectorAll('input, select, textarea').forEach((el) => {
    const type = (el as HTMLInputElement).type
    if (['file', 'checkbox', 'radio', 'hidden', 'button', 'submit'].includes(type)) return
    el.classList.add('verification_text')
  })
}

onMounted(tagInputs)
onUpdated(tagInputs)
</script>
