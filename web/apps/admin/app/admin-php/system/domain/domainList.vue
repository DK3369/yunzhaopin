<template>
<div id="domainListApp" class="moduleElenAl">

    <div class="moduleSeachs">
        <div class="moduleSeachInpt">
            <el-input :placeholder="lc('admin_01053')" v-model="search.keyword" class="input-with-select" size="small" clearable></el-input>
            <el-button type="primary" icon="el-icon-search" size="small" @click="handelSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
        </div>
        <div class="">
            <a href="javascript:;" @click="addDomain">
                <el-button type="primary" icon="el-icon-document-add" size="small">{{ lc('admin_system_00196') }}</el-button>
            </a>
            <a href="javascript:;" @click="domainConfigShow = true">
                <el-button type="primary" icon="el-icon-document-add" size="small" plain>{{ lc('admin_system_00195') }}</el-button>
            </a>
        </div>
    </div>

    <div class="moduleElTable oduleEldujegh">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_system_00189')" type="info" :closable="false"></el-alert>
        </div>

        <el-table :data="tableData" border style="width: 100%;" :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="calc(100% - 48px)" @selection-change="handleSelectionChange" ref="multipleTable" v-loading="loading" :empty-text="emptytext">
            <el-table-column type="selection" width="55"></el-table-column>
            <el-table-column prop="title" :label="lc('admin_system_00166')" width="220"></el-table-column>
            <el-table-column prop="name" :label="lc('admin_system_00193')"></el-table-column>
            <el-table-column prop="city" :label="lc('wap_js_00083')"></el-table-column>
            <el-table-column prop="hy_n" :label="lc('admin_user_company_00373')"></el-table-column>
            <el-table-column prop="style" :label="lc('admin_system_00350')"></el-table-column>
            <el-table-column :label="lc('admin_system_00263')">
                <el-switch #default="scope" v-model="scope.row.typeStatus" active-color="#1890FF" inactive-color="#B8BDC9" @change="changeType(scope)"></el-switch>
            </el-table-column>
            <el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
                <template #default="scope">
                    <div class="cz_button">
                        <el-button size="small" @click="editDomain(scope)">{{ lc('wap_js_00073') }}</el-button>
                        <el-button size="small" @click="delDomain(scope)" type="danger">{{ lc('wap_js_00077') }}</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
    </div>
    <div class="modulePaging">
        <div class="modulecz modulePagButn" style="margin-left: 10px;">
            <el-checkbox :indeterminate="isIndeterminate" v-model="checkAll" @change="handleCheckAllChange">{{ lc('wap_js_00074') }}</el-checkbox>
            <el-button size="small" @click="delDomainSel">{{ lc('member_com_00055') }}</el-button>
        </div>
        <div class="modulePagNum">
            <div class="modulePagNum" style="margin: 0 auto;">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" v-model:current-page="currentPage" v-model:page-size="pageSize" :page-sizes="pageSizes" layout="total, sizes, prev, pager, next, jumper" :total="total"></el-pagination>
            </div>
        </div>
    </div>
    <!-- 分站设置弹窗 -->
    <div class="modluDrawer">
        <el-drawer :title="lc('admin_system_00195')" v-model="domainConfigShow" :modal-append-to-body="false" size="85%">
            <domain-config @child-event="closeDomainConfig"></domain-config>
        </el-drawer>
    </div>
    <!-- 分站添加/修改 弹窗 -->
    <div class="modluDrawer">
        <el-drawer :title="addDomainTitle" v-model="domainAddShow" :modal-append-to-body="false" size="85%">
            <domain-add :domain_id="domainId" @child-event="closeDomainAdd"></domain-add>
        </el-drawer>
    </div>
</div>
</template>

<script>
import DomainAdd from './component/domainAdd.vue'
import DomainConfig from './component/domainConfig.vue'

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
                search: {
                    keyword: null
                },
                domainId: 0,
                tableData: [],

                total: 0,
                currentPage: 1,
                prevPage: 0,
                pageSize: 0,
                pageSizes: [],

                // 批量选择
                checkAll: false,
                isIndeterminate: false,
                selectedItem: [],

                domainConfigShow: false,
                domainAddShow: false,
                addDomainTitle: lc('admin_system_00196'),
            }
        },
        components: {
            'domain-add': DomainAdd,
            'domain-config': DomainConfig,
        },
        created: function () {
            this.getDomainList();


        },
        methods: {
            getDomainList() {
                var that = this;
                var params = JSON.parse(JSON.stringify(this.search));
                params.pageSize = that.pageSize;
                params.page = that.currentPage;
				that.loading = true;
                that.emptytext = lc('admin_user_weipin_00026');
                httpPost('m=system&c=domain_list&a=index', params, {hideloading: true}).then(function (res) {
                    let body = res.data || {};
                    that.loading = false;
                    if (body.error != 0) {
                        that.tableData = [];
                        that.emptytext = body.msg || lc('wap_js_00113');
                        return;
                    }
                    let data = body.data || {};
                    that.tableData = Array.isArray(data.list) ? data.list : [];
                    that.total = data.total;
                    that.pageSize = parseInt(data.pageSize);
                    that.pageSizes = data.pageSizes;
                    if (that.prevPage != that.currentPage) {
                        that.prevPage = that.currentPage;
                        const wrap = that.$refs.multipleTable && that.$refs.multipleTable.bodyWrapper;
                        if (wrap) wrap.scrollTop = 0;
                    }
                    if (that.tableData.length === 0){
                        that.emptytext = lc('wap_js_00113');
                    }
                }).catch(function (error) {
                    that.loading = false;
                    console.log(error);
                })
            },
            handelSearch: function () {
                this.currentPage = 1
                this.getDomainList()
            },
            handleSelectionChange(val) {
                this.selectedItem = val;
                if (this.selectedItem.length == 0) {
                    this.isIndeterminate = false;
                    this.checkAll = false;
                } else {
                    if (this.selectedItem.length == this.tableData.length) {
                        this.isIndeterminate = false;
                        this.checkAll = true;
                    } else {
                        this.isIndeterminate = true;
                        this.checkAll = false;
                    }
                }
            },
            handleCheckAllChange(val) {
                val ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
            },
            changeType(e) {
                let _this = this;
                let typeStatusVal = !e.row.typeStatus ? 2 : 1;
                let typeStatusBefore = e.row.type;
                if (typeStatusBefore == typeStatusVal) {
                    return;
                }
                let param = {type: typeStatusVal, id: e.row.id};
                httpPost('m=system&c=domain_list&a=changeDomainType', param).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        _this.getDomainList()
                        message.success(res.msg);
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                })
            },
            addDomain() {
                var self = this;
                self.domainId = 0;
                self.addDomainTitle = lc('admin_system_00196');
                self.domainAddShow = true;
            },
            editDomain(scope) {
                var self = this;
                self.domainId = parseInt(scope.row.id);
                self.addDomainTitle = lc('admin_system_00194');
                self.domainAddShow = true;
            },
            delDomain(scope, isMore) {
                var that = this;
                let name = '';
                let idArr = [], nameArr = [];
                let params = {};
                if (isMore) {
                    this.selectedItem.forEach((item) => {

                        idArr.push(item.id);
                        nameArr.push(item.title);
                    });
                    name = '（' + nameArr.join('，') + '）';
                    params.id = idArr;
                } else {

                    name = '（' + scope.row.title + '）';
                    params.id = scope.row.id;
                }

                delConfirm(this, params, this.delete, lc('admin_system_00172') + name + lc('admin_system_00192'));
            },
            delDomainSel() {
                var that = this;
                if (!that.selectedItem.length) {
                    message.error(lc('admin_system_00190'));
                    return;
                }
                this.delDomain(null, true);
            },
            delete(params) {
                var self = this;
                httpPost('m=system&c=domain_list&a=delDomain', params).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg, function () {
                            self.getDomainList();
                        });
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                })
            },
            handleSizeChange(val) {

                this.pageSize = val;
                this.getDomainList();
            },
            handleCurrentChange(val) {

                this.currentPage = val;
                this.getDomainList();
            },
            closeDomainConfig() {
                this.domainConfigShow = false;
                this.getDomainList();
            },
            closeDomainAdd() {
                this.domainAddShow = false;
                this.getDomainList();
            }
        }
    }
</script>
