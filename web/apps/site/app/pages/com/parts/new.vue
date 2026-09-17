<script setup lang="ts">
import { ApiError } from '~/utils/envelope'

const morning = ['0101', '0201', '0301', '0401', '0501', '0601', '0701']
const noon = ['0102', '0202', '0302', '0402', '0502', '0602', '0702']
const afternoon = ['0103', '0203', '0303', '0403', '0503', '0603', '0703']
const dayKeys = ['wap_com_00338', 'wap_com_00339', 'wap_js_00029', 'wap_js_00032', 'wap_js_00030', 'wap_js_00031', 'wap_js_00033']

const api = useApi()
const { t } = useI18n()
const router = useRouter()
const editId = computed(() => Number(useRoute().query.id || 0))
const { data: partCats } = await usePartCats()
const roots = computed(() => (partCats.value || []).filter((c) => !c.parent_id).sort((a, b) => a.id - b.id))
function childrenOf(rootIdx: number) {
  const root = roots.value[rootIdx]
  if (!root) return []
  return (partCats.value || []).filter((c) => Number(c.parent_id) === root.id)
}
const typeItems = computed(() => childrenOf(0))
const salaryTypeItems = computed(() => childrenOf(1))
const cycleItems = computed(() => childrenOf(2))
const sexItems = computed(() => childrenOf(3))

const form = reactive({
  name: '',
  type: 0,
  provinceid: 0,
  cityid: 0,
  three_cityid: 0,
  address: '',
  salary: 0,
  salary_type: 0,
  billing_cycle: 0,
  number: 1,
  sex: 0,
  linkman: '',
  linktel: '',
  content: '',
  x: '',
  y: '',
  sdate: 0,
  edate: 0,
  deadline: 0,
  worktime: '',
})
const worktimes = ref<string[]>([])
const longTerm = ref(true)
const sdateN = ref('')
const edateN = ref('')
const deadlineN = ref('')
const msg = ref('')

function toDateInput(ts: number) {
  if (!ts) return ''
  const d = new Date(ts * 1000)
  const m = `${d.getMonth() + 1}`.padStart(2, '0')
  const day = `${d.getDate()}`.padStart(2, '0')
  return `${d.getFullYear()}-${m}-${day}`
}
function fromDateInput(s: string) {
  if (!s) return 0
  const t = Date.parse(`${s}T00:00:00`)
  return Number.isFinite(t) ? Math.floor(t / 1000) : 0
}

if (editId.value) {
  const row = await api
    .post<{
      name?: string
      part_type?: number
      type?: number
      province_id?: number
      city_id?: number
      three_city_id?: number
      address?: string
      salary?: number
      salary_type?: number
      billing_cycle?: number
      number?: number
      sex?: number
      linkman?: string
      linktel?: string
      content?: string
      x?: string
      y?: string
      sdate?: number
      edate?: number
      deadline?: number
      worktime?: string
    }>('/v1/mcenter/com-parts/detail', { id: editId.value })
    .catch(() => null)
  if (row) {
    form.name = String(row.name || '')
    form.type = Number(row.part_type || row.type || 0)
    form.provinceid = Number(row.province_id || 0)
    form.cityid = Number(row.city_id || 0)
    form.three_cityid = Number(row.three_city_id || 0)
    form.address = String(row.address || '')
    form.salary = Number(row.salary || 0)
    form.salary_type = Number(row.salary_type || 0)
    form.billing_cycle = Number(row.billing_cycle || 0)
    form.number = Number(row.number || 1)
    form.sex = Number(row.sex || 0)
    form.linkman = String(row.linkman || '')
    form.linktel = String(row.linktel || '')
    form.content = String(row.content || '')
    form.x = String(row.x || '')
    form.y = String(row.y || '')
    form.sdate = Number(row.sdate || 0)
    form.edate = Number(row.edate || 0)
    form.deadline = Number(row.deadline || 0)
    worktimes.value = String(row.worktime || '')
      .split(',')
      .map((s) => s.trim())
      .filter(Boolean)
    longTerm.value = !form.edate
    sdateN.value = toDateInput(form.sdate)
    edateN.value = toDateInput(form.edate)
    deadlineN.value = toDateInput(form.deadline)
  }
}

function toggleTime(v: string) {
  if (worktimes.value.includes(v)) worktimes.value = worktimes.value.filter((x) => x !== v)
  else worktimes.value = [...worktimes.value, v]
}
function toggleAll(on: boolean) {
  worktimes.value = on ? [...morning, ...noon, ...afternoon] : []
}

async function save() {
  msg.value = ''
  form.sdate = fromDateInput(sdateN.value)
  form.edate = longTerm.value ? 0 : fromDateInput(edateN.value)
  form.deadline = fromDateInput(deadlineN.value)
  form.worktime = worktimes.value.join(',')
  if (!form.name.trim() || form.salary <= 0) {
    msg.value = t('ui.failed')
    return
  }
  try {
    if (editId.value) await api.post('/v1/mcenter/com-parts/update', { id: editId.value, ...form })
    else await api.post('/v1/mcenter/com-parts/create', { ...form })
    await router.push('/com/parts')
  } catch (e: unknown) {
    msg.value = e instanceof ApiError ? e.message : e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('member_com_00480') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00480')">
    <form class="com_release_box site-pc" @submit.prevent="save">
      <ul>
        <MemberReleaseRow :label="$t('wap_com_00288')" required>
          <input v-model="form.name" required class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00311')">
          <select v-model.number="form.type">
            <option :value="0">{{ $t('wap_com_00311') }}</option>
            <option v-for="c in typeItems" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.headcount')">
          <input v-model.number="form.number" type="number" min="1" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00328')">
          <table class="tjob_timetable">
            <tr>
              <th />
              <th v-for="k in dayKeys" :key="k">{{ $t(k) }}</th>
            </tr>
            <tr>
              <th>{{ $t('wap_com_00336') }}</th>
              <td v-for="v in morning" :key="v">
                <input type="checkbox" :checked="worktimes.includes(v)" @change="toggleTime(v)" />
              </td>
            </tr>
            <tr>
              <th>{{ $t('wap_com_00337') }}</th>
              <td v-for="v in noon" :key="v">
                <input type="checkbox" :checked="worktimes.includes(v)" @change="toggleTime(v)" />
              </td>
            </tr>
            <tr>
              <th>{{ $t('wap_com_00340') }}</th>
              <td v-for="v in afternoon" :key="v">
                <input type="checkbox" :checked="worktimes.includes(v)" @change="toggleTime(v)" />
              </td>
            </tr>
            <tr>
              <td colspan="8">
                <label>
                  <input
                    type="checkbox"
                    :checked="worktimes.length === 21"
                    @change="toggleAll(($event.target as HTMLInputElement).checked)"
                  />
                  {{ $t('wap_js_00074') }}
                </label>
              </td>
            </tr>
          </table>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_js_00135')">
          <input v-model="sdateN" type="date" />
          <label>
            <input v-model="longTerm" type="checkbox" />
            {{ $t('wap_js_00135') }}
          </label>
          <input v-if="!longTerm" v-model="edateN" type="date" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00234')">
          <input v-model="deadlineN" type="date" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_user_00106')" required>
          <input v-model.number="form.salary" type="number" min="1" required />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00290')">
          <select v-model.number="form.salary_type">
            <option :value="0">{{ $t('common.all') }}</option>
            <option v-for="c in salaryTypeItems" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_user_00199')">
          <select v-model.number="form.billing_cycle">
            <option :value="0">{{ $t('member_user_00199') }}</option>
            <option v-for="c in cycleItems" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00303')">
          <select v-model.number="form.sex">
            <option :value="0">{{ $t('common.not_limited') }}</option>
            <option v-for="c in sexItems" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_user_00198')">
          <LocationFields
            v-model:province-id="form.provinceid"
            v-model:city-id="form.cityid"
            v-model:district-id="form.three_cityid"
          />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_00040')"><input v-model="form.address" class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_00317')">
          <MapPick v-model:x="form.x" v-model:y="form.y" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_01431')"><input v-model="form.linkman" class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('common.phone')"><input v-model="form.linktel" class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.job_desc')" area>
          <RichEditor v-model="form.content" />
        </MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('common.save') }}</button>
    </form>
    <div class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="save">
        <MemberField wap :label="$t('wap_com_00288')"><input v-model="form.name" required /></MemberField>
        <MemberField wap :label="$t('wap_com_00311')">
          <select v-model.number="form.type">
            <option :value="0">{{ $t('wap_com_00311') }}</option>
            <option v-for="c in typeItems" :key="'h5t-' + c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('ui.headcount')"><input v-model.number="form.number" type="number" min="1" /></MemberField>
        <MemberField wap :label="$t('wap_com_00328')">
          <div v-for="(v, i) in morning" :key="'h5m-' + v">
            <label><input type="checkbox" :checked="worktimes.includes(v)" @change="toggleTime(v)" /> {{ $t(dayKeys[i] || '') }} {{ $t('wap_com_00336') }}</label>
            <label><input type="checkbox" :checked="worktimes.includes(noon[i] || '')" @change="toggleTime(noon[i] || '')" /> {{ $t('wap_com_00337') }}</label>
            <label><input type="checkbox" :checked="worktimes.includes(afternoon[i] || '')" @change="toggleTime(afternoon[i] || '')" /> {{ $t('wap_com_00340') }}</label>
          </div>
        </MemberField>
        <MemberField wap :label="$t('wap_js_00135')"><input v-model="sdateN" type="date" /></MemberField>
        <MemberField wap :label="$t('wap_js_00135')">
          <input v-model="longTerm" type="checkbox" />
        </MemberField>
        <MemberField v-if="!longTerm" wap :label="$t('wap_js_00135')"><input v-model="edateN" type="date" /></MemberField>
        <MemberField wap :label="$t('wap_com_00234')"><input v-model="deadlineN" type="date" /></MemberField>
        <MemberField wap :label="$t('member_user_00106')"><input v-model.number="form.salary" type="number" min="1" required /></MemberField>
        <MemberField wap :label="$t('wap_com_00290')">
          <select v-model.number="form.salary_type">
            <option :value="0">{{ $t('common.all') }}</option>
            <option v-for="c in salaryTypeItems" :key="'h5st-' + c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('member_user_00199')">
          <select v-model.number="form.billing_cycle">
            <option :value="0">{{ $t('member_user_00199') }}</option>
            <option v-for="c in cycleItems" :key="'h5c-' + c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('wap_com_00303')">
          <select v-model.number="form.sex">
            <option :value="0">{{ $t('common.not_limited') }}</option>
            <option v-for="c in sexItems" :key="'h5s-' + c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('member_user_00198')">
          <LocationFields
            v-model:province-id="form.provinceid"
            v-model:city-id="form.cityid"
            v-model:district-id="form.three_cityid"
          />
        </MemberField>
        <MemberField wap :label="$t('wap_00040')"><input v-model="form.address" /></MemberField>
        <MemberField wap :label="$t('wap_00317')">
          <MapPick v-model:x="form.x" v-model:y="form.y" />
        </MemberField>
        <MemberField wap :label="$t('wap_01431')"><input v-model="form.linkman" /></MemberField>
        <MemberField wap :label="$t('common.phone')"><input v-model="form.linktel" /></MemberField>
        <MemberField wap area :label="$t('ui.job_desc')">
          <RichEditor v-model="form.content" />
        </MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.save') }}</button>
      </form>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
