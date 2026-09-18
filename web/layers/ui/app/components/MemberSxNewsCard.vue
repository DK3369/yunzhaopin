<template>
  <div class="m_cardbg">
    <div v-if="kicker" class="sx_new_tit">{{ kicker }}</div>
    <div class="sx_new_cont"><template v-if="segs.length"><template v-for="(p, i) in segs" :key="i"><NuxtLink v-if="p.to" :to="p.to" class="sys_a" @click.stop>{{ p.n }}</NuxtLink><span v-else>{{ p.n }}</span></template></template><NuxtLink v-else-if="to" :to="to" class="sys_a">{{ title }}</NuxtLink><template v-else>{{ title }}</template><slot /></div>
    <div class="sx_new_bot">
      <div class="sx_new_data">{{ meta }}</div>
      <div v-if="onDelete" class="sx_new_icon" @click.stop="onDelete">
        <img src="/legacy/h5/images/resume_del.png" alt="" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  title?: string
  parts?: Array<{ n?: string; to?: string | null }>
  kicker?: string
  time?: string
  sub?: string
  to?: string
  onDelete?: () => void
}>()

const segs = computed(() =>
  (props.parts || [])
    .map((p) => ({ n: String(p.n || ''), to: p.to || '' }))
    .filter((p) => p.n),
)
const meta = computed(() => [props.sub, props.time].filter(Boolean).join(' · '))
</script>
