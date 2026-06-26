<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="{yun:}t key='admin_tool_00215'{/yun}" type="success" :closable="false"></el-alert>
        </div>
        <div class="moduleElTable">
            <el-table :data="tableData" border style="width: 100%" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column prop="name" label="{yun:}t key='admin_tool_00216'{/yun}"></el-table-column>
                <el-table-column prop="type" label="{yun:}t key='admin_tool_00217'{/yun}" ></el-table-column>
                <el-table-column prop="num" label="{yun:}t key='admin_tool_00214'{/yun}"></el-table-column>
                <el-table-column prop="size" label="{yun:}t key='admin_tool_00213'{/yun}"></el-table-column>
                <el-table-column prop="chip" label="{yun:}t key='admin_tool_00218'{/yun}"></el-table-column>
                <el-table-column prop="charset" label="{yun:}t key='admin_tool_00219'{/yun}"></el-table-column>
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="180">
                    <template slot-scope="scope">
                        <el-button size="mini" plain @click="optimizeDB(scope,2)">{yun:}t key='admin_tool_00220'{/yun}</el-button>
                        <el-button size="mini" plain @click="optimizeDB(scope,3)">{yun:}t key='admin_tool_00221'{/yun}</el-button>
                    </template>
                </el-table-column>
            </el-table>
        </div>
    </div>
</template>

<script>
    module.exports = {
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
                emptytext: window.yunAdminT("{yun:}t key='wap_js_00113'{/yun}"),
                tableData: []
            }
        },
        mounted() {

        },
        methods: {
            async getOptTable() {
                this.loading = true;
                this.emptytext = window.yunAdminT("{yun:}t key='admin_user_weipin_00026'{/yun}");
                let res = await httpPost('m=tool&c=database&a=getOptTable',{},{hideloading: true});
                if (res.data.error == 0) {

                    this.tableData = res.data.data;
                    this.loading = false;
                    if (this.tableData.length === 0){
                        this.emptytext = window.yunAdminT("{yun:}t key='wap_js_00113'{/yun}");
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