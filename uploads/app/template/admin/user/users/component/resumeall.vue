<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-input placeholder="请输入你要搜索的关键字" @keyup.enter.native="search" size="small" v-model="searchForm.keyword"
                          clearable>
					<el-select v-model="searchForm.keytype" style="padding-left: 12px;" size="small" slot="prepend" placeholder="期望职位">
					    <el-option label="期望职位" :value="1"></el-option>
					    <el-option label="姓名" :value="2"></el-option>
					    <el-option label="简历ID" :value="3"></el-option>
					    <el-option label="用户名" :value="4"></el-option>
					    <el-option label="手机号" :value="5"></el-option>
					    <el-option label="教育经历" :value="6"></el-option>
					    <el-option label="工作经历" :value="7"></el-option>
					    <el-option label="项目经历" :value="8"></el-option>
					    <el-option label="培训经历" :value="9"></el-option>
					    <el-option label="职业技能" :value="10"></el-option>
					    <el-option label="IP" :value="11"></el-option>
					</el-select>
                </el-input>
            </div>
            <!--收起部分-->
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="searchForm.time_type" size="small" slot="prepend" placeholder="筛选日期" clearable @change="handleTimeChange">
                    <el-option label="创建时间" value="adtime"></el-option>
                    <el-option label="更新时间" value="uptime"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmalltwo" :class="{ 'searchbutnOnff': seachbutn }">
                <el-date-picker v-model="searchForm.times" type="daterange" align="right" unlink-panels range-separator="至" start-placeholder="开始日期" end-placeholder="结束日期" :picker-options="timeOptions" value-format="yyyy-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
            </div>
            <div v-for="(searchItem, searchIndex) in searchList" :key="searchIndex" class="tableSeachInpt tableSeachInptsmall"
                :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="searchForm[searchItem.param]" slot="prepend" :clearable="true"
                    :placeholder="searchItem.name" size="small" @change="search">
                    <el-option v-for="(searchLabel, searchValue) in searchItem.value" :key="searchValue" :label="searchLabel"
                        :value="searchValue"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt" :class="{ 'searchbutnOnff': seachbutn }">
                <div class="block">
                    <!--7.0 统一类别选择-->
                    <job_class @confirm="confirmJobSearch"></job_class>
                </div>
            </div>
            <div class=" tableSeachInpt" :class="{ 'searchbutnOnff': seachbutn }">
                <div class="block">
                    <!--7.0 统一城市选择-->
                    <city_class @confirm="confirmCitySearch"></city_class>
                </div>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">查询</el-button>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" plain icon="el-icon-plus" size="mini" @click="openAdd">新增简历</el-button>

            </div>
            <div class="tableSeachInpt tableSeachzk" :class="{ 'searchbutnKai': seachbutn }">
                <el-button type="info" class="zhankai" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>展开<i class="el-icon-arrow-down el-icon--right"></i>
                </el-button>
                <el-button type="info" class="shouqi" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>合并<i class="el-icon-arrow-up el-icon--right"></i>
                </el-button>
            </div>
        </div>
        <div class="admin_datatip">
            <i class="el-icon-document"></i> 数据统计：共 <span class="cp_n" @click="init">{{ resumeAllNum }}</span> 条
            <span class="admin_datatip_n">未审核：<span class="cp_n" @click="statusSearch('4')">{{ resumeStatusNum1 ? resumeStatusNum1 : 0 }}</span> 条</span>
            <span class="admin_datatip_n">未通过：<span class="cp_n" @click="statusSearch('3')">{{ resumeStatusNum2 ? resumeStatusNum2 : 0 }}</span> 条</span>
            <span class="admin_datatip_n">已锁定：<span class="cp_n" @click="statusSearch('2')">{{ resumeStatusNum3 ? resumeStatusNum3 : 0 }}</span> 条</span>
            <span class="admin_datatip_n">未成年：<span class="cp_n" @click="statusSearch('1')">{{ resumeTeenNum ? resumeTeenNum : 0 }}</span> 条</span>
            <span class="admin_datatip_n">搜索结果： {{ total }} 条</span>
        </div>
        <div class="moduleElTable" :class="{ 'moduleElTabGetResuma': tableHig }"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="list" style="width: 100%" stripe ref="multipleTable" @selection-change="handleSelectionChange"


                @mousedown.native="mouseDownHandler"
                @mouseup.native="mouseUpHandler"
                @mousemove.native="mouseMoveHandler"


                @sort-change="sortChange" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="50"></el-table-column>
                <el-table-column prop="id" label="简历ID" width="90" sortable="custom"></el-table-column>
                <el-table-column label="姓名/手机号" min-width="165">
                    <template slot-scope="scope">
                        <div class=" ">
                            <div class="username">
                                <el-link type="primary" :underline="false" @click="toMember(scope.row)">{{ scope.row.uname}}</el-link>
                            </div>
                            <div class=" ">
                                {{ scope.row.moblie }}
								<div class="telgsd" v-if="scope.row.moblie_address">{{  scope.row.moblie_address }}</div>
                            </div>

                             <!-- <span class="gsd">{{ scope.row.username }}</span>
                          <a href="index.php?m=user_member&c=Imitate&uid={yun:}$v.uid{/yun}" target="_blank" class="admin_com_name" >{yun:}$v.username{/yun}</a>-->
                        </div>
                    </template>

                </el-table-column>
                <el-table-column label="基本信息" min-width="230">
                    <template slot-scope="scope">
                        <div class=" ">
                            <div>
                                <span class="user_resumejob" @click="openPreview(scope.row)">{{ scope.row.name }}</span>
                               <!-- <span v-if="scope.row.defaults == 1" class="user_resumrmr">默认</span>-->
                            </div>
                            <div class="">
                                {{ scope.row.age_n }}岁
                                .{{ scope.row.sex_n }}
                                <span v-if="scope.row.edu_n">.{{ scope.row.edu_n }}学历</span>
                                <span v-if="scope.row.exp_n">.{{ scope.row.exp_n }}经验</span>
                            </div>
                            <div class="">
                                <span class="gsd">
                                    <el-tooltip effect="dark" :disabled="scope.row.citynum <= 1"
                                        :content="scope.row.cityall" placement="top" v-if="scope.row.city_n">
                                        <span>{{ scope.row.city_n }}.</span>
                                    </el-tooltip>
                                   <!-- <span>{{ scope.row.salary }} </span>-->

                                </span>
                            </div>
                        </div>
                    </template>

                </el-table-column>

                <el-table-column label="完整度/状态" min-width="110" align="center">
                    <template slot-scope="scope">
                        <el-tag type="danger" size="small" effect="dark" v-if="scope.row.integrity < 65">{{ scope.row.integrity }}%</el-tag>
						<el-tag type="success" size="small" effect="dark" v-else>{{ scope.row.integrity }}%</el-tag>

                        <div v-if="scope.row.status == 1" class="jlzt">
                            <el-button type="text" @click="openStatus(scope.row)"><i class="el-icon-unlock"></i> 公开
                            </el-button>
                        </div>
                        <div v-else-if="scope.row.status == 3" class="jlzt">
                            <el-button type="text" @click="openStatus(scope.row)"><i class="el-icon-unlock"></i> 投递可见
                            </el-button>
                        </div>
                        <div v-else class="jlwgk jlzt">
                            <el-button type="text" @click="openStatus(scope.row)"><i class="el-icon-lock"></i> 未公开
                            </el-button>
                        </div>
                    </template>

                </el-table-column>
                <el-table-column label="投递岗位" width="80" align="center">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            {{ scope.row.sq_num ? scope.row.sq_num : 0 }}
                        </div>
                        <div class="moduleProps" v-if="scope.row.sq_num > 0">
                            <span class="jobtj">
                                <el-button type="text" @click="openJobSqlLog(scope.row)">查看</el-button>
                            </span>
                        </div>
                    </template>

                </el-table-column>
				<el-table-column prop="comd" label="简历状态" min-width="130">
				    <template slot-scope="scope">
				        <div class="job_tg_bth">
				            <el-switch v-model="scope.row.rec_resume" inactive-text="推荐" :width="30" active-value="1"
				                inactive-value="0" @change="changeRec($event, scope.row)">
				            </el-switch>
				        </div>
				        <div class="job_tg_bth jobBthChufa">
				            <!--因点击switch会触发值的改变，固需遮罩层触发事件-->
				            <div class="chufaButn" @click="openTop(scope.row)">事件</div>
				            <el-switch v-model="scope.row.top_day" inactive-text="置顶" :width="30"
				                :active-value="scope.row.top_day > 0 ? scope.row.top_day : 1"
				                inactive-value="0"></el-switch>
				        </div>
				    </template>
				</el-table-column>
                <el-table-column prop="logintime" label="更新/创建时间 " min-width="150">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span class="gsd">{{ scope.row.lastupdate_n }}</span>
                            <span>{{ scope.row.ctime_n }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="ip" label="来源/IP/归属地" min-width="150">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span>{{ source[scope.row.source] }}<span v-if="scope.row.doc == 1">粘贴</span></span>
                            <span v-if="scope.row.add_ip">{{ scope.row.add_ip }}</span>
                            <span v-if="scope.row.ip_address" class="gsd"> {{ scope.row.ip_address }}</span>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column prop="zt" label="状态" width="120" fixed="right">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <div v-if="scope.row.r_status == '2'">
                                <span class="admin_state3">已锁定</span>
                                <div style="display:inline-block" v-if="scope.row.lock_info">
                                    <el-popover trigger="hover" placement="right">
                                        <p>{{ scope.row.lock_info }}</p>
                                        <div slot="reference" class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div>
                                    </el-popover>
                                </div>
                            </div>
                            <span v-else-if="scope.row.state == 1" class="admin_state1">已审核</span>
                            <span v-else-if="scope.row.state == 3" class="admin_state2">
                                未通过
                                <el-tooltip effect="dark" :content="scope.row.statusbody" placement="top"
                                    v-if="scope.row.statusbody">
                                    <i class="el-icon-warning-outline"></i>
                                </el-tooltip>
                            </span>
                            <span v-else-if="scope.row.state == 2" class="admin_state3">被举报</span>
                            <span v-else class="admin_state5">未审核</span>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column label="操作" width="140" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="openAudit(scope.row)">审核</el-button>

                            <el-button size="mini" plain @click="refresh(scope.row)">刷新</el-button>
                        </div>
                        <div class="cz_button" style="margin-top: 10px;">
                            <el-button size="mini" plain @click="openRemark(scope.row)">备注</el-button>

                            <el-button type="danger" size="mini" @click="openDel(scope.$index)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging" style="height: initial; flex-wrap: wrap; padding-top: 10px;">
            <div class="bottomButnBull" style="width:100%;">
                <div class="bottomButnBlak">
                    <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
                    @change="checkAll">全选</el-checkbox>
                    <el-button size="mini" @click="batch('del')">批量删除</el-button>
                    <el-button size="mini" @click="batch('audit')">批量审核</el-button>
                    <el-button size="mini" @click="batch('refresh')">批量刷新</el-button>
                    <el-button size="mini" @click="batch('rec')">批量推荐</el-button>
                    <el-button size="mini" @click="batch('rec_cancel')">取消推荐</el-button>
                    <el-button size="mini" @click="batch('top')">批量置顶</el-button>
                    <el-button size="mini" @click="batch('top_cancel')">取消置顶</el-button>
                    <el-button size="mini" @click="batch('export')">导出</el-button>
                </div>
            </div>
            <div class="modulePagNum" style="padding-top: 8px;">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page="page" :page-sizes="pageSizes" :page-size="limit"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <div class="modluDrawer">
            <!-- 导出字段选择弹出 -->
            <el-dialog title="选择导出字段" :visible.sync="dialogExport" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="650px">
                <div class="tck_setname">
                    <el-checkbox-group v-model="ruleFormExport.type" @change="handleCheckedExportType">
                        <el-checkbox :label="field" v-for="(fieldName, field) in typeExport" :key="field">{{ fieldName }}</el-checkbox>
                    </el-checkbox-group>
                    <el-checkbox :indeterminate="isIndeterminateExport" v-model="checkAllExport"
                        @change="handleCheckAllExport">全选</el-checkbox>
                </div>
                <div class="daochuNumer">
                    <div class="daochuTite">
                        <span>导出数量</span>
                    </div>
                    <div class="daochuFrom">
                        <div class="daochuFroInpt">
                            <el-input v-model="ruleFormExport.limit"
                                @input="inputIntNumber($event, 'ruleFormExport', 'limit')"></el-input>
                        </div>
                        <div>
                            <el-alert :closable="false" title="数字太大会导致运行缓慢，请慎重填写" type="info" show-icon>
                            </el-alert>
                        </div>
                    </div>

                    <!-- <span>
                        <el-input v-model="ruleFormExport.limit"
                            @input="inputIntNumber($event, 'ruleFormExport', 'limit')"></el-input>
                    </span>
                    <el-alert :closable="false" title="数字太大会导致运行缓慢，请慎重填写" type="info" show-icon>
                    </el-alert> -->
                </div>
                <div class="daochuNumer">
                    <div class="daochuTite">
                        <span>导出区段</span>
                    </div>
                    <div class="daochuFrom">
                        <div class="daochuFroInpt">
                            <el-input v-model="ruleFormExport.section"></el-input>
                        </div>
                        <div>
                            <el-alert :closable="false" title="例如101,100  从101条开始导出100条" type="info" show-icon>
                            </el-alert>
                        </div>
                    </div>

                    <!-- <span><el-input v-model="ruleFormExport.section"></el-input></span>
                    <el-alert :closable="false" title="例如101,100  从101条开始导出100条" type="info" show-icon>
                    </el-alert> -->
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogExport = false">取 消</el-button>
                    <el-button type="primary" @click="submitExport" :disabled="saveLoading">确 认</el-button>
                </span>
            </el-dialog>
        </div>
        <!--公开简历-->
        <div class="modluDrawer">
            <el-dialog title="简历状态" :visible.sync="dialogStatus" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div class="wxsettip_small ">姓名</div>
                <el-input :value="detail.uname" :disabled="true"></el-input>
                <div class="wxsettip_small ">简历状态</div>
                <div class="wxsettip_Sealect">
                    <el-select v-model="ruleFormStatus.status" placeholder="请选择">
                        <el-option key="1" label="公开" value="1"></el-option>
                        <el-option key="3" label="投递企业可见" value="3"></el-option>
                        <el-option key="2" label="隐藏" value="2"></el-option>
                    </el-select>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogStatus = false">取 消</el-button>
                    <el-button type="primary" @click="submitStatus" :disabled="saveLoading">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--简历置顶-->
        <div class="modluDrawer">
            <el-dialog title="简历置顶" :visible.sync="dialogTop" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div class="wxsettip_small ">置顶天数</div>
                <el-input v-model="ruleFormTop.addday" @input="inputIntNumber($event, 'ruleFormTop', 'addday')">
                    <template slot="append">天</template>
                </el-input>
                <template v-if="detail.top_day > 0">
                    <div class="danqainDataFlex">
                        <div class="wxsettip_small ">当前结束日期：</div>
                        <div style="color:#f60">{{ detail.topdate_n }}</div>
                    </div>

                </template>
                <div>
                    如需取消置顶简历请单击 <el-checkbox v-model="ruleFormTop.s" true-label="1" false-label="0"></el-checkbox> 点击确定即可
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogTop = false">取 消</el-button>
                    <el-button type="primary" @click="submitTop" :disabled="saveLoading">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--简历备注-->
        <div class=" ">
            <el-dialog title="简历备注" :visible.sync="dialogRemark" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div class="wxsettip_small ">简历标签</div>
                <div class="wxsettip_Sealect">
                    <el-select v-model="ruleFormRemark.label" placeholder="请选择">
                        <el-option v-for="labelkey in userdata.user_label" :key="labelkey" :label="userclass_name[labelkey]"
                            :value="labelkey">
                        </el-option>
                    </el-select>
                </div>
                <div class="wxsettip_small ">客服评价</div>
                <el-input v-model="ruleFormRemark.content" type="textarea" placeholder="请输入客服评价"></el-input>

                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogRemark = false">取 消</el-button>
                    <el-button type="primary" @click="submitRemark">确 定</el-button>
                </span>
            </el-dialog>
        </div>

        <!--投递记录-->
        <el-drawer title="投递岗位记录" :append-to-body="true" :visible.sync="drawerJobSqLog" size="80%">
            <div class="uploadTable" style="padding:0px 20px;font-size:14px;color:#666">
                <div class="moduleElHight">
                    <div class="moduleElTable moduleElMoreInt" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
                        <el-table :data="jobSqLog.list" style="width: 100%" stripe ref="table2"
                            :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading">
                            <template slot="empty">
                                <p>{{dataText}}</p>
                            </template>
                            <el-table-column prop="job_name" label="投递职位">
                                <template slot-scope="scope">
                                    <div class="moduleProps">
                                        <el-link type="primary" :underline="false"
                                            @click="openPage(scope.row.job_comapply)">{{ scope.row.job_name }}</el-link>
                                    </div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="com_name" label="所属企业">
                                <template slot-scope="scope">
                                    <div class="moduleProps">
                                        <el-link type="primary" :underline="false"
                                            @click="openPage(scope.row.company_show)">{{ scope.row.com_name }}</el-link>
                                    </div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="datetime_n_n" label="投递时间"></el-table-column>
                            <el-table-column label="是否查看">
                                <template slot-scope="scope">
                                    <div class="admin_state">
                                        <span class="admin_state1" v-if="scope.row.is_browse == 2">已查看</span>
                                        <span class="admin_state2" v-else-if="scope.row.is_browse == 3">待通知</span>
                                        <span class="admin_state3" v-else-if="scope.row.is_browse == 4">不合适</span>
                                        <span class="admin_state4" v-else-if="scope.row.is_browse == 5">未接通</span>
                                        <span class="admin_state5" v-else>未查看</span>
                                    </div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="isdel_n" label="状态"></el-table-column>
                        </el-table>
                    </div>
                    <div class="modulePaging">
                        <div></div>
                        <div class="modulePagNum">
                            <el-pagination background @size-change="handleSizeChangeJobSqlLog"
                                @current-change="handleCurrentChangeJobSqlLog" :current-page="jobSqLog.page"
                                :page-sizes="jobSqLog.pageSizes" :page-size="jobSqLog.limit"
                                layout="total, sizes, prev, pager, next, jumper" :total="jobSqLog.total">
                            </el-pagination>
                        </div>
                    </div>
                </div>
            </div>
        </el-drawer>
        <!--批量审核-->
        <el-dialog title="批量审核" :visible.sync="dialogAudit" :modal-append-to-body="false" :show-close="true" width="500px">
            <div class="toolClasDia fenpeizhand">
                <div class="toolClasList">
                    <div class="toolClasTite">
                        <span>审核操作：</span>
                    </div>
                    <div class="toolClasCont">
                        <el-radio v-model="ruleFormAudit.status" label="1">正常</el-radio>
                        <el-radio v-model="ruleFormAudit.status" label="3">未通过</el-radio>
                    </div>
                </div>
                <div class="toolClasList">
                    <div class="toolClasTite">
                        <span>审核说明：</span>
                    </div>
                    <div class="toolClasCont">
                        <el-input type="textarea" :rows="2" placeholder="" v-model="ruleFormAudit.statusbody">
                        </el-input>
                    </div>
                </div>
            </div>
            <span slot="footer" class="dialog-footer">
                <el-button @click="dialogAudit = false">取 消</el-button>
                <el-button type="primary" @click="submitBatchAudit">确 定</el-button>
            </span>
        </el-dialog>
        <!--简历审核-->
        <el-drawer title="简历详情" :visible.sync="drawerAudit" @closed="closedAudit"
            :modal-append-to-body="false" size="90%" :append-to-body="true">
            <div class="shbox" style="padding-right: 380px;;" v-loading="expectLoading">
                <div style="overflow-y: auto;position: relative;height: 100%; padding-right: 25px; border-right: 1px solid #eee;">
                    <div class="shshow_tit">
                        <i class="el-icon-office-building"></i> 基本资料
                        <span class="shshow_cz">
                            <el-button type="text" @click="openBasic">
                                <i class="el-icon-edit"></i>编辑资料
                            </el-button>
                        </span>
                    </div>
                    <div class="userinfo_box">
                        <div class="userinfo_l"><img :src="resume.photo" width="70" height="70"></div>
                        <div class="userinfo_r">
                            <div class="userinfo_name">{{ resume.name }}</div>
                            <div class="userinfo">
                                {{ resume.sex_n }}
                                <span v-if="resume.age">，{{ resume.age }}岁</span>
                                <span v-if="resume.height">，{{ resume.height }}cm</span>
                                <span v-if="resume.weight">，{{ resume.weight }}kg</span>
                                <span v-if="resume.marriage_n">，{{ resume.marriage_n }}</span>
                                <span v-if="resume.living">，现居{{ resume.living }}</span>
                            </div>
                            <div class="userinfo" v-if="resume.edu_n || resume.exp_n">
                                <span v-if="resume.edu_n">{{ resume.edu_n }}学历 </span>
                                <span class="userline" v-if="resume.edu_n && resume.exp_n">|</span>
                                <span v-if="resume.exp_n">{{ resume.exp_n }}经验</span>
                            </div>
                        </div>
                    </div>
                    <div class="shshow_p">
                        <div class="cominfo" v-if="resume.telphone"><i class="el-icon-mobile"></i>
                            联系电话：{{ resume.telphone }}</div>
                        <div class="cominfo" v-if="resume.email"><i class="el-icon-message"></i>
                            联系邮箱：{{ resume.email }}</div>
                        <div class="cominfo" v-if="resume.idcard"><i class="el-icon-postcard"></i>
                            身份证号：{{ resume.idcard }}</div>
                        <div class="cominfo" v-if="resume.domicile"><i class="el-icon-location-outline"></i>
                            户籍所在地：{{ resume.domicile }}</div>
                        <div class="cominfo" v-if="resume.address"><i class="el-icon-location-information"></i>
                            详细地址：{{ resume.address }}</div>
                    </div>

                    <!--个人优势-->
                    <div class="user_resume_list">
                        <div class="shshow_tit">
                            <i class="el-icon-medal-1"></i> 个人优势
                        </div>
                        <div class="shshow_p">
                            <el-tag size="mini" v-for="(tagItem,key) in resume.arrayTag" :key="key">{{ tagItem }}</el-tag>
                            <div class="cominfo">{{ resume.description }}</div>
                        </div>
                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">总结优势，突出亮点，个人优势将突出展示给HR</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openTag">
                                    <i class="el-icon-circle-plus-outline"></i> {{ (resume.arrayTag &&
                                        resume.arrayTag.length > 0) || resume.description ? '编辑' : '添加' }}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!--求职意向-->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-notebook-2"></i> 求职意向</div>
                        <div class="shshow_p" v-if="expectData.expect">
                            <div class="cominfo">期望职位： {{ expectData.expect.name }} </div>
                            <div class="cominfo">从事职位： {{ expectData.expect.job_classname }}</div>
                            <div class="cominfo">期望地点： {{ expectData.expect.city_classname }}</div>
                            <div class="cominfo">期望薪资： {{ expectData.expect.salary }}</div>
                            <div class="cominfo">从事行业： {{ expectData.expect.hy_n }}</div>
                            <div class="cominfo">到岗时间： {{ expectData.expect.report_n }}</div>
                            <div class="cominfo">工作性质： {{ expectData.expect.type_n }}</div>
                            <div class="cominfo">求职状态： {{ expectData.expect.jobstatus_n }}</div>
                        </div>


                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">建议完善求职偏好</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openJob">
                                    <i class="el-icon-circle-plus-outline"></i> {{ expectData.expect ? '编辑' : '添加' }}
                                </el-button>
                            </div>
                        </div>
                    </div>

                    <!--工作经历-->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-suitcase-1"></i> 工作经历</div>
                        <!--循环-->
                        <div class="user_resume_show" v-for="(work, workkey) in expectData.work" :key="workkey">
                            <div class="user_resume_addname ">{{ work.name }}
                                <el-button type="text" @click="openWork(workkey)">
                                    <i class="el-icon-edit"></i> 修改
                                </el-button>
                                <el-button type="text" @click="delResumeFb('work', workkey, work.id)">
                                    <i class="el-icon-delete"></i> 删除
                                </el-button>
                            </div>
                            <div class="user_resume_addjy">
                                <div class=" ">{{ work.title }}</div>
                                <div class="user_resume_time">{{ work.sdate_n }}-{{ work.edate_n }}</div>
                            </div>
                            <div class="user_resume_ms">{{ work.content }}</div>
                        </div>
                        <!--循环-->
                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">展示工作经验、工作能力否符合岗位要求的重要依据</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openWork('')">
                                    <i class="el-icon-circle-plus-outline"></i> 添加
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!--教育经历-->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-school"></i> 教育经历</div>
                        <!--循环-->
                        <div class="user_resume_show" v-for="(edu, edukey) in expectData.edu" :key="edukey">
                            <div class="user_resume_addname ">{{ edu.name }}
                                <el-button type="text" @click="openEdu(edukey)">
                                    <i class="el-icon-edit"></i> 修改
                                </el-button>
                                <el-button type="text" @click="delResumeFb('edu', edukey, edu.id)">
                                    <i class="el-icon-delete"></i> 删除
                                </el-button>
                            </div>
                            <div class="user_resume_addjy">
                                <div class=" ">{{ edu.specialty }}<span class="userline"
                                        v-if="edu.specialty && edu.education_n">|</span>{{ edu.education_n }}</div>
                                <div class="user_resume_time">{{ edu.sdate_n }}-{{ edu.edate_n }}</div>
                            </div>
                        </div>
                        <!--循环-->
                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">补足HR对学历背景的了解</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openEdu('')">
                                    <i class="el-icon-circle-plus-outline"></i> 添加
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!--培训经历-->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-data-analysis"></i> 培训经历</div>
                        <!--循环-->
                        <div class="user_resume_show" v-for="(training, trainingKey) in expectData.training" :key="trainingKey">
                            <div class="user_resume_addname ">{{ training.name }}
                                <el-button type="text" @click="openTraining(trainingKey)">
                                    <i class="el-icon-edit"></i> 修改
                                </el-button>
                                <el-button type="text" @click="delResumeFb('training', trainingKey, training.id)">
                                    <i class="el-icon-delete"></i> 删除
                                </el-button>
                            </div>
                            <div class="user_resume_addjy">
                                <div class=" ">{{ training.title }} </div>
                                <div class="user_resume_time">{{ training.sdate_n }}-{{ training.edate_n }}</div>
                            </div>
                            <div class="user_resume_ms">{{ training.content }}</div>
                        </div>
                        <!--循环-->

                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">展示培训经验否符合岗位要求的重要依据</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openTraining('')">
                                    <i class="el-icon-circle-plus-outline"></i> 添加
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!--职业技能-->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-reading"></i> 职业技能</div>
                        <!--循环-->
                        <div class="user_resume_show" v-for="(skill, skillkey) in expectData.skill" :key="skillkey">
                            <div class="user_resume_addname ">{{ skill.name }}
                                <el-button type="text" @click="openSkill(skillkey)">
                                    <i class="el-icon-edit"></i> 修改
                                </el-button>
                                <el-button type="text" @click="delResumeFb('skill', skillkey, skill.id)">
                                    <i class="el-icon-delete"></i> 删除
                                </el-button>
                            </div>
                            <div class="user_resume_addjy">
                                <div class=" ">{{ skill.ing_n }} </div>
                                <div class="user_resume_time">{{ skill.longtime }} 年</div>
                            </div>
                            <div class="user_resume_ms" v-if="skill.pic">
                                <img :src="skill.pic" width="95" height="70" :preview-src-list="skill.pic">
                            </div>
                        </div>
                        <!--循环-->

                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">技能专长建议填写职业技能为简历加分</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openSkill('')">
                                    <i class="el-icon-circle-plus-outline"></i> 添加
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!--项目经历-->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-wallet"></i> 项目经历</div>
                        <!--循环-->
                        <div class="user_resume_show" v-for="(project, projectkey) in expectData.project" :key="projectkey">
                            <div class="user_resume_addname ">{{ project.name }}
                                <el-button type="text" @click="openProject(projectkey)">
                                    <i class="el-icon-edit"></i> 修改
                                </el-button>
                                <el-button type="text" @click="delResumeFb('project', projectkey, project.id)">
                                    <i class="el-icon-delete"></i> 删除
                                </el-button>
                            </div>
                            <div class="user_resume_addjy">
                                <div class=" ">{{ project.title }}</div>
                                <div class="user_resume_time">{{ project.sdate_n }}-{{ project.edate_n }}</div>
                            </div>
                            <div class="user_resume_ms">{{ project.content }}</div>
                        </div>
                        <!--循环-->

                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">展示工作经验、能力，这也是HR判断是否符合岗位要求的重要依据。</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openProject('')">
                                    <i class="el-icon-circle-plus-outline"></i> 添加
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!--其他描述-->
                    <div class="user_resume_list" style="padding-bottom:80px; ;">
                        <div class="shshow_tit"><i class="el-icon-mic"></i> 其他描述</div>
                        <!--循环-->
                        <div class="user_resume_show" v-for="(other, otherkey) in expectData.other" :key="otherkey">
                            <div class="user_resume_addname ">{{ other.name }}
                                <el-button type="text" @click="openOther(otherkey)">
                                    <i class="el-icon-edit"></i> 修改
                                </el-button>
                                <el-button type="text" @click="delResumeFb('other', otherkey, other.id)">
                                    <i class="el-icon-delete"></i> 删除
                                </el-button>
                            </div>
                            <div class="user_resume_ms">{{ other.content }}</div>
                        </div>
                        <!--循环-->
                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">其他加分补充</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openOther('')">
                                    <i class="el-icon-circle-plus-outline"></i> 添加
                                </el-button>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="shcz" style="top:60px;right:30px;">
                    <template v-if="detail.r_status == 2">
                        <div class="wxsettip_small ">简历审核</div>
                        <template>
                            <el-radio-group v-model="ruleFormAudit.r_status">
                                <el-radio label="1">正常</el-radio>
                                <el-radio label="2">锁定</el-radio>
                            </el-radio-group>
                            <el-alert v-if="detail.lock_info" :closable="false" :title="'锁定原因：' + detail.lock_info"
                                type="warning" show-icon>
                            </el-alert>
                        </template>
                    </template>
                    <template v-if="ruleFormAudit.r_status == 1">
                        <div class="wxsettip_small ">简历审核</div>
                        <template>
                            <el-radio-group v-model="ruleFormAudit.status">
                                <el-radio label="1">正常</el-radio>
                                <el-radio label="3">未通过</el-radio>
                            </el-radio-group>
                        </template>
                        <div class="wxsettip_small ">审核说明模板</div>
                        <el-select v-model="auditTpl" placeholder="请选择" @change="changeTpl">
                            <el-option v-for="auditkey in userdata.user_audit" :key="auditkey"
                                :label="userclass_name[auditkey]" :value="auditkey">
                            </el-option>
                        </el-select>
                        <div class="wxsettip_small ">审核说明</div>
                        <el-input type="textarea" :rows="2" v-model="ruleFormAudit.statusbody">
                        </el-input>
                        <template v-if="ruleFormAudit.content">
                            <div class="wxsettip_small ">备注信息</div>
                            <el-input type="textarea" :rows="2" v-model="ruleFormAudit.content">
                            </el-input>
                        </template>
                        <div class=" shczbth">
                            <el-button type="primary" @click="submitAudit(1)">提 交</el-button>
                        </div>
                        <div v-if="todoAuditNum > 0" class=" shczbth">
                            <el-button type="primary" @click="submitAudit(2)" plain>提交，并审核下一个</el-button>
                        </div>
                    </template>
                </div>
            </div>
        </el-drawer>

        <!---编辑简历 基本资料-->
        <el-drawer title="编辑基本资料" :append-to-body="true" :visible.sync="drawerBasic" :wrapper-closable="false" size="60%">
            <div class="uploadTable" style="padding:0px 20px;">
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="120">名称</th>
                            <th width=" ">状态</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">姓名</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.name" placeholder="请输入姓名"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">性别</div>
                            </td>
                            <td>
                                <div class="job_set_list">
                                    <el-radio-group v-model="ruleFormBasic.sex">
                                        <el-radio v-for="(sex, sexkey) in user_sex" :key="sexkey" :label="sexkey">{{ sex }}</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">出生年月</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-date-picker v-model="ruleFormBasic.birthday" type="month" placeholder="选择月">
                                    </el-date-picker>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">最高学历</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormBasic.edu" placeholder="请选择">
                                        <el-option v-for="edukey in userdata.user_edu" :key="edukey"
                                            :label="userclass_name[edukey]" :value="edukey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">工作经验</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormBasic.exp" placeholder="请选择">
                                        <el-option v-for="wordkey in userdata.user_word" :key="wordkey"
                                            :label="userclass_name[wordkey]" :value="wordkey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">联系电话</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.telphone" placeholder="请输入联系电话"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">联系邮箱</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.email" placeholder="请输入联系邮箱"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">身份证号</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.idcard" placeholder="请输入身份证号"
                                        @input="inputIdcard($event, 'ruleFormBasic', 'idcard')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">户籍所在地</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.domicile" placeholder="请输入户籍所在地"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">现居地</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.living" placeholder="请输入现居地"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">详细地址</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.address" placeholder="请输入详细地址"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">身高</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.height" placeholder="请输入身高"
                                        @input="inputFloatNumber($event, 'ruleFormBasic', 'height')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">体重</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.weight" placeholder="请输入体重"
                                        @input="inputFloatNumber($event, 'ruleFormBasic', 'weight')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">婚姻状况</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormBasic.marriage" placeholder="请选择">
                                        <el-option v-for="marriagekey in userdata.user_marriage" :key="marriagekey"
                                            :label="userclass_name[marriagekey]" :value="marriagekey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">民族</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.nationality" placeholder="请输入民族"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">个人主页/博客</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.homepage" placeholder="请输入个人主页/博客"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">QQ</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.qq" placeholder="请输入QQ"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">个人二维码</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept" action="" :auto-upload="false"
                                        :on-change="handleChangeWxewm" :show-file-list="false">
                                        <img v-if="ruleFormBasic.wxewm_n" :src="ruleFormBasic.wxewm_n" class="avatar">
                                        <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                                    </el-upload>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">自我介绍</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" placeholder="请自我介绍下吧"
                                        v-model="ruleFormBasic.description">
                                    </el-input>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="setBasicButn" style="border: none; height: 80px;">
                <el-button type="primary" size="medium" @click="submitBasic">提交</el-button>
            </div>


        </el-drawer>
        <!---编辑求职意向-->
        <el-drawer title="编辑求职意向" :append-to-body="true" :visible.sync="drawerJob" :wrapper-closable="false" size="60%">
            <div class="uploadTable" style="padding:0px 20px;">
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="120">名称</th>
                            <th width=" ">状态</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">期望职位</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormJob.name" placeholder="请输入期望职位">
                                    </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">从事职位</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <!--7.0 统一类别选择-->
                                    <job_class multiple :max="5" @confirm="confirmJob" :selected="jobSelected"></job_class>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">期望地点</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <!--7.0 统一城市选择-->
                                    <city_class multiple :max="5" @confirm="confirmCity" :selected="citySelected"></city_class>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">期望薪资</div>
                            </td>
                            <td>
                                <div class="TableInpt" style="max-width: 700px;">
                                    <el-select v-model="ruleFormJob.minsalary" placeholder="请选择" @change="salaryChange" style="margin-right:8px;">
                                        <el-option v-for="maxsalary1Val in minsalaryList" :key="maxsalary1Val" :label="maxsalary1Val" :value="maxsalary1Val">
                                        </el-option>
                                    </el-select>
                                    <el-select v-model="ruleFormJob.maxsalary" placeholder="请选择">
                                        <el-option v-for="maxsalary2Val in maxsalaryList" :key="maxsalary2Val" :label="maxsalary2Val" :value="maxsalary2Val">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">从事行业</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormJob.hy" placeholder="请选择">
                                        <el-option v-for="industrykey in industry_index" :key="industrykey"
                                            :label="industry_name[industrykey]" :value="industrykey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">到岗时间</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormJob.report" placeholder="请选择">
                                        <el-option v-for="reportkey in userdata.user_report" :key="reportkey"
                                            :label="userclass_name[reportkey]" :value="reportkey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">工作性质</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormJob.type" placeholder="请选择">
                                        <el-option v-for="typekey in userdata.user_type" :key="typekey"
                                            :label="userclass_name[typekey]" :value="typekey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">求职状态</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormJob.jobstatus" placeholder="请选择">
                                        <el-option v-for="jobstatuskey in userdata.user_jobstatus" :key="jobstatuskey"
                                            :label="userclass_name[jobstatuskey]" :value="jobstatuskey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="setBasicButn" style="border: none; height: 80px;">
                <el-button type="primary" size="medium" @click="submitJob">提交</el-button>
            </div>
        </el-drawer>

        <!---编辑个人优势-->
        <div class="modluDrawer">
            <el-dialog title="个人优势" :visible.sync="dialogTag" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">优势标签</div>
                    <div class="wxsettipBiaoqin">
                        <el-tag :key="tagkey" v-for="(tag, tagkey) in userTag" :disable-transitions="false"
                            @click="checkTag(tag)" :effect="ruleFormTag.tag.indexOf(tag) > -1 ? 'dark' : 'light'">
                            {{ tag }}
                        </el-tag>
                        <el-input class="input-new-tag" v-if="inputTag" v-model="tagval"
                            autofoucs size="small" @keyup.enter.native="confirmTag">
                        </el-input>
                        <el-button v-else class="button-new-tag" size="small" @click="showTag">+ 新增
                        </el-button>
                    </div>
                    <div class="wxsettip_small ">自我评价</div>
                    <el-input type="textarea"
                        :placeholder="'用一两句话总结一下自己的优势，突出亮点。\n示例：\n2年销售管理经验，在担任区域负责人期间，带领区域同事做到移动业务量全省第一。口齿伶俐、思维灵敏，管理组织能力强，精通各种营销手段。'"
                        v-model="ruleFormTag.description" :autosize="{ minRows: 3, maxRows: 6 }">
                    </el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogTag = false">取 消</el-button>
                    <el-button type="primary" @click="submitTag">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑工作经历-->
        <div class="modluDrawer">
            <el-dialog title="工作经历" :visible.sync="dialogWork" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">公司名称</div>
                    <div class=""><el-input v-model="ruleFormWork.name" placeholder="请输入公司名称"></el-input> </div>
                    <div class="wxsettip_small ">担任职位</div>
                    <div class=""><el-input v-model="ruleFormWork.title" placeholder="请输入担任职位"></el-input> </div>
                    <div class="wxsettip_small ">工作时间</div>
                    <div class="wxsettip_Sealect" style="display: flex; align-items: center;">
                        <el-date-picker v-model="ruleFormWork.sdate" type="month" placeholder="选择开始时间">
                        </el-date-picker>
                        <el-date-picker style="margin: 0 8px;" :disabled="todayCheck" v-model="ruleFormWork.edate"
                            type="month" placeholder="选择结束时间">
                        </el-date-picker>
                        <el-checkbox v-model="todayCheck" @change="todayChange($event, 'work')">至今</el-checkbox>
                    </div>
                    <div class="wxsettip_small ">工作内容</div>
                    <el-input type="textarea" :placeholder="'请简短介绍公司与自己负责的任务，分条罗列在什么项目中，通过某些动作或技能达到可量化的结果。\n示例：\n1、主要负责新员工入职培训；\n2、分析制定员工每月个人销售业绩；\n3、帮助员工提高每日客单价，整体店面管理工作等；'"
                              v-model="ruleFormWork.content" :autosize="{ minRows: 3, maxRows: 6 }">
                    </el-input>
                </div>

                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogWork = false">取 消</el-button>
                    <el-button type="primary" @click="submitWork">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑学历-->
        <div class="modluDrawer">
            <el-dialog title="教育经历" :visible.sync="dialogEdu" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">学校名称</div>
                    <div class=""><el-input v-model="ruleFormEdu.name" placeholder="请输入学校名称"></el-input> </div>
                    <div class="wxsettip_small ">在校时间</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeEdu" type="monthrange" range-separator="至"
                            start-placeholder="开始日期" end-placeholder="结束日期">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">最高学历</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="ruleFormEdu.education" placeholder="请选择">
                            <el-option v-for="edukey in userdata.user_edu" :key="edukey" :label="userclass_name[edukey]"
                                :value="edukey">
                            </el-option>
                        </el-select>
                    </div>
                    <div class="wxsettip_small ">所学专业</div>
                    <div class=""><el-input v-model="ruleFormEdu.specialty" placeholder="请输入专业名称"></el-input> </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogEdu = false">取 消</el-button>
                    <el-button type="primary" @click="submitEdu">确 定</el-button>
                </span>
            </el-dialog>
        </div>

        <!---编辑培训经历-->
        <div class="modluDrawer">
            <el-dialog title="培训经历" :visible.sync="dialogTraining" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">培训中心</div>
                    <div class=""><el-input v-model="ruleFormTraining.name" placeholder="请输入培训中心"></el-input> </div>
                    <div class="wxsettip_small ">培训方向</div>
                    <div class=""><el-input v-model="ruleFormTraining.title" placeholder="请输入培训方向"></el-input> </div>
                    <div class="wxsettip_small ">培训时间</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeTraining" type="monthrange" range-separator="至"
                            start-placeholder="开始日期" end-placeholder="结束日期">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">培训描述</div>
                    <el-input type="textarea" placeholder="请简短介绍培训经历所获收获等；" v-model="ruleFormTraining.content"
                        :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogTraining = false">取 消</el-button>
                    <el-button type="primary" @click="submitTraining">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑项目经历-->
        <div class="modluDrawer">
            <el-dialog title="项目经历" :visible.sync="dialogProject" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">项目名称</div>
                    <div class=""><el-input v-model="ruleFormProject.name" placeholder="请输入项目名称"></el-input> </div>
                    <div class="wxsettip_small ">担任职务</div>
                    <div class=""><el-input v-model="ruleFormProject.title" placeholder="请输入担任职务"></el-input> </div>
                    <div class="wxsettip_small ">项目时间</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeProject" type="monthrange" range-separator="至"
                            start-placeholder="开始日期" end-placeholder="结束日期">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">项目描述</div>
                    <el-input type="textarea" placeholder="请简短介绍公司与自己负责的任务，分条罗列在什么项目中，通过某些动作或技能达到可量化的结果。
示例：
1、主要负责新员工入职培训；
2、分析制定员工每月个人销售业绩；
3、帮助员工提高每日客单价，整体店面管理工作等；" v-model="ruleFormProject.content" :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogProject = false">取 消</el-button>
                    <el-button type="primary" @click="submitProject">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑其他-->
        <div class="modluDrawer">
            <el-dialog title="其他加分项" :visible.sync="dialogOther" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">标题</div>
                    <div class=""><el-input v-model="ruleFormOther.name" placeholder="请输入标题名称"></el-input> </div>
                    <div class="wxsettip_small ">描述</div>
                    <el-input type="textarea" v-model="ruleFormOther.content" placeholder="请简短介绍其他加分优势"
                        :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogOther = false">取 消</el-button>
                    <el-button type="primary" @click="submitOther">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑技能-->
        <div class="modluDrawer">
            <el-dialog title="职业技能" :visible.sync="dialogSkill" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">技能名称</div>
                    <div class=""><el-input v-model="ruleFormSkill.name" placeholder="请输入技能名称"></el-input> </div>
                    <div class="wxsettip_small ">掌握时间</div>
                    <div class="wxsettip_Sealect">
                        <el-input v-model="ruleFormSkill.longtime" placeholder="请输入掌握时间">
                            <template slot="append">年</template>
                        </el-input>
                    </div>
                    <div class="wxsettip_small ">熟练程度</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="ruleFormSkill.ing" placeholder="请选择">
                            <el-option v-for="ingkey in userdata.user_ing" :key="ingkey" :label="userclass_name[ingkey]"
                                :value="ingkey">
                            </el-option>
                        </el-select>
                    </div>
                    <div class="wxsettip_small ">技能证书</div>
                    <div>
                        <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept" action="" :auto-upload="false"
                            :on-change="handleChangeSkillPic" :show-file-list="false">
                            <img v-if="ruleFormSkill.pic_n" :src="ruleFormSkill.pic_n" class="avatar">
                            <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                        </el-upload>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogSkill = false">取 消</el-button>
                    <el-button type="primary" @click="submitSkill">确 定</el-button>
                </span>
            </el-dialog>
        </div>

        <!--删除弹窗-->
        <div class="modluDrawer">
            <el-dialog title="删除简历数据" :visible.sync="dialogDel" :with-header="true" append-to-body :show-close="true"
                width="300px">
                <div>
                    <el-checkbox v-model="ruleFormDel.delAccount" true-label="1" false-label="0">同步删除账号</el-checkbox>
                </div>
                <div>
                    <i class="el-icon-warning"></i> 勾选删除账号所有数据信息
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogDel = false">取 消</el-button>
                    <el-button type="primary" @click="delSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>

        <div class="modluDrawer">
            <!--预览简历-->
            <el-drawer title="预览简历" :visible.sync="drawerPreview" append-to-body size="60%">
                <preview :id="detail.id"></preview>
            </el-drawer>
            <!--新增简历-->
            <el-drawer title="新增简历" :visible.sync="drawerAdd" append-to-body :wrapper-closable="false" size="45%">
                <add @child-event="closeAdd"></add>
            </el-drawer>
        </div>
    </div>
</template>

<script>
module.exports = {
    props: {
        status: {type: String, default: ''}
    },
    data: function () {
        return {


            mouseFlag: false,
            mouseOffset: 0,



            pic_accept: localStorage.getItem("pic_accept"),
            loading: false,
			dataText: '数据加载中',
            value: true,
            seachbutn: true,
            tableHig: true,

            // 来源
            source: {},

            // 搜索筛选项
            searchList: [],
            searchForm: {
				keytype: 1,
                status: this.status,
                time_type: 'adtime',
                times: [],
			},
            timeOptions: {
                shortcuts: [{
                    text: '昨天',
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        start.setTime(start.getTime() - 3600 * 1000 * 24);
                        end.setTime(end.getTime() - 3600 * 1000 * 24);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '今天',
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '本周',
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() - 1) * 24 * 60 * 60 * 1000);
                        const end = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '上周',
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() + 6) * 24 * 60 * 60 * 1000);
                        const end = new Date(new Date().setHours(0, 0, 0) + (0 - new Date().getDay()) *24 * 60 * 60 *1000);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '本月',
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 1).setHours(0, 0, 0));
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '上月',
                    onClick(picker) {
                        const end = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 0).setHours(23, 59, 59, 59));
                        const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth() - 1, 1).setHours(0, 0, 0));
                        picker.$emit('pick', [start, end]);
                    }
                }]
            },
            isSearchTime: false,
            // 列表
            page: 1,
            limit: 0,
            list: [],
            total: 0,
            pageSizes: [],

            // 列表排序
            t: '',
            order: '',

            checkedAll: false, // 全选
            checkedAllIndeterminate: false,
            multipleSelection: [], // 多选值存储
            idArr: [],

            detail: {},
            index: '',

            resumeAllNum: 0,
            resumeStatusNum1: 0,
            resumeStatusNum2: 0,
            resumeStatusNum3: 0,
            resumeTeenNum: 0,

            saveLoading: false,

            // 简历状态
            dialogStatus: false,
            ruleFormStatus: {},

            // 置顶
            dialogTop: false,
            ruleFormTop: {},

            // 导出
            dialogExport: false,
            isIndeterminateExport: false,
            checkAllExport: false,
            typeExport: {}, // 导出字段
            ruleFormExport: {
                type: [],
                limit: '',
                section: ''
            },

            // 备注
            dialogRemark: false,
            ruleFormRemark: {},

            // 审核
            dialogAudit: false, // 批量审核
            drawerAudit: false,
            ruleFormAudit: {},
            auditTpl: '',
            todoAuditNum: 0,
            resume: {},
            expectData: {},

            // 缓存
            user_sex: {},
            userclass_name: {},
            userdata: {},
            industry_index: [],
            industry_name: {},

            // 预览简历
            drawerPreview: false,

            // 添加
            drawerAdd: false,

            // 删除
            dialogDel: false,
            ruleFormDel: {},

            expectLoading: false,

            // 编辑基本资料
            drawerBasic: false,
            ruleFormBasic: {},
            // 个人优势
            dialogTag: false,
            ruleFormTag: {},
            userTag: [],
            inputTag: false,
            tagval: '',
            // 求职意向
            drawerJob: false,
            ruleFormJob: {},
            jobSelected: null,
            citySelected: null,
            minsalaryList: [],
            maxsalaryList: [],

            todayCheck: false, // 至今选中

            // 工作经历
            dialogWork: false,
            indexWork: -1,
            ruleFormWork: {},
            // 教育经历
            dialogEdu: false,
            indexEdu: -1,
            daterangeEdu: [],
            ruleFormEdu: {},
            // 培训经历
            dialogTraining: false,
            indexTraining: -1,
            daterangeTraining: [],
            ruleFormTraining: {},
            // 技能提升
            dialogSkill: false,
            indexSkill: -1,
            ruleFormSkill: {},
            // 项目经历
            dialogProject: false,
            indexProject: -1,
            daterangeProject: [],
            ruleFormProject: {},
            // 其他描述
            dialogOther: false,
            indexOther: -1,
            ruleFormOther: {},

            // 投递记录
            drawerJobSqLog: false,
            jobSqLog: {},

            prevPage: 0,
            prevPage2: 0
        }
    },
    components: {
        'add': httpVueLoader('./resume_add.vue'),
        'job_class': httpVueLoader('../../../component/job_class.vue'),
        'city_class': httpVueLoader('../../../component/city_class.vue'),
        'preview': httpVueLoader('../../../component/resume_preview.vue')
    },
    mounted() {
        var that = this
        setTimeout(function () {
            that.getConfigFun();
        }, 200)
    },
    created() {
        var that = this;
        let params = window.parent.homeapp.$route.params;
        let query = window.parent.homeapp.$route.query;
        
        if (!$.isEmptyObject(query.params)) {
            params = {...params,...query.params};
        }
        
        if (!$.isEmptyObject(params)) {
            delete params.activeName;
            this.getParams(params);
        }
        this.init();
    },
    methods: {


        mouseDownHandler(e) {
            this.mouseOffset = e.clientX;
            this.mouseFlag = true;
        },
        mouseUpHandler(e) {
            this.mouseFlag = false;
        },
        mouseMoveHandler(e) {
            // 这里面需要注意，通过ref需要那个那个包含table元素的父元素
            let divData = this.$refs.multipleTable.bodyWrapper;
            if (this.mouseFlag) {
                // 设置水平方向的元素的位置
                divData.scrollLeft -= (- this.mouseOffset + (this.mouseOffset = e.clientX));
            }
        },




		getParams:function(params={},search=false){
			var that = this;
			for(let i in params){
				if(typeof that.searchForm[i]!='undefined'){
                    that.searchForm[i] = params[i];
                }
			}
			if(search){
				this.search();
			}
		},
        init() {
            // this.resetSearch();
            this.getCountData();
            this.search();
        },

        resetSearch() {
            this.searchForm = {
                keytype: 1
            };
            this.limit = 0;
        },

        statusSearch(status) {
            this.resetSearch();

            if (status == 1) {
                this.searchForm.teen = status;
            } else {
                this.searchForm.status = status;
            }

            this.search();
        },

        // 搜索职位选择
        confirmJobSearch(data) {
            this.searchForm.job_class = data.jobId.join(',');
        },
        // 搜索城市选择
        confirmCitySearch(data) {
            this.searchForm.city_class = data.cityId.join(',');
        },

        getCountData() {
            let that = this;

            httpPost('m=user&c=users_resume&a=resumeNum', {}, {hideloading: true}).then(function (response) {
                let res = response.data;

                that.resumeAllNum = res.resumeAllNum;
                that.resumeStatusNum1 = res.resumeStatusNum1;
                that.resumeStatusNum2 = res.resumeStatusNum2;
                that.resumeStatusNum3 = res.resumeStatusNum3;
                that.resumeTeenNum = res.resumeTeenNum;
            })
        },

        handleSizeChange(val) {
            this.limit = val;
            scrollToTop()
            this.getList();
        },
        handleCurrentChange(val) {
            this.page = val;
            this.getList();
        },
        sortChange(event) {
            this.t = event.order ? event.prop : '';
            this.order = event.order ? event.order == 'descending' ? 'desc' : 'asc' : '';
            this.search();
        },
        search() {
            this.page = 1;
            this.getList();
        },

        getConfigFun:function(){
            let that = this;
            httpPost('m=user&c=users_resume&a=getConfig', {},{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.typeExport = res.data.exportType;
                    that.source = res.data.source;
                    that.searchList = res.data.search_list;
                }
            })
        },
        getList() {
            let that = this,
                searchForm = that.searchForm,
                params = {
                    page: that.page,
                    limit: that.limit,
                    t: that.t,
                    order: that.order,
                };
            that.loading = true;
            httpPost('m=user&c=users_resume', { ...params, ...searchForm }, {hideloading: true}).then(function (response) {
                let res = response.data,
                    data = res.data;

                that.list = data.list;
                that.total = parseInt(data.total);
                that.pageSizes = data.page_sizes;
                if (that.limit === 0) {
                    that.limit = parseInt(data.limit); // 取系统配置默认数量
                }
                if (that.page > data.page) {
                    that.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
                }
                that.loading = false;
                if(that.prevPage != that.page){
                    that.prevPage = that.page;
                    that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    scrollToTop()
                }
                if (that.list.length === 0) {
                    that.dataText = "暂无数据";
                }
            })
        },

        // 批量操作
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
            let that = this;
            if (this.multipleSelection.length == 0 && type == 'del') {
                message.error('请选择要删除的数据');
                return false;
            }else if(this.multipleSelection.length == 0){
                message.error('请选择要操作的数据项');
                return false;
            }

            let idArr = [];
            this.multipleSelection.forEach(function (item) {
                idArr.push(item.id);
            })
            this.idArr = idArr;

            if (type == 'del') {
                this.openDel();
            } else if (type == 'audit') {
                this.openBatchAudit();
            } else if (type == 'refresh') {
                this.refresh();
            } else if (type == 'rec') {
                delConfirm(this, null, function (params) {
                    that.changeRec(1);
                }, '确定批量推荐？')
            } else if (type == 'rec_cancel') {
                delConfirm(this, null, function (params) {
                    that.changeRec(0);
                }, '确定取消推荐？')
            } else if (type == 'top') {
                this.openTop('');
            } else if (type == 'top_cancel') {
                this.openTop('', '1');
            } else if (type == 'export'){
                this.dialogExport = true;
            }
        },
        checkAll(val) {
            val ? this.checkedAllIndeterminate = false : '';
            this.$refs.multipleTable.toggleAllSelection();
        },

        // 删除
        openDel(idx) {
            if (typeof idx == 'undefined') { // 批量删除
                this.ruleFormDel = {
                    del: this.idArr,
                    delAccount: 0
                }
            } else {// 单个删除
                this.ruleFormDel = {
                    del: this.list[idx].id,
                    delAccount: 0
                }
            }

            this.dialogDel = true;
        },
        delSubmit() {
            let that = this,
                ruleForm = this.ruleFormDel;

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=delResume', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    that.saveLoading = false;
                    message.error(res.msg);
                } else {
                    that.dialogDel = false;
                    that.getList();
                    that.$refs.multipleTable.clearSelection();
                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    })
                }
            })
        },

        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g, '');
        },
        inputFloatNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9.]/g, '');
        },
        inputIdcard(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9Xx.]/g, '');
        },

        // 简历状态
        openStatus(row) {
            this.detail = row;
            this.ruleFormStatus = {
                uid: row.uid,
                status: row.status
            };
            this.dialogStatus = true;
            if (typeof this.userdata.user_label === 'undefined') {
                this.getCache();
            }
        },
        submitStatus() {
            let that = this,
                params = that.ruleFormStatus;

            if (!params.status || params.status === '0') {
                message.warning('请选择简历状态');
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=cstatus', params).then(function (res) {
                if (res.data.error > 0) {
                    message.error(res.data.msg);
                } else {
                    that.dialogStatus = false;
                    that.getList();
                    message.success(res.data.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },
        // 推荐
        changeRec(val, row) {
            let that = this,
                id = '';

            if (typeof row === 'undefined') {
                id = this.idArr;
            } else {
                id = row.id;
            }

            httpPost('m=user&c=users_resume&a=rec', { id: id, rec: val }).then(function (res) {
                if (res.data.error > 0) {
                    message.error(res.data.msg);
                } else if (typeof row === 'undefined') { // 批量推荐
                    message.success(res.data.msg);
                    that.getList();
                }
            })
        },
        // 置顶
        openTop(row, s = '0') {
            this.detail = row;
            if (row == '') {
                this.ruleFormTop = {
                    id: this.idArr,
                    addday: '',
                    s: s
                };
            } else {
                this.ruleFormTop = {
                    id: row.id,
                    addday: '',
                    s: s
                };
            }
            this.dialogTop = true;
        },
        submitTop() {
            let that = this,
                params = that.ruleFormTop;

            if (params.s === '1') { // 取消置顶
            } else { // 置顶
                if (!params.addday) {
                    message.warning('请输入置顶天数');
                    return false;
                }
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=top', params).then(function (res) {
                if (res.data.error > 0) {
                    message.error(res.data.msg);
                } else {
                    that.dialogTop = false;
                    that.getList();
                    message.success(res.data.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },
        handleCheckAllExport(val) {
            this.ruleFormExport.type = val ? Object.keys(this.typeExport) : [];
            this.isIndeterminateExport = false;
        },
        handleCheckedExportType(value) {
            let typeArr = Object.keys(this.typeExport),
                checkedCount = value.length;
            this.checkAllExport = checkedCount === typeArr.length;
            this.isIndeterminateExport = checkedCount > 0 && checkedCount < typeArr.length;
        },
        submitExport() {
            let that = this,
                params = that.ruleFormExport;

            if (params.type.length == 0) {
                message.warning('请选择导出字段');
                return;
            }

            let idArr = [];
            this.multipleSelection.forEach(function (item) {
                idArr.push(item.id);
            })
            if (idArr.length > 0) {
                params.ids = idArr;
            }
			that.saveLoading = true;
            httpPost('m=user&c=users_resume&a=export_check', params).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    httpPost('m=user&c=users_resume&a=xls', { type: params.type }).then(function (response2) {
                        let res2 = response2.data;

                        if (res2.error > 0) {
                            message.error(res2.msg);
                        } else {
                            that.dialogExport = false;
                            utilFile.downloadFileByByte(res2.data.file, res2.data.file_name);
                        }
                    })
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },
        // 备注
        openRemark(row) {
            this.detail = row;
            this.ruleFormRemark = {
                id: row.id,
                uid: row.uid,
                label: row.label > 0 ? row.label : '',
                content: row.content,
            };
            this.dialogRemark = true;
            if (typeof this.userdata.user_label === 'undefined') {
                this.getCache();
            }
        },
        submitRemark() {
            let that = this,
                params = that.ruleFormRemark;

            if (!params.label || params.label === '0') {
                message.warning('请选择简历标签');
                return false;
            }
            if (!params.content) {
                message.warning('请输入客服评价');
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=label', params).then(function (res) {
                if (res.data.error > 0) {
                    message.error(res.data.msg);
                } else {
                    that.dialogRemark = false;
                    that.getList();
                    message.success(res.data.msg)
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },
        // 批量审核
        openBatchAudit() {
            this.ruleFormAudit = {
                id: this.idArr,
                status: '1',
                statusbody: ''
            };
            this.dialogAudit = true;
        },
        submitBatchAudit() {
            let that = this,
                params = that.ruleFormAudit;

            if (typeof params.status == 'undefined' || params.status === '' || params.status === '0') {
                message.warning('请选择审核状态');
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=status', params).then(function (res) {
                if (res.data.error > 0) {
                    message.error(res.data.msg);
                } else {
                    that.dialogAudit = false;
                    that.getList();
                    message.success(res.data.msg, function () {
                        that.$refs.multipleTable.clearSelection();
                    });
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },
        // 打开审核
        openAudit(row) {
            this.getAudit(row.id);
            this.drawerAudit = true;
        },
        setFormAudit() {
            let detail = this.detail;
            this.ruleFormAudit = {
                single: 1, // 单个审核
                id: detail.id,
                uid: detail.uid,
                r_status: detail.r_status,
                status: detail.state=='3'?'3':'1',
                statusbody: detail.statusbody,
                content: detail.content
            };
            this.auditTpl = '';
        },
        // 关闭审核
        closedAudit() {
            if (this.refreshList) {
                this.getList();
            }
        },
        // 获取详情
        async getAudit(id) {
            this.expectLoading = true;
            let response = await httpPost('m=user&c=users_resume&a=resumeAudit', { id: id });
            let res = response.data,
                data = res.data;

            this.todoAuditNum = data.snum;

            this.detail = data.info;
            this.resume = data.resume ? data.resume : {};
            this.expectData = data.expectData;

            this.user_sex = data.user_sex;
            this.userclass_name = data.userclass_name;
            this.userdata = data.userdata;
            this.industry_index = data.industry_index;
            this.industry_name = data.industry_name;
            this.expectLoading = false;

            this.setFormAudit();
        },
        // 切换审核模板
        changeTpl(val) {
            this.ruleFormAudit.statusbody = this.userclass_name[val];
        },
        // 提交审核
        submitAudit(atype) {
            let that = this,
                detail = that.detail,
                params = that.ruleFormAudit,
                url = 'm=user&c=users_resume&a=status';

            if (typeof params.status == 'undefined' || params.status === '' || params.status === '0') {
                message.warning('请选择审核状态');
                return false;
            }

            if (typeof params.r_status !== 'undefined') {
                if (params.r_status == 1) {
                    params.lock_status = params.r_status;
                } else {
                    message.warning('锁定操作异常');
                    return false;
                }
            }

            if (detail.r_status != 1) {
                url = 'm=user&c=users_resume&a=resumestatus';
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            params.atype = atype;

            httpPost(url, params).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    that.saveLoading = false;
                    message.error(res.msg);
                } else {
                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                    that.refreshList = true;
                    if (typeof res.data !== 'undefined' && typeof res.data.next_id !== 'undefined') { // 审核下一个
                        that.getAudit(res.data.next_id);
                    } else {
                        that.drawerAudit = false;
                        that.$refs.multipleTable.clearSelection();
                    }
                }
            })
        },

        getCache() {
            let that = this;

            httpPost('m=user&c=users_resume&a=getCache', {}, { hideloading: true }).then(function (response) {
                let res = response.data,
                    data = res.data;

                that.userdata = data.userdata;
                that.userclass_name = data.userclass_name;
            })
        },

        // 预览简历
        openPreview(row) {
            this.detail = row;
            this.drawerPreview = true;
        },

        // 新增简历
        openAdd() {
            let that =this;
            httpPost('m=user&c=users_resume&a=add', {add:1}).then(function (response) {
                let res = response.data;
                that.drawerAdd = true;
            })
        },
        closeAdd() {
            this.drawerAdd = false;
            this.getList();
        },

        // 刷新简历
        refresh(row) {
            let that = this,
                params = {};

            if (typeof row === 'undefined') { // 批量刷新
                params.ids = this.idArr;
            } else { // 单个刷新
                params.id = row.id;
            }

            delConfirm(this, params, function (params) {
                httpPost('m=user&c=users_resume&a=refresh', params).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        message.error(res.msg);
                    } else {
                        that.getList();
                        message.success(res.msg);
                    }
                })
            }, '确认刷新简历？')
        },

        toMember(row) {
            let that = this;
            that.getMemberUrl(row.uid);
        },

        async getMemberUrl(uid) {
            let response = await httpPost('m=user&c=users_member&a=Imitate', { uid: uid });

            let res = response.data;
            if (res.error === 0) {
                window.open(res.data.url);
            } else {
                message.error(res.msg);
            }
        },

        openPage(url) {
            window.open(url);
        },

        // 编辑资料
        openBasic() {
            let resume = this.resume;
            this.ruleFormBasic = {
                uid: resume.uid,
                name: resume.name,
                sex: resume.sex,
                birthday: resume.birthday ? new Date(resume.birthday) : '',
                edu: resume.edu && resume.edu > 0 ? resume.edu : '',
                exp: resume.exp && resume.exp > 0 ? resume.exp : '',
                telphone: resume.telphone,
                email: resume.email,
                idcard: resume.idcard,
                domicile: resume.domicile,
                living: resume.living,
                address: resume.address,
                height: resume.height,
                weight: resume.weight,
                marriage: resume.marriage && resume.marriage > 0 ? resume.marriage : '',
                nationality: resume.nationality,
                homepage: resume.homepage,
                qq: resume.qq,
                description: resume.description,
                wxewm_n: resume.wxewm_n
            };
            this.drawerBasic = true;
        },
        // 上传时触发
        handleChangeWxewm(file, fileList) {
            this.$set(this.ruleFormBasic, 'file', file.raw);
            this.$set(this.ruleFormBasic, 'wxewm_n', file.url);
        },
        submitBasic() {
            let that = this,
                ruleForm = that.ruleFormBasic,
                formData = new FormData();

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            $.each(ruleForm, function (key, value) {
                if (key != 'wxewm_n') {
                    if (key == 'birthday' && value !== '' ) {
                        value = formatMonth(value);
                    }
                    if(value !== '' && value != null){
                        formData.append(key, value);
                    }
                }
            });

            httpPost('m=user&c=users_member&a=editSave', formData).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.drawerBasic = false;
                    that.refreshList = true;
                    // 重新拉取详情
                    that.getAudit(that.detail.id);
                    message.success(res.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },
        // 个人优势
        openTag() {
            let resume = deepClone(this.resume),
                // expect = this.expectData.expect,
                user_tag = this.userdata.user_tag,
                userclass_name = this.userclass_name,
                userTag = [];

            if (user_tag.length > 0) {
                user_tag.forEach(function (item) {
                    userTag.push(userclass_name[item]);
                })
            }
            if (resume.arrayTag && resume.arrayTag.length > 0) {
                resume.arrayTag.forEach(function (item) {
                    if (userTag.indexOf(item) < 0) {
                        userTag.push(item); // 不在已有标签里的,追加标签
                    }
                })
            }

            this.userTag = userTag;
            this.ruleFormTag = {
                uid: resume.uid,
                // eid: expect ? expect.id : '',
                tag: resume.arrayTag ? resume.arrayTag : [],
                description: resume.description
            };
            this.dialogTag = true;
        },
        showTag() {
            this.tagval = '';
            this.inputTag = true;
        },
        confirmTag() {
            let tag = this.ruleFormTag.tag
            userTag = this.userTag,
                tagval = this.tagval,
                len = tagval.length;

            if (len > 0) {
                if (len < 2 || len > 8) {
                    message.warning('请输入2-8个标签字符');
                    return false;
                }
                if (tag.length >= 5) {
                    message.warning('标签最多选择5个');
                    return false;
                }
                if (userTag.indexOf(tagval) > -1) {
                    message.warning('标签已存在');
                    return false;
                }
                tag.push(tagval);
                userTag.push(tagval);
                this.ruleFormTag.tag = tag;
                this.userTag = userTag;
            }
            this.inputTag = false;
        },
        checkTag(val) {
            let tag = this.ruleFormTag.tag,
                index = tag.indexOf(val);

            if (index > -1) { // 二次点击取消选中
                tag.splice(index, 1);
            } else { // 首次点击选中
                if (tag.length >= 5) {
                    message.warning('标签最多选择5个');
                    return false;
                }
                tag.push(val);
            }

            this.ruleFormTag.tag = tag;
        },
        submitTag() {
            let that = this,
                ruleForm = that.ruleFormTag;

            if (ruleForm.eid == '') {
                message.warning('请先完善求职意向');
                return false;
            }
            if (ruleForm.tag.length > 5) {
                message.warning('标签最多选择5个');
                return false;
            }
            if (ruleForm.description == '' || ruleForm.description == null) {
                message.warning('请输入自我评价');
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=saveTag', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogTag = false;
                    that.refreshList = true;
                    that.resume.arrayTag = ruleForm.tag;
                    that.resume.description = ruleForm.description;
                    message.success(res.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },
        // 求职意向
        openJob() {
            let resume = this.resume,
                expect = this.expectData.expect;

            this.jobSelected = expect.jobnameArr;
            this.citySelected = expect.citynameArr;

            let salaryList = deepClone(this.expectData.salary),
                maxsalaryList = [];
            salaryList.forEach(function(item, index) {
                if (index > 0) {
                    if (expect.maxsalary > 0) {
                        if (parseInt(expect.minsalary) < parseInt(item)) {
                            maxsalaryList.push(item)
                        }
                    } else {
                        maxsalaryList.push(item)
                    }
                }
            })
            salaryList.splice(salaryList.length-1, 1);
            this.minsalaryList = salaryList;
            this.maxsalaryList = maxsalaryList;

            this.ruleFormJob = {
                uid: resume.uid,
                eid: expect.id,
                job_classid: expect.job_classid, // TODO 选择职位
                city_classid: expect.city_classid, // TODO 选择城市
                name: expect.name,
                minsalary: expect.minsalary && expect.minsalary > 0 ? parseInt(expect.minsalary) : '',
                maxsalary: expect.maxsalary && expect.maxsalary > 0 ? parseInt(expect.maxsalary) : '',
                hy: expect.hy && expect.hy > 0 ? expect.hy : '',
                report: expect.report && expect.report > 0 ? expect.report : '',
                type: expect.type && expect.type > 0 ? expect.type : '',
                jobstatus: expect.jobstatus && expect.jobstatus > 0 ? expect.jobstatus : '',
            };
            this.drawerJob = true;
        },
        salaryChange(val) {
            let that = this,
                maxsalaryList = [],
                i = 0;
            this.expectData.salary.forEach(function(item, index) {
                if (parseInt(val) < parseInt(item)) {
                    maxsalaryList.push(item)
                    if (i === 0) {
                        that.ruleFormJob.maxsalary = item;
                    }
                    i++;
                }
            })
            this.maxsalaryList = maxsalaryList;
        },
        confirmJob(data) {
            this.ruleFormJob.job_classid = data.jobId.join(',');
        },
        confirmCity(data) {
            this.ruleFormJob.city_classid = data.cityId.join(',');
        },
        submitJob() {
            let that = this,
                ruleForm = that.ruleFormJob;

            if (ruleForm.name == "") {
                message.warning('请输入期望职位');
                return false;
            }
            if (ruleForm.job_classid == "") {
                message.warning('请选择从事职位');
                return false;
            }
            if (ruleForm.city_classid == '') {
                message.warning('请选择期望地点');
                return false;
            }
            if (ruleForm.minsalary == "" || ruleForm.minsalary == "0") {
                message.warning('请输入期望薪资');
                return false;
            }
            if (ruleForm.maxsalary && parseInt(ruleForm.maxsalary) <= parseInt(ruleForm.minsalary)) {
                message.warning('最高薪资必须大于最低薪资');
                return false;
            }
            if (ruleForm.report == "") {
                message.warning('请选择到岗时间');
                return false;
            }
            if (ruleForm.type == "") {
                message.warning('请选择工作性质');
                return false;
            }
            if (ruleForm.jobstatus == "") {
                message.warning('请选择求职状态');
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=saveExpect', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.drawerJob = false;
                    that.refreshList = true;
                    // 重新拉取详情
                    that.getAudit(ruleForm.eid);
                    message.success(res.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },

        // 至今选择
        todayChange(val, type) {
            if (type == 'work') {
                this.$set(this.ruleFormWork, 'edate', '');
            }
        },

        // 工作经历
        openWork(index) {
            let expectData = this.expectData,
                expect = expectData.expect,
                workList = expectData.work;

            if (index !== '') {
                let work = deepClone(workList[index])
                this.ruleFormWork = {
                    uid: expectData.uid,
                    eid: expect.id,
                    id: work.id,
                    name: work.name,
                    title: work.title,
                    sdate: work.sdate > 0 ? new Date(work.sdate_n) : '',
                    edate: work.edate > 0 ? new Date(work.edate_n) : '',
                    content: work.content,
                };

                if (work.edate == '0') {
                    this.todayCheck = true;
                }
                this.indexWork = index;
            } else {
                this.ruleFormWork = {
                    uid: expectData.uid,
                    eid: expect.id,
                    id: '',
                    name: '',
                    title: '',
                    sdate: '',
                    edate: '',
                    content: '',
                };
                this.todayCheck = false;
                this.indexWork = -1
            }

            this.dialogWork = true;
        },
        submitWork() {
            let that = this,
                indexWork = that.indexWork,
                ruleForm = that.ruleFormWork;

            if (ruleForm.eid == "") {
                message.warning('请先完善求职意向');
                return false;
            }
            if (ruleForm.name == "") {
                message.warning('请输入公司名称');
                return false;
            }
            if (ruleForm.sdate == "") {
                message.warning('请选择工作时间');
                return false
            }
            ruleForm.sdate = formatMonth(ruleForm.sdate);
            if (ruleForm.edate != '') {
                if (ruleForm.sdate >= ruleForm.edate) {
                    message.warning('结束日期必须大于开始日期');
                    return false
                }
                ruleForm.edate = formatMonth(ruleForm.edate);
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=work', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogWork = false;
                    that.refreshList = true;

                    // 拼接工作经历数据 - 减少请求服务器
                    if (ruleForm.id == '') {
                        let work = deepClone(ruleForm);
                        work.id = res.data.id;
                        work.sdate = 1;
                        work.sdate_n = ruleForm.sdate;
                        work.edate = ruleForm.edate != '' ? 2 : 0;
                        work.edate_n = ruleForm.edate != '' ? ruleForm.edate : '至今';
                        that.expectData.work.unshift(work);
                    } else {
                        let work = that.expectData.work[indexWork];
                        work.name = ruleForm.name;
                        work.title = ruleForm.title;
                        work.sdate = 1;
                        work.sdate_n = ruleForm.sdate;
                        work.edate = ruleForm.edate != '' ? 2 : 0;
                        work.edate_n = ruleForm.edate != '' ? ruleForm.edate : '至今';
                        work.content = ruleForm.content;
                        that.expectData.work[indexWork] = deepClone(work);
                    }

                    message.success(res.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },

        // 工作经历
        openEdu(index) {
            let expectData = this.expectData,
                expect = expectData.expect,
                eduList = expectData.edu;

            if (index !== '') {
                let edu = deepClone(eduList[index])
                this.ruleFormEdu = {
                    uid: expectData.uid,
                    eid: expect.id,
                    id: edu.id,
                    name: edu.name,
                    education: edu.education > 0 ? edu.education : '',
                    specialty: edu.specialty,
                    title: '', // 此字段没实际意义，暂时占位
                };
                this.daterangeEdu = [
                    new Date(edu.sdate_n),
                    new Date(edu.edate_n)
                ];
                this.indexEdu = index;
            } else {
                this.ruleFormEdu = {
                    uid: expectData.uid,
                    eid: expect.id,
                    id: '',
                    name: '',
                    sdate: '',
                    edate: '',
                    education: '',
                    specialty: '',
                    title: '', // 此字段没实际意义，暂时占位
                };
                this.daterangeEdu = [];
                this.indexEdu = -1
            }

            this.dialogEdu = true;
        },
        submitEdu() {
            let that = this,
                indexEdu = that.indexEdu,
                daterangeEdu = that.daterangeEdu,
                ruleForm = that.ruleFormEdu;

            if (ruleForm.eid == "") {
                message.warning('请先完善求职意向');
                return false;
            }
            if (ruleForm.name == "") {
                message.warning('请输入学校名称');
                return false;
            }
            if (daterangeEdu.length == 0) {
                message.warning('请选择在校时间');
                return false
            }
            if (ruleForm.education == "") {
                message.warning('请选择最高学历');
                return false
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            ruleForm.sdate = formatMonth(daterangeEdu[0]);
            ruleForm.edate = formatMonth(daterangeEdu[1]);

            httpPost('m=user&c=users_resume&a=edu', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogEdu = false;
                    that.refreshList = true;

                    // 拼接工作经历数据 - 减少请求服务器
                    if (ruleForm.id == '') {
                        let edu = deepClone(ruleForm);
                        edu.id = res.data.id;
                        edu.sdate_n = ruleForm.sdate;
                        edu.edate_n = ruleForm.edate;
                        edu.education_n = that.userclass_name[ruleForm.education];
                        that.expectData.edu.unshift(edu);
                    } else {
                        let edu = that.expectData.edu[indexEdu];
                        edu.name = ruleForm.name;
                        edu.title = ruleForm.title;
                        edu.sdate_n = ruleForm.sdate;
                        edu.edate_n = ruleForm.edate;
                        edu.education = ruleForm.education;
                        edu.education_n = that.userclass_name[ruleForm.education];
                        edu.specialty = ruleForm.specialty;
                        that.expectData.edu[indexEdu] = deepClone(edu);
                    }

                    message.success(res.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },

        // 培训经历
        openTraining(index) {
            let expectData = this.expectData,
                expect = expectData.expect,
                trainingList = expectData.training;

            if (index !== '') {
                let training = deepClone(trainingList[index])
                this.ruleFormTraining = {
                    uid: expectData.uid,
                    eid: expect.id,
                    id: training.id,
                    name: training.name,
                    title: training.title,
                    content: training.content,
                };
                this.daterangeTraining = [
                    new Date(training.sdate_n),
                    new Date(training.edate_n)
                ];
                this.indexTraining = index;
            } else {
                this.ruleFormTraining = {
                    uid: expectData.uid,
                    eid: expect.id,
                    id: '',
                    name: '',
                    title: '',
                    sdate: '',
                    edate: '',
                    content: '',
                };
                this.daterangeTraining = [];
                this.indexTraining = -1
            }

            this.dialogTraining = true;
        },
        submitTraining() {
            let that = this,
                indexTraining = that.indexTraining,
                daterangeTraining = that.daterangeTraining,
                ruleForm = that.ruleFormTraining;

            if (ruleForm.eid == "") {
                message.warning('请先完善求职意向');
                return false;
            }
            if (ruleForm.name == "") {
                message.warning('请输入培训中心');
                return false;
            }
            if (daterangeTraining.length == 0) {
                message.warning('请选择培训时间');
                return false
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            ruleForm.sdate = formatMonth(daterangeTraining[0]);
            ruleForm.edate = formatMonth(daterangeTraining[1]);

            httpPost('m=user&c=users_resume&a=training', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogTraining = false;
                    that.refreshList = true;

                    // 拼接工作经历数据 - 减少请求服务器
                    if (ruleForm.id == '') {
                        let training = deepClone(ruleForm);
                        training.id = res.data.id;
                        training.sdate_n = ruleForm.sdate;
                        training.edate_n = ruleForm.edate;
                        that.expectData.training.unshift(training);
                    } else {
                        let training = that.expectData.training[indexTraining];
                        training.name = ruleForm.name;
                        training.title = ruleForm.title;
                        training.sdate_n = ruleForm.sdate;
                        training.edate_n = ruleForm.edate;
                        training.content = ruleForm.content;
                        that.expectData.training[indexTraining] = deepClone(training);
                    }

                    message.success(res.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },

        // 职业技能
        openSkill(index) {
            let expectData = this.expectData,
                expect = expectData.expect,
                skillList = expectData.skill;

            if (index !== '') {
                let skill = deepClone(skillList[index])
                this.ruleFormSkill = {
                    uid: expectData.uid,
                    eid: expect.id,
                    id: skill.id,
                    name: skill.name,
                    longtime: skill.longtime,
                    ing: skill.ing,
                    pic_n: skill.pic,
                };
                this.indexSkill = index;
            } else {
                this.ruleFormSkill = {
                    uid: expectData.uid,
                    eid: expect.id,
                    id: '',
                    name: '',
                    longtime: '',
                    ing: '',
                    pic_n: '',
                };
                this.indexSkill = -1
            }

            this.dialogSkill = true;
        },
        // 上传时触发
        handleChangeSkillPic(file, fileList) {
            this.$set(this.ruleFormSkill, 'file', file.raw);
            this.$set(this.ruleFormSkill, 'pic_n', file.url);
        },
        submitSkill() {
            let that = this,
                indexSkill = that.indexSkill,
                ruleForm = that.ruleFormSkill,
                formData = new FormData();

            if (ruleForm.eid == "") {
                message.warning('请先完善求职意向');
                return false;
            }
            if (ruleForm.name == "") {
                message.warning('请输入技能名称');
                return false;
            }
            if (ruleForm.ing == "") {
                message.warning('请选择熟练程度');
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            $.each(ruleForm, function (key, value) {
                if (key != 'pic_n') {
                    formData.append(key, value);
                }
            });

            httpPost('m=user&c=users_resume&a=skill', formData).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogSkill = false;
                    that.refreshList = true;

                    // 拼接工作经历数据 - 减少请求服务器
                    if (ruleForm.id == '') {
                        let skill = deepClone(ruleForm);
                        skill.id = res.data.id;
                        skill.ing_n = that.userclass_name[ruleForm.ing];
                        skill.pic = ruleForm.pic_n;
                        that.expectData.skill.push(skill);
                    } else {
                        let skill = that.expectData.skill[indexSkill];
                        skill.name = ruleForm.name;
                        skill.longtime = ruleForm.longtime;
                        skill.ing_n = that.userclass_name[ruleForm.ing];
                        skill.pic = ruleForm.pic_n;
                        that.expectData.skill[indexSkill] = deepClone(skill);
                    }

                    message.success(res.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },

        // 项目经历
        openProject(index) {
            let expectData = this.expectData,
                expect = expectData.expect,
                projectList = expectData.project;

            if (index !== '') {
                let project = deepClone(projectList[index])
                this.ruleFormProject = {
                    uid: expectData.uid,
                    eid: expect.id,
                    id: project.id,
                    name: project.name,
                    title: project.title,
                    content: project.content,
                };
                this.daterangeProject = [
                    new Date(project.sdate_n),
                    new Date(project.edate_n)
                ];
                this.indexProject = index;
            } else {
                this.ruleFormProject = {
                    uid: expectData.uid,
                    eid: expect.id,
                    id: '',
                    name: '',
                    title: '',
                    sdate: '',
                    edate: '',
                    content: '',
                };
                this.daterangeProject = [];
                this.indexProject = -1
            }

            this.dialogProject = true;
        },
        submitProject() {
            let that = this,
                indexProject = that.indexProject,
                daterangeProject = that.daterangeProject,
                ruleForm = that.ruleFormProject;

            if (ruleForm.eid == "") {
                message.warning('请先完善求职意向');
                return false;
            }
            if (ruleForm.name == "") {
                message.warning('请输入项目名称');
                return false;
            }
            if (daterangeProject.length == 0) {
                message.warning('请选择项目时间');
                return false
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            ruleForm.sdate = formatMonth(daterangeProject[0]);
            ruleForm.edate = formatMonth(daterangeProject[1]);

            httpPost('m=user&c=users_resume&a=project', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogProject = false;
                    that.refreshList = true;

                    // 拼接工作经历数据 - 减少请求服务器
                    if (ruleForm.id == '') {
                        let project = deepClone(ruleForm);
                        project.id = res.data.id;
                        project.sdate_n = ruleForm.sdate;
                        project.edate_n = ruleForm.edate;
                        that.expectData.project.unshift(project);
                    } else {
                        let project = that.expectData.project[indexProject];
                        project.name = ruleForm.name;
                        project.title = ruleForm.title;
                        project.sdate_n = ruleForm.sdate;
                        project.edate_n = ruleForm.edate;
                        project.content = ruleForm.content;
                        that.expectData.project[indexProject] = deepClone(project);
                    }

                    message.success(res.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },

        // 其他描述
        openOther(index) {
            let expectData = this.expectData,
                expect = expectData.expect,
                otherList = expectData.other;

            if (index !== '') {
                let other = deepClone(otherList[index])
                this.ruleFormOther = {
                    uid: expectData.uid,
                    eid: expect.id,
                    id: other.id,
                    name: other.name,
                    content: other.content,
                };
                this.indexOther = index;
            } else {
                this.ruleFormOther = {
                    uid: expectData.uid,
                    eid: expect.id,
                    id: '',
                    name: '',
                    content: '',
                };
                this.indexOther = -1
            }

            this.dialogOther = true;
        },
        submitOther() {
            let that = this,
                indexOther = that.indexOther,
                ruleForm = that.ruleFormOther;

            if (ruleForm.eid == "") {
                message.warning('请先完善求职意向');
                return false;
            }
            if (ruleForm.name == "") {
                message.warning('请输入标题名称');
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=other', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogOther = false;
                    that.refreshList = true;

                    // 拼接工作经历数据 - 减少请求服务器
                    if (ruleForm.id == '') {
                        let other = deepClone(ruleForm);
                        other.id = res.data.id;
                        that.expectData.other.push(other);
                    } else {
                        let other = that.expectData.other[indexOther];
                        other.name = ruleForm.name;
                        other.content = ruleForm.content;
                        that.expectData.other[indexOther] = deepClone(other);
                    }

                    message.success(res.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },

        // 公用删除附表数据
        delResumeFb(type, index, id) {
            let that = this,
                expectData = that.expectData;

            delConfirm(this, {}, function () {
                httpPost('m=user&c=users_resume&a=delResumeFb', {
                    table: type,
                    id: id,
                    eid: expectData.expect.id,
                    uid: expectData.uid,
                }).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        message.error(res.msg);
                    } else {
                        that.refreshList = true;
                        that.expectData[type].splice(index, 1);
                        message.success(res.msg);
                    }
                })
            }, '确定要删除该项内容？');
        },

        // 投递岗位记录
        openJobSqlLog(row) {
            this.detail = row;
            this.$set(this.$data, 'jobSqLog', {
                page: 1,
                limit: 0,
                total: 0
            });
            this.getJobSqLog();
            this.drawerJobSqLog = true;
        },
        handleSizeChangeJobSqlLog(val) {
            this.jobSqLog.limit = val;
            this.getJobSqLog();
        },
        handleCurrentChangeJobSqlLog(val) {
            this.jobSqLog.page = val;
            this.getJobSqLog();
        },
        getJobSqLog() {
            let that = this,
                jobSqLog = deepClone(that.jobSqLog),
                params = {
                    page: jobSqLog.page,
                    limit: jobSqLog.limit,
                    eid: that.detail.id
                };
            that.loading = true;
            httpPost('m=user&c=users_member&a=jobSqLog', params).then(function (response) {
                let res = response.data,
                    data = res.data;

                jobSqLog.list = data.list;
                jobSqLog.total = parseInt(data.total);
                jobSqLog.pageSizes = data.page_sizes;
                if (jobSqLog.limit === 0) {
                    jobSqLog.limit = parseInt(data.limit); // 取系统配置默认数量
                }
                if (jobSqLog.page > data.page) {
                    jobSqLog.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
                }
                if(that.prevPage2 != jobSqLog.page){
                    that.prevPage2 = jobSqLog.page;
                    that.$refs.table2.bodyWrapper.scrollTop = 0;
                }
                that.jobSqLog = jobSqLog;
                that.loading = false;
                if (that.jobSqLog.list.length === 0) {
                    that.dataText = "暂无数据";
                }
            })
        },
        handleTimeChange() {
            if (this.searchForm.time_type != '' && Array.isArray(this.searchForm.times) && this.searchForm.times.length) {

                this.isSearchTime = true;
                this.search();
            }
            if (this.isSearchTime && this.searchForm.time_type == '' && this.searchForm.times == null){

                this.isSearchTime = false;
                this.search();
            }
        }
    },
};
</script>
<style>
    .tableSeachInptsmall .el-input{width:initial}
    .tableSeachInptsmall .el-select{margin-right:0!important}
    .el-input-group__prepend{background-color:#fff;padding:0 0 0 5px}
    .jlzt .el-button--text{font-size:12px}
    .jlwgk .el-button--text{color:#666;font-size:12px}
    .button-new-tag{margin-left:10px;height:32px;line-height:30px;padding-top:0;padding-bottom:0}
    .input-new-tag{width:90px;margin-left:10px;vertical-align:bottom}
    .avatar-uploader .el-upload{border:1px dashed #d9d9d9;border-radius:6px;cursor:pointer;position:relative;overflow:hidden}
    .avatar-uploader .el-upload:hover{border-color:#409eff}
    .avatar-uploader-icon{font-size:28px;color:#8c939d;width:100px;height:100px;line-height:100px;text-align:center}
    .avatar{width:100px;height:100px;display:block}
    .fenpeizhand .toolClasList{flex-wrap:wrap}
    .toolClasTipse{overflow:hidden;position:relative;padding-left:75px;width:calc(100% - 75px)}
    .toolClasTipse .el-alert{overflow:hidden;position:relative;padding:6px 0;background:0 0}
    .moduleElTabResuall{height:calc(100% - 188px)!important}
    .moduleElTabGetResuma{height:calc(100% - 134px)!important}
    @media (max-width:1480px){.moduleElTabResuall{height:calc(100% - 234px)!important}
        .moduleElTabGetResuma{height:calc(100% - 134px)!important}
    }

</style>
