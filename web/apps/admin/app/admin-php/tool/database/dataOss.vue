<template>
<div id="ossConfigApp" class="moduleDome">
    <div class="tableDome_tip" style="margin-top: 12px;">
        <el-alert :title="lc('admin_tool_00330')" type="success" :closable="false"></el-alert>
    </div>
    <div class=" moduleTable">

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
                    <div class="TableTite">{{ lc('admin_tool_00331') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="ossConfig.sy_oss">
                            <el-radio :label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio :label="2">{{ lc('resume_00030') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_tool_00330') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_tool_00332') }}</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input v-model="ossConfig.userdomain" placeholder="https://phpyun50.oss-cn-beijing.aliyuncs.com "></el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_tool_00329') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">Access Key ID</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input v-model="ossConfig.access_id" placeholder=" "></el-input>
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
                    <div class="TableTite">Access Key Secret</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input v-model="ossConfig.access_key" placeholder=" "></el-input>
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
                    <div class="TableTite">EndPoint</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input v-model="ossConfig.endpoint" placeholder=" "></el-input>
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
                    <div class="TableTite">Bucket</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input v-model="ossConfig.bucket" placeholder=" "></el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_tool_00333') }}</span>
                    </div>
                </td>
            </tr>
            </tbody>
        </table>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="setOssConfig" :disabled="saveLoading">{{ lc('wap_user_00176') }}</el-button>
        </div>
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
        data: function () {
            return {
                ossConfig: [],
                saveLoading: false
            }
        },
        created(){
            this.getOssConfig();
        },
        methods: {
            async getOssConfig() {
                let res = await httpPost('m=tool&c=dataOss');
                if (res.data.error == 0) {

                    this.ossConfig = res.data.data;
                    this.ossConfig.sy_oss = this.ossConfig.sy_oss == '1' ? 1 : 2;
                }
            },
            setOssConfig: function () {
                let that = this;
                let params = {
                    ossConfig: 1,
                    sy_oss: that.ossConfig.sy_oss,
                    userdomain: that.ossConfig.userdomain,
                    access_id: that.ossConfig.access_id,
                    access_key: that.ossConfig.access_key,
                    endpoint: that.ossConfig.endpoint,
                    bucket: that.ossConfig.bucket
                };
                that.saveLoading = true;
                httpPost('m=tool&c=dataOss&a=setOssConfig', params).then(function (res) {
                    if (res.data.error == 0) {
                        message.success(res.data.msg);
                    } else {
                        message.error(res.data.msg);
                    }
                }).finally(function () {
                    setTimeout(function () {
                        that.saveLoading = false;
                    }, 2000);
                });
            },
        }
    }
</script>
