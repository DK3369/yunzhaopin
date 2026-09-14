<template>
  <div class="user_new_tdzt">
    <span v-for="(s, i) in steps" :key="i" class="td_zt" :class="{ td_ztmy: s.my }">
      <i :class="s.kind === 'y' ? 'td_zt_y' : s.kind === 'xz' ? 'td_zt_xz' : 'td_zt_w'" />
      {{ s.label }}
      <template v-if="s.kind !== 'xz' && i < steps.length - 1">
        <i class="td_zt_q" />
        <i class="td_zt_q2" />
        <i class="td_zt_q3" />
        <i class="td_zt_q4" />
      </template>
    </span>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  isBrowse?: number
  invited?: boolean
  withdrawn?: boolean
}>()
const { t } = useI18n()

type Step = { label: string; kind: 'y' | 'w' | 'xz'; my?: boolean }

const steps = computed<Step[]>(() => {
  const b = Number(props.isBrowse || 0)
  const L = {
    sent: t('wap_user_00357'),
    viewed: t('wap_user_00258'),
    talk: t('wap_user_00359'),
    invite: t('wap_user_00216'),
    cancel: t('wap_user_00358'),
    interview: t('wap_user_00266'),
    hired: t('wap_user_00356'),
    unfit: t('wap_user_00354'),
  }
  if (props.invited) {
    return [
      { label: L.sent, kind: 'y' },
      { label: L.viewed, kind: 'y' },
      { label: L.talk, kind: 'y' },
      { label: L.invite, kind: 'xz' },
    ]
  }
  if (b === 1 && props.withdrawn) {
    return [
      { label: L.sent, kind: 'y' },
      { label: L.cancel, kind: 'xz' },
    ]
  }
  if (b === 1) {
    return [
      { label: L.sent, kind: 'xz' },
      { label: L.viewed, kind: 'w', my: true },
      { label: L.interview, kind: 'w', my: true },
      { label: L.hired, kind: 'w' },
    ]
  }
  if (b === 2) {
    return [
      { label: L.sent, kind: 'y' },
      { label: L.viewed, kind: 'xz', my: true },
      { label: L.interview, kind: 'w', my: true },
      { label: L.hired, kind: 'w' },
    ]
  }
  if (b === 3) {
    return [
      { label: L.sent, kind: 'y' },
      { label: L.viewed, kind: 'y' },
      { label: L.interview, kind: 'xz', my: true },
      { label: L.hired, kind: 'w' },
    ]
  }
  if (b === 4) {
    return [
      { label: L.sent, kind: 'y' },
      { label: L.viewed, kind: 'y' },
      { label: L.unfit, kind: 'xz', my: true },
      { label: L.hired, kind: 'w' },
    ]
  }
  if (b === 7) {
    return [
      { label: L.sent, kind: 'y' },
      { label: L.viewed, kind: 'y' },
      { label: L.interview, kind: 'y' },
      { label: L.hired, kind: 'xz', my: true },
    ]
  }
  return [{ label: L.sent, kind: 'w' }]
})
</script>
