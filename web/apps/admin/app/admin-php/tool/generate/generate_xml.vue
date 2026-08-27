<template>
<div id="moduapp" class="tableDome">
    <div class="tableDome_tip">
        <el-alert :title="lc('admin_tool_00417')" type="success"
            :closable="false">
        </el-alert>
    </div>
    <div class="moduleTable">
        <table class="tableVue">
            <thead>
            <tr align="left">
                <th width="100">{{ lc('member_com_00021') }}</th>
                <th width="260">{{ lc('admin_system_00465') }}</th>
                <th width=" ">{{ lc('member_com_00207') }}</th>
            </tr>
            </thead>
            <tbody>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_tool_00430') }}</div>
                </td>
                <td>
                    <div class="TableSelect">
                        <el-select v-model="ruleForm.type" :placeholder="lc('wap_user_00100')">
                            <el-option value="sitemap" :label="lc('wap_js_00075')"></el-option>
                            <el-option value="company" :label="lc('wap_user_00153')"></el-option>
                            <el-option value="job" :label="lc('wap_user_00154')"></el-option>
                            <el-option value="resume" :label="lc('wap_com_00428')"></el-option>
                            <el-option value="ask" :label="lc('wap_user_00223')"></el-option>
                            <el-option value="news" :label="lc('admin_tool_00428')"></el-option>
                        </el-select>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span></span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_tool_00431') }}</div>
                </td>
                <td>
                    <div class="TableSelect">
                        <el-select v-model="ruleForm.order" :placeholder="lc('wap_user_00100')">
                            <el-option value="uptime" :label="lc('wap_00326')"></el-option>
                            <el-option value="addtime" :label="lc('member_com_00300')"></el-option>
                        </el-select>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span></span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_tool_00423') }}</div>
                </td>
                <td>
                    <div class="TableSelect">
                        <el-select v-model="ruleForm.frequency" :placeholder="lc('wap_user_00100')">
                            <el-option value="always" :label="lc('admin_tool_00427')"></el-option>
                            <el-option value="hourly" :label="lc('admin_tool_00425')"></el-option>
                            <el-option value="daily" :label="lc('admin_system_00269')"></el-option>
                            <el-option value="weekly" :label="lc('admin_system_00268')"></el-option>
                            <el-option value="monthly" :label="lc('admin_system_00270')"></el-option>
                            <el-option value="yearly" :label="lc('admin_tool_00429')"></el-option>
                            <el-option value="never" :label="lc('admin_tool_00426')"></el-option>
                        </el-select>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span></span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_tool_00424') }}</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input v-model="ruleForm.limit" @input="inputIntNumber($event, 'ruleForm', 'limit')" :placeholder="lc('admin_user_00342')"></el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_tool_00419') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_tool_00422') }}</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input v-model="ruleForm.name" :placeholder="lc('admin_tool_00433')"></el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_tool_00418') }}</span>
                    </div>
                </td>
            </tr>
            </tbody>
        </table>
    </div>
    <div class="setBasicButn" style="border: none;">
        <el-button type="primary" size="medium" @click="submitForm">{{ lc('wap_user_00176') }}</el-button>
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
                submitLoading: false,
                ruleForm: {
                    type: 'sitemap',
                    order: 'uptime',
                    frequency: 'always',
                    limit: '100',
                    name: 'sitemap',
                },
            }
        },
        methods: {
            inputIntNumber(val, form, key) {
                this.$data[form][key] = val.replace(/[^0-9]/g, '');
                // console.log(this[form])
                // this.$set(this[form], key, val.replace(/[^0-9]/g,''))
            },
            submitForm() {
                let _this = this;
                let params = JSON.parse(JSON.stringify(this.ruleForm));
                if (params.limit < 1 || params.limit == '') {
                    message.error(lc('admin_tool_00420'));
                    return false;
                }
                if (!params.name.length) {
                    message.error(lc('admin_tool_00433'));
                    return false;
                }
                if (_this.submitLoading) {
                    return false;
                }
                _this.submitLoading = true;
                message.success(lc('admin_tool_00434'));
                httpPost('m=tool&c=generate_xml&a=archive', params).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg);
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                }).finally(function () {
                    setTimeout(() => {
                        _this.submitLoading = false;
                    }, 1000);
                });
            }
        }
    }
</script>
