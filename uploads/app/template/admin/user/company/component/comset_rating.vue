<template>
    <!--会员-企业-企业设置：套餐设置-->
    <div class="setUpload">
        <div class="uploadTable">
            <div class="admin_datatip">
                {{ lc('admin_00713') }}
            </div>
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">{{ lc('member_com_00021') }}</th>
                    <th width="400">{{ lc('member_user_00181') }}</th>
                    <th>{{ lc('member_com_00207') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00318') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_vip_type" label="1">{{ lc('admin_user_company_00199') }}</el-radio>
                            <el-radio v-model="ruleForm.com_vip_type" label="2">{{ lc('admin_user_company_00198') }}</el-radio>
                            <el-radio v-model="ruleForm.com_vip_type" label="0">{{ lc('admin_user_company_00321') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {{ lc('admin_00714') }}
                            </div>
                            <div class="tc_checktip el-icon-info">
                                {{ lc('admin_00715') }}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00319') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_integral_online" label="1">{{ lc('admin_user_company_00322') }}</el-radio>
                            <el-radio v-model="ruleForm.com_integral_online" label="2">{{ lc('admin_00716') }}</el-radio>
                            <el-radio v-model="ruleForm.com_integral_online" label="3">{{ lc('admin_00717') }}</el-radio>
                            <el-radio v-model="ruleForm.com_integral_online" label="4">{{ lc('admin_00718') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {{ lc("admin_company_00003", [lc("admin_user_company_00322"), lc("common_02033"), config.integral_pricename, lc("wap_com_00383")]) }}
                            </div>
                            <div class="tc_checktip el-icon-info">
                                {{ lc('admin_00719') }}
                            </div>
                            <div class="tc_checktip el-icon-info">
                                {{ lc("admin_company_00004", [config.integral_pricename]) }}
                            </div>
                            <div class="tc_checktip el-icon-info">{{ lc('admin_00720') }}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00320') }}</div>
                    </td>
                    <td>
                        <el-checkbox-group v-model="ruleForm.com_single_can">
                            <div class="tc_checkbox">
                                <el-checkbox label="issuejob">{{ lc('wap_com_00028') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="jobtop">{{ lc('wap_com_00238') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="jobrec">{{ lc('wap_com_00237') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="joburgent">{{ lc('member_com_00247') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="sxjob">{{ lc('wap_com_00029') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="downresume">{{ lc('wap_00451') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="invite">{{ lc('resume_00029') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="zph">{{ lc('admin_user_company_00210') }}</el-checkbox>
                            </div>
                        </el-checkbox-group>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {{ lc('admin_00721') }}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00315') }}</div>
                    </td>
                    <td>
                        <el-checkbox-group v-model="ruleForm.sy_only_price">
                            <div class="tc_checkbox">
                                <el-checkbox label="vip">{{ lc('admin_user_company_00034') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="pack">{{ lc('admin_user_company_00290') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="issuejob">{{ lc('wap_com_00028') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="jobtop">{{ lc('wap_com_00238') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="jobrec">{{ lc('wap_com_00237') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="joburgent">{{ lc('member_com_00247') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="sxjob">{{ lc('wap_com_00029') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="downresume">{{ lc('wap_00451') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="invite">{{ lc('resume_00029') }}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="zph">{{ lc('admin_user_company_00210') }}</el-checkbox>
                            </div>
                        </el-checkbox-group>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {{ lc('admin_00722') }}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00723') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.tg_back" label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio v-model="ruleForm.tg_back" label="2">{{ lc('common.close') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {{ lc('admin_00724') }}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00316') }}</div>
                    </td>
                    <td>
                        <div class="TableButn w_500">
                            <el-select v-model="ruleForm.rating_add" multiple :placeholder="lc('wap_user_00100')">
                                <el-option v-for="item in qy_rows" :key="item.id" :label="item.name" :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {{ lc('admin_00725') }}<span style="color: red;">{{ lc('admin_user_company_00311') }}</span>
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00317') }}</div>
                    </td>
                    <td>
                        <div class="TableButn w_500">
                            <template v-if="qy_rows.length">
                                <el-select v-model="ruleForm.job_ms_rating" multiple :placeholder="lc('wap_user_00100')">
                                    <el-option v-for="item in qy_rows" :key="item.id" :label="item.name" :value="item.id"></el-option>
                                </el-select>
                            </template>
                            <template v-else>
                                {{ lc('admin_00726') }}
                                <el-button style="color: red;" @click="handleAdd">{{ lc('admin_00727') }}</el-button>
                            </template>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {{ lc('admin_00728') }}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00314') }}</div>
                    </td>
                    <td>
                        <div class="TableButn w_500">
                            <el-select v-model="ruleForm.com_package_open" multiple :placeholder="lc('wap_user_00100')">
                                <el-option :label="lc('admin_user_company_00297')" value="0"></el-option>
                                <el-option v-for="item in qy_rows" :key="item.id" :label="item.name" :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {{ lc('admin_00729') }}<span style="color:red">{{ lc('admin_00730') }}</span>
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00312') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect">
                            <template v-if="qy_rows.length">
                                <el-select v-model="ruleForm.com_rating" :placeholder="lc('wap_user_00100')">
                                    <el-option v-for="(item, index) in qy_rows" :key="index" :label="item.name"
                                        :value="item.id">
                                    </el-option>
                                </el-select>
                            </template>
                            <template v-else>
                                {{ lc('admin_00726') }}
                                <el-button style="color: red;" @click="handleAdd">{{ lc('admin_00727') }}</el-button>
                            </template>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {{ lc('admin_user_company_00312') }}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00313') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect">
                            <template v-if="qy_rows.length">
                                <el-select v-model="ruleForm.com_vip_done" :placeholder="lc('wap_user_00100')">
                                    <el-option :label="lc('admin_user_company_00297')" value="0"></el-option>
                                    <el-option v-for="(item, index) in qy_rows" :key="index" :label="item.name"
                                        :value="item.id">
                                    </el-option>
                                </el-select>
                            </template>
                            <template v-else>
                                {{ lc('admin_00726') }}
                                <el-button style="color: red;" @click="handleAdd">{{ lc('admin_00727') }}</el-button>
                            </template>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {{ lc('admin_00731') }}
                            </div>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{{ lc('common.submit') }}</el-button>
        </div>
        <!--设置会员套餐-->
        <el-drawer :title="titleAddEdit" :visible.sync="addVisible" :destroy-on-close="true" :modal-append-to-body="false" :wrapper-closable="false" size="770px">
            <comrating_index_edit :id="0" :config="config" @child-event-list="handleCloseList"></comrating_index_edit>
        </el-drawer>
    </div>
</template>

<script>
module.exports = {
    data: function () {
        return {
            searchForm: {},
            config: {
                integral_pricename: '',
            },
            qy_rows: [],
            tcPackage: [],
            ruleForm: {
                //{{ lc('admin_user_company_00318') }}
                com_vip_type: null,
                //{{ lc('admin_user_company_00319') }}
                com_integral_online: null,
                //{{ lc('admin_user_company_00320') }}
                com_single_can: [],
                //{{ lc('admin_user_company_00315') }}
                sy_only_price: [],
                //{{ lc('admin_00723') }}
                tg_back: null,
                //{{ lc('admin_user_company_00316') }}
                rating_add: [],
                //{{ lc('admin_user_company_00317') }}
                job_ms_rating: [],
                //{{ lc('admin_user_company_00314') }}
                com_package_open: [],
                //{{ lc('admin_user_company_00312') }}
                com_rating: null,
                //{{ lc('admin_user_company_00313') }}
                com_vip_done: null,
            },
            submitLoading: false,
            titleAddEdit: lc('admin_00689'),
            addVisible: false,
        }
    },
    created() {
        this.getList();
    },
    methods: {
        getList(need = 'all') {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.searchForm));
            for (let index in params) {
                (params[index] === '') && (params[index] = null);
            }
            params.need = need;
            httpPost('m=user&c=company_comset&a=rating', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.qy_rows = res.data.qy_rows;
                    _this.tcPackage = res.data.tcPackage;
                    if (need == 'all') {
                        _this.config.integral_pricename = res.data.config.integral_pricename;
                        
                        let config = res.data.config ? res.data.config : {};
                        //{{ lc('admin_user_company_00318') }}
                        _this.ruleForm.com_vip_type = config.com_vip_type !== undefined ? config.com_vip_type : null;
                        //{{ lc('admin_user_company_00319') }}
                        _this.ruleForm.com_integral_online = config.com_integral_online !== undefined ? config.com_integral_online : null;
                        //{{ lc('admin_user_company_00320') }}
                        _this.ruleForm.com_single_can = config.com_single_can;
                        //{{ lc('admin_user_company_00315') }}
                        _this.ruleForm.sy_only_price = config.sy_only_price;
                        //{{ lc('admin_00723') }}
                        _this.ruleForm.tg_back = config.tg_back !== undefined ? config.tg_back : null;
                        //{{ lc('admin_user_company_00316') }}
                        _this.ruleForm.rating_add = config.rating_add;
                        //{{ lc('admin_user_company_00317') }}
                        _this.ruleForm.job_ms_rating = config.job_ms_rating;
                        //{{ lc('admin_user_company_00314') }}
                        _this.ruleForm.com_package_open = config.com_package_open;
                        //{{ lc('admin_user_company_00312') }}
                        _this.ruleForm.com_rating = config.com_rating !== undefined ? config.com_rating : null;
                        //{{ lc('admin_user_company_00313') }}
                        _this.ruleForm.com_vip_done = config.com_vip_done !== undefined ? config.com_vip_done : null;
                    }
                    if (_this.qy_rows.length < 1) {
                        _this.getBaseData();
                    }
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            params.config = lc('common.submit');
            params.isDayActionAllCheck = 0;
            params.isDayActionZeroCostCheck = 0;
            params.com_single_can = params.com_single_can.length > 0 ? params.com_single_can : '';
            params.sy_only_price = params.sy_only_price.length > 0 ? params.sy_only_price : '';
            params.rating_add = Array.isArray(params.rating_add) ? params.rating_add.join(',') : '';
            params.job_ms_rating = Array.isArray(params.job_ms_rating) ? params.job_ms_rating.join(',') : '';
            params.com_package_open = params.com_package_open.length > 0 ? params.com_package_open : '';
            _this.submitLoading = true;
            httpPost('m=user&c=company_comset&a=save', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.getList();
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
        handleAdd(){
            this.titleAddEdit = lc('admin_00689');
            this.addVisible = true;
        },
        handleCloseList() {
            this.addVisible = false;
            this.getList('rating_list');//重新获取等级
        },
        getBaseData() {
            let _this = this;
            httpPost('m=user&c=company_comrating&a=baseData', {}, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    for (let i in res.data.config) {
                        _this.config[i] = res.data.config[i];
                    }
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
    },
    components: {
        'comrating_index_edit': httpVueLoader('./comrating_index_edit.vue'),
    }
};
</script>
<style scoped>
    .w_500 .el-input--suffix{width: 500px;}
</style>
