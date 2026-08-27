<template>
    <div class="setBasicAll setBasicyuing" v-loading="loading">
        <div class="zhuceTable" style="max-height: calc(100% - 80px);">
            <div class="yujinTablCont">
                <div class="yujinTablContTlte">
                    <span>{{ lc('admin_system_00411') }}</span>
                </div>
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="180">{{ lc('member_com_00021') }}</th>
                            <th width="220">{{ lc('admin_00993') }}</th>
                            <th width="140">{{ lc('member_user_00181') }}</th>
                            <th>{{ lc('member_com_00207') }}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00402') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_addjob" @input="inputIntNumber($event, 'config', 'warning_addjob')">
                                        <template #suffix><span class="slotspan">{{ lc('admin_system_00408') }}</span></template>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_addjob_type" :active-text="lc('member_com_00287')" :inactive-text="lc('resume_00030')"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00398') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00399') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_downresume" @input="inputIntNumber($event, 'config', 'warning_downresume')">
                                        <template #suffix><span class="slotspan">{{ lc('admin_system_00408') }}</span></template>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_downresume_type" :active-text="lc('member_com_00287')" :inactive-text="lc('resume_00030')"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00398') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00401') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_addresume" @input="inputIntNumber($event, 'config', 'warning_addresume')">
                                        <template #suffix><span class="slotspan">{{ lc('admin_system_00408') }}</span></template>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_addresume_type" :active-text="lc('member_com_00287')" :inactive-text="lc('resume_00030')"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00398') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00400') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_recharge" @input="inputIntNumber($event, 'config', 'warning_recharge')">
                                        <template #suffix><span class="slotspan">{{ lc('admin_user_00350') }}</span></template>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_recharge_type" :active-text="lc('member_com_00287')" :inactive-text="lc('resume_00030')"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00398') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00403') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.sy_hour_msgnum" @input="inputIntNumber($event, 'config', 'sy_hour_msgnum')">
                                        <template #suffix><span class="slotspan">{{ lc('admin_system_00409') }}</span></template>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_closemsg_type" :active-text="lc('member_com_00287')" :inactive-text="lc('resume_00030')"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00398') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00405') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_lookresume" @input="inputIntNumber($event, 'config', 'warning_lookresume')">
                                        <template #suffix><span class="slotspan">{{ lc('admin_system_00408') }}</span></template>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_lookresume_type" :active-text="lc('member_com_00287')" :inactive-text="lc('resume_00030')"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00398') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00406') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_lookjob" @input="inputIntNumber($event, 'config', 'warning_lookjob')">
                                        <template #suffix><span class="slotspan">{{ lc('admin_system_00408') }}</span></template>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_lookjob_type" :active-text="lc('member_com_00287')" :inactive-text="lc('resume_00030')"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00398') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00404') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_teljob" @input="inputIntNumber($event, 'config', 'warning_teljob')">
                                        <template #suffix><span class="slotspan">{{ lc('wap_com_00049') }}</span></template>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_teljob_type" :active-text="lc('member_com_00287')" :inactive-text="lc('resume_00030')"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00398') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00394') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_reg_ip" @input="inputIntNumber($event, 'config', 'warning_reg_ip')">
                                        <template #suffix><span class="slotspan">{{ lc('wap_com_00049') }}</span></template>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_reg_ip_type" :active-text="lc('member_com_00287')" :inactive-text="lc('resume_00030')"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00398') }}</span>
                                </div>
                            </td>
                        </tr>
                        
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00395') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_exchange_link" @input="inputIntNumber($event, 'config', 'warning_exchange_link')">
                                        <template #suffix><span class="slotspan">{{ lc('wap_com_00049') }}</span></template>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_exchange_link_type" :active-text="lc('member_com_00287')"
                                        :inactive-text="lc('resume_00030')" active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00398') }}</span>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="yujinTablCont">
                <div class="yujinTablContTlte">
                    <span>{{ lc('admin_system_00413') }}</span>
                </div>
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="180">{{ lc('member_com_00021') }}</th>
                            <th width="220">{{ lc('admin_00993') }}</th>
                            <th width="300">{{ lc('member_user_00181') }}</th>
                            <th>{{ lc('member_com_00207') }}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00407') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_sendresume"  @input="inputIntNumber($event, 'config', 'warning_sendresume')">
                                        <template #suffix><span class="slotspan">{{ lc('admin_system_00408') }}</span></template>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio v-model="config.warning_sendresume_type" label="1">{{ lc('common_02082') }}</el-radio>
                                    <el-radio v-model="config.warning_sendresume_type" label="2">{{ lc('admin_00994') }}</el-radio>
                                    <el-radio v-model="config.warning_sendresume_type" label="3">{{ lc('admin_00995') }}</el-radio>
                                </div>
                                <div class="TableInpt" style="padding-top: 12px;" v-if="config.warning_sendresume_type == 2">
                                    <el-input :placeholder="lc('admin_00996')" v-model="config.warning_sendresume_tips"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00396') }}</span>
                                </div>
                            </td>
                        </tr>

                    </tbody>
                </table>
            </div>
            <div class="yujinTablCont">
                <div class="yujinTablContTlte">
                    <span>{{ lc('admin_system_00412') }}</span>
                </div>
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="180">{{ lc('member_com_00021') }}</th>
                            <th width="220">{{ lc('admin_00993') }}</th>
                            <th width="300">{{ lc('member_user_00181') }}</th>
                            <th>{{ lc('member_com_00207') }}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00407') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_sqjob" @input="inputIntNumber($event, 'config', 'warning_sqjob')">
                                        <template #suffix><span class="slotspan">{{ lc('admin_system_00410') }}</span></template>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio v-model="config.warning_sqjob_type" label="1">{{ lc('common_02082') }}</el-radio>
                                    <el-radio v-model="config.warning_sqjob_type" label="2">{{ lc('admin_00994') }}</el-radio>
                                    <el-radio v-model="config.warning_sqjob_type" label="3">{{ lc('admin_00995') }}</el-radio>
                                </div>
                                <div class="TableInpt" style="padding-top: 12px;" v-if="config.warning_sqjob_type == 2">
                                    <el-input :placeholder="lc('admin_00996')" v-model="config.warning_sqjob_tips"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00393') }}</span>
                                </div>
                            </td>
                        </tr>

                    </tbody>
                </table>
            </div>
        </div>

        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
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
    props: {
        setshow: Boolean
    },
    data: function () {
        return {
            loading: false,
            config: {},
            saveLoading: false
        }
    },
    watch: {
        setshow: {
            handler(val) {
                if (val == true){
                    this.getConfig();
                }
            },
            immediate: true,
            deep: true,
        },
    },
    methods: {
        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g,'');
        },
        async getConfig() {
            let that = this;
            let params = {};
            this.loading = true;

            httpPost('m=system&c=warning&a=getWarningConfig', params).then((result) => {

                this.loading = false;

                var res = result.data

                if (res.error == 0) {
                    that.config = res.data
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        save: function () {
            let _this = this;

            let params = {
                warning_addjob_type: this.config.warning_addjob_type == 1 ? 1 : 2,
                warning_addjob: this.config.warning_addjob,

                warning_downresume_type: this.config.warning_downresume_type == 1 ? 1 : 2,
                warning_downresume: this.config.warning_downresume,

                warning_addresume_type: this.config.warning_addresume_type == 1 ? 1 : 2,
                warning_addresume: this.config.warning_addresume,

                warning_recharge_type: this.config.warning_recharge_type == 1 ? 1 : 2,
                warning_recharge: this.config.warning_recharge,

                warning_closemsg_type: this.config.warning_closemsg_type == 1 ? 1 : 2,
                sy_hour_msgnum: this.config.sy_hour_msgnum,

                warning_lookresume_type: this.config.warning_lookresume_type == 1 ? 1 : 2,
                warning_lookresume: this.config.warning_lookresume,

                warning_lookjob_type: this.config.warning_lookjob_type == 1 ? 1 : 2,
                warning_lookjob: this.config.warning_lookjob,

                warning_teljob_type: this.config.warning_teljob_type == 1 ? 1 : 2,
                warning_teljob: this.config.warning_teljob,

                warning_reg_ip_type: this.config.warning_reg_ip_type == 1 ? 1 : 2,
                warning_reg_ip: this.config.warning_reg_ip,

                warning_exchange_link_type: this.config.warning_exchange_link_type == 1 ? 1 : 2,
                warning_exchange_link: this.config.warning_exchange_link,

                warning_sendresume: this.config.warning_sendresume,
                warning_sendresume_type: this.config.warning_sendresume_type,
                warning_sendresume_tips: this.config.warning_sendresume_tips,
                warning_sendresume_tipss: this.config.warning_sendresume_tipss,

                warning_sqjob: this.config.warning_sqjob,
                warning_sqjob_type: this.config.warning_sqjob_type,
                warning_sqjob_tips: this.config.warning_sqjob_tips,
                warning_sqjob_tipss: this.config.warning_sqjob_tipss,

            };
            _this.saveLoading = true
            httpPost('m=system&c=warning&a=config', params).then((response) => {
                var res = response.data;
                if (res.error == 0) {
                    message.success(lc('wap_user_00264'));
                    this.$emit('update:setshow', false);
                } else {
                    message.error(lc('admin_system_00397'));
                }
            }).finally(function () {
                setTimeout(function () {
                    _this.saveLoading = false;
                }, 2000);
            });
        }
    },
};
</script>
<style scoped></style>