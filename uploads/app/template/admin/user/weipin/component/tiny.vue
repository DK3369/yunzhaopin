<template>
    <div class="moduleElHight">
        <div class="moduleElSearchInf">
            <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                <div class="moduleInptList moduleInptWidt">
                    <el-input size="small" placeholder="{yun:}t key='admin_user_weipin_00003'{/yun}" @keyup.enter.native="search" v-model="searchForm.keyword" clearable class="input-with-select">
                        <el-select v-model="searchForm.type" slot="prepend" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                            <el-option label="{yun:}t key='admin_00429'{/yun}" :value="1"></el-option>
                            <el-option label="{yun:}t key='wap_com_00353'{/yun}" :value="2"></el-option>
                            <el-option label="{yun:}t key='wap_user_00241'{/yun}" :value="3"></el-option>
                        </el-select>
                    </el-input>
                </div>
                <div v-for="(searchItem, searchIndex) in searchList" :key="searchIndex" class="moduleInptList">
                    <el-select size="small" v-model="searchForm[searchItem.param]" slot="prepend" :clearable="true"
                        :placeholder="searchItem.name" @change="search">
                        <el-option v-for="(searchLabel, searchValue) in searchItem.value" :key="searchValue" :label="searchLabel"
                            :value="searchValue"></el-option>
                    </el-select>
                </div>
                <div class="moduleInptList">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                </div>
            </div>
            <!--<div class="moduleSeachButn moduleSeachMart">-->
            <!--    <span>总数：29</span>-->
            <!--    <span>未审核：2</span>-->
            <!--    <span>搜索结果：29</span>-->
            <!--</div>-->
        </div>
        <div class="admin_datatip">
            <i class="el-icon-document"></i>
            {{ lc("admin_data_stats") }} <span @click="init">{{ lc("admin_total_count", [tinyAllNum]) }}</span>
            <span class="admin_datatip_n"><span @click="statusSearch('2')">{{ lc("admin_pending_review_count", [tinyStatusNum ? tinyStatusNum : 0]) }}</span> </span>
            <span class="admin_datatip_n">{{ lc("admin_search_results_count", [total]) }}</span>
        </div>
        <div class="moduleElTable moduleElMoreInt" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="list" border style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
                @sort-change="sortChange" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" label="{yun:}t key='member_com_00345'{/yun}" width="90" sortable="custom">
                </el-table-column>
                <el-table-column label="{yun:}t key='wap_00529'{/yun}" min-width="180">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span><el-link :underline="false" @click="openDetail(scope.row)" type="primary">{{
                                scope.row.username }}</el-link></span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="sex_n" label="{yun:}t key='wap_com_00303'{/yun}" width="140">
                </el-table-column>
                <el-table-column prop="exp_n" label="{yun:}t key='wap_00526'{/yun}" width="140">
                </el-table-column>
                <el-table-column prop="job" label="{yun:}t key='wap_com_00353'{/yun}" min-width="160">
                </el-table-column>
                <el-table-column prop="mobile" label="{yun:}t key='wap_01557'{/yun}" width="140">
                </el-table-column>
                <el-table-column prop="time" label="{yun:}t key='admin_user_weipin_00030'{/yun}" width="140" sortable="custom">
                    <template slot-scope="scope">
                        <div>{{ scope.row.time_n }}</div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='member_user_00181'{/yun}" width="120">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class="admin_state">
                                <span v-if="scope.row.status == 1" class="admin_state1">{yun:}t key='wap_user_00165'{/yun}</span>
                                <span v-else-if="scope.row.status == 0" class="admin_state5">{yun:}t key='wap_user_00166'{/yun}</span>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_user_weipin_00050'{/yun}" width="140">
                    <template slot-scope="scope">
                        <div class="moduleProps moduleTrButn">
                            <span>{{ domainList[scope.row.did] }}</span>
                            <el-button type="text" @click="openDomain(scope.row)">{yun:}t key='admin_user_weipin_00048'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" label="{yun:}t key='member_user_00048'{/yun}" width="140">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <el-button size="mini" style="margin-right: 8px;" @click="openAudit(scope.row)">{yun:}t key='member_user_00152'{/yun}</el-button>
                            <el-popover placement="bottom" width="60" trigger="hover">
                                <div class="moduleMores">
                                    <el-button size="mini" @click="openDetail(scope.row)">{yun:}t key='wap_com_00427'{/yun}</el-button>
                                    <el-button size="mini" @click="openAdd(scope.row)">{yun:}t key='wap_js_00073'{/yun}</el-button>
                                    <el-button size="mini" @click="refresh(scope.row)">{yun:}t key='wap_user_00334'{/yun}</el-button>
                                    <el-button size="mini" @click="del(scope.$index)" type="danger">{yun:}t key='common.delete'{/yun}</el-button>
                                </div>
                                <el-button size="mini" slot="reference" @click="visible = !visible">{yun:}t key='common.more'{/yun}</el-button>
                            </el-popover>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
                    @change="checkAll">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button @click="batch('del')" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
                <el-button @click="batch('audit')" size="mini">{yun:}t key='admin_user_weipin_00037'{/yun}</el-button>
                <el-button @click="batch('audit_cancel')" size="mini">{yun:}t key='admin_user_weipin_00018'{/yun}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page="page" :page-sizes="pageSizes" :page-size="limit"
                    layout="total, sizes, prev, pager, next, jumper" :total="total" :pager-count="pagerCount">
                </el-pagination>
            </div>
        </div>
        <!-- 弹窗 -->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_user_weipin_00029'{/yun}" :visible.sync="dialogDomain" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="500px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='member_user_00091'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <span>{{ detail.username }}</span>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_user_weipin_00020'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-select v-model="ruleFormDomain.did" filterable placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                <el-option v-for="(item, key) in domainList" :key="key" :label="item" :value="key">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogDomain = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="saveDomain">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_user_weipin_00032'{/yun}" :visible.sync="dialogAudit" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="500px">
                <div class="toolClasDia fenpeizhand">
                    <div class="zhaopTacOne">
                        <div class="zhaopTacTwo">
                            <div>
                                <span>{yun:}t key='admin_00422'{/yun}</span>
                            </div>
                            <div>
                                <span>{yun:}t key='admin_00423'{/yun}</span>
                            </div>
                            <div>
                                <span>{yun:}t key='admin_00424'{/yun}</span>
                            </div>
                            <div>
                                <span>{yun:}t key='admin_00425'{/yun}</span>
                            </div>
                            <div>
                                <span>{yun:}t key='admin_00426'{/yun}</span>
                            </div>
                            <div>
                                <span>{yun:}t key='admin_00427'{/yun}</span>
                            </div>
                            <div>
                                <span>{yun:}t key='admin_00428'{/yun}</span>
                            </div>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_user_weipin_00065'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-radio v-model="ruleFormAudit.status" label="1">{yun:}t key='wap_user_00165'{/yun}</el-radio>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAudit = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="saveAudit">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <div class="modluDrawer">
            <el-drawer title="{yun:}t key='member_com_00028'{/yun}" :visible.sync="drawerDetail" :modal-append-to-body="false" size="640px">

                <div class="tableTancyans">
                    <div class="zhaopTacTwo">
                        <table border="1">
                            <tbody>
                                <tr>
                                    <td width="25%">
                                        <div class="TableTite">{yun:}t key='wap_00529'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.username }}</div>
                                    </td>
                                    <td width="25%">
                                        <div class="TableTite">{yun:}t key='wap_com_00303'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.companyname }}</div>
                                    </td>
                                </tr>

                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='wap_00526'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.exp_n }}</div>
                                    </td>
                                    <td>
                                        <div class="TableTite">{yun:}t key='wap_com_00353'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">
                                            {{ detail.job }}
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='wap_00349'{/yun}</div>
                                    </td>
                                    <td colspan="3">
                                        <div class="TableData">
                                            {{ detail.city_one }}{{ detail.city_two }}{{ detail.city_three }}
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='member_user_00163'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.mobile }}</div>
                                    </td>
                                    <td>
                                        <div class="TableTite">{yun:}t key='wap_js_00088'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.time_n }}</div>
                                    </td>
                                </tr>

                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='wap_00527'{/yun}</div>
                                    </td>
                                    <td colspan="3">
                                        <div class="TableData">
                                            {{ detail.production }}
                                        </div>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    <div class="drawerModLis" style="justify-content: initial;">
                        <div class="drawerModTite" style="width: initial;">
                            <span>{yun:}t key='wap_com_00320'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <div class="admin_state">
                                <span v-if="detail.status == 1" class="admin_state1">{yun:}t key='wap_user_00165'{/yun}</span>
                                <span v-else class="admin_state5">{yun:}t key='wap_user_00166'{/yun}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </el-drawer>
            <el-drawer title="{yun:}t key='wap_js_00066'{/yun}" :visible.sync="drawerAdd" :modal-append-to-body="false" size="40%">
                <div class="moduleSchools">
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_00529'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.username" placeholder="{yun:}t key='wap_user_00234'{/yun}"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_com_00303'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-select v-model="ruleFormAdd.sex" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                <el-option v-for="(sexitem, sexkey) in user_sex" :key="sexkey" :label="sexitem"
                                    :value="sexkey">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_00526'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-select v-model="ruleFormAdd.exp" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                <el-option v-for="worditem in user_word" :key="worditem.id" :label="worditem.name"
                                    :value="worditem.id">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_00349'{/yun}</span>
                        </div>
                        <div class="drawerModInpt" style="display: flex; align-items: center;">
                            <el-select v-model="ruleFormAdd.provinceid" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="changeCity($event, 1)">
                                <el-option v-for="item in provinceList" :key="item.id" :label="item.name" :value="item.id">
                                </el-option>
                            </el-select>
                            <el-select v-model="ruleFormAdd.cityid" placeholder="{yun:}t key='wap_user_00100'{/yun}" style="margin: 0 10px;"
                                @change="changeCity($event, 2)">
                                <el-option v-for="item in cityList" :key="item.id" :label="item.name" :value="item.id">
                                </el-option>
                            </el-select>
                            <el-select v-model="ruleFormAdd.three_cityid" placeholder="{yun:}t key='wap_user_00100'{/yun}" clearable>
                                <el-option v-for="item in regionList" :key="item.id" :label="item.name" :value="item.id">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_com_00353'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.job" placeholder="{yun:}t key='admin_user_weipin_00062'{/yun}"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='member_user_00163'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.mobile" placeholder="{yun:}t key='wap_user_00142'{/yun}"
                                @input="inputIntNumber($event, 'ruleFormAdd', 'mobile')"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_00527'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input type="textarea" :rows="2" v-model="ruleFormAdd.production" placeholder="{yun:}t key='admin_user_weipin_00064'{/yun}">
                            </el-input>
                        </div>
                    </div>
                    <div class="drawerModLis" style="align-items: initial;">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_user_00371'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.password" show-password
                                @input="inputPassword($event, 'ruleFormAdd', 'password')"></el-input>
                            <el-alert title="{yun:}t key='admin_user_weipin_00007'{/yun}" :closable="false" type="info" show-icon>
                            </el-alert>
                        </div>
                    </div>
                    <div class="setBasicButn" style="border: none;">
                        <el-button type="primary" size="medium" @click="saveAdd" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
                    </div>
                </div>
            </el-drawer>
        </div>
    </div>
</template>

<script>
module.exports = {
    props: {
        status: { type: String, default: '' }
    },
    data: function () {
        return {
            loading: false,
			pagerCount: 5,
            dataText: "{yun:}t key='admin_user_weipin_00026'{/yun}",
            // 搜索筛选项
            searchList: [],
            searchForm: {
                type: 2,
                status: this.status,
            },

            // list
            page: 1,
            limit: 0,
            list: [],
            total: 0,
            pageSizes: [],

            // 列表排序
            t: '',
            order: '",

            checkedAll: false, // {yun:}t key='wap_js_00074'{/yun}
            checkedAllIndeterminate: false,
            multipleSelection: [], // 多选值存储
            idArr: [],

            detail: {},

            tinyStatusNum: 0,
            tinyAllNum: 0,

            saveLoading: false,

            // 分站切换
            dialogDomain: false,
            ruleFormDomain: {},
            domainList: {},

            // Audit
            dialogAudit: false,
            ruleFormAudit: {},

            // {yun:}t key='admin_user_weipin_00038'{/yun}
            dialogExtension: false,
            ruleFormExtension: {},

            // {yun:}t key='wap_com_00427'{/yun}
            drawerDetail: false,

            // {yun:}t key='wap_js_00091'{/yun}/{yun:}t key='common.edit'{/yun}
            drawerAdd: false,
            ruleFormAdd: {},
            user_sex: {},
            user_word: [],
            provinceList: [],
            cityList: [],
            regionList: [],

            visible: false,

            prevPage: 0
        }
    },

    mounted() {
        var that = this
        setTimeout(function () {
            that.getCacheFun();
        }, 200)
    },
    created() {
        this.init();
    },
    methods: {
        init() {
            // this.resetSearch();
            this.getCountData();
            this.search();
        },
        getCacheFun:function(){
            let that = this;
            httpPost("m=user&c=weipin_tiny&a=getCache', {},{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.searchList = res.data.search_list;
                    that.domainList = res.data.dname;
                }
            })
        },
        resetSearch() {
            this.searchForm = {
                type: 2
            };
            this.limit = 0;
        },

        statusSearch(status) {
            this.searchForm.status = status;
            this.search();
        },

        getCountData() {
            let that = this;

            httpPost('m=user&c=weipin_tiny&a=tinyNum', {}, { hideloading: true }).then(function (response) {
                let res = response.data;

                that.tinyStatusNum = res.tinyStatusNum;
                that.tinyAllNum = res.tinyAllNum;
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
        sortChange(event) {
            this.t = event.order ? event.prop : '';
            this.order = event.order ? event.order == 'descending' ? 'desc' : 'asc' : '';
            this.search();
        },
        search() {
            this.page = 1;
            this.getList();
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
            httpPost('m=user&c=weipin_tiny', { ...params, ...searchForm }, {hideloading: true}).then(function (response) {
                let res = response.data,
                    data = res.data;

                that.list = data.list;
                that.total = parseInt(data.total);
                that.pageSizes = data.page_sizes;
                if (that.limit === 0) {
                    that.limit = parseInt(data.limit); // 取系统配置默认数量
                }
                if (that.page > data.page) {
                    that.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
                }
                that.loading = false;
                if(that.prevPage != that.page){
                    that.prevPage = that.page;
                    that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                }
                if (that.list.length === 0) {
                    that.dataText = "{yun:}t key='wap_js_00113'{/yun}";
                }
            })
        },

        // 批量操作
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
            let that = this;
            if (this.multipleSelection.length == 0 && type == 'del') {
                message.error("{yun:}t key='admin_user_weipin_00005'{/yun}");
                return false;
            }else if (this.multipleSelection.length == 0){
                message.error("{yun:}t key='admin_user_weipin_00001'{/yun}");
                return false;
            }

            let idArr = [];
            this.multipleSelection.forEach(function (item) {
                idArr.push(item.id);
            })
            this.idArr = idArr;

            if (type == 'del') {
                this.del();
            } else if (type == 'audit') {
                delConfirm(this, null, function (params) {
                    that.ruleFormAudit = {
                        id: that.idArr,
                        status: 1,
                    };
                    that.saveAudit();
                }, "{yun:}t key='admin_user_weipin_00011'{/yun}")
            } else if (type == 'audit_cancel') {
                delConfirm(this, null, function (params) {
                    that.ruleFormAudit = {
                        id: that.idArr,
                        status: 0,
                    };
                    that.saveAudit();
                }, "{yun:}t key='admin_user_weipin_00004'{/yun}")
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

            if (typeof idx == 'undefined') { // {yun:}t key='member_com_00055'{/yun}
                params.del = this.idArr;
                msg = lc('common_00853');
            } else {// {yun:}t key='common_01711'{/yun}
                params.del = that.list[idx].id;
                msg = lc('admin_00333');
            }

            delConfirm(this, params, function (params) {
                httpPost('m=user&c=weipin_tiny&a=del', params).then(function (res) {
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

        openDomain(row) {
            if (typeof row == 'undefined') { // {yun:}t key='admin_yunying_00106'{/yun}
                this.detail = {};
                this.$set(this.ruleFormDomain, "id', this.idArr);
                this.$set(this.ruleFormDomain, 'did', '');
            } else { // 单个操作
                this.detail = row;
                this.$set(this.ruleFormDomain, 'id', row.id);
                this.$set(this.ruleFormDomain, 'did', row.did ? '' + row.did  : '');
            }

            this.dialogDomain = true;
        },

        saveDomain() {
            let that = this,
                ruleForm = that.ruleFormDomain;

            if (!ruleForm.did) {
                message.error("{yun:}t key='admin_user_weipin_00002'{/yun}");
                return false;
            }

            if (that.saveLoading) {
                return false;
            }

            that.saveLoading = true;

            httpPost('m=user&c=weipin_tiny&a=checksitedid', ruleForm).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogDomain = false;
                    that.getList();
                    message.success(res.msg)
                }
            })
        },

        openAudit(row) {
            let that = this;

            if (typeof row != 'undefined') { // 单个审核
                that.detail = row;
                that.ruleFormAudit = {
                    id: row.id,
                    status: row.status,
                };

                that.dialogAudit = true;
            } else { // {yun:}t key='admin_user_weipin_00037'{/yun}
                that.detail = {};
                that.ruleFormAudit = {
                    id: this.idArr,
                    status: "',
                };
            }
        },

        saveAudit() {
            let that = this,
                params = that.ruleFormAudit;

            if (params.status === '') {
                message.error("{yun:}t key='admin_user_weipin_00015'{/yun}");
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=weipin_tiny&a=status', params).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogAudit = false;
                    that.getList();
                    that.$refs.multipleTable.clearSelection();
                    message.success(res.msg);
                }
            })
        },

        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g, '');
        },
        inputPassword(val, form, key) {
            this.$data[form][key] = val.replace(/^ +| +$/g, '');
        },

        openDetail(row) {
            let that = this;

            that.detail = row;
            that.drawerDetail = true;
        },

        getCache() {
            let that = this;

            httpPost('m=user&c=weipin_tiny&a=getCache', {}, { hideloading: true }).then(function (response) {
                let res = response.data,
                    data = res.data;

                that.user_sex = data.user_sex;
                that.user_word = data.user_word;
            })
        },

        getCity(level) {
            let that = this;

            httpPost('m=common&c=cache&a=getCity', {
                provinceid: that.ruleFormAdd.provinceid,
                cityid: that.ruleFormAdd.cityid,
                level: typeof level !== "undefined" ? level : ''
            }, { hideloading: true }).then(function (response) {
                let res = response.data,
                    data = res.data;

                if (typeof data.provinceList !== "undefined") {
                    that.provinceList = data.provinceList;
                }
                if (typeof data.cityList !== "undefined") {
                    that.cityList = data.cityList;
                }
                if (typeof data.regionList !== "undefined") {
                    that.regionList = data.regionList;
                }
            })
        },

        changeCity(val, level) {
            if (level == 1) {
                this.ruleFormAdd.provinceid = val;
                this.ruleFormAdd.cityid = '';
                this.ruleFormAdd.three_cityid = '';
            } else if (level == 2) {
                this.ruleFormAdd.cityid = val;
                this.ruleFormAdd.three_cityid = '';
            }
            this.getCity(level);
        },

        openAdd(row) {
            let that = this;
            that.user_word.length == 0 && this.getCache(); // 首次加载时才加载缓存

            if (typeof row !== 'undefined') {
                that.ruleFormAdd = {
                    id: row.id,
                    username: row.username,
                    sex: row.sex,
                    exp: row.exp,
                    provinceid: row.provinceid > 0 ? row.provinceid : '',
                    cityid: row.cityid > 0 ? row.cityid : '',
                    three_cityid: row.three_cityid > 0 ? row.three_cityid : '',
                    job: row.job,
                    mobile: row.mobile,
                    production: row.production,
                    password: ''
                };

                that.getCity();
                that.drawerAdd = true;
            } else {
                this.ruleFormAdd = {};
                that.getCity();
                this.drawerAdd = true;
            }
        },

        saveAdd() {
            let that = this,
                ruleForm = that.ruleFormAdd;

            if (typeof ruleForm.username === 'undefined' || $.trim(ruleForm.username) == "") {
                message.error("{yun:}t key='wap_user_00234'{/yun}");
                return false;
            }
            if (typeof ruleForm.job === 'undefined' || $.trim(ruleForm.job) == "") {
                message.error("{yun:}t key='admin_user_weipin_00063'{/yun}");
                return false;
            }
            if (typeof ruleForm.mobile === 'undefined' || $.trim(ruleForm.mobile) == "") {
                message.error("{yun:}t key='wap_js_00119'{/yun}");
                return false;
            } else if (!isjsMobile(ruleForm.mobile)) {
                message.error("{yun:}t key='wap_js_00117'{/yun}");
                return false;
            }
            if (typeof ruleForm.production === 'undefined' || $.trim(ruleForm.production) == "") {
                message.error("{yun:}t key='admin_user_weipin_00064'{/yun}");
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=weipin_tiny&a=save', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    that.saveLoading = false;
                    message.error(res.msg);
                } else {
                    that.drawerAdd = false;
                    that.getList();
                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
        },

        refresh(row) {
            let that = this;

            message.confirm(lc('admin_vue_00024'), function () {
                httpPost('m=user&c=weipin_tiny&a=refresh', { id: row.id }).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        message.error(res.msg);
                    } else {
                        that.getList();
                        message.success(res.msg);
                    }
                })
            })
        },
    },
};
</script>
<style scoped>
.drawerModInpt .el-alert {
    background: none;
    padding: 6px 0;
}

.moduleSeachMart {
    display: flex;
    align-items: center;
}

.moduleSeachMart span {
    display: block;
    color: #647386;
    padding-left: 12px;
}

.moduleInptWidt .el-select .el-input {
    width: 120px;
}
</style>