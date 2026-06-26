<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="{yun:}t key='admin_tool_00073'{/yun}" type="success" :closable="false"></el-alert>
        </div>
        <div style="padding: 0px;margin: 0px;height: calc(100% - 90px);width: 100%;">
            <el-table :data="tableData" border style="width: 100%" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" @selection-change="handleSelectionChange" ref="backTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="80"></el-table-column>
                <el-table-column prop="name" label="{yun:}t key='admin_tool_00067'{/yun}"></el-table-column>
                <el-table-column prop="time" label="{yun:}t key='wap_js_00088'{/yun}"></el-table-column>
                <el-table-column prop="dbname" label="{yun:}t key='admin_tool_00069'{/yun}"></el-table-column>
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="150">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <a href="javascript:;" @click="backIn(scope)">
                                <el-button size="small">{yun:}t key='admin_tool_00068'{/yun}</el-button>
                            </a>
                            <el-button type="danger" size="small" @click="delBack(scope)" >{yun:}t key='common.delete'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulecz">
                <el-checkbox :indeterminate="isIndeterminate" v-model="checkAll" @change="handleCheckAllChange">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button size="mini" @click="delBackSel"> {yun:}t key='member_com_00055'{/yun}</el-button>
            </div>
        </div>
    </div>
</template>

<script>
    module.exports = {
        props: {
            need: Number
        },
        watch: {
            need: {
                handler(newValue, oldValue) {
                    if (newValue != 0 && newValue != oldValue) {

                        this.getBackFile();
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
                tableData: [],

                // 批量选择
                checkAll: false,
                isIndeterminate: false,
                selectedItem: [],
            }
        },
        mounted() {
        },
        methods: {
            async getBackFile() {
                this.loading = true;
                this.emptytext = window.yunAdminT("{yun:}t key='admin_user_weipin_00026'{/yun}");
                let res = await httpPost('m=tool&c=database&a=getBackFile');
                if (res.data.error == 0) {

                    this.tableData = res.data.data;
                    this.loading = false;
                    if (this.tableData.length === 0){
                        this.emptytext = window.yunAdminT("{yun:}t key='wap_js_00113'{/yun}");
                    }
                }
            },
            handleSelectionChange(val) {
                this.selectedItem = val;
                if (this.selectedItem.length == 0) {
                    this.isIndeterminate = false;
                    this.checkAll = false;
                } else {
                    if (this.selectedItem.length == this.tableData.length) {
                        this.isIndeterminate = false;
                        this.checkAll = true;
                    } else {
                        this.isIndeterminate = true;
                        this.checkAll = false;
                    }
                }
            },
            handleCheckAllChange(val){
                val ? this.$refs.backTable.toggleAllSelection() : this.$refs.backTable.clearSelection();
            },
            backIn(scope){

                let that = this;
                let params = {};
                params.ver = scope.row.version;
                params.mypath = scope.row.name;
                message.confirm(window.yunAdminT("{yun:}t key='admin_tool_00070'{/yun}"), function () {
                    window.location.href = baseUrl+'m=tool&c=database&a=backIn&mypath=' + scope.row.name +'&ver='+scope.row.version+'&pytoken=' + localStorage.getItem('pytoken');
                })

                // httpPost('m=tool&c=database&a=backIn', params).then(function (response) {
                //     let res = response.data;
                //     if (res.error == 0) {
                //         message.success(res.msg, function () {
                //
                //             self.getBackFile();
                //         });
                //     } else {
                //         message.error(res.msg);
                //     }
                // }).catch(function (error) {
                //     console.log(error);
                // })
            },
            delBack(scope, isMore) {
                var that = this;
                let name = '';
                let sqlArr = [];
                let params = {};
                if (isMore) {
                    this.selectedItem.forEach((item) => {

                        sqlArr.push(item.name);
                    });
                    params.sql = sqlArr;
                } else {

                    params.sql = scope.row.name;
                }

                delConfirm(this, params, this.delete, window.yunAdminT("{yun:}t key='admin_tool_00071'{/yun}"));
            },
            delBackSel() {
                var that = this;
                if (!that.selectedItem.length) {
                    message.error(window.yunAdminT("{yun:}t key='admin_tool_00072'{/yun}"));
                    return;
                }
                this.delBack(null, true);
            },
            delete(params){
                var self = this;
                httpPost('m=tool&c=database&a=delBack', params).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg, function () {
                            self.getBackFile();
                        });
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                })
            },
            downRec(path){
            },
        },
    };
</script>
<style scoped>
    .moduleSeachmore{padding:0}
    .moduleSeachs{padding:0 0 12px 0;width:100%}
    .moduleElTable{padding:0;margin:0;height:calc(100% - 136px);width:100%}
    .tableSeachInptsmalltwo{margin-bottom:0;margin-right:12px}
    .tableSeachInptsmalltwo .el-input__inner{height:32px;line-height:32px;width:260px;padding:0 5px}
    .el-dialog__body{padding:0 20px}
</style>