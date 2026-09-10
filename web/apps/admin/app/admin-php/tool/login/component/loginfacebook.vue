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
                                        v-model="configdata.sy_facebooklogin"
                                        inline-prompt
                                        :width="52"
                                        :active-value="'1'"
                                        :inactive-value="'0'"
                                        :active-text="lc('member_com_00287')"
                                        :inactive-text="lc('resume_00030')"
                                    />
                                    <span class="login-switch-state" :class="configdata.sy_facebooklogin === '1' ? 'is-on' : 'is-off'">
                                        {{ configdata.sy_facebooklogin === '1' ? lc('admin_tool_00694') : lc('admin_tool_00695') }}
                                    </span>
                                </div>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_tool_00693') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_tool_00691') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="configdata.sy_facebookappid">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>123456789012345</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_tool_00692') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="configdata.sy_facebookappkey" show-password>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span></span>
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
const lc = (...a) => window.lc(...a)

export default {
    props: {
        config: {
            type: Object,
            default: {}
        },
    },
    watch: {
        config: {
            handler(val) {
                this.configdata = val || {}
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
    methods: {
        post: function() {
            this.$emit('post-set', { type: 'facebook', config: this.configdata })
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
