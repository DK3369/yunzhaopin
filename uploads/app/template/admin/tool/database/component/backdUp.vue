<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="该页面展示了网站数据库信息，可对数据库进行备案还原操作，也可通过其它软件进行操作" type="success" :closable="false"></el-alert>
        </div>
        <div style="padding: 0px;margin: 0px;height: calc(100% - 90px);width: 100%;">
            <el-table :data="tableData" border style="width: 100%" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" @selection-change="handleSelectionChange" ref="backTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="80"></el-table-column>
                <el-table-column prop="name" label="文件名"></el-table-column>
                <el-table-column prop="time" label="时间"></el-table-column>
                <el-table-column prop="dbname" label="数据库名称"></el-table-column>
                <el-table-column label="操作" width="150">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <a href="javascript:;" @click="backIn(scope)">
                                <el-button size="small">恢复</el-button>
                            </a>
                            <el-button type="danger" size="small" @click="delBack(scope)" >删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulecz">
                <el-checkbox :indeterminate="isIndeterminate" v-model="checkAll" @change="handleCheckAllChange">全选</el-checkbox>
                <el-button size="mini" @click="delBackSel"> 批量删除</el-button>
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
                emptytext: '暂无数据',
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
                this.emptytext = "数据加载中";
                let res = await httpPost('m=tool&c=database&a=getBackFile');
                if (res.data.error == 0) {

                    this.tableData = res.data.data;
                    this.loading = false;
                    if (this.tableData.length === 0){
                        this.emptytext = "暂无数据";
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
                message.confirm('确定导入数据？', function () {
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

                delConfirm(this, params, this.delete, '您确定要删除数据库备份信息？');
            },
            delBackSel() {
                var that = this;
                if (!that.selectedItem.length) {
                    message.error('请选择要删除的备份数据');
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