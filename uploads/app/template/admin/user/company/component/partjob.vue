<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="moduleSeacFouyr" style="padding: 2px 8px 12px 0;">
                <el-input v-model="search_params.keyword" @keyup.enter.native="search" placeholder="请输入搜索内容" size="small"
                    clearable>
                    <el-select v-model="search_params.type" size="small" slot="prepend" placeholder="公司名称">
                        <el-option label="公司名称" value="1"></el-option>
                        <el-option label="职位名称" value="2"></el-option>
                    </el-select>
                </el-input>

            </div>
            <!--收起部分-->
            <div class="tableSeachInpt tableSeachInptsmall" v-for="(searchitem, searchidx) in searchlist" :key="searchidx"
                :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="search_params[searchidx]" size="small" slot="prepend" :placeholder="searchitem.name"
                    clearable @change="search">
                    <el-option v-for="(item, index) in searchitem.value" :label="item" :key="index"
                        :value="index"></el-option>
                </el-select>
            </div>

            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">查询</el-button>
            </div>
            <div class="tableSeachzk" :class="{ 'searchbutnKai': seachbutn }" style="margin-bottom: 8px;">
                <el-button type="info" class="zhankai" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>展开<i class="el-icon-arrow-down el-icon--right"></i></el-button>
                <el-button type="info" class="shouqi" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>合并<i class="el-icon-arrow-up el-icon--right"></i></el-button>
            </div>
        </div>
        <div class="admin_datatip"><i class="el-icon-document"></i> 数据统计：
            <span class="admin_datatip_n">共：{{ partAllNum }} 条 </span>
            <span class="admin_datatip_n">未审核：{{ status1Num }} 条 </span>
            <span class="admin_datatip_n">未通过：{{ status2Num }} 条</span>
            <span class="admin_datatip_n">已过期：{{ status3Num }} 条</span>
            <span class="admin_datatip_n">搜索结果：{{ total }} 条 </span>
        </div>
        <div class="moduleElTable moduleElMediDFur" :class="{ 'modulElTableGaiPart': tableHig }"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe @sort-change='sortChange'
                @mousedown.native="mouseDownHandler"
                @mouseup.native="mouseUpHandler"
                @mousemove.native="mouseMoveHandler"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                :default-sort="{ prop: 'id', order: 'descending' }" @selection-change="handleSelectionChange"
                ref="multipleTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="职位ID" width="120" sortable="custom"></el-table-column>
                <el-table-column label="职位/企业" min-width="180" show-overflow-tooltip>
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <div class=" ">
                                <el-link type="primary" target="_blank" :href="props.row.webjob_url">{{ props.row.name
                                }}
                                </el-link>
                            </div>
                            <div class=" ">
                                <el-link target="_blank" :href="props.row.webcom_url">{{ props.row.com_name }}</el-link>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="工作类型/招人数" width="130">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <span class=" ">{{ props.row.type_n }}</span>
                            <span class=" ">招聘{{ props.row.number }}人</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label=" 薪水/类型" width="130">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <span class=" ">{{ props.row.salary }}{{ props.row.salary_type_n }}</span>
                            <span class=" ">{{ props.row.billing_cycle_n }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="logintime" label="更新/结束时间 " width="150">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <span>{{ props.row.lastupdate_n_n }}</span>
                            <span class="gsd">{{ props.row.end_n }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="logintime" label="报名人数 " width="150">
                    <template slot-scope="props">
                        <!--<a href="index.php?m=admin_comlog&c=partapply&jobid={yun:}$v.id{/yun}" class="admin_cz_sc">-->
                        <!--{yun:}$v.applynum{/yun}人<div class="admin_mb5">查看</div>-->
                        <!--</a>-->
                        <div class="moduleProps" v-if="props.row.applynum > 0">
                            <span>{{ props.row.applynum }}人</span>
                            <div class="jobtj">
                                <el-link @click="applylog(props.row)">查看<i class="el-icon-view el-icon--right"></i>
                                </el-link>
                            </div>
                        </div>
                        <span v-else>暂无报名</span>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label=" 职位推广" width="130">
                    <template slot-scope="props">
                        <el-tooltip class="item" effect="dark" placement="top-start">
                            <div slot="content">
                                <span style="line-height: 20px;">职位推荐剩余{{ props.row.isrec ? props.row.rec_day : 0 }}天</span>
                            </div>
                            <div class="job_tg_bth">
                                <el-switch v-model="props.row.isrec" @change="tgchange($event, props.row, 2)"
                                    inactive-text="推荐"></el-switch>
                            </div>
                        </el-tooltip>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label=" 招聘状态" width="130">
                    <template slot-scope="props">
                        <el-switch v-model="props.row.iszp" @change="zpstatuschange($event, props.row)"></el-switch>
                        {{ props.row.status != 1 ? '招聘中' : '已下架' }}
                    </template>
                </el-table-column>
                <el-table-column prop="zt" label="状态">
                    <template slot-scope="props">
                        <div class="admin_state">
                            <span v-if="(props.row.edate > 0 && props.row.edate < nowtime) || props.row.state == 2"
                                class="admin_state5">已过期</span>
                            <div v-else-if="props.row.r_status == '2'">
                                <span class="admin_state3">已锁定</span>
                                <div style="display:inline-block">
                                    <el-popover trigger="hover" placement="right" v-if="props.row.lock_info">
                                        <p>{{ props.row.lock_info }}</p>
                                        <div slot="reference" class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div>
                                    </el-popover>
                                </div>
                            </div>
                            <span v-else-if="props.row.state == 1" class="admin_state1">已审核</span>
                            <span v-else-if="props.row.state == 0" class="admin_state4">待审核</span>
                            <div v-else-if="props.row.state == 3">
                                <span class="admin_state2">未通过</span>
                                <div style="display:inline-block">
                                    <el-popover trigger="hover" placement="right" v-if="props.row.lock_info">
                                        <p>{{ props.row.statusbody }}</p>
                                        <div slot="reference" class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div>
                                    </el-popover>
                                </div>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="操作" width="200" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="small " plain @click="jobAudit(scope.row)">审核</el-button>
                            <el-button size="small " plain @click="edit(scope.row)">修改</el-button>
                            <el-button type="danger  " size=" " @click="delrow(scope.row.id)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">全选</el-checkbox>
                <el-button @click="delAllBottom" size="mini">批量删除</el-button>
                <el-button @click="multipleStatus" size="mini">审核</el-button>
                <el-button @click="multipleyq" size="mini">延期</el-button>
                <el-button @click="refresh" size="mini">刷新</el-button>
                <el-button @click="multitg" size="mini">推荐</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page="currentPage" :page-sizes="pageSizes" :page-size="perPage"
                    layout="total, sizes, prev, pager, next, jumper" :total="total" :pager-count="pagerCount">
                </el-pagination>
            </div>
        </div>
        <!--兼职报名记录弹窗-->
        <div class="modluDrawer" v-if="curr_job">
            <el-drawer title="匹配简历" :visible.sync="drawerapplylog" :modal-append-to-body="false" append-to-body
                :show-close="true" :with-header="true" size="95%">
                <comlog_partapply style="margin-left: 10px;" ref="partapplylog" :job="curr_job"></comlog_partapply>
            </el-drawer>
        </div>
        <!--职位推广弹窗-->
        <div class="modluDrawer">
            <el-dialog :title="jobtgtit" :visible.sync="jobtgdrawer" :with-header="true" append-to-body :show-close="true"
                width="400px">
                <div class="wxsettip_small">推荐天数</div>
                <el-input type="number" placeholder="请输入天数" v-model="jobtgdays">
                    <template slot="append">天</template>
                </el-input>
                <div class="wxsettip_small" v-if="jobtgetime != ''">当前结束日期</div>
                <el-input v-if="jobtgetime != ''" v-model="jobtgetime" disabled>
                </el-input>
                <div style="margin-top:10px;">
                    <i class="el-icon-warning"></i>
                    如需取消推荐职位请单击
                    <el-checkbox v-model="qxtgchecked" true-label="1" false-label="0"></el-checkbox>
                    <span>点击确认即可</span>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="jobtgdrawer = false">取 消</el-button>
                    <el-button type="primary" @click="jobTgSubmit" :loading="tg_loading">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--批量延期弹窗-->
        <div class="modluDrawer">
            <el-dialog title="批量延期" :visible.sync="yqdrawer" :with-header="true" append-to-body :show-close="true"
                width="400px">
                <div class="wxsettip_small">延长天数</div>
                <el-input type="number" placeholder="请输入天数" v-model="yqdays">
                    <template slot="append">天</template>
                </el-input>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="yqdrawer = false">取 消</el-button>
                    <el-button type="primary" @click="yqSubmit" :loading="yq_loading">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--批量审核职位-->
        <div class="modluDrawer">
            <el-dialog title="职位批量审核" width="300px" :visible.sync="drawerauditmultiple" append-to-body
                :modal-append-to-body="false">
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
                    <el-button type="primary" @click="multipleStatusSave" :loading="status_loading">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--职位详情审核 ---------------------------------------------------------------------->
        <el-drawer title="兼职审核" :visible.sync="jobdrawersh" :modal-append-to-body="false" size="80%">
            <div class="shbox" v-if="auditInfo">
                <div class="shinfo">
                    <div class="shcomname">{{ auditInfo.name }}</div>
                    <div class="jobshcom">{{ auditInfo.com_name }}
                        <!--<el-tag type="danger" size="mini">{{searchlist['rating'].value[auditInfo.rating]}}</el-tag>-->
                    </div>
                    <div class="sh_zwsz_add">
                        联系人：{{ auditInfo.linkman }} <span class="shcomtel_n"
                            v-if="auditInfo.linktel">联系电话：{{ auditInfo.linktel }} </span>
                        <span v-if="auditInfo.crm_salesman">业务员：{{ auditInfo.crm_salesman }}</span>
                    </div>
                    <!--<div class="shcomtel">注册时间：{{auditInfo.reg_date_n}}-->
                    <!--<span class="shcomtel_n">最近登录时间：{{auditInfo.login_date_n}} </span>-->
                    <!--<span v-if="auditInfo.add_ip">IP：{{auditInfo.add_ip}}</span>-->
                    <!--<span v-if="auditInfo.add_ip" class="shcomtel_n">IP归属地：{{ip_address}}</span>-->
                    <!--</div>-->
                    <div class="shshowall">
                        <div class="shshow">
                            <div class="shshow_tit"><i class="el-icon-document"></i> 基本要求</div>
                            <div class="shshow_p">
                                <div class="">工作要求：{{ auditInfo.type_n }}</div>
                                <div class="" v-if="auditInfo.number">招聘人数：{{ auditInfo.number }}</div>
                                <div class="" v-if="auditInfo.sex_n">性别要求：{{ auditInfo.sex_n }}</div>
                                <div class="" v-if="auditInfo.billing_cycle_n">结算周期：{{ auditInfo.billing_cycle_n }}</div>
                                <div class="" v-if="auditInfo.sdate_n">兼职有效期：{{ auditInfo.sdate_n }}</div>
                                <div class="" v-if="auditInfo.address">工作地址：{{ auditInfo.address }}</div>
                            </div>
                            <div style="display: flex; flex-direction: column;">
                                <span>工作时间</span>
                                <table class="tjob_timetable" style="float:left; margin-top: 5px;">
                                    <tbody>
                                        <tr>
                                            <th>&nbsp;</th>
                                            <th>周一</th>
                                            <th>周二</th>
                                            <th>周三</th>
                                            <th>周四</th>
                                            <th>周五</th>
                                            <th>周六</th>
                                            <th>周日</th>
                                        </tr>
                                        <tr>
                                            <th>上午</th>
                                            <td v-for="val in cacheData.part_morning" :key="val">
                                                <el-checkbox
                                                    :checked="auditInfo.worktime_n.findIndex(item => item === val) !== -1"
                                                    disabled></el-checkbox>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th>下午</th>
                                            <td v-for="val in cacheData.part_noon" :key="val">
                                                <el-checkbox
                                                    :checked="auditInfo.worktime_n.findIndex(item => item === val) !== -1"
                                                    disabled></el-checkbox>
                                            </td>
                                        </tr>
                                        <tr>
                                            <th>晚上</th>
                                            <td v-for="val in cacheData.part_afternoon" :key="val">
                                                <el-checkbox
                                                    :checked="auditInfo.worktime_n.findIndex(item => item === val) !== -1"
                                                    disabled></el-checkbox>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>
                            <div class="shshow_tit"><i class="el-icon-office-building"></i> 工作内容</div>
                            <div class="shshow_p">
                                <div class="" v-html="auditInfo.content"></div>
                            </div>
                        </div>
                        <div v-if="auditInfo.c_status == 2">
                            <div class="wxsettip_small ">锁定状态</div>
                            <template>
                                <el-radio v-model="auditInfo.c_status" label="1">正常</el-radio>
                                <el-radio v-model="auditInfo.c_status" label="2">锁定</el-radio>
                            </template>
                            <div class="wxsettip_small ">锁定原因</div>
                            <el-input type="textarea" disabled :rows="2" placeholder="锁定原因" :value="auditInfo.statusbody">
                            </el-input>
                        </div>
                        <div v-else class="shcz">
                            <div class="wxsettip_small ">职位审核</div>
                            <template>
                                <el-radio v-model="auditInfo.state" label="1">通过</el-radio>
                                <el-radio v-model="auditInfo.state" label="3">未通过</el-radio>
                            </template>
                            <div class="wxsettip_small " v-if="auditInfo.r_status == 0">企业审核</div>
                            <template>
                                <el-checkbox v-if="auditInfo.r_status == 0" :value="true" disabled>同步审核</el-checkbox>
                            </template>
                            <div class="wxsettip_small ">审核状态说明</div>
                            <el-input type="textarea" :rows="2" placeholder="请输入审核状态说明" v-model="auditInfo.statusbody">
                            </el-input>
                            <div class=" shczbth">
                                <el-button type="primary" :loading="audit_load" @click="audit(1)">提 交</el-button>
                            </div>
                            <div class=" shczbth" v-if="sh_num > 0">
                                <el-button type="primary" :loading="audit_load" @click="audit(2)"
                                    plain>提交，并审核下一个</el-button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </el-drawer>
        <!--修改职位提示-->
        <el-drawer title="修改职位" :visible.sync="drawerEditJob" append-to-body :wrapper-closable="false" size="60%">
            <div class="uploadTable" style="padding:0px 20px;" v-if="curr_job">
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
                                <div class="TableTite">职位名称</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="curr_job.name" placeholder="请输入职位名称"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">工作类型</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-select v-model="curr_job.type" placeholder="请选择工作类型">
                                        <el-option v-for="item in cacheData.partdata.part_type" :key="item"
                                            :label="cacheData.partclass_name[item]" :value="item">
                                        </el-option>
                                    </el-select>
                                    <!--<el-cascader style="width: 480px;" v-model="sel_jobtype" :options="job_types" filterable-->
                                    <!--clearable></el-cascader>-->
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">招聘人数</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="curr_job.number" placeholder="请输入招聘人数"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">兼职时间</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <table class="tjob_timetable" style="float:left">
                                        <tbody>
                                            <tr>
                                                <th>&nbsp;</th>
                                                <th>周一</th>
                                                <th>周二</th>
                                                <th>周三</th>
                                                <th>周四</th>
                                                <th>周五</th>
                                                <th>周六</th>
                                                <th>周日</th>
                                            </tr>
                                            <tr>
                                                <th>上午</th>
                                                <td v-for="val in cacheData.part_morning" :key="val">
                                                    <el-checkbox
                                                        :checked="checkedworktime.findIndex(item => item === val) !== -1"
                                                        @change="worktimeChange(val)"></el-checkbox>
                                                </td>
                                            </tr>
                                            <tr>
                                                <th>下午</th>
                                                <td v-for="val in cacheData.part_noon" :key="val">
                                                    <el-checkbox
                                                        :checked="checkedworktime.findIndex(item => item === val) !== -1"
                                                        @change="worktimeChange(val)"></el-checkbox>
                                                </td>
                                            </tr>
                                            <tr>
                                                <th>晚上</th>
                                                <td v-for="val in cacheData.part_afternoon" :key="val">
                                                    <el-checkbox
                                                        :checked="checkedworktime.findIndex(item => item === val) !== -1"
                                                        @change="worktimeChange(val)"></el-checkbox>
                                                </td>
                                            </tr>
                                            <tr style="border-bottom: none;">
                                                <td colspan="8">
                                                    <el-checkbox style="width:110px;margin-left:0" size="small" border
                                                        :indeterminate="isIndeterminate" v-model="worktimeCheckAll"
                                                        @change="handleColCheckAllChange">全选</el-checkbox>
                                                </td>
                                            </tr>
                                        </tbody>
                                    </table>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">兼职有效期</div>
                            </td>
                            <td>
                                <div style="display: flex;">
                                    <div class="TableInpt">
                                        <el-date-picker readonly v-model="curr_job.sdate_n" value-format="yyyy-MM-dd"
                                            type="date" placeholder="选择日期" :picker-options="pickerOptions">
                                        </el-date-picker>
                                    </div>
                                    <div class="TableInptline">-</div>
                                    <div class="TableInpt" v-if="!iscq">
                                        <el-date-picker v-model="curr_job.edate_n" value-format="yyyy-MM-dd" type="date"
                                            placeholder="选择日期" :picker-options="pickerOptions">
                                        </el-date-picker>
                                    </div>
                                    <el-checkbox style="margin-left:10px;" v-model="iscq" label="长期招聘" size="medium"
                                        border></el-checkbox>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">薪水</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="curr_job.salary" placeholder="请输入薪水"></el-input>
                                    <el-select style="margin-left:10px;" v-model="curr_job.salary_type"
                                        placeholder="请选择薪资类型">
                                        <el-option v-for="item in cacheData.partdata.part_salary_type" :key="item"
                                            :label="cacheData.partclass_name[item]" :value="item">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">薪资结算</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-select v-model="curr_job.billing_cycle" placeholder="请选择薪资结算">
                                        <el-option v-for="item in cacheData.partdata.part_billing_cycle" :key="item"
                                            :label="cacheData.partclass_name[item]" :value="item">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">工作内容</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <div id="partjobeditor—wrapper" style="border: 1px solid #ccc;">
                                        <div id="partjobtoolbar-container"><!-- 工具栏 --></div>
                                        <div id="partjobeditor-container" style="height: 300px;"><!-- 编辑器 --></div>
                                    </div>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">性别</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-select v-model="curr_job.sex" placeholder="请选择性别">
                                        <el-option v-for="(item, index) in cacheData.part_sex" :key="index" :label="item"
                                            :value="index">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">工作地点</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-cascader v-model="sel_city" :options="cacheData.citys" @change="citychange"
                                        filterable collapse-tags clearable></el-cascader>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">详细地址</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-input v-model="curr_job.address" placeholder="请输入详细地址"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr v-if="mapurl">
                            <td>
                                <div class="TableTite"> </div>
                            </td>
                            <td>
                                <div id="conrtainer" style="width:100%;height:300px; position:relative; z-index:1"></div>
                            </td>
                        </tr>
                        <tr v-if="curr_job.id">
                            <td>
                                <div class="TableTite">审核状态</div>
                            </td>
                            <td>
                                <div class="job_set_list">
                                    <font v-if="curr_job.state == 1" color="blue">已审核</font>
                                    <font v-else-if="curr_job.state == 3" color="red">未通过</font>
                                    <font v-else color="red">未审核</font>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="setBasicButn" style="border: none; height: 80px;">
                <el-button type="primary" size="medium" :loading="save_load" @click="jobsave">提交</el-button>
            </div>
        </el-drawer>
    </div>
</template>
<script>
let jobeditor = null, jobtoolbar = null,editorInterval = null;
const { createEditor, createToolbar } = window.wangEditor;
module.exports = {
    props: {
        state: { type: String, default: '' }
    },
    data: function () {
        return {
            mouseFlag: false,
            mouseOffset: 0,
            loading: false,
            pagerCount: 5,
            emptytext: '暂无数据',
            searchlist: null,
            search_params: {
                type: '1',
                keyword: '',
                state: this.state,
                status: '',
                lastupdate: '',
                edate: '',
                billing_cycle: ''
            },
            checkedAll: false,
            selectedItem: [],
            tableData: [],
            currentPage: 1,
            perPage: 0,
            pageSizes: [],
            total: 0,
            sort_type: '',
            sort_col: '',
            partAllNum: 0,
            status1Num: 0,
            status2Num: 0,
            status3Num: 0,
            seachbutn: true,
            tableHig: true,
            cacheData: [],
            nowtime: 0,
            curr_job: null,
            drawerapplylog: false,
            // 职位推荐
            jobtgtit: '',
            jobtgdays: '',
            jobtgetime: '',
            qxtgchecked: '0',
            jobtgdrawer: false,
            // 批量延期
            yqdays: '',
            yqdrawer: false,
            // 批量审核
            drawerauditmultiple: false,
            multiStatus: '',
            multiStatusBody: '',
            // 职位审核
            jobdrawersh: false,
            auditInfo: null,
            audit_load: false,
            sh_num: 0,
            //修改职位
            drawerEditJob: false,
            jobCompany: null,
            showJob: false,
            isIndeterminate: true,
            checkedworktime: [],
            worktimeCheckAll: false,
            pickerOptions: {//el-date-picker 时间限定
                disabledDate(time) {
                    // 今天及今天之前的日期
                    // return time.getTime() > Date.now();
                    // 今天及今天之后的日期
                    return time.getTime() < Date.now() - 8.64e7;
                }
            },
            iscq: false,// 是否长期招聘
            sel_city: [],
            mapkey: '',
            mapurl: '',
            mapsecret: '',
            today: '',
            save_load: false,
            islook: false,
            tg_loading: false,
            yq_loading: false,
            status_loading: false,
            prevPage: 0
        }
    },
    created() {
        this.getList();
    },
    mounted() {
        var that = this
        setTimeout(function () {
            that.getTjNum();
            that.getCacheFun();
        }, 200)
    },
    beforeDestroy() {
        jobeditor = null; 
        jobtoolbar = null;
        editorInterval = null;
    },
    components: {
        'comlog_partapply': httpVueLoader('./comlog_partapply.vue'),
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



        getParams: function (params = {}) {
            var that = this;
            for (let i in params) {
                if (i != 'page' && typeof that.search_params[i] != 'undefined') {
                    that.search_params[i] = params[i];
                }
            }
        },
        writeJs: function (url, secret) {
            return new Promise((resolve, reject) => {
                // 如果已加载直接返回
                if (typeof window.AMap !== 'undefined') {
                    resolve(window.AMap);
                    return true;
                }
                // 地图异步加载回调处理
                window.onAMapCallback = function () {
                    resolve(AMap);
                };
                // 设置安全密钥
                window._AMapSecurityConfig = {
                    securityJsCode: secret,
                }
                // 插入script脚本
                let scriptNode = document.createElement('script');
                scriptNode.setAttribute('type', 'text/javascript');
                scriptNode.setAttribute('src', url);
                document.body.appendChild(scriptNode);
            });
        },
        citychange: function (data) {
            this.sel_city = data
        },
        jobsave: function () {
            var that = this
            that.checkedworktime
            if (that.curr_job.name == '') {
                message.error('职位名称不能为空')
                return false;
            }
            if (that.curr_job.type == "") {
                message.error("请选择兼职类型"); return false;
            }
            if (that.curr_job.number < 1) {
                message.error("请输入招聘人数"); return false;
            }
            if (that.checkedworktime.length == 0) {
                message.error("请选择兼职时间"); return false;
            }
            if (that.curr_job.sdate_n == "") {
                message.error("请选择开始日期"); return false;
            } else {
                that.curr_job.sdate = that.curr_job.sdate_n
            }
            if (!that.iscq) {
                if (that.edate_n == "") {
                    message.error("请选择结束日期"); return false;
                }
                if (toDate(that.curr_job.edate_n).getTime() < toDate(that.curr_job.sdate_n).getTime() || toDate(that.curr_job.edate_n).getTime() < toDate(that.today).getTime()) {
                    message.error("请正确选择工作日期"); return false;
                }
                that.curr_job.edate = that.curr_job.edate_n
            }
            if (that.curr_job.salary == "" || that.curr_job.salary < 1) {
                message.error("请输入薪资水平"); return false;
            }
            if (that.curr_job.salary_type == "") {
                message.error("请选择薪水类型"); return false;
            }
            if (that.curr_job.billing_cycle == "") {
                message.error("请选择结算周期"); return false;
            }
            // 去除html标签后判断内容是否为空
            var regex = /(<([^>]+)>)/ig
            var content = jobeditor.getHtml().replace(regex, "")
            if (content == "") {
                message.error('请输入兼职内容')
                return false;
            } else {
                that.curr_job.content = jobeditor.getHtml();
            }

            if (that.cacheData.city_type.length) {
                if (that.sel_city[0] == '') {
                    message.error('请选择所在地区')
                    return false;
                }
            } else {
                if (that.sel_city[1] == '') {
                    message.error('请选择工作地点')
                    return false;
                }
            }
            if (that.sel_city[0] > 0) {
                that.curr_job.provinceid = that.sel_city[0]
            }
            that.curr_job.cityid = that.sel_city[1] ? that.sel_city[1] : 0
            that.curr_job.three_cityid = that.sel_city[2] ? that.sel_city[2] : 0
            if (that.curr_job.address == "") {
                message.error("请输入详细地址"); return false;
            }
            if (that.curr_job.x == "" || that.curr_job.y == "") {
                message.error("请选择地图"); return false;
            }
            if (that.curr_job.linkman == "") {
                message.error("请输入联系人"); return false;
            }
            if (that.curr_job.linktel == "") {
                message.error("请输入联系手机"); return false;
            }
            if (isjsMobile(that.curr_job.linktel) == false) {
                message.error('请正确填写联系手机'); return false;
            }
            that.curr_job.update = 1;
            that.save_load = true;
            httpPost('m=user&c=partjob&a=show', that.curr_job).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.drawerEditJob = false
                        that.getList()
                    })
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            }).finally(function () {
                setTimeout(function () {
                    that.save_load = false;
                }, 2000);
            });
        },
        // 职位修改兼职时间全选checkbox
        handleColCheckAllChange(val) {
            var that = this
            if (val) {
                that.checkedworktime = []
                that.cacheData.part_morning.forEach(item => {
                    that.checkedworktime.push(item);
                });
                that.cacheData.part_noon.forEach(item => {
                    that.checkedworktime.push(item);
                });
                that.cacheData.part_afternoon.forEach(item => {
                    that.checkedworktime.push(item);
                });
            } else {
                that.checkedworktime = []
            }
            this.isIndeterminate = false;
        },
        // 职位修改兼职时间checkbox
        worktimeChange(value) {
            var idx = this.checkedworktime.findIndex(item => item === value)
            if (idx !== -1) {
                this.checkedworktime.splice(idx, 1)
            } else {
                this.checkedworktime.push(value)
            }
            this.checkedworktime
            let totallen = this.cacheData.part_morning.length + this.cacheData.part_noon.length + this.cacheData.part_afternoon.length
            this.worktimeCheckAll = this.checkedworktime.length === totallen;
            this.isIndeterminate = this.checkedworktime.length > 0 && this.checkedworktime.length < totallen;
        },
        // 职位修改
        edit: function (row) {
            var that = this
            httpPost('m=user&c=partjob&a=show', { id: row.id }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.curr_job = res.data.show
                    that.today = res.data.today
                    that.checkedworktime = that.curr_job.worktime_n
                    that.jobCompany = res.data.company
                    that.showJob = true
                    if (that.curr_job.edate == 0) {
                        that.curr_job.edate_n = ''
                        that.iscq = true
                    } else {
                        that.iscq = false
                    }
                    that.sel_city = []
                    if (that.curr_job.provinceid > 0) {
                        that.sel_city.push(that.curr_job.provinceid)
                    }
                    if (that.curr_job.cityid > 0) {
                        that.sel_city.push(that.curr_job.cityid)
                    }
                    if (that.curr_job.three_cityid > 0) {
                        that.sel_city.push(that.curr_job.three_cityid)
                    }
                    that.initEditor(that.curr_job.content);
                    setTimeout(function () {
                        
                        var map_url = that.mapurl + "&callback=onAMapCallback";
                        that.writeJs(map_url, that.mapsecret).then(value => {
                            that.openMap();
                        });
                    }, 300)
                    that.drawerEditJob = true
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        getMap: function () {
            var map = new AMap.Map('conrtainer', {
                zoom: 15,
                center: [this.curr_job.x, this.curr_job.y]
            });
            var marker = new AMap.Marker({
                position: new AMap.LngLat(this.curr_job.x, this.curr_job.y)
            });
            map.add(marker);
            return map;
        },
        openMap: function () {
            var that = this;
            var data = get_map_config();
            if (data && data.indexOf('map_x') > -1) {
                var config = eval('(' + data + ')');
                var rating, map_control_type, map_control_anchor;
                if (!that.curr_job.x && !that.curr_job.y) {
                    that.curr_job.x = config.map_x;
                    that.curr_job.y = config.map_y;
                }
                var map = that.getMap();
                map.on("click", function (e) {
                    var lngLat = e.lnglat;
                    that.curr_job.x = lngLat.lng
                    that.curr_job.y = lngLat.lat
                    map.clearMap();
                    var marker = new AMap.Marker({
                        position: new AMap.LngLat(lngLat.lng, lngLat.lat)
                    });
                    map.add(marker);
                });
            }
        },
        initEditor: function (content=null) {
            var that = this
            clearInterval(editorInterval);
            editorInterval = setInterval(()=>{
                if (jobeditor !== null){
                    clearInterval(editorInterval);
                    if(content!==null){
                        jobeditor.setHtml(content);
                    }
                }else{
                    let editorConfig = {
                        MENU_CONF: {
                            uploadImage: {
                                server: baseUrl + 'm=index&c=uploadfile',
                                fieldName: 'file'
                            }
                        }
                    };
                    if (!jobeditor) {
                        jobeditor = createEditor({
                            selector: '#partjobeditor-container',
                            html: '',
                            config: editorConfig,
                            mode: 'simple'
                        });
                    }
                    if (!jobtoolbar) {
                        jobtoolbar = createToolbar({
                            editor: jobeditor,
                            selector: '#partjobtoolbar-container',
                            config: {
                                excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                            },
                            mode: 'simple'
                        });
                    }
                }
            },300);
            
        },
        audit: function (atype) {
            var that = this
            if (!that.auditInfo.state) {
                message.error("请选择审核状态")
                return false;
            }
            var params = {
                single: 1,
                status: that.auditInfo.state,
                pid: that.auditInfo.id,
                uid: that.auditInfo.uid,
                statusbody: that.auditInfo.statusbody,
                atype: atype
            };
            if (that.auditInfo.c_status == 2) {
                message.error("锁定操作异常")
                return false;
            } else {
                params.lock_status = 1;
            }
            if (that.auditInfo.r_status == '0') {
                var url = 'm=user&c=partjob&a=tbStatus';
            } else {
                var url = 'm=user&c=partjob&a=status';
            }
            that.audit_load = true;
            httpPost(url, params).then(function (result) {
                that.audit_load = false;
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        if (res.data == undefined) {
                            that.jobdrawersh = false
                        } else if (res.data.job) {
                            that.jobAudit(res.data.job)
                        }
                        that.getList()
                    })
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 职位审核弹窗
        jobAudit: function (row) {
            var that = this
            httpPost('m=user&c=partjob&a=partAudit', { id: row.id }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.auditInfo = res.data.info
                    that.auditInfo.state = that.auditInfo.state == 3 ? '3' : '1'
                    that.sh_num = res.data.snum
                    that.jobdrawersh = true
                } else {
                    message.error('获取职位审核信息失败')
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 批量推荐
        multitg: function () {
            this.jobtgetime = ''
            if (this.selectedItem.length == 0) {
                message.error('请选择要操作的数据项')
                return false
            }
            this.jobtgtit = '职位批量推荐'
            this.tgjid = this.selectedItem.join(',')
            this.jobtgdrawer = true
        },
        // 批量延期
        multipleyq() {
            var that = this
            if (!that.selectedItem.length) {
                message.error('请选择要操作的数据项')
                return false;
            }
            that.yqdrawer = true
            that.yqdays = ''
        },
        // 批量延期保存
        yqSubmit() {
            var that = this
            if (!that.selectedItem.length) {
                message.error('请选择要操作的数据项')
                return false;
            }
            that.yq_loading = true;
            httpPost('m=user&c=partjob&a=ctime', {
                jobid: that.selectedItem.join(','),
                days: that.yqdays
            }).then(function (result) {
                that.yq_loading = false;
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.yqdrawer = false
                        that.getList()
                    })
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 批量审核
        multipleStatus() {
            var that = this
            if (!that.selectedItem.length) {
                message.error('请选择要操作的数据项')
                return false;
            }
            that.drawerauditmultiple = true
        },
        // 批量审核保存
        multipleStatusSave() {
            var that = this
            if (!that.selectedItem.length) {
                message.error('请选择要操作的数据项')
                return false;
            }
            that.status_loading = true;
            httpPost('m=user&c=partjob&a=status', {
                pid: that.selectedItem.join(','),
                status: that.multiStatus,
                statusbody: that.multiStatusBody
            }).then(function (result) {
                that.status_loading = false;
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
            httpPost('m=user&c=partjob&a=refresh', {
                ids: this.selectedItem.join(',')
            }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    message.success('刷新成功', function () {
                        that.getList()
                    })
                } else {
                    message.error('刷新失败')
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 职位推荐设置
        tgchange: function (val, data) {
            this.curr_job = data
            this.tgjid = data.id
            this.curr_job.isrec = !this.curr_job.isrec// 防止switch状态直接改变
            this.jobtgetime = data.rec_time_n != undefined ? data.rec_time_n : ''
            this.jobtgtit = '职位推荐'
            this.jobtgdrawer = true
        },
        // 职位推广提交
        jobTgSubmit: function () {
            var that = this
            var url = 'm=user&c=partjob&a=recommend'
            if (that.qxtgchecked == 0 && that.jobtgdays == '') {
                message.error('推荐天数不能为空')
                return false
            }
            var params = {
                pid: that.tgjid,
                days: that.jobtgdays,
                s: that.qxtgchecked
            }
            that.tg_loading = true;
            httpPost(url, params).then(function (result) {
                that.tg_loading = false;
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
            that.curr_job.iszp = !that.curr_job.iszp// 提交请求前禁止switch状态改变
            httpPost('m=user&c=partjob&a=checkstate', {
                id: that.curr_job.id,
                state: val ? 2 : 1
            }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.curr_job.iszp = !that.curr_job.iszp// 操作成功改变switch 选中状态
                    that.getList()
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 兼职申请记录
        applylog: function (row) {
            var that = this
            this.curr_job = row
            this.drawerapplylog = true
            this.$nextTick(function () {
                that.$refs.partapplylog.getList()
            })
        },
        // 获取职位数量统计
        getTjNum: function () {
            var that = this;
            httpPost('m=user&c=partjob&a=partNum', {}, { hideloading: true }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {

                    that.partAllNum = res.data.partAllNum ? res.data.partAllNum : 0
                    that.status1Num = res.data.partStatusNum1 ? res.data.partStatusNum1 : 0
                    that.status2Num = res.data.partStatusNum2 ? res.data.partStatusNum2 : 0
                    that.status3Num = res.data.partStatusNum3 ? res.data.partStatusNum3 : 0
                }
            }).catch(function (e) {
                console.log(e)
            })
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
            scrollToTop()
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
            httpPost('m=user&c=partjob&a=getCacheData', {}, { hideloading: true }).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.cacheData = res.data.cache;
                    that.mapkey = res.data.mapkey;
                    that.mapurl = res.data.mapurl;
                    that.mapsecret = res.data.mapsecret;
                    that.nowtime = parseInt(new Date().getTime() / 1000);
                    that.searchlist = res.data.search_list
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
            if (that.search_params.state) {
                params.state = that.search_params.state
            }
            if (that.search_params.status) {
                params.status = that.search_params.status
            }
            if (that.search_params.lastupdate) {
                params.lastupdate = that.search_params.lastupdate
            }
            if (that.search_params.edate) {
                params.edate = that.search_params.edate
            }
            if (that.search_params.billing_cycle) {
                params.billing_cycle = that.search_params.billing_cycle
            }
            if (that.sort_type && that.sort_col) {
                params.order = that.sort_type
                params.t = that.sort_col
            }
            that.loading = true;
            that.emptytext = "数据加载中";
            httpPost('m=user&c=partjob&a=index', params, { hideloading: true }).then(function (result) {
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
                        scrollToTop()
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
            httpPost('m=user&c=partjob&a=del', params).then(function (response) {
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
    },
};
function get_map_config(){
    var config="";
    var weburl = localStorage.getItem("sy_weburl");
    $.ajax( {
        async : false,
        type : "post",
        url : weburl + '/index.php?m=ajax&c=mapconfig',
        data : {id:""},
        success : function(set) {
            config=set;
        }
    });
    return config;
}
</script>
<style scoped>
.tjob_timetable {
    width: 360px;
    background: #ddd;
}

.tjob_timetable th,
.tjob_timetable td {
    background: #fff;
    font-weight: normal;
    padding: 5px;
    text-align: center;
    font-size: 12px;
}

.tjob_timetable th {
    background: #f8f8f8
}

.tjob_timetable tr th {
    height: 27px;
    width: 45px;
    min-width: 45px;
    text-align: center;
}

.tableSeacFromer {
    margin-right: 8px;
}

.tableSeacFromer .el-input-group__prepend {
    padding: 0;
    background: none;
}

.tableSeacFromer .el-select {
    margin-right: 0;
    width: 160px;
}

.tableSeacFromer .el-input {
    margin-right: 0;
}

.moduleSeacFouyr {
    overflow: hidden;
    position: relative;
    padding-right: 8px;
    padding-bottom: 12px;
}

.moduleSeacFouyr .el-select {
    overflow: hidden;
    position: relative;
    width: 120px;
}

.moduleElMediDFur {
    height: calc(100% - 140px);
}

@media (max-width: 1480px) {

    .moduleElMediDFur {
        height: calc(100% - 180px) !important;
    }

    .modulElTableGaiPart {
        height: calc(100% - 130px) !important;
    }
}</style>