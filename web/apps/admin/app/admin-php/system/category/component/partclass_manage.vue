<template>
    <div class="moduleElenAlCategorySub">
        <div class="moduleSeachs categorySub">
            <div></div>
            <div class="categoryTopBtn">
                <el-button class="" type="primary" icon="el-icon-document-add" size="small" @click="addVisible = true">{{ lc('admin_00197') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable moduleElTableCategoreSub">
            <el-table :data="tableData" border style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('admin_system_00098')" width="100"></el-table-column>
                <el-table-column :label="lc('admin_system_00097')" property="name">
                    <template #default="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']" :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.name" @blur="alterData(scope)"></el-input>
                        <span v-else>
                            <template v-if="scope.$index == 0">
                                {{ lc("admin_level1_category_value", [scope.row.name]) }}
                            </template>
                            <template v-else>
                                &emsp;&emsp;┗{{ scope.row.name }}<img @click="editData(scope)" class="editIcon" src="/admin/php-admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14">
                            </template>
                        </span>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_system_00100')" property="sort">
                    <template #default="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']" :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.sort" @blur="alterData(scope, 'int')" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                        <span v-else>
                            <template v-if="scope.$index == 0"></template>
                            <template v-else>{{ scope.row.sort }}<img @click="editData(scope)" class="editIcon" src="/admin/php-admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14"></template>
                        </span>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="70">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="small" @click="deleteRow(scope)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="">
                <div class="">
                    <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                    <el-button size="small" @click="deleteRow(null, true)">{{ lc('member_com_00055') }}</el-button>
                </div>
            </div>
        </div>
        <el-dialog :title="lc('admin_system_00125')" width="40%" v-model="addVisible" :modal-append-to-body="false" :append-to-body="true">
            <partclass_add :position="position" @child-event-getlist="handleList"></partclass_add>
        </el-dialog>
    </div>
</template>

<script>
import PartclassAdd from './partclass_add.vue'

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
        id: {type: [Number, String], default: 0},
    },
    data: function () {
        return {
            emptytext: window.yunAdminT(lc('wap_js_00113')),
            tableData: [], //表格数据
            checked: false,
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            position: [],//一级分类列表
            addVisible: false,
            oldData: null,
            loading: false
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
        alterData(scope, type) {
            if (this.oldData == null) {
                return false;
            }
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            if (type === 'int') {
                row[column.property] = row[column.property].replace(/[^0-9]/g, '');
            }
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = false;
            this.$set(this.tableData, index, copyRow);
            if (row[column.property] === this.oldData[column.property]) {
                return false;
            }
            let _this = this;
            let sendData = {id: row.id};
            sendData[column.property] = row[column.property];
            httpPost('m=system&c=category_partclass&a=ajax', sendData, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(window.yunAdminT(lc('admin_user_company_00208')));
                } else {
                    message.error(window.yunAdminT(lc('admin_00187')));
                }
                _this.oldData = null;
                _this.getList();
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleSelectionChange(val) {
            this.selectedItem = val;
            if (this.selectedItem.length == 0) {
                this.isIndeterminate = false;
                this.checked = false;
            } else {
                if (this.selectedItem.length == this.tableData.length) {
                    this.isIndeterminate = false;
                    this.checked = true;
                } else {
                    this.isIndeterminate = true;
                    this.checked = false;
                }
            }
        },
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
        getList() {
            this.addVisible = false;
            let _this = this;
            let params = {id: this.id};
            _this.loading = true;
            _this.emptytext = window.yunAdminT(lc('admin_user_weipin_00026'));
            httpPost('m=system&c=category_partclass&a=up', params).then(function (response) {
                let res = response.data;
                let list = [];
                if (res.data.class1) {
                    list.push(res.data.class1);
                }
                for (let item of res.data.class2) {
                    list.push(item);
                }
                _this.tableData = list;
                _this.position = res.data.position;
                _this.loading = false;
                if (_this.tableData.length === 0){
                    _this.emptytext = window.yunAdminT(lc('wap_js_00113'));
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleList(ctype) {
            if (ctype == 1) {
                this.addVisible = false;
                this.$emit("child-event-getlist");
            } else {
                this.getList();
            }
        },
        deleteRow(scope, isMore) {
            let params = {};
            if (isMore) {
                if (!this.selectedItem.length) {
                    message.error(window.yunAdminT(lc('admin_user_weipin_00005')));
                    return false;
                }
                let list = [];
                for (let item of this.selectedItem) {
                    list.push(item.id);
                }
                params.delType = 'more';
                params.del = list;
            } else {
                // let index = scope.$index;
                // this.tableData.splice(index, 1);
                params.delType = 'single';
                params.delid = scope.row.id;
            }

            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=system&c=category_partclass&a=del', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(window.yunAdminT(lc('admin_user_00187')));
                    _this.getList();
                } else {
                    message.error(window.yunAdminT(lc('admin_user_00186')));
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
    },
    components: {
        'partclass_add': PartclassAdd,
    }
}
</script>

<style scoped>
	.el-table .el-table__cell { padding: 12px 0; }
</style>