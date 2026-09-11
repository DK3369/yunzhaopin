<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_tool_00710')" type="success" :closable="false"></el-alert>
        </div>
        <div class="moduleTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{{ lc('member_com_00021') }}</th>
                    <th width="400">{{ lc('member_user_00181') }}</th>
                    <th>{{ lc('member_com_00207') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00697') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="form.url" placeholder="http://72.62.75.195:3000/"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00705') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00698') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="form.enabled">
                                <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                                <el-radio label="0">{{ lc('common.close') }}</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00699') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00699') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt" style="display:flex;align-items:center;gap:8px;flex-wrap:wrap;">
                            <el-input v-model="form.hours" style="width:90px;"></el-input>
                            <span>{{ lc('admin_tool_00700') }}</span>
                            <el-input v-model="form.minutes" style="width:90px;"></el-input>
                            <span>{{ lc('admin_tool_00701') }}</span>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00703') }}：{{ form.last_run_n || lc('admin_tool_00709') }}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
                <el-button type="success" size="medium" @click="runNow" :disabled="runLoading">{{ lc('admin_tool_00702') }}</el-button>
            </div>
            <div v-if="form.last_msg" class="tableDome_tip" style="margin-top:12px;">
                <el-alert :title="form.last_msg" type="info" :closable="false"></el-alert>
            </div>
            <div class="moduleTable" style="margin-top:16px;">
                <div class="TableTite" style="margin-bottom:8px;">{{ lc('admin_tool_00704') }}</div>
                <table class="tableVue">
                    <thead>
                    <tr align="left">
                        <th>{{ lc('admin_tool_00703') }}</th>
                        <th>{{ lc('admin_tool_00706') }}</th>
                        <th>{{ lc('admin_tool_00707') }}</th>
                        <th>{{ lc('admin_tool_00708') }}</th>
                        <th>{{ lc('member_com_00207') }}</th>
                    </tr>
                    </thead>
                    <tbody>
                    <tr v-for="row in logs" :key="row.id">
                        <td>{{ row.started_at_n }}</td>
                        <td>{{ row.fetched }}</td>
                        <td>{{ row.inserted }}</td>
                        <td>{{ row.skipped }}</td>
                        <td>{{ row.error || '-' }}</td>
                    </tr>
                    <tr v-if="!logs.length">
                        <td colspan="5">{{ lc('admin_tool_00709') }}</td>
                    </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</template>

<script>
const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }

export default {
    data: function () {
        return {
            saveLoading: false,
            runLoading: false,
            form: {
                url: 'http://72.62.75.195:3000/',
                enabled: '0',
                hours: 6,
                minutes: 0,
                last_run_n: '',
                last_msg: '',
            },
            logs: [],
        }
    },
    created() {
        this.load()
    },
    methods: {
        async load() {
            const res = await httpPost('m=tool&c=dataCollection&a=scrapeGet')
            if (res.data.error == 0 && res.data.data) {
                const d = res.data.data
                this.form.url = d.url || this.form.url
                this.form.enabled = String(d.enabled == null ? '0' : d.enabled)
                this.form.hours = d.hours == null ? 6 : d.hours
                this.form.minutes = d.minutes == null ? 0 : d.minutes
                this.form.last_run_n = d.last_run_n || ''
                this.form.last_msg = d.last_msg || ''
                this.logs = Array.isArray(d.logs) ? d.logs : []
            }
        },
        save() {
            const that = this
            that.saveLoading = true
            httpPost('m=tool&c=dataCollection&a=scrapeSave', {
                url: that.form.url,
                enabled: that.form.enabled,
                hours: that.form.hours,
                minutes: that.form.minutes,
            }).then(function (res) {
                if (res.data.error == 0) {
                    message.success(res.data.msg)
                    that.load()
                } else {
                    message.error(res.data.msg)
                }
            }).finally(function () {
                that.saveLoading = false
            })
        },
        runNow() {
            const that = this
            that.runLoading = true
            httpPost('m=tool&c=dataCollection&a=scrapeRun', {}, { timeout: 180000 }).then(function (res) {
                if (res.data.error == 0) {
                    const d = res.data.data || {}
                    const extra = d.inserted != null
                        ? ` ${lc('admin_tool_00707')} ${d.inserted} / ${lc('admin_tool_00708')} ${d.skipped}`
                        : ''
                    message.success((res.data.msg || '') + extra)
                    that.load()
                } else {
                    message.error(res.data.msg)
                }
            }).finally(function () {
                that.runLoading = false
            })
        },
    },
}
</script>
<style scoped>
    .moduleTable {
        max-height: none;
    }
</style>
