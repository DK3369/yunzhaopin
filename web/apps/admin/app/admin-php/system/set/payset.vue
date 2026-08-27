<template>
<div id="moduapp" class="moduleDome">
        <div class="moduleHeadr">
            <h3>{{ lc('admin_system_00527') }}</h3>
            <span>{{ lc('admin_system_00475') }}</span>
        </div>
        <div class="playModule">
            <div class="playModuleList">
                <div class="playModuName">
                    <span>{{ lc('wap_user_00319') }}</span>
                </div>
                <div class="playModuLay">
                    <span>{{ lc('admin_system_00476') }}</span>
                </div>
                <div class="playModuLink">
                    <a href="https://b.alipay.com/index.htm" target="_blank">{{ lc('admin_system_00508') }}</a>
                </div>
                <div class="playModuSet">
                    <div class="playModLogo">
                        <img src="/admin/php-admin/images/zf1.png" alt="">
                    </div>
                    <div class="playModButn" v-if="config.alipay != 1">
                        <el-button size="small" @click="change_pay('alipay')"type="primary">{{ lc('wap_js_00105') }}</el-button>
                    </div>
                    <div class="playModButn" v-else>
                        <el-button  size="small" @click="change_pay_un('alipay')">{{ lc('admin_system_00535') }}</el-button>
                        <el-button  size="small" @click="alipay_config = true"  type="primary">{{ lc('wap_com_00307') }}</el-button>
                    </div>
                </div>
            </div>
            <div class="playModuleList">
                <div class="playModuName">
                    <span>{{ lc('admin_system_00532') }}</span>
                </div>
                <div class="playModuLay">
                    <span>{{ lc('admin_system_00474') }}</span>
                </div>
                <!--<div class="playModuLink">-->
                    <!--<a href="">立即在线申请</a>-->
                <!--</div>-->
                <div class="playModuSet">
                    <div class="playModLogo">
                        <img src="/admin/php-admin/images/zf3.png" alt="">
                    </div>
                    <div class="playModButn" v-if="config.tenpay!=1">
                        <el-button  size="small" @click="change_pay('tenpay')"type="primary">{{ lc('wap_js_00105') }}</el-button>
                    </div>
                    <div class="playModButn" v-else>
                        <el-button  size="small" @click="change_pay_un('tenpay')">{{ lc('admin_system_00535') }}</el-button>
                        <el-button  size="small" @click="tenpay_config = true"type="primary">{{ lc('wap_com_00307') }}</el-button>
                    </div>
                </div>
            </div>
            <div class="playModuleList">
                <div class="playModuName">
                    <span>{{ lc('admin_system_00529') }}</span>
                </div>
                <div class="playModuLay">
                    <span>{{ lc('admin_system_00499') }}<br>
                        {{ lc('admin_system_00480') }}</span>
                </div>
                <!--<div class="playModuLink">-->
                    <!--<a href="">立即在线申请</a>-->
                <!--</div>-->
                <div class="playModuSet">
                    <div class="playModLogo">
                        <img src="/admin/php-admin/images/zf4.png" alt="">
                    </div>
                    <div class="playModButn" v-if="config.bank!=1">
                        <el-button  size="small" @click="change_pay('bank')" type="primary">{{ lc('wap_js_00105') }}</el-button>
                    </div>
                    <div class="playModButn" v-else>
                        <el-button  size="small" @click="change_pay_un('bank')">{{ lc('admin_system_00535') }}</el-button>
                        <el-button  size="small" @click="show_bank_config"type="primary">{{ lc('wap_com_00307') }}</el-button>
                    </div>
                </div>
            </div>
        </div>

        <!-- 支付宝设置弹窗 -->
        <div class="modluDrawer">
            <el-drawer :title="lc('admin_system_00516')" v-model="alipay_config" :modal-append-to-body="false" :show-close="true"
                       :with-header="true" size="35%">
                <el-alert type="info" :closable="false" style="background: none;">
                    {{ lc('admin_system_00479') }}</el-alert>
                <div class="drawerModInfo drawerModInfoOne">
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00511') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-select v-model="alipaydata.alipaytype" :placeholder="lc('admin_00014')">
                                <el-option :label="lc('admin_system_00506')" value="1"></el-option>
                            </el-select>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00482')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00518') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="alipaydata.sy_alipayname" :placeholder="lc('admin_system_00503')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00486')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00524') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-radio v-model="alipaydata.sy_alipayKeyType" label="1">{{ lc('admin_system_00512') }}</el-radio>
                            <el-radio v-model="alipaydata.sy_alipayKeyType" label="2">{{ lc('admin_system_00523') }}</el-radio>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00487')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis" v-if="alipaydata.sy_alipayKeyType == '1'">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00496') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="alipaydata.sy_alipayid" :placeholder="lc('admin_system_00492')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00485')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis" v-if="alipaydata.sy_alipayKeyType == '1'">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00498') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="alipaydata.sy_alipaycode" :placeholder="lc('admin_system_00501')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00477')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis" v-if="alipaydata.sy_alipayKeyType == '1'">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00517') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="alipaydata.sy_alipayemail" :placeholder="lc('admin_system_00502')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00491')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis" v-if="alipaydata.sy_alipayKeyType == '2'">
                        <div class="drawerModTite">
                            <span>APPID</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="alipaydata.sy_alipayappid" :placeholder="lc('admin_system_00500')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00497')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis" v-if="alipaydata.sy_alipayKeyType == '2'">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00514') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input type="textarea" v-model="alipaydata.sy_alipayprivatekey" :placeholder="lc('admin_system_00501')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00488')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis" v-if="alipaydata.sy_alipayKeyType == '2'">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00515') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input type="textarea" v-model="alipaydata.sy_alipaypublickey" :placeholder="lc('admin_system_00502')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00495')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                </div>
                <div class="setBasicButn" style="border: none;">
                    <el-button type="primary" size="medium" :loading="save_load" @click="submitPayConf(1)">{{ lc('wap_user_00176') }}</el-button>
                </div>
            </el-drawer>
        </div>
        <!-- 财付通设置弹窗 -->
        <div class="modluDrawer">
            <el-drawer :title="lc('admin_system_00521')" v-model="tenpay_config" :modal-append-to-body="false" :show-close="true"
                       :with-header="true" size="35%">
                <div class="drawerModInfo drawerModInfoOne">
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00531') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="tenpaydata.sy_tenpayid" :placeholder="lc('admin_system_00509')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00493')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00525') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="tenpaydata.sy_tenpaycode" :placeholder="lc('admin_system_00504')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00478')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                </div>
                <div class="setBasicButn" style="border: none;">
                    <el-button type="primary" size="medium" :loading="save_load" @click="submitPayConf(3)">{{ lc('wap_user_00176') }}</el-button>
                </div>
            </el-drawer>
        </div>
        <!-- 银行卡设置弹窗 -->
        <div class="modluDrawer">
            <el-drawer :key="nowtime" :title="lc('admin_system_00522')" v-model="bank_config" :modal-append-to-body="false" :show-close="true"
                       :with-header="true" size="50%">
                <div class="drawerModInfo drawerModInfoOne">
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('member_user_00248') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="bank.bank_name" :placeholder="lc('admin_system_00505')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00507')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00530') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="bank.bank_number" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')" :placeholder="lc('admin_system_00520')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00483')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00513') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="bank.bank_address" :placeholder="lc('admin_system_00510')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00489')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00518') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="bank.name" :placeholder="lc('admin_system_00503')"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert :title="lc('admin_system_00526')" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                </div>
                <div class="setBasicButn" style="border: none;">
                    <el-button type="primary" size="medium" :loading="save_load" @click="submitPayConf(4)">{{ lc('wap_user_00176') }}</el-button>
                </div>
                <div class="drawerModInfo drawerModInfoOne" style="padding-left: 12px;">
                    <el-table :data="bankrows" border style="width: 100%"
                              :header-cell-style="{background:'#f5f7fa',color:'#606266'}" v-loading="loading" :empty-text="emptytext">
                        <el-table-column prop="bank_name" :label="lc('member_user_00248')"></el-table-column>
                        <el-table-column prop="bank_number" :label="lc('admin_system_00534')"></el-table-column>
                        <el-table-column prop="bank_address" :label="lc('member_user_00250')"></el-table-column>
                        <el-table-column prop="name" :label="lc('member_user_00249')"></el-table-column>
                        <el-table-column fixed="right" :label="lc('member_user_00048')" width="110">
                            <template #default="scope">
                                <div class="moduleElTaCaoz">
                                    <el-button type="text" size="small" @click="bank_edit(scope.row)">{{ lc('wap_js_00073') }}</el-button>
                                    <el-button type="text" size="small" @click="bank_del(scope.row)">{{ lc('wap_js_00077') }}</el-button>
                                </div>
                            </template>
                        </el-table-column>
                    </el-table>
                </div>
            </el-drawer>
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
                    emptytext: lc('wap_js_00113'),
                    loading: false,
                    config: {},
                    params: {config: 1},
                    alipay_config: false,
                    wechat_config: false,
                    tenpay_config: false,
                    bank_config: false,
                    alipaydata: {},
                    tenpaydata:{},
                    bankrows:[],
                    bank:{bank_name: '', bank_number: '', bank_address: '', name: '', id: ''},

                    nowtime: new Date().getTime(),
					save_load:false,

                    

                }
            },
            components: {

            },
            created: function () {
                this.getInfo();
            },
            methods: {
                changenowtime(){
                    this.nowtime = new Date().getTime()
                },
                bank_del(row){
                    delConfirm(this, row.id, this.delete, '', this.changenowtime);
                },
                async delete(id) {
                    let that = this;
                    let params= {
                        del: id
                    };
                    httpPost('m=system&c=set_payset&a=del', params).then(function (response) {
                        if (response.data.error == 0) {
                            message.success(response.data.msg, function(){
                                that.changenowtime()
                                that.getInfo();
                            });
                        }else{
                            message.error(response.data.msg);
                        }
                    }).catch(function (error) {
                        console.log(error);
                    })
                },
                show_bank_config(){
                    this.bank = {bank_name: '', bank_number: '', bank_address: '', name: '', id: ''}
                    this.bank_config = true
                },
                bank_edit(row){
                    var that = this
                    that.bank = deepClone(row)
                },
                async getInfo() {
                    var that = this
                    that.loading = true;
                    that.emptytext = lc('admin_user_weipin_00026');
                    let res = await httpPost('m=system&c=set_payset&a=index', {});
                    if (res.data.error == 0) {
                        let data = res.data.data;
                        that.config = data.config
                        that.alipaydata = data.alipaydata
                        that.tenpaydata = data.tenpaydata
                        that.bankrows = data.bankrows;
                        that.loading = false;
                        if (that.bankrows.length === 0){
                            that.emptytext = lc('wap_js_00113');
                        }
                    }
                },
                change_pay(paytype) {
                    var that = this
                    var paytype;
                    if (paytype == "alipay") {
                        that.params.alipay = 1
                        that.params.alipaytype = 1
                    } else if (paytype == "tenpay") {
                        that.params.tenpay = 1
                    } else {
                        that.params.bank = 1
                    }
                    that.save()
                },
                change_pay_un(paytype) {
                    var paytype;
                    var that = this
                    if (paytype == "alipay") {
                        that.params.alipay = 0
                    } else if (paytype == "tenpay") {
                        that.params.tenpay = 0
                    } else {
                        that.params.bank = 0
                    }
                    that.save()
                },
                save(){
                    var that = this
                    httpPost('m=system&c=set_payset&a=save', that.params).then(function (res) {
                        if (res.data.error == 0) {
                            that.getInfo();
                        } else {
                            that.$message.error(res.data.msg);
                        }
                    });
                },
                submitPayConf(payType){
                    var that = this
                    if (payType == 1) {
                        var params = that.alipaydata
                        var act = 'alipay'
                    } else if (payType == 3) {
                        var params = {
                            sy_tenpayid:that.tenpaydata.sy_tenpayid,
                            sy_tenpaycode:that.tenpaydata.sy_tenpaycode
                        }
                        var act = 'tenpay'
                    } else {
                        var params = that.bank
                        var act = 'bank'

                        if (!this.bank.bank_name){
                            message.error(lc('admin_system_00505'));
                            return;
                        }
                        if (!this.bank.bank_number){
                            message.error(lc('admin_system_00520'));
                            return;
                        }
                        if (!this.bank.bank_address){
                            message.error(lc('admin_system_00510'));
                            return;
                        }
                        if (!this.bank.name){
                            message.error(lc('admin_system_00503'));
                            return;
                        }
                    }
                    params.pay_config = 1;
					that.save_load = true;
                    httpPost('m=system&c=set_payset&a=' + act, params).then(function (res) {
						that.save_load = false;
                        if (res.data.error == 0) {
                            that.$message.success({
                                message: res.data.msg,
                                onClose: function () {
                                    if (payType == 1) {
                                        that.alipay_config = false
                                    } else if (payType == 2) {
                                        that.wechat_config = false
                                    } else if (payType == 3) {
                                        that.tenpay_config = false
                                    } else {
                                        that.bank = {bank_name: '', bank_number: '', bank_address: '', name: '', id: ''}
                                    }
                                    that.getInfo()
                                }
                            });
                        } else {
                            that.$message.error(res.data.msg);
                        }
                    });
                },
            }
        }
</script>
