<template>
    <!--会员-企业-企业设置：套餐设置-->
    <div class="setUpload">
        <div class="uploadTable">
            <div class="admin_datatip">
                注：该页面的功能设置，不会对现有的数据生效，只对后产生的数据生效，请根据需要，谨慎设置
            </div>
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">名称</th>
                    <th width="400">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">企业会员模式</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_vip_type" label="1">时间模式</el-radio>
                            <el-radio v-model="ruleForm.com_vip_type" label="2">套餐模式</el-radio>
                            <el-radio v-model="ruleForm.com_vip_type" label="0">同时开启</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                时间模式在设置的时间内，不受套餐数限制，但是有每日上限操作。
                            </div>
                            <div class="tc_checktip el-icon-info">
                                套餐模式在设置的时间内，受套餐数限制，用完即止。
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">套餐消费模式</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_integral_online" label="1">抵扣消费</el-radio>
                            <el-radio v-model="ruleForm.com_integral_online" label="2">金额消费</el-radio>
                            <el-radio v-model="ruleForm.com_integral_online" label="3">{{ config.integral_pricename }}消费</el-radio>
                            <el-radio v-model="ruleForm.com_integral_online" label="4">套餐消费</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                抵扣消费：账户{{ config.integral_pricename }}抵扣部分或者全部金额操作！在会员套餐使用完的情况下，会员可以通过{{ config.integral_pricename }}抵扣金额，或者购买会员的方式进行消费的模式
                            </div>
                            <div class="tc_checktip el-icon-info">
                                金额消费：在会员套餐使用完的情况下，会员通过消费金额，或者购买会员的方式进行套餐相关的操作
                            </div>
                            <div class="tc_checktip el-icon-info">
                                {{ config.integral_pricename }}消费：在会员套餐使用完的情况下，会员通过购买消费账户{{ config.integral_pricename }}，或者购买会员的方式进行套餐相关的操作
                            </div>
                            <div class="tc_checktip el-icon-info">套餐消费：在会员套餐使用完的情况下，会员仅可购买会员才可以继续进行套餐内操作</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">可单独购买</div>
                    </td>
                    <td>
                        <el-checkbox-group v-model="ruleForm.com_single_can">
                            <div class="tc_checkbox">
                                <el-checkbox label="issuejob">上架职位</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="jobtop">职位置顶</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="jobrec">职位推荐</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="joburgent">职位紧急</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="sxjob">刷新职位</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="downresume">下载简历</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="invite">邀请面试</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="zph">报名招聘会</el-checkbox>
                            </div>
                        </el-checkbox-group>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                已选中的项目可以在套餐外单独使用金额、{{ config.integral_pricename }}等购买，未选中则必须购买相应的套餐或增值包
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">强制金额消费项目</div>
                    </td>
                    <td>
                        <el-checkbox-group v-model="ruleForm.sy_only_price">
                            <div class="tc_checkbox">
                                <el-checkbox label="vip">会员套餐</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="pack">增值服务</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="issuejob">上架职位</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="jobtop">职位置顶</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="jobrec">职位推荐</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="joburgent">职位紧急</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="sxjob">刷新职位</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="downresume">下载简历</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="invite">邀请面试</el-checkbox>
                            </div>
                            <div class="tc_checkbox">
                                <el-checkbox label="zph">报名招聘会</el-checkbox>
                            </div>
                        </el-checkbox-group>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                已选中的项目仅可使用金额购买
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">职位推广取消</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.tg_back" label="1">开启</el-radio>
                            <el-radio v-model="ruleForm.tg_back" label="2">关闭</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                开启职位推广取消功能：企业会员主动关闭职位推广，剩余推广套餐，返还到企业账号（例如：设置置顶10天，推广了5天，关闭推广，剩余5天返回到企业账户）
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">购买会员套餐累加</div>
                    </td>
                    <td>
                        <div class="TableButn w_500">
                            <el-select v-model="ruleForm.rating_add" multiple placeholder="请选择">
                                <el-option v-for="item in qy_rows" :key="item.id" :label="item.name" :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                当前会员套餐未到期，购买新会员，选中的套餐数据按累加计算（套餐数量和套餐时间）。<span style="color: red;">职位上架数量不会累加</span>
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">会员职位免审核</div>
                    </td>
                    <td>
                        <div class="TableButn w_500">
                            <template v-if="qy_rows.length">
                                <el-select v-model="ruleForm.job_ms_rating" multiple placeholder="请选择">
                                    <el-option v-for="item in qy_rows" :key="item.id" :label="item.name" :value="item.id"></el-option>
                                </el-select>
                            </template>
                            <template v-else>
                                暂无等级，
                                <el-button style="color: red;" @click="handleAdd">添加会员等级</el-button>
                            </template>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                勾选后，该会员类型职位免审核
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">会员套餐开放控制</div>
                    </td>
                    <td>
                        <div class="TableButn w_500">
                            <el-select v-model="ruleForm.com_package_open" multiple placeholder="请选择">
                                <el-option label="过期会员" value="0"></el-option>
                                <el-option v-for="item in qy_rows" :key="item.id" :label="item.name" :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                勾选套餐所属会员用户，前台无法直接购买其他会员套餐和增值服务，会员套餐不可见；<span style="color:red">如若有绑定套餐，用户可见且可购买。</span>
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">企业注册默认等级</div>
                    </td>
                    <td>
                        <div class="TableSelect">
                            <template v-if="qy_rows.length">
                                <el-select v-model="ruleForm.com_rating" placeholder="请选择">
                                    <el-option v-for="(item, index) in qy_rows" :key="index" :label="item.name"
                                        :value="item.id">
                                    </el-option>
                                </el-select>
                            </template>
                            <template v-else>
                                暂无等级，
                                <el-button style="color: red;" @click="handleAdd">添加会员等级</el-button>
                            </template>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                企业注册默认等级
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">会员到期后默认为</div>
                    </td>
                    <td>
                        <div class="TableSelect">
                            <template v-if="qy_rows.length">
                                <el-select v-model="ruleForm.com_vip_done" placeholder="请选择">
                                    <el-option label="过期会员" value="0"></el-option>
                                    <el-option v-for="(item, index) in qy_rows" :key="index" :label="item.name"
                                        :value="item.id">
                                    </el-option>
                                </el-select>
                            </template>
                            <template v-else>
                                暂无等级，
                                <el-button style="color: red;" @click="handleAdd">添加会员等级</el-button>
                            </template>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip el-icon-info">
                                如设置的不是过期会员，用户现有会员等级到期后按选择的等级设置套餐数量、到期时间
                            </div>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">提交</el-button>
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
                //企业会员模式
                com_vip_type: null,
                //套餐消费模式
                com_integral_online: null,
                //可单独购买
                com_single_can: [],
                //强制金额消费项目
                sy_only_price: [],
                //职位推广取消
                tg_back: null,
                //购买会员套餐累加
                rating_add: [],
                //会员职位免审核
                job_ms_rating: [],
                //会员套餐开放控制
                com_package_open: [],
                //企业注册默认等级
                com_rating: null,
                //会员到期后默认为
                com_vip_done: null,
            },
            submitLoading: false,
            titleAddEdit: '设置会员套餐',
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
                        //企业会员模式
                        _this.ruleForm.com_vip_type = config.com_vip_type !== undefined ? config.com_vip_type : null;
                        //套餐消费模式
                        _this.ruleForm.com_integral_online = config.com_integral_online !== undefined ? config.com_integral_online : null;
                        //可单独购买
                        _this.ruleForm.com_single_can = config.com_single_can;
                        //强制金额消费项目
                        _this.ruleForm.sy_only_price = config.sy_only_price;
                        //职位推广取消
                        _this.ruleForm.tg_back = config.tg_back !== undefined ? config.tg_back : null;
                        //购买会员套餐累加
                        _this.ruleForm.rating_add = config.rating_add;
                        //会员职位免审核
                        _this.ruleForm.job_ms_rating = config.job_ms_rating;
                        //会员套餐开放控制
                        _this.ruleForm.com_package_open = config.com_package_open;
                        //企业注册默认等级
                        _this.ruleForm.com_rating = config.com_rating !== undefined ? config.com_rating : null;
                        //会员到期后默认为
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
            params.config = '提交';
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
            this.titleAddEdit = '设置会员套餐';
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
