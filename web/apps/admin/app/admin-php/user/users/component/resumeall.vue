<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-input :placeholder="lc('admin_user_00158')" @keyup.enter="search" size="small" v-model="searchForm.keyword"
                          clearable>
					<template #prepend><el-select v-model="searchForm.keytype" style="padding-left: 12px;" size="small" :placeholder="lc('wap_user_00015')">
					    <el-option :label="lc('wap_user_00015')" :value="1"></el-option>
					    <el-option :label="lc('wap_00529')" :value="2"></el-option>
					    <el-option :label="lc('member_com_00012')" :value="3"></el-option>
					    <el-option :label="lc('admin_user_00140')" :value="4"></el-option>
					    <el-option :label="lc('wap_01619')" :value="5"></el-option>
					    <el-option :label="lc('wap_00459')" :value="6"></el-option>
					    <el-option :label="lc('wap_00457')" :value="7"></el-option>
					    <el-option :label="lc('wap_00465')" :value="8"></el-option>
					    <el-option :label="lc('wap_00455')" :value="9"></el-option>
					    <el-option :label="lc('wap_00461')" :value="10"></el-option>
					    <el-option label="IP" :value="11"></el-option>
					</el-select></template>
                </el-input>
            </div>
            <!-- Collapsed section -->
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="searchForm.time_type" size="small" :placeholder="lc('admin_user_00135')" clearable @change="handleTimeChange">
                    <el-option :label="lc('member_com_00087')" value="adtime"></el-option>
                    <el-option :label="lc('wap_00326')" value="uptime"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmalltwo" :class="{ 'searchbutnOnff': seachbutn }">
                <el-date-picker v-model="searchForm.times" type="daterange" align="right" unlink-panels :range-separator="lc('admin_company_00019')" :start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')" :picker-options="timeOptions" value-format="YYYY-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
            </div>
            <div v-for="(searchItem, searchIndex) in searchList" :key="searchIndex" class="tableSeachInpt tableSeachInptsmall"
                :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="searchForm[searchItem.param]" :clearable="true"
                    :placeholder="searchItem.name" size="small" @change="search">
                    <el-option v-for="(searchLabel, searchValue) in searchItem.value" :key="searchValue" :label="searchLabel"
                        :value="searchValue"></el-option>
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
            <div class="tableSeachInpt">
                <el-button type="primary" plain icon="el-icon-plus" size="small" @click="openAdd">{{ lc('admin_user_00193') }}</el-button>

            </div>
            <div class="tableSeachInpt tableSeachzk" :class="{ 'searchbutnKai': seachbutn }">
                <el-button type="info" class="zhankai" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="small" plain>{{ lc('admin_user_00145') }}<i class="el-icon-arrow-down el-icon--right"></i>
                </el-button>
                <el-button type="info" class="shouqi" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="small" plain>{{ lc('admin_user_00144') }}<i class="el-icon-arrow-up el-icon--right"></i>
                </el-button>
            </div>
        </div>
        <div class="admin_datatip">
            <i class="el-icon-document"></i> {{ lc("admin_data_stats") }} <span class="cp_n" @click="init">{{ lc("admin_total_count", [resumeAllNum]) }}</span>
            <span class="admin_datatip_n"><span class="cp_n" @click="statusSearch('4')">{{ lc("admin_pending_review_count", [resumeStatusNum1 ? resumeStatusNum1 : 0]) }}</span></span>
            <span class="admin_datatip_n"><span class="cp_n" @click="statusSearch('3')">{{ lc("admin_failed_count", [resumeStatusNum2 ? resumeStatusNum2 : 0]) }}</span></span>
            <span class="admin_datatip_n"><span class="cp_n" @click="statusSearch('2')">{{ lc("admin_locked_count", [resumeStatusNum3 ? resumeStatusNum3 : 0]) }}</span></span>
            <span class="admin_datatip_n">{{ lc('admin_00495') }}<span class="cp_n" @click="statusSearch('1')">{{ resumeTeenNum ? resumeTeenNum : 0 }}</span> {{ lc('common_02088') }}</span>
            <span class="admin_datatip_n">{{ lc("admin_search_results_count", [total]) }}</span>
        </div>
        <div class="moduleElTable" :class="{ 'moduleElTabGetResuma': tableHig }"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="list" style="width: 100%" stripe ref="multipleTable" @selection-change="handleSelectionChange"


                @mousedown="mouseDownHandler"
                @mouseup="mouseUpHandler"
                @mousemove="mouseMoveHandler"


                @sort-change="sortChange" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading">
                <template #empty>
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="50"></el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00012')" width="90" sortable="custom"></el-table-column>
                <el-table-column :label="lc('admin_user_00243')" min-width="165">
                    <template #default="scope">
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
                <el-table-column :label="lc('wap_00456')" min-width="230">
                    <template #default="scope">
                        <div class=" ">
                            <div>
                                <span class="user_resumejob" @click="openPreview(scope.row)">{{ scope.row.name }}</span>
                               <!-- <span v-if="scope.row.defaults == 1" class="user_resumrmr">Default</span>-->
                            </div>
                            <div class="">
                                {{ lc('admin_00496') }}
                                <span v-if="scope.row.edu_n">{{ lc('admin_00497') }}</span>
                                <span v-if="scope.row.exp_n">{{ lc('admin_00498') }}</span>
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

                <el-table-column :label="lc('admin_00509')" min-width="110" align="center">
                    <template #default="scope">
                        <el-tag type="danger" size="small" effect="dark" v-if="scope.row.integrity < 65">{{ scope.row.integrity }}%</el-tag>
						<el-tag type="success" size="small" effect="dark" v-else>{{ scope.row.integrity }}%</el-tag>

                        <div v-if="scope.row.status == 1" class="jlzt">
                            <el-button type="text" @click="openStatus(scope.row)"><i class="el-icon-unlock"></i> {{ lc('wap_js_00005') }}
                            </el-button>
                        </div>
                        <div v-else-if="scope.row.status == 3" class="jlzt">
                            <el-button type="text" @click="openStatus(scope.row)"><i class="el-icon-unlock"></i> {{ lc('admin_user_00249') }}
                            </el-button>
                        </div>
                        <div v-else class="jlwgk jlzt">
                            <el-button type="text" @click="openStatus(scope.row)"><i class="el-icon-lock"></i> {{ lc('admin_user_00253') }}
                            </el-button>
                        </div>
                    </template>

                </el-table-column>
                <el-table-column :label="lc('admin_00510')" width="80" align="center">
                    <template #default="scope">
                        <div class="moduleProps">
                            {{ scope.row.sq_num ? scope.row.sq_num : 0 }}
                        </div>
                        <div class="moduleProps" v-if="scope.row.sq_num > 0">
                            <span class="jobtj">
                                <el-button type="text" @click="openJobSqlLog(scope.row)">{{ lc('wap_com_00427') }}</el-button>
                            </span>
                        </div>
                    </template>

                </el-table-column>
				<el-table-column prop="comd" :label="lc('member_com_00110')" min-width="130">
				    <template #default="scope">
				        <div class="job_tg_bth">
				            <el-switch v-model="scope.row.rec_resume" :inactive-text="lc('wap_01465')" :width="30" active-value="1"
				                inactive-value="0" @change="changeRec($event, scope.row)">
				            </el-switch>
				        </div>
				        <div class="job_tg_bth jobBthChufa">
				            <!-- Clicking switch changes the value, so use an overlay to trigger the event -->
				            <div class="chufaButn" @click="openTop(scope.row)">{{ lc('admin_00499') }}</div>
				            <el-switch v-model="scope.row.top_day" :inactive-text="lc('wap_user_00335')" :width="30"
				                :active-value="scope.row.top_day > 0 ? scope.row.top_day : 1"
				                inactive-value="0"></el-switch>
				        </div>
				    </template>
				</el-table-column>
                <el-table-column prop="logintime" :label="lc('admin_vue_00019')" min-width="150">
                    <template #default="scope">
                        <div class="moduleProps">
                            <span class="gsd">{{ scope.row.lastupdate_n }}</span>
                            <span>{{ scope.row.ctime_n }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="ip" :label="lc('admin_00512')" min-width="150">
                    <template #default="scope">
                        <div class="moduleProps">
                            <span>{{ source[scope.row.source] }}<span v-if="scope.row.doc == 1">{{ lc('admin_user_00258') }}</span></span>
                            <span v-if="scope.row.add_ip">{{ scope.row.add_ip }}</span>
                            <span v-if="scope.row.ip_address" class="gsd"> {{ scope.row.ip_address }}</span>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column prop="zt" :label="lc('member_user_00181')" width="120" fixed="right">
                    <template #default="scope">
                        <div class="admin_state">
                            <div v-if="scope.row.r_status == '2'">
                                <span class="admin_state3">{{ lc('admin_user_00138') }}</span>
                                <div style="display:inline-block" v-if="scope.row.lock_info">
                                    <el-popover trigger="hover" placement="right">
                                        <p>{{ scope.row.lock_info }}</p>
                                        <template #reference><div class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div></template>
                                    </el-popover>
                                </div>
                            </div>
                            <span v-else-if="scope.row.state == 1" class="admin_state1">{{ lc('wap_user_00165') }}</span>
                            <span v-else-if="scope.row.state == 3" class="admin_state2">
                                {{ lc('wap_user_00167') }}
                                <el-tooltip effect="dark" :content="scope.row.statusbody" placement="top"
                                    v-if="scope.row.statusbody">
                                    <i class="el-icon-warning-outline"></i>
                                </el-tooltip>
                            </span>
                            <span v-else-if="scope.row.state == 2" class="admin_state3">{{ lc('admin_user_00255') }}</span>
                            <span v-else class="admin_state5">{{ lc('wap_user_00166') }}</span>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column :label="lc('member_user_00048')" width="140" fixed="right">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" plain @click="openAudit(scope.row)">{{ lc('member_user_00152') }}</el-button>

                            <el-button size="small" plain @click="refresh(scope.row)">{{ lc('wap_user_00334') }}</el-button>
                        </div>
                        <div class="cz_button" style="margin-top: 10px;">
                            <el-button size="small" plain @click="openRemark(scope.row)">{{ lc('member_user_00242') }}</el-button>

                            <el-button type="danger" size="small" @click="openDel(scope.$index)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging" style="height: initial; flex-wrap: wrap; padding-top: 10px;">
            <div class="bottomButnBull" style="width:100%;">
                <div class="bottomButnBlak">
                    <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
                    @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>
                    <el-button size="small" @click="batch('del')">{{ lc('member_com_00055') }}</el-button>
                    <el-button size="small" @click="batch('audit')">{{ lc('admin_user_weipin_00037') }}</el-button>
                    <el-button size="small" @click="batch('refresh')">{{ lc('admin_user_00248') }}</el-button>
                    <el-button size="small" @click="batch('rec')">{{ lc('admin_user_00237') }}</el-button>
                    <el-button size="small" @click="batch('rec_cancel')">{{ lc('wap_com_00230') }}</el-button>
                    <el-button size="small" @click="batch('top')">{{ lc('admin_00500') }}</el-button>
                    <el-button size="small" @click="batch('top_cancel')">{{ lc('wap_com_00231') }}</el-button>
                    <el-button size="small" @click="batch('export')">{{ lc('admin_user_00257') }}</el-button>
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
            <!-- Export field selection dialog -->
            <el-dialog :title="lc('admin_user_00246')" v-model="dialogExport" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="650px">
                <div class="tck_setname">
                    <el-checkbox-group v-model="ruleFormExport.type" @change="handleCheckedExportType">
                        <el-checkbox :label="field" v-for="(fieldName, field) in typeExport" :key="field">{{ fieldName }}</el-checkbox>
                    </el-checkbox-group>
                    <el-checkbox :indeterminate="isIndeterminateExport" v-model="checkAllExport"
                        @change="handleCheckAllExport">{{ lc('wap_js_00074') }}</el-checkbox>
                </div>
                <div class="daochuNumer">
                    <div class="daochuTite">
                        <span>{{ lc('admin_00501') }}</span>
                    </div>
                    <div class="daochuFrom">
                        <div class="daochuFroInpt">
                            <el-input v-model="ruleFormExport.limit"
                                @input="inputIntNumber($event, 'ruleFormExport', 'limit')"></el-input>
                        </div>
                        <div>
                            <el-alert :closable="false" :title="lc('admin_00513')" type="info" show-icon>
                            </el-alert>
                        </div>
                    </div>

                    <!-- <span>
                        <el-input v-model="ruleFormExport.limit"
                            @input="inputIntNumber($event, 'ruleFormExport', 'limit')"></el-input>
                    </span>
                    <el-alert :closable="false" :title="lc('admin_00513')" type="info" show-icon>
                    </el-alert> -->
                </div>
                <div class="daochuNumer">
                    <div class="daochuTite">
                        <span>{{ lc('admin_00502') }}</span>
                    </div>
                    <div class="daochuFrom">
                        <div class="daochuFroInpt">
                            <el-input v-model="ruleFormExport.section"></el-input>
                        </div>
                        <div>
                            <el-alert :closable="false" :title="lc('admin_00514')" type="info" show-icon>
                            </el-alert>
                        </div>
                    </div>

                    <!-- <span><el-input v-model="ruleFormExport.section"></el-input></span>
                    <el-alert :closable="false" :title="lc('admin_00514')" type="info" show-icon>
                    </el-alert> -->
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogExport = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitExport" :disabled="saveLoading">{{ lc('admin_user_00254') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Public resume -->
        <div class="modluDrawer">
            <el-dialog :title="lc('member_com_00110')" v-model="dialogStatus" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div class="wxsettip_small ">{{ lc('wap_00529') }}</div>
                <el-input :value="detail.uname" :disabled="true"></el-input>
                <div class="wxsettip_small ">{{ lc('member_com_00110') }}</div>
                <div class="wxsettip_Sealect">
                    <el-select v-model="ruleFormStatus.status" :placeholder="lc('wap_user_00100')">
                        <el-option key="1" :label="lc('wap_js_00005')" value="1"></el-option>
                        <el-option key="3" :label="lc('admin_00515')" value="3"></el-option>
                        <el-option key="2" :label="lc('admin_user_00259')" value="2"></el-option>
                    </el-select>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogStatus = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitStatus" :disabled="saveLoading">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Resume top placement -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_user_00207')" v-model="dialogTop" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div class="wxsettip_small ">{{ lc('wap_user_00209') }}</div>
                <el-input v-model="ruleFormTop.addday" @input="inputIntNumber($event, 'ruleFormTop', 'addday')">
                    <template #append>{{ lc('common_02067') }}</template>
                </el-input>
                <template v-if="detail.top_day > 0">
                    <div class="danqainDataFlex">
                        <div class="wxsettip_small ">{{ lc('admin_00503') }}</div>
                        <div style="color:#f60">{{ detail.topdate_n }}</div>
                    </div>

                </template>
                <div>
                    {{ lc('admin_00504') }} <el-checkbox v-model="ruleFormTop.s" true-label="1" false-label="0"></el-checkbox> {{ lc('admin_00505') }}
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogTop = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitTop" :disabled="saveLoading">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Resume remark -->
        <div class=" ">
            <el-dialog :title="lc('wap_com_00070')" v-model="dialogRemark" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div class="wxsettip_small ">{{ lc('admin_00506') }}</div>
                <div class="wxsettip_Sealect">
                    <el-select v-model="ruleFormRemark.label" :placeholder="lc('wap_user_00100')">
                        <el-option v-for="labelkey in userdata.user_label" :key="labelkey" :label="userclass_name[labelkey]"
                            :value="labelkey">
                        </el-option>
                    </el-select>
                </div>
                <div class="wxsettip_small ">{{ lc('admin_00507') }}</div>
                <el-input v-model="ruleFormRemark.content" type="textarea" :placeholder="lc('admin_00516')"></el-input>

                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogRemark = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitRemark">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>

        <!-- Application records -->
        <el-drawer :title="lc('admin_00517')" :append-to-body="true" v-model="drawerJobSqLog" size="80%">
            <div class="uploadTable" style="padding:0px 20px;font-size:14px;color:#666">
                <div class="moduleElHight">
                    <div class="moduleElTable moduleElMoreInt" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
                        <el-table :data="jobSqLog.list" style="width: 100%" stripe ref="table2"
                            :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading">
                            <template #empty>
                                <p>{{dataText}}</p>
                            </template>
                            <el-table-column prop="job_name" :label="lc('wap_01596')">
                                <template #default="scope">
                                    <div class="moduleProps">
                                        <el-link type="primary" :underline="false"
                                            @click="openPage(scope.row.job_comapply)">{{ scope.row.job_name }}</el-link>
                                    </div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="com_name" :label="lc('admin_user_00247')">
                                <template #default="scope">
                                    <div class="moduleProps">
                                        <el-link type="primary" :underline="false"
                                            @click="openPage(scope.row.company_show)">{{ scope.row.com_name }}</el-link>
                                    </div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="datetime_n_n" :label="lc('member_user_00431')"></el-table-column>
                            <el-table-column :label="lc('admin_user_00250')">
                                <template #default="scope">
                                    <div class="admin_state">
                                        <span class="admin_state1" v-if="scope.row.is_browse == 2">{{ lc('wap_user_00258') }}</span>
                                        <span class="admin_state2" v-else-if="scope.row.is_browse == 3">{{ lc('admin_user_00252') }}</span>
                                        <span class="admin_state3" v-else-if="scope.row.is_browse == 4">{{ lc('wap_user_00354') }}</span>
                                        <span class="admin_state4" v-else-if="scope.row.is_browse == 5">{{ lc('member_com_00108') }}</span>
                                        <span class="admin_state5" v-else>{{ lc('wap_user_00260') }}</span>
                                    </div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="isdel_n" :label="lc('member_user_00181')"></el-table-column>
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
        <!-- Batch review -->
        <el-dialog :title="lc('admin_user_weipin_00037')" v-model="dialogAudit" :modal-append-to-body="false" :show-close="true" width="500px">
            <div class="toolClasDia fenpeizhand">
                <div class="toolClasList">
                    <div class="toolClasTite">
                        <span>{{ lc('admin_user_weipin_00065') }}</span>
                    </div>
                    <div class="toolClasCont">
                        <el-radio v-model="ruleFormAudit.status" label="1">{{ lc('admin_user_00149') }}</el-radio>
                        <el-radio v-model="ruleFormAudit.status" label="3">{{ lc('wap_user_00167') }}</el-radio>
                    </div>
                </div>
                <div class="toolClasList">
                    <div class="toolClasTite">
                        <span>{{ lc('member_user_00450') }}</span>
                    </div>
                    <div class="toolClasCont">
                        <el-input type="textarea" :rows="2" placeholder="" v-model="ruleFormAudit.statusbody">
                        </el-input>
                    </div>
                </div>
            </div>
            <template #footer><span class="dialog-footer">
                <el-button @click="dialogAudit = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="submitBatchAudit">{{ lc('wap_com_00019') }}</el-button>
            </span></template>
        </el-dialog>
        <!-- Resume review -->
        <el-drawer :title="lc('member_com_00028')" v-model="drawerAudit" @closed="closedAudit"
            :modal-append-to-body="false" size="90%" :append-to-body="true">
            <div class="shbox" style="padding-right: 380px;;" v-loading="expectLoading">
                <div style="overflow-y: auto;position: relative;height: 100%; padding-right: 25px; border-right: 1px solid #eee;">
                    <div class="shshow_tit">
                        <i class="el-icon-office-building"></i> {{ lc('wap_user_00341') }}
                        <span class="shshow_cz">
                            <el-button type="text" @click="openBasic">
                                <i class="el-icon-edit"></i>{{ lc('admin_user_00227') }}
                            </el-button>
                        </span>
                    </div>
                    <div class="userinfo_box">
                        <div class="userinfo_l"><img :src="resume.photo" width="70" height="70"></div>
                        <div class="userinfo_r">
                            <div class="userinfo_name">{{ resume.name }}</div>
                            <div class="userinfo">
                                {{ resume.sex_n }}
                                <span v-if="resume.age">{{ lc('admin_user_00198') }}</span>
                                <span v-if="resume.height">，{{ resume.height }}cm</span>
                                <span v-if="resume.weight">，{{ resume.weight }}kg</span>
                                <span v-if="resume.marriage_n">，{{ resume.marriage_n }}</span>
                                <span v-if="resume.living">{{ lc('admin_00468') }}</span>
                            </div>
                            <div class="userinfo" v-if="resume.edu_n || resume.exp_n">
                                <span v-if="resume.edu_n">{{ lc('admin_00469') }} </span>
                                <span class="userline" v-if="resume.edu_n && resume.exp_n">|</span>
                                <span v-if="resume.exp_n">{{ lc('admin_00470') }}</span>
                            </div>
                        </div>
                    </div>
                    <div class="shshow_p">
                        <div class="cominfo" v-if="resume.telphone"><i class="el-icon-mobile"></i>
                            {{ lc("admin_contact_phone_value", [resume.telphone]) }}</div>
                        <div class="cominfo" v-if="resume.email"><i class="el-icon-message"></i>
                            {{ lc("admin_email_value", [resume.email]) }}</div>
                        <div class="cominfo" v-if="resume.idcard"><i class="el-icon-postcard"></i>
                            {{ lc("admin_idcard_value", [resume.idcard]) }}</div>
                        <div class="cominfo" v-if="resume.domicile"><i class="el-icon-location-outline"></i>
                            {{ lc("admin_domicile_value", [resume.domicile]) }}</div>
                        <div class="cominfo" v-if="resume.address"><i class="el-icon-location-information"></i>
                            {{ lc("admin_detail_address_value", [resume.address]) }}</div>
                    </div>

                    <!-- Personal strengths -->
                    <div class="user_resume_list">
                        <div class="shshow_tit">
                            <i class="el-icon-medal-1"></i> {{ lc('wap_user_00326') }}
                        </div>
                        <div class="shshow_p">
                            <el-tag size="small" v-for="(tagItem,key) in resume.arrayTag" :key="key">{{ tagItem }}</el-tag>
                            <div class="cominfo">{{ resume.description }}</div>
                        </div>
                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">Summarize strengths and highlights. Personal strengths are shown prominently to HR.</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openTag">
                                    <i class="el-icon-circle-plus-outline"></i> {{ (resume.arrayTag &&
                                        resume.arrayTag.length > 0) || resume.description ? lc('common.edit') : lc('wap_js_00091') }}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!-- Job intention -->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-notebook-2"></i> {{ lc('wap_00460') }}</div>
                        <div class="shshow_p" v-if="expectData.expect">
                            <div class="cominfo">{{ lc("admin_expected_position_value", [expectData.expect.name]) }} </div>
                            <div class="cominfo">{{ lc("admin_current_position_value", [expectData.expect.job_classname]) }}</div>
                            <div class="cominfo">{{ lc("admin_expected_location_value", [expectData.expect.city_classname]) }}</div>
                            <div class="cominfo">{{ lc("admin_expected_salary_value", [expectData.expect.salary]) }}</div>
                            <div class="cominfo">{{ lc("admin_industry_value", [expectData.expect.hy_n]) }}</div>
                            <div class="cominfo">{{ lc("admin_arrival_time_value", [expectData.expect.report_n]) }}</div>
                            <div class="cominfo">{{ lc("admin_work_nature_value", [expectData.expect.type_n]) }}</div>
                            <div class="cominfo">{{ lc("admin_job_status_value", [expectData.expect.jobstatus_n]) }}</div>
                        </div>


                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">Recommend completing job preferences</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openJob">
                                    <i class="el-icon-circle-plus-outline"></i> {{ lc('admin_00472') }}
                                </el-button>
                            </div>
                        </div>
                    </div>

                    <!-- Work experience -->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-suitcase-1"></i> {{ lc('wap_00457') }}</div>
                        <!-- Loop -->
                        <div class="user_resume_show" v-for="(work, workkey) in expectData.work" :key="workkey">
                            <div class="user_resume_addname ">{{ work.name }}
                                <el-button type="text" @click="openWork(workkey)">
                                    <i class="el-icon-edit"></i> {{ lc('wap_js_00073') }}
                                </el-button>
                                <el-button type="text" @click="delResumeFb('work', workkey, work.id)">
                                    <i class="el-icon-delete"></i> {{ lc('common.delete') }}
                                </el-button>
                            </div>
                            <div class="user_resume_addjy">
                                <div class=" ">{{ work.title }}</div>
                                <div class="user_resume_time">{{ work.sdate_n }}-{{ work.edate_n }}</div>
                            </div>
                            <div class="user_resume_ms">{{ work.content }}</div>
                        </div>
                        <!-- Loop -->
                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">Important basis for judging whether work experience and ability match job requirements</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openWork('')">
                                    <i class="el-icon-circle-plus-outline"></i> {{ lc('wap_js_00091') }}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!-- Education experience -->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-school"></i> {{ lc('wap_00459') }}</div>
                        <!-- Loop -->
                        <div class="user_resume_show" v-for="(edu, edukey) in expectData.edu" :key="edukey">
                            <div class="user_resume_addname ">{{ edu.name }}
                                <el-button type="text" @click="openEdu(edukey)">
                                    <i class="el-icon-edit"></i> {{ lc('wap_js_00073') }}
                                </el-button>
                                <el-button type="text" @click="delResumeFb('edu', edukey, edu.id)">
                                    <i class="el-icon-delete"></i> {{ lc('common.delete') }}
                                </el-button>
                            </div>
                            <div class="user_resume_addjy">
                                <div class=" ">{{ edu.specialty }}<span class="userline"
                                        v-if="edu.specialty && edu.education_n">|</span>{{ edu.education_n }}</div>
                                <div class="user_resume_time">{{ edu.sdate_n }}-{{ edu.edate_n }}</div>
                            </div>
                        </div>
                        <!-- Loop -->
                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">Help HR understand the education background</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openEdu('')">
                                    <i class="el-icon-circle-plus-outline"></i> {{ lc('wap_js_00091') }}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!-- Training experience -->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-data-analysis"></i> {{ lc('wap_00455') }}</div>
                        <!-- Loop -->
                        <div class="user_resume_show" v-for="(training, trainingKey) in expectData.training" :key="trainingKey">
                            <div class="user_resume_addname ">{{ training.name }}
                                <el-button type="text" @click="openTraining(trainingKey)">
                                    <i class="el-icon-edit"></i> {{ lc('wap_js_00073') }}
                                </el-button>
                                <el-button type="text" @click="delResumeFb('training', trainingKey, training.id)">
                                    <i class="el-icon-delete"></i> {{ lc('common.delete') }}
                                </el-button>
                            </div>
                            <div class="user_resume_addjy">
                                <div class=" ">{{ training.title }} </div>
                                <div class="user_resume_time">{{ training.sdate_n }}-{{ training.edate_n }}</div>
                            </div>
                            <div class="user_resume_ms">{{ training.content }}</div>
                        </div>
                        <!-- Loop -->

                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">Important basis for judging whether training experience matches job requirements</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openTraining('')">
                                    <i class="el-icon-circle-plus-outline"></i> {{ lc('wap_js_00091') }}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!-- Professional skills -->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-reading"></i> {{ lc('wap_00461') }}</div>
                        <!-- Loop -->
                        <div class="user_resume_show" v-for="(skill, skillkey) in expectData.skill" :key="skillkey">
                            <div class="user_resume_addname ">{{ skill.name }}
                                <el-button type="text" @click="openSkill(skillkey)">
                                    <i class="el-icon-edit"></i> {{ lc('wap_js_00073') }}
                                </el-button>
                                <el-button type="text" @click="delResumeFb('skill', skillkey, skill.id)">
                                    <i class="el-icon-delete"></i> {{ lc('common.delete') }}
                                </el-button>
                            </div>
                            <div class="user_resume_addjy">
                                <div class=" ">{{ skill.ing_n }} </div>
                                <div class="user_resume_time">{{ lc('admin_user_00238') }}</div>
                            </div>
                            <div class="user_resume_ms" v-if="skill.pic">
                                <img :src="skill.pic" width="95" height="70" :preview-src-list="skill.pic">
                            </div>
                        </div>
                        <!-- Loop -->

                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">Add professional skills to strengthen the resume</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openSkill('')">
                                    <i class="el-icon-circle-plus-outline"></i> {{ lc('wap_js_00091') }}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!-- Project experience -->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-wallet"></i> {{ lc('wap_00465') }}</div>
                        <!-- Loop -->
                        <div class="user_resume_show" v-for="(project, projectkey) in expectData.project" :key="projectkey">
                            <div class="user_resume_addname ">{{ project.name }}
                                <el-button type="text" @click="openProject(projectkey)">
                                    <i class="el-icon-edit"></i> {{ lc('wap_js_00073') }}
                                </el-button>
                                <el-button type="text" @click="delResumeFb('project', projectkey, project.id)">
                                    <i class="el-icon-delete"></i> {{ lc('common.delete') }}
                                </el-button>
                            </div>
                            <div class="user_resume_addjy">
                                <div class=" ">{{ project.title }}</div>
                                <div class="user_resume_time">{{ project.sdate_n }}-{{ project.edate_n }}</div>
                            </div>
                            <div class="user_resume_ms">{{ project.content }}</div>
                        </div>
                        <!-- Loop -->

                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">Show work experience and ability as an important basis for HR evaluation.</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openProject('')">
                                    <i class="el-icon-circle-plus-outline"></i> {{ lc('wap_js_00091') }}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!-- Other description -->
                    <div class="user_resume_list" style="padding-bottom:80px; ;">
                        <div class="shshow_tit"><i class="el-icon-mic"></i> {{ lc('admin_00068') }}</div>
                        <!-- Loop -->
                        <div class="user_resume_show" v-for="(other, otherkey) in expectData.other" :key="otherkey">
                            <div class="user_resume_addname ">{{ other.name }}
                                <el-button type="text" @click="openOther(otherkey)">
                                    <i class="el-icon-edit"></i> {{ lc('wap_js_00073') }}
                                </el-button>
                                <el-button type="text" @click="delResumeFb('other', otherkey, other.id)">
                                    <i class="el-icon-delete"></i> {{ lc('common.delete') }}
                                </el-button>
                            </div>
                            <div class="user_resume_ms">{{ other.content }}</div>
                        </div>
                        <!-- Loop -->
                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">Other supporting details</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openOther('')">
                                    <i class="el-icon-circle-plus-outline"></i> {{ lc('wap_js_00091') }}
                                </el-button>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="shcz" style="top:60px;right:30px;">
                    <template v-if="detail.r_status == 2">
                        <div class="wxsettip_small ">{{ lc('admin_user_00251') }}</div>
                        <template>
                            <el-radio-group v-model="ruleFormAudit.r_status">
                                <el-radio label="1">{{ lc('admin_user_00149') }}</el-radio>
                                <el-radio label="2">{{ lc('admin_user_00150') }}</el-radio>
                            </el-radio-group>
                            <el-alert v-if="detail.lock_info" :closable="false" :title="lc('admin_00744') + '：' + detail.lock_info"
                                type="warning" show-icon>
                            </el-alert>
                        </template>
                    </template>
                    <template v-if="ruleFormAudit.r_status == 1">
                        <div class="wxsettip_small ">{{ lc('admin_user_00251') }}</div>
                        <template>
                            <el-radio-group v-model="ruleFormAudit.status">
                                <el-radio label="1">{{ lc('admin_user_00149') }}</el-radio>
                                <el-radio label="3">{{ lc('wap_user_00167') }}</el-radio>
                            </el-radio-group>
                        </template>
                        <div class="wxsettip_small ">{{ lc('admin_user_00244') }}</div>
                        <el-select v-model="auditTpl" :placeholder="lc('wap_user_00100')" @change="changeTpl">
                            <el-option v-for="auditkey in userdata.user_audit" :key="auditkey"
                                :label="userclass_name[auditkey]" :value="auditkey">
                            </el-option>
                        </el-select>
                        <div class="wxsettip_small ">{{ lc('member_user_00062') }}</div>
                        <el-input type="textarea" :rows="2" v-model="ruleFormAudit.statusbody">
                        </el-input>
                        <template v-if="ruleFormAudit.content">
                            <div class="wxsettip_small ">{{ lc('wap_01435') }}</div>
                            <el-input type="textarea" :rows="2" v-model="ruleFormAudit.content">
                            </el-input>
                        </template>
                        <div class=" shczbth">
                            <el-button type="primary" @click="submitAudit(1)">{{ lc('member_com_00248') }}</el-button>
                        </div>
                        <div v-if="todoAuditNum > 0" class=" shczbth">
                            <el-button type="primary" @click="submitAudit(2)" plain>{{ lc('admin_user_00239') }}</el-button>
                        </div>
                    </template>
                </div>
            </div>
        </el-drawer>

        <!-- Edit resume basic information -->
        <el-drawer :title="lc('admin_00475')" :append-to-body="true" v-model="drawerBasic" :wrapper-closable="false" size="60%">
            <div class="uploadTable" style="padding:0px 20px;">
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="120">{{ lc('member_com_00021') }}</th>
                            <th width=" ">{{ lc('member_user_00181') }}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_00529') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.name" :placeholder="lc('wap_user_00234')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_com_00303') }}</div>
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
                                <div class="TableTite">{{ lc('wap_user_00236') }}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-date-picker v-model="ruleFormBasic.birthday" type="month" :placeholder="lc('admin_user_00192')">
                                    </el-date-picker>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_user_00092') }}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormBasic.edu" :placeholder="lc('wap_user_00100')">
                                        <el-option v-for="edukey in userdata.user_edu" :key="edukey"
                                            :label="userclass_name[edukey]" :value="edukey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_user_00240') }}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormBasic.exp" :placeholder="lc('wap_user_00100')">
                                        <el-option v-for="wordkey in userdata.user_word" :key="wordkey"
                                            :label="userclass_name[wordkey]" :value="wordkey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_user_00265') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.telphone" :placeholder="lc('wap_com_00322')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_com_00016') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.email" :placeholder="lc('wap_com_00009')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_user_00173') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.idcard" :placeholder="lc('admin_00476')"
                                        @input="inputIdcard($event, 'ruleFormBasic', 'idcard')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('member_user_00158') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.domicile" :placeholder="lc('admin_00477')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_user_00230') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.living" :placeholder="lc('admin_00478')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_01362') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.address" :placeholder="lc('wap_00905')"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('member_user_00165') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.height" :placeholder="lc('admin_00479')"
                                        @input="inputFloatNumber($event, 'ruleFormBasic', 'height')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('member_user_00160') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.weight" :placeholder="lc('admin_00480')"
                                        @input="inputFloatNumber($event, 'ruleFormBasic', 'weight')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_com_00282') }}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormBasic.marriage" :placeholder="lc('wap_user_00100')">
                                        <el-option v-for="marriagekey in userdata.user_marriage" :key="marriagekey"
                                            :label="userclass_name[marriagekey]" :value="marriagekey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('member_user_00164') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.nationality" :placeholder="lc('admin_00481')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('member_user_00155') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.homepage" :placeholder="lc('admin_00482')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">QQ</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.qq" :placeholder="lc('admin_user_00217')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('resume_00003') }}</div>
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
                                <div class="TableTite">{{ lc('wap_00527') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" :placeholder="lc('admin_user_00208')"
                                        v-model="ruleFormBasic.description">
                                    </el-input>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="setBasicButn" style="border: none; height: 80px;">
                <el-button type="primary" size="medium" @click="submitBasic">{{ lc('common.submit') }}</el-button>
            </div>


        </el-drawer>
        <!-- Edit job intention -->
        <el-drawer :title="lc('admin_00483')" :append-to-body="true" v-model="drawerJob" :wrapper-closable="false" size="60%">
            <div class="uploadTable" style="padding:0px 20px;">
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="120">{{ lc('member_com_00021') }}</th>
                            <th width=" ">{{ lc('member_user_00181') }}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_user_00015') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormJob.name" :placeholder="lc('admin_00484')">
                                    </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_user_00218') }}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <!-- 7.0 unified category selector -->
                                    <job_class multiple :max="5" @confirm="confirmJob" :selected="jobSelected"></job_class>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_user_00226') }}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <!-- 7.0 unified city selector -->
                                    <city_class multiple :max="5" @confirm="confirmCity" :selected="citySelected"></city_class>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_user_00016') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt" style="max-width: 700px;">
                                    <el-select v-model="ruleFormJob.minsalary" :placeholder="lc('wap_user_00100')" @change="salaryChange" style="margin-right:8px;">
                                        <el-option v-for="maxsalary1Val in minsalaryList" :key="maxsalary1Val" :label="maxsalary1Val" :value="maxsalary1Val">
                                        </el-option>
                                    </el-select>
                                    <el-select v-model="ruleFormJob.maxsalary" :placeholder="lc('wap_user_00100')">
                                        <el-option v-for="maxsalary2Val in maxsalaryList" :key="maxsalary2Val" :label="maxsalary2Val" :value="maxsalary2Val">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_user_00010') }}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormJob.hy" :placeholder="lc('wap_user_00100')">
                                        <el-option v-for="industrykey in industry_index" :key="industrykey"
                                            :label="industry_name[industrykey]" :value="industrykey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_com_00279') }}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormJob.report" :placeholder="lc('wap_user_00100')">
                                        <el-option v-for="reportkey in userdata.user_report" :key="reportkey"
                                            :label="userclass_name[reportkey]" :value="reportkey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_user_00012') }}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormJob.type" :placeholder="lc('wap_user_00100')">
                                        <el-option v-for="typekey in userdata.user_type" :key="typekey"
                                            :label="userclass_name[typekey]" :value="typekey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('wap_user_00017') }}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormJob.jobstatus" :placeholder="lc('wap_user_00100')">
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
                <el-button type="primary" size="medium" @click="submitJob">{{ lc('common.submit') }}</el-button>
            </div>
        </el-drawer>

        <!-- Edit personal strengths -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_user_00326')" v-model="dialogTag" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00219') }}</div>
                    <div class="wxsettipBiaoqin">
                        <el-tag :key="tagkey" v-for="(tag, tagkey) in userTag" :disable-transitions="false"
                            @click="checkTag(tag)" :effect="ruleFormTag.tag.indexOf(tag) > -1 ? 'dark' : 'light'">
                            {{ tag }}
                        </el-tag>
                        <el-input class="input-new-tag" v-if="inputTag" v-model="tagval"
                            autofoucs size="small" @keyup.enter="confirmTag">
                        </el-input>
                        <el-button v-else class="button-new-tag" size="small" @click="showTag">{{ lc('admin_00474') }}
                        </el-button>
                    </div>
                    <div class="wxsettip_small ">{{ lc('wap_00463') }}</div>
                    <el-input type="textarea"
                        :placeholder="lc('admin_vue_00011')"
                        v-model="ruleFormTag.description" :autosize="{ minRows: 3, maxRows: 6 }">
                    </el-input>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogTag = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitTag">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Edit work experience -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_00457')" v-model="dialogWork" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_01403') }}</div>
                    <div class=""><el-input v-model="ruleFormWork.name" :placeholder="lc('wap_00137')"></el-input> </div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00091') }}</div>
                    <div class=""><el-input v-model="ruleFormWork.title" :placeholder="lc('wap_user_00045')"></el-input> </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00223') }}</div>
                    <div class="wxsettip_Sealect" style="display: flex; align-items: center;">
                        <el-date-picker v-model="ruleFormWork.sdate" type="month" :placeholder="lc('wap_com_00323')">
                        </el-date-picker>
                        <el-date-picker style="margin: 0 8px;" :disabled="todayCheck" v-model="ruleFormWork.edate"
                            type="month" :placeholder="lc('wap_com_00324')">
                        </el-date-picker>
                        <el-checkbox v-model="todayCheck" @change="todayChange($event, 'work')">{{ lc('wap_js_00170') }}</el-checkbox>
                    </div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00086') }}</div>
                    <el-input type="textarea" :placeholder="lc('admin_vue_00012')"
                              v-model="ruleFormWork.content" :autosize="{ minRows: 3, maxRows: 6 }">
                    </el-input>
                </div>

                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogWork = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitWork">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Edit education -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_00459')" v-model="dialogEdu" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00085') }}</div>
                    <div class=""><el-input v-model="ruleFormEdu.name" :placeholder="lc('wap_user_00044')"></el-input> </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00220') }}</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeEdu" type="monthrange" :range-separator="lc('admin_company_00019')"
                            :start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00092') }}</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="ruleFormEdu.education" :placeholder="lc('wap_user_00100')">
                            <el-option v-for="edukey in userdata.user_edu" :key="edukey" :label="userclass_name[edukey]"
                                :value="edukey">
                            </el-option>
                        </el-select>
                    </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00224') }}</div>
                    <div class=""><el-input v-model="ruleFormEdu.specialty" :placeholder="lc('wap_user_00042')"></el-input> </div>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogEdu = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitEdu">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>

        <!-- Edit training experience -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_00455')" v-model="dialogTraining" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00221') }}</div>
                    <div class=""><el-input v-model="ruleFormTraining.name" :placeholder="lc('admin_00485')"></el-input> </div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00083') }}</div>
                    <div class=""><el-input v-model="ruleFormTraining.title" :placeholder="lc('admin_user_00209')"></el-input> </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00222') }}</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeTraining" type="monthrange" :range-separator="lc('admin_company_00019')"
                            :start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00082') }}</div>
                    <el-input type="textarea" :placeholder="lc('admin_user_00200')" v-model="ruleFormTraining.content"
                        :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogTraining = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitTraining">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Edit project experience -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_00465')" v-model="dialogProject" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00099') }}</div>
                    <div class=""><el-input v-model="ruleFormProject.name" :placeholder="lc('wap_user_00046')"></el-input> </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00225') }}</div>
                    <div class=""><el-input v-model="ruleFormProject.title" :placeholder="lc('admin_00486')"></el-input> </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00229') }}</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeProject" type="monthrange" :range-separator="lc('admin_company_00019')"
                            :start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00228') }}</div>
                    <el-input type="textarea" :placeholder="lc('admin_vue_00012')" v-model="ruleFormProject.content" :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogProject = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitProject">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Edit other details -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_00216')" v-model="dialogOther" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00103') }}</div>
                    <div class=""><el-input v-model="ruleFormOther.name" :placeholder="lc('admin_00487')"></el-input> </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00231') }}</div>
                    <el-input type="textarea" v-model="ruleFormOther.content" :placeholder="lc('admin_user_00203')"
                        :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogOther = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitOther">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Edit skills -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_00461')" v-model="dialogSkill" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00089') }}</div>
                    <div class=""><el-input v-model="ruleFormSkill.name" :placeholder="lc('admin_user_00210')"></el-input> </div>
                    <div class="wxsettip_small ">{{ lc('wap_00458') }}</div>
                    <div class="wxsettip_Sealect">
                        <el-input v-model="ruleFormSkill.longtime" :placeholder="lc('admin_user_00211')">
                            <template #append>{{ lc('common_02077') }}</template>
                        </el-input>
                    </div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00094') }}</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="ruleFormSkill.ing" :placeholder="lc('wap_user_00100')">
                            <el-option v-for="ingkey in userdata.user_ing" :key="ingkey" :label="userclass_name[ingkey]"
                                :value="ingkey">
                            </el-option>
                        </el-select>
                    </div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00090') }}</div>
                    <div>
                        <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept" action="" :auto-upload="false"
                            :on-change="handleChangeSkillPic" :show-file-list="false">
                            <img v-if="ruleFormSkill.pic_n" :src="ruleFormSkill.pic_n" class="avatar">
                            <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                        </el-upload>
                    </div>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogSkill = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitSkill">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>

        <!-- Delete dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_00241')" v-model="dialogDel" :with-header="true" append-to-body :show-close="true"
                width="300px">
                <div>
                    <el-checkbox v-model="ruleFormDel.delAccount" true-label="1" false-label="0">{{ lc('admin_user_00242') }}</el-checkbox>
                </div>
                <div>
                    <i class="el-icon-warning"></i> {{ lc('admin_00508') }}
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogDel = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="delSubmit">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>

        <div class="modluDrawer">
            <!-- Resume preview -->
            <el-drawer :title="lc('wap_user_00217')" v-model="drawerPreview" append-to-body size="60%">
                <preview :id="detail.id"></preview>
            </el-drawer>
            <!-- Add resume -->
            <el-drawer :title="lc('admin_user_00193')" v-model="drawerAdd" append-to-body :wrapper-closable="false" size="45%">
                <add @child-event="closeAdd"></add>
            </el-drawer>
        </div>
    </div>
</template>

<script>
import ResumeAdd from './resume_add.vue'
import JobClass from '../../../component/job_class.vue'
import CityClass from '../../../component/city_class.vue'
import ResumePreview from '../../../component/resume_preview.vue'

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
        status: {type: String, default: ''}
    },
    data: function () {
        return {


            mouseFlag: false,
            mouseOffset: 0,



            pic_accept: localStorage.getItem("pic_accept"),
            loading: false,
			dataText: lc('admin_user_weipin_00026'),
            value: true,
            seachbutn: true,
            tableHig: true,

            // Source
            source: {},

            // Search filters
            searchList: [],
            searchForm: {
				keytype: 1,
                status: this.status,
                time_type: 'adtime',
                times: [],
			},
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
            isSearchTime: false,
            // list
            page: 1,
            limit: 0,
            list: [],
            total: 0,
            pageSizes: [],

            // {{ lc('admin_00959') }}
            t: '',
            order: '',

            checkedAll: false, // {{ lc('wap_js_00074') }}
            checkedAllIndeterminate: false,
            multipleSelection: [], // Multi-select value storage
            idArr: [],

            detail: {},
            index: "",

            resumeAllNum: 0,
            resumeStatusNum1: 0,
            resumeStatusNum2: 0,
            resumeStatusNum3: 0,
            resumeTeenNum: 0,

            saveLoading: false,

            // {{ lc('member_com_00110') }}
            dialogStatus: false,
            ruleFormStatus: {},

            // top/sticky
            dialogTop: false,
            ruleFormTop: {},

            // {{ lc('admin_user_00257') }}
            dialogExport: false,
            isIndeterminateExport: false,
            checkAllExport: false,
            typeExport: {}, // Export fields
            ruleFormExport: {
                type: [],
                limit: '',
                section: ''
            },

            // remark
            dialogRemark: false,
            ruleFormRemark: {},

            // Audit
            dialogAudit: false, // {{ lc('admin_user_weipin_00037') }}
            drawerAudit: false,
            ruleFormAudit: {},
            auditTpl: "",
            todoAuditNum: 0,
            resume: {},
            expectData: {},

            // {{ lc('common_02022') }}
            user_sex: {},
            userclass_name: {},
            userdata: {},
            industry_index: [],
            industry_name: {},

            // {{ lc('wap_user_00217') }}
            drawerPreview: false,

            // Add
            drawerAdd: false,

            // Delete
            dialogDel: false,
            ruleFormDel: {},

            expectLoading: false,

            // {{ lc('admin_00475') }}
            drawerBasic: false,
            ruleFormBasic: {},
            // personal advantage
            dialogTag: false,
            ruleFormTag: {},
            userTag: [],
            inputTag: false,
            tagval: "",
            // Job intention
            drawerJob: false,
            ruleFormJob: {},
            jobSelected: null,
            citySelected: null,
            minsalaryList: [],
            maxsalaryList: [],

            todayCheck: false, // Present selected

            // Work experience
            dialogWork: false,
            indexWork: -1,
            ruleFormWork: {},
            // Educational experience
            dialogEdu: false,
            indexEdu: -1,
            daterangeEdu: [],
            ruleFormEdu: {},
            // Training experience
            dialogTraining: false,
            indexTraining: -1,
            daterangeTraining: [],
            ruleFormTraining: {},
            // Skill improvement
            dialogSkill: false,
            indexSkill: -1,
            ruleFormSkill: {},
            // Project experience
            dialogProject: false,
            indexProject: -1,
            daterangeProject: [],
            ruleFormProject: {},
            // {{ lc('admin_00068') }}
            dialogOther: false,
            indexOther: -1,
            ruleFormOther: {},

            // {{ lc('admin_00551') }}
            drawerJobSqLog: false,
            jobSqLog: {},

            prevPage: 0,
            prevPage2: 0
        }
    },
    components: {
        'add': ResumeAdd,
        'job_class': JobClass,
        'city_class': CityClass,
        'preview': ResumePreview
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
            // The ref must point to the parent element that contains the table element
            let divData = this.$refs.multipleTable.bodyWrapper;
            if (this.mouseFlag) {
                // Set horizontal scroll position
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

        // Search job selector
        confirmJobSearch(data) {
            this.searchForm.job_class = data.jobId.join(',');
        },
        // Search city selector
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
                    that.limit = parseInt(data.limit); // Use default count from system config
                }
                if (that.page > data.page) {
                    that.page = parseInt(data.page); // Use latest page after the last page is deleted
                }
                that.loading = false;
                if(that.prevPage != that.page){
                    that.prevPage = that.page;
                    that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    scrollToTop()
                }
                if (that.list.length === 0) {
                    that.dataText = lc('wap_js_00113');
                }
            })
        },

        // Batch operation
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
                message.error(lc('admin_user_weipin_00005'));
                return false;
            }else if(this.multipleSelection.length == 0){
                message.error(lc('admin_user_weipin_00001'));
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
                }, lc('admin_vue_00020'))
            } else if (type == 'rec_cancel') {
                delConfirm(this, null, function (params) {
                    that.changeRec(0);
                }, lc('admin_vue_00021'))
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

        // Delete
        openDel(idx) {
            if (typeof idx == 'undefined') { // {{ lc('member_com_00055') }}
                this.ruleFormDel = {
                    del: this.idArr,
                    delAccount: 0
                }
            } else {// {{ lc('common_01711') }}
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

        // {{ lc('member_com_00110') }}
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
                message.warning(lc('wap_00809'));
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
        // recommend
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
                } else if (typeof row === 'undefined') { // {{ lc('admin_user_00237') }}
                    message.success(res.data.msg);
                    that.getList();
                }
            })
        },
        // top/sticky
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

            if (params.s === '1') { // {{ lc('wap_com_00231') }}
            } else { // {{ lc('wap_user_00335') }}
                if (!params.addday) {
                    message.warning(lc('wap_00976'));
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
                message.warning(lc('admin_vue_00022'));
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
        // remark
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
                message.warning(lc('admin_vue_00023'));
                return false;
            }
            if (!params.content) {
                message.warning(lc('admin_00516'));
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
        // BatchAudit
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
                message.warning(lc('admin_user_weipin_00015'));
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
        // Open review
        openAudit(row) {
            this.getAudit(row.id);
            this.drawerAudit = true;
        },
        setFormAudit() {
            let detail = this.detail;
            this.ruleFormAudit = {
                single: 1, // Single review
                id: detail.id,
                uid: detail.uid,
                r_status: detail.r_status,
                status: detail.state=='3'?'3':'1',
                statusbody: detail.statusbody,
                content: detail.content
            };
            this.auditTpl = '';
        },
        // Close review
        closedAudit() {
            if (this.refreshList) {
                this.getList();
            }
        },
        // Get details
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
        // Switch review template
        changeTpl(val) {
            this.ruleFormAudit.statusbody = this.userclass_name[val];
        },
        // Submit review
        submitAudit(atype) {
            let that = this,
                detail = that.detail,
                params = that.ruleFormAudit,
                url = 'm=user&c=users_resume&a=status';

            if (typeof params.status == 'undefined' || params.status === '' || params.status === '0') {
                message.warning(lc('admin_user_weipin_00015'));
                return false;
            }

            if (typeof params.r_status !== 'undefined') {
                if (params.r_status == 1) {
                    params.lock_status = params.r_status;
                } else {
                    message.warning(lc('admin_company_00001'));
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
                    if (typeof res.data !== 'undefined' && typeof res.data.next_id !== 'undefined') { // Review next item
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

        // {{ lc('wap_user_00217') }}
        openPreview(row) {
            this.detail = row;
            this.drawerPreview = true;
        },

        // {{ lc('admin_user_00193') }}
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

        // refreshResume
        refresh(row) {
            let that = this,
                params = {};

            if (typeof row === 'undefined') { // {{ lc('admin_user_00248') }}
                params.ids = this.idArr;
            } else { // Single refresh
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
            }, lc('admin_vue_00024'))
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

        // {{ lc('admin_user_00227') }}
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
        // Triggered during upload
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
                    // Reload details
                    that.getAudit(that.detail.id);
                    message.success(res.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },
        // personal advantage
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
                        userTag.push(item); // Append tags that are not already present
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
                    message.warning(lc('wap_user_00060'));
                    return false;
                }
                if (tag.length >= 5) {
                    message.warning(lc('admin_user_00206'));
                    return false;
                }
                if (userTag.indexOf(tagval) > -1) {
                    message.warning(lc('wap_user_00074'));
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

            if (index > -1) { // Second click cancels selection
                tag.splice(index, 1);
            } else { // First click selects
                if (tag.length >= 5) {
                    message.warning(lc('admin_user_00206'));
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
                message.warning(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.tag.length > 5) {
                message.warning(lc('admin_user_00206'));
                return false;
            }
            if (ruleForm.description == '' || ruleForm.description == null) {
                message.warning(lc('admin_01319'));
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
        // Job intention
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
                job_classid: expect.job_classid, // TODO {{ lc('admin_00300') }}
                city_classid: expect.city_classid, // TODO {{ lc('member_user_00362') }}
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
                message.warning(lc('admin_00484'));
                return false;
            }
            if (ruleForm.job_classid == "") {
                message.warning(lc('admin_vue_00013'));
                return false;
            }
            if (ruleForm.city_classid == '') {
                message.warning(lc('admin_vue_00014'));
                return false;
            }
            if (ruleForm.minsalary == "" || ruleForm.minsalary == "0") {
                message.warning(lc('admin_vue_00015'));
                return false;
            }
            if (ruleForm.maxsalary && parseInt(ruleForm.maxsalary) <= parseInt(ruleForm.minsalary)) {
                message.warning(lc('member_user_00095'));
                return false;
            }
            if (ruleForm.report == "") {
                message.warning(lc('wap_00980'));
                return false;
            }
            if (ruleForm.type == "") {
                message.warning(lc('wap_js_00163'));
                return false;
            }
            if (ruleForm.jobstatus == "") {
                message.warning(lc('wap_00934'));
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
                    // Reload details
                    that.getAudit(ruleForm.eid);
                    message.success(res.msg);
                }
            }).finally(function () {
				setTimeout(function () {
					that.saveLoading = false;
				}, 2000);
			});
        },

        // Select present
        todayChange(val, type) {
            if (type == 'work') {
                this.$set(this.ruleFormWork, 'edate', '');
            }
        },

        // Work experience
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
                message.warning(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.name == "") {
                message.warning(lc('wap_00137'));
                return false;
            }
            if (ruleForm.sdate == "") {
                message.warning(lc('admin_user_00213'));
                return false
            }
            ruleForm.sdate = formatMonth(ruleForm.sdate);
            if (ruleForm.edate != '') {
                if (ruleForm.sdate >= ruleForm.edate) {
                    message.warning(lc('admin_user_00201'));
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

                    // Build experience data locally to reduce server requests
                    if (ruleForm.id == '') {
                        let work = deepClone(ruleForm);
                        work.id = res.data.id;
                        work.sdate = 1;
                        work.sdate_n = ruleForm.sdate;
                        work.edate = ruleForm.edate != '' ? 2 : 0;
                        work.edate_n = ruleForm.edate != '' ? ruleForm.edate : lc('wap_js_00170');
                        that.expectData.work.unshift(work);
                    } else {
                        let work = that.expectData.work[indexWork];
                        work.name = ruleForm.name;
                        work.title = ruleForm.title;
                        work.sdate = 1;
                        work.sdate_n = ruleForm.sdate;
                        work.edate = ruleForm.edate != '' ? 2 : 0;
                        work.edate_n = ruleForm.edate != '' ? ruleForm.edate : lc('wap_js_00170');
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

        // Work experience
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
                    title: '', // Placeholder field with no current business meaning
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
                    title: '', // Placeholder field with no current business meaning
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
                message.warning(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.name == "") {
                message.warning(lc('wap_user_00044'));
                return false;
            }
            if (daterangeEdu.length == 0) {
                message.warning(lc('admin_vue_00016'));
                return false
            }
            if (ruleForm.education == "") {
                message.warning(lc('wap_user_00049'));
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

                    // Build experience data locally to reduce server requests
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

        // Training experience
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
                message.warning(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.name == "") {
                message.warning(lc('admin_00485'));
                return false;
            }
            if (daterangeTraining.length == 0) {
                message.warning(lc('admin_user_00212'));
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

                    // Build experience data locally to reduce server requests
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

        // Vocational skills
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
        // Triggered during upload
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
                message.warning(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.name == "") {
                message.warning(lc('admin_user_00210'));
                return false;
            }
            if (ruleForm.ing == "") {
                message.warning(lc('wap_user_00072'));
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

                    // Build experience data locally to reduce server requests
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

        // Project experience
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
                message.warning(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.name == "") {
                message.warning(lc('wap_user_00046'));
                return false;
            }
            if (daterangeProject.length == 0) {
                message.warning(lc('admin_user_00214'));
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

                    // Build experience data locally to reduce server requests
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

        // {{ lc('admin_00068') }}
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
                message.warning(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.name == "") {
                message.warning(lc('admin_00487'));
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

                    // Build experience data locally to reduce server requests
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

        // Common delete for related table data
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
            }, lc('admin_user_00204'));
        },

        // Application job records
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
                    jobSqLog.limit = parseInt(data.limit); // Use default count from system config
                }
                if (jobSqLog.page > data.page) {
                    jobSqLog.page = parseInt(data.page); // Use latest page after the last page is deleted
                }
                if(that.prevPage2 != jobSqLog.page){
                    that.prevPage2 = jobSqLog.page;
                    that.$refs.table2.bodyWrapper.scrollTop = 0;
                }
                that.jobSqLog = jobSqLog;
                that.loading = false;
                if (that.jobSqLog.list.length === 0) {
                    that.dataText = lc('wap_js_00113');
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
