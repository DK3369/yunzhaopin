<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-input :placeholder="lc('admin_00446')" size="small" @keyup.enter="doUserQuery" v-model="search.keyword" class="input-with-select" clearable>
                    <template #prepend><el-select v-model="search.type" :placeholder="lc('admin_user_00140')">
                        <el-option :label="lc('admin_user_00140')" value="1"></el-option>
                        <el-option :label="lc('admin_user_00130')" value="3"></el-option>
                    </el-select></template>
                </el-input>
            </div>
            <div class="tableSeachInpt">
                <el-input :placeholder="lc('wap_user_00076')" @keyup.enter="doUserQuery" size="small" prefix-icon="el-icon-search" v-model="search.content" clearable>
                </el-input>
            </div>
            <div class="tableSeachInpt tableSeachInptsmalltwo">
                <el-date-picker v-model="search.time" type="daterange" :range-separator="lc('admin_company_00019')" :start-placeholder="lc('admin_00343')" value-format="YYYY-MM-dd" :end-placeholder="lc('admin_00344')" size="small" @change="doUserQuery"></el-date-picker>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-select v-model="search.operas" size="small" :placeholder="lc('admin_user_00155')" clearable @change="doUserQuery">
                    <el-option v-for="(value, key) in operasArr" :key="key" :label="value" :value="key"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-select v-model="search.parrs" size="small" :placeholder="lc('wap_com_00030')" clearable @change="doUserQuery">
                    <el-option :label="lc('admin_user_00156')" value="1"></el-option>
                    <el-option :label="lc('wap_js_00073')" value="2"></el-option>
                    <el-option :label="lc('wap_js_00077')" value="3"></el-option>
                    <el-option :label="lc('admin_company_00020')" value="4"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-select v-model="search.end" size="small" :placeholder="lc('member_user_00241')" clearable @change="doUserQuery">
                    <el-option v-for="(item, key) in time" :key="key" :label="item.label" :value="item.value"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="small" @click="doUserQuery">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable moduleElTabUselod"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe @selection-change="selectChange" ref="multipleTable"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" @sort-change="shortChange"
                v-loading="loading">
                <template #empty>
                    <p>{{ dataText }}</p>
                </template>
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="uid" :label="lc('admin_user_00130')" width="110" sortable="custom"></el-table-column>
                <el-table-column prop="username" :label="lc('admin_user_00140')" width="150"></el-table-column>
                <el-table-column prop="zzh" :label="lc('wap_00529')"  width="100" show-overflow-tooltip>
                    <template #default="scope">
                        <el-button type="text" @click="handlePreview(scope)" style="padding: 0">{{
                            scope.row.rname
                        }}
                        </el-button>
                    </template>
                </el-table-column>
                <el-table-column prop="neirong" :label="lc('wap_user_00102')" min-width="180" show-overflow-tooltip>
                    <template #default="scope">
                        {{ scope.row.content }}
                        <template v-if="scope.row.sub_n">
                            ；{{ scope.row.sub_n }}
                        </template>
                    </template>
                </el-table-column>
                <el-table-column prop="ip" label="IP" width="130"></el-table-column>
                <el-table-column prop="ctime_ymd" :label="lc('wap_js_00088')" width="170" sortable="custom"></el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="80" fixed="right">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="small" @click="del(scope.row)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="batchDel" size="small">{{ lc('member_com_00055') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination :total="total" @current-change="userPageChange" :page-size="pageSize" :page-sizes="pageSizes"
                    @size-change="handleSizeChange" v-model:current-page="page"
                    layout="total, sizes, prev, pager, next, jumper">
                </el-pagination>
            </div>
        </div>
        <div class="modluDrawer">
            <el-drawer :title="lc('member_user_00037')" :append-to-body="true" v-model="resumePreviewVisible" :destroy-on-close="true"
                size="530px">
                <resume_preview :id="info.eid" :uid="info.uid"></resume_preview>
            </el-drawer>
        </div>
    </div>
</template>
    
<script>
import ResumePreview from '../../../component/resume_preview.vue'

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
        typelist: Array,
        time: Array,
    },
    data: function () {
        return {
            loading: false,
            dataText: lc('admin_user_weipin_00026'),

            checkedAll: false,
            
            tableData: [],
            search: {
                type: '1',
                content: '',
                keyword: '',
                parrs: '',
                end: "",
                time: "",
                operas: ''
            },
            operasArr: {
                88: lc('admin_user_00157'),
                2: lc('common.resume'),
                6: lc('wap_00574'),
                5: lc('wap_user_00193'),
                7: lc('wap_00456'),
                11: lc('admin_user_00152'),
                8: lc('member_user_00226'),
                12: lc('member_com_00093'),
                16: lc('wap_js_00081'),
                17: lc('common_06524'),
                18: lc('common.message'),
                19: lc('wap_user_00223'),
                23: lc('wap_com_00350'),
                25: lc('admin_user_00154'),
                26: lc('wap_user_00221')
            },
            items: [
                { type: '', label: lc('admin_user_00149') },
            ],
            idsArr: [],
            total: 0,
            page: 1,
            pageSize: 0,
            pageSizes: [],
            uri: "m=user&c=",
            info: {},
            resumePreviewVisible: false,

            prevPage: 0

        }
    },
    created() {
        this.getList();

    },
    components: {
        'resume_preview': ResumePreview,
    },
    methods: {
        shortChange(e) {
            let orderMap = { ascending: 'asc', descending: 'desc' }
            this.search.t = e.prop == 'ctime_ymd' ? 'ctime' : e.prop;
            this.search.order = orderMap[e.order];
            this.page = 1;
            this.getList();
        },
        selectChange: function (val) {
            this.idsArr = [];
            let _this = this;
            if (val.length) {
                val.forEach(item => {
                    _this.idsArr.push(item.id);
                });
            }
            if (_this.idsArr.length == 0) {
                _this.checkedAll = false;
            } else {
                if (_this.idsArr.length == _this.tableData.length) {
                    _this.checkedAll = true;
                } else {
                    _this.checkedAll = false;
                }
            }
        },
        doUserQuery() {
            this.page = 1
            this.getList()
        },
        userPageChange(val) {
            this.page = val
            this.getList()
        },
        handleSizeChange(val) {
            this.pageSize = val;
            this.getList();
        },
        getList: function () {
            let _this = this;
            let url = _this.uri + 'admin_memberlog&a=index';
            _this.search.page = this.page;
            _this.search.pageSize = this.pageSize;
            _this.loading = true;
            httpPost(url, _this.search, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.tableData = res.data.data;
                    _this.total = res.data.total;
                    _this.loading = false;
                    _this.pageSizes = res.data.pageSizes;
                    if(_this.prevPage != _this.page){
                        _this.prevPage = _this.page;
                        _this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    if (_this.tableData.length === 0) {
                        _this.dataText = lc('wap_js_00113');
                    }
                }
            })
        },
        del: function (detail) {
            let _this = this,
                params = {};
            params.del = detail.id;
            let url = this.uri + 'admin_memberlog&a=delLog';
            let msg = lc('admin_vue_00028');
            delConfirm(_this, params, function (params) {
                httpPost(url, params).then(function (res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.success(res.data.msg, function () {
                            _this.getList();
                        });
                    }
                })
            }, msg);
        },
        batchDel: function () {
            let ids = this.idsArr;
            if (!ids.length) {
                message.error(lc('admin_vue_00030'));
                return
            }
            let _this = this,
                params = {};
            params.del = ids;
            let url = this.uri + 'admin_memberlog&a=delLog'
            let msg = lc('admin_vue_00028');
            delConfirm(_this, params, function (params) {
                httpPost(url, params).then(function (res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.success(res.data.msg, function () {
                            _this.getList();
                        });
                    }
                })
            }, msg);
        },
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
        handlePreview(scope) {
            this.info = scope.row;
            console.log(scope.row);
            this.resumePreviewVisible = true;
        },
    },
};
</script>
<style scoped>
.moduleElHight .moduleElTable {
    padding: 0;
    margin: 0;
    /* height: calc(100% - 92px); */
    width: 100%;
}
.tableSeachInptsmall .el-input {
    width: initial;
}

.tableSeachInptsmall .el-select {
    margin-right: 0px !important;
}

.el-input-group__prepend {
    background-color: #ffffff;
    padding: 0 0 0 20px;
}
.moduleElTabUselod{
    height: calc(100% - 95px);
}
@media (max-width: 1510px) {
    .moduleElTabUselod {
        height: calc(100% - 136px) !important;
    }
}
</style> 