<template>
<div id="cityfl" class="moduleElenAl">
        <div class="moduleSeachs">
            <div class="moduleSeachleft">
                <div class="tableSeachInpt newsinput" style="margin-bottom: 0px;;">
                    <el-input v-model="searchForm.keyword" :placeholder="lc('admin_00340')" size="small"
                              clearable prefix-icon="el-icon-search">
                    </el-input>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall newsinput" style="margin-bottom: 0px;;"
                     v-for="(searchItem, searchIndex) in searchList">
                    <el-select v-model="searchForm[searchItem.param]" size="small" :clearable="true" :placeholder="searchItem.name" @change="search">
                        <el-option v-for="(searchLabel, searchValue) in searchItem.value" :label="searchLabel" :value="searchValue"></el-option>
                    </el-select>
                </div>

                <div class="newsbtnbox" style="margin-bottom: 0px;;">
                    <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>
            <div class="nrtopbtn" style="padding: 6px 12px;">
                <el-button type="primary" icon="el-icon-document-add" size="small"
                    @click="openAdd('')">{{ lc('admin_00106') }}</el-button>
            </div>
        </div>

        <div class="moduleElTable">
            <el-table :data="list" :default-sort="{prop: 'date', order: 'descending'}" stripe border :empty-text="emptytext"
                      ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="sortChange"
                style="width: 100%;" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading">
                <el-table-column type="selection" width="55"> </el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="90" sortable="custom">
                </el-table-column>
                <el-table-column prop="title" :label="lc('admin_00102')">
                </el-table-column>
                <el-table-column prop="datetime" :label="lc('admin_user_weipin_00030')" sortable="custom" :formatter="formatter">
                </el-table-column>
                <el-table-column prop="view_num" :label="lc('wap_com_00112')" sortable="custom">
                </el-table-column>
                <el-table-column prop="startime" :label="lc('admin_company_00005')" sortable="custom" :formatter="formatter">
                </el-table-column>
                <el-table-column prop="endtime" :label="lc('admin_company_00006')" sortable="custom" :formatter="formatter">
                </el-table-column>
                <el-table-column prop="did" :label="lc('admin_00151')" :formatter="formatter">
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="200" fixed="right">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small " plain @click="openDomain(scope.row)">{{ lc('admin_user_weipin_00050') }}</el-button>
                            <el-button size="small " plain @click="openAdd(scope.row)">{{ lc('wap_js_00073') }}</el-button>
                            <el-button type="danger" size="small" @click="del(scope.$index)">{{ lc('wap_js_00077') }}</el-button>
                        </div>
                    </template>
                </el-table-column>

            </el-table>
        </div>
        <div class="modulePaging">
            <div class="">
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate" @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="batch('del')" size="small">{{ lc('member_com_00055') }}</el-button>
                <el-button @click="batch('domain')" size="small">{{ lc('admin_user_00279') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                               :current-page="page" :page-sizes="pageSizes" :page-size="limit"
                               layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>

        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_weipin_00029')" width="500px" v-model="dialogDomain" :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div v-if="detail.id" class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00104') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <span>{{detail.title}}</span>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_user_weipin_00020') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-select v-model="ruleFormDomain.did" filterable :placeholder="lc('wap_user_00100')">
                                <el-option v-for="(item, key) in domainList" :key="key" :label="item" :value="key">
                                </el-option>
                            </el-select>
                        </div>
                    </div>

                </div>
                <template #footer><div class="dialog-footer">
                    <el-button type="primary" @click="saveDomain">{{ lc('wap_js_00094') }}</el-button>
                </div></template>
            </el-dialog>

            <el-drawer :title="detail.id ? lc('admin_00105') : lc('admin_00106')" v-model="drawerAdd" :modal-append-to-body="false" :show-close="true" :with-header="true" size="900px">
                <add :id="detail.id ? detail.id : ''" @child-event="closeAdd"></add>
            </el-drawer>
        </div>
    </div>
</template>

<script>
import AnnouncementAdd from './component/announcement_add.vue'

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
                emptytext: lc('wap_js_00113'),
                loading: false,
                // Search filters
                searchList: [],
                searchForm: {
                    type: '1'
                },

                // List
                page: 1,
                limit: 0,
                list: [],
                total: 0,
                pageSizes: [],

                // List sorting
                t: '',
                order: '',

                checkedAll: false, // Select all
                checkedAllIndeterminate: false,
                multipleSelection: [], // Multi-select value storage
                idArr: [],

                detail: {},

                saveLoading: false,

                // Add
                drawerAdd: false,

                domainList: {},

                // Domain switch
                dialogDomain: false,
                ruleFormDomain: {},
                prevPage:0
            }
        },
        components: {
            'add': AnnouncementAdd,
        },
        created() {
            this.getGroup()
            this.getList();


        },
        methods: {
            handleSizeChange(val) {
                this.limit = val;
                this.getList();
            },
            handleCurrentChange(val) {
                this.page = val;
                this.getList();
            },
            sortChange(event) {
                this.t = event.order ? event.prop : '';
                this.order = event.order ? event.order == 'descending' ? 'desc' : 'asc' : '';
                this.search();
            },
            search() {
                this.page = 1;
                this.getList();
            },
            getGroup() {
                let that = this;
                httpPost('m=neirong&c=announcement&a=getGroup', {}, {hideloading: true}).then(function (response) {
                    let res = response.data,
                        data = res.data;
                    that.searchList = data.search_list;
                    that.domainList = data.domainList;
                })
            },
            getList() {
                let that = this,
                    searchForm = that.searchForm,
                    params = {
                        page: that.page,
                        limit: that.limit,
                        t: that.t,
                        order: that.order,
                    };
                    that.loading = true;
                    that.emptytext = lc('admin_user_weipin_00026');
                httpPost('m=neirong&c=announcement', {...params, ...searchForm}, {hideloading: true}).then(function (response) {
                    let res = response.data,
                        data = res.data;
                    that.list = data.list;
                    that.total = parseInt(data.total);
                    that.pageSizes = data.page_sizes;
                    if (that.limit === 0) {
                        that.limit = parseInt(data.limit); // Use default count from system config
                    }
                    if (that.page > data.page) {
                        that.page = parseInt(data.page); // Use latest page after the last page is deleted
                    }
                    if(that.prevPage != that.page){
                        that.prevPage = that.page;
                        that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    that.loading = false;
                    if (that.list.length === 0){
                        that.emptytext = lc('wap_js_00113');
                    }
                })
            },

            // Batch operation
            handleSelectionChange(val) {
                if (val.length == 0) {
                    this.checkedAll = false;
                    this.checkedAllIndeterminate = false;
                } else {
                    if (val.length === this.list.length) {
                        this.checkedAll = true;
                        this.checkedAllIndeterminate = false;
                    } else {
                        this.checkedAll = false;
                        this.checkedAllIndeterminate = true;
                    }
                }
                this.multipleSelection = val;
            },
            batch(type) {
                if (this.multipleSelection.length == 0) {
                    let msg = lc('admin_user_weipin_00001')
                    if (type == 'del') {
                        msg = lc('admin_00136')
                    }
                    message.error(msg);
                    return false;
                }

                let idArr = [];
                this.multipleSelection.forEach(function(item) {
                    idArr.push(item.id);
                })
                this.idArr = idArr;

                if (type == 'del') {
                    this.del();
                } else if (type == 'domain') {
                    this.openDomain();
                }
            },
            checkAll(val) {
                val ? this.checkedAllIndeterminate = false : '';
                this.$refs.multipleTable.toggleAllSelection();
            },

            del(idx) {
                let that = this,
                    params = {},
                    msg = '';

                if (typeof idx == 'undefined') { // Batch delete
                    params.del = this.idArr;
                    msg = lc('common_00853');
                } else {// Single delete
                    params.id = that.list[idx].id;
                    msg = lc('admin_00333');
                }

                delConfirm(this, params, function (params) {
                    httpPost('m=neirong&c=announcement&a=del', params).then(function(res) {
                        if (res.data.error > 0) {
                            message.error(res.data.msg);
                        } else {
                            that.getList();
                            that.$refs.multipleTable.clearSelection();
                            message.success(res.data.msg);
                        }
                    })
                }, msg)
            },

            formatter(row, column) {
                if (column.property == 'datetime') {
                    return row.datetime_n;
                } else if (column.property == 'startime') {
                    return row.startime == '0' ? lc('admin_00147') : row.startime_n;
                } else if (column.property == 'endtime') {
                    return row.endtime == '0' ? lc('admin_00147') : row.endtime_n;
                } else if (column.property == 'did') {
                    return row.dname;
                }
            },

            openAdd(row) {
                let that =this;
                httpPost('m=neirong&c=announcement&a=add', {}).then(function (response) {
                    let res = response.data;
                    that.detail = row == '' ? {} : row;
                    that.drawerAdd = true;
                })
            },

            closeAdd() {
                this.drawerAdd = false;
                this.getList();
            },

            openDomain(row) {
                if (typeof row == 'undefined') { // Batch operation
                    this.detail = {};
                    this.$set(this.ruleFormDomain, 'id', this.idArr);
                    this.$set(this.ruleFormDomain, 'did', '');
                } else { // Single operation
                    this.detail = row;
                    this.$set(this.ruleFormDomain, 'id', row.id);
                    this.$set(this.ruleFormDomain, 'did', row.dname ? '' + row.did + '' : '');
                }

                this.dialogDomain = true;
            },

            saveDomain() {
                let that = this,
                    ruleForm = that.ruleFormDomain;

                if (!ruleForm.did) {
                    message.warning(lc('admin_user_weipin_00002'));
                    return false;
                }

                if (that.saveLoading) {
                    return false;
                }

                that.saveLoading = true;

                httpPost('m=neirong&c=announcement&a=checksitedid', ruleForm).then(function (response) {
                    let res = response.data;

                    that.saveLoading = false;
                    if (res.error > 0) {
                        message.error(res.msg);
                    } else {
                        that.dialogDomain = false;
                        that.getList();
                        that.$refs.multipleTable.clearSelection();
                        message.success(res.msg)
                    }
                })
            },
        }
    }
</script>
