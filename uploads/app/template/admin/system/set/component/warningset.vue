<template>
    <div class="setBasicAll setBasicyuing" v-loading="loading">
        <div class="zhuceTable" style="max-height: calc(100% - 80px);">
            <div class="yujinTablCont">
                <div class="yujinTablContTlte">
                    <span>{yun:}t key='admin_system_00411'{/yun}</span>
                </div>
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                            <th width="220">{yun:}t key='admin_00993'{/yun}</th>
                            <th width="140">{yun:}t key='member_user_00181'{/yun}</th>
                            <th>{yun:}t key='member_com_00207'{/yun}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00402'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_addjob" @input="inputIntNumber($event, 'config', 'warning_addjob')">
                                        <span slot="suffix" class="slotspan">{yun:}t key='admin_system_00408'{/yun}</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_addjob_type" active-text="{yun:}t key='member_com_00287'{/yun}" inactive-text="{yun:}t key='resume_00030'{/yun}"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00398'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00399'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_downresume" @input="inputIntNumber($event, 'config', 'warning_downresume')">
                                        <span slot="suffix" class="slotspan">{yun:}t key='admin_system_00408'{/yun}</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_downresume_type" active-text="{yun:}t key='member_com_00287'{/yun}" inactive-text="{yun:}t key='resume_00030'{/yun}"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00398'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00401'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_addresume" @input="inputIntNumber($event, 'config', 'warning_addresume')">
                                        <span slot="suffix" class="slotspan">{yun:}t key='admin_system_00408'{/yun}</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_addresume_type" active-text="{yun:}t key='member_com_00287'{/yun}" inactive-text="{yun:}t key='resume_00030'{/yun}"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00398'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00400'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_recharge" @input="inputIntNumber($event, 'config', 'warning_recharge')">
                                        <span slot="suffix" class="slotspan">{yun:}t key='admin_user_00350'{/yun}</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_recharge_type" active-text="{yun:}t key='member_com_00287'{/yun}" inactive-text="{yun:}t key='resume_00030'{/yun}"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00398'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00403'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.sy_hour_msgnum" @input="inputIntNumber($event, 'config', 'sy_hour_msgnum')">
                                        <span slot="suffix" class="slotspan">{yun:}t key='admin_system_00409'{/yun}</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_closemsg_type" active-text="{yun:}t key='member_com_00287'{/yun}" inactive-text="{yun:}t key='resume_00030'{/yun}"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00398'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00405'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_lookresume" @input="inputIntNumber($event, 'config', 'warning_lookresume')">
                                        <span slot="suffix" class="slotspan">{yun:}t key='admin_system_00408'{/yun}</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_lookresume_type" active-text="{yun:}t key='member_com_00287'{/yun}" inactive-text="{yun:}t key='resume_00030'{/yun}"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00398'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00406'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_lookjob" @input="inputIntNumber($event, 'config', 'warning_lookjob')">
                                        <span slot="suffix" class="slotspan">{yun:}t key='admin_system_00408'{/yun}</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_lookjob_type" active-text="{yun:}t key='member_com_00287'{/yun}" inactive-text="{yun:}t key='resume_00030'{/yun}"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00398'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00404'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_teljob" @input="inputIntNumber($event, 'config', 'warning_teljob')">
                                        <span slot="suffix" class="slotspan">{yun:}t key='wap_com_00049'{/yun}</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_teljob_type" active-text="{yun:}t key='member_com_00287'{/yun}" inactive-text="{yun:}t key='resume_00030'{/yun}"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00398'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00394'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_reg_ip" @input="inputIntNumber($event, 'config', 'warning_reg_ip')">
                                        <span slot="suffix" class="slotspan">{yun:}t key='wap_com_00049'{/yun}</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_reg_ip_type" active-text="{yun:}t key='member_com_00287'{/yun}" inactive-text="{yun:}t key='resume_00030'{/yun}"
                                        active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00398'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                        
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00395'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_exchange_link" @input="inputIntNumber($event, 'config', 'warning_exchange_link')">
                                        <span slot="suffix" class="slotspan">{yun:}t key='wap_com_00049'{/yun}</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-switch v-model="config.warning_exchange_link_type" active-text="{yun:}t key='member_com_00287'{/yun}"
                                        inactive-text="{yun:}t key='resume_00030'{/yun}" active-value="1"></el-switch>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00398'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="yujinTablCont">
                <div class="yujinTablContTlte">
                    <span>{yun:}t key='admin_system_00413'{/yun}</span>
                </div>
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                            <th width="220">{yun:}t key='admin_00993'{/yun}</th>
                            <th width="300">{yun:}t key='member_user_00181'{/yun}</th>
                            <th>{yun:}t key='member_com_00207'{/yun}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00407'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_sendresume"  @input="inputIntNumber($event, 'config', 'warning_sendresume')">
                                        <span slot="suffix" class="slotspan">{yun:}t key='admin_system_00408'{/yun}</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio v-model="config.warning_sendresume_type" label="1">{yun:}t key='common_02082'{/yun}</el-radio>
                                    <el-radio v-model="config.warning_sendresume_type" label="2">{yun:}t key='admin_00994'{/yun}</el-radio>
                                    <el-radio v-model="config.warning_sendresume_type" label="3">{yun:}t key='admin_00995'{/yun}</el-radio>
                                </div>
                                <div class="TableInpt" style="padding-top: 12px;" v-if="config.warning_sendresume_type == 2">
                                    <el-input placeholder="{yun:}t key='admin_00996'{/yun}" v-model="config.warning_sendresume_tips"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00396'{/yun}</span>
                                </div>
                            </td>
                        </tr>

                    </tbody>
                </table>
            </div>
            <div class="yujinTablCont">
                <div class="yujinTablContTlte">
                    <span>{yun:}t key='admin_system_00412'{/yun}</span>
                </div>
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                            <th width="220">{yun:}t key='admin_00993'{/yun}</th>
                            <th width="300">{yun:}t key='member_user_00181'{/yun}</th>
                            <th>{yun:}t key='member_com_00207'{/yun}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00407'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="0" v-model="config.warning_sqjob" @input="inputIntNumber($event, 'config', 'warning_sqjob')">
                                        <span slot="suffix" class="slotspan">{yun:}t key='admin_system_00410'{/yun}</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio v-model="config.warning_sqjob_type" label="1">{yun:}t key='common_02082'{/yun}</el-radio>
                                    <el-radio v-model="config.warning_sqjob_type" label="2">{yun:}t key='admin_00994'{/yun}</el-radio>
                                    <el-radio v-model="config.warning_sqjob_type" label="3">{yun:}t key='admin_00995'{/yun}</el-radio>
                                </div>
                                <div class="TableInpt" style="padding-top: 12px;" v-if="config.warning_sqjob_type == 2">
                                    <el-input placeholder="{yun:}t key='admin_00996'{/yun}" v-model="config.warning_sqjob_tips"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00393'{/yun}</span>
                                </div>
                            </td>
                        </tr>

                    </tbody>
                </table>
            </div>
        </div>

        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
    </div>
</template>
    
<script>
module.exports = {
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
                    message.success("{yun:}t key='wap_user_00264'{/yun}");
                    this.$emit('update:setshow', false);
                } else {
                    message.error("{yun:}t key='admin_system_00397'{/yun}");
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