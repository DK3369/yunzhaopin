<template>
    <div class="moduleElHight">

        <div class="moduleSeachbig" v-if="!simple">
            <div class="tableSeachInpt tableSeachInptsmall tableSeacFromer" style="padding: 2px 0;">
                <el-input v-model="search_params.keyword" @keyup.enter="search" :placeholder="lc('admin_00340')" size="small"
                    clearable>
                    <template #prepend><el-select v-model="search_params.type" size="small" :placeholder="lc('admin_user_00140')">
                        <el-option :label="lc('admin_user_company_00344')" value="1"></el-option>
                        <el-option :label="lc('admin_user_company_00370')" value="3"></el-option>
                        <el-option label="IP" value="4"></el-option>
                    </el-select></template>
                </el-input>
            </div>
            <!-- Collapsed section -->
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="search_params.time_type" size="small" :placeholder="lc('admin_user_00135')" clearable @change="handleTimeChange">
                    <el-option :label="lc('admin_user_weipin_00030')" value="sdate"></el-option>
                    <el-option :label="lc('admin_user_company_00272')" value="lastup"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmalltwo" :class="{ 'searchbutnOnff': seachbutn }">
                <el-date-picker v-model="search_params.times" type="daterange" align="right" unlink-panels :range-separator="lc('admin_company_00019')" :start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')" :picker-options="timeOptions" value-format="YYYY-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
            </div>


            <div class="tableSeachInpt tableSeachInptsmall" v-for="(searchitem, searchidx) in searchlist" :key="searchidx"
                :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="search_params[searchidx]" size="small" :placeholder="searchitem.name"
                    clearable @change="search">
                    <el-option v-for="(item, index) in searchitem.value" :label="item" :key="index"
                        :value="index"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt" :class="{ 'searchbutnOnff': seachbutn }">
                <div class="block">
                    <!-- 7.0 unified category selector -->
                    <job_class @confirm="confirmJobSearch"></job_class>
                </div>
            </div>
            <div class=" tableSeachInpt" :class="{ 'searchbutnOnff': seachbutn }">
                <div class="block">
                    <!-- 7.0 unified city selector -->
                    <city_class @confirm="confirmCitySearch"></city_class>
                </div>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
            <div class="tableSeachzk" :class="{ 'searchbutnKai': seachbutn }" style="margin-bottom: 8px;">
                <el-button type="info" class="zhankai" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="small" plain>{{ lc('admin_user_00145') }}<i class="el-icon-arrow-down el-icon--right"></i>
                </el-button>
                <el-button type="info" class="shouqi" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="small" plain>{{ lc('admin_user_00144') }}<i class="el-icon-arrow-up el-icon--right"></i>
                </el-button>
            </div>
        </div>
        <div class="admin_datatip" v-if="!simple">
            <i class="el-icon-document"></i> {{ lc("admin_data_stats") }}
            <span class="admin_datatip_n">{{ lc("admin_total_count", [allNum]) }}</span>
            <span class="admin_datatip_n">{{ lc("admin_pending_review_count", [status1Num]) }}</span>
            <span class="admin_datatip_n">{{ lc("admin_failed_count", [status2Num]) }}</span>
            <span class="admin_datatip_n">{{ lc("admin_offline_count", [status3Num]) }}</span>
            <span class="admin_datatip_n">{{ lc("admin_search_results_count", [total]) }}</span>
        </div>

        <div class="moduleElTable" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe @sort-change='sortChange' @mousedown="mouseDownHandler" @mouseup="mouseUpHandler" @mousemove="mouseMoveHandler"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" @selection-change="handleSelectionChange"
                ref="multipleTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('admin_user_company_00370')" width="90" sortable="custom"></el-table-column>
                <el-table-column :label="lc('admin_user_company_00360')" min-width="220">
                    <template #default="props">
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
                <el-table-column :label="lc('admin_00750')" min-width="100" v-if="!simple">
                    <template #default="props">
                        <div class="">
                            <span class="" v-if="props.row.rating_name"> {{ props.row.rating_name }}</span>
                            <div class=""> <span class="gsd"> {{ props.row.crm_salesman }}</span></div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" :label="lc('admin_user_company_00038')" width="130">
                    <template #default="props">
                        <div class="moduleProps">
                            <span>{{ lc('admin_user_company_00369') }}<el-button type="text" style="padding: 0;" @click="showComJobLogBox(props.row, 0)">{{ props.row.snum }}</el-button></span>
                            <span>{{ lc('admin_00746') }}<el-button type="text" style="padding: 0;" @click="showComJobLogBox(props.row, 1)">{{ props.row.browseNum }}</el-button></span>
                            <span>{{ lc('admin_00747') }}<el-button type="text" style="padding: 0;" @click="showComUserIdMsgBox(props.row)">{{ props.row.inviteNum }}</el-button></span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" :label="lc('wap_com_00236')" width="130">
                    <template #default="props">
                        <div class="job_tg_bth">
                            <el-switch v-model="props.row.istop" @change="tgchange($event, props.row, 1)" :inactive-text="lc('wap_user_00209')"></el-switch>
                        </div>
                        <div class="job_tg_bth">
                            <el-switch v-model="props.row.isrec" @change="tgchange($event, props.row, 2)" :inactive-text="lc('wap_com_00237')"></el-switch>
                        </div>
                        <div class="job_tg_bth">
                            <el-switch v-model="props.row.isurgent" @change="tgchange($event, props.row, 3)" :inactive-text="lc('member_com_00247')"></el-switch>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" :label="lc('member_user_00178')" width="130">
                    <template #default="props">
                        <el-switch v-model="props.row.iszp" @change="zpstatuschange($event, props.row)"></el-switch>
                        <div class="gsd">{{ lc('admin_00748') }}</div>
                    </template>
                </el-table-column>
                <el-table-column prop="logintime" :label="lc('admin_00751')" width="150">
                    <template #default="props">
                        <div class="moduleProps">
                            <span class="gsd">{{ props.row.sdate_n }}</span>
                            <span>{{ props.row.lastupdate_n_n }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="ip" :label="lc('admin_00512')" width="140" v-if="!simple">
                    <template #default="props">
                        <div class="moduleProps">
                            <span>{{ source[props.row.source] }}</span>
                            <span v-if="props.row.add_ip">{{ props.row.add_ip }}</span>
                            <span class="gsd" v-if="props.row.add_ip && props.row.ip_address"> {{ props.row.ip_address}}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" :label="lc('admin_00752')" width="130">
                    <template #default="props">
                        <div class="moduleProps">
                            <span class=" "> {{ props.row.jobhits }}/{{ props.row.jobexpoure }}</span>
                        </div>
                        <div class="jobtj">
                            <el-link icon="el-icon-edit" size="small" @click="jobhitedit(props.row)">{{ lc('wap_js_00073') }}</el-link>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="zt" :label="lc('member_user_00181')" fixed="right">
                    <template #default="props">
                        <div class="admin_state">
                            <template v-if="props.row.r_status == '2'">
                                <span class="admin_state3">{{ lc('admin_user_00138') }}</span>
                                <div style="display:inline-block" v-if="props.row.lock_info">
                                    <el-popover trigger="hover" placement="right">
                                        <p>{{ props.row.lock_info }}</p>
                                        <template #reference><div class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div></template>
                                    </el-popover>
                                </div>
                            </template>
                            <template v-else-if="props.row.state == 1">
                    			<span class="admin_state1">{{ lc('wap_user_00165') }}</span>
                    		</template>
                            <template v-else-if="props.row.state == 0">
                    			<!-- For unreviewed jobs, show company review status; locked companies are handled separately above -->
                    			<div v-if="props.row.r_status != '2'">
                    				<div>
                    					<span class="admin_state4" v-if="props.row.r_status == '0'">{{ lc('admin_user_company_00355') }}</span>
                    					<span class="admin_state1" v-else-if="props.row.r_status == '1'">{{ lc('admin_user_company_00354') }}</span>
                    					<span class="admin_state2" v-else-if="props.row.r_status == '3'">{{ lc('admin_user_company_00356') }}</span>
                    					<span class="admin_state3" v-else-if="props.row.r_status == '4'">{{ lc('admin_user_company_00365') }}</span>
                    				</div>
                    				<div>
                    					<span class="admin_state4">{{ lc('admin_user_company_00358') }}</span>
                    				</div>
                    			</div>
                    			<div v-else>
                    				<span class="admin_state4">{{ lc('wap_user_00166') }}</span>
                    			</div>
                    		</template>
                            <template v-else-if="props.row.state == 3">
                    			<!-- For rejected jobs, show company review status; locked companies are handled separately above -->
                    			<div v-if="props.row.r_status != '2'">
                    				<div>
                    					<span class="admin_state4" v-if="props.row.r_status == '0'">{{ lc('admin_user_company_00355') }}</span>
                    					<span class="admin_state1" v-else-if="props.row.r_status == '1'">{{ lc('admin_user_company_00354') }}</span>
                    					<span class="admin_state2" v-else-if="props.row.r_status == '3'">{{ lc('admin_user_company_00356') }}</span>
                    					<span class="admin_state3" v-else-if="props.row.r_status == '4'">{{ lc('admin_user_company_00365') }}</span>
                    				</div>
                    				<div>
                    					<span class="admin_state2">{{ lc('admin_user_company_00359') }}</span>
                    				</div>
                    			</div>
                    			<div v-else>
                    				<span class="admin_state2">{{ lc('wap_user_00167') }}</span>
                    			</div>
                                <div style="display:inline-block" v-if="props.row.statusbody">
                                    <el-popover trigger="hover" placement="right">
                                        <p>{{ props.row.statusbody }}</p>
                                        <template #reference><div class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div></template>
                                    </el-popover>
                                </div>
                            </template>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="140" fixed="right">
                    <template #default="scope">
                        <div class="moduleElTaCaoz">
                            <el-button size="small" plain @click="jobAudit(scope.row)">{{ lc('member_user_00152') }}</el-button>
                            <el-button size="small" v-if="scope.row.status == 1" plain @click="msg(lc('common_01539'))">{{ lc('member_com_00269') }}</el-button>
                            <el-button size="small" v-else plain @click="tw(scope.row)">{{ lc('admin_user_company_00157') }}</el-button>
                            <el-button size="small" plain @click="edit(scope.row)">{{ lc('wap_js_00073') }}</el-button>
                            <el-popover placement="bottom" width="90" trigger="hover">
                                <div class="moduleMores">
                                    <template v-if="search_params.openautho == 2">
                                        <el-button type="text" @click="linkopen(scope.row)">{{ lc('admin_user_company_00372') }}</el-button>
                                        
                                    </template>
                                    <el-button v-else type="text" @click="linkopen(scope.row)">{{ lc('admin_00749') }}</el-button>
                                    <template v-if="scope.row.status == 0 && scope.row.state == 1 && scope.row.r_status == 1">
                                        <el-button @click="getJobHtml(scope.row.id)" type="text">{{ lc('wap_com_00232') }}</el-button>
                                        <el-button v-if="hbNum > 0 && hb_isopen == 1" @click="createhb(scope.row)" type="text">{{ lc('wap_01572') }}</el-button>
                                        <el-button type="text" @click="resumematch(scope.row)">{{ lc('member_com_00296') }}</el-button>
                                    </template>
                                    <el-button @click="yyrefresh(scope.row)" type="text">{{ lc('member_com_00267') }}</el-button>
                                    <el-button v-if="scope.row.is_depower == 1" @click="depower(2, scope.row.id)" type="text">{{ lc('admin_user_company_00367') }}</el-button>
                                    <el-button v-else type="text" @click="depower(1, scope.row.id)">{{ lc('admin_user_company_00374') }}</el-button>
                                    <el-button type="text" @click="delrow(scope.row.id)">{{ lc('admin_user_company_00366') }}</el-button>
                                </div>
                                <template #reference><el-button size="small" plain @click="visible = !visible">{{ lc('common.more') }}</el-button></template>
                            </el-popover>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging" style="height: initial; flex-wrap: wrap; padding-top: 10px;">
            <div class="bottomButnBull" style="width:100%;">
                <div class="bottomButnBlak">
                    <el-checkbox v-model="checkedAll" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                    <el-button @click="delAllBottom" size="small">{{ lc('member_com_00055') }}</el-button>
                    <el-button @click="multipleStatus" size="small">{{ lc('member_user_00152') }}</el-button>
                    <el-button @click="multitg(3)" size="small">{{ lc('wap_00222') }}</el-button>
                    <el-button @click="multitg(1)" size="small">{{ lc('wap_user_00335') }}</el-button>
                    <el-button @click="multitg(2)" size="small">{{ lc('common.recommended') }}</el-button>
                    <el-button @click="refresh" size="small">{{ lc('wap_user_00334') }}</el-button>
                    
                    <el-button @click="exportdrawer = true" size="small">{{ lc('admin_user_00257') }}</el-button>
                    <el-button @click="multicate" size="small">{{ lc('admin_user_company_00371') }}</el-button>
                    <el-button @click="twtaskall" size="small">{{ lc('admin_user_company_00157') }}</el-button>
                </div>
            </div>
            <div class="modulePagNum" style="padding-top: 8px;">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" :current-page="currentPage" :page-sizes="pageSizes" :page-size="perPage" layout="total, sizes, prev, pager, next, jumper" :total="total"></el-pagination>
            </div>
        </div>
        <!-- Application records dialog -->
        <el-drawer :title="applyJobBoxTitle" v-model="drawerCompanyJobLog" append-to-body size="80%">
            <companyjoblog ref="companyjoblog" :searchjobid="jobid" :searchbrowse="sqJobBrowse" searchclass="drawer" v-if="drawerCompanyJobLog"></companyjoblog>
        </el-drawer>
        <!-- Interview records dialog -->
        <el-drawer :title="interviewBoxTitle" v-model="drawerCompanyUserIdMsg" append-to-body size="80%">
            <companyuseridmsg ref="companyuseridmsg" :searchjobid="jobid" v-if="drawerCompanyUserIdMsg" searchclass="drawer"></companyuseridmsg>
        </el-drawer>
        <!-- Exposure dialog -->
        <div class="modluDrawer" v-if="curr_job">
            <el-dialog :title="lc('admin_00753')" v-model="bgdrawer" :modal-append-to-body="false" append-to-body width="390px">
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_com_00157') }}</div>
                    <el-input v-model="curr_job.com_name" :placeholder="lc('wap_com_00157')" :disabled="true"></el-input>
                    <div class="wxsettip_small ">{{ lc('wap_com_00111') }}</div>
                    <el-input type="number" v-model="curr_job.jobexpoure" :placeholder="lc('wap_com_00111')"></el-input>
                    <div class="wxsettip_small ">{{ lc('wap_com_00112') }}</div>
                    <el-input type="number" v-model="curr_job.jobhits" :placeholder="lc('wap_com_00112')"></el-input>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="bgdrawer = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" :loading="jobhit_load" @click="jobhiteditsave">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Company post-to-feed dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_company_00345')" v-model="twdrawer" :modal-append-to-body="false" append-to-body width="450px">
                <div v-if="curr_job || multitw">
                    <div v-if="!multitw" class="wxsettip_small ">{{ lc('wap_com_00288') }}</div>
                    <el-input v-if="!multitw" :placeholder="lc('wap_com_00288')" v-model="curr_job.name" :disabled="true"></el-input>
                    <div class="wxsettip_small ">{{ lc('admin_user_company_00159') }}</div>
                    <el-checkbox v-model="twtask_urgent">{{ lc('admin_user_company_00156') }}</el-checkbox>
                    <el-checkbox v-model="twtask_wcmoments">{{ lc('admin_user_company_00152') }}</el-checkbox>
                    <el-checkbox v-model="twtask_gzh">{{ lc('admin_user_company_00148') }}</el-checkbox>
                    <div class="wxsettip_small ">{{ lc('member_user_00242') }}</div>
                    <el-input type="textarea" :rows="2" :placeholder="lc('admin_00621')" v-model="twtask_content"></el-input>
                    <div class="tw_tip" v-if="twTip">
                        <el-alert :title="twTip" type="warning" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="twdrawer = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" :loading="twtask_load" @click="addTwTask">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Selective contact visibility dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_company_00341')" v-model="drawerlinkopen" :modal-append-to-body="false" append-to-body width="350px">
                <div v-if="curr_job">
                    <el-radio-group v-model="curr_job.linkopen" size="small">
                        <el-radio label="1" border>{{ lc('wap_js_00098') }}</el-radio>
                        <el-radio label="2" border>{{ lc('admin_user_company_00304') }}</el-radio>
                    </el-radio-group>
                    <div class="tw_tip">
                        <el-alert :title="lc('wap_js_00098')" :description="lc('admin_user_company_00338')" type="warning" show-icon :closable="false"></el-alert>
                        <el-alert :title="lc('admin_user_company_00304')" type="warning" show-icon :closable="false">
                            <span>{{ lc('admin_user_company_00333') }}<br />{{ lc('admin_user_company_00336') }}</span>
                        </el-alert>
                    </div>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="drawerlinkopen = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="setlinkopen">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Job detail review ---------------------------------------------------------------------->
        <el-drawer :title="lc('admin_user_company_00326')" v-model="jobdrawersh" :modal-append-to-body="false" append-to-body size="80%">
            <job_review :id="statusId" :comclass_name="jobcomclassnamecache" :job_audit="job_audit" @confirm="jobdrawersh=false;getList()"   ></job_review>
        </el-drawer>
        <!-- Job promotion dialog -->
        <div class="modluDrawer">
            <el-dialog :title="jobtgtit" v-model="jobtgdrawer" append-to-body width="400px">
                <div class="wxsettip_small" v-if="jobtgtype == 1">{{ lc('wap_user_00209') }}</div>
                <div class="wxsettip_small" v-else-if="jobtgtype == 2">{{ lc('wap_com_00041') }}</div>
                <div class="wxsettip_small" v-else-if="jobtgtype == 3">{{ lc('wap_com_00043') }}</div>
                <el-input type="number" :placeholder="lc('admin_00614')" v-model="jobtgdays">
                    <template #append>{{ lc('common_02067') }}</template>
                </el-input>
                <div class="wxsettip_small" v-if="jobtgetime != ''">{{ lc('admin_00613') }}</div>
                <el-input v-if="jobtgetime != ''" v-model="jobtgetime" disabled></el-input>
                <div style="margin-top:10px;">
                    <i class="el-icon-warning"></i>
                    {{ lc('admin_user_company_00037') }}
                    <span v-if="jobtgtype == 1">{{ lc('wap_com_00238') }}</span>
                    <span v-else-if="jobtgtype == 2">{{ lc('home.recommended_jobs') }}</span>
                    <span v-else-if="jobtgtype == 3">{{ lc('member_com_00326') }}</span>
                    {{ lc('admin_user_company_00039') }}
                    <el-checkbox v-model="qxtgchecked" true-label="1" false-label="0"></el-checkbox>
                    <span>{{ lc('admin_user_company_00036') }}</span>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="jobtgdrawer = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" :loading="jobtg_load" @click="jobTgSubmit">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Copy text dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_com_00232')" v-model="drawercopy" append-to-body width="290px">
                <div id="to_copy" v-html="htmlcont"></div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="drawercopy = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" id="copyBtn" data-clipboard-action="copy" data-clipboard-target="#to_copy" @click="handleCopyText('copyBtn')">{{ lc('wap_com_00232') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Generate poster dialog -->
        <div class="modluDrawer">
            <el-drawer :title="lc('wap_01572')" v-model="drawerhb" :modal-append-to-body="false" append-to-body size="95%">
                <div class="waixunHaib">
                    <ul>
                        <li class="" v-for="(item, index) in hbarr" :key="index">
                            <div class="hb_listbox">
                                <div class="poster_pic"><img :src="item.pic_n"></div>
                                <div class="hb_listbox_name" style="background:#fff;">
                                    <div class="hb_cz">
                                        <a href="javascript:;" @click="showHb(item.id)">{{ lc('wap_00071') }}</a>
                                        <a href="javascript:;" @click="downHb(item.id)">{{ lc('wap_00070') }}</a>
                                    </div>
                                </div>
                            </div>
                        </li>
                    </ul>
                </div>
            </el-drawer>
        </div>
        <!-- Poster preview dialog -->
        <div class="tck_setbox" v-if="hburl != ''">
            <el-dialog :title="lc('admin_user_company_00142')" v-model="showhb" append-to-body width="300px">
                <div class="code_img" style="display:flex;justify-content: center;margin-bottom: 20px;">
                    <img :src="hburl" :key="hbkey" width="260">
                </div>
            </el-dialog>
        </div>
        <!-- Matching resumes dialog -->
        <div class="modluDrawer">
            <el-drawer :title="lc('member_com_00296')" v-model="drawermatchresume" :modal-append-to-body="false" append-to-body size="95%">
                <matchresume ref="matchresume" :job="curr_job" :jobtypes="job_types" :citytypes="city_types"></matchresume>
            </el-drawer>
        </div>
        <!-- Batch job review -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_company_00350')" width="300px" v-model="drawerauditmultiple" append-to-body :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_user_weipin_00065') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-radio v-model="multiStatus" label="1">{{ lc('admin_user_00149') }}</el-radio>
                            <el-radio v-model="multiStatus" label="3">{{ lc('wap_user_00167') }}</el-radio>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('member_user_00450') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input type="textarea" v-model="multiStatusBody" :rows="2" :placeholder="lc('admin_00676')">
                            </el-input>
                        </div>
                    </div>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="drawerauditmultiple = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" :loading="multipleStatus_load" @click="multipleStatusSave">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Export fields dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_00246')" v-model="exportdrawer" append-to-body width="740px">
                <div style="">
                    <el-checkbox-group v-model="checkedCols" @change="handleColCheckedChange">
                        <el-checkbox style="width:110px;margin-bottom: 5px;margin-left:0" size="small" border v-for="(item, index) in cols" :key="index" :label="item.value">{{ item.label }}</el-checkbox>
                        <el-checkbox style="width:110px;margin-left:0" size="small" border :indeterminate="isIndeterminate" v-model="colCheckAll" @change="handleColCheckAllChange">{{ lc('wap_js_00074') }}</el-checkbox>
                    </el-checkbox-group>
                </div>
                <div class="wxsettip_small">{{ lc('admin_00501') }}</div>
                <el-input type="number" :placeholder="lc('admin_00686')" v-model="exp_num"></el-input>
                <el-alert style="margin-top: 10px;" :title="lc('admin_user_company_00076')" type="warning" show-icon :closable="false"></el-alert>
                <template #footer><span class="dialog-footer">
                    <el-button @click="exportdrawer = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" :loading="export_load" @click="submitExport">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Batch category transfer -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_company_00348')" v-model="drawermulticate" append-to-body width="300px">
                <div class="wxsettip_small">{{ lc('member_com_00091') }}</div>
                <div class="TableSelect">
                    <el-select v-model="multihy" :placeholder="lc('wap_user_00100')">
                        <el-option v-for="(item, index) in cacheData.industry_index" :key="index" :label="cacheData.industry_name[item]" :value="item"></el-option>
                    </el-select>
                </div>
                <div class="wxsettip_small">{{ lc('wap_user_00018') }}</div>
                <div class="TableInpt">
                    <el-cascader style="width:260px;" v-model="multijobtype" :options="job_types" filterable clearable></el-cascader>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="drawermulticate = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" :loading="multicate_load" @click="submitMulticate">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Edit job notice -->
        <el-drawer :title="lc('admin_00754')" v-model="drawerEditJob" append-to-body :wrapper-closable="false" size="880px">
            <addjob ref="jobedit" :jid="jobid" :jtypes="job_types" :ctypes="city_types" v-if="drawerEditJob"></addjob>
        </el-drawer>
    </div>
    <!--        Reserved job refresh -->
    <div class="modluDrawer">
        <el-dialog :title="lc('admin_00755')" v-model="drawertz" :with-header="true" append-to-body :show-close="true"
                   width="400px">
            <div>
                <div class="wxsettip_small">{{ lc('wap_00850') }}</div>
                <div class="TableInpt">
                    <el-radio v-model="curr_data.reserve_status" label="1">{{ lc('member_com_00287') }}</el-radio>
                    <el-radio v-model="curr_data.reserve_status" label="2">{{ lc('common.close') }}</el-radio>
                </div>
                <div class="wxsettip_small">{{ lc('wap_com_00227') }}</div>
                <div class="TableSelect">
                    <el-select v-model="curr_data.reserve_interval" :placeholder="lc('wap_user_00100')">
                        <el-option v-for="(item, index) in jg_data" :key="index" :label="item.label" :value="item.value">
                        </el-option>
                    </el-select>
                </div>
                <div v-if="curr_data.reserve_interval == 1" class="wxsettip_small">{{ lc('admin_user_company_00361') }}</div>
                <div class="TableInpt" v-if="curr_data.reserve_interval == 1">
                    <el-input v-model="userinterval" :placeholder="lc('admin_00756')" size="small" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')">
                        <template #append>{{ lc('wap_com_00247') }}</template>
                    </el-input>
                </div>
                <div class="wxsettip_small">{{ lc('wap_com_00234') }}</div>
                <div class="TableInpt">
                    <el-date-picker v-model="curr_data.reserve_end" value-format="YYYY-MM-dd" type="date" :placeholder="lc('admin_00346')" :picker-options="pickerOptions">
                    </el-date-picker>
                </div>
                <div class="wxsettip_small">{{ lc('wap_com_00220') }}</div>
                <div class="TableInpt">
                    <el-time-picker v-model="curr_data.s_time" value-format="HH:mm">
                    </el-time-picker>
                    <div class="TableInptline">-</div>
                    <el-time-picker v-model="curr_data.e_time" value-format="HH:mm">
                    </el-time-picker>
                </div>
            </div>
            <template #footer><span class="dialog-footer">
					<el-button @click="drawertz = false">{{ lc('admin_user_weipin_00043') }}</el-button>
					<el-button type="primary" @click="submitTz" :loading="saveLoading">{{ lc('wap_com_00019') }}</el-button>
				</span></template>
        </el-dialog>
    </div>
</template>
<script>
import ComlogIndex from './comlog_index.vue'
import JobReview from './job_review.vue'
import ComlogUseridmsg from './comlog_useridmsg.vue'
import MatchResume from './match_resume.vue'
import Addjob from './addjob.vue'
import JobClass from '../../../component/job_class.vue'
import CityClass from '../../../component/city_class.vue'

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
        uid: { type: String, default: '' },
        state: { type: String, default: '' },
        status: { type: [String, Number], default: '' },
        crmindex: { type: String, default: '' },
        adtime: { type: String, default: '' },
        keyword: { type: String, default: '' },
        type: { type: String, default: '1' },
        simple: { // Simplified job management used by the CRM module
            type: Boolean,
            default: false
        },
        tsjl: { // Simplified job management used by the CRM module
            type: Boolean,
            default: false
        },
        scrolltop: { // Scroll to top when member job management changes pages
            type: Boolean,
            default: false
        },
    },
    data: function () {
        return {
            mouseFlag: false,
            mouseOffset: 0,
            loading: false,
            emptytext: lc('wap_js_00113'),
            allNum: 0,
            status1Num: 0,
            status2Num: 0,
            status3Num: 0,
            // {{ lc('admin_user_company_00348') }}
            drawermulticate: false,
            multihy: '',
            multijobtype: [],
            multitw: false, // Whether this is a batch feed-post task
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
                { label: lc('admin_user_company_00370'), value: 'id' },
                { label: lc('admin_user_company_00120'), value: 'uid' },
                { label: lc('wap_com_00288'), value: 'name' },
                { label: lc('admin_user_company_00373'), value: 'hy' },
                { label: lc('admin_user_company_00362'), value: 'job1' },
                { label: lc('admin_user_company_00364'), value: 'job1_son' },
                { label: lc('admin_user_company_00363'), value: 'job_post' },
                { label: lc('wap_user_00250'), value: 'provinceid' },
                { label: lc('common_02076'), value: 'cityid' },
                { label: lc('wap_com_00179'), value: 'three_cityid' },
                { label: lc('member_com_00017'), value: 'minsalary,maxsalary' },
                { label: lc('wap_com_00333'), value: 'zp_num' },
                { label: lc('wap_user_00240'), value: 'exp' },
                { label: lc('wap_com_00279'), value: 'report' },
                { label: lc('wap_com_00332'), value: 'sex' },
                { label: lc('member_com_00011'), value: 'edu' },
                { label: lc('wap_com_00282'), value: 'marriage' },
                { label: lc('admin_00343'), value: 'sdate' },
                { label: lc('wap_00326'), value: 'lastdate' },
                { label: lc('wap_com_00284'), value: 'zp_minage,zp_maxage' },
                { label: lc('wap_com_00292'), value: 'lang' },
                { label: lc('wap_com_00173'), value: 'welfare' },
                { label: lc('wap_01403'), value: 'com_name' },
                { label: lc('wap_00324'), value: 'pr' },
                { label: lc('wap_com_00163'), value: 'mun' }
            ],
            sort_type: '',
            sort_col: '',
            curr_job: null,
            auditInfo: null,
            r_status:'',
            job_audit:{},
            drawerCompanyJobLog: false,
            applyJobBoxTitle: lc('admin_user_company_00352'),
            sqJobBrowse: '',
            drawerCompanyUserIdMsg: false,
            interviewBoxTitle: lc('admin_company_00045'),
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

            // Appointment refresh
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
                {label: lc('admin_00757'), value: '60'},
                {label: lc('admin_00758'), value: '120'},
                {label: lc('admin_00759'), value: '180'},
                {label: lc('admin_00760'), value: '240'},
                {label: lc('admin_00761'), value: '300'},
                {label: lc('admin_00762'), value: '360'},
                {label: lc('admin_00763'), value: '420'},
                {label: lc('admin_00764'), value: '480'},
                {label: lc('wap_00852'), value: '1'},
            ],
            pickerOptions: {// el-date-picker date limits
                disabledDate(time) {
                    // Today and earlier dates
                    // return time.getTime() > Date.now();

                    // Today and later dates
                    return time.getTime() < Date.now() - 8.64e7;
                }
            },
            statusId:0,
            isSearchTime: false,
            timeOptions: {
                shortcuts: [{
                    text: lc('common_02000'),
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        start.setTime(start.getTime() - 3600 * 1000 * 24);
                        end.setTime(end.getTime() - 3600 * 1000 * 24);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: lc('common_01940'),
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: lc('admin_user_00146'),
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() - 1) * 24 * 60 * 60 * 1000);
                        const end = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: lc('admin_user_00142'),
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() + 6) * 24 * 60 * 60 * 1000);
                        const end = new Date(new Date().setHours(0, 0, 0) + (0 - new Date().getDay()) *24 * 60 * 60 *1000);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: lc('admin_user_00147'),
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 1).setHours(0, 0, 0));
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: lc('admin_user_00143'),
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
        'companyjoblog': ComlogIndex,
        'job_review': JobReview,
        'companyuseridmsg': ComlogUseridmsg,
        'matchresume': MatchResume,
        'addjob': Addjob,
        'job_class': JobClass,
        'city_class': CityClass,
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
            // The ref must point to the parent element that contains the table element
            let divData = this.$refs.multipleTable.bodyWrapper;
            if (this.mouseFlag) {
                // Set horizontal scroll position
                divData.scrollLeft -= (- this.mouseOffset + (this.mouseOffset = e.clientX));
            }
        },

        // Edit job
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
        // Get job count statistics
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

        // Batch edit job category
        multicate: function () {
            if (this.selectedItem.length == 0) {
                message.error(lc('admin_user_weipin_00001'))
                return false
            }
            this.multijobtype = ['', '', '']
            this.drawermulticate = true
        },
        // Submit batch job category edit
        submitMulticate: function () {
            var that = this
            if (that.selectedItem.length == 0) {
                message.error(lc('admin_user_weipin_00001'))
                return false
            }
            if (that.multihy == "") {
                message.error(lc('admin_user_company_00346'))
                return false
            }
            if (that.multijobtype[0] == '' || that.multijobtype[1] == '') {
                message.error(lc('wap_com_00272'))
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
        // {{ lc('admin_user_00257') }}exp_num
        submitExport() {
            let that = this
            if (that.checkedCols.length == 0) {
                message.error(lc('admin_user_weipin_00001'));
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
        // BatchAudit
        multipleStatus() {
            var that = this
            if (!that.selectedItem.length) {
                message.error(lc('admin_user_weipin_00001'));
                return false;
            }
            that.drawerauditmultiple = true
        },
        // Save batch review
        multipleStatusSave() {
            var that = this
            if (!that.selectedItem.length) {
                message.error(lc('admin_user_weipin_00001'));
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
        // Batch refresh
        refresh: function () {
            var that = this
            if (this.selectedItem.length == 0) {
                message.error(lc('admin_user_weipin_00001'))
                return false
            }
            httpPost('m=user&c=company_job&a=refresh', {
                ids: this.selectedItem.join(',')
            }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    message.success(lc('wap_01714'), function () {
                        that.getList()
                    })
                } else {
                    message.error(lc('wap_01713'))
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // Apply or cancel demotion
        depower: function (type, id) {
            var msg = '',
                that = this;
            if (type == 1) {
                msg = lc('admin_user_company_00334');
            } else {
                msg = lc('admin_user_company_00330');
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
        // {{ lc('member_com_00296') }}
        resumematch: function (row) {
            var that = this
            that.curr_job = row
            that.drawermatchresume = true
            that.$nextTick(function () {
                that.$refs.matchresume.getList()
            })
        },
        // Generate poster dialog
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
                    message.error(lc('admin_user_company_00096'))
                    return false
                }
            }).catch(function (e) {

            })
        },
        // Download poster
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
                    // Release URL object after use
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
        // Preview poster
        showHb(style) {
            this.hburl = this.basehburl + '&id=' + this.curr_job.id + '&hb=' + style
            this.hbkey = Math.random()
            this.showhb = true
        },
        // Copy text dialog
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

        // {{ lc('wap_com_00232') }}
        handleCopyText: function (id) {
            let clipboard = new ClipboardJS("#" + id);
            clipboard.on('success', (e) => {
                e.clearSelection();
                clipboard.destroy();
                message.success(lc('admin_user_company_00368'));
            });
            // Copy failed
            clipboard.on('error', (e) => {
                clipboard.destroy();
                message.error(lc('admin_user_company_00339'));
            });
        },
        
        // Permission and visibility permission
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
        // Message prompt
        msg: function (msg) {
            message.error(msg)
            return false
        },
        // Batch feed-post task
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
                        that.twTip = lc('admin_user_company_00329');
                    }
                    if (item.status != '0') {
                        statusMsg = lc('wap_com_00242');
                    }
                    if (item.state != '1') {
                        stateMsg = lc('wap_user_00166');
                    }
                    if (item.r_status != '1') {
                        rstatusMsg = lc('admin_user_company_00343');
                    }
                }
            })
            if (statusMsg != '' || stateMsg != '' || rstatusMsg != '') {
                var msg = lc('admin_user_company_00353');
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
                msg += lc('admin_user_company_00347');
                message.error(msg)
                return false
            }
            if (that.selectedItem.length == 0) {
                message.error(lc('admin_user_weipin_00001'))
                return false
            } else if (twnum > 0) {
                delConfirm(this, {}, function (params) {
                    that.multitw = true
                    that.twdrawer = true
                }, lc('admin_user_company_00331'))
            } else {
                that.multitw = true
                that.twdrawer = true
            }
        },
        // Feed post
        tw: function (row) {
            var that = this
            this.curr_job = row
            if (this.curr_job.tw_num > 0) {
                delConfirm(this, {}, function (params) {
                    that.addTw()
                }, lc('admin_user_company_00335'))
            } else {
                that.addTw()
            }
        },
        addTw: function () {
            var nowTime = parseInt(new Date().getTime() / 1000);
            lastupdate = Number(this.curr_job.lastupdate);
            this.twTip = '';
            if (nowTime - lastupdate > 60 * 60 * 24 * 3) {
                this.twTip = lc('admin_user_company_00332');
            }
            this.twdrawer = true
        },
        // Add feed-post task
        addTwTask: function () {
            var that = this
            var params = {
                twtask_content: that.twtask_content,
                twtask_urgent: that.twtask_urgent ? 1 : 0,
                twtask_wcmoments: that.twtask_wcmoments ? 1 : 0,
                twtask_gzh: that.twtask_gzh ? 1 : 0,
            };
            if (that.multitw) { // Batch feed-post task
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
        // Select job review template
        auditTplChange: function (data) {
            this.auditInfo.statusbody = this.cacheData.comclass_name[data]
        },
        // Job review dialog
        jobAudit: function (row) {

            let that = this;
            this.statusId= row.id;
            that.jobdrawersh = true;
        },
        // Batch promotion
        multitg: function (type) {
            this.jobtgtype = type
            this.jobtgetime = ''
            if (this.selectedItem.length == 0) {
                message.error(lc('admin_user_weipin_00001'))
                return false
            }
            if (type == 1) { // Top placement
                this.jobtgtit = lc('admin_user_company_00351')
            } else if (type == 2) { // Recommendation
                this.jobtgtit = lc('admin_company_00044')
            } else if (type == 3) { // Urgent promotion
                this.jobtgtit = lc('admin_user_company_00349')
            }
            this.tgjid = this.selectedItem.join(',')
            this.jobtgdrawer = true
        },
        // Job promotion settings
        tgchange: function (val, data, type) {
            this.jobtgtype = type
            this.curr_job = data
            this.tgjid = data.id
            if (type == 1) { // {{ lc('wap_user_00335') }}
                this.curr_job.istop = !this.curr_job.istop // Prevent switch state from changing before request result
                this.jobtgetime = data.top_time_n ? data.top_time_n : ''
                this.jobtgtit = lc('wap_com_00238')
            } else if (type == 2) { // Recommendation
                this.curr_job.isrec = !this.curr_job.isrec // Prevent switch state from changing before request result
                this.jobtgetime = data.rec_time_n != undefined ? data.rec_time_n : ''
                this.jobtgtit = lc('wap_com_00237')
            } else if (type == 3) { // Urgent promotion
                this.curr_job.isurgent = !this.curr_job.isurgent // Prevent switch state from changing before request result
                this.jobtgetime = data.urgent_time_n ? data.urgent_time_n : ''
                this.jobtgtit = lc('member_com_00613')
            }
            this.jobtgdrawer = true
        },
        // Submit job promotion
        jobTgSubmit: function () {
            var that = this
            var url = 'm=user&c=company_job&a='
            if (that.jobtgtype == 1) {
                url += 'xuanshang'
                if (that.qxtgchecked == 0 && that.jobtgdays == '') {
                    message.error(lc('common_06281'))
                    return false
                }
            } else if (that.jobtgtype == 2) {
                url += 'recommend'
                if (that.qxtgchecked == 0 && that.jobtgdays == '') {
                    message.error(lc('common_06282'))
                    return false
                }
            } else if (that.jobtgtype == 3) {
                url += 'urgent'
                if (that.qxtgchecked == 0 && that.jobtgdays == '') {
                    message.error(lc('admin_company_00027'))
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
        // Change job recruitment status
        zpstatuschange: function (val, row) {
            var that = this
            that.curr_job = row
            that.curr_job.iszp = !that.curr_job.iszp // Prevent switch state changes before submitting request
            httpPost('m=user&c=company_job&a=checkstate', { id: that.curr_job.id, state: val ? 2 : 1 }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.curr_job.iszp = !that.curr_job.iszp // Change switch selected state after successful operation
                    that.getList()
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // Edit view count
        jobhitedit: function (row) {
            var that = this
            that.curr_job = deepClone(row);
            that.curr_job.jobhits = that.curr_job.job_hits;
            that.curr_job.jobexpoure = that.curr_job.job_expoure;
            if (that.curr_job.r_status == 2) {
                message.error(lc('admin_user_company_00337'))
                return false
            } else {
                that.bgdrawer = true
            }
        },
        jobhiteditsave: function () {
            var that = this
            if (parseInt(that.curr_job.jobexpoure) < parseInt(that.curr_job.jobhits)) {
                message.error(lc('admin_user_company_00340'));
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
            that.emptytext = lc('admin_user_weipin_00026');

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
                        that.emptytext = lc('wap_js_00113');
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
                message.error(lc('admin_user_weipin_00005'));
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
                    message.success(lc('wap_user_00264'), function () {
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
        // Search job selector
        confirmJobSearch(data) {
            this.search_params.job_class = data.jobId.join(',');
        },
        // Search city selector
        confirmCitySearch(data) {
            this.search_params.city_class = data.cityId.join(',');
        },
        // View job application records
        showComJobLogBox: function (e, browse) {
            let _this = this;
            _this.jobid = e.id;
            _this.applyJobBoxTitle = lc("admin_company_00040", [e.name]);
            _this.sqJobBrowse = browse == 1 ? '1' : '';
            _this.drawerCompanyJobLog = true;
        },
        // View job interview records
        showComUserIdMsgBox: function (e) {
            let _this = this;
            _this.jobid = e.id;
            _this.interviewBoxTitle = lc("admin_company_00041", [e.name]);
            _this.drawerCompanyUserIdMsg = true;
        },
        // Appointment refresh
        yyrefresh(detail) {
            let date = new Date();
            // this.curr_data = detail;
            this.curr_data.reserve_end = (detail.reserve_end == lc('common_01936') ||detail.reserve_end == ''|| detail.reserve_end == undefined)  ? '' : detail.reserve_end;
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
                message.error(lc('member_com_00279'));
                return false;
            } else if (that.curr_data.reserve_status == 1) {
                if (that.curr_data.reserve_interval <= 0) {
                    message.error(lc('wap_00851'));
                    return false;
                }
                if (that.curr_data.reserve_interval == 1 && that.userinterval == '') {
                    message.error(lc('admin_company_00018'));
                    return false;
                }
                if (that.curr_data.s_time.length > 0 && that.curr_data.e_time.length > 0) {
                    var stime = that.curr_data.s_time.split(':');
                    var etime = that.curr_data.e_time.split(':');
                    if (parseInt(stime[0]) > parseInt(etime[0]) || (parseInt(stime[0]) == parseInt(etime[0]) && parseInt(stime[1]) >= parseInt(etime[1]))) {
                        message.error(lc('wap_com_00213'));
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