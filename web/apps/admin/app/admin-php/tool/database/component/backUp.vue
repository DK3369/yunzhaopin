<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_tool_00073')" type="success" :closable="false"></el-alert>
        </div>

        <div class="moduleTable" v-show="!progressShow">
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
                        <div class="TableTite">{{ lc('admin_tool_00074') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="backType" @change="checkBackType">
                                <el-radio label="1">{{ lc('admin_tool_00075') }}</el-radio>
                                <el-radio label="2">{{ lc('admin_tool_00076') }}</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00077') }}</span>
                        </div>
                    </td>
                </tr>
                <tr v-show="dbTableShow">
                    <td>
                        <div class="TableTite"></div>
                    </td>
                    <td colspan="2">
                        <div class="TableButn">
                            <el-checkbox :indeterminate="isIndeterminate" v-model="checkAll" @change="handleCheckAllChange">{{ lc('wap_js_00074') }}</el-checkbox>
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
                        <div class="TableTite">{{ lc('admin_tool_00078') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="maxFileSize" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00079') }}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="backUp">{{ lc('common.submit') }}</el-button>
            </div>
        </div>
        <div v-show="progressShow">
            <el-progress :text-inside="true" text-color="#fff" :stroke-width="20" :percentage="percentage" status="exception"></el-progress>
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
                    message.error(window.yunAdminT(lc('admin_tool_00080')));
                    return false;
                }
                delConfirm(this, param, that.backUpDb, window.yunAdminT(lc('admin_tool_00081')));

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