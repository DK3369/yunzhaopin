<template>
    <div class="moduleElenAl" style="position: relative; padding: 0 20px;">
        <div class="moduleElTable" style="margin-top: 0; padding: 0; width: 100%; height: calc(100% - 60px);">
            <el-table :key="timer" :data="list" border style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
                :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="160">
                </el-table-column>
                <el-table-column :label="lc('admin_01197')">
                    <template slot-scope="scope">
                        <div class="moduleElTaPax" v-if="scope.row.keyid ==0">
                            <span>{{ lc('admin_system_00111') }}</span>
                            <span :id="'pname'+scope.row.id">{{scope.row.name}}</span>
                            <input type="text" :value="scope.row.name" :id="'cname'+scope.row.id" @blur="subname" class="input-text hidden">
                            <img src="../../../admin/images/bine.png" alt="" style="cursor:pointer;" @click="checkname(scope.row.id)">
                        </div>
                        <div class="moduleElTaPax" v-else>
                            <span> ┗</span>
                            <span :id="'pname'+scope.row.id">{{scope.row.name}}</span>
                            <input type="text" :value="scope.row.name" :id="'cname'+scope.row.id" @blur="subname" class="input-text hidden">
                            <img src="../../../admin/images/bine.png" alt="" style="cursor:pointer;" @click="checkname(scope.row.id)">
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_01198')" width="">
                    <template slot-scope="scope">
                        <div class="moduleElTaPax">
                            <span :id="'psort'+scope.row.id">{{scope.row.sort}}</span>
                            <input type="number" :value="scope.row.sort" :id="'csort'+scope.row.id" @blur="subsort" class="input-text hidden">
                            <img src="../../../admin/images/bine.png" alt="" style="cursor:pointer;" @click="checksort(scope.row.id)">
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="mini" @click="del(scope.$index)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate" @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>

                <el-button @click="batch('del')" size="mini">{{ lc('member_com_00055') }}</el-button>

            </div>
        </div>
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_yunying_00111')" :visible.sync="classbox" :with-header="true" :modal-append-to-body="false"
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
                                <el-option v-for="(item, index) in list" :key="index" :label="item.name" :value="item.id"></el-option>
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
                <span slot="footer" class="dialog-footer">
                    <el-button @click="classbox = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="save">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
	</div>
</template>
    <!-- script -->
    <script>
        module.exports = {
			props: {
				cid_p: {
					type: String,
					default: ''
				}
			},
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
					cid:'',

                    id: '',
                    inputname:'',
					
					timer:'0'
                }
            },
			watch: {
				cid_p: {
					handler(n) {
						if (n != '') {
							this.cid = n.trim();
							this.getList();
						}
					},
					deep: true,
					immediate: true
				},
			},
            methods: {
                subsort(){
                    let that = this;
                    var sort = $("#csort"+that.id).val();
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
                    $("#psort"+id).hide();
                    $("#csort"+id).show();
                    $("#csort"+id).focus();
                },
                subname(){
                    let that = this;
                    var name = $("#cname"+that.id).val();
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
                    $("#pname"+id).hide();
                    $("#cname"+id).show();
                    $("#cname"+id).focus();
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
                    let that = this,
                        params = {};
                    if (that.cid !='') {
                        params.id = that.cid;
                    }
                    that.loading = true;
                    httpPost('m=yunying&c=shop_class&a=up',params).then(function (response) {
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
                        message.error(lc('admin_00136'));
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

                    if (typeof idx == 'undefined") { // {{ lc('member_com_00055') }}
                        params.del = this.idArr;
                        msg = lc('common_00853');
                    } else {// {{ lc('common_01711') }}
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
                goBack() {
                    history.go(-1);
                },
            }
        };
    </script>
    <style type="text/css">
        .input-text{width: 173px;
    height: 36px;
    line-height: 36px;
    border: 1px solid #dcdee2;
    padding: 0px 0px 0px 5px;
    margin-right: 10px;
    color: #17233d;}
        .hidden{display: none;}
    </style>
</body>

</html>