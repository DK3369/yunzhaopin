<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_tool_00215')" type="success" :closable="false"></el-alert>
        </div>
        <div class="moduleElTable">
            <el-table :data="tableData" border style="width: 100%" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column prop="name" :label="lc('admin_tool_00216')"></el-table-column>
                <el-table-column prop="type" :label="lc('admin_tool_00217')" ></el-table-column>
                <el-table-column prop="num" :label="lc('admin_tool_00214')"></el-table-column>
                <el-table-column prop="size" :label="lc('admin_tool_00213')"></el-table-column>
                <el-table-column prop="chip" :label="lc('admin_tool_00218')"></el-table-column>
                <el-table-column prop="charset" :label="lc('admin_tool_00219')"></el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="180">
                    <template #default="scope">
                        <el-button size="small" plain @click="optimizeDB(scope,2)">{{ lc('admin_tool_00220') }}</el-button>
                        <el-button size="small" plain @click="optimizeDB(scope,3)">{{ lc('admin_tool_00221') }}</el-button>
                    </template>
                </el-table-column>
            </el-table>
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
            optimize: Number
        },
        watch: {
            optimize: {
                handler(newValue, oldValue) {
                    if (newValue == 1) {
                        this.getOptTable();
                    }
                },
                deep: true,
                immediate: true
            }
        },
        data: function () {
            return {
                loading: false,
                emptytext: window.yunAdminT(lc('wap_js_00113')),
                tableData: []
            }
        },
        mounted() {

        },
        methods: {
            async getOptTable() {
                this.loading = true;
                this.emptytext = window.yunAdminT(lc('admin_user_weipin_00026'));
                let res = await httpPost('m=tool&c=database&a=getOptTable',{},{hideloading: true});
                if (res.data.error == 0) {

                    this.tableData = res.data.data;
                    this.loading = false;
                    if (this.tableData.length === 0){
                        this.emptytext = window.yunAdminT(lc('wap_js_00113'));
                    }
                }
            },
            optimizeDB:function(scope, type){

                let that = this;
                let params = {};
                params.name = scope.row.name;
                params.type = type;

                httpPost('m=tool&c=database&a=optimizeTable', params).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg, function () {
                            that.getOptTable();
                        });
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                })
            }
        },
    };
</script>
<style scoped>
    .moduleSeachmore{padding:0}
    .moduleSeachs{padding:0 0 12px 0;width:100%}
    .moduleElTable{padding:0;margin:0;height:calc(100% - 80px);width:100%}
    .tableSeachInptsmalltwo{margin-bottom:0;margin-right:12px}
    .tableSeachInptsmalltwo .el-input__inner{height:32px;line-height:32px;width:260px;padding:0 5px}
    .el-dialog__body{padding:0 20px}
</style>