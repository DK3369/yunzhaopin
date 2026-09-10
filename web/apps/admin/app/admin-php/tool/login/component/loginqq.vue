<template>
    <div class="moduleElHight">
        <div class="moduleTable" style="height: max-content; max-height: calc(100% - 80px);">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="200">{{ lc('member_com_00021') }}</th>
                        <th width="500">{{ lc('member_user_00181') }}</th>
                        <th>{{ lc('member_com_00207') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_tool_00437') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <div class="login-switch">
                                    <el-switch
                                        v-model="configdata.sy_qqlogin"
                                        inline-prompt
                                        :width="52"
                                        :active-value="'1'"
                                        :inactive-value="'0'"
                                        :active-text="lc('member_com_00287')"
                                        :inactive-text="lc('resume_00030')"
                                    />
                                    <span class="login-switch-state" :class="configdata.sy_qqlogin === '1' ? 'is-on' : 'is-off'">
                                        {{ configdata.sy_qqlogin === '1' ? lc('admin_tool_00694') : lc('admin_tool_00695') }}
                                    </span>
                                </div>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span> </span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">appid</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="configdata.sy_qqappid">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>1002478xx</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">appkey</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="configdata.sy_qqappkey">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>4dd1c30d472676914f2fbfbnjt33</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_tool_00438') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <div class="login-switch">
                                    <el-switch
                                        v-model="configdata.sy_qqdt"
                                        inline-prompt
                                        :width="52"
                                        :active-value="'1'"
                                        :inactive-value="'0'"
                                        :active-text="lc('common_02085')"
                                        :inactive-text="lc('common_02063')"
                                    />
                                    <span class="login-switch-state" :class="configdata.sy_qqdt === '1' ? 'is-on' : 'is-off'">
                                        {{ configdata.sy_qqdt === '1' ? lc('admin_tool_00694') : lc('admin_tool_00695') }}
                                    </span>
                                </div>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_tool_00436') }}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="post">{{ lc('common.submit') }}</el-button>
        </div>
    </div>
</template>
<script>
const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }
const delConfirm = (...a) => window.delConfirm(...a)
const formatDate = (...a) => window.formatDate(...a)
const formatMonth = (...a) => window.formatMonth(...a)
const formatDatetime = (...a) => window.formatDatetime(...a)
const deepClone = (...a) => window.deepClone(...a)
const scrollToTop = (...a) => window.scrollToTop(...a)
const isEmpty = (...a) => window.isEmpty(...a)
const isArray = (...a) => window.isArray(...a)
const $ = typeof window !== 'undefined' && window.$ ? window.$ : Object.assign(function(){ return { length: 0 } }, {})
const echarts = typeof window !== 'undefined' && window.echarts ? window.echarts : { init(){ return { setOption(){}, resize(){} } }, graphic: { LinearGradient: function(){} } }

export default {
    props: {
        config: {
            type: Object,
            default: {}
        },
    },
    watch: {
        config: {
            handler(val, oldVal) {
                this.configdata = val;
            },
            immediate: true,
            deep: true,
        }
    },
    data: function() {
        return {
            configdata: {}

        }
    },

    mounted() {

    },
    methods: {
        post: function() {
            this.$emit('post-set', { type: 'qq', config: this.configdata })
        }
    },
};
</script>
<style scoped>
.login-switch { display: flex; align-items: center; gap: 12px; }
.login-switch-state { font-size: 14px; font-weight: 600; }
.login-switch-state.is-on { color: #67c23a; }
.login-switch-state.is-off { color: #909399; }
</style>