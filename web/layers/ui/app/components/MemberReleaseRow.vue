<template>
  <li>
    <div class="com_release_name">
      <i v-if="required" class="ff0">*</i>
      {{ label }}
    </div>
    <div ref="box" class="com_release_textnew" :class="{ 'com_release_textnew--area': area }">
      <slot />
    </div>
  </li>
</template>

<script setup lang="ts">
defineProps<{
  label: string
  required?: boolean
  area?: boolean
}>()

const box = ref<HTMLElement | null>(null)

function tagInputs() {
  box.value?.querySelectorAll('input, select, textarea').forEach((el) => {
    const type = (el as HTMLInputElement).type
    if (['file', 'checkbox', 'radio', 'hidden', 'button', 'submit'].includes(type)) return
    if (type === 'text' || !type || el.tagName === 'TEXTAREA' || el.tagName === 'SELECT' || type === 'number' || type === 'email' || type === 'password' || type === 'date' || type === 'datetime-local') {
      el.classList.add('com_release_textnew_text')
    }
  })
}

onMounted(tagInputs)
onUpdated(tagInputs)
</script>
