<template>
<div id="daohaapp" class="moduleElenAl">
        <div class="moduleSeachs">
            <div class="moduleSeachInpt">
				<el-input :placeholder="lc('admin_system_00198')" style="margin-right: 8px;" v-model="searchOption.keyword" clearable size="small" prefix-icon="el-icon-search">
					<template #prepend><el-select v-model="searchOption.type" size="small" :placeholder="lc('wap_01431')">
					    <el-option :label="lc('wap_01431')" value="1"></el-option>
					    <el-option :label="lc('admin_system_00199')" value="2"></el-option>
					</el-select></template>
				</el-input>
                <el-select v-model="searchOption.feedbacktype" style="margin-right: 8px;" size="small" clearable @change="search" :placeholder="lc('admin_system_00203')">
                    <el-option :label="lc('common_01983')" value="1"></el-option>
                    <el-option :label="lc('wap_00111')" value="2"></el-option>
                    <el-option :label="lc('wap_00113')" value="3"></el-option>
                    <el-option :label="lc('wap_00112')" value="4"></el-option>
                </el-select>
                <el-select v-model="searchOption.feedbacktime" style="margin-right: 8px;" size="small" clearable :placeholder="lc('admin_system_00200')" @change="search">
                    <el-option :label="lc('common_01940')" value="1"></el-option>
                    <el-option :label="lc('admin_user_00179')" value="3"></el-option>
                    <el-option :label="lc('admin_user_00178')" value="7"></el-option>
                    <el-option :label="lc('admin_user_00180')" value="15"></el-option>
                    <el-option :label="lc('admin_user_00175')" value="30"></el-option>
                </el-select>
                <el-select v-model="searchOption.feedbackstatus" size="small" clearable :placeholder="lc('admin_user_00161')" @change="search">
                    <el-option :label="lc('admin_user_00164')" value="1"></el-option>
                    <el-option :label="lc('admin_user_00163')" value="2"></el-option>
                </el-select>
                <el-button type="primary" size="small" icon="el-icon-search" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table :data="tableData" border style="width: 100%"
                :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" @sort-change="shortChange"
                @selection-change="handleSelectionChange" ref="multipleTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" :label="lc('common_02108')" width="80" sortable="custom">
                </el-table-column>
                <el-table-column prop="infotype_n" :label="lc('admin_system_00203')">
                </el-table-column>
				<el-table-column prop="username" :label="lc('wap_01431')">
				</el-table-column>
                <el-table-column prop="mobile" :label="lc('wap_00109')">
                </el-table-column>
                <el-table-column prop="content" :label="lc('admin_system_00199')">
                    <template #default="scope">
                        <el-popover trigger="hover" placement="top" width="300" v-if="scope.row.content_n">
                            <p>{{scope.row.content}}</p>
                            <template #reference><div>
                                <span class="ellipsis">{{scope.row.content_n}}</span>
                            </div></template>
                        </el-popover>
                        <span v-else>{{scope.row.content}}</span>
                    </template>
                </el-table-column>
                <el-table-column prop="ctime_n" :label="lc('admin_system_00200')">
                </el-table-column>
                <el-table-column :label="lc('admin_user_00161')">
                    <template #default="scope">
                        <el-tag type="success" size="small" v-if="scope.row.status==2">{{ lc('admin_user_00163') }}</el-tag>
                        <el-tag type="danger" size="small" v-else>{{ lc('admin_user_00164') }}</el-tag>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" @click="handle(scope.row)">{{ lc('admin_user_00165') }}</el-button>
                            <el-button size="small" @click="delrow(scope.row)" type="danger">{{ lc('wap_js_00077') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulecz modulePagButn">
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button size="small" @click="delAllBottom">{{ lc('member_com_00055') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                   :current-page="page" :page-sizes="pageSizes" :page-size="limit"
                   layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <!-- 处理弹窗 -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_system_00202')" v-model="handleBox" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="500px">
                <el-form :model="ruleFormHandle" ref="ruleFormHandle" label-width="90px">
                    <el-form-item :label="lc('admin_user_00161')">
                        <el-radio-group v-model="ruleFormHandle.status">
                            <el-radio label="1">{{ lc('admin_user_00164') }}</el-radio>
                            <el-radio label="2">{{ lc('admin_user_00163') }}</el-radio>
                        </el-radio-group>
                    </el-form-item>
                    <el-form-item :label="lc('admin_system_00201')" prop="content">
                        <el-input type="textarea" :rows="2" v-model="ruleFormHandle.content"></el-input>
                    </el-form-item>

                </el-form>
                <template #footer><div class="dialog-footer">
                    <el-button @click="handleBox = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" :loading="save_load" @click="submitForm('ruleFormHandle')">{{ lc('wap_com_00019') }}</el-button>
                </div></template>
            </el-dialog>
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
                    emptytext: window.lc('wap_js_00113'),
                    loading: false,
                    searchOption: {
                        type: '1',
                        feedbacktype: '',
                        feedbacktime: '',
                        feedbackstatus: '',
                        keyword: ''
                    },
                    page: 1,
                    prevPage: 0,
                    limit: 0,
                    list: [],
                    total: 0,
                    pageSizes: [],

                    checkedAll: false,
                    selectedItem: [],
                    handleBox: false,
                    tableData: [],
                    ruleFormHandle: {
                        status: '',
                        content: '',
                        id: ''
                    },

					save_load:false,
                }
            },
            created() {
                var that = this
                let query = window.parent.homeapp.$route.query;
                if (query.status) {
                    that.searchOption.feedbackstatus = query.status;
                }

                this.getList();
            },
            methods: {
                shortChange(e) {
                    let orderMap = {ascending: 'asc', descending: 'desc'}
                    this.searchOption.t = e.order ? e.prop : null;
                    this.searchOption.order = orderMap[e.order];
                    this.search();
                },
                handle(row) {
                    this.ruleFormHandle.status = row.status;
                    this.ruleFormHandle.content = row.handlecontent;
                    this.ruleFormHandle.id = row.id;
                    this.handleBox = true;
                },
                async submitForm(formName) {
                    let that = this,
                        params = this.$data[formName];
					that.save_load = true;
                    httpPost('m=system&c=info_feedback&a=status', params).then(function (data) {
						that.save_load = false;
                        let res = data.data;
                        if (res.error == 0) {
                            message.success(res.msg, function () {
                                that.handleBox = false;
                                that.getList();
                            })
                        } else {
                            message.error(res.msg);
                        }
                    }).catch(function (error) {
                        console.log(error)
                    })
                },
                search() {
                    this.page = 1;
                    this.getList();
                },
                async getList() {
                    let that = this;

                    let searchOption = that.searchOption;
                    let params = {
                            page: that.page,
                            pageSize: that.limit,
                        };
                    that.loading = true;
                    that.emptytext = window.lc('admin_user_weipin_00026');
                    httpPost('m=system&c=info_feedback&a=index', {...params, ...searchOption}, {hideloading: true}).then(function (data) {
                        let res = data.data;
                        if (res.error == 0) {
                            that.tableData = res.data.list;
                            if (that.prevPage != that.page) {
                                that.prevPage = that.page;
                                that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                            }
                            that.loading = false;
                            that.total = parseInt(res.data.total);
                            that.pageSizes = res.data.pageSizes;
                            that.limit = parseInt(res.data.pageSize);
                            if (that.page > res.data.page) {
                                that.page = parseInt(res.data.page); // 最后一页被删除后，取最新的页数
                            }
                            if (that.tableData.length === 0){
                                that.emptytext = window.lc('wap_js_00113');
                            }
                        }
                    }).catch(function (error) {
                        console.log(error)
                    })
                },
                handleSizeChange(val) {
                    this.limit = val;
                    this.getList();
                },
                handleCurrentChange(val) {
                    this.page = val;
                    this.getList();
                },
                selectAllBottom(value) {
                    value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
                },
                handleSelectionChange(val) {
                    this.selectedItem = [];
                    let _this = this;
                    if (val.length) {
                        val.forEach(item => {
                            _this.selectedItem.push(item.id);
                        });
                    }
                    if (_this.selectedItem.length == 0) {
                        _this.checkedAll = false;
                    } else {
                        if (_this.selectedItem.length == _this.tableData.length) {
                            _this.checkedAll = true;
                        } else {
                            _this.checkedAll = false;
                        }
                    }
                },
                delrow(row) {
                    delConfirm(this, row.id, this.delete);
                },
                delAllBottom() {
                    if (!this.selectedItem.length) {
                        this.$message({ showClose: true, message: lc('admin_user_weipin_00005'), type: 'warning' });
                        return false;
                    }
                    delConfirm(this, this.selectedItem, this.delete);
                },
                async delete(Ids) {
                    let _this = this;
                    let params = {
                        del: Ids
                    };
                    httpPost('m=system&c=info_feedback&a=del', params).then(function (response) {
                        if (response.data.error == 0) {
                            message.success(window.lc('wap_user_00264'));
                            _this.getList();
                        } else {
                            message.error(response.data.msg);
                        }
                    }).catch(function (error) {
                        console.log(error);
                    })
                },

            }
        }
</script>
