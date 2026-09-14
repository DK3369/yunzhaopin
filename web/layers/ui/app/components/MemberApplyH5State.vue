<template>
  <div class="Posted_card_state">
    <template v-for="(s, i) in steps" :key="i">
      <div :class="s.on ? 'Posted_state_underway' : 'Posted_state_without'">
        <div :class="s.on ? 'state_underway_icon' : 'state_been_icon'">
          <img
            :src="s.on ? '/legacy/h5/images/deliver_pass.png' : s.done ? '/legacy/h5/images/deliver_been.png' : '/legacy/h5/images/deliver_no_show.png'"
            alt=""
            width="100%"
            height="100%"
          />
        </div>
        <div :class="s.on ? 'state_underway_text' : 'state_without_text'">{{ s.label }}</div>
      </div>
      <div v-if="i < steps.length - 1" :class="s.done ? 'Posted_state_dot' : 'Posted_state_nodot'">· · · ·</div>
    </template>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  isBrowse?: number
  invited?: boolean
  withdrawn?: boolean
}>()
const { t } = useI18n()

type Step = { label: string; on: boolean; done: boolean }

const steps = computed<Step[]>(() => {
  const b = Number(props.isBrowse || 0)
  const L = {
    sent: t('wap_user_00357'),
    viewed: t('wap_user_00258'),
    interview: t('wap_user_00266'),
    hired: t('wap_user_00356'),
    unfit: t('wap_user_00354'),
    invite: t('wap_user_00216'),
    cancel: t('wap_user_00358'),
  }
  if (props.invited) {
    return [
      { label: L.sent, on: false, done: true },
      { label: L.viewed, on: false, done: true },
      { label: L.invite, on: true, done: true },
    ]
  }
  if (b === 1 && props.withdrawn) {
    return [
      { label: L.sent, on: false, done: true },
      { label: L.cancel, on: true, done: true },
    ]
  }
  if (b === 1) {
    return [
      { label: L.sent, on: true, done: true },
      { label: L.viewed, on: false, done: false },
      { label: L.interview, on: false, done: false },
      { label: L.hired, on: false, done: false },
    ]
  }
  if (b === 2) {
    return [
      { label: L.sent, on: false, done: true },
      { label: L.viewed, on: true, done: true },
      { label: L.interview, on: false, done: false },
      { label: L.hired, on: false, done: false },
    ]
  }
  if (b === 3) {
    return [
      { label: L.sent, on: false, done: true },
      { label: L.viewed, on: false, done: true },
      { label: L.interview, on: true, done: true },
      { label: L.hired, on: false, done: false },
    ]
  }
  if (b === 4) {
    return [
      { label: L.sent, on: false, done: true },
      { label: L.viewed, on: false, done: true },
      { label: L.unfit, on: true, done: true },
      { label: L.hired, on: false, done: false },
    ]
  }
  if (b === 7) {
    return [
      { label: L.sent, on: false, done: true },
      { label: L.viewed, on: false, done: true },
      { label: L.interview, on: false, done: true },
      { label: L.hired, on: true, done: true },
    ]
  }
  return [{ label: L.sent, on: false, done: false }]
})
</script>
