<template>
<div id="cityfl" class="moduleElenAl">
    <div class="moduleSeachs">
        <div class="">{{ lc('wap_user_00018') }}</div>
        <div class="nrtopbtn">
            <el-button size="small" icon="el-icon-plus" @click="addVisible = true">{{ lc('admin_00222') }}</el-button>
            <el-button size="small" style="margin-left: 10px;" icon="el-icon-refresh"
                @click="pinyin">{{ lc('admin_system_00073') }}</el-button>
            <el-button size="small" icon="el-icon-refresh-right" @click="chachongVisible = true">{{ lc('admin_system_00065') }}</el-button>
            <el-button size="small" type="primary" icon="el-icon-delete" @click="clearPinYin">{{ lc('admin_system_00072') }}</el-button>
        </div>
    </div>

    <div class="moduleElTable">
        <el-table :data="tableData" stripe border style="width: 100%;" height="100%"
            :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" ref="multipleTable"
            @selection-change="handleSelectionChange" v-loading="loading" :empty-text="emptytext">
            <el-table-column type="selection" width="55"></el-table-column>
            <el-table-column prop="id" :label="lc('member_com_00345')" width="80"></el-table-column>
            <el-table-column :label="lc('admin_system_00109')" property="name">
                <template #default="scope">
                    <el-input v-if="scope.row[scope.column.property + 'isShow']"
                        :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                        v-model="scope.row.name" @blur="alterData(scope)"></el-input>
                    <span v-else>
                        <template>
                            {{ lc("admin_level1_category_value", [scope.row.name]) }}<img @click="editData(scope)" class="editIcon"
                            src="/admin/php-admin/images/bine.png" alt="" style="margin-left: 4px;" width="14"
                            height="14">
                        </template>
                    </span>
                </template>
            </el-table-column>
            <el-table-column :label="lc('admin_system_00108')" property="e_name">
                <template #default="scope">
                    <el-input v-if="scope.row[scope.column.property + 'isShow']"
                        :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                        v-model="scope.row.e_name" @blur="alterData(scope)"></el-input>
                    <span v-else>
                        {{ scope.row.e_name }}<img @click="editData(scope)" class="editIcon"
                        src="/admin/php-admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14">
                    </span>
                </template>
            </el-table-column>
            <el-table-column :label="lc('admin_system_00110')" property="s_name">
                <template #default="scope">
                    <el-input v-if="scope.row[scope.column.property + 'isShow']"
                        :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                        v-model="scope.row.s_name" @blur="alterData(scope)"></el-input>
                    <span v-else>
                        {{ scope.row.s_name }}<img @click="editData(scope)" class="editIcon"
                        src="/admin/php-admin/images/bine.png" alt="" style="margin-left: 4px;" width="14"
                        height="14">
                    </span>
                </template>
            </el-table-column>
            <el-table-column :label="lc('admin_system_00114')" property="rec" width="80">
                <template #default="scope">
                    <el-switch v-model="scope.row.rec_n" @change="changeRec(scope)"></el-switch>
                </template>
            </el-table-column>
            <el-table-column :label="lc('admin_system_00113')" property="sort" width="80">
                <template #default="scope">
                    <el-input v-if="scope.row[scope.column.property + 'isShow']"
                        :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                        v-model="scope.row.sort" @blur="alterData(scope, 'int')"
                        onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                    <span v-else>
                        {{ scope.row.sort }}<img @click="editData(scope)" class="editIcon"
                        src="/admin/php-admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14">
                    </span>
                </template>
            </el-table-column>
            <el-table-column header-align="center" align="right" :label="lc('member_user_00048')" :width="140">
                <template #default="scope">
                    <div class="cz_button">
                        <el-button size="small" @click="openManage(scope)">{{ lc('wap_com_00304') }}</el-button>
                        <el-button size="small" @click="deleteRow(scope)" type="danger">{{ lc('wap_js_00077') }}</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
    </div>
    <div class="modulePaging">
        <div class="">
            <div class="modulecz modulePagButn">
                <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="deleteRow(null, true)" size="small">{{ lc('member_com_00055') }}</el-button>
            </div>
        </div>
    </div>
    <div class="modluDrawer">
        <el-drawer :title="titleAddEdit" v-model="addVisible" :modal-append-to-body="false" :show-close="true"
            :destroy-on-close="true" size="80%">
            <job_class_edit :tid="0" :id="0" @child-event-getlist="getList"></job_class_edit>
        </el-drawer>
    </div>
    <div class="modluDrawer">
        <el-drawer :title="lc('wap_com_00304')" v-model="manageVisible" :modal-append-to-body="false" :append-to-body="true" :show-close="true"
            :destroy-on-close="true" size="95%">
            <job_class_manage :id="info.id" @child-event-getlist="getList"></job_class_manage>
        </el-drawer>
    </div>
    <div class="modluDrawer">
        <el-drawer :title="lc('admin_system_00062')" v-model="chachongVisible" :modal-append-to-body="false" size="50%"
            :destroy-on-close="true" :wrapper-closable="false" :close-on-press-escape="false">
            <job_class_chachong></job_class_chachong>
        </el-drawer>
    </div>
</div>
</template>

<script>
import JobClassChachong from './component/job_class_chachong.vue'
import JobClassEdit from './component/job_class_edit.vue'
import JobClassManage from './component/job_class_manage.vue'

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
                emptytext: window.lc('wap_js_00113'),
                loading: false,
                title: window.lc('admin_00222'),
                tableData: [], //表格数据
                checked: false,
                isIndeterminate: false,// checkbox 的不确定状态
                selectedItem: [],
                addVisible: false,
                titleAddEdit: window.lc('admin_00222'),
                oldData: null,
                chachongVisible: false,
                chachongPage: 0,
                position: [],
                positionTwo: [],//第二级分类
                moveVisible: false,
                moveForm: {
                    pid: null, //自身id
                    type: 0,
                    nid: null, //父类id
                    keyid: null //第二级分类id
                },
                info: {},
                manageVisible: false,
                
            }
        },
        created() {
            this.getList();


        },
        methods: {
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
                _this.loading = true;
                _this.emptytext = window.lc('admin_user_weipin_00026');
                httpPost('m=system&c=category_job_class&a=index').then(function (response) {
                    let res = response.data;
                    res.data.forEach((item) => {
                        item.rec_n = item.rec > 0 ? true : false;
                    });
                    _this.tableData = res.data;
                    _this.loading = false;
                    if (_this.tableData.length === 0){
                        _this.emptytext = window.lc('wap_js_00113');
                    }
                }).catch(function (error) {
                    console.log(error);
                });
            },
            deleteRow(scope, isMore) {
                let params = {};
                if (isMore) {
                    if (!this.selectedItem.length) {
                        message.error(window.lc('admin_user_weipin_00005'));
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
                httpPost('m=system&c=category_job_class&a=del', params).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        message.success(window.lc('admin_user_00187'));
                        _this.getList();
                    } else {
                        message.error(window.lc('admin_user_00186'));
                    }
                }).catch(function (error) {
                    console.log(error);
                });
            },
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
                httpPost('m=system&c=category_job_class&a=ajax', sendData, {hideloading: true}).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        message.success(window.lc('admin_user_company_00208'));
                    } else {
                        message.error(window.lc('admin_00187'));
                    }
                    _this.oldData = null;
                    _this.getList();
                }).catch(function (error) {
                    console.log(error);
                });
            },
            openManage(scope) {
                this.info = scope.row;
                this.manageVisible = true;
            },
            changeRec(scope) {
                let recVal = scope.row.rec_n ? 1 : 0;
                let recBefore = scope.row.rec;
                if (recBefore == recVal) {
                    return;
                }
                let _this = this;
                let params = {rec: recVal, id: scope.row.id};
                httpPost('m=system&c=category_job_class&a=setrec', params).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        _this.getList();
                        message.success(window.lc('admin_user_company_00208'));
                    } else {
                        message.error(window.lc('admin_00187'));
                    }
                }).catch(function (error) {
                    console.log(error);
                })
            },
            pinyin() {
                delConfirm(this, {
                    page: 0,
                    pagesize: 100
                }, this.doPinyin, window.lc('admin_system_00074'));
            },
            /**
             * 生成拼音
             * @param params {page:0, pagesize:100}
             */
            doPinyin(params) {
                let _this = this;
                httpPost('m=system&c=category_job_class&a=ajaxpinyin', params).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        message.success(window.lc('admin_system_00081'));
                        _this.getList();
                    } else if (res.error === 1) {
                        message.warning(res.msg);
                        params.page = res.data.page;
                        _this.doPinyin(params);
                    }
                }).catch(function (error) {
                    console.log(error);
                });
            },
            clearPinYin() {
                let _this = this;
                delConfirm(this, {}, function () {
                    httpPost('m=system&c=category_job_class&a=clearpinyin').then(function (response) {
                        message.success(window.lc('admin_system_00080'));
                        _this.getList();
                    }).catch(function (error) {
                        console.log(error);
                    });
                }, window.lc('admin_system_00075'));
            },
        },
        components: {
            'job_class_chachong': JobClassChachong,
            'job_class_edit': JobClassEdit,
            'job_class_manage': JobClassManage,
        }
    }
</script>
