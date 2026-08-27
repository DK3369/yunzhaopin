<template>
    <div class="tableDome tableDomeTanc" style="top: 40px;">
        <div class="moduleTable">
            <el-table :data="tableData" stripe border style="width: 100%;" height="100%"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading" :empty-text="emptytext">
                <el-table-column prop="id" label="ID" width="55"></el-table-column>
                <el-table-column prop="name" :label="lc('admin_00219')"></el-table-column>
                <el-table-column :label="lc('admin_system_00090')" property="e_name">
                    <template #default="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']" :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.e_name" @blur="alterData(scope)"></el-input>
                        <span v-else>
                            {{ scope.row.e_name }}<img @click="editData(scope)" class="editIcon"
                            src="/admin/php-admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14">
                        </span>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="getList">{{ lc('wap_00318') }}</el-button>
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
            loading: false,
            emptytext: window.yunAdminT(lc('wap_js_00113')),
            page: 0,
            tableData: [], //表格数据
        }
    },
    created() {
        this.getList();
    },
    methods: {
        editData(scope) {
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            this.oldData = JSON.parse(JSON.stringify(row));
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = true;
            this.$set(this.tableData, index, copyRow);
            this.$nextTick(() => {
                let ref = column.property + index;
                $("#" + ref).focus();
            });
        },
        alterData(scope) {
            if (this.oldData == null) {
                return false;
            }
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = false;
            this.$set(this.tableData, index, copyRow);
            if (row[column.property] === this.oldData[column.property]) {
                return false;
            }
            let _this = this;
            let sendData = {id: row.id};
            sendData['name'] = row['name'];
            sendData[column.property] = row[column.property];
            httpPost('m=system&c=category_city&a=ajax', sendData).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(window.yunAdminT(lc('admin_user_company_00208')));
                } else {
                    message.error(window.yunAdminT(lc('admin_00187')));
                }
                _this.oldData = null;
            }).catch(function (error) {
                console.log(error);
            });
        },
        getList() {
            let _this = this;
            _this.loading = true;
            let params = {"page": this.page};
            _this.emptytext = window.yunAdminT(lc('admin_user_weipin_00026'));
            httpPost('m=system&c=category_city&a=ajaxchachong', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.tableData = res.data.list;
                    _this.page = res.data.page;
                    if (_this.tableData.length === 0){
                        _this.emptytext = window.yunAdminT(lc('wap_js_00113'));
                    }
                }
                _this.loading = false;
            }).catch(function (error) {
                console.log(error);
            });
        }
    },
}
</script>

<style scoped>
.setBasicButn {
    height: unset;
}
</style>