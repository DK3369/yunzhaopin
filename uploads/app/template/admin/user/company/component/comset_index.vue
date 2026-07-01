<template>
    <!--会员-企业-企业设置：企业设置-->
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">{{ lc('member_com_00021') }}</th>
                    <th width="480">{{ lc('member_user_00181') }}</th>
                    <th>{{ lc('member_com_00207') }}</th>
                </tr>
                </thead>
                <tbody>
                <!--审核信息-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00291') }}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00281')" v-model="ruleForm.com_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('wap_01717')" v-model="ruleForm.com_job_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('wap_00321')" v-model="ruleForm.com_partjob_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('wap_com_00075')" v-model="ruleForm.com_cert_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('wap_com_00149')" v-model="ruleForm.com_logo_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('wap_user_00157')" v-model="ruleForm.com_show_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00283')" v-model="ruleForm.com_banner_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00282')" v-model="ruleForm.com_revise_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_00709')" v-model="ruleForm.com_yqmb_status"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            {{ lc('admin_00557') }}
                        </div>
                    </td>
                </tr>
                <!--强制操作-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00337') }}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('member_user_00447')" v-model="ruleForm.com_enforce_info"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('member_com_00071')" v-model="ruleForm.com_enforce_mobilecert"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('wap_com_00186')" v-model="ruleForm.com_enforce_emailcert"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('wap_com_00075')" v-model="ruleForm.com_enforce_licensecert"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00289')" v-model="ruleForm.com_enforce_setposition"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00260')" v-model="ruleForm.com_gzgzh"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            {{ lc('admin_00701') }}
                        </div>
                    </td>
                </tr>
                <!--企业资质上传项-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00259') }}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00063')" v-model="ruleForm.com_social_credit"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00261')" v-model="ruleForm.com_cert_owner"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00302')" v-model="ruleForm.com_cert_wt"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00286')" v-model="ruleForm.com_cert_other"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            {{ lc('admin_00702') }}
                        </div>
                    </td>
                </tr>
                <!--委托书-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('wap_com_00057') }}</div>
                    </td>
                    <td style="display: flex;align-items: center;">
                        <el-upload :accept="file_accept" :action="uploadAction" :on-change="uploadChange"
                                   :show-file-list="false">
                            <el-button size="small" type="primary">{{ lc('admin_user_company_00278') }}</el-button>
                        </el-upload>
                        <div style="margin-left: 15px;">
                            {{ fileName }}
                        </div>
                        <el-link style="margin-left: 15px;" v-if="ruleForm.exa_cert_wt" type="primary"
                                 :href="sy_weburl+'/'+ruleForm.exa_cert_wt">{{ lc('wap_com_00063') }}
                        </el-link>
                    </td>
                    <td>
                        <div class="TableShuom">
                            {{ lc('admin_00703') }}
                        </div>
                    </td>
                </tr>
                <!--功能开关-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00287') }}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('wap_com_00408')" v-model="ruleForm.com_message"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('wap_com_00291')" v-model="ruleForm.com_job_myswitch"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00293')" v-model="ruleForm.com_job_sexswitch"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00250')" v-model="ruleForm.com_free_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('wap_com_00103')" v-model="ruleForm.com_zpdata"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                        </div>
                    </td>
                </tr>
                <!--职位刷新-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('wap_com_00045') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_job_reserve" label="1">{{ lc('member_com_00267') }}</el-radio>
                            <el-radio v-model="ruleForm.com_job_reserve" label="0">{{ lc('wap_com_00239') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00228') }}<br>{{ lc('admin_user_company_00230') }}</span>
                        </div>
                    </td>
                </tr>
                <template v-if="ruleForm.com_job_reserve == 1">
                    <!--预约刷新时间间隔-->
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_user_company_00257') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_reserve_refresh_interval" :placeholder="lc('admin_user_00342')">
                                    <span slot="suffix" class="slotspan">{{ lc('common_02058') }}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_user_company_00235') }}</span>
                            </div>
                        </td>
                    </tr>
                    <!--预约刷新消费套餐-->
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_user_company_00258') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_reserve_refresh_price" :placeholder="lc('admin_user_00342')">
                                    <span slot="suffix" class="slotspan">{{ lc('common_02052') }}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_user_company_00240') }}</span>
                            </div>
                        </td>
                    </tr>
                    <!--刷新职位增值服务-->
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_user_company_00252') }}</div>
                        </td>
                        <td>
                            <div class="TableButn w_500">
                                <el-select v-model="ruleForm.sy_reserve_service_id" :placeholder="lc('wap_user_00100')">
                                    <el-option v-for="item in com_servers" :key="item.id" :label="item.name" :value="item.id"></el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>
                                    {{ lc('admin_00704') }}
                                    <el-button type="text" @click="handleView">{{ lc('wap_com_00427') }}>></el-button>
                                </span>
                            </div>
                        </td>
                    </tr>
                </template>
                <!--人才搜索-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00280') }}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00275')" v-model="ruleForm.com_search"
                                         @change="handleComSearch"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('admin_user_company_00274')" v-model="ruleForm.com_status_search"
                                         @change="handleComStatusSearch"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            {{ lc('admin_00705') }}
                        </div>
                    </td>
                </tr>
                <!--人才下载-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00279') }}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox :label="lc('wap_00190')" v-model="ruleForm.com_lietou_job"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            {{ lc('admin_00706') }}
                        </div>
                    </td>
                </tr>
                <!--企业搜索器-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00273') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.com_finder" :placeholder="lc('admin_user_00342')">
                                <span slot="suffix" class="slotspan">{{ lc('common_02050') }}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_00314') }}</span>
                        </div>
                    </td>
                </tr>
                <!--邀请面试模板数-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00264') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.com_yqmb_num" :placeholder="lc('admin_user_00342')">
                                <span slot="suffix" class="slotspan">{{ lc('common_02050') }}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00244') }}</span>
                        </div>
                    </td>
                </tr>
                <!--会员到期提醒-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00265') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sy_maturityday" :placeholder="lc('admin_user_00342')">
                                <span slot="suffix" class="slotspan">{{ lc('common_02067') }}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00229') }}</span>
                        </div>
                    </td>
                </tr>
                <!--展示职位投递数限制-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00249') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sy_sq_job_num" :placeholder="lc('admin_user_00342')">
                                <span slot="suffix" class="slotspan">{{ lc('common_02050') }}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00222') }}</span>
                        </div>
                    </td>
                </tr>
                <!--职位列表置顶（移动端首页）-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00246') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.joblist_top_index" label="0">{{ lc('admin_user_00340') }}</el-radio>
                            <el-radio v-model="ruleForm.joblist_top_index" label="1">{{ lc('admin_user_company_00285') }}</el-radio>
                            <el-radio v-model="ruleForm.joblist_top_index" label="2">{{ lc('admin_user_company_00298') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00707') }}<br/>{{ lc('admin_user_company_00242') }}</span>
                        </div>
                    </td>
                </tr>
                <!--职位列表置顶-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00708') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.joblist_top" label="1">{{ lc('admin_user_company_00285') }}</el-radio>
                            <el-radio v-model="ruleForm.joblist_top" label="0">{{ lc('admin_user_company_00277') }}</el-radio>
                            <el-radio v-model="ruleForm.joblist_top" label="2">{{ lc('admin_user_company_00295') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00227') }}<br>{{ lc('admin_user_company_00236') }}</span>
                        </div>
                    </td>
                </tr>
                <!--职位名称锁定-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00271') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.joblock" label="1">{{ lc('admin_user_00150') }}</el-radio>
                            <el-radio v-model="ruleForm.joblock" label="0">{{ lc('admin_user_company_00301') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00238') }}</span>
                        </div>
                    </td>
                </tr>
                <!--职位投递要求限制-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00256') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.sqjob_req" label="1">{{ lc('admin_user_company_00248') }}</el-radio>
                            <el-radio v-model="ruleForm.sqjob_req" label="0">{{ lc('admin_user_company_00243') }}</el-radio>
                            <el-radio v-model="ruleForm.sqjob_req" label="2">{{ lc('common.not_limited') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00234') }}</span>
                        </div>
                    </td>
                </tr>
                <!--名企排序-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00288') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.hotcom_top" label="0">{{ lc('admin_user_company_00266') }}</el-radio>
                            <el-radio v-model="ruleForm.hotcom_top" label="1">{{ lc('admin_user_company_00272') }}</el-radio>
                            <el-radio v-model="ruleForm.hotcom_top" label="2">{{ lc('admin_user_company_00305') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00232') }}</span>
                        </div>
                    </td>
                </tr>
                <!--会员到期职位下架-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00251') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.jobunder" label="1">{{ lc('wap_com_00245') }}</el-radio>
                            <el-radio v-model="ruleForm.jobunder" label="0">{{ lc('admin_user_company_00303') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00237') }}</span>
                        </div>
                    </td>
                </tr>
                <!--职位下架延期设置-->
                <template v-if="ruleForm.jobunder == '1'">
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_user_company_00255') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.job_under_delay" :placeholder="lc('admin_user_00342')">
                                    <span slot="suffix" class="slotspan">{{ lc('common_02067') }}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_user_company_00239') }}</span>
                            </div>
                        </td>
                    </tr>
                </template>
                <!--查看企业联系方式-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00254') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_link_look" label="1">{{ lc('admin_user_company_00296') }}</el-radio>
                            <el-radio v-model="ruleForm.com_link_look" label="2">{{ lc('admin_user_company_00292') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00225') }}</span>
                        </div>
                    </td>
                </tr>
                <tr v-if="ruleForm.com_link_look != '2'">
                    <td>
                        <div class="TableTite"></div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_login_link" label="1">{{ lc('admin_user_company_00304') }}</el-radio>
                            <el-radio v-model="ruleForm.com_login_link" label="2">{{ lc('admin_user_company_00300') }}</el-radio>
                            <el-radio v-model="ruleForm.com_login_link" label="3">{{ lc('admin_user_company_00276') }}</el-radio>
                            <el-radio v-model="ruleForm.com_login_link" label="4">{{ lc('admin_user_company_00294') }}</el-radio>
                            <el-radio v-model="ruleForm.com_login_link" label="5">{{ lc('wap_com_00235') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00231') }}</span>
                        </div>
                    </td>
                </tr>
                <!--屏蔽企业联系方式-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00253') }}</div>
                    </td>
                    <td>
                        <div class="TableButn w_500">
                            <el-select v-model="ruleForm.com_link_no" multiple :placeholder="lc('wap_user_00100')">
                                <el-option :label="lc('admin_user_company_00297')" value="0"></el-option>
                                <el-option v-for="item in qy_rows" :key="item.id" :label="item.name" :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00223') }}</span>
                        </div>
                    </td>
                </tr>
                <!--联系方式提示语-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00262') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sy_link_tips" maxlength="255"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00233') }}</span>
                        </div>
                    </td>
                </tr>
				<!--职位全文搜索-->
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_company_00270') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.job_full_text_search" label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio v-model="ruleForm.job_full_text_search" label="2">{{ lc('common.close') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_company_00224') }}<br>{{ lc('admin_user_company_00226') }}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{{ lc('common.submit') }}</el-button>
        </div>
        <div class="modluDrawer">
            <el-drawer :title="lc('admin_user_company_00290')" :visible.sync="serverVisible" :append-to-body="true"
                       :modal-append-to-body="false"
                       :show-close="true" :destroy-on-close="true" size="95%">
                <div style="overflow: hidden; position: relative; padding: 0 20px; height: 100%;">
                    <comrating_server></comrating_server>
                </div>
                
            </el-drawer>
        </div>
    </div>
</template>

<script>
module.exports = {
    data: function () {
        return {
            file_accept: localStorage.getItem("file_accept"),
            sy_weburl: localStorage.getItem("sy_weburl"),
            searchForm: {},
            qy_rows: [],
            com_servers: [],
            ruleForm: {
                // AuditInfo
                com_status: false,//企业会员
                com_job_status: false,//发布职位
                com_partjob_status: false,//发布兼职
                com_cert_status: false,//企业资质
                com_logo_status: false,//企业LOGO
                com_show_status: false,//企业环境
                com_banner_status: false,//企业横幅
                com_revise_status: false,//企业修改
                
                com_yqmb_status: false,//邀请面试模板
                //强制操作
                com_enforce_info: false,//信息完善
                com_enforce_mobilecert: false,//手机认证
                com_enforce_emailcert: false,//邮箱认证
                com_enforce_licensecert: false,//企业资质
                com_enforce_setposition: false,//地理位置
                com_gzgzh: false,//关注微信公众号
                //企业资质上传项
                com_social_credit: false,//统一社会信用代码
                com_cert_owner: false,//经办人身份证件
                com_cert_wt: false,//承诺函
                com_cert_other: false,//其他材料
                //委托书/承诺函范本
                exa_cert_wt: '',
                //{{ lc('admin_user_company_00287') }}
                com_message: false,//{{ lc('wap_com_00408') }}
                
                com_job_myswitch: false,//{{ lc('wap_com_00291') }}
                com_job_sexswitch: false,//{{ lc('admin_user_company_00293') }}
                com_free_status: false,//{{ lc('admin_user_company_00250') }}
                com_zpdata: false, // {{ lc('wap_com_00103') }}
                // Jobrefresh
                com_job_reserve: "0",//{{ lc('wap_com_00045') }}
                sy_reserve_refresh_interval: "",//{{ lc('admin_user_company_00257') }}
                sy_reserve_refresh_price: "",//{{ lc('admin_user_company_00258') }}
                sy_reserve_service_id: "",//{{ lc('admin_user_company_00252') }}
                //{{ lc('admin_user_company_00280') }}
                com_search: false,//{{ lc('admin_user_company_00275') }}
                com_status_search: false,//{{ lc('admin_user_company_00274') }}
                //{{ lc('admin_user_company_00279') }}
                com_lietou_job: false,//{{ lc('wap_00190') }}
                //{{ lc('admin_user_company_00273') }}
                com_finder: "",
                //{{ lc('admin_user_company_00264') }}
                com_yqmb_num: "",
                //{{ lc('admin_user_company_00265') }}
                sy_maturityday: "",
                //{{ lc('admin_user_company_00249') }}
                sy_sq_job_num: "",
                // {{ lc('wap_com_00238') }}（移动端首页）
                joblist_top_index: null,
                // JobListtop/sticky
                joblist_top: null,
                //{{ lc('admin_user_company_00271') }}
                joblock: "0",
                //{{ lc('admin_user_company_00256') }}
                sqjob_req: null,
                //{{ lc('admin_user_company_00288') }}
                hotcom_top: null,
                //{{ lc('admin_user_company_00251') }}
                jobunder: null,
                //{{ lc('admin_user_company_00255') }}
                job_under_delay: null,
                //{{ lc('admin_user_company_00254') }}
                com_link_look: null,
                com_login_link: null,
                //{{ lc('admin_user_company_00253') }}
                com_link_no: [],
                //{{ lc('admin_user_company_00262') }}
                sy_link_tips: "",
				//{{ lc('admin_user_company_00270') }}
                job_full_text_search:null
            },
            file: [],//暂存文件
            fileName: '',
            serverVisible: false,
            submitLoading: false,
            uploadAction: baseUrl + 'm=common&c=common_upload'
        }
    },
    created() {
        this.getList();
    },
    methods: {
        uploadChange(file) {
            //this.ruleForm.exa_cert_wt = URL.createObjectURL(file.raw);
            this.fileName = file.name;
            // 复刻文件信息
            this.file = file.raw;
        },
        getList() {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.searchForm));
            for (let index in params) {
                (params[index] === '') && (params[index] = null);
            }
            httpPost('m=user&c=company_comset&a=index', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.qy_rows = res.data.qy_rows;
                    _this.com_servers = res.data.com_servers;
                    let config = res.data.config ? res.data.config : {};
                    // AuditInfo
                    _this.ruleForm.com_status = config.com_status == '0' ? true : false;//{{ lc('admin_user_company_00281') }}
                    _this.ruleForm.com_job_status = config.com_job_status == "0" ? true : false;//{{ lc('common.publish_job') }}
                    _this.ruleForm.com_partjob_status = config.com_partjob_status == "0" ? true : false;//{{ lc('wap_00321') }}
                    _this.ruleForm.com_cert_status = config.com_cert_status == "1" ? true : false;//{{ lc('wap_com_00075') }}
                    _this.ruleForm.com_logo_status = config.com_logo_status == "1" ? true : false;//{{ lc('common.company') }}LOGO
                    _this.ruleForm.com_show_status = config.com_show_status == "1" ? true : false;//{{ lc('wap_user_00157') }}
                    _this.ruleForm.com_banner_status = config.com_banner_status == "1" ? true : false;//{{ lc('admin_user_company_00283') }}
                    _this.ruleForm.com_revise_status = config.com_revise_status == "0" ? true : false;//{{ lc('admin_user_company_00282') }}
                   
                    _this.ruleForm.com_yqmb_status = config.com_yqmb_status == "0" ? true : false;//{{ lc('admin_00709') }}
                    //{{ lc('admin_user_00337') }}
                    _this.ruleForm.com_enforce_info = config.com_enforce_info == "1" ? true : false;//{{ lc('admin_tool_00124') }}
                    _this.ruleForm.com_enforce_mobilecert = config.com_enforce_mobilecert == "1" ? true : false;//{{ lc('member_com_00071') }}
                    _this.ruleForm.com_enforce_emailcert = config.com_enforce_emailcert == "1" ? true : false;//{{ lc('wap_com_00186') }}
                    _this.ruleForm.com_enforce_licensecert = config.com_enforce_licensecert == "1" ? true : false;//{{ lc('wap_com_00075') }}
                    _this.ruleForm.com_enforce_setposition = config.com_enforce_setposition == "1" ? true : false;//{{ lc('admin_user_company_00289') }}
                    _this.ruleForm.com_gzgzh = config.com_gzgzh == "1" ? true : false;//{{ lc('admin_user_company_00260') }}
                    //{{ lc('admin_user_company_00259') }}
                    _this.ruleForm.com_social_credit = config.com_social_credit == "1" ? true : false;//{{ lc('admin_user_company_00063') }}
                    _this.ruleForm.com_cert_owner = config.com_cert_owner == "1" ? true : false;//{{ lc('admin_user_company_00261') }}
                    _this.ruleForm.com_cert_wt = config.com_cert_wt == "1" ? true : false;//{{ lc('admin_user_company_00302') }}
                    _this.ruleForm.com_cert_other = config.com_cert_other == "1" ? true : false;//{{ lc('member_com_00069') }}
                    //{{ lc('common_01902') }}/承诺函范本
                    _this.ruleForm.exa_cert_wt = config.exa_cert_wt !== undefined ? config.exa_cert_wt : "";
                    //{{ lc('admin_user_company_00287') }}
                    _this.ruleForm.com_message = config.com_message == "1" ? true : false;//{{ lc('wap_com_00408') }}
                    
                    _this.ruleForm.com_job_myswitch = config.com_job_myswitch == "1" ? true : false;//{{ lc('wap_com_00291') }}
                    _this.ruleForm.com_job_sexswitch = config.com_job_sexswitch == "1" ? true : false;//{{ lc('admin_user_company_00293') }}
                    _this.ruleForm.com_free_status = config.com_free_status == "1" ? true : false;//{{ lc('admin_user_company_00250') }}
                    _this.ruleForm.com_zpdata = config.com_zpdata == "1" ? true : false;//{{ lc('admin_user_company_00250') }}
                    // Jobrefresh
                    _this.ruleForm.com_job_reserve = config.com_job_reserve == '1' ? '1' : '0';//{{ lc('wap_com_00045') }}
                    _this.ruleForm.sy_reserve_refresh_interval = config.sy_reserve_refresh_interval !== undefined ? config.sy_reserve_refresh_interval : "";//{{ lc('admin_user_company_00257') }}
                    _this.ruleForm.sy_reserve_refresh_price = config.sy_reserve_refresh_price !== undefined ? config.sy_reserve_refresh_price : "";//{{ lc('admin_user_company_00258') }}
                    _this.ruleForm.sy_reserve_service_id = config.sy_reserve_service_id !== undefined ? config.sy_reserve_service_id : "";//{{ lc('admin_user_company_00252') }}
                    //{{ lc('admin_user_company_00280') }}
                    _this.ruleForm.com_search = config.com_search == "1" ? true : false;//{{ lc('admin_user_company_00275') }}
                    _this.ruleForm.com_status_search = config.com_status_search == "1" ? true : false;//{{ lc('admin_user_company_00274') }}
                    //{{ lc('admin_user_company_00279') }}
                    _this.ruleForm.com_lietou_job = config.com_lietou_job == "1" ? true : false;//{{ lc('wap_00190') }}
                    //{{ lc('admin_user_company_00273') }}
                    _this.ruleForm.com_finder = config.com_finder !== undefined ? config.com_finder : "";
                    //{{ lc('admin_user_company_00264') }}
                    _this.ruleForm.com_yqmb_num = config.com_yqmb_num !== undefined ? config.com_yqmb_num : "";
                    //{{ lc('admin_user_company_00265') }}
                    _this.ruleForm.sy_maturityday = config.sy_maturityday !== undefined ? config.sy_maturityday : "";
                    //{{ lc('admin_user_company_00249') }}
                    _this.ruleForm.sy_sq_job_num = config.sy_sq_job_num !== undefined ? config.sy_sq_job_num : "";
                    // {{ lc('wap_com_00238') }}（移动端首页）
                    _this.ruleForm.joblist_top_index = config.joblist_top_index !== undefined ? config.joblist_top_index : null;
                    // JobListtop/sticky
                    _this.ruleForm.joblist_top = config.joblist_top !== undefined ? config.joblist_top : null;
                    //{{ lc('admin_user_company_00271') }}
                    _this.ruleForm.joblock = config.joblock == '1' ? '1' : '0';
                    //{{ lc('admin_user_company_00256') }}
                    _this.ruleForm.sqjob_req = config.sqjob_req !== undefined ? config.sqjob_req : null;
                    //{{ lc('admin_user_company_00288') }}
                    _this.ruleForm.hotcom_top = config.hotcom_top !== undefined ? config.hotcom_top : null;
                    //{{ lc('admin_user_company_00251') }}
                    _this.ruleForm.jobunder = config.jobunder == '1' || config.jobunder == '0' ? config.jobunder : null;
                    //{{ lc('admin_user_company_00255') }}
                    _this.ruleForm.job_under_delay = config.job_under_delay !== undefined ? config.job_under_delay : null;
                    //{{ lc('admin_user_company_00254') }}
                    _this.ruleForm.com_link_look = config.com_link_look !== undefined ? config.com_link_look : null;
                    _this.ruleForm.com_login_link = config.com_login_link !== undefined ? config.com_login_link : null;
                    //{{ lc('admin_user_company_00253') }}
                    _this.ruleForm.com_link_no = res.data.com_link_no;
                    //{{ lc('admin_user_company_00262') }}
                    _this.ruleForm.sy_link_tips = config.sy_link_tips !== undefined ? config.sy_link_tips : "";
					//{{ lc('admin_user_company_00270') }}
                    _this.ruleForm.job_full_text_search = config.job_full_text_search !== undefined ? config.job_full_text_search : null;
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleComSearch(value) {
            if (!value) {
                this.ruleForm.com_status_search = false;
            }
        },
        handleComStatusSearch(value) {
            if (value) {
                this.ruleForm.com_search = true;
            }
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            let formData = new FormData();
            formData.append('config', lc('common.submit'));
            // AuditInfo
            formData.append('com_status', params.com_status ? 0 : 1);//{{ lc('admin_user_company_00281') }}
            formData.append("com_job_status", params.com_job_status ? 0 : 1);//{{ lc('common.publish_job') }}
            formData.append("com_partjob_status", params.com_partjob_status ? 0 : 1);//{{ lc('wap_00321') }}
            formData.append("com_cert_status", params.com_cert_status ? 1 : 0);//{{ lc('wap_com_00075') }}
            formData.append("com_logo_status", params.com_logo_status ? 1 : 2);//{{ lc('common.company') }}LOGO
            formData.append("com_show_status", params.com_show_status ? 1 : 0);//{{ lc('wap_user_00157') }}
            formData.append("com_banner_status", params.com_banner_status ? 1 : 0);//{{ lc('admin_user_company_00283') }}
            formData.append("com_revise_status", params.com_revise_status ? 0 : 1);//{{ lc('admin_user_company_00282') }}
            
            formData.append("com_yqmb_status", params.com_yqmb_status ? 0 : 1);//{{ lc('admin_00709') }}
            //{{ lc('admin_user_00337') }}
            formData.append("com_enforce_info", params.com_enforce_info ? 1 : 0);//{{ lc('admin_tool_00124') }}
            formData.append("com_enforce_mobilecert", params.com_enforce_mobilecert ? 1 : 0);//{{ lc('member_com_00071') }}
            formData.append("com_enforce_emailcert", params.com_enforce_emailcert ? 1 : 0);//{{ lc('wap_com_00186') }}
            formData.append("com_enforce_licensecert", params.com_enforce_licensecert ? 1 : 0);//{{ lc('wap_com_00075') }}
            formData.append("com_enforce_setposition", params.com_enforce_setposition ? 1 : 0);//{{ lc('admin_user_company_00289') }}
            formData.append("com_gzgzh", params.com_gzgzh ? 1 : 0);//{{ lc('admin_user_company_00260') }}
            //{{ lc('admin_user_company_00259') }}
            formData.append("com_social_credit", params.com_social_credit ? 1 : 0);//{{ lc('admin_user_company_00063') }}
            formData.append("com_cert_owner", params.com_cert_owner ? 1 : 0);//{{ lc('admin_user_company_00261') }}
            formData.append("com_cert_wt", params.com_cert_wt ? 1 : 0);//{{ lc('admin_user_company_00302') }}
            formData.append("com_cert_other", params.com_cert_other ? 1 : 0);//{{ lc('member_com_00069') }}
            //{{ lc('common_01902') }}/承诺函范本
            if (this.file.length !== 0) {
                formData.append("exa_cert_wt_files", this.file);
            }
            //{{ lc('admin_user_company_00287') }}
            formData.append("com_message", params.com_message ? 1 : 0);//{{ lc('wap_com_00408') }}
            
            formData.append("com_job_myswitch", params.com_job_myswitch ? 1 : 0);//{{ lc('wap_com_00291') }}
            formData.append("com_job_sexswitch", params.com_job_sexswitch ? 1 : 0);//{{ lc('admin_user_company_00293') }}
            formData.append("com_free_status", params.com_free_status ? 1 : 0);//{{ lc('admin_user_company_00250') }}
            formData.append("com_zpdata", params.com_zpdata ? 1 : 0);//{{ lc('admin_user_company_00250') }}
            // Jobrefresh
            formData.append("com_job_reserve", params.com_job_reserve);//{{ lc('wap_com_00045') }}
            formData.append("sy_reserve_refresh_interval", params.sy_reserve_refresh_interval);//{{ lc('admin_user_company_00257') }}
            formData.append("sy_reserve_refresh_price", params.sy_reserve_refresh_price);//{{ lc('admin_user_company_00258') }}
            formData.append("sy_reserve_service_id", params.sy_reserve_service_id);//{{ lc('admin_user_company_00252') }}
            //{{ lc('admin_user_company_00280') }}
            formData.append("com_search", params.com_search ? 1 : 0);//{{ lc('admin_user_company_00275') }}
            formData.append("com_status_search", params.com_status_search ? 1 : 0);//{{ lc('admin_user_company_00274') }}
            //{{ lc('admin_user_company_00279') }}
            formData.append("com_lietou_job", params.com_lietou_job ? 1 : 0);//{{ lc('wap_00190') }}
            //{{ lc('admin_user_company_00273') }}
            formData.append("com_finder", params.com_finder);
            //{{ lc('admin_user_company_00264') }}
            formData.append("com_yqmb_num", params.com_yqmb_num ? params.com_yqmb_num : 0);
            //{{ lc('admin_user_company_00265') }}
            formData.append("sy_maturityday", params.sy_maturityday);
            //{{ lc('admin_user_company_00249') }}
            formData.append("sy_sq_job_num", params.sy_sq_job_num ? params.sy_sq_job_num : 0);
            // {{ lc('wap_com_00238') }}（移动端首页）
            formData.append('joblist_top_index', params.joblist_top_index);
            // JobListtop/sticky
            formData.append('joblist_top', params.joblist_top);
            //{{ lc('admin_user_company_00271') }}
            formData.append("joblock", params.joblock);
            //{{ lc('admin_user_company_00256') }}
            formData.append("sqjob_req", params.sqjob_req);
            //{{ lc('admin_user_company_00288') }}
            formData.append("hotcom_top", params.hotcom_top);
            //{{ lc('admin_user_company_00251') }}
            formData.append("jobunder", params.jobunder);
            //{{ lc('admin_user_company_00255') }}
            formData.append("job_under_delay", params.job_under_delay);
            //{{ lc('admin_user_company_00254') }}
            formData.append('com_link_look', params.com_link_look);
            formData.append('com_login_link', params.com_login_link);
            //{{ lc('admin_user_company_00253') }}
            formData.append('com_link_no', params.com_link_no.join(','));
            //{{ lc('admin_user_company_00262') }}
            formData.append("sy_link_tips", params.sy_link_tips);
			//{{ lc('admin_user_company_00270') }}
            formData.append('job_full_text_search', params.job_full_text_search);
            _this.submitLoading = true;
            httpPost('m=user&c=company_comset&a=save', formData).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.resetForm();
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
        resetForm(formName) {
            //this.$refs[formName].resetFields();
            this.file = [];
            this.fileName = '';
        },
        handleView() {
            this.serverVisible = true;
        },
    },
    watch: {
        "ruleForm.sy_reserve_refresh_interval": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9]/g, '');
                    this.ruleForm.sy_reserve_refresh_interval = newValue;
                    if (newValue.length && newValue == 0) {
                        message.error(lc('admin_company_00022'));
                        this.ruleForm.sy_reserve_refresh_interval = 20;
                    }
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.sy_reserve_refresh_price": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9]/g, '');
                    this.ruleForm.sy_reserve_refresh_price = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.sy_reserve_service_id": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9]/g, '');
                    this.ruleForm.sy_reserve_service_id = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.com_finder": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9]/g, '');
                    this.ruleForm.com_finder = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.com_yqmb_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9]/g, '');
                    this.ruleForm.com_yqmb_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.sy_maturityday": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9]/g, '');
                    this.ruleForm.sy_maturityday = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.job_under_delay": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9]/g, '');
                    this.ruleForm.job_under_delay = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.sy_sq_job_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9]/g, '');
                    this.ruleForm.sy_sq_job_num = newValue;
                }
            },
            deep: true,
            immediate: true
        }
    },
    components: {
        'comrating_server': httpVueLoader('./comrating_server.vue'),
    }
};
</script>
<style scoped>
    .w_500 .el-input--suffix{width: 500px;}
</style>
