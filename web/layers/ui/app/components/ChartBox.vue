<script setup lang="ts">
type EChartsLike = {
  setOption: (opt: Record<string, unknown>, notMerge?: boolean) => void
  resize: () => void
  dispose: () => void
}

const props = withDefaults(
  defineProps<{ option: Record<string, unknown>; height?: string }>(),
  { height: '280px' },
)

const el = ref<HTMLDivElement | null>(null)
let chart: EChartsLike | null = null
let onResize: (() => void) | null = null

async function boot() {
  if (!import.meta.client || !el.value) return
  const echarts = await import('echarts')
  chart = echarts.init(el.value) as unknown as EChartsLike
  chart.setOption(props.option, true)
  onResize = () => chart?.resize()
  window.addEventListener('resize', onResize)
}

watch(
  () => props.option,
  (opt) => {
    chart?.setOption(opt, true)
  },
  { deep: true },
)

onMounted(() => {
  void boot()
})
onBeforeUnmount(() => {
  if (onResize) window.removeEventListener('resize', onResize)
  chart?.dispose()
  chart = null
})
</script>

<template>
  <div ref="el" class="chart-box" :style="{ height: props.height, width: '100%' }" />
</template>
