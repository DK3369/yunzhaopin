<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="该页面展示了网站数据库信息，可对数据库进行备案还原操作，也可通过其它软件进行操作" type="success" :closable="false"></el-alert>
        </div>

        <div class="moduleTable" v-show="!progressShow">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">名称</th>
                    <th width="400">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">选择备份类型</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="backType" @change="checkBackType">
                                <el-radio label="1">全部备份</el-radio>
                                <el-radio label="2">自定义备份</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>全部备份：备份数据库所有表，自定义备份：根据自行选择备份数据表</span>
                        </div>
                    </td>
                </tr>
                <tr v-show="dbTableShow">
                    <td>
                        <div class="TableTite"></div>
                    </td>
                    <td colspan="2">
                        <div class="TableButn">
                            <el-checkbox :indeterminate="isIndeterminate" v-model="checkAll" @change="handleCheckAllChange">全选</el-checkbox>
                            <div style="margin: 15px 0;"></div>
                            <div v-for="(table,tkey) in dbTable" :key="tkey">
                                <el-checkbox-group v-model="checkedTable" @change="handleCheckedTableChange">
                                    <el-checkbox  v-for="(item,index) in table" :key="index" :label="item.name" >{{item.name}}</el-checkbox>
                                </el-checkbox-group>
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">分卷备份</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="maxFileSize" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>文件长度限制(kb)</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="backUp">提交</el-button>
            </div>
        </div>
        <div v-show="progressShow">
            <el-progress :text-inside="true" text-color="#fff" :stroke-width="20" :percentage="percentage" status="exception"></el-progress>
        </div>
    </div>
</template>

<script>
    module.exports = {
        data: function () {
            return {
                progressShow: false,
                percentage: 0,

                backType: '1',

                dbTableShow: false,
                dbTable: [],
                dbLength: 0,
                checkedTable:[],
                checkAll: false,
                isIndeterminate: true,

                maxFileSize: 2048,
                backName: '',
            }
        },
        created(){
            this.getDbTable();
        },
        mounted() {
        },
        methods: {
            async getDbTable() {
                let res = await httpPost('m=tool&c=database&a=getDbTable',{},{hideloading: true});
                if (res.data.error == 0) {

                    this.dbTable = res.data.data.dbTable;
                    this.dbLength = res.data.data.dbLength;
                }
            },
            checkBackType: function(val){
                let that = this;
                that.dbTableShow = val == 2 ? true : false;
            },
            handleCheckAllChange(val) {
                let that = this;
                that.checkedTable = [];
                if (val){
                    that.dbTable.forEach(function (item) {
                        item.forEach(function (v) {
                            that.checkedTable.push(v.name.toString());
                        })
                    });
                }
                that.isIndeterminate = false;
            },
            handleCheckedTableChange(value) {

                let checkedCount = value.length;
                this.checkAll = checkedCount === this.dbLength;
                this.isIndeterminate = checkedCount > 0 && checkedCount < this.dbLength;
            },
            backUp: function () {

                let that = this;
                let param = {};
                param.table = that.checkedTable;
                param.maxFileSize = that.maxFileSize;
                param.backType = that.backType

                if (that.backType == 2 && that.checkedTable.length == 0){
                    message.error('请选择需要备份的数据！');
                    return false;
                }
                delConfirm(this, param, that.backUpDb, '确定备份数据？');

            },
            backUpDb: function (param) {
                let that = this;

                httpPost('m=tool&c=database&a=backUp', param).then(function (res) {
                    if (res.data.error == 0) {

                        that.progressShow = true;
                        let data = res.data.data;
                        let count = that.checkedTable.length
                        if(that.backType == '1'){
                            count = that.dbLength
                        }
                        that.BackupDatabaseFileSize(data.a, data.t, data.s, data.p, data.mypath, '', '', '', '', data.waitbaktime, count);
                    } else {

                        message.error(res.data.msg);
                        return false;
                    }
                });
            },
            BackupDatabaseFileSize(a, t, s, p, mypath, alltotal, thenof, fnum, stime, waitbaktime, count){

                let that = this,
                    param = {};
                param.t = t;
                param.s = s;
                param.p = p;
                param.t = t;
                param.mypath = mypath;
                param.alltotal = alltotal;
                param.thenof = thenof;
                param.fnum = fnum;
                param.stime = stime;
                httpPost('m=tool&c=database&a='+a, param).then(function (res) {
                    if (res.data.error == 0) {
                        let data = res.data.data;
                        if (data.t) {
                            var n = parseInt(accMul(accDiv(data.t, count), 100));
                            that.percentage = n;
                            that.BackupDatabaseFileSize(a, data.t, data.s, data.p, data.mypath, data.alltotal, data.thenof, data.fnum, data.stime, data.waitbaktime, count);
                        }
                    }else if (res.data.error == 2){

                        message.success(res.data.msg, function () {

                            that.progressShow = false;
                            that.backType = '1';
                            that.dbTableShow = false;
                            that.checkedTable = [];
                            that.percentage = 0;
                        });
                    }else {

                        message.error(res.data.msg);
                        return false;
                    }
                })
            },
        }
    };

    function timestampToTime(timestamp) {
        var date = new Date(timestamp);
        var Y = date.getFullYear() + '-';
        var M = (date.getMonth()+1 < 10 ? '0'+(date.getMonth()+1) : date.getMonth()+1) + '-';
        var D = (date.getDate() < 10 ? '0'+date.getDate() : date.getDate()) + ' ';
        // var h = (date.getHours() < 10 ? '0'+date.getHours() : date.getHours()) + ':';
        // var m = (date.getMinutes() < 10 ? '0'+date.getMinutes() : date.getMinutes()) + ':';
        // var s = (date.getSeconds() < 10 ? '0'+date.getSeconds() : date.getSeconds());
        // strDate = Y+M+D+h+m+s;
        strDate = Y+M+D;
        return strDate;

    }
</script>
<style scoped>
    .moduleTable {max-height: calc(100% - (60px + 10px));}
    .tableVue .el-checkbox{width: 24%;margin-bottom: 10px;}
</style>