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
                            <div class="TableTite">{{ lc('admin_tool_00439') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-switch v-model="configdata.sy_sinalogin" :active-text="lc('member_com_00287')" :inactive-text="lc('resume_00030')" active-value="1" inactive-value="0"></el-switch>
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
                                <el-input :placeholder="lc('wap_user_00076')" v-model="configdata.sy_sinaappid">
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
                                <el-input :placeholder="lc('wap_user_00076')" v-model="configdata.sy_sinaappkey">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>4dd1c30d472676914f2fbfbnjt33</span>
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
module.exports = {
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
            this.$emit('post-set', { type: 'sina', config: this.configdata })
        }
    },
};
</script>
<style scoped>
</style>