<template>
    <!--会员-企业-企业设置：企业设置-->
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">名称</th>
                    <th width="480">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>
                <!--审核信息-->
                <tr>
                    <td>
                        <div class="TableTite">审核信息</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox label="企业会员" v-model="ruleForm.com_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="发布岗位" v-model="ruleForm.com_job_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="发布兼职" v-model="ruleForm.com_partjob_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="企业资质" v-model="ruleForm.com_cert_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="企业logo" v-model="ruleForm.com_logo_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="企业环境" v-model="ruleForm.com_show_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="企业横幅" v-model="ruleForm.com_banner_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="企业修改" v-model="ruleForm.com_revise_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="邀请面试模板" v-model="ruleForm.com_yqmb_status"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            开启则需审核后展示
                        </div>
                    </td>
                </tr>
                <!--强制操作-->
                <tr>
                    <td>
                        <div class="TableTite">强制操作</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox label="完善信息" v-model="ruleForm.com_enforce_info"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="手机认证" v-model="ruleForm.com_enforce_mobilecert"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="邮箱认证" v-model="ruleForm.com_enforce_emailcert"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="企业资质" v-model="ruleForm.com_enforce_licensecert"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="地理位置" v-model="ruleForm.com_enforce_setposition"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="关注微信公众号" v-model="ruleForm.com_gzgzh"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            强制操作完成才能发布招聘信息
                        </div>
                    </td>
                </tr>
                <!--企业资质上传项-->
                <tr>
                    <td>
                        <div class="TableTite">企业资质上传项</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox label="统一社会信用代码" v-model="ruleForm.com_social_credit"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="经办人身份证件" v-model="ruleForm.com_cert_owner"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="承诺函" v-model="ruleForm.com_cert_wt"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="其他资料" v-model="ruleForm.com_cert_other"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            除了企业执照外,须上传的企业资质
                        </div>
                    </td>
                </tr>
                <!--委托书-->
                <tr>
                    <td>
                        <div class="TableTite">委托书/承诺函范本</div>
                    </td>
                    <td style="display: flex;align-items: center;">
                        <el-upload :accept="file_accept" :action="uploadAction" :on-change="uploadChange"
                                   :show-file-list="false">
                            <el-button size="small" type="primary">上传范本</el-button>
                        </el-upload>
                        <div style="margin-left: 15px;">
                            {{ fileName }}
                        </div>
                        <el-link style="margin-left: 15px;" v-if="ruleForm.exa_cert_wt" type="primary"
                                 :href="sy_weburl+'/'+ruleForm.exa_cert_wt">查看范本
                        </el-link>
                    </td>
                    <td>
                        <div class="TableShuom">
                            请上传文档类型文件
                        </div>
                    </td>
                </tr>
                <!--功能开关-->
                <tr>
                    <td>
                        <div class="TableTite">功能开关</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox label="求职咨询" v-model="ruleForm.com_message"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="薪资面议" v-model="ruleForm.com_job_myswitch"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="性别选项" v-model="ruleForm.com_job_sexswitch"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="认证企业职位免审核" v-model="ruleForm.com_free_status"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="招聘数据" v-model="ruleForm.com_zpdata"></el-checkbox>
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
                        <div class="TableTite">职位刷新</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_job_reserve" label="1">预约刷新</el-radio>
                            <el-radio v-model="ruleForm.com_job_reserve" label="0">自动刷新</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>预约刷新根据会员刷新套餐数量，设置刷新次数，每N分钟刷新一次，<br>自动刷新企业会员选择不同天数，支付一定金额，每日刷新一次</span>
                        </div>
                    </td>
                </tr>
                <template v-if="ruleForm.com_job_reserve == 1">
                    <!--预约刷新时间间隔-->
                    <tr>
                        <td>
                            <div class="TableTite">预约刷新时间间隔</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_reserve_refresh_interval" placeholder="请输入数字">
                                    <span slot="suffix" class="slotspan">分</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>最低N分钟，企业可自定义，不得低于后台设置参数</span>
                            </div>
                        </td>
                    </tr>
                    <!--预约刷新消费套餐-->
                    <tr>
                        <td>
                            <div class="TableTite">预约刷新消费套餐</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_reserve_refresh_price" placeholder="请输入数字">
                                    <span slot="suffix" class="slotspan">份</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>单个职位一次刷新所需消耗套餐数量；</span>
                            </div>
                        </td>
                    </tr>
                    <!--刷新职位增值服务-->
                    <tr>
                        <td>
                            <div class="TableTite">刷新职位增值服务</div>
                        </td>
                        <td>
                            <div class="TableButn w_500">
                                <el-select v-model="ruleForm.sy_reserve_service_id" placeholder="请选择">
                                    <el-option v-for="item in com_servers" :key="item.id" :label="item.name" :value="item.id"></el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>
                                    请选择刷新套餐增值服务；
                                    <el-button type="text" @click="handleView">查看>></el-button>
                                </span>
                            </div>
                        </td>
                    </tr>
                </template>
                <!--人才搜索-->
                <tr>
                    <td>
                        <div class="TableTite">人才搜索</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox label="已登录会员" v-model="ruleForm.com_search"
                                         @change="handleComSearch"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox label="已审核会员" v-model="ruleForm.com_status_search"
                                         @change="handleComStatusSearch"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            会员已登录、会员已审核才能搜索人才
                        </div>
                    </td>
                </tr>
                <!--人才下载-->
                <tr>
                    <td>
                        <div class="TableTite">人才下载</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox label="在招职位" v-model="ruleForm.com_lietou_job"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            有正在招聘的职位才能下载简历
                        </div>
                    </td>
                </tr>
                <!--企业搜索器-->
                <tr>
                    <td>
                        <div class="TableTite">企业搜索器</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.com_finder" placeholder="请输入数字">
                                <span slot="suffix" class="slotspan">个</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>数量太多，发送订阅邮件会很慢，为空则不限</span>
                        </div>
                    </td>
                </tr>
                <!--邀请面试模板数-->
                <tr>
                    <td>
                        <div class="TableTite">邀请面试模板数</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.com_yqmb_num" placeholder="请输入数字">
                                <span slot="suffix" class="slotspan">个</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>企业邀请面试可保存的模板数</span>
                        </div>
                    </td>
                </tr>
                <!--会员到期提醒-->
                <tr>
                    <td>
                        <div class="TableTite">会员到期提醒</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sy_maturityday" placeholder="请输入数字">
                                <span slot="suffix" class="slotspan">天</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>默认为30天,此设置将向站长发送邮件，请先配置站长Email</span>
                        </div>
                    </td>
                </tr>
                <!--展示职位投递数限制-->
                <tr>
                    <td>
                        <div class="TableTite">展示职位投递数限制</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sy_sq_job_num" placeholder="请输入数字">
                                <span slot="suffix" class="slotspan">个</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>控制职位详情是否展示投递数量；默认：0；在设置大于0的情况下，职位投递数超过设置数量，职位详情才会显示投递数量</span>
                        </div>
                    </td>
                </tr>
                <!--职位列表置顶（移动端首页）-->
                <tr>
                    <td>
                        <div class="TableTite">职位置顶（移动端首页）</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.joblist_top_index" label="0">不显示</el-radio>
                            <el-radio v-model="ruleForm.joblist_top_index" label="1">全部显示</el-radio>
                            <el-radio v-model="ruleForm.joblist_top_index" label="2">随机5条</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>全部显示：优先展示置顶岗位<br/>随机5条：随机展示5条置顶岗位</span>
                        </div>
                    </td>
                </tr>
                <!--职位列表置顶-->
                <tr>
                    <td>
                        <div class="TableTite">职位置顶（列表）</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.joblist_top" label="1">全部显示</el-radio>
                            <el-radio v-model="ruleForm.joblist_top" label="0">随机20条</el-radio>
                            <el-radio v-model="ruleForm.joblist_top" label="2">搜索置顶</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>全部显示、随机20条：任意搜索项，展示符合搜索结果的置顶职位，<br>搜索置顶：搜索项中必须有职位分类或关键字</span>
                        </div>
                    </td>
                </tr>
                <!--职位名称锁定-->
                <tr>
                    <td>
                        <div class="TableTite">职位名称锁定</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.joblock" label="1">锁定</el-radio>
                            <el-radio v-model="ruleForm.joblock" label="0">不锁定</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>若选择“锁定”，则职位名称发布后不可修改</span>
                        </div>
                    </td>
                </tr>
                <!--职位投递要求限制-->
                <tr>
                    <td>
                        <div class="TableTite">职位投递要求限制</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.sqjob_req" label="1">不符合条件不可投递</el-radio>
                            <el-radio v-model="ruleForm.sqjob_req" label="0">不符合条件自动标记为不合适</el-radio>
                            <el-radio v-model="ruleForm.sqjob_req" label="2">不限制</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>“条件”指的是发布职位时填写的“投递要求设置”</span>
                        </div>
                    </td>
                </tr>
                <!--名企排序-->
                <tr>
                    <td>
                        <div class="TableTite">名企排序</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.hotcom_top" label="0">后台手动设置</el-radio>
                            <el-radio v-model="ruleForm.hotcom_top" label="1">职位更新时间</el-radio>
                            <el-radio v-model="ruleForm.hotcom_top" label="2">随机</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>“后台手动设置”是指添加后台添加名企时设置的排序</span>
                        </div>
                    </td>
                </tr>
                <!--会员到期职位下架-->
                <tr>
                    <td>
                        <div class="TableTite">会员到期职位下架</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.jobunder" label="1">下架</el-radio>
                            <el-radio v-model="ruleForm.jobunder" label="0">保持</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>若选择“下架”，则会员到期后职位自动下架</span>
                        </div>
                    </td>
                </tr>
                <!--职位下架延期设置-->
                <template v-if="ruleForm.jobunder == '1'">
                    <tr>
                        <td>
                            <div class="TableTite">职位下架延期设置</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.job_under_delay" placeholder="请输入数字">
                                    <span slot="suffix" class="slotspan">天</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>会员到期职位自动下架，可以延期X天下架</span>
                            </div>
                        </td>
                    </tr>
                </template>
                <!--查看企业联系方式-->
                <tr>
                    <td>
                        <div class="TableTite">查看企业联系方式</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_link_look" label="1">普通模式</el-radio>
                            <el-radio v-model="ruleForm.com_link_look" label="2">微信扫码</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>微信扫码模式，PC和微信客户端内扫码后，会将联系方式通过配置的微信公众号发送</span>
                        </div>
                    </td>
                </tr>
                <tr v-if="ruleForm.com_link_look != '2'">
                    <td>
                        <div class="TableTite"></div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_login_link" label="1">开放</el-radio>
                            <el-radio v-model="ruleForm.com_login_link" label="2">不开放</el-radio>
                            <el-radio v-model="ruleForm.com_login_link" label="3">登录后显示</el-radio>
                            <el-radio v-model="ruleForm.com_login_link" label="4">拥有简历</el-radio>
                            <el-radio v-model="ruleForm.com_login_link" label="5">投递简历</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>若选择“开放”，则进入企业自身联系方式设置条件判断</span>
                        </div>
                    </td>
                </tr>
                <!--屏蔽企业联系方式-->
                <tr>
                    <td>
                        <div class="TableTite">屏蔽企业联系方式</div>
                    </td>
                    <td>
                        <div class="TableButn w_500">
                            <el-select v-model="ruleForm.com_link_no" multiple placeholder="请选择">
                                <el-option label="过期会员" value="0"></el-option>
                                <el-option v-for="item in qy_rows" :key="item.id" :label="item.name" :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>选中会员等级，对应的的企业联系方式即为不开放状态。本项设置优先度高于上面“查看企业联系方式”设置</span>
                        </div>
                    </td>
                </tr>
                <!--联系方式提示语-->
                <tr>
                    <td>
                        <div class="TableTite">联系方式提示语</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sy_link_tips" maxlength="255"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>联系方式未公开的情况下，求职者查看联系方式的提示</span>
                        </div>
                    </td>
                </tr>
				<!--职位全文搜索-->
                <tr>
                    <td>
                        <div class="TableTite">职位全文搜索</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.job_full_text_search" label="1">开启</el-radio>
                            <el-radio v-model="ruleForm.job_full_text_search" label="2">关闭</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>如开启职位全文搜索，查询职位关键词将匹配职位名称、企业名称、城市分类、职位描述、职位类别<br>如关闭职位全文搜索，查询职位关键词将匹配职位名称、企业名称、城市分类</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">提交</el-button>
        </div>
        <div class="modluDrawer">
            <el-drawer title="增值服务" :visible.sync="serverVisible" :append-to-body="true"
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
                //审核信息
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
                //功能开关
                com_message: false,//求职咨询
                
                com_job_myswitch: false,//薪资面议
                com_job_sexswitch: false,//性别选项
                com_free_status: false,//认证企业职位免审核
                com_zpdata: false, // 招聘数据
                //职位刷新
                com_job_reserve: '0',//职位刷新
                sy_reserve_refresh_interval: '',//预约刷新时间间隔
                sy_reserve_refresh_price: '',//预约刷新消费套餐
                sy_reserve_service_id: '',//刷新职位增值服务
                //人才搜索
                com_search: false,//已登录会员
                com_status_search: false,//已审核会员
                //人才下载
                com_lietou_job: false,//在招职位
                //企业搜索器
                com_finder: '',
                //邀请面试模板数
                com_yqmb_num: '',
                //会员到期提醒
                sy_maturityday: '',
                //展示职位投递数限制
                sy_sq_job_num: '',
                // 职位置顶（移动端首页）
                joblist_top_index: null,
                //职位列表置顶
                joblist_top: null,
                //职位名称锁定
                joblock: '0',
                //职位投递要求限制
                sqjob_req: null,
                //名企排序
                hotcom_top: null,
                //会员到期职位下架
                jobunder: null,
                //职位下架延期设置
                job_under_delay: null,
                //查看企业联系方式
                com_link_look: null,
                com_login_link: null,
                //屏蔽企业联系方式
                com_link_no: [],
                //联系方式提示语
                sy_link_tips: '',
				//职位全文搜索
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
                    //审核信息
                    _this.ruleForm.com_status = config.com_status == '0' ? true : false;//企业会员
                    _this.ruleForm.com_job_status = config.com_job_status == '0' ? true : false;//发布职位
                    _this.ruleForm.com_partjob_status = config.com_partjob_status == '0' ? true : false;//发布兼职
                    _this.ruleForm.com_cert_status = config.com_cert_status == '1' ? true : false;//企业资质
                    _this.ruleForm.com_logo_status = config.com_logo_status == '1' ? true : false;//企业LOGO
                    _this.ruleForm.com_show_status = config.com_show_status == '1' ? true : false;//企业环境
                    _this.ruleForm.com_banner_status = config.com_banner_status == '1' ? true : false;//企业横幅
                    _this.ruleForm.com_revise_status = config.com_revise_status == '0' ? true : false;//企业修改
                   
                    _this.ruleForm.com_yqmb_status = config.com_yqmb_status == '0' ? true : false;//邀请面试模板
                    //强制操作
                    _this.ruleForm.com_enforce_info = config.com_enforce_info == '1' ? true : false;//信息完善
                    _this.ruleForm.com_enforce_mobilecert = config.com_enforce_mobilecert == '1' ? true : false;//手机认证
                    _this.ruleForm.com_enforce_emailcert = config.com_enforce_emailcert == '1' ? true : false;//邮箱认证
                    _this.ruleForm.com_enforce_licensecert = config.com_enforce_licensecert == '1' ? true : false;//企业资质
                    _this.ruleForm.com_enforce_setposition = config.com_enforce_setposition == '1' ? true : false;//地理位置
                    _this.ruleForm.com_gzgzh = config.com_gzgzh == '1' ? true : false;//关注微信公众号
                    //企业资质上传项
                    _this.ruleForm.com_social_credit = config.com_social_credit == '1' ? true : false;//统一社会信用代码
                    _this.ruleForm.com_cert_owner = config.com_cert_owner == '1' ? true : false;//经办人身份证件
                    _this.ruleForm.com_cert_wt = config.com_cert_wt == '1' ? true : false;//承诺函
                    _this.ruleForm.com_cert_other = config.com_cert_other == '1' ? true : false;//其他材料
                    //委托书/承诺函范本
                    _this.ruleForm.exa_cert_wt = config.exa_cert_wt !== undefined ? config.exa_cert_wt : '';
                    //功能开关
                    _this.ruleForm.com_message = config.com_message == '1' ? true : false;//求职咨询
                    
                    _this.ruleForm.com_job_myswitch = config.com_job_myswitch == '1' ? true : false;//薪资面议
                    _this.ruleForm.com_job_sexswitch = config.com_job_sexswitch == '1' ? true : false;//性别选项
                    _this.ruleForm.com_free_status = config.com_free_status == '1' ? true : false;//认证企业职位免审核
                    _this.ruleForm.com_zpdata = config.com_zpdata == '1' ? true : false;//认证企业职位免审核
                    //职位刷新
                    _this.ruleForm.com_job_reserve = config.com_job_reserve == '1' ? '1' : '0';//职位刷新
                    _this.ruleForm.sy_reserve_refresh_interval = config.sy_reserve_refresh_interval !== undefined ? config.sy_reserve_refresh_interval : '';//预约刷新时间间隔
                    _this.ruleForm.sy_reserve_refresh_price = config.sy_reserve_refresh_price !== undefined ? config.sy_reserve_refresh_price : '';//预约刷新消费套餐
                    _this.ruleForm.sy_reserve_service_id = config.sy_reserve_service_id !== undefined ? config.sy_reserve_service_id : '';//刷新职位增值服务
                    //人才搜索
                    _this.ruleForm.com_search = config.com_search == '1' ? true : false;//已登录会员
                    _this.ruleForm.com_status_search = config.com_status_search == '1' ? true : false;//已审核会员
                    //人才下载
                    _this.ruleForm.com_lietou_job = config.com_lietou_job == '1' ? true : false;//在招职位
                    //企业搜索器
                    _this.ruleForm.com_finder = config.com_finder !== undefined ? config.com_finder : '';
                    //邀请面试模板数
                    _this.ruleForm.com_yqmb_num = config.com_yqmb_num !== undefined ? config.com_yqmb_num : '';
                    //会员到期提醒
                    _this.ruleForm.sy_maturityday = config.sy_maturityday !== undefined ? config.sy_maturityday : '';
                    //展示职位投递数限制
                    _this.ruleForm.sy_sq_job_num = config.sy_sq_job_num !== undefined ? config.sy_sq_job_num : '';
                    // 职位置顶（移动端首页）
                    _this.ruleForm.joblist_top_index = config.joblist_top_index !== undefined ? config.joblist_top_index : null;
                    //职位列表置顶
                    _this.ruleForm.joblist_top = config.joblist_top !== undefined ? config.joblist_top : null;
                    //职位名称锁定
                    _this.ruleForm.joblock = config.joblock == '1' ? '1' : '0';
                    //职位投递要求限制
                    _this.ruleForm.sqjob_req = config.sqjob_req !== undefined ? config.sqjob_req : null;
                    //名企排序
                    _this.ruleForm.hotcom_top = config.hotcom_top !== undefined ? config.hotcom_top : null;
                    //会员到期职位下架
                    _this.ruleForm.jobunder = config.jobunder == '1' || config.jobunder == '0' ? config.jobunder : null;
                    //职位下架延期设置
                    _this.ruleForm.job_under_delay = config.job_under_delay !== undefined ? config.job_under_delay : null;
                    //查看企业联系方式
                    _this.ruleForm.com_link_look = config.com_link_look !== undefined ? config.com_link_look : null;
                    _this.ruleForm.com_login_link = config.com_login_link !== undefined ? config.com_login_link : null;
                    //屏蔽企业联系方式
                    _this.ruleForm.com_link_no = res.data.com_link_no;
                    //联系方式提示语
                    _this.ruleForm.sy_link_tips = config.sy_link_tips !== undefined ? config.sy_link_tips : '';
					//职位全文搜索
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
            formData.append('config', '提交');
            //审核信息
            formData.append('com_status', params.com_status ? 0 : 1);//企业会员
            formData.append('com_job_status', params.com_job_status ? 0 : 1);//发布职位
            formData.append('com_partjob_status', params.com_partjob_status ? 0 : 1);//发布兼职
            formData.append('com_cert_status', params.com_cert_status ? 1 : 0);//企业资质
            formData.append('com_logo_status', params.com_logo_status ? 1 : 2);//企业LOGO
            formData.append('com_show_status', params.com_show_status ? 1 : 0);//企业环境
            formData.append('com_banner_status', params.com_banner_status ? 1 : 0);//企业横幅
            formData.append('com_revise_status', params.com_revise_status ? 0 : 1);//企业修改
            
            formData.append('com_yqmb_status', params.com_yqmb_status ? 0 : 1);//邀请面试模板
            //强制操作
            formData.append('com_enforce_info', params.com_enforce_info ? 1 : 0);//信息完善
            formData.append('com_enforce_mobilecert', params.com_enforce_mobilecert ? 1 : 0);//手机认证
            formData.append('com_enforce_emailcert', params.com_enforce_emailcert ? 1 : 0);//邮箱认证
            formData.append('com_enforce_licensecert', params.com_enforce_licensecert ? 1 : 0);//企业资质
            formData.append('com_enforce_setposition', params.com_enforce_setposition ? 1 : 0);//地理位置
            formData.append('com_gzgzh', params.com_gzgzh ? 1 : 0);//关注微信公众号
            //企业资质上传项
            formData.append('com_social_credit', params.com_social_credit ? 1 : 0);//统一社会信用代码
            formData.append('com_cert_owner', params.com_cert_owner ? 1 : 0);//经办人身份证件
            formData.append('com_cert_wt', params.com_cert_wt ? 1 : 0);//承诺函
            formData.append('com_cert_other', params.com_cert_other ? 1 : 0);//其他材料
            //委托书/承诺函范本
            if (this.file.length !== 0) {
                formData.append('exa_cert_wt_files', this.file);
            }
            //功能开关
            formData.append('com_message', params.com_message ? 1 : 0);//求职咨询
            
            formData.append('com_job_myswitch', params.com_job_myswitch ? 1 : 0);//薪资面议
            formData.append('com_job_sexswitch', params.com_job_sexswitch ? 1 : 0);//性别选项
            formData.append('com_free_status', params.com_free_status ? 1 : 0);//认证企业职位免审核
            formData.append('com_zpdata', params.com_zpdata ? 1 : 0);//认证企业职位免审核
            //职位刷新
            formData.append('com_job_reserve', params.com_job_reserve);//职位刷新
            formData.append('sy_reserve_refresh_interval', params.sy_reserve_refresh_interval);//预约刷新时间间隔
            formData.append('sy_reserve_refresh_price', params.sy_reserve_refresh_price);//预约刷新消费套餐
            formData.append('sy_reserve_service_id', params.sy_reserve_service_id);//刷新职位增值服务
            //人才搜索
            formData.append('com_search', params.com_search ? 1 : 0);//已登录会员
            formData.append('com_status_search', params.com_status_search ? 1 : 0);//已审核会员
            //人才下载
            formData.append('com_lietou_job', params.com_lietou_job ? 1 : 0);//在招职位
            //企业搜索器
            formData.append('com_finder', params.com_finder);
            //邀请面试模板数
            formData.append('com_yqmb_num', params.com_yqmb_num ? params.com_yqmb_num : 0);
            //会员到期提醒
            formData.append('sy_maturityday', params.sy_maturityday);
            //展示职位投递数限制
            formData.append('sy_sq_job_num', params.sy_sq_job_num ? params.sy_sq_job_num : 0);
            // 职位置顶（移动端首页）
            formData.append('joblist_top_index', params.joblist_top_index);
            //职位列表置顶
            formData.append('joblist_top', params.joblist_top);
            //职位名称锁定
            formData.append('joblock', params.joblock);
            //职位投递要求限制
            formData.append('sqjob_req', params.sqjob_req);
            //名企排序
            formData.append('hotcom_top', params.hotcom_top);
            //会员到期职位下架
            formData.append('jobunder', params.jobunder);
            //职位下架延期设置
            formData.append('job_under_delay', params.job_under_delay);
            //查看企业联系方式
            formData.append('com_link_look', params.com_link_look);
            formData.append('com_login_link', params.com_login_link);
            //屏蔽企业联系方式
            formData.append('com_link_no', params.com_link_no.join(','));
            //联系方式提示语
            formData.append('sy_link_tips', params.sy_link_tips);
			//职位全文搜索
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
                        message.error('请填写时间间隔！');
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
