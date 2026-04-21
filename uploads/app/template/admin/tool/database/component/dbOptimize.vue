<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="手动清理数据表碎片，定期清理数据表碎片可以提升数据库的查询速度。" type="success" :closable="false"></el-alert>
        </div>
        <div class="moduleElTable">
            <el-table :data="tableData" border style="width: 100%" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column prop="name" label="数据表"></el-table-column>
                <el-table-column prop="type" label="数据表类型" ></el-table-column>
                <el-table-column prop="num" label="数量"></el-table-column>
                <el-table-column prop="size" label="数据"></el-table-column>
                <el-table-column prop="chip" label="碎片"></el-table-column>
                <el-table-column prop="charset" label="字符集"></el-table-column>
                <el-table-column label="操作" width="180">
                    <template slot-scope="scope">
                        <el-button size="mini" plain @click="optimizeDB(scope,2)">修复</el-button>
                        <el-button size="mini" plain @click="optimizeDB(scope,3)">优化</el-button>
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
                emptytext: '暂无数据',
                tableData: []
            }
        },
        mounted() {

        },
        methods: {
            async getOptTable() {
                this.loading = true;
                this.emptytext = "数据加载中";
                let res = await httpPost('m=tool&c=database&a=getOptTable',{},{hideloading: true});
                if (res.data.error == 0) {

                    this.tableData = res.data.data;
                    this.loading = false;
                    if (this.tableData.length === 0){
                        this.emptytext = "暂无数据";
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