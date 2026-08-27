<template>
<div id="daohaapp" class="moduleElenAl">
    <div class="tableDome">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_tool_00381')" type="success"
                :closable="false">
            </el-alert>
        </div>

        <div class="moduleTable">
            <el-checkbox-group v-model="ruleForm.cache" @change="changeGroup">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="100" colspan="5">
                        <div class="tool_hc_tit"> {{ lc('admin_tool_00384') }}</div>
                    </th>
                </tr>
                </thead>
                    <tbody>
                    <template v-for="(item, index) in tableData">
                        <template v-if="index%5==0">
                            <tr></tr>
                        </template>
                        <td>
                            <el-checkbox :label="item.id">{{ item.name }}</el-checkbox>
                        </td>
                    </template>
                    </tbody>
                <tbody>
                </tbody>
            </table>
            </el-checkbox-group>
        </div>
        <div class="modulePaging">
            <div class="modulecz modulePagButn">
                <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
            </div>
            <div class="modulePagNum">
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm">{{ lc('admin_tool_00384') }}</el-button>
        </div>
        <div class="tool_hc_sm">
            <div class=""><i class="el-icon-question"></i> {{ lc('admin_tool_00385') }}</div>
            <div> {{ lc('admin_tool_00382') }}</div>
            <div> {{ lc('admin_tool_00380') }}</div>
            <div> {{ lc('admin_tool_00383') }}</div>
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
                tableData: [],
                checked: false, //全选
                isIndeterminate: false, // checkbox 的不确定状态
                submitLoading: false,
                ruleForm: {
                    madeall: '1',
                    cache: [],
                },
            }
        },
        created() {
            this.getBaseData();
        },
        methods: {
            selectAllBottom(value) {
                let _this = this;
                if (value) {
                    this.checked = true;
                    this.isIndeterminate = false;
                    this.ruleForm.cache = [];
                    _this.tableData.forEach(item => {
                        _this.ruleForm.cache.push(item.id);
                    });
                } else {
                    this.checked = false;
                    this.isIndeterminate = false;
                    this.ruleForm.cache = [];
                }
            },
            changeGroup(value) {
                if (value.length) {
                    if (value.length == this.tableData.length) {
                        this.isIndeterminate = false;
                        this.checked = true;
                    } else {
                        this.isIndeterminate = true;
                        this.checked = false;
                    }
                } else {
                    this.isIndeterminate = false;
                    this.checked = false;
                }
            },
            getBaseData() {
                let _this = this;
                httpPost('m=tool&c=generate_cache&a=index', {}, {hideloading: true}).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        _this.tableData = res.data;
                    }
                }).catch(function (error) {
                    console.log(error);
                });
            },
            submitForm() {
                let _this = this;
                let params = JSON.parse(JSON.stringify(this.ruleForm));
                if (!params.cache.length) {
                    message.error(lc('admin_tool_00386'));
                    return false;
                }
                if (_this.submitLoading) {
                    return false;
                }
                _this.submitLoading = true;
                httpPost('m=tool&c=generate_cache&a=cache', params).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg);
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                }).finally(function () {
                    _this.submitLoading = false;
                });
            },
        }
    }
</script>
