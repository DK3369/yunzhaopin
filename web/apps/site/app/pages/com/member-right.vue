<script setup lang="ts">
import { formatUnixDate, isUnauthErr } from '~/utils/site'

type Current = {
  active: boolean
  package_code?: string | null
  started_at?: number | null
  expires_at?: number | null
}
type Pack = {
  id: number
  code: string
  name: string
  duration_days: number
  price_yuan: number
  desc?: unknown
}

const api = useApi()
const { t } = useI18n()

const { data: current, error } = await useAsyncData('com-vip-current', () =>
  api.post<Current>('/v1/mcenter/vip/current', {}),
)
const { data: packs } = await useAsyncData('com-vip-packages', () =>
  api.post<Pack[]>('/v1/mcenter/vip/packages', {}).catch(() => [] as Pack[]),
)

const packList = computed<Pack[]>(() => (Array.isArray(packs.value) ? packs.value : []))

function descLines(desc: unknown): string[] {
  if (!desc) return []
  if (Array.isArray(desc)) return desc.map((d) => String(d))
  if (typeof desc === 'string') return [desc]
  if (typeof desc === 'object') {
    return Object.entries(desc as Record<string, unknown>).map(([k, v]) => `${k}: ${v}`)
  }
  return []
}

useSeoMeta({ title: t('wap_com_00097') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_com_00097') }}</h1>
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <div class="job-card">
        <h2>{{ $t('wap_01229') }}</h2>
        <template v-if="current?.active">
          <p>{{ $t('wap_00025') }}: {{ current.package_code }}</p>
          <p>
            {{ $t('member_com_00315') }}:
            {{ formatUnixDate(current.started_at) }} ~ {{ formatUnixDate(current.expires_at) }}
          </p>
        </template>
        <template v-else>
          <p class="muted">{{ current?.expires_at ? $t('wap_com_00319') : $t('ui.no_data') }}</p>
          <p v-if="current?.expires_at" class="muted">
            {{ $t('wap_01394') }}{{ formatUnixDate(current.expires_at) }}
          </p>
        </template>
        <p>
          <NuxtLink to="/com/pay">{{ $t('member_com_00041') }}</NuxtLink>
        </p>
      </div>

      <h2>{{ $t('member_com_00610') }}</h2>
      <p v-if="!packList.length" class="muted">{{ $t('ui.no_data') }}</p>
      <div class="stack">
        <article v-for="p in packList" :key="p.id" class="job-card">
          <h3>{{ p.name }}</h3>
          <p>{{ $t('default_00093') }}: ¥{{ p.price_yuan }} · {{ p.duration_days }}d</p>
          <ul v-if="descLines(p.desc).length">
            <li v-for="(line, i) in descLines(p.desc)" :key="i">{{ line }}</li>
          </ul>
        </article>
      </div>
    </template>
    <p>
      <NuxtLink to="/com">{{ $t('ui.back_com') }}</NuxtLink>
    </p>
  </section>
</template>
