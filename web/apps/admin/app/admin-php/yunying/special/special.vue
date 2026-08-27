<template>
<div id="daohaapp" class="moduleElenAl">
        <div class="moduleSeachs">
            <div class="moduleSeachInpt">
                <el-input v-model="searchForm.keyword" size="small" :placeholder="lc('wap_user_00076')" clearable></el-input>
                <el-button type="primary" icon="el-icon-search" size="small" @click="handleSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="small" @click="handleAdd">{{ lc('admin_01235') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table :data="tableData" border style="width: 100%"
                :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" ref="multipleTable"
                @selection-change="handleSelectionChange" @sort-change="shortChange" v-loading="loading">
                <template #empty>
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="100">
                </el-table-column>
                <el-table-column prop="title" :label="lc('member_com_00343')" min-width="220">
                    <template #default="scope">
                        <el-link type="primary" :href="scope.row.title_href"
                            target="_blank">{{scope.row.title}}</el-link>
                    </template>
                </el-table-column>
                <el-table-column prop="limit" :label="lc('admin_yunying_00122')" width="150">
                </el-table-column>
                <el-table-column prop="tpl" :label="lc('wap_js_00090')" width="140">
                </el-table-column>
                <el-table-column :label="lc('member_user_00181')" width="120" align="center">
                    <template #default="scope">
                        <el-switch v-model="scope.row.display_switch" active-color="#1890FF" inactive-color="#B8BDC9"
                            @change="changeDisplay(scope)">
                        </el-switch>
                    </template>
                </el-table-column>
                <el-table-column prop="sort" :label="lc('admin_vue_00044')" sortable="custom" width="120">
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
                <el-table-column prop="comnum" :label="lc('admin_yunying_00142')" width="120">
                </el-table-column>
                <el-table-column prop="booking" :label="lc('admin_00316')" width="120">
                    <template #default="scope">
                        {{scope.row.booking ? scope.row.booking : 0}}
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="210" align="center">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" @click="handleViewCompany(scope)" type=" ">{{ lc('wap_com_00427') }}</el-button>
                            <el-button size="small" @click="editRow(scope)">{{ lc('wap_00148') }}</el-button>
                            <el-button type="danger" size="mini " @click="deleteRow(scope)">{{ lc('wap_js_00077') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulecz modulePagButn">
                <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>

                <el-button @click="deleteRow(null, true)" size="small">{{ lc('member_com_00055') }}</el-button>

            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    v-model:current-page="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes"
                    layout="total, sizes, prev, pager, next, jumper"
                    :total="total">
                </el-pagination>
            </div>
        </div>
        <div class="modluDrawer">
            <el-drawer :title="titleAddEdit" v-model="addVisible" :modal-append-to-body="false" :show-close="true"
                :destroy-on-close="true" size="90%">
                <special_edit :id="id" :file-data="fileData" :qy-data="qyData" @child-event="handleAddClose">
                </special_edit>
            </el-drawer>
        </div>
        <!-- 查看企业 -->
        <div class="modluDrawer">
            <el-drawer :title="titleViewCompany" v-model="viewCompanyVisible" :append-to-body="true"
                :modal-append-to-body="false" :show-close="true" :destroy-on-close="true" size="95%">
                <special_view_index :id="id" @child-event="handleViewCompanyClose"></special_view_index>
            </el-drawer>
        </div>
    </div>
</template>

<script>
import SpecialEdit from './component/special_edit.vue'
import SpecialViewIndex from './component/special_view_index.vue'

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
                    dataText: lc('admin_user_weipin_00026'),
                    searchForm: {
                        page: 1,
                        limit: null,
                        keyword: null,//关键字
                    },
                    total: 0,
                    classData: [],//广告分类
                    domainData: [],//站点
                    tableData: [],
                    pageSizes: [],
                    checked: false,//全选
                    isIndeterminate: false,// checkbox 的不确定状态
                    selectedItem: [],
                    addVisible: false,
                    titleAddEdit: lc('admin_01236'),
                    oldData: null,
                    //添加 编辑
                    id: 0,
                    fileData: [],//专题模板
                    qyData: [],//企业等级
                    //查看
                    titleViewCompany: lc('wap_00559'),
                    viewCompanyVisible: false,
                    prevPage:0
                }
            },
            components: {
                'special_edit': SpecialEdit,
                'special_view_index': SpecialViewIndex,
            },
            created: function () {
                this.getList();


                this.getBaseData();
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
                shortChange(e) {
                    let orderMap = { ascending: 'asc', descending: 'desc' }
                    this.searchForm.t = e.order ? e.prop : null;
                    this.searchForm.order = orderMap[e.order];
                    this.searchForm.page = 1;
                    this.getList();
                },
                handleSizeChange(val) {
                    this.searchForm.limit = val;
                    this.getList();
                },
                handleCurrentChange(val) {
                    this.searchForm.page = val;
                    this.getList();
                },
                handleSearch() {
                    this.searchForm.page = 1
                    this.getList()
                },
                getList() {
                    let _this = this;
                    _this.loading = true;
                    let params = JSON.parse(JSON.stringify(this.searchForm));
                    for (let index in params) {
                        (params[index] === '') && (params[index] = null);
                    }
                    httpPost('m=yunying&c=special_special&a=index', params, {hideloading: true}).then(function (response) {
                        let res = response.data;
                        if (res.error === 0) {
                            _this.tableData = res.data.list;
                            _this.total = res.data.total;
                            _this.searchForm.limit = res.data.perPage;
                            _this.pageSizes = res.data.pageSizes;
                            if(_this.prevPage != _this.searchForm.page){
                                _this.prevPage = _this.searchForm.page;
                                _this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                            }
                            _this.loading = false;
                            if (_this.tableData.length === 0) {
                                _this.dataText = lc('wap_js_00113');
                            }
                        }
                    }).catch(function (error) {
                        console.log(error);
                    });
                },
                getBaseData() {
                    let _this = this;
                    httpPost('m=yunying&c=special_special&a=get_base_data', {}, { hideloading: true }).then(function (response) {
                        let res = response.data;
                        if (res.error === 0) {
                            _this.fileData = Object.freeze(res.data.file);
                            _this.qyData = Object.freeze(res.data.qy_rows);
                        }
                    }).catch(function (error) {
                        console.log(error);
                    });
                },
                handleAdd() {

                    let _this = this;
                    httpPost('m=yunying&c=special_special&a=add', {add:1}, { hideloading: true }).then(function (response) {
                        let res = response.data;
                        if (res.error === 0) {
                            _this.titleAddEdit = lc('admin_01236');
                            _this.id = 0;
                            _this.addVisible = true;
                        }
                    }).catch(function (error) {
                        console.log(error);
                    });
                },
                handleAddClose() {
                    this.addVisible = false;
                    this.getList();
                },
                editRow(scope) {
                    let _this = this;
                    httpPost('m=yunying&c=special_special&a=add', {add:1}, { hideloading: true }).then(function (response) {
                        let res = response.data;
                        if (res.error === 0) {
                            _this.titleAddEdit = lc('admin_01237');
                            _this.id = parseInt(scope.row.id);
                            _this.addVisible = true;
                        }
                    }).catch(function (error) {
                        console.log(error);
                    });
                },
                deleteRow(scope, isMore) {
                    let params = {};
                    if (isMore) {
                        if (!this.selectedItem.length) {
                            message.error(lc('admin_user_weipin_00005'));
                            return false;
                        }
                        let list = [];
                        for (let item of this.selectedItem) {
                            list.push(item.id);
                        }
                        params.del = list;
                    } else {
                        // let index = scope.$index;
                        // this.tableData.splice(index, 1);
                        params.id = scope.row.id;
                    }

                    delConfirm(this, params, this.delete);
                },
                delete(params) {
                    let _this = this;
                    httpPost('m=yunying&c=special_special&a=del', params).then(function (response) {
                        let res = response.data;
                        if (res.error === 0) {
                            message.success(lc('admin_user_00187'));
                            _this.getList();
                        } else {
                            message.error(lc('admin_user_00186'));
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
                    let params = { id: row.id };
                    params[column.property] = row[column.property];
                    httpPost('m=yunying&c=special_special&a=setOrder', params, { hideloading: true }).then(function (response) {
                        let res = response.data;
                        if (res.error === 0) {
                            message.success(lc('admin_user_company_00208'));
                        } else {
                            message.error(lc('admin_00187'));
                        }
                        _this.oldData = null;
                        _this.getList();
                    }).catch(function (error) {
                        console.log(error);
                    });
                },
                changeDisplay(scope) {
                    let _this = this;
                    let id = scope.row.id;
                    let rec = scope.row.display_switch ? 1 : 0;
                    let type = 'rec_display';
                    let params = { id: id, rec: rec, type: type };
                    httpPost('m=yunying&c=special_special&a=recommend', params).then(function (response) {
                        let res = response.data;
                        if (res.error === 0) {
                            _this.getList();
                            message.success(lc('admin_user_company_00208'));
                        } else {
                            message.error(lc('admin_00187'));
                        }
                    }).catch(function (error) {
                        console.log(error);
                    })
                },
                handleViewCompany(scope) {
                    this.id = scope.row.id;
                    this.titleViewCompany = scope.row.title + lc('wap_00559');
                    this.viewCompanyVisible = true;
                },
                handleViewCompanyClose() {
                    this.viewCompanyVisible = false;
                }
            }
        }
</script>
