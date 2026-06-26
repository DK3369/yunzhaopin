<template>
    <!--会员-企业-企业设置：套餐设置-->
    <div class="setUpload">
        <div class="uploadTable">
            <div class="admin_datatip">
                {yun:}t key='admin_00713'{/yun}
            </div>
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                    <th width="400">{yun:}t key='member_user_00181'{/yun}</th>
                    <th>{yun:}t key='member_com_00207'{/yun}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_company_00318'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_vip_type" label="1">{yun:}t key='admin_user_company_00199'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.com_vip_type" label="2">{yun:}t key='admin_user_company_00198'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.com_vip_type" label="0">{yun:}t key='admin_user_company_00321'{/yun}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {yun:}t key='admin_00714'{/yun}
                            </div>
                            <div class="tc_checktip el-icon-info">
                                {yun:}t key='admin_00715'{/yun}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_company_00319'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_integral_online" label="1">{yun:}t key='admin_user_company_00322'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.com_integral_online" label="2">{yun:}t key='admin_00716'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.com_integral_online" label="3">{yun:}t key='admin_00717'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.com_integral_online" label="4">{yun:}t key='admin_00718'{/yun}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {yun:}t key='admin_user_company_00322'{/yun}：{yun:}t key='common_02033'{/yun}{{ config.integral_pricename }}抵扣部分或者全部金额操作！在会员套餐使用完的情况下，会员可以通过{{ config.integral_pricename }}{yun:}t key='wap_com_00383'{/yun}，或者购买会员的方式进行消费的模式
                            </div>
                            <div class="tc_checktip el-icon-info">
                                {yun:}t key='admin_00719'{/yun}
                            </div>
                            <div class="tc_checktip el-icon-info">
                                {{ config.integral_pricename }}{yun:}t key='member_com_00031'{/yun}：在会员套餐使用完的情况下，会员通过购买消费账户{{ config.integral_pricename }}，或者购买会员的方式进行套餐相关的操作
                            </div>
                            <div class="tc_checktip el-icon-info">{yun:}t key='admin_00720'{/yun}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_company_00320'{/yun}</div>
                    </td>
                    <td>
                        <el-checkbox-group v-model="ruleForm.com_single_can">
                            <div class="tc_checkbox">
                                <el-checkbox label="issuejob">{yun:}t key='wap_com_00028'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="jobtop">{yun:}t key='wap_com_00238'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="jobrec">{yun:}t key='wap_com_00237'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="joburgent">{yun:}t key='member_com_00247'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="sxjob">{yun:}t key='wap_com_00029'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="downresume">{yun:}t key='wap_00451'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="invite">{yun:}t key='resume_00029'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="zph">{yun:}t key='admin_user_company_00210'{/yun}</el-checkbox>
                            </div>
                        </el-checkbox-group>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {yun:}t key='admin_00721'{/yun}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_company_00315'{/yun}</div>
                    </td>
                    <td>
                        <el-checkbox-group v-model="ruleForm.sy_only_price">
                            <div class="tc_checkbox">
                                <el-checkbox label="vip">{yun:}t key='admin_user_company_00034'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="pack">{yun:}t key='admin_user_company_00290'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="issuejob">{yun:}t key='wap_com_00028'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="jobtop">{yun:}t key='wap_com_00238'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="jobrec">{yun:}t key='wap_com_00237'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="joburgent">{yun:}t key='member_com_00247'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="sxjob">{yun:}t key='wap_com_00029'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="downresume">{yun:}t key='wap_00451'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="invite">{yun:}t key='resume_00029'{/yun}</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="zph">{yun:}t key='admin_user_company_00210'{/yun}</el-checkbox>
                            </div>
                        </el-checkbox-group>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {yun:}t key='admin_00722'{/yun}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00723'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.tg_back" label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.tg_back" label="2">{yun:}t key='common.close'{/yun}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {yun:}t key='admin_00724'{/yun}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_company_00316'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn w_500">
                            <el-select v-model="ruleForm.rating_add" multiple placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                <el-option v-for="item in qy_rows" :key="item.id" :label="item.name" :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {yun:}t key='admin_00725'{/yun}<span style="color: red;">{yun:}t key='admin_user_company_00311'{/yun}</span>
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_company_00317'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn w_500">
                            <template v-if="qy_rows.length">
                                <el-select v-model="ruleForm.job_ms_rating" multiple placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="item in qy_rows" :key="item.id" :label="item.name" :value="item.id"></el-option>
                                </el-select>
                            </template>
                            <template v-else>
                                {yun:}t key='admin_00726'{/yun}
                                <el-button style="color: red;" @click="handleAdd">{yun:}t key='admin_00727'{/yun}</el-button>
                            </template>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {yun:}t key='admin_00728'{/yun}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_company_00314'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn w_500">
                            <el-select v-model="ruleForm.com_package_open" multiple placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                <el-option label="过期会员" value="0"></el-option>
                                <el-option v-for="item in qy_rows" :key="item.id" :label="item.name" :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {yun:}t key='admin_00729'{/yun}<span style="color:red">{yun:}t key='admin_00730'{/yun}</span>
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_company_00312'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect">
                            <template v-if="qy_rows.length">
                                <el-select v-model="ruleForm.com_rating" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="(item, index) in qy_rows" :key="index" :label="item.name"
                                        :value="item.id">
                                    </el-option>
                                </el-select>
                            </template>
                            <template v-else>
                                {yun:}t key='admin_00726'{/yun}
                                <el-button style="color: red;" @click="handleAdd">{yun:}t key='admin_00727'{/yun}</el-button>
                            </template>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {yun:}t key='admin_user_company_00312'{/yun}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_company_00313'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect">
                            <template v-if="qy_rows.length">
                                <el-select v-model="ruleForm.com_vip_done" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option label="过期会员" value="0"></el-option>
                                    <el-option v-for="(item, index) in qy_rows" :key="index" :label="item.name"
                                        :value="item.id">
                                    </el-option>
                                </el-select>
                            </template>
                            <template v-else>
                                {yun:}t key='admin_00726'{/yun}
                                <el-button style="color: red;" @click="handleAdd">{yun:}t key='admin_00727'{/yun}</el-button>
                            </template>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                {yun:}t key='admin_00731'{/yun}
                            </div>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{yun:}t key='common.submit'{/yun}</el-button>
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
                integral_pricename: '",
            },
            qy_rows: [],
            tcPackage: [],
            ruleForm: {
                //{yun:}t key='admin_user_company_00318'{/yun}
                com_vip_type: null,
                //{yun:}t key='admin_user_company_00319'{/yun}
                com_integral_online: null,
                //{yun:}t key='admin_user_company_00320'{/yun}
                com_single_can: [],
                //{yun:}t key='admin_user_company_00315'{/yun}
                sy_only_price: [],
                //{yun:}t key='admin_00723'{/yun}
                tg_back: null,
                //{yun:}t key='admin_user_company_00316'{/yun}
                rating_add: [],
                //{yun:}t key='admin_user_company_00317'{/yun}
                job_ms_rating: [],
                //{yun:}t key='admin_user_company_00314'{/yun}
                com_package_open: [],
                //{yun:}t key='admin_user_company_00312'{/yun}
                com_rating: null,
                //{yun:}t key='admin_user_company_00313'{/yun}
                com_vip_done: null,
            },
            submitLoading: false,
            titleAddEdit: "设置会员套餐',
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
                    if (need == 'all") {
                        _this.config.integral_pricename = res.data.config.integral_pricename;
                        
                        let config = res.data.config ? res.data.config : {};
                        //{yun:}t key='admin_user_company_00318'{/yun}
                        _this.ruleForm.com_vip_type = config.com_vip_type !== undefined ? config.com_vip_type : null;
                        //{yun:}t key='admin_user_company_00319'{/yun}
                        _this.ruleForm.com_integral_online = config.com_integral_online !== undefined ? config.com_integral_online : null;
                        //{yun:}t key='admin_user_company_00320'{/yun}
                        _this.ruleForm.com_single_can = config.com_single_can;
                        //{yun:}t key='admin_user_company_00315'{/yun}
                        _this.ruleForm.sy_only_price = config.sy_only_price;
                        //{yun:}t key='admin_00723'{/yun}
                        _this.ruleForm.tg_back = config.tg_back !== undefined ? config.tg_back : null;
                        //{yun:}t key='admin_user_company_00316'{/yun}
                        _this.ruleForm.rating_add = config.rating_add;
                        //{yun:}t key='admin_user_company_00317'{/yun}
                        _this.ruleForm.job_ms_rating = config.job_ms_rating;
                        //{yun:}t key='admin_user_company_00314'{/yun}
                        _this.ruleForm.com_package_open = config.com_package_open;
                        //{yun:}t key='admin_user_company_00312'{/yun}
                        _this.ruleForm.com_rating = config.com_rating !== undefined ? config.com_rating : null;
                        //{yun:}t key='admin_user_company_00313'{/yun}
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
            params.config = "提交';
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
            this.titleAddEdit = "{yun:}t key='admin_00689'{/yun}";
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
