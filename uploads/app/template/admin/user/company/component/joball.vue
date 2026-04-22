<template>
    <div class="moduleElHight">

        <div class="moduleSeachbig" v-if="!simple">
            <div class="tableSeachInpt tableSeachInptsmall tableSeacFromer" style="padding: 2px 0;">
                <el-input v-model="search_params.keyword" @keyup.enter.native="search" placeholder="请输入搜索内容" size="small"
                    clearable>
                    <el-select v-model="search_params.type" size="small" slot="prepend" placeholder="用户名">
                        <el-option label="职位/企业名称" value="1"></el-option>
                        <el-option label="职位ID" value="3"></el-option>
                        <el-option label="IP" value="4"></el-option>
                    </el-select>
                </el-input>
            </div>
            <!--收起部分-->
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="search_params.time_type" size="small" slot="prepend" placeholder="筛选日期" clearable @change="handleTimeChange">
                    <el-option label="发布时间" value="sdate"></el-option>
                    <el-option label="更新时间" value="lastup"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmalltwo" :class="{ 'searchbutnOnff': seachbutn }">
                <el-date-picker v-model="search_params.times" type="daterange" align="right" unlink-panels range-separator="至" start-placeholder="开始日期" end-placeholder="结束日期" :picker-options="timeOptions" value-format="yyyy-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
            </div>


            <div class="tableSeachInpt tableSeachInptsmall" v-for="(searchitem, searchidx) in searchlist" :key="searchidx"
                :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="search_params[searchidx]" size="small" slot="prepend" :placeholder="searchitem.name"
                    clearable @change="search">
                    <el-option v-for="(item, index) in searchitem.value" :label="item" :key="index"
                        :value="index"></el-option>
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
            <div class="tableSeachzk" :class="{ 'searchbutnKai': seachbutn }" style="margin-bottom: 8px;">
                <el-button type="info" class="zhankai" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>展开<i class="el-icon-arrow-down el-icon--right"></i>
                </el-button>
                <el-button type="info" class="shouqi" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>合并<i class="el-icon-arrow-up el-icon--right"></i>
                </el-button>
            </div>
        </div>
        <div class="admin_datatip" v-if="!simple">
            <i class="el-icon-document"></i> 数据统计：
            <span class="admin_datatip_n">共：{{ allNum }} 条</span>
            <span class="admin_datatip_n">未审核：{{ status1Num }} 条</span>
            <span class="admin_datatip_n">未通过：{{ status2Num }} 条</span>
            <span class="admin_datatip_n">已下架：{{ status3Num }} 条</span>
            <span class="admin_datatip_n">搜索结果：{{total }} 条</span>
        </div>

        <div class="moduleElTable" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe @sort-change='sortChange' @mousedown.native="mouseDownHandler" @mouseup.native="mouseUpHandler" @mousemove.native="mouseMoveHandler"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" @selection-change="handleSelectionChange"
                ref="multipleTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="职位ID" width="90" sortable="custom"></el-table-column>
                <el-table-column label="职位/企业" min-width="220">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <div class=" ">
                                <el-link :href="props.row.joburl" target="_blank" type="primary">{{ props.row.name }}</el-link>
                            </div>
                            <div class=" ">
                                <el-link :href="props.row.comurl" target="_blank">{{ props.row.com_name }}</el-link>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="等级/业务员 " min-width="100" v-if="!simple">
                    <template slot-scope="props">
                        <div class="">
                            <span class="" v-if="props.row.rating_name"> {{ props.row.rating_name }}</span>
                            <div class=""> <span class="gsd"> {{ props.row.crm_salesman }}</span></div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="简历量" width="130">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <span>简历数：<el-button type="text" style="padding: 0;" @click="showComJobLogBox(props.row, 0)">{{ props.row.snum }}</el-button></span>
                            <span>未查看：<el-button type="text" style="padding: 0;" @click="showComJobLogBox(props.row, 1)">{{ props.row.browseNum }}</el-button></span>
                            <span>已面试：<el-button type="text" style="padding: 0;" @click="showComUserIdMsgBox(props.row)">{{ props.row.inviteNum }}</el-button></span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label=" 职位推广" width="130">
                    <template slot-scope="props">
                        <div class="job_tg_bth">
                            <el-switch v-model="props.row.istop" @change="tgchange($event, props.row, 1)" inactive-text="置顶"></el-switch>
                        </div>
                        <div class="job_tg_bth">
                            <el-switch v-model="props.row.isrec" @change="tgchange($event, props.row, 2)" inactive-text="推荐"></el-switch>
                        </div>
                        <div class="job_tg_bth">
                            <el-switch v-model="props.row.isurgent" @change="tgchange($event, props.row, 3)" inactive-text="紧急"></el-switch>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label=" 招聘状态" width="130">
                    <template slot-scope="props">
                        <el-switch v-model="props.row.iszp" @change="zpstatuschange($event, props.row)"></el-switch>
                        <div class="gsd">{{ props.row.status != 1 ? '招聘中' : '已下架' }}</div>
                    </template>
                </el-table-column>
                <el-table-column prop="logintime" label="发布/更新时间" width="150">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <span class="gsd">{{ props.row.sdate_n }}</span>
                            <span>{{ props.row.lastupdate_n_n }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="ip" label="来源/IP/归属地" width="140" v-if="!simple">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <span>{{ source[props.row.source] }}</span>
                            <span v-if="props.row.add_ip">{{ props.row.add_ip }}</span>
                            <span class="gsd" v-if="props.row.add_ip && props.row.ip_address"> {{ props.row.ip_address}}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label=" 浏览量/曝光量" width="130">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <span class=" "> {{ props.row.jobhits }}/{{ props.row.jobexpoure }}</span>
                        </div>
                        <div class="jobtj">
                            <el-link icon="el-icon-edit" size="mini" @click="jobhitedit(props.row)">修改</el-link>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="zt" label="状态" fixed="right">
                    <template slot-scope="props">
                        <div class="admin_state">
                            <template v-if="props.row.r_status == '2'">
                                <span class="admin_state3">已锁定</span>
                                <div style="display:inline-block" v-if="props.row.lock_info">
                                    <el-popover trigger="hover" placement="right">
                                        <p>{{ props.row.lock_info }}</p>
                                        <div slot="reference" class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div>
                                    </el-popover>
                                </div>
                            </template>
                            <template v-else-if="props.row.state == 1">
                    			<span class="admin_state1">已审核</span>
                    		</template>
                            <template v-else-if="props.row.state == 0">
                    			<!--职位未审核的要显示企业审核状态，企业锁定的前面有单独处理-->
                    			<div v-if="props.row.r_status != '2'">
                    				<div>
                    					<span class="admin_state4" v-if="props.row.r_status == '0'">企:未审核</span>
                    					<span class="admin_state1" v-else-if="props.row.r_status == '1'">企:已审核</span>
                    					<span class="admin_state2" v-else-if="props.row.r_status == '3'">企:未通过</span>
                    					<span class="admin_state3" v-else-if="props.row.r_status == '4'">企:暂停</span>
                    				</div>
                    				<div>
                    					<span class="admin_state4">职:未审核</span>
                    				</div>
                    			</div>
                    			<div v-else>
                    				<span class="admin_state4">未审核</span>
                    			</div>
                    		</template>
                            <template v-else-if="props.row.state == 3">
                    			<!--职位未通过的要显示企业审核状态，企业锁定的前面有单独处理-->
                    			<div v-if="props.row.r_status != '2'">
                    				<div>
                    					<span class="admin_state4" v-if="props.row.r_status == '0'">企:未审核</span>
                    					<span class="admin_state1" v-else-if="props.row.r_status == '1'">企:已审核</span>
                    					<span class="admin_state2" v-else-if="props.row.r_status == '3'">企:未通过</span>
                    					<span class="admin_state3" v-else-if="props.row.r_status == '4'">企:暂停</span>
                    				</div>
                    				<div>
                    					<span class="admin_state2">职:未通过</span>
                    				</div>
                    			</div>
                    			<div v-else>
                    				<span class="admin_state2">未通过</span>
                    			</div>
                                <div style="display:inline-block" v-if="props.row.statusbody">
                                    <el-popover trigger="hover" placement="right">
                                        <p>{{ props.row.statusbody }}</p>
                                        <div slot="reference" class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div>
                                    </el-popover>
                                </div>
                            </template>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="操作" width="140" fixed="right">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <el-button size="small" plain @click="jobAudit(scope.row)">审核</el-button>
                            <el-button size="small" v-if="scope.row.status == 1" plain @click="msg('职位已下架！')">匹配</el-button>
                            <el-button size="small" v-else plain @click="tw(scope.row)">推文</el-button>
                            <el-button size="small" plain @click="edit(scope.row)">修改</el-button>
                            <el-popover placement="bottom" width="90" trigger="hover">
                                <div class="moduleMores">
                                    <template v-if="search_params.openautho == 2">
                                        <el-button type="text" @click="linkopen(scope.row)">权限</el-button>
                                        
                                    </template>
                                    <el-button v-else type="text" @click="linkopen(scope.row)">开放权限</el-button>
                                    <template v-if="scope.row.status == 0 && scope.row.state == 1 && scope.row.r_status == 1">
                                        <el-button @click="getJobHtml(scope.row.id)" type="text">复制文本</el-button>
                                        <el-button v-if="hbNum > 0 && hb_isopen == 1" @click="createhb(scope.row)" type="text">生成海报</el-button>
                                        <el-button type="text" @click="resumematch(scope.row)">匹配简历</el-button>
                                    </template>
                                    <el-button @click="yyrefresh(scope.row)" type="text">预约刷新</el-button>
                                    <el-button v-if="scope.row.is_depower == 1" @click="depower(2, scope.row.id)" type="text">取消降权</el-button>
                                    <el-button v-else type="text" @click="depower(1, scope.row.id)">降权</el-button>
                                    <el-button type="text" @click="delrow(scope.row.id)">删除职位</el-button>
                                </div>
                                <el-button size="small" plain slot="reference" @click="visible = !visible">更多</el-button>
                            </el-popover>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging" style="height: initial; flex-wrap: wrap; padding-top: 10px;">
            <div class="bottomButnBull" style="width:100%;">
                <div class="bottomButnBlak">
                    <el-checkbox v-model="checkedAll" @change="selectAllBottom">全选</el-checkbox>
                    <el-button @click="delAllBottom" size="mini">批量删除</el-button>
                    <el-button @click="multipleStatus" size="mini">审核</el-button>
                    <el-button @click="multitg(3)" size="mini">紧急</el-button>
                    <el-button @click="multitg(1)" size="mini">置顶</el-button>
                    <el-button @click="multitg(2)" size="mini">推荐</el-button>
                    <el-button @click="refresh" size="mini">刷新</el-button>
                    
                    <el-button @click="exportdrawer = true" size="mini">导出</el-button>
                    <el-button @click="multicate" size="mini">分类</el-button>
                    <el-button @click="twtaskall" size="mini">推文</el-button>
                </div>
            </div>
            <div class="modulePagNum" style="padding-top: 8px;">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" :current-page="currentPage" :page-sizes="pageSizes" :page-size="perPage" layout="total, sizes, prev, pager, next, jumper" :total="total"></el-pagination>
            </div>
        </div>
        <!--申请记录弹出框-->
        <el-drawer :title="applyJobBoxTitle" :visible.sync="drawerCompanyJobLog" append-to-body size="80%">
            <companyjoblog ref="companyjoblog" :searchjobid="jobid" :searchbrowse="sqJobBrowse" searchclass="drawer" v-if="drawerCompanyJobLog"></companyjoblog>
        </el-drawer>
        <!--面试记录弹出框-->
        <el-drawer :title="interviewBoxTitle" :visible.sync="drawerCompanyUserIdMsg" append-to-body size="80%">
            <companyuseridmsg ref="companyuseridmsg" :searchjobid="jobid" v-if="drawerCompanyUserIdMsg" searchclass="drawer"></companyuseridmsg>
        </el-drawer>
        <!--曝光量弹出-->
        <div class="modluDrawer" v-if="curr_job">
            <el-dialog title="浏览量编辑" :visible.sync="bgdrawer" :modal-append-to-body="false" append-to-body width="390px">
                <div>
                    <div class="wxsettip_small ">企业名称</div>
                    <el-input v-model="curr_job.com_name" placeholder="企业名称" :disabled="true"></el-input>
                    <div class="wxsettip_small ">曝光量</div>
                    <el-input type="number" v-model="curr_job.jobexpoure" placeholder="曝光量"></el-input>
                    <div class="wxsettip_small ">浏览量</div>
                    <el-input type="number" v-model="curr_job.jobhits" placeholder="浏览量"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="bgdrawer = false">取 消</el-button>
                    <el-button type="primary" :loading="jobhit_load" @click="jobhiteditsave">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--企业发送至推文弹窗-->
        <div class="modluDrawer">
            <el-dialog title="职位发送至推文" :visible.sync="twdrawer" :modal-append-to-body="false" append-to-body width="450px">
                <div v-if="curr_job || multitw">
                    <div v-if="!multitw" class="wxsettip_small ">职位名称</div>
                    <el-input v-if="!multitw" placeholder="职位名称" v-model="curr_job.name" :disabled="true"></el-input>
                    <div class="wxsettip_small ">标签</div>
                    <el-checkbox v-model="twtask_urgent">加急</el-checkbox>
                    <el-checkbox v-model="twtask_wcmoments">朋友圈</el-checkbox>
                    <el-checkbox v-model="twtask_gzh">公众号</el-checkbox>
                    <div class="wxsettip_small ">备注</div>
                    <el-input type="textarea" :rows="2" placeholder="请输入备注" v-model="twtask_content"></el-input>
                    <div class="tw_tip" v-if="twTip">
                        <el-alert :title="twTip" type="warning" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="twdrawer = false">取 消</el-button>
                    <el-button type="primary" :loading="twtask_load" @click="addTwTask">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--联系方式选择性开放弹窗-->
        <div class="modluDrawer">
            <el-dialog title="联系方式选择性开放" :visible.sync="drawerlinkopen" :modal-append-to-body="false" append-to-body width="350px">
                <div v-if="curr_job">
                    <el-radio-group v-model="curr_job.linkopen" size="small">
                        <el-radio label="1" border>默认</el-radio>
                        <el-radio label="2" border>开放</el-radio>
                    </el-radio-group>
                    <div class="tw_tip">
                        <el-alert title="默认" description="默认职位联系方式逻辑不变" type="warning" show-icon :closable="false"></el-alert>
                        <el-alert title="开放" type="warning" show-icon :closable="false">
                            <span>开启登录短信验证码，未登录用户有预留<br />信息提示，非必填。登录用户直接查看</span>
                        </el-alert>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawerlinkopen = false">取 消</el-button>
                    <el-button type="primary" @click="setlinkopen">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--职位详情审核 ---------------------------------------------------------------------->
        <el-drawer title="职位审核" :visible.sync="jobdrawersh" :modal-append-to-body="false" append-to-body size="80%">
            <job_review :id="statusId" :comclass_name="jobcomclassnamecache" :job_audit="job_audit" @confirm="jobdrawersh=false;getList()"   ></job_review>
        </el-drawer>
        <!--职位推广弹窗-->
        <div class="modluDrawer">
            <el-dialog :title="jobtgtit" :visible.sync="jobtgdrawer" append-to-body width="400px">
                <div class="wxsettip_small" v-if="jobtgtype == 1">置顶天数</div>
                <div class="wxsettip_small" v-else-if="jobtgtype == 2">推荐天数</div>
                <div class="wxsettip_small" v-else-if="jobtgtype == 3">紧急天数</div>
                <el-input type="number" placeholder="请输入天数" v-model="jobtgdays">
                    <template slot="append">天</template>
                </el-input>
                <div class="wxsettip_small" v-if="jobtgetime != ''">当前结束日期</div>
                <el-input v-if="jobtgetime != ''" v-model="jobtgetime" disabled></el-input>
                <div style="margin-top:10px;">
                    <i class="el-icon-warning"></i>
                    如需取消
                    <span v-if="jobtgtype == 1">职位置顶</span>
                    <span v-else-if="jobtgtype == 2">推荐职位</span>
                    <span v-else-if="jobtgtype == 3">紧急职位</span>
                    请单击
                    <el-checkbox v-model="qxtgchecked" true-label="1" false-label="0"></el-checkbox>
                    <span>点击确认即可</span>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="jobtgdrawer = false">取 消</el-button>
                    <el-button type="primary" :loading="jobtg_load" @click="jobTgSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--复制文本弹窗-->
        <div class="modluDrawer">
            <el-dialog title="复制文本" :visible.sync="drawercopy" append-to-body width="290px">
                <div id="to_copy" v-html="htmlcont"></div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawercopy = false">取 消</el-button>
                    <el-button type="primary" id="copyBtn" data-clipboard-action="copy" data-clipboard-target="#to_copy" @click="handleCopyText('copyBtn')">复制文本</el-button>
                </span>
            </el-dialog>
        </div>
        <!--生成海报弹窗-->
        <div class="modluDrawer">
            <el-drawer :title="'生成海报'" :visible.sync="drawerhb" :modal-append-to-body="false" append-to-body size="95%">
                <div class="waixunHaib">
                    <ul>
                        <li class="" v-for="(item, index) in hbarr" :key="index">
                            <div class="hb_listbox">
                                <div class="poster_pic"><img :src="item.pic_n"></div>
                                <div class="hb_listbox_name" style="background:#fff;">
                                    <div class="hb_cz">
                                        <a href="javascript:;" @click="showHb(item.id)">预览</a>
                                        <a href="javascript:;" @click="downHb(item.id)">下载</a>
                                    </div>
                                </div>
                            </div>
                        </li>
                    </ul>
                </div>
            </el-drawer>
        </div>
        <!-- 海报预览弹窗 -->
        <div class="tck_setbox" v-if="hburl != ''">
            <el-dialog title="海报预览" :visible.sync="showhb" append-to-body width="300px">
                <div class="code_img" style="display:flex;justify-content: center;margin-bottom: 20px;">
                    <img :src="hburl" :key="hbkey" width="260">
                </div>
            </el-dialog>
        </div>
        <!--匹配简历弹窗-->
        <div class="modluDrawer">
            <el-drawer title="匹配简历" :visible.sync="drawermatchresume" :modal-append-to-body="false" append-to-body size="95%">
                <matchresume ref="matchresume" :job="curr_job" :jobtypes="job_types" :citytypes="city_types"></matchresume>
            </el-drawer>
        </div>
        <!--批量审核职位-->
        <div class="modluDrawer">
            <el-dialog title="职位批量审核" width="300px" :visible.sync="drawerauditmultiple" append-to-body :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>审核操作：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-radio v-model="multiStatus" label="1">正常</el-radio>
                            <el-radio v-model="multiStatus" label="3">未通过</el-radio>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>审核说明：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input type="textarea" v-model="multiStatusBody" :rows="2" placeholder="请输入审核说明">
                            </el-input>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawerauditmultiple = false">取 消</el-button>
                    <el-button type="primary" :loading="multipleStatus_load" @click="multipleStatusSave">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--导出字段弹窗-->
        <div class="modluDrawer">
            <el-dialog title="选择导出字段" :visible.sync="exportdrawer" append-to-body width="740px">
                <div style="">
                    <el-checkbox-group v-model="checkedCols" @change="handleColCheckedChange">
                        <el-checkbox style="width:110px;margin-bottom: 5px;margin-left:0" size="small" border v-for="(item, index) in cols" :key="index" :label="item.value">{{ item.label }}</el-checkbox>
                        <el-checkbox style="width:110px;margin-left:0" size="small" border :indeterminate="isIndeterminate" v-model="colCheckAll" @change="handleColCheckAllChange">全选</el-checkbox>
                    </el-checkbox-group>
                </div>
                <div class="wxsettip_small">导出数量</div>
                <el-input type="number" placeholder="请输入导出数量" v-model="exp_num"></el-input>
                <el-alert style="margin-top: 10px;" title="数字太大会导致运行缓慢，请慎重填写。" type="warning" show-icon :closable="false"></el-alert>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="exportdrawer = false">取 消</el-button>
                    <el-button type="primary" :loading="export_load" @click="submitExport">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--批量转移类别-->
        <div class="modluDrawer">
            <el-dialog title="批量转移类别" :visible.sync="drawermulticate" append-to-body width="300px">
                <div class="wxsettip_small">行业类别</div>
                <div class="TableSelect">
                    <el-select v-model="multihy" placeholder="请选择">
                        <el-option v-for="(item, index) in cacheData.industry_index" :key="index" :label="cacheData.industry_name[item]" :value="item"></el-option>
                    </el-select>
                </div>
                <div class="wxsettip_small">职位类别</div>
                <div class="TableInpt">
                    <el-cascader style="width:260px;" v-model="multijobtype" :options="job_types" filterable clearable></el-cascader>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawermulticate = false">取 消</el-button>
                    <el-button type="primary" :loading="multicate_load" @click="submitMulticate">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--修改职位提示-->
        <el-drawer title="修改职位" :visible.sync="drawerEditJob" append-to-body :wrapper-closable="false" size="880px">
            <addjob ref="jobedit" :jid="jobid" :jtypes="job_types" :ctypes="city_types" v-if="drawerEditJob"></addjob>
        </el-drawer>
    </div>
    <!--        预约刷新职位-->
    <div class="modluDrawer">
        <el-dialog title="预约刷新调整" :visible.sync="drawertz" :with-header="true" append-to-body :show-close="true"
                   width="400px">
            <div>
                <div class="wxsettip_small">刷新状态</div>
                <div class="TableInpt">
                    <el-radio v-model="curr_data.reserve_status" label="1">开启</el-radio>
                    <el-radio v-model="curr_data.reserve_status" label="2">关闭</el-radio>
                </div>
                <div class="wxsettip_small">刷新间隔</div>
                <div class="TableSelect">
                    <el-select v-model="curr_data.reserve_interval" placeholder="请选择">
                        <el-option v-for="(item, index) in jg_data" :key="index" :label="item.label" :value="item.value">
                        </el-option>
                    </el-select>
                </div>
                <div v-if="curr_data.reserve_interval == 1" class="wxsettip_small">自定义间隔</div>
                <div class="TableInpt" v-if="curr_data.reserve_interval == 1">
                    <el-input v-model="userinterval" placeholder="请输入自定义间隔" size="small" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')">
                        <template slot="append">分钟</template>
                    </el-input>
                </div>
                <div class="wxsettip_small">截止日期</div>
                <div class="TableInpt">
                    <el-date-picker v-model="curr_data.reserve_end" value-format="yyyy-MM-dd" type="date" placeholder="选择日期" :picker-options="pickerOptions">
                    </el-date-picker>
                </div>
                <div class="wxsettip_small">刷新时间段</div>
                <div class="TableInpt">
                    <el-time-picker v-model="curr_data.s_time" value-format="HH:mm">
                    </el-time-picker>
                    <div class="TableInptline">-</div>
                    <el-time-picker v-model="curr_data.e_time" value-format="HH:mm">
                    </el-time-picker>
                </div>
            </div>
            <span slot="footer" class="dialog-footer">
					<el-button @click="drawertz = false">取 消</el-button>
					<el-button type="primary" @click="submitTz" :loading="saveLoading">确 定</el-button>
				</span>
        </el-dialog>
    </div>
</template>
<script>
module.exports = {
    props: {
        uid: { type: String, default: '' },
        state: { type: String, default: '' },
        status: { type: [String, Number], default: '' },
        crmindex: { type: String, default: '' },
        adtime: { type: String, default: '' },
        keyword: { type: String, default: '' },
        type: { type: String, default: '1' },
        simple: { // 简化职位管理，CRM模块调用
            type: Boolean,
            default: false
        },
        tsjl: { // 简化职位管理，CRM模块调用
            type: Boolean,
            default: false
        },
        scrolltop: { // 会员职位管理切换分页信息滚动回顶部
            type: Boolean,
            default: false
        },
    },
    data: function () {
        return {
            mouseFlag: false,
            mouseOffset: 0,
            loading: false,
            emptytext: '暂无数据',
            allNum: 0,
            status1Num: 0,
            status2Num: 0,
            status3Num: 0,
            // 批量转移类别
            drawermulticate: false,
            multihy: '',
            multijobtype: [],
            multitw: false, // 是否是批量推文任务
            drawermatchresume: false,
            drawercopy: false,
            htmlcont: '',
            hbNum: '0',
            hb_isopen: '0',
            drawerlinkopen: false,
            twTip: '',
            twtask_urgent: false,
            twtask_wcmoments: false,
            twtask_gzh: false,
            twtask_content: '',
            cacheData: {},
            sh_num: 0,
            jobtgtype: '',
            jobtgtit: '',
            jobtgdays: '',
            jobtgetime: '',
            qxtgchecked: '0',
            jobtgdrawer: false,
            inputVisible: false,
            inputValue: '',
            exp_num: '',
            isIndeterminate: true,
            checkedCols: [],
            colCheckAll: false,
            exportdrawer: false,
            searchlist: null,
            source: [],
            search_params: {
                type: '1',
                keyword: this.keyword,
                uid: this.uid,
                state: this.state,
                status: this.status,
                jtype: '',
                exp: '',
                edu: '',
                source: '',
                adtime: this.adtime,
                rating: '',
                openautho: '',
                
                is_depower: '',
                fromCrmIndex: this.crmindex,
                time_type:'sdate',
                times:''
            },
            checkedAll: false,
            selectedItem: [],
            tableData: [],
            currentPage: 1,
            perPage: 0,
            pageSizes: [],
            total: 0,
            drawerauditmultiple: false,
            multiStatus: '',
            multiStatusBody: '',
            cols: [
                { label: '职位ID', value: 'id' },
                { label: '企业UID', value: 'uid' },
                { label: '职位名称', value: 'name' },
                { label: '行业', value: 'hy' },
                { label: '一级类别', value: 'job1' },
                { label: '二级类别', value: 'job1_son' },
                { label: '三级类别', value: 'job_post' },
                { label: '省', value: 'provinceid' },
                { label: '市', value: 'cityid' },
                { label: '县', value: 'three_cityid' },
                { label: '薪水', value: 'minsalary,maxsalary' },
                { label: '招聘人数', value: 'zp_num' },
                { label: '工作经验', value: 'exp' },
                { label: '到岗时间', value: 'report' },
                { label: '性别要求', value: 'sex' },
                { label: '教育程度', value: 'edu' },
                { label: '婚姻状况', value: 'marriage' },
                { label: '开始日期', value: 'sdate' },
                { label: '更新时间', value: 'lastdate' },
                { label: '年龄要求', value: 'zp_minage,zp_maxage' },
                { label: '语言要求', value: 'lang' },
                { label: '福利待遇', value: 'welfare' },
                { label: '公司名称', value: 'com_name' },
                { label: '公司性质', value: 'pr' },
                { label: '企业规模', value: 'mun' }
            ],
            sort_type: '',
            sort_col: '',
            curr_job: null,
            auditInfo: null,
            r_status:'',
            job_audit:{},
            drawerCompanyJobLog: false,
            applyJobBoxTitle: '职位申请记录',
            sqJobBrowse: '',
            drawerCompanyUserIdMsg: false,
            interviewBoxTitle: '职位面试记录',
            editjob: null,
            drawerEditJob: false,
            jobCompany: null,
            jobAddressList: [],
            job_types: [],
            city_types: [],
            sel_jobtype: [],
            jionly: 0,
            jobcomdatacache: [],
            jobcomclassnamecache: [],
            checkedwelfare: [],
            showJob: false,
            visible: false,
            drawerhb: false,
            hbarr: [],
            basehburl: '',
            hburl: '',
            hbkey: '',
            showhb: false,
            tgjid: '',
            islook: false,
            bgdrawer: false,
            twdrawer: false,
            jobdrawersh: false,
            seachbutn: true,
            tableHig: true,
            jobid: '',
            jobtg_load: false,
            multipleStatus_load: false,
            export_load: false,
            multicate_load: false,
            twtask_load: false,
            jobhit_load: false,
            audit_load: false,
            prevPage: 0,

            //  预约刷新
            curr_data: {
                reserve_end:'',
                reserve_interval:'60',
                s_time:'09:00',
                e_time:'17:00',
                reserve_status:2
            },
            drawertz: false,
            userinterval:'',
            saveLoading:false,
            jg_data: [
                {label: '每隔1小时', value: '60'},
                {label: '每隔2小时', value: '120'},
                {label: '每隔3小时', value: '180'},
                {label: '每隔4小时', value: '240'},
                {label: '每隔5小时', value: '300'},
                {label: '每隔6小时', value: '360'},
                {label: '每隔7小时', value: '420'},
                {label: '每隔8小时', value: '480'},
                {label: '自定义刷新间隔', value: '1'},
            ],
            pickerOptions: {//el-date-picker 时间限定
                disabledDate(time) {
                    // 今天及今天之前的日期
                    // return time.getTime() > Date.now();
                    // 今天及今天之后的日期
                    return time.getTime() < Date.now() - 8.64e7;
                }
            },
            statusId:0,
            isSearchTime: false,
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

        }
    },
    components: {
        'companyjoblog': httpVueLoader('./comlog_index.vue'),
        'job_review': httpVueLoader('./job_review.vue'),
        'companyuseridmsg': httpVueLoader('./comlog_useridmsg.vue'),
        'matchresume': httpVueLoader('./match_resume.vue'),
        'addjob': httpVueLoader('./addjob.vue'),
        'job_class': httpVueLoader('../../../component/job_class.vue'),
        'city_class': httpVueLoader('../../../component/city_class.vue'),
    },
    mounted() {
        var that = this
        setTimeout(function () {
            that.getTjNum();
            that.getCacheFun();
            that.getHBFun();
        }, 200)
    },
    created() {
        let params = window.parent.homeapp.$route.params;
        let query = window.parent.homeapp.$route.query;

        if (!$.isEmptyObject(query)) {
            params = {...params,...query};
        }
        if (!$.isEmptyObject(params)) {
            delete params.activeName;
            this.getParams(params);
        }
        this.getList();
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

        // 职位修改
        edit: function (row) {
            var that = this
            that.jobid = row.id
            that.drawerEditJob = true;
            setTimeout(function() {
                that.$nextTick(function () {
                    that.$refs.jobedit.edit();
                })
            }, 500);
        },
        // 获取职位数量统计
        getTjNum: function () {
            var that = this;
            httpPost('m=user&c=company_job&a=jobNum', {}, { hideloading: true }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    that.allNum = res.data.jobAllNum ? res.data.jobAllNum : 0
                    that.status1Num = res.data.jobStatusNum1 ? res.data.jobStatusNum1 : 0
                    that.status2Num = res.data.jobStatusNum2 ? res.data.jobStatusNum2 : 0
                    that.status3Num = res.data.jobStatusNum3 ? res.data.jobStatusNum3 : 0
                }
            }).catch(function (e) {
                console.log(e)
            })
        },

        // 批量修改职位类别
        multicate: function () {
            if (this.selectedItem.length == 0) {
                message.error('请选择要操作的数据项')
                return false
            }
            this.multijobtype = ['', '', '']
            this.drawermulticate = true
        },
        // 批量修改职位类别提交
        submitMulticate: function () {
            var that = this
            if (that.selectedItem.length == 0) {
                message.error('请选择要操作的数据项')
                return false
            }
            if (that.multihy == "") {
                message.error("请选择行业类别")
                return false
            }
            if (that.multijobtype[0] == '' || that.multijobtype[1] == '') {
                message.error("请选择职位类别")
                return false
            }
            that.multicate_load = true;
            httpPost('m=user&c=company_job&a=saveclass', {
                jobid: that.selectedItem.join(','),
                hy: that.multihy,
                job1: that.multijobtype[0],
                job1_son: that.multijobtype[1],
                job_post: that.multijobtype[2],
            }).then(function (response) {
                that.multicate_load = false;
                let res = response.data;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    message.success(res.msg, function () {
                        that.drawermulticate = false;
                        that.getList();
                    })
                }
            })
        },
        handleColCheckAllChange(val) {
            var that = this
            if (val) {
                that.cols.forEach(item => {
                    that.checkedCols.push(item.value);
                });
            } else {
                that.checkedCols = []
            }
            this.isIndeterminate = false;
        },
        handleColCheckedChange(value) {
            let checkedCount = value.length;
            this.colCheckAll = checkedCount === this.cols.length;
            this.isIndeterminate = checkedCount > 0 && checkedCount < this.cols.length;
        },
        // 导出exp_num
        submitExport() {
            let that = this
            if (that.checkedCols.length == 0) {
                message.error('请选择要操作的数据项');
                return;
            }
            params = {
                pid: that.selectedItem.join(','),
                type: that.checkedCols,
                limit: that.exp_num
            }
            that.export_load = true;
            httpPost('m=user&c=company_job&a=xls', params).then(function (response) {
                that.export_load = false;
                let res = response.data;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.exportdrawer = false;
                    utilFile.downloadFileByByte(res.data.file, res.data.file_name);
                }
            })
        },
        // 批量审核
        multipleStatus() {
            var that = this
            if (!that.selectedItem.length) {
                message.error('请选择要操作的数据项');
                return false;
            }
            that.drawerauditmultiple = true
        },
        // 批量审核保存
        multipleStatusSave() {
            var that = this
            if (!that.selectedItem.length) {
                message.error('请选择要操作的数据项');
                return false;
            }
            that.multipleStatus_load = true;
            httpPost('m=user&c=company_job&a=status', {
                pid: that.selectedItem.join(','),
                status: that.multiStatus,
                statusbody: that.multiStatusBody
            }).then(function (result) {
                that.multipleStatus_load = false;
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.drawerauditmultiple = false
                        that.multiStatus = ''
                        that.multiStatusBody = ''
                        that.getList()
                    })
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 批量刷新
        refresh: function () {
            var that = this
            if (this.selectedItem.length == 0) {
                message.error("请选择要操作的数据项")
                return false
            }
            httpPost('m=user&c=company_job&a=refresh', {
                ids: this.selectedItem.join(',')
            }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    message.success('刷新成功！', function () {
                        that.getList()
                    })
                } else {
                    message.error('刷新失败！')
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 降权、取消降权
        depower: function (type, id) {
            var msg = '',
                that = this;
            if (type == 1) {
                msg = '确定降权？降权后职位仅在企业详情显示';
            } else {
                msg = '确定取消降权？取消后职位将正常显示在职位列表';
            }

            var params = {
                id: id,
                is_depower: type
            };

            delConfirm(that, params, this.depowerPost,msg)
        },
        async depowerPost(params) {

            let that = this;

            httpPost('m=user&c=company_job&a=depower', params).then(function (result) {

                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList()
                    })
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 匹配简历
        resumematch: function (row) {
            var that = this
            that.curr_job = row
            that.drawermatchresume = true
            that.$nextTick(function () {
                that.$refs.matchresume.getList()
            })
        },
        // 生成海报弹窗
        createhb: function (row) {
            var that = this
            that.curr_job = row
            httpPost('m=user&c=company_job&a=whb', {}).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    that.hbarr = res.data.comHb
                    that.basehburl = res.data.hburl
                    that.drawerhb = true
                } else {
                    message.error('获取企业海报模板失败')
                    return false
                }
            }).catch(function (e) {

            })
        },
        // 下载海报
        downHb(style) {
            var that = this
            let image = new Image()
            image.setAttribute('crossOrigin', 'anonymous')
            that.hburl = that.basehburl + '&id=' + that.curr_job.id + '&hb=' + style
            image.src = that.hburl
            image.onload = () => {
                let canvas = document.createElement('canvas')
                canvas.width = image.width
                canvas.height = image.height
                let ctx = canvas.getContext('2d')
                ctx.drawImage(image, 0, 0, image.width, image.height)
                canvas.toBlob((blob) => {
                    let url = URL.createObjectURL(blob)
                    download(url, 'whb' + style)
                    // 用完释放URL对象
                    URL.revokeObjectURL(url)
                })
            }

            function download(href, name) {
                let eleLink = document.createElement('a')
                eleLink.download = name
                eleLink.href = href
                eleLink.click()
                eleLink.remove()
            }
        },
        // 预览海报
        showHb(style) {
            this.hburl = this.basehburl + '&id=' + this.curr_job.id + '&hb=' + style
            this.hbkey = Math.random()
            this.showhb = true
        },
        // 复制文本弹窗
        getJobHtml: function (id) {
            var that = this
            httpPost('m=user&c=company_job&a=getJobHtml', { id: id }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.htmlcont = res.data
                    that.drawercopy = true
                }
            }).catch(function (e) {
                console.log(e)
            })
        },

        // 复制文本
        handleCopyText: function (id) {
            let clipboard = new ClipboardJS('#' + id); // 获取点击按钮的元素
            clipboard.on('success', (e) => {
                e.clearSelection();
                clipboard.destroy();
                message.success('复制成功');
            });
            // 复制失败
            clipboard.on('error', (e) => {
                clipboard.destroy();
                message.error('该浏览器不支持自动复制');
            });
        },
        
        // 权限、开放权限
        linkopen: function (row) {
            this.curr_job = row
            this.drawerlinkopen = true
        },
        setlinkopen: function () {
            var that = this
            httpPost('m=user&c=company_job&a=setlinkopen', { linkjobid: that.curr_job.id, linkopen: that.curr_job.linkopen }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.drawerlinkopen = false
                    })
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 消息提示
        msg: function (msg) {
            message.error(msg)
            return false
        },
        // 批量推文任务
        twtaskall: function () {
            var that = this
            var nowTime = parseInt(new Date().getTime() / 1000);
            var lastupdate = '';
            var twnum = 0;
            var statusMsg = '';
            var stateMsg = '';
            var rstatusMsg = '';
            this.tableData.forEach(function (item) {
                if (that.selectedItem.includes(item.id)) {
                    if (twnum == 0) {
                        twnum = parseInt(item.tw_num);
                    }
                    lastupdate = parseInt(item.lastupdate);
                    if (that.twTip == '' && nowTime - lastupdate > 60 * 60 * 24 * 3) {
                        that.twTip = '有部分职位已超过3天没有更新，请确认无误后添加';
                    }
                    if (item.status != '0') {
                        statusMsg = '已下架';
                    }
                    if (item.state != '1') {
                        stateMsg = '未审核';
                    }
                    if (item.r_status != '1') {
                        rstatusMsg = '对应企业未审核';
                    }
                }
            })
            if (statusMsg != '' || stateMsg != '' || rstatusMsg != '') {
                var msg = '部分所选职位';
                var douhao = '';
                if (statusMsg != '') {
                    msg += douhao + statusMsg;
                    douhao = '、';
                }
                if (stateMsg != '') {
                    msg += douhao + stateMsg;
                    douhao = '、';
                }
                if (rstatusMsg != '') {
                    msg += douhao + rstatusMsg;
                    douhao = '、';
                }
                msg += '，请重新选择。';
                message.error(msg)
                return false
            }
            if (that.selectedItem.length == 0) {
                message.error("请选择要操作的数据项")
                return false
            } else if (twnum > 0) {
                delConfirm(this, {}, function (params) {
                    that.multitw = true
                    that.twdrawer = true
                }, '有部分职位已添加推文未发送，是否继续添加？')
            } else {
                that.multitw = true
                that.twdrawer = true
            }
        },
        // 推文
        tw: function (row) {
            var that = this
            this.curr_job = row
            if (this.curr_job.tw_num > 0) {
                delConfirm(this, {}, function (params) {
                    that.addTw()
                }, '该职位上有推文未发送，是否继续添加？')
            } else {
                that.addTw()
            }
        },
        addTw: function () {
            var nowTime = parseInt(new Date().getTime() / 1000);
            lastupdate = Number(this.curr_job.lastupdate);
            this.twTip = '';
            if (nowTime - lastupdate > 60 * 60 * 24 * 3) {
                this.twTip = '职位已超过3天没有更新，请确认无误后添加';
            }
            this.twdrawer = true
        },
        // 添加推文任务
        addTwTask: function () {
            var that = this
            var params = {
                twtask_content: that.twtask_content,
                twtask_urgent: that.twtask_urgent ? 1 : 0,
                twtask_wcmoments: that.twtask_wcmoments ? 1 : 0,
                twtask_gzh: that.twtask_gzh ? 1 : 0,
            };
            if (that.multitw) { // 批量推文任务
                params.twtask_jobid = that.selectedItem.join(',')
            } else {
                params.twtask_jobid = that.curr_job.id
            }
            that.twtask_load = true;
            httpPost('m=user&c=company_job&a=addTuiWenTask', params).then(function (result) {
                that.twtask_load = false;
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.multitw = false
                        that.twdrawer = false
                        that.tableData.forEach(item => {
                            if (item.id == that.curr_job.id){
                                item.tw_num++;
                            }
                        });
                    })
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 职位审核审核模板选择
        auditTplChange: function (data) {
            this.auditInfo.statusbody = this.cacheData.comclass_name[data]
        },
        // 职位审核弹窗
        jobAudit: function (row) {

            let that = this;
            this.statusId= row.id;
            that.jobdrawersh = true;
        },
        // 批量推广
        multitg: function (type) {
            this.jobtgtype = type
            this.jobtgetime = ''
            if (this.selectedItem.length == 0) {
                message.error('请选择要操作的数据项')
                return false
            }
            if (type == 1) { // 置顶
                this.jobtgtit = '职位批量置顶'
            } else if (type == 2) { // 推荐
                this.jobtgtit = '职位批量推荐'
            } else if (type == 3) { // 紧急
                this.jobtgtit = '紧急批量招聘'
            }
            this.tgjid = this.selectedItem.join(',')
            this.jobtgdrawer = true
        },
        // 职位推广设置
        tgchange: function (val, data, type) {
            this.jobtgtype = type
            this.curr_job = data
            this.tgjid = data.id
            if (type == 1) { // 置顶
                this.curr_job.istop = !this.curr_job.istop // 防止switch状态直接改变
                this.jobtgetime = data.top_time_n ? data.top_time_n : ''
                this.jobtgtit = '职位置顶'
            } else if (type == 2) { // 推荐
                this.curr_job.isrec = !this.curr_job.isrec // 防止switch状态直接改变
                this.jobtgetime = data.rec_time_n != undefined ? data.rec_time_n : ''
                this.jobtgtit = '职位推荐'
            } else if (type == 3) { // 紧急
                this.curr_job.isurgent = !this.curr_job.isurgent // 防止switch状态直接改变
                this.jobtgetime = data.urgent_time_n ? data.urgent_time_n : ''
                this.jobtgtit = '紧急招聘'
            }
            this.jobtgdrawer = true
        },
        // 职位推广提交
        jobTgSubmit: function () {
            var that = this
            var url = 'm=user&c=company_job&a='
            if (that.jobtgtype == 1) {
                url += 'xuanshang'
                if (that.qxtgchecked == 0 && that.jobtgdays == '') {
                    message.error('置顶天数不能为空')
                    return false
                }
            } else if (that.jobtgtype == 2) {
                url += 'recommend'
                if (that.qxtgchecked == 0 && that.jobtgdays == '') {
                    message.error('推荐天数不能为空')
                    return false
                }
            } else if (that.jobtgtype == 3) {
                url += 'urgent'
                if (that.qxtgchecked == 0 && that.jobtgdays == '') {
                    message.error('紧急天数不能为空')
                    return false
                }
            }
            var params = {
                pid: that.tgjid,
                days: that.jobtgdays,
                s: that.qxtgchecked
            }
            that.jobtg_load = true;
            httpPost(url, params).then(function (result) {
                that.jobtg_load = false;
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList()
                        that.jobtgdrawer = false
                        that.jobtgdays = ''
                        that.qxtgchecked = '0'
                    })
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 职位招聘状态修改
        zpstatuschange: function (val, row) {
            var that = this
            that.curr_job = row
            that.curr_job.iszp = !that.curr_job.iszp // 提交请求前禁止switch状态改变
            httpPost('m=user&c=company_job&a=checkstate', { id: that.curr_job.id, state: val ? 2 : 1 }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.curr_job.iszp = !that.curr_job.iszp // 操作成功改变switch 选中状态
                    that.getList()
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 浏览量修改
        jobhitedit: function (row) {
            var that = this
            that.curr_job = deepClone(row);
            that.curr_job.jobhits = that.curr_job.job_hits;
            that.curr_job.jobexpoure = that.curr_job.job_expoure;
            if (that.curr_job.r_status == 2) {
                message.error("用户已锁定,无法修改相关信息")
                return false
            } else {
                that.bgdrawer = true
            }
        },
        jobhiteditsave: function () {
            var that = this
            if (parseInt(that.curr_job.jobexpoure) < parseInt(that.curr_job.jobhits)) {
                message.error("曝光量不能低于浏览量");
                return false;
            }
            var params = {
                pid: that.curr_job.id,
                jobhits: that.curr_job.jobhits,
                jobexpoure: that.curr_job.jobexpoure
            }
            that.jobhit_load = true;
            httpPost('m=user&c=company_job&a=upjobhits', params).then(function (result) {
                that.jobhit_load = false;
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList()
                        that.bgdrawer = false
                    })
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        showInput() {
            this.inputVisible = true;
            this.$nextTick(_ => {
                this.$refs.saveTagInput.$refs.input.focus();
            });
        },
        welfareInputConfirm() {
            let inputValue = this.inputValue;
            if (inputValue) {
                this.curr_job.all_welfare.push(inputValue);
                this.checkedwelfare.push(inputValue)
            }
            this.inputVisible = false;
            this.inputValue = '';
        },
        handleSelectionChange(val) {
            this.selectedItem = [];
            let _this = this;
            if (val.length) {
                val.forEach(item => {
                    _this.selectedItem.push(item.id);
                });
            }
            if (_this.selectedItem.length == 0) {
                _this.checkedAll = false;
            } else {
                if (_this.selectedItem.length == _this.tableData.length) {
                    _this.checkedAll = true;
                } else {
                    _this.checkedAll = false;
                }
            }
        },
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
        handleSizeChange(val) {
            this.perPage = val;
            if (this.scrolltop) {
                scrollToTop()
            }
            this.getList()
        },
        handleCurrentChange(val) {
            this.currentPage = val;
            this.getList()
        },
        sortChange: function (column) {
            if (column.order == 'descending') {
                this.sort_type = 'desc';
            } else if (column.order == 'ascending') {
                this.sort_type = 'asc';
            } else {
                this.sort_type = '';
            }
            this.sort_col = column.prop
            this.search();
        },
        search() {
            this.currentPage = 1;
            this.getList();
        },
        getCacheFun: function () {
            let that = this;
            httpPost('m=user&c=company_job&a=getCacheData', {}, { hideloading: true }).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.cacheData = res.data.cache
                    that.job_types = res.data.job_types
                    that.city_types = res.data.city_types
                    that.jionly = res.data.jionly
                    that.jobcomdatacache = res.data.comdata
                    that.jobcomclassnamecache = res.data.comclass_name;

                    that.searchlist = res.data.search_list;
                    that.source = res.data.search_list.source.value;
                }
            })
        },
        getHBFun: function () {
            let that = this;
            httpPost('m=user&c=company_job&a=getHbData', {}, { hideloading: true }).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.hb_isopen = res.data.hb_isopen;
                    that.hbNum = res.data.hbNum;
                }
            })
        },
        async getList() {
            let that = this;
            let params = {
                page: that.currentPage,
                pageSize: that.perPage
            }
            if (that.search_params.type) {
                params.type = that.search_params.type
            }
            if (that.search_params.keyword) {
                params.keyword = that.search_params.keyword
            }
            if (that.search_params.uid) {
                params.uid = that.search_params.uid
            }
            if (that.search_params.state) {
                params.state = that.search_params.state
            }
            if (that.search_params.status) {
                params.status = that.search_params.status
            }
            if (that.search_params.jtype) {
                params.jtype = that.search_params.jtype
            }
            if (that.search_params.exp) {
                params.exp = that.search_params.exp
            }
            if (that.search_params.edu) {
                params.edu = that.search_params.edu
            }
            if (that.search_params.source) {
                params.source = that.search_params.source
            }
            if (that.search_params.adtime) {
                params.adtime = that.search_params.adtime
            }
            if (that.search_params.rating) {
                params.rating = that.search_params.rating
            }
            if (that.search_params.openautho) {
                params.openautho = that.search_params.openautho
            }
            
            if (that.search_params.is_depower) {
                params.is_depower = that.search_params.is_depower
            }
            if (that.search_params.gw) {
                params.gw = that.search_params.gw
            }
            if (that.search_params.job_class) {
                params.job_class = that.search_params.job_class
            }
            if (that.search_params.city_class) {
                params.city_class = that.search_params.city_class
            }
            if (that.search_params.fromCrmIndex) {
                params.fromCrmIndex = that.search_params.fromCrmIndex
            }
            if (that.search_params.time_type != '') {
                params.time_type = that.search_params.time_type;
            }
            if (Array.isArray(that.search_params.times) && that.search_params.times.length == 2) {
                params.times = that.search_params.times;
            }
            if (that.sort_type && that.sort_col) {
                params.order = that.sort_type
                params.t = that.sort_col
            }
            that.loading = true;
            that.emptytext = "数据加载中";

            if (this.simple){
                params.simple = 1;
            }
            httpPost('m=user&c=company_job&a=index', params, { hideloading: true }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.tableData = res.data.list
                    that.perPage = parseInt(res.data.perPage)
                    that.pageSizes = res.data.pageSizes
                    that.total = parseInt(res.data.total)
                    that.loading = false;
                    if (that.prevPage != that.currentPage) {
                        that.prevPage = that.currentPage;
                        that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                        if (that.scrolltop) {
                            scrollToTop()
                        }
                    }
                    if (that.tableData.length === 0) {
                        that.emptytext = "暂无数据";
                    }
                }

            }).catch(function (e) {
                console.log(e)
            })
        },

        delrow(id) {
            delConfirm(this, id, this.delete);
        },
        delAllBottom() {
            if (!this.selectedItem.length) {
                message.error('请选择要删除的数据');
                return false;
            }
            delConfirm(this, this.selectedItem, this.delete);
        },
        async delete(id) {
            let that = this;
            let params = {
                del: id
            };
            httpPost('m=user&c=company_job&a=del', params).then(function (response) {
                if (response.data.error == 0) {
                    message.success('操作成功', function () {
                        that.$refs.multipleTable.clearSelection();
                        that.getList();
                    });
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function (error) {
                console.log(error);
            })
        },
        // 搜索职位选择
        confirmJobSearch(data) {
            this.search_params.job_class = data.jobId.join(',');
        },
        // 搜索城市选择
        confirmCitySearch(data) {
            this.search_params.city_class = data.cityId.join(',');
        },
        //  查看职位申请记录
        showComJobLogBox: function (e, browse) {
            let _this = this;
            _this.jobid = e.id;
            _this.applyJobBoxTitle = e.name + ' 申请记录';
            _this.sqJobBrowse = browse == 1 ? '1' : '';
            _this.drawerCompanyJobLog = true;
        },
        //  查看职位面试记录
        showComUserIdMsgBox: function (e) {
            let _this = this;
            _this.jobid = e.id;
            _this.interviewBoxTitle = e.name + ' 面试记录';
            _this.drawerCompanyUserIdMsg = true;
        },
        // 预约刷新
        yyrefresh(detail) {
            let date = new Date();
            // this.curr_data = detail;
            this.curr_data.reserve_end = (detail.reserve_end == '不限' ||detail.reserve_end == ''|| detail.reserve_end == undefined)  ? '' : detail.reserve_end;
            this.userinterval = 0;
            var intervalArr = ['60', '120', '180', '240', '300', '360', '420', '480'];
            if (intervalArr.indexOf(detail.reserve_interval) < 0) {
                this.userinterval = detail.reserve_interval
                this.curr_data.reserve_interval = '60'
            }else{
                this.curr_data.reserve_interval = detail.reserve_interval;
            }
            this.curr_data.s_time = detail.s_time == undefined|| detail.s_time == ""  ?'09:00' :detail.s_time;
            this.curr_data.e_time = detail.e_time == undefined|| detail.e_time == "" ?'17:00' :detail.e_time;
            this.curr_data.uid = detail.uid;
            this.curr_data.id = detail.id;
            let that = this;

            if(detail.is_reserve !='1'){
                httpPost('m=user&c=company_job&a=getRefresh', {
                    job_id: this.curr_data.id
                }).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        let refreshStatus = res.data.refreshStatus;
                        if (refreshStatus == 0){
                            that.curr_data.reserve_status = '2';
                        }else{
                            that.curr_data.reserve_status = '2';
                            that.curr_data.s_time = res.data.s_time == undefined|| res.data.s_time == ""  ?'09:00' :res.data.s_time;
                            that.curr_data.e_time = res.data.e_time == undefined|| res.data.e_time == "" ?'17:00' :res.data.e_time;
                            that.curr_data.reserve_interval = res.data.interval ==0?'60':res.data.interval;
                        }
                    } else {
                        message.error(response.data.msg);
                    }
                })
            }else{
                this.curr_data.reserve_status = detail.reserve_status == undefined ?'2' :detail.reserve_status;
            }
            this.drawertz = true
        },
        submitTz: function(){
            var that = this
            if (that.curr_data.reserve_status == '' || that.curr_data.reserve_status == 0 || that.curr_data.reserve_status == undefined) {
                message.error('请选择预约状态');
                return false;
            } else if (that.curr_data.reserve_status == 1) {
                if (that.curr_data.reserve_interval <= 0) {
                    message.error('请选择刷新间隔');
                    return false;
                }
                if (that.curr_data.reserve_interval == 1 && that.userinterval == '') {
                    message.error('请填写自定义刷新间隔');
                    return false;
                }
                if (that.curr_data.s_time.length > 0 && that.curr_data.e_time.length > 0) {
                    var stime = that.curr_data.s_time.split(':');
                    var etime = that.curr_data.e_time.split(':');
                    if (parseInt(stime[0]) > parseInt(etime[0]) || (parseInt(stime[0]) == parseInt(etime[0]) && parseInt(stime[1]) >= parseInt(etime[1]))) {
                        message.error('请合理设置刷新时间段');
                        return false;
                    }
                }
            }
            that.saveLoading= true;
            httpPost('m=user&c=company_job&a=upReserveJob', {
                job_id: that.curr_data.id,
                end_time: that.curr_data.reserve_end,
                interval: that.curr_data.reserve_interval == 1 ? that.userinterval : that.curr_data.reserve_interval,
                status: that.curr_data.reserve_status,
                s_time: that.curr_data.s_time,
                e_time: that.curr_data.e_time,
                uid: that.curr_data.uid
            }).then(function (response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function(){
                        that.getList();
                        that.drawertz = false
                    });
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function() {
                setTimeout(function() {
                    that.saveLoading = false;
                }, 2000);
            });
        },
        handleTimeChange() {
            if (this.search_params.time_type != '' && Array.isArray(this.search_params.times) && this.search_params.times.length) {

                this.isSearchTime = true;
                this.search();
            }
            if (this.isSearchTime && this.search_params.time_type == '' && this.search_params.times == null){

                this.isSearchTime = false;
                this.search();
            }
        },
        getParams:function(params={},search=false){
            var that = this;
            for(let i in params){
                if(typeof that.search_params[i]!='undefined'){
                    that.search_params[i] = params[i];
                }
            }
            console.log(that.search_params);
        },



    },
};
</script>
<style>
    .el-tag+.el-tag{margin-left:10px}
    .button-new-tag{margin-left:10px;height:32px;line-height:30px;padding-top:0;padding-bottom:0}
    .input-new-tag{width:90px;margin-left:10px;vertical-align:bottom}
    .job_set_list{margin-bottom:10px;margin-top:10px;font-size:14px;color:#606266}
    .job_set_pd{padding-left:20px}
    .TableInptline{line-height:35px;padding:0 10px}
    .jobchecked{padding:8px 0 0 10px}
    .cominfocz{padding:15px 0;position:fixed;overflow:hidden;right:0;bottom:0;width:calc(95% - 20px);background:#fff;z-index:222;border-top:1px solid #eee}
    .el-dialog__body{padding:0 20px}
    .jobshcom{font-size:14px;color:#999;padding:10px 0 0 0}
    .wxsettip_small{padding:15px 0 20px 0}
    .waixunHaib{overflow:hidden;position:relative;padding:0 20px;width:100%}
    .waixunHaib ul{
        overflow:hidden;
        position:relative;
        width:calc(100% - 16px);
        display:flex;
        padding:0 8px;
        flex-wrap:wrap;
        align-items:center;
        justify-content:initial;
    }
    .waixunHaib ul::after{
        overflow:hidden;
        position:relative;
        display:block;
        content:"";
        width:calc(19% - 8px);
        display: none;
    }
    .waixunHaib ul li{
        overflow:hidden;
        position:relative;
        width:calc(20% - 20px);
        padding: 0 10px;
        margin-bottom:15px;
    }
    .hb_listbox{overflow:hidden;position:relative}
    .poster_pic{width:100%}
    .poster_pic img{width:100%;border-radius:3px;box-shadow:0 5px 10px 0 rgba(111,116,132,.1)}
    .hb_listbox_name{font-size:15px;width:100%;text-align:center;padding-top:10px}
    .hb_cz{padding-top:10px}
    .tableSeachInptsmall .el-input{width:initial}
    .tableSeacFromer{margin-right:8px}
    .tableSeacFromer .el-input-group__prepend{padding:0;background:0 0}
    .tableSeacFromer .el-select{margin-right:0;width:160px;padding-left:20px}
    .tableSeacFromer .el-input{margin-right:0}
    .shshowall{overflow:hidden;position:relative;height:calc(100% - 50px - 50px - 45px - 25px)}
    .shshow{overflow-y:auto;position:relative;height:100%;min-height:initial}
    .shcz{overflow-y:auto;height:calc(100% - 20px)}
    .moduleElJoball{height:calc(100% - 182px)}
    .moduleElJoball{height:calc(100% - 184px)!important}
    .modulElTableGaijop{height:calc(100% - 134px)!important}
    @media (max-width:1480px){.moduleElJoball{height:calc(100% - 240px)!important}
        .modulElTableGaijop{height:calc(100% - 140px)!important}
    }
    .waixunHaib{overflow:hidden;position:relative;padding:0 20px;width:100%}
    /*.waixunHaib ul{overflow:hidden;position:relative;width:calc(100% - 16px);display:flex;padding:0 8px;flex-wrap:wrap;align-items:center;justify-content:space-between}
    .waixunHaib ul::after{overflow:hidden;position:relative;display:block;content:"";width:calc(19% - 8px)}
    .waixunHaib ul li{overflow:hidden;position:relative;width:calc(19% - 12px);margin-bottom:15px}*/
</style>