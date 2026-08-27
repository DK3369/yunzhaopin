<template>
<div id="daohaapp" class="moduleElenAl" v-cloak>
        <div class="moduleSeachs">
            <div class="moduleSeachleft" style="width: calc(100% - 105px);">
                <div class="moduleInptList">
                    <el-input :placeholder="lc('admin_user_weipin_00003')" v-model="search.keyword" size="small" class="input-with-select" clearable>
                        <template #prepend><el-select v-model="search.typeca" :placeholder="lc('wap_user_00100')">
                            <el-option :label="lc('wap_user_00311')" value="1"></el-option>
                            <el-option :label="lc('admin_user_00140')" value="2"></el-option>
                            <el-option :label="lc('admin_user_company_00041')" value="3"></el-option>
                        </el-select></template>
                    </el-input>
                </div>
                <div class=" tableSeachInptsmall newsinput">
                    <el-select v-model="search.typezf" size="small" style="margin-right: 0;" :placeholder="lc('member_user_00240')" clearable @change="doUserQuery">
                        <el-option v-for="(val,key) in pay" :key="key" :label="val" :value="key"></el-option>
                    </el-select>
                </div>
                <div class=" tableSeachInptlodsi newsinput">
                    <el-cascader size="small"
                        v-model="type_v"
                        :placeholder="lc('wap_user_00318')"
                        :options="type_cascader"
                        :props="{expandTrigger:'hover'}"
                        @change="typeChange"
                        clearable>
                    </el-cascader>
                </div>
                <div class=" tableSeachInptsmall newsinput">
                    <el-select v-model="search.order_state" style="margin-right: 0;" size="small" :placeholder="lc('admin_yunying_00087')" clearable @change="doUserQuery">
                        <el-option v-for="item in payArr" :key="item.value" :label="item.label" :value="item.value"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInptsmalltwo">
                    <el-date-picker v-model="search.times" type="daterange" align="right" unlink-panels :range-separator="lc('admin_company_00019')" :start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')" :picker-options="timeOptions" value-format="YYYY-MM-dd" size="small" @change="doUserQuery"></el-date-picker>
                </div>
                <div class="newsbtnbox">
                    <el-button type="primary" icon="el-icon-search" size="small" @click="doUserQuery">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-folder-checked" size="small" @click="exportOrder">{{ lc('admin_yunying_00082') }}</el-button>
            </div>
        </div>

        <div class="moduleElTable moduleElTableSizes">
            <div class="admin_datatip"><i class="el-icon-document"></i>
                <span class="admin_datatip_n cp_n" @click="stateFun('0')">{{ lc("admin_total_amount", [orderSum.orderPriceAll]) }} </span>
                <span class="admin_datatip_n cp_n" @click="stateFun('2')">{{ lc("admin_paid_amount", [orderSum.orderPayed]) }} </span>
                <span class="admin_datatip_n cp_n" @click="stateFun('3')" v-if="orderSum.orderPaying">{{ lc("admin_pending_confirm_amount", [orderSum.orderPaying]) }}</span>
                <span class="admin_datatip_n cp_n" @click="stateFun('1')" v-if="orderSum.orderPay">{{ lc("admin_pending_payment_amount", [orderSum.orderPay]) }}</span>
                <span class="admin_datatip_n">{{ lc("admin_search_results_count", [total]) }}</span>
            </div>
            <el-table :data="tableData" border style="width: 100%" @selection-change="selectChange"
                @sort-change="shortChange" ref="multipleTable"
                :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="calc(100% - 43px)"
                v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="80" sortable="custom"></el-table-column>
                <el-table-column :label="lc('admin_01263')" min-width="150" show-overflow-tooltip>
                    <template #default="scope">
                        <div class="cz_button">
                            {{scope.row.comname}}<br />{{scope.row.username}}
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="chongzhi" :label="lc('admin_yunying_00075')" min-width="150">
                    <template #default="scope">
                        <div class="cz_button">
                            {{scope.row.order_id}}<br />{{scope.row.order_type_n}}<br />
                            {{scope.row.type_n}}{{scope.row.rating_name}}
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="order_price" :label="lc('admin_user_company_00044')" width="120" sortable="custom">
                    <template #default="scope">
                        <span style="color: red;">{{scope.row.order_price}}</span>
                    </template>
                </el-table-column>
                <el-table-column prop="order_time" :label="lc('wap_user_00314')" width="180" sortable="custom"></el-table-column>
                <el-table-column prop="zt" :label="lc('member_user_00181')" width="150" align="center">
                    <template #default="scope">
                        <div v-html="scope.row.order_state_n"></div>
                    </template>
                </el-table-column>
                <el-table-column prop="yewuyuan" :label="lc('admin_user_company_00049')" width="130" align="center">
                    <template #default="scope">
                        <div v-if="scope.row.crm_name">{{scope.row.crm_name}}</div>
                        <div v-else><span style="color: red;">{{ lc('wap_user_00181') }}</span></div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="130" align="center">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" @click="detailFun(scope.row)" type="primary">{{ lc('wap_com_00427') }}</el-button>
                            <el-button type="danger" size="small " @click="delRow(scope.row)">{{ lc('wap_js_00077') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>

        </div>
        <div class="modulePaging">
            <div class="modulecz modulePagButn">
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="batchDel" size="small">{{ lc('member_com_00055') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination :total="total" @current-change="userPageChange" @size-change="userPageSizeChange"
                    :page-size="pageSize" :page-sizes="pageSizes" v-model:current-page="page"
                    layout="total, sizes, prev, pager, next, jumper">
                </el-pagination>
            </div>
        </div>
        <!-- 查看更多 -->

        <el-drawer :title="lc('admin_user_company_00048')" v-model="drawer3" :direction="direction" append-to-body size="540px"
            :modal-append-to-body="false">
            <div class="dd_xqbox ddanQboxcont">
                <div class="ddanQboxBorder">
                    <div class="dd_list">
                        <span class="dd_name">{{ lc('member_com_00021') }}</span>
                        <span class="dd_texts">{{detail.comname}}</span>
                    </div>
                    <div class="dd_list">
                        <span class="dd_name">{{ lc('wap_user_00320') }}</span>
                        <span>{{detail.order_id}}</span>
                    </div>
                    <div class="dd_list">
                        <span class="dd_name">{{ lc('admin_yunying_00079') }}</span>
                        <span>{{detail.order_time_ymd}}</span>
                    </div>
                    <div class="dd_list">
                        <span class="dd_name">{{ lc('admin_user_00140') }}</span>
                        <div class="yun_admin_divh" v-if="detail.type==3 || detail.order_type =='bank'">
                            {{detail.comname}}+
                        </div>
                        {{detail.username}}
                    </div>
                    <div class="dd_list">
                        <span class="dd_name" v-if="detail.order_state==1">{{ lc('admin_yunying_00083') }}</span>
                        <span class="dd_name" v-else>{{ lc('admin_user_company_00044') }}</span>
                        {{ lc("admin_currency_yuan", [detail.order_price]) }}
                    </div>
                    <template v-if="detail.type =='3' || detail.order_type =='bank'">
                        <div class="dd_list">
                            {{ lc("admin_bank_name_value", [detail.bankname]) }}
                        </div>
                        <div class="dd_list">
                            {{ lc("admin_bank_account_value", [detail.bankid]) }}
                        </div>
                        <div class="dd_list">
                            {{ lc("admin_remit_amount_value", [detail.order_price]) }}
                        </div>
                    </template>
                    <template v-if="detail.type =='2'">
                        <div class="dd_list">
                            {{ lc("admin_income_points_value", [integral_pricename, detail.integral]) }}
                        </div>
                    </template>

                    <div class="dd_list">{{ lc("admin_order_type_value", [detail.type_n]) }}</div>
                    <div class="dd_list">{{ lc("admin_payment_status_value", [detail.order_state_n]) }}</div>
                    <div class="dd_list">{{ lc("admin_contract_status_value", [lc(htpics.length > 0 ? 'wap_user_00123' : 'admin_yunying_00088')]) }}</div>
                    <template v-if="detail.type==3 || detail.order_type =='bank'">
                        <div class="dd_list"><span class="dd_name">{{ lc('admin_user_company_00043') }}</span>
                            <template v-if="detail.order_state ==1 || !detail.order_pic">{{ lc('admin_yunying_00076') }}</template>
                            <template v-else>
                                <img :src="detail.order_pic" :alt="lc('admin_yunying_00089')" width="160px" height="60px">
                            </template>
                        </div>
                    </template>
                    <template>
                        <div class="dd_list" v-if="detail.crm_name">{{ lc("admin_salesperson_value", [detail.crm_name]) }}
                        </div>
                    </template>
                    <div class="dd_list"><span class="dd_name">{{ lc('admin_vue_00040') }}</span>
                        {{detail.order_remark}}
                    </div>
                    <div class="dd_list" v-if="htpics">
                        <span class="dd_name" style="float: left">{{ lc('admin_user_company_00030') }}</span>
                        <el-image v-for="item in htpics" :preview-src-list="yySrcList" style="margin-right: 10px;width: 100px; height: 100px" :src="item.pic_n" :id="item.id"></el-image>
                    </div>
                    <!-- <div class="dd_listbth">
                        <el-button type="primary" @click="uploadHeTong">{{ lc('admin_yunying_00077') }}</el-button>
                        <el-button type="primary" @click="drawer6 = true">{{ lc('admin_yunying_00080') }}</el-button>
                        <el-button type="primary" @click="confirmOrder">{{ lc('admin_yunying_00084') }}</el-button>
                    </div> -->
                </div>
                <div class="dd_listbth">
                    <el-button type="success" plain @click="uploadHeTong">{{ lc('admin_yunying_00077') }}</el-button>
                    <el-button v-if="detail.order_state ==1 || detail.order_state == 3" type="primary" plain @click="drawer6 = true" style="margin: 0 12px;">{{ lc('admin_yunying_00080') }}</el-button>
                    <el-button v-if="detail.order_state ==1 || detail.order_state == 3" type="primary" @click="confirmOrder">{{ lc('admin_yunying_00084') }}</el-button>
                </div>
            </div>

            <!-- 修改订单 -->
            <div class="modluDrawer">
                <el-dialog :title="lc('admin_yunying_00080')" v-model="drawer6" :with-header="true" append-to-body
                    :modal-append-to-body="false" :show-close="true" width="450px">
                    <div class="wxsettip_small ">{{ lc('admin_yunying_00081') }}</div>
                    <el-input placeholder="" v-model="detail.order_price"></el-input>
                    <div class="wxsettip_small ">{{ lc('admin_vue_00040') }}</div>
                    <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')" v-model="detail.order_remark">
                    </el-input>
                    <template #footer><span class="dialog-footer">
                        <el-button @click="drawer6 = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                        <el-button type="primary" @click="editOrder" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
                    </span></template>
                </el-dialog>
            </div>

            <!-- 上传合同 -->
            <div class="modluDrawer">
                <el-dialog :title="lc('admin_yunying_00077')" v-model="drawer5" :with-header="true" append-to-body
                    :modal-append-to-body="false" :show-close="true" width="30%">
                    <el-upload :action="uploadAction" multiple :limit="3" list-type="picture-card" :accept="pic_accept"
                        :on-success="handleAvatarSuccess" :file-list="fileList" ref="files" :on-exceed="exceedFun"
                        :before-upload="onBeforeUpload">
                        <i class="el-icon-plus"></i>
                        <div>
                            <img class="el-upload-list__item-thumbnail" :src="file.url" alt="">
                            <span class="el-upload-list__item-actions">
                                <span class="el-upload-list__item-delete" @click="handleRemove(file)">
                                    <i class="el-icon-delete"></i>
                                </span>
                            </span>
                        </div>
                    </el-upload>
                    <div style="font-size: 12px;color: #8c939d">
                        <i class="el-icon-warning-outline"></i>
                        <span>{{ lc('admin_user_company_00029') }}</span>
                    </div>
                    <!--            <el-upload-->
                    <!--                    class="upload-demo"-->
                    <!--                    :action='uploadAction'-->
                    <!--                    :on-preview="handlePreview"-->
                    <!--                    :on-remove="handleRemove"-->
                    <!--                    :before-remove="beforeRemove"-->
                    <!--                    multiple-->
                    <!--                    :limit="3"-->
                    <!--                    :on-exceed="handleExceed"-->
                    <!--                    :file-list="fileList" :accept="pic_accept">-->
                    <!--                <el-button size="small" type="primary">点击上传</el-button>-->
                    <!--                <div class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div>-->
                    <!--            </el-upload>-->
                    <template #footer><span class="dialog-footer">
                        <el-button @click="drawer5 = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                        <el-button type="primary" @click="saveImg" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
                    </span></template>
                </el-dialog>
            </div>
        </el-drawer>
    </div>
</template>

<script>
import Navxiugai from './component/navxiugai.vue'

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
                    pic_accept: localStorage.getItem("pic_accept"),
                    loading: false,
                    emptytext: lc('wap_js_00113'),

                    checkedAll: false,
                    drawer: false,
                    drawer2: false,
                    drawer3: false,
                    direction: 'rtl',
                    drawer5: false,
                    drawer6: false,
                    uploadAction: baseUrl + 'm=yunying&c=finance_company_order&a=multiupload',
                    fileList: [],
                    tableData: [],
                    type_v:'',
                    type_visible:false,
                    search: {
                        typeca: '1',
                        order_state: '',
                        typezf: '',
                        typedd: '',
                        pay_state: '',
                        type: '',
                        searchVal: '',
                        times: [],
                        rating: ''
                    },
                    searchTypedd : '',
                    timeOptions: {
                        shortcuts: [{
                            text: lc('common_02000'),
                            onClick(picker) {
                                const end = new Date();


                                const start = new Date();
                                start.setTime(start.getTime() - 3600 * 1000 * 24);
                                end.setTime(end.getTime() - 3600 * 1000 * 24);
                                picker.$emit('pick', [start, end]);
                            }
                        }, {
                            text: lc('common_01940'),
                            onClick(picker) {
                                const end = new Date();
                                const start = new Date();
                                picker.$emit('pick', [start, end]);
                            }
                        }, {
                            text: lc('admin_user_00146'),
                            onClick(picker) {
                                const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() - 1) * 24 * 60 * 60 * 1000);
                                const end = new Date();
                                picker.$emit('pick', [start, end]);
                            }
                        }, {
                            text: lc('admin_user_00142'),
                            onClick(picker) {
                                const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() + 6) * 24 * 60 * 60 * 1000);
                                const end = new Date(new Date().setHours(0, 0, 0) + (0 - new Date().getDay()) *24 * 60 * 60 *1000);
                                picker.$emit('pick', [start, end]);
                            }
                        }, {
                            text: lc('admin_user_00147'),
                            onClick(picker) {
                                const end = new Date();
                                const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 1).setHours(0, 0, 0));
                                picker.$emit('pick', [start, end]);
                            }
                        }, {
                            text: lc('admin_user_00143'),
                            onClick(picker) {
                                const end = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 0).setHours(23, 59, 59, 59));
                                const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth() - 1, 1).setHours(0, 0, 0));
                                picker.$emit('pick', [start, end]);
                            }
                        }]
                    },
                    select: '',
                    value: true,
                    currentPage4: 4,
                    payArr: [{
                        value: '0',
                        label: lc('admin_01264')
                    }, {
                        value: '1',
                        label: lc('admin_yunying_00085')
                    }, {
                        value: '2',
                        label: lc('admin_01265')
                    }, {
                        value: '3',
                        label: lc('admin_yunying_00086')
                    },
                    {
                        value: '4',
                        label: lc('admin_yunying_00078')
                    }],
                    total: 0,
                    page: 1,
                    idsArr: [],
                    pageSize: 0,
                    pageSizes: [],
                    uri: "m=yunying&c=",
                    orderSum: {},
                    pay: {},
                    ordertype: {},
                    ratingarr: [],
                    vipTypeShow: false,
                    //  弹窗
                    detail: {},
                    htpics: [],
                    picurl: [], // 保存图片地址
                    integral_pricename: '',
                    submitLoading: false,
                    prevPage:0,
                    yySrcList:[],
                    
                    type_cascader:[],
                }
            },
            components: {
                // 'navxiugai': Navxiugai,
            },
            created() {
				var that = this
				var pqdata={};
				let params = window.parent.homeapp.$route.params;
				
                let query = window.parent.homeapp.$route.query;
				
				if (!$.isEmptyObject(params)) {
					pqdata = params;
				}else if(!$.isEmptyObject(query)){
					pqdata = query;
				}
				
				this.getParams(pqdata);
                this.getList();
                this.searchType();
            },
            methods: {
                getParams: function (params = {}) {
                    var that = this;
                    var rating='',type_v = [];
                    for (let i in params) {
                        if (typeof that.search[i] != 'undefined') {
                            if(i == 'typedd'){
                                that.searchTypedd = isNaN(params[i]) ? params[i] : params[i].toString();
                            } else {
                                that.search[i] = isNaN(params[i]) ? params[i] : params[i].toString();
                                if(i=='rating'){
                                    rating = that.search[i]
                                }
                            }
                        }
                    }
                    if(that.searchTypedd){
                        type_v.push(that.searchTypedd);
                    }
                    if(rating){
                        type_v.push(rating);
                    }
                    that.type_v = type_v;
                },
                stateFun: function ($staus) {
                    this.type_v = [];
                    this.search = {
                        pay_state: '',
                        time: '',
                        type: '',
                        keyword: '',
                        times: '',
                        typezf: '',
                        typedd: '',
                        rating: '',
                        order_state: $staus ? $staus : '',
                        typeca: ''
                    }
                    this.doUserQuery()
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
                uploadHeTong: function () {
                    this.handleImg();
                    this.drawer5 = true
                },
                handleImg() {
                    let htpics = this.htpics;
                    this.fileList = [];
                    for (const key in htpics) {
                        this.fileList.push({ name: '11111', url: htpics[key]['pic_n'], id: htpics[key]['id'] });
                    }
                },
                confirmOrder: function () {
                    let id = this.detail.id;
                    let _this = this;
                    let url = _this.uri + 'finance_company_order&a=setpay';
                    let sendData = {
                        id: id
                    }
                    delConfirm(_this, sendData, function (sendData) {
                        httpPost(url, sendData).then(function (response) {
                            let res = response.data;
                            if (res.error == 0) {
                                message.success(res.msg)
                                _this.getList();
                                _this.drawer3 = false;
                            } else {
                                message.error(res.msg)
                            }
                        })
                    }, lc('admin_01266'));
                },
                editOrder() {
                    let id = this.detail.id;
                    let _this = this;
                    let url = _this.uri + 'finance_company_order&a=save';
                    let sendData = {
                        order_price: _this.detail.order_price,
                        order_remark: _this.detail.order_remark,
                        id: id
                    }
                    _this.submitLoading = true;

                    httpPost(url, sendData).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            message.success(res.msg, _this.getList())
                        } else {
                            message.error(res.msg)
                        }
                        _this.drawer6 = false;
                    }).catch(function (error) {
                        console.log(error);
                    }).finally(function () {
                        _this.submitLoading = false;
                    });
                },
                htpicsFun: function (id) {
                    let _this = this;
                    let url = _this.uri + 'finance_company_order&a=edit';
                    httpPost(url, { id: id }).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            _this.htpics = res.data.htpics
                            _this.handleImg()
                        } else {
                            message.error(res.msg)
                        }
                    })
                },
                detailFun: function (row) {
                    let id = row.id;
                    let _this = this;
                    let url = _this.uri + 'finance_company_order&a=edit';
                    _this.yySrcList = [];
                    httpPost(url, { id: id }).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            _this.detail = res.data.detail
                            _this.htpics = res.data.htpics
                            if (_this.htpics.length>0) {
                                for (let j in _this.htpics) {
                                    _this.yySrcList.push(_this.htpics[j]['pic_n']);
                                }
                            }
                            _this.integral_pricename = res.data.integral_pricename
                            _this.drawer3 = true;
                        } else {
                            message.error(res.msg)
                        }
                    })
                },
                searchType: function () {
                    let _this = this;
                    let url = _this.uri + 'finance_company_order&a=searchType';
                    httpPost(url, {}, {hideloading: true}).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            _this.pay = res.data.pay;
                            _this.ordertype = res.data.ordertype;
                            _this.ratingarr = res.data.ratingarr;
							if(res.data.ratingarr.length>0){
								res.data.ratingarr.unshift({value:0,label:lc('wap_js_00075')});
							}
                            var type_cascader = [];
                            var one = {};
                            var onechild = [];
                            
                            for(let i in  _this.ordertype){
                                one = {value:i,label:_this.ordertype[i]};
                                if(i=='1'){
                                    one = {value:i,label:_this.ordertype[i],children:res.data.ratingarr};
									if(res.data.ratingarr.length>0){
										one.children = res.data.ratingarr
									}
                                }
                                type_cascader.push(one);
                            }
                            _this.type_cascader = type_cascader;
                        }
                        _this.search.typedd = _this.searchTypedd;
                        _this.searchTypedd = '';
                    })
                },
                typeChange(){
					this.search.rating = '';
					this.search.typedd = '';
					if(this.type_v.length>0){
						if(this.type_v[0]=='1' && this.type_v.length>1){
							this.search.rating = this.type_v[1]>0?this.type_v[1]:'';
						}
						this.search.typedd = this.type_v[0];
					}
					this.doUserQuery();
				},
                doUserQuery() {
                    this.page = 1
                    this.getList()
                },
                
                userPageChange(val) {
                    this.page = val
                    this.getList()
                },
                userPageSizeChange(val) {
                    this.pageSize = val
                    this.getList()
                },
                getList() {
                    let _this = this;
                    let url = _this.uri + 'finance_company_order&a=index';
                    this.search.page = this.page;
                    this.search.pageSize = this.pageSize;
                    let sendData = deepClone(this.search);
                    if(this.searchTypedd != ''){
                        sendData.typedd = this.searchTypedd;
                    }
                    _this.loading = true;
                    _this.emptytext = lc('admin_user_weipin_00026');
                    httpPost(url, sendData, {hideloading: true}).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            _this.tableData = res.data.data;
                            _this.total = res.data.total;
                            if (res.data.hasOwnProperty('orderSum')) {
                                _this.orderSum = res.data.orderSum;
                            } else {
                                _this.orderSum = {};
                            }
                            if(_this.prevPage != _this.page){
                                _this.prevPage = _this.page;
                                _this.$nextTick(function () {
                                    let table = _this.$refs.multipleTable;
                                    if (table && table.bodyWrapper) {
                                        table.bodyWrapper.scrollTop = 0;
                                    }
                                });
                            }
                            _this.loading = false;
                            _this.pageSizes = res.data.pageSizes;
                            if (_this.tableData.length === 0) {
                                _this.emptytext = lc('wap_js_00113');
                            }
                        }
                    })
                },
                handleRemove(file) {
                    let _this = this;
                    let url = _this.uri + 'finance_company_order&a=htpic_del';
                    let orderid = this.detail.id;
                    this.$confirm(lc('admin_system_00632'), lc('wap_js_00125'), {
                        confirmButtonText: lc('common_02016'),
                        cancelButtonText: lc('wap_js_00080'),
                        type: 'warning'
                    }).then(() => {
                        if (file.id) {
                            let sendData = {
                                delid: file.id
                            }
                            httpPost(url, sendData).then(function (response) {
                                let res = response.data;
                                if (res.error == 0) {
                                    message.success(res.msg, _this.htpicsFun(orderid))
                                }
                            })
                        } else {
                            let picurl = file.response.picurl
                            _this.picurl = _this.picurl.filter(item => item != picurl);
                            _this.$refs.files.handleRemove(file)
                        }
                    }).catch(() => {
                    });
                },
                handlePreview(file) {
                    console.log(file);
                },
                onBeforeUpload: function (file) {
                    const isJPG =
                        file.type === 'image/jpg' || file.type === 'image/png' || file.type === 'image/jpeg' || file.type === 'image/gif';
                    const isLt2M = file.size / 1024 / 1024 < 5;
                    if (!isJPG) {
                        this.$message.error(lc('admin_system_00633'));
                    }
                    if (!isLt2M) {
                        this.$message.error(lc('admin_system_00634'));
                    }
                    return isJPG && isLt2M;
                },
                beforeRemove(file, fileList) {
                    return this.$confirm(lc('admin_yunying_00201', [file.name]));
                },
                handleSizeChange(val) {
                    this.pageSize = val;
                    this.getList();
                },
                handleCurrentChange(val) {
                    console.log(`Current page: ${val}`);
                },
                handleAvatarSuccess(res, file) {
                    if (res.error == 0) {
                        this.picurl.push(res.picurl);
                    }
                },
                exceedFun(files, fileList) {
                    this.$message.error(lc('admin_company_00015'));
                },
                saveImg: function () {
                    let _this = this;
                    let url = _this.uri + 'finance_company_order&a=uploadsave';
                    let orderid = this.detail.id;
                    if (this.picurl.length <= 0) {
                        return;
                    }
                    let sendData = {
                        order_id: orderid,
                        picurl: this.picurl
                    }
                    _this.submitLoading = true;
                    httpPost(url, sendData).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            _this.picurl = [];
                            message.success(res.msg, _this.htpicsFun(orderid))
                            _this.drawer5 = false
                            _this.submitLoading = false;
                        } else {
                            message.error(res.msg);
                            _this.submitLoading = false;
                        }
                    }).catch(function (error) {
                        console.log(error);
                    })

                },
                batchDel: function () {
                    let ids = this.idsArr;
                    if (!ids.length) {
                        message.error(lc('admin_01267'));
                        return
                    }
                    let _this = this;
                    let url = this.uri + 'finance_company_order&a=del'

                    delConfirm(this, { del: ids }, function (params) {
                        httpPost(url, params).then(function (response) {
                            let res = response.data;
                            if (res.error == 0) {
                                message.success(res.msg, _this.getList());
                            } else {
                                message.error(res.msg);
                            }
                        })
                    }, lc('admin_yunying_00074'))

                },
                delRow: function (row) {
                    let id = row.id;
                    let _this = this;
                    let url = this.uri + 'finance_company_order&a=del'
                    delConfirm(this, { id: id }, function (params) {
                        httpPost(url, params).then(function (response) {
                            let res = response.data;
                            if (res.error == 0) {
                                message.success(res.msg, _this.getList());
                            } else {
                                message.error(res.msg);
                            }
                        })
                    }, lc('admin_yunying_00074'))
                },
                exportOrder: function () {
                    let _this = this;
                    let ids = this.idsArr;
                    let url = this.uri + 'finance_company_order&a=xls'
                    _this.$confirm(lc('admin_01268'), lc('wap_user_00205'), {
                        confirmButtonText: lc('common_02016'),
                        cancelButtonText: lc('wap_js_00080'),
                        type: 'warning'
                    }).then(() => {
                        httpPost(url, { uid: ids, time: this.search.times }).then(function (response) {
                            let res = response.data;
                            if (res.error > 0) {
                                message.error(res.msg);
                            } else {
                                utilFile.downloadFileByByte(res.data.file, `${res.data.file_name}`)
                            }

                        })
                    }).catch(() => {
                    });
                },
                selectAllBottom: function (value) {
                    value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
                },
                shortChange(e) {
                    let orderMap = { ascending: 'asc', descending: 'desc' }
                    this.search.t = e.order ? e.prop : null;
                    this.search.order = orderMap[e.order];
                    this.page = 1;
                    this.getList();
                }

            }
        }
</script>
