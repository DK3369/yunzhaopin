<template>
<div id="daohaapp" class="moduleElenAl">
        <div class="moduleSeachs">
            <div class="moduleSeachInpt">{{ lc('admin_yunying_00114') }}</div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="small"
                    @click="classbox = true">{{ lc('admin_00222') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table :key="timer" :data="list" border style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
                :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" v-loading="loading">
                <template #empty>
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="160">
                </el-table-column>
                <el-table-column :label="lc('admin_01197')">
                    <template #default="scope">
                        <div class="moduleElTaPax">
                            <span>{{ lc('admin_system_00111') }}</span>
                            <span :id="'name'+scope.row.id">{{scope.row.name}}</span>
                            <input type="text" :value="scope.row.name" :id="'inputname'+scope.row.id" @blur="subname" class="input-text hidden">
                            <img src="/admin/php-admin/images/bine.png" alt="" style="cursor:pointer;" @click="checkname(scope.row.id)">
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_01198')" width="220">
                    <template #default="scope">
                        <div class="moduleElTaPax">
                            <span :id="'sort'+scope.row.id">{{scope.row.sort}}</span>
                            <input type="number" :value="scope.row.sort" :id="'inputsort'+scope.row.id" @blur="subsort" class="input-text hidden">
                            <img src="/admin/php-admin/images/bine.png" alt="" style="cursor:pointer;" @click="checksort(scope.row.id)">
                        </div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" @click="up(scope.row.id)">{{ lc('admin_user_company_00371') }}</el-button>
                            <el-button size="small" @click="del(scope.$index)" type="danger">{{ lc('wap_js_00077') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulecz modulePagButn">
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate" @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="batch('del')">{{ lc('member_com_00055') }}</el-button>
            </div>
        </div>
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_yunying_00111')" v-model="classbox" :with-header="true"  :modal-append-to-body="false"
                :show-close="true" width="500px">
                <div class="yunyinDialog">
                    
                    <div class="yunyinDiaList">
                        <div class="yunyinDiaTite">
                            <span>{{ lc('admin_yunying_00113') }}</span>
                        </div>
                        <div class="yunyinDiaInpt">
                            <el-radio v-model="btype" label="1" @input="radioLevel">{{ lc('admin_00290') }}</el-radio>
                            <el-radio v-model="btype" label="2" @input="radioLevel">{{ lc('admin_00291') }}</el-radio>
                        </div>
                    </div>
                    <div class="yunyinDiaList" v-if="isShow == true">
                        <div class="yunyinDiaTite">
                            <span>{{ lc('admin_00290') }}</span>
                        </div>
                        <div class="yunyinDiaInpt">
                            <el-select v-model="nid" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="(item, index) in list" :label="item.name" :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </div>
                    <div class="yunyinDiaList">
                        <div class="yunyinDiaTite">
                            <span>{{ lc('admin_00219') }}</span>
                        </div>
                        <div class="yunyinDiaInpt">
                            <el-input type="textarea" :rows="2" :placeholder="lc('admin_yunying_00112')" v-model="classname">
                            </el-input>
                        </div>
                    </div>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="classbox = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="save">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
		<div class="modluDrawer">
		    <el-drawer :title="lc('admin_00199')" v-model="classContBox" :modal-append-to-body="false" :show-close="true" append-to-body
		        :with-header="true" size="80%" @close="getList">
		        <classcont :cid_p="cid" @child-event="closeClass"></classcont>
		    </el-drawer>
		</div>
    </div>
</template>

<script>
import Classcont from './component/classcont.vue'

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
                    list: [],

                    checkedAll: false, // 全选
                    checkedAllIndeterminate: false,
                    multipleSelection: [], // 多选值存储
                    idArr: [],

                    classbox:false,
                    classname:'',
                    btype: '',
                    nid:'',
                    isShow: false,

                    id: '',
                    inputname:'',
					classContBox:false,
					cid:'',
					
					timer:'0'
                }
            },
			components: {
			    'classcont': Classcont,
			},
            created: function () {
                this.getList();


            },
            methods: {
				closeClass(){
					this.classContBox = false;
					this.getList();
				},
                up(id){
					this.cid = id;
					this.classContBox = true;
                },
                subsort(){
                    let that = this;
                    var sort = $("#inputsort"+that.id).val();
                    if (sort == '') {
                        message.error(lc('admin_01199'));
                        return false;
                    }
                    let params= {
                        sort: sort,
                        id: that.id,
                    };
                    httpPost('m=yunying&c=shop_class&a=ajax', params).then(function (res) {
                        message.success(res.data.msg, function () {
							that.timer = new Date().getTime();
                            that.getList();
                        });
                    });
                    
                },
                checksort(id){
                    this.id = id;
                    $("#sort"+id).hide();
                    $("#inputsort"+id).show();
                    $("#inputsort"+id).focus();
                },
                subname(){
                    let that = this;
                    var name = $("#inputname"+that.id).val();
                    if (name == '') {
                        message.error(lc('admin_01200'));
                        return false;
                    }
                    let params= {
                        name: name,
                        id: that.id,
                    };
                    httpPost('m=yunying&c=shop_class&a=ajax', params).then(function (res) {
                        message.success(res.data.msg, function () {
                            that.timer = new Date().getTime();
							that.getList();
                        });
                    });
                    
                },
                checkname(id){
                    this.id = id;
                    $("#name"+id).hide();
                    $("#inputname"+id).show();
                    $("#inputname"+id).focus();
                },
                save() {
                    let that = this;
                    let params = {
                        ctype: that.btype,
                        nid: that.nid
                    };
                    var position = that.classname.split("\n");
                    var name=position.join("-");
                    if (position == '') {
                        message.error(lc('admin_01200'));
                    }
                    params['name'] = name;
                    httpPost('m=yunying&c=shop_class&a=save', params).then(function (res) {
                        if (res.data.error == 0) {
                            that.classbox= false;
                            message.success(res.data.msg, function () {
                                that.btype = '';
                                that.nid = '';
                                that.classname = '';
                                that.getList();
                            });
                        } else {
                            message.error(res.data.msg);
                        }
                    });

                },
                radioLevel(e){
                    if (e == 2) {
                        this.isShow = true;
                    }else{
                        this.isShow = false; 
                    }
                },
                handleSizeChange(val) {
                    this.limit = val;
                    this.getList();
                },
                handleCurrentChange(val) {
                    this.page = val;
                    this.getList();
                },
                search() {
                    this.page = 1;
                    this.getList();
                },
                getList() {
                    let that = this;
                    that.loading = true;
                    httpPost('m=yunying&c=shop_class',{}, {hideloading: true}).then(function (response) {
                        let res = response.data,
                            data = res.data;

                        that.list = data.list;
                        that.loading = false;
                        if (that.list.length === 0) {
                            that.dataText = lc('wap_js_00113');
                        }
                    })
                },
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
                        message.error(lc('admin_user_weipin_00005'));
                        return false;
                    }

                    let idArr = [];
                    this.multipleSelection.forEach(function(item) {
                        idArr.push(item.id);
                    })
                    this.idArr = idArr;

                    if (type == 'del') {
                        this.del();
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

                    if (typeof idx == 'undefined') { // 批量删除
                        params.del = this.idArr;
                        msg = lc('common_00853');
                    } else {// 单个删除
                        params.del = that.list[idx].id;
                        msg = lc('admin_00333');
                    }

                    delConfirm(this, params, function (params) {
                        httpPost('m=yunying&c=shop_class&a=del', params).then(function(res) {
                            if (res.data.error > 0) {
                                message.error(res.data.msg);
                            } else {
                                message.success(res.data.msg, function () {
                                    that.$refs.multipleTable.clearSelection();
                                    that.getList();
                                });
                            }
                        })
                    }, msg)
                },
            }
        }
</script>
