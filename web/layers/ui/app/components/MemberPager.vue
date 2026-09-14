<template>
  <div v-if="maxPage > 1" class="diggg">
    <div class="site-pc">
      <a href="javascript:;" @click.prevent="go(page - 1)">{{ $t('default_00326') }}</a>
      <template v-for="(item, i) in pcItems" :key="i">
        <em v-if="item.kind === 'ellipsis'" style="padding: 0 10px">...</em>
        <a v-else-if="item.kind === 'cur'" href="javascript:;" class="selected" @click.prevent>{{ item.n }}</a>
        <a v-else href="javascript:;" @click.prevent="go(item.n)">{{ item.n }}</a>
      </template>
      <a href="javascript:;" @click.prevent="go(page + 1)">{{ $t('default_00327') }}</a>
      <em v-if="maxPage > 8" class="pages_b_no">
        <input v-model="jump" class="input-num" inputmode="numeric" @keydown.enter.prevent="doJump" />
        <input class="bt-confirm" type="button" :value="$t('common.confirm')" @click="doJump" />
      </em>
    </div>
    <div class="site-h5">
      <a href="javascript:;" @click.prevent="go(page - 1)">{{ $t('default_00326') }}</a>
      <select :value="page" @change="go(Number(($event.target as HTMLSelectElement).value))">
        <option v-for="n in h5Pages" :key="n" :value="n">{{ n }}</option>
      </select>
      <a href="javascript:;" @click.prevent="go(page + 1)">{{ $t('default_00327') }}</a>
      <em>{{ $t('common_06239').replace('[maxPage]', String(maxPage)) }}</em>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    page: number
    pageSize: number
    total: number
  }>(),
  { page: 1, pageSize: 20, total: 0 },
)
const emit = defineEmits<{ 'update:page': [number] }>()

const jump = ref('')
const maxPage = computed(() => Math.max(1, Math.ceil(Math.max(0, props.total) / props.pageSize)))

type PcItem = { kind: 'cur' | 'num'; n: number } | { kind: 'ellipsis' }

const pcItems = computed<PcItem[]>(() => {
  const n = maxPage.value
  const cur = Math.min(Math.max(1, props.page), n)
  const show = n > 10 ? 5 : 3
  const around = n > 10 ? 2 : 1
  const out: PcItem[] = []
  if (n > show && cur - around > 1) out.push({ kind: 'ellipsis' })
  let start = Math.max(1, cur - around)
  let end = Math.min(n, cur + around)
  if (end - start + 1 < show) {
    if (start === 1) end = Math.min(n, start + show - 1)
    else start = Math.max(1, end - show + 1)
  }
  for (let i = start; i <= end; i++) out.push({ kind: i === cur ? 'cur' : 'num', n: i })
  if (n > show && cur < n - around) out.push({ kind: 'ellipsis' })
  return out
})

const h5Pages = computed(() => {
  const n = maxPage.value
  const cur = props.page
  let start = 1
  let end = n
  if (n > 20) {
    if (cur >= 10) {
      start = cur - 10
      end = Math.min(n, cur + 10)
    } else {
      end = 20
    }
  }
  const arr: number[] = []
  for (let i = start; i <= end; i++) arr.push(i)
  return arr
})

function go(n: number) {
  const next = Math.min(maxPage.value, Math.max(1, Math.floor(n) || 1))
  if (next !== props.page) emit('update:page', next)
}
function doJump() {
  go(Number(jump.value))
}
</script>
