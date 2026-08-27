<template>
<div id="daohaapp" class="moduleElenAl">
    <div class="moduleSeachs">
        <div class="moduleSeachleft">
            <div class="moduleInptList">
                <el-input :placeholder="lc('admin_user_weipin_00003')" v-model="searchForm.keyword" size="small" class="input-with-select" clearable>
                    <template #prepend><el-select v-model="searchForm.type" size="small" :placeholder="lc('member_com_00345')">
                        <el-option :label="lc('member_com_00345')" value="1"></el-option>
                        <el-option :label="lc('member_com_00021')" value="2"></el-option>
                    </el-select></template>
                </el-input>
            </div>
            <div class="newsbtnbox">
                <el-button type="primary" icon="el-icon-search" size="small" @click="handleSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
        <div class="moduleSeachButn">
            <el-button type="primary" icon="el-icon-document-add" size="small" @click="handleAdd">{{ lc('admin_00222') }}</el-button>
        </div>
    </div>
    <div class="moduleElTable">
        <el-table :data="tableData" border style="width: 100%" :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="shortChange" v-loading="loading">
            <template #empty>
                <p>{{dataText}}</p>
            </template>
            <el-table-column type="selection" width="55"></el-table-column>
            <el-table-column prop="id" :label="lc('member_com_00345')" width="90"></el-table-column>
            <el-table-column prop="class_name" :label="lc('admin_system_00357')"></el-table-column>
            <el-table-column prop="place_n" :label="lc('admin_01174')" width="200"></el-table-column>
            <el-table-column :label="lc('admin_yunying_00054')" width="100">
                <template #default="scope">
                    <div v-if="scope.row.hrefn" class="moduleElImage">
                        <el-image :src="scope.row.hrefn ? scope.row.hrefn : ''" :preview-src-list="scope.row.hrefn ? [scope.row.hrefn] : []"></el-image>
                    </div>
                    <span v-else>{{ lc('common_02082') }}</span>
                </template>
            </el-table-column>
            <el-table-column prop="orders" :label="lc('admin_vue_00044')" sortable="custom" width="100">
                <template #default="scope">
                    <el-input type="number" v-if="scope.row[scope.column.property + 'isShow']" :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index" v-model="scope.row.orders" @blur="alterData(scope)"></el-input>
                    <span v-else>{{ scope.row.orders }}<img @click="editData(scope)" class="editIcon" src="/admin/php-admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14"></span>
                </template>
            </el-table-column>
            <el-table-column :label="lc('admin_yunying_00056')" width="120">
                <template #default="scope">
                    <el-link type="primary" :underline="false" @click="handleCopy(scope)">{{ lc('admin_yunying_00053') }}</el-link>
                </template>
            </el-table-column>
            <el-table-column prop="type" :label="lc('admin_yunying_00055')" width="130">
                <template #default="scope">
                    <template v-if="scope.row.type == 1">
                        <el-link type="primary" :underline="false" @click="handleDelbuy(scope)">{{ lc('wap_js_00080') }}</el-link>
                    </template>
                    <template v-else>
                        <el-link type="primary" :underline="false" @click="handleBuy(scope)">{{ lc('admin_yunying_00055') }}</el-link>
                    </template>
                </template>
            </el-table-column>
            <el-table-column fixed="right" :label="lc('member_user_00048')" width="130" align="center">
                <template #default="scope">
                    <div class="cz_button">
                        <el-button size="small" @click="editRow(scope)" type=" ">{{ lc('wap_js_00073') }}</el-button>
                        <el-button type="danger" size="small " @click="deleteRow(scope)">{{ lc('wap_js_00077') }}</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
    </div>
    <div class="modulePaging">
        <div class="modulecz modulePagButn">
            <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
            <el-button @click="deleteRow(null, true)">{{ lc('member_com_00055') }}</el-button>
        </div>
        <div class="modulePagNum">
            <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" v-model:current-page="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes" layout="total, sizes, prev, pager, next, jumper" :total="total"></el-pagination>
        </div>
    </div>
    <!--添加类别 修改-->
    <div class="modluDrawer">
        <el-drawer :title="titleAddEdit" v-model="addVisible" :modal-append-to-body="false" :show-close="true" :before-close="handleClose" :destroy-on-close="true" size="35%">
            <ad_class_edit :info="info" :integral_pricename="integral_pricename" :pic_maxsize="pic_maxsize" :pic_type="pic_type" :change-to-buy="changeToBuy" @child-event-list="handleCloseList"></ad_class_edit>
        </el-drawer>
    </div>
    <!--内部调用-->
    <el-dialog :title="lc('admin_yunying_00053')" width="40%" v-model="copyVisible" :modal-append-to-body="false">
        <div>
            <el-input id="elementCode" type="textarea" rows="5" v-model="code"></el-input>
            <i class="el-icon-info" style="margin-top: 10px;">{{ lc('admin_yunying_00052') }}</i>
            <i class="el-icon-info" style="margin-top: 10px;">{{ lc('admin_yunying_00051') }}</i>
        </div>
        <template #footer><div class="dialog-footer">
            <el-button id="copyBtn" type="primary" data-clipboard-action="copy" data-clipboard-target="#elementCode" @click="handleCopyValue">{{ lc('admin_yunying_00073') }}</el-button>
        </div></template>
    </el-dialog>
</div>
</template>

<script>
import AdClassEdit from './component/ad_class_edit.vue'

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
                    type: '1',
                    page: 1,
                    limit: null,
                },
                total: 0,
                tableData: [],
                pageSizes: [],
                checked: false,//全选
                isIndeterminate: false,// checkbox 的不确定状态
                selectedItem: [],
                addVisible: false,
                titleAddEdit: lc('admin_01175'),
                info: {},
                integral_pricename: '',
                pic_maxsize: '',
                pic_type: '',
                copyVisible: false,
                code: '',
                changeToBuy: false,//设为可购买广告类别
                prevPage:0
            }
        },
        components: {
            'ad_class_edit': AdClassEdit,
        },
        created: function () {
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
            shortChange(e) {
                let orderMap = {ascending: 'asc', descending: 'desc'}
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
                this.searchForm.page = 1;
                this.getList();
            },
            getList() {
                let _this = this;
                let params = JSON.parse(JSON.stringify(this.searchForm));
                for (let index in params) {
                    (params[index] === '') && (params[index] = null);
                }
                _this.loading = true;
                httpPost('m=yunying&c=ad_class&a=index', params, {hideloading: true}).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        _this.integral_pricename = res.data.integral_pricename;
                        _this.pic_maxsize = res.data.pic_maxsize;
                        _this.pic_type = res.data.pic_type;
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
            handleAdd() {

                this.titleAddEdit = lc('admin_01175');
                this.info = {};
                this.addVisible = true;
            },
            handleCloseList() {
                this.addVisible = false;
                this.getList();
            },
            handleClose(done) {
                done();
                this.addVisible = false;
                this.changeToBuy = false;
            },
            editRow(scope) {
                this.titleAddEdit = lc('admin_01176');
                this.info = scope.row;
                this.addVisible = true;
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
                    params.delType = 'more';
                    params.del = list;
                } else {
                    // let index = scope.$index;
                    // this.tableData.splice(index, 1);
                    params.delType = 'single';
                    params.id = scope.row.id;
                }

                delConfirm(this, params, this.delete);
            },
            delete(params) {
                let _this = this;
                httpPost('m=yunying&c=ad_class&a=del', params).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        message.success(lc('admin_user_00187'));
                        _this.getList();
                    } else {
                        message.error(res.msg);
                        
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
                sendData[column.property] = row[column.property];
                httpPost('m=yunying&c=ad_class&a=upsort', sendData).then(function (response) {
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
            /**
             * 内部调用
             */
            handleCopy(scope) {
                let row = scope.row;
                this.copyVisible = true;
                this.code = '{yun\:}adlist classid=' + row.id + ' limit=5 item=adlist_' + row.id + '{/yun}\r\n{yun\:}$adlist_' + row.id + '.html{/yun}\r\n{yun\:}/adlist{/yun}';
            },
            /**
             * 复制代码
             */
            handleCopyValue() {
                let clipboard = new ClipboardJS('#copyBtn'); // 获取点击按钮的元素
                /* 注意此事件监听是异步的   */
                clipboard.on('success', (e) => {
                    e.clearSelection();
                    // 释放内存
                    clipboard.destroy();
                    message.success(lc('admin_user_company_00368'));
                });
                // 复制失败
                clipboard.on('error', (e) => {
                    // 释放内存
                    clipboard.destroy();
                    message.error(lc('admin_user_company_00339'));
                });
            },
            /**
             * 设为购买
             */
            handleBuy(scope) {
                this.titleAddEdit = lc('admin_01177');
                this.info = scope.row;
                this.addVisible = true;
                this.changeToBuy = true;
            },
            /**
             * 取消购买
             */
            handleDelbuy(scope) {
                let params = {};
                params.id = scope.row.id;

                delConfirm(this, params, this.delbuy, lc('admin_01178'));
            },
            /**
             * 取消购买
             */
            delbuy(params) {
                let _this = this;
                httpPost('m=yunying&c=ad_class&a=delbuy', params).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        message.success(res.msg);
                        _this.getList();
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                });
            },
        }
    }
</script>
