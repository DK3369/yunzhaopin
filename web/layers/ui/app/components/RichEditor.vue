<script setup lang="ts">
const props = defineProps<{ modelValue: string; placeholder?: string }>()
const emit = defineEmits<{ 'update:modelValue': [string] }>()

const Toolbar = shallowRef<unknown>(null)
const Editor = shallowRef<unknown>(null)
const editorRef = shallowRef<{ destroy: () => void } | null>(null)
const ready = ref(false)
const html = computed({
  get: () => props.modelValue,
  set: (v: string) => emit('update:modelValue', v),
})

const editorConfig = computed(() => ({
  placeholder: props.placeholder || '',
  MENU_CONF: {
    uploadImage: {
      async customUpload(file: File, insertFn: (url: string, alt: string, href: string) => void) {
        const r = await $fetch<{ key: string; url: string }>('/api/upload/content', {
          method: 'POST',
          body: file,
          headers: { 'content-type': file.type || 'image/jpeg' },
        })
        const url = r.url || r.key
        insertFn(url, '', url)
      },
    },
  },
}))

onMounted(async () => {
  if (!import.meta.client) return
  await import('@wangeditor/editor/dist/css/style.css')
  const mod = await import('@wangeditor/editor-for-vue')
  Toolbar.value = mod.Toolbar
  Editor.value = mod.Editor
  ready.value = true
})

function handleCreated(editor: { destroy: () => void }) {
  editorRef.value = editor
}

onBeforeUnmount(() => {
  editorRef.value?.destroy()
  editorRef.value = null
})
</script>

<template>
  <ClientOnly>
    <div v-if="ready" class="rich-editor">
      <component :is="Toolbar" :editor="editorRef" mode="default" />
      <component
        :is="Editor"
        v-model="html"
        :default-config="editorConfig"
        mode="default"
        style="height: 280px; overflow-y: hidden"
        @on-created="handleCreated"
      />
    </div>
    <template #fallback>
      <textarea v-model="html" rows="8" class="com_release_textnew_text" />
    </template>
  </ClientOnly>
</template>

<style scoped>
.rich-editor {
  border: 1px solid #ccc;
  z-index: 10;
  width: 100%;
}
</style>
