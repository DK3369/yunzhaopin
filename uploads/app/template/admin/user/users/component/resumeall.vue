<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-input placeholder="{yun:}t key='admin_user_00158'{/yun}" @keyup.enter.native="search" size="small" v-model="searchForm.keyword"
                          clearable>
					<el-select v-model="searchForm.keytype" style="padding-left: 12px;" size="small" slot="prepend" placeholder="{yun:}t key='wap_user_00015'{/yun}">
					    <el-option label="{yun:}t key='wap_user_00015'{/yun}" :value="1"></el-option>
					    <el-option label="{yun:}t key='wap_00529'{/yun}" :value="2"></el-option>
					    <el-option label="{yun:}t key='member_com_00012'{/yun}" :value="3"></el-option>
					    <el-option label="{yun:}t key='admin_user_00140'{/yun}" :value="4"></el-option>
					    <el-option label="{yun:}t key='wap_01619'{/yun}" :value="5"></el-option>
					    <el-option label="{yun:}t key='wap_00459'{/yun}" :value="6"></el-option>
					    <el-option label="{yun:}t key='wap_00457'{/yun}" :value="7"></el-option>
					    <el-option label="{yun:}t key='wap_00465'{/yun}" :value="8"></el-option>
					    <el-option label="{yun:}t key='wap_00455'{/yun}" :value="9"></el-option>
					    <el-option label="{yun:}t key='wap_00461'{/yun}" :value="10"></el-option>
					    <el-option label="IP" :value="11"></el-option>
					</el-select>
                </el-input>
            </div>
            <!--收起部分-->
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="searchForm.time_type" size="small" slot="prepend" placeholder="{yun:}t key='admin_user_00135'{/yun}" clearable @change="handleTimeChange">
                    <el-option label="{yun:}t key='member_com_00087'{/yun}" value="adtime"></el-option>
                    <el-option label="{yun:}t key='wap_00326'{/yun}" value="uptime"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmalltwo" :class="{ 'searchbutnOnff': seachbutn }">
                <el-date-picker v-model="searchForm.times" type="daterange" align="right" unlink-panels range-separator="{yun:}t key='admin_company_00019'{/yun}" start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}" :picker-options="timeOptions" value-format="yyyy-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
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
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" plain icon="el-icon-plus" size="mini" @click="openAdd">{yun:}t key='admin_user_00193'{/yun}</el-button>

            </div>
            <div class="tableSeachInpt tableSeachzk" :class="{ 'searchbutnKai': seachbutn }">
                <el-button type="info" class="zhankai" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>{yun:}t key='admin_user_00145'{/yun}<i class="el-icon-arrow-down el-icon--right"></i>
                </el-button>
                <el-button type="info" class="shouqi" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>{yun:}t key='admin_user_00144'{/yun}<i class="el-icon-arrow-up el-icon--right"></i>
                </el-button>
            </div>
        </div>
        <div class="admin_datatip">
            <i class="el-icon-document"></i> {{ lc("admin_data_stats") }} <span class="cp_n" @click="init">{{ lc("admin_total_count", [resumeAllNum]) }}</span>
            <span class="admin_datatip_n"><span class="cp_n" @click="statusSearch('4')">{{ lc("admin_pending_review_count", [resumeStatusNum1 ? resumeStatusNum1 : 0]) }}</span></span>
            <span class="admin_datatip_n"><span class="cp_n" @click="statusSearch('3')">{{ lc("admin_failed_count", [resumeStatusNum2 ? resumeStatusNum2 : 0]) }}</span></span>
            <span class="admin_datatip_n"><span class="cp_n" @click="statusSearch('2')">{{ lc("admin_locked_count", [resumeStatusNum3 ? resumeStatusNum3 : 0]) }}</span></span>
            <span class="admin_datatip_n">{yun:}t key='admin_00495'{/yun}<span class="cp_n" @click="statusSearch('1')">{{ resumeTeenNum ? resumeTeenNum : 0 }}</span> {yun:}t key='common_02088'{/yun}</span>
            <span class="admin_datatip_n">{{ lc("admin_search_results_count", [total]) }}</span>
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
                <el-table-column prop="id" label="{yun:}t key='member_com_00012'{/yun}" width="90" sortable="custom"></el-table-column>
                <el-table-column label="{yun:}t key='admin_user_00243'{/yun}" min-width="165">
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
                <el-table-column label="{yun:}t key='wap_00456'{/yun}" min-width="230">
                    <template slot-scope="scope">
                        <div class=" ">
                            <div>
                                <span class="user_resumejob" @click="openPreview(scope.row)">{{ scope.row.name }}</span>
                               <!-- <span v-if="scope.row.defaults == 1" class="user_resumrmr">默认</span>-->
                            </div>
                            <div class="">
                                {yun:}t key='admin_00496'{/yun}
                                <span v-if="scope.row.edu_n">{yun:}t key='admin_00497'{/yun}</span>
                                <span v-if="scope.row.exp_n">{yun:}t key='admin_00498'{/yun}</span>
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

                <el-table-column label="{yun:}t key='admin_00509'{/yun}" min-width="110" align="center">
                    <template slot-scope="scope">
                        <el-tag type="danger" size="small" effect="dark" v-if="scope.row.integrity < 65">{{ scope.row.integrity }}%</el-tag>
						<el-tag type="success" size="small" effect="dark" v-else>{{ scope.row.integrity }}%</el-tag>

                        <div v-if="scope.row.status == 1" class="jlzt">
                            <el-button type="text" @click="openStatus(scope.row)"><i class="el-icon-unlock"></i> {yun:}t key='wap_js_00005'{/yun}
                            </el-button>
                        </div>
                        <div v-else-if="scope.row.status == 3" class="jlzt">
                            <el-button type="text" @click="openStatus(scope.row)"><i class="el-icon-unlock"></i> {yun:}t key='admin_user_00249'{/yun}
                            </el-button>
                        </div>
                        <div v-else class="jlwgk jlzt">
                            <el-button type="text" @click="openStatus(scope.row)"><i class="el-icon-lock"></i> {yun:}t key='admin_user_00253'{/yun}
                            </el-button>
                        </div>
                    </template>

                </el-table-column>
                <el-table-column label="{yun:}t key='admin_00510'{/yun}" width="80" align="center">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            {{ scope.row.sq_num ? scope.row.sq_num : 0 }}
                        </div>
                        <div class="moduleProps" v-if="scope.row.sq_num > 0">
                            <span class="jobtj">
                                <el-button type="text" @click="openJobSqlLog(scope.row)">{yun:}t key='wap_com_00427'{/yun}</el-button>
                            </span>
                        </div>
                    </template>

                </el-table-column>
				<el-table-column prop="comd" label="{yun:}t key='member_com_00110'{/yun}" min-width="130">
				    <template slot-scope="scope">
				        <div class="job_tg_bth">
				            <el-switch v-model="scope.row.rec_resume" inactive-text="{yun:}t key='wap_01465'{/yun}" :width="30" active-value="1"
				                inactive-value="0" @change="changeRec($event, scope.row)">
				            </el-switch>
				        </div>
				        <div class="job_tg_bth jobBthChufa">
				            <!--因点击switch会触发值的改变，固需遮罩层触发事件-->
				            <div class="chufaButn" @click="openTop(scope.row)">{yun:}t key='admin_00499'{/yun}</div>
				            <el-switch v-model="scope.row.top_day" inactive-text="{yun:}t key='wap_user_00335'{/yun}" :width="30"
				                :active-value="scope.row.top_day > 0 ? scope.row.top_day : 1"
				                inactive-value="0"></el-switch>
				        </div>
				    </template>
				</el-table-column>
                <el-table-column prop="logintime" label="{yun:}t key='admin_vue_00019'{/yun}" min-width="150">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span class="gsd">{{ scope.row.lastupdate_n }}</span>
                            <span>{{ scope.row.ctime_n }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="ip" label="{yun:}t key='admin_00512'{/yun}" min-width="150">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span>{{ source[scope.row.source] }}<span v-if="scope.row.doc == 1">{yun:}t key='admin_user_00258'{/yun}</span></span>
                            <span v-if="scope.row.add_ip">{{ scope.row.add_ip }}</span>
                            <span v-if="scope.row.ip_address" class="gsd"> {{ scope.row.ip_address }}</span>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column prop="zt" label="{yun:}t key='member_user_00181'{/yun}" width="120" fixed="right">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <div v-if="scope.row.r_status == '2'">
                                <span class="admin_state3">{yun:}t key='admin_user_00138'{/yun}</span>
                                <div style="display:inline-block" v-if="scope.row.lock_info">
                                    <el-popover trigger="hover" placement="right">
                                        <p>{{ scope.row.lock_info }}</p>
                                        <div slot="reference" class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div>
                                    </el-popover>
                                </div>
                            </div>
                            <span v-else-if="scope.row.state == 1" class="admin_state1">{yun:}t key='wap_user_00165'{/yun}</span>
                            <span v-else-if="scope.row.state == 3" class="admin_state2">
                                {yun:}t key='wap_user_00167'{/yun}
                                <el-tooltip effect="dark" :content="scope.row.statusbody" placement="top"
                                    v-if="scope.row.statusbody">
                                    <i class="el-icon-warning-outline"></i>
                                </el-tooltip>
                            </span>
                            <span v-else-if="scope.row.state == 2" class="admin_state3">{yun:}t key='admin_user_00255'{/yun}</span>
                            <span v-else class="admin_state5">{yun:}t key='wap_user_00166'{/yun}</span>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="140" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="openAudit(scope.row)">{yun:}t key='member_user_00152'{/yun}</el-button>

                            <el-button size="mini" plain @click="refresh(scope.row)">{yun:}t key='wap_user_00334'{/yun}</el-button>
                        </div>
                        <div class="cz_button" style="margin-top: 10px;">
                            <el-button size="mini" plain @click="openRemark(scope.row)">{yun:}t key='member_user_00242'{/yun}</el-button>

                            <el-button type="danger" size="mini" @click="openDel(scope.$index)">{yun:}t key='common.delete'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging" style="height: initial; flex-wrap: wrap; padding-top: 10px;">
            <div class="bottomButnBull" style="width:100%;">
                <div class="bottomButnBlak">
                    <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
                    @change="checkAll">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                    <el-button size="mini" @click="batch('del')">{yun:}t key='member_com_00055'{/yun}</el-button>
                    <el-button size="mini" @click="batch('audit')">{yun:}t key='admin_user_weipin_00037'{/yun}</el-button>
                    <el-button size="mini" @click="batch('refresh')">{yun:}t key='admin_user_00248'{/yun}</el-button>
                    <el-button size="mini" @click="batch('rec')">{yun:}t key='admin_user_00237'{/yun}</el-button>
                    <el-button size="mini" @click="batch('rec_cancel')">{yun:}t key='wap_com_00230'{/yun}</el-button>
                    <el-button size="mini" @click="batch('top')">{yun:}t key='admin_00500'{/yun}</el-button>
                    <el-button size="mini" @click="batch('top_cancel')">{yun:}t key='wap_com_00231'{/yun}</el-button>
                    <el-button size="mini" @click="batch('export')">{yun:}t key='admin_user_00257'{/yun}</el-button>
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
            <el-dialog title="{yun:}t key='admin_user_00246'{/yun}" :visible.sync="dialogExport" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="650px">
                <div class="tck_setname">
                    <el-checkbox-group v-model="ruleFormExport.type" @change="handleCheckedExportType">
                        <el-checkbox :label="field" v-for="(fieldName, field) in typeExport" :key="field">{{ fieldName }}</el-checkbox>
                    </el-checkbox-group>
                    <el-checkbox :indeterminate="isIndeterminateExport" v-model="checkAllExport"
                        @change="handleCheckAllExport">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                </div>
                <div class="daochuNumer">
                    <div class="daochuTite">
                        <span>{yun:}t key='admin_00501'{/yun}</span>
                    </div>
                    <div class="daochuFrom">
                        <div class="daochuFroInpt">
                            <el-input v-model="ruleFormExport.limit"
                                @input="inputIntNumber($event, 'ruleFormExport', 'limit')"></el-input>
                        </div>
                        <div>
                            <el-alert :closable="false" title="{yun:}t key='admin_00513'{/yun}" type="info" show-icon>
                            </el-alert>
                        </div>
                    </div>

                    <!-- <span>
                        <el-input v-model="ruleFormExport.limit"
                            @input="inputIntNumber($event, 'ruleFormExport', 'limit')"></el-input>
                    </span>
                    <el-alert :closable="false" title="{yun:}t key='admin_00513'{/yun}" type="info" show-icon>
                    </el-alert> -->
                </div>
                <div class="daochuNumer">
                    <div class="daochuTite">
                        <span>{yun:}t key='admin_00502'{/yun}</span>
                    </div>
                    <div class="daochuFrom">
                        <div class="daochuFroInpt">
                            <el-input v-model="ruleFormExport.section"></el-input>
                        </div>
                        <div>
                            <el-alert :closable="false" title="{yun:}t key='admin_00514'{/yun}" type="info" show-icon>
                            </el-alert>
                        </div>
                    </div>

                    <!-- <span><el-input v-model="ruleFormExport.section"></el-input></span>
                    <el-alert :closable="false" title="{yun:}t key='admin_00514'{/yun}" type="info" show-icon>
                    </el-alert> -->
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogExport = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitExport" :disabled="saveLoading">{yun:}t key='admin_user_00254'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--公开简历-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='member_com_00110'{/yun}" :visible.sync="dialogStatus" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div class="wxsettip_small ">{yun:}t key='wap_00529'{/yun}</div>
                <el-input :value="detail.uname" :disabled="true"></el-input>
                <div class="wxsettip_small ">{yun:}t key='member_com_00110'{/yun}</div>
                <div class="wxsettip_Sealect">
                    <el-select v-model="ruleFormStatus.status" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                        <el-option key="1" label="{yun:}t key='wap_js_00005'{/yun}" value="1"></el-option>
                        <el-option key="3" label="{yun:}t key='admin_00515'{/yun}" value="3"></el-option>
                        <el-option key="2" label="{yun:}t key='admin_user_00259'{/yun}" value="2"></el-option>
                    </el-select>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogStatus = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitStatus" :disabled="saveLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--简历置顶-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_user_00207'{/yun}" :visible.sync="dialogTop" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div class="wxsettip_small ">{yun:}t key='wap_user_00209'{/yun}</div>
                <el-input v-model="ruleFormTop.addday" @input="inputIntNumber($event, 'ruleFormTop', 'addday')">
                    <template slot="append">{yun:}t key='common_02067'{/yun}</template>
                </el-input>
                <template v-if="detail.top_day > 0">
                    <div class="danqainDataFlex">
                        <div class="wxsettip_small ">{yun:}t key='admin_00503'{/yun}</div>
                        <div style="color:#f60">{{ detail.topdate_n }}</div>
                    </div>

                </template>
                <div>
                    {yun:}t key='admin_00504'{/yun} <el-checkbox v-model="ruleFormTop.s" true-label="1" false-label="0"></el-checkbox> {yun:}t key='admin_00505'{/yun}
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogTop = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitTop" :disabled="saveLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--简历备注-->
        <div class=" ">
            <el-dialog title="{yun:}t key='wap_com_00070'{/yun}" :visible.sync="dialogRemark" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div class="wxsettip_small ">{yun:}t key='admin_00506'{/yun}</div>
                <div class="wxsettip_Sealect">
                    <el-select v-model="ruleFormRemark.label" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                        <el-option v-for="labelkey in userdata.user_label" :key="labelkey" :label="userclass_name[labelkey]"
                            :value="labelkey">
                        </el-option>
                    </el-select>
                </div>
                <div class="wxsettip_small ">{yun:}t key='admin_00507'{/yun}</div>
                <el-input v-model="ruleFormRemark.content" type="textarea" placeholder="{yun:}t key='admin_00516'{/yun}"></el-input>

                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogRemark = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitRemark">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>

        <!--投递记录-->
        <el-drawer title="{yun:}t key='admin_00517'{/yun}" :append-to-body="true" :visible.sync="drawerJobSqLog" size="80%">
            <div class="uploadTable" style="padding:0px 20px;font-size:14px;color:#666">
                <div class="moduleElHight">
                    <div class="moduleElTable moduleElMoreInt" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
                        <el-table :data="jobSqLog.list" style="width: 100%" stripe ref="table2"
                            :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading">
                            <template slot="empty">
                                <p>{{dataText}}</p>
                            </template>
                            <el-table-column prop="job_name" label="{yun:}t key='wap_01596'{/yun}">
                                <template slot-scope="scope">
                                    <div class="moduleProps">
                                        <el-link type="primary" :underline="false"
                                            @click="openPage(scope.row.job_comapply)">{{ scope.row.job_name }}</el-link>
                                    </div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="com_name" label="{yun:}t key='admin_user_00247'{/yun}">
                                <template slot-scope="scope">
                                    <div class="moduleProps">
                                        <el-link type="primary" :underline="false"
                                            @click="openPage(scope.row.company_show)">{{ scope.row.com_name }}</el-link>
                                    </div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="datetime_n_n" label="{yun:}t key='member_user_00431'{/yun}"></el-table-column>
                            <el-table-column label="{yun:}t key='admin_user_00250'{/yun}">
                                <template slot-scope="scope">
                                    <div class="admin_state">
                                        <span class="admin_state1" v-if="scope.row.is_browse == 2">{yun:}t key='wap_user_00258'{/yun}</span>
                                        <span class="admin_state2" v-else-if="scope.row.is_browse == 3">{yun:}t key='admin_user_00252'{/yun}</span>
                                        <span class="admin_state3" v-else-if="scope.row.is_browse == 4">{yun:}t key='wap_user_00354'{/yun}</span>
                                        <span class="admin_state4" v-else-if="scope.row.is_browse == 5">{yun:}t key='member_com_00108'{/yun}</span>
                                        <span class="admin_state5" v-else>{yun:}t key='wap_user_00260'{/yun}</span>
                                    </div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="isdel_n" label="{yun:}t key='member_user_00181'{/yun}"></el-table-column>
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
        <el-dialog title="{yun:}t key='admin_user_weipin_00037'{/yun}" :visible.sync="dialogAudit" :modal-append-to-body="false" :show-close="true" width="500px">
            <div class="toolClasDia fenpeizhand">
                <div class="toolClasList">
                    <div class="toolClasTite">
                        <span>{yun:}t key='admin_user_weipin_00065'{/yun}</span>
                    </div>
                    <div class="toolClasCont">
                        <el-radio v-model="ruleFormAudit.status" label="1">{yun:}t key='admin_user_00149'{/yun}</el-radio>
                        <el-radio v-model="ruleFormAudit.status" label="3">{yun:}t key='wap_user_00167'{/yun}</el-radio>
                    </div>
                </div>
                <div class="toolClasList">
                    <div class="toolClasTite">
                        <span>{yun:}t key='member_user_00450'{/yun}</span>
                    </div>
                    <div class="toolClasCont">
                        <el-input type="textarea" :rows="2" placeholder="" v-model="ruleFormAudit.statusbody">
                        </el-input>
                    </div>
                </div>
            </div>
            <span slot="footer" class="dialog-footer">
                <el-button @click="dialogAudit = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                <el-button type="primary" @click="submitBatchAudit">{yun:}t key='wap_com_00019'{/yun}</el-button>
            </span>
        </el-dialog>
        <!--简历审核-->
        <el-drawer title="{yun:}t key='member_com_00028'{/yun}" :visible.sync="drawerAudit" @closed="closedAudit"
            :modal-append-to-body="false" size="90%" :append-to-body="true">
            <div class="shbox" style="padding-right: 380px;;" v-loading="expectLoading">
                <div style="overflow-y: auto;position: relative;height: 100%; padding-right: 25px; border-right: 1px solid #eee;">
                    <div class="shshow_tit">
                        <i class="el-icon-office-building"></i> {yun:}t key='wap_user_00341'{/yun}
                        <span class="shshow_cz">
                            <el-button type="text" @click="openBasic">
                                <i class="el-icon-edit"></i>{yun:}t key='admin_user_00227'{/yun}
                            </el-button>
                        </span>
                    </div>
                    <div class="userinfo_box">
                        <div class="userinfo_l"><img :src="resume.photo" width="70" height="70"></div>
                        <div class="userinfo_r">
                            <div class="userinfo_name">{{ resume.name }}</div>
                            <div class="userinfo">
                                {{ resume.sex_n }}
                                <span v-if="resume.age">{yun:}t key='admin_user_00198'{/yun}</span>
                                <span v-if="resume.height">，{{ resume.height }}cm</span>
                                <span v-if="resume.weight">，{{ resume.weight }}kg</span>
                                <span v-if="resume.marriage_n">，{{ resume.marriage_n }}</span>
                                <span v-if="resume.living">{yun:}t key='admin_00468'{/yun}</span>
                            </div>
                            <div class="userinfo" v-if="resume.edu_n || resume.exp_n">
                                <span v-if="resume.edu_n">{yun:}t key='admin_00469'{/yun} </span>
                                <span class="userline" v-if="resume.edu_n && resume.exp_n">|</span>
                                <span v-if="resume.exp_n">{yun:}t key='admin_00470'{/yun}</span>
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

                    <!--个人优势-->
                    <div class="user_resume_list">
                        <div class="shshow_tit">
                            <i class="el-icon-medal-1"></i> {yun:}t key='wap_user_00326'{/yun}
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
                                        resume.arrayTag.length > 0) || resume.description ? '{yun:}t key='common.edit'{/yun}' : '{yun:}t key='wap_js_00091'{/yun}' }}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!--求职意向-->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-notebook-2"></i> {yun:}t key='wap_00460'{/yun}</div>
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
                            <!-- <div class="">建议完善求职偏好</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openJob">
                                    <i class="el-icon-circle-plus-outline"></i> {yun:}t key='admin_00472'{/yun}
                                </el-button>
                            </div>
                        </div>
                    </div>

                    <!--工作经历-->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-suitcase-1"></i> {yun:}t key='wap_00457'{/yun}</div>
                        <!--循环-->
                        <div class="user_resume_show" v-for="(work, workkey) in expectData.work" :key="workkey">
                            <div class="user_resume_addname ">{{ work.name }}
                                <el-button type="text" @click="openWork(workkey)">
                                    <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                                </el-button>
                                <el-button type="text" @click="delResumeFb('work', workkey, work.id)">
                                    <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
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
                                    <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!--教育经历-->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-school"></i> {yun:}t key='wap_00459'{/yun}</div>
                        <!--循环-->
                        <div class="user_resume_show" v-for="(edu, edukey) in expectData.edu" :key="edukey">
                            <div class="user_resume_addname ">{{ edu.name }}
                                <el-button type="text" @click="openEdu(edukey)">
                                    <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                                </el-button>
                                <el-button type="text" @click="delResumeFb('edu', edukey, edu.id)">
                                    <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
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
                                    <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!--培训经历-->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-data-analysis"></i> {yun:}t key='wap_00455'{/yun}</div>
                        <!--循环-->
                        <div class="user_resume_show" v-for="(training, trainingKey) in expectData.training" :key="trainingKey">
                            <div class="user_resume_addname ">{{ training.name }}
                                <el-button type="text" @click="openTraining(trainingKey)">
                                    <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                                </el-button>
                                <el-button type="text" @click="delResumeFb('training', trainingKey, training.id)">
                                    <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
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
                                    <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!--职业技能-->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-reading"></i> {yun:}t key='wap_00461'{/yun}</div>
                        <!--循环-->
                        <div class="user_resume_show" v-for="(skill, skillkey) in expectData.skill" :key="skillkey">
                            <div class="user_resume_addname ">{{ skill.name }}
                                <el-button type="text" @click="openSkill(skillkey)">
                                    <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                                </el-button>
                                <el-button type="text" @click="delResumeFb('skill', skillkey, skill.id)">
                                    <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
                                </el-button>
                            </div>
                            <div class="user_resume_addjy">
                                <div class=" ">{{ skill.ing_n }} </div>
                                <div class="user_resume_time">{yun:}t key='admin_user_00238'{/yun}</div>
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
                                    <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!--项目经历-->
                    <div class="user_resume_list">
                        <div class="shshow_tit"><i class="el-icon-wallet"></i> {yun:}t key='wap_00465'{/yun}</div>
                        <!--循环-->
                        <div class="user_resume_show" v-for="(project, projectkey) in expectData.project" :key="projectkey">
                            <div class="user_resume_addname ">{{ project.name }}
                                <el-button type="text" @click="openProject(projectkey)">
                                    <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                                </el-button>
                                <el-button type="text" @click="delResumeFb('project', projectkey, project.id)">
                                    <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
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
                                    <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                                </el-button>
                            </div>
                        </div>
                    </div>
                    <!--其他描述-->
                    <div class="user_resume_list" style="padding-bottom:80px; ;">
                        <div class="shshow_tit"><i class="el-icon-mic"></i> {yun:}t key='admin_00068'{/yun}</div>
                        <!--循环-->
                        <div class="user_resume_show" v-for="(other, otherkey) in expectData.other" :key="otherkey">
                            <div class="user_resume_addname ">{{ other.name }}
                                <el-button type="text" @click="openOther(otherkey)">
                                    <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                                </el-button>
                                <el-button type="text" @click="delResumeFb('other', otherkey, other.id)">
                                    <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
                                </el-button>
                            </div>
                            <div class="user_resume_ms">{{ other.content }}</div>
                        </div>
                        <!--循环-->
                        <div class="user_resume_add userEsumeAdds">
                            <!-- <div class="">其他加分补充</div> -->
                            <div class="user_resume_addbth">
                                <el-button type="primary" size="small" style="width:150px" @click="openOther('')">
                                    <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                                </el-button>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="shcz" style="top:60px;right:30px;">
                    <template v-if="detail.r_status == 2">
                        <div class="wxsettip_small ">{yun:}t key='admin_user_00251'{/yun}</div>
                        <template>
                            <el-radio-group v-model="ruleFormAudit.r_status">
                                <el-radio label="1">{yun:}t key='admin_user_00149'{/yun}</el-radio>
                                <el-radio label="2">{yun:}t key='admin_user_00150'{/yun}</el-radio>
                            </el-radio-group>
                            <el-alert v-if="detail.lock_info" :closable="false" :title="lc('admin_00744') + '：' + detail.lock_info"
                                type="warning" show-icon>
                            </el-alert>
                        </template>
                    </template>
                    <template v-if="ruleFormAudit.r_status == 1">
                        <div class="wxsettip_small ">{yun:}t key='admin_user_00251'{/yun}</div>
                        <template>
                            <el-radio-group v-model="ruleFormAudit.status">
                                <el-radio label="1">{yun:}t key='admin_user_00149'{/yun}</el-radio>
                                <el-radio label="3">{yun:}t key='wap_user_00167'{/yun}</el-radio>
                            </el-radio-group>
                        </template>
                        <div class="wxsettip_small ">{yun:}t key='admin_user_00244'{/yun}</div>
                        <el-select v-model="auditTpl" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="changeTpl">
                            <el-option v-for="auditkey in userdata.user_audit" :key="auditkey"
                                :label="userclass_name[auditkey]" :value="auditkey">
                            </el-option>
                        </el-select>
                        <div class="wxsettip_small ">{yun:}t key='member_user_00062'{/yun}</div>
                        <el-input type="textarea" :rows="2" v-model="ruleFormAudit.statusbody">
                        </el-input>
                        <template v-if="ruleFormAudit.content">
                            <div class="wxsettip_small ">{yun:}t key='wap_01435'{/yun}</div>
                            <el-input type="textarea" :rows="2" v-model="ruleFormAudit.content">
                            </el-input>
                        </template>
                        <div class=" shczbth">
                            <el-button type="primary" @click="submitAudit(1)">{yun:}t key='member_com_00248'{/yun}</el-button>
                        </div>
                        <div v-if="todoAuditNum > 0" class=" shczbth">
                            <el-button type="primary" @click="submitAudit(2)" plain>{yun:}t key='admin_user_00239'{/yun}</el-button>
                        </div>
                    </template>
                </div>
            </div>
        </el-drawer>

        <!---编辑简历 基本资料-->
        <el-drawer title="{yun:}t key='admin_00475'{/yun}" :append-to-body="true" :visible.sync="drawerBasic" :wrapper-closable="false" size="60%">
            <div class="uploadTable" style="padding:0px 20px;">
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="120">{yun:}t key='member_com_00021'{/yun}</th>
                            <th width=" ">{yun:}t key='member_user_00181'{/yun}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_00529'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.name" placeholder="{yun:}t key='wap_user_00234'{/yun}"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_com_00303'{/yun}</div>
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
                                <div class="TableTite">{yun:}t key='wap_user_00236'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-date-picker v-model="ruleFormBasic.birthday" type="month" placeholder="{yun:}t key='admin_user_00192'{/yun}">
                                    </el-date-picker>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_user_00092'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormBasic.edu" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                        <el-option v-for="edukey in userdata.user_edu" :key="edukey"
                                            :label="userclass_name[edukey]" :value="edukey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_user_00240'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormBasic.exp" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                        <el-option v-for="wordkey in userdata.user_word" :key="wordkey"
                                            :label="userclass_name[wordkey]" :value="wordkey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_user_00265'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.telphone" placeholder="{yun:}t key='wap_com_00322'{/yun}"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_com_00016'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.email" placeholder="{yun:}t key='wap_com_00009'{/yun}"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_user_00173'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.idcard" placeholder="{yun:}t key='admin_00476'{/yun}"
                                        @input="inputIdcard($event, 'ruleFormBasic', 'idcard')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='member_user_00158'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.domicile" placeholder="{yun:}t key='admin_00477'{/yun}"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_user_00230'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.living" placeholder="{yun:}t key='admin_00478'{/yun}"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_01362'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.address" placeholder="{yun:}t key='wap_00905'{/yun}"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='member_user_00165'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.height" placeholder="{yun:}t key='admin_00479'{/yun}"
                                        @input="inputFloatNumber($event, 'ruleFormBasic', 'height')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='member_user_00160'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.weight" placeholder="{yun:}t key='admin_00480'{/yun}"
                                        @input="inputFloatNumber($event, 'ruleFormBasic', 'weight')"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_com_00282'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormBasic.marriage" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                        <el-option v-for="marriagekey in userdata.user_marriage" :key="marriagekey"
                                            :label="userclass_name[marriagekey]" :value="marriagekey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='member_user_00164'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.nationality" placeholder="{yun:}t key='admin_00481'{/yun}"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='member_user_00155'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.homepage" placeholder="{yun:}t key='admin_00482'{/yun}"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">QQ</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormBasic.qq" placeholder="{yun:}t key='admin_user_00217'{/yun}"> </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='resume_00003'{/yun}</div>
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
                                <div class="TableTite">{yun:}t key='wap_00527'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" placeholder="{yun:}t key='admin_user_00208'{/yun}"
                                        v-model="ruleFormBasic.description">
                                    </el-input>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="setBasicButn" style="border: none; height: 80px;">
                <el-button type="primary" size="medium" @click="submitBasic">{yun:}t key='common.submit'{/yun}</el-button>
            </div>


        </el-drawer>
        <!---编辑求职意向-->
        <el-drawer title="{yun:}t key='admin_00483'{/yun}" :append-to-body="true" :visible.sync="drawerJob" :wrapper-closable="false" size="60%">
            <div class="uploadTable" style="padding:0px 20px;">
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="120">{yun:}t key='member_com_00021'{/yun}</th>
                            <th width=" ">{yun:}t key='member_user_00181'{/yun}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_user_00015'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormJob.name" placeholder="{yun:}t key='admin_00484'{/yun}">
                                    </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_user_00218'{/yun}</div>
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
                                <div class="TableTite">{yun:}t key='admin_user_00226'{/yun}</div>
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
                                <div class="TableTite">{yun:}t key='wap_user_00016'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt" style="max-width: 700px;">
                                    <el-select v-model="ruleFormJob.minsalary" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="salaryChange" style="margin-right:8px;">
                                        <el-option v-for="maxsalary1Val in minsalaryList" :key="maxsalary1Val" :label="maxsalary1Val" :value="maxsalary1Val">
                                        </el-option>
                                    </el-select>
                                    <el-select v-model="ruleFormJob.maxsalary" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                        <el-option v-for="maxsalary2Val in maxsalaryList" :key="maxsalary2Val" :label="maxsalary2Val" :value="maxsalary2Val">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_user_00010'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormJob.hy" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                        <el-option v-for="industrykey in industry_index" :key="industrykey"
                                            :label="industry_name[industrykey]" :value="industrykey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_com_00279'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormJob.report" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                        <el-option v-for="reportkey in userdata.user_report" :key="reportkey"
                                            :label="userclass_name[reportkey]" :value="reportkey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_user_00012'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormJob.type" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                        <el-option v-for="typekey in userdata.user_type" :key="typekey"
                                            :label="userclass_name[typekey]" :value="typekey">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='wap_user_00017'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormJob.jobstatus" placeholder="{yun:}t key='wap_user_00100'{/yun}">
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
                <el-button type="primary" size="medium" @click="submitJob">{yun:}t key='common.submit'{/yun}</el-button>
            </div>
        </el-drawer>

        <!---编辑个人优势-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_user_00326'{/yun}" :visible.sync="dialogTag" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00219'{/yun}</div>
                    <div class="wxsettipBiaoqin">
                        <el-tag :key="tagkey" v-for="(tag, tagkey) in userTag" :disable-transitions="false"
                            @click="checkTag(tag)" :effect="ruleFormTag.tag.indexOf(tag) > -1 ? 'dark' : 'light'">
                            {{ tag }}
                        </el-tag>
                        <el-input class="input-new-tag" v-if="inputTag" v-model="tagval"
                            autofoucs size="small" @keyup.enter.native="confirmTag">
                        </el-input>
                        <el-button v-else class="button-new-tag" size="small" @click="showTag">{yun:}t key='admin_00474'{/yun}
                        </el-button>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_00463'{/yun}</div>
                    <el-input type="textarea"
                        :placeholder="{yun:}t key='admin_vue_00011'{/yun}"
                        v-model="ruleFormTag.description" :autosize="{ minRows: 3, maxRows: 6 }">
                    </el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogTag = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitTag">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑工作经历-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_00457'{/yun}" :visible.sync="dialogWork" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_01403'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormWork.name" placeholder="{yun:}t key='wap_00137'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00091'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormWork.title" placeholder="{yun:}t key='wap_user_00045'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00223'{/yun}</div>
                    <div class="wxsettip_Sealect" style="display: flex; align-items: center;">
                        <el-date-picker v-model="ruleFormWork.sdate" type="month" placeholder="{yun:}t key='wap_com_00323'{/yun}">
                        </el-date-picker>
                        <el-date-picker style="margin: 0 8px;" :disabled="todayCheck" v-model="ruleFormWork.edate"
                            type="month" placeholder="{yun:}t key='wap_com_00324'{/yun}">
                        </el-date-picker>
                        <el-checkbox v-model="todayCheck" @change="todayChange($event, 'work')">{yun:}t key='wap_js_00170'{/yun}</el-checkbox>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00086'{/yun}</div>
                    <el-input type="textarea" :placeholder="{yun:}t key='admin_vue_00012'{/yun}"
                              v-model="ruleFormWork.content" :autosize="{ minRows: 3, maxRows: 6 }">
                    </el-input>
                </div>

                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogWork = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitWork">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑学历-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_00459'{/yun}" :visible.sync="dialogEdu" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00085'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormEdu.name" placeholder="{yun:}t key='wap_user_00044'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00220'{/yun}</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeEdu" type="monthrange" range-separator="{yun:}t key='admin_company_00019'{/yun}"
                            start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00092'{/yun}</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="ruleFormEdu.education" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                            <el-option v-for="edukey in userdata.user_edu" :key="edukey" :label="userclass_name[edukey]"
                                :value="edukey">
                            </el-option>
                        </el-select>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00224'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormEdu.specialty" placeholder="{yun:}t key='wap_user_00042'{/yun}"></el-input> </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogEdu = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitEdu">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>

        <!---编辑培训经历-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_00455'{/yun}" :visible.sync="dialogTraining" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00221'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormTraining.name" placeholder="{yun:}t key='admin_00485'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00083'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormTraining.title" placeholder="{yun:}t key='admin_user_00209'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00222'{/yun}</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeTraining" type="monthrange" range-separator="{yun:}t key='admin_company_00019'{/yun}"
                            start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00082'{/yun}</div>
                    <el-input type="textarea" placeholder="{yun:}t key='admin_user_00200'{/yun}" v-model="ruleFormTraining.content"
                        :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogTraining = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitTraining">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑项目经历-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_00465'{/yun}" :visible.sync="dialogProject" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00099'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormProject.name" placeholder="{yun:}t key='wap_user_00046'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00225'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormProject.title" placeholder="{yun:}t key='admin_00486'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00229'{/yun}</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeProject" type="monthrange" range-separator="{yun:}t key='admin_company_00019'{/yun}"
                            start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00228'{/yun}</div>
                    <el-input type="textarea" :placeholder="{yun:}t key='admin_vue_00012'{/yun}" v-model="ruleFormProject.content" :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogProject = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitProject">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑其他-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_user_00216'{/yun}" :visible.sync="dialogOther" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00103'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormOther.name" placeholder="{yun:}t key='admin_00487'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00231'{/yun}</div>
                    <el-input type="textarea" v-model="ruleFormOther.content" placeholder="{yun:}t key='admin_user_00203'{/yun}"
                        :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogOther = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitOther">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑技能-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_00461'{/yun}" :visible.sync="dialogSkill" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00089'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormSkill.name" placeholder="{yun:}t key='admin_user_00210'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_00458'{/yun}</div>
                    <div class="wxsettip_Sealect">
                        <el-input v-model="ruleFormSkill.longtime" placeholder="{yun:}t key='admin_user_00211'{/yun}">
                            <template slot="append">{yun:}t key='common_02077'{/yun}</template>
                        </el-input>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00094'{/yun}</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="ruleFormSkill.ing" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                            <el-option v-for="ingkey in userdata.user_ing" :key="ingkey" :label="userclass_name[ingkey]"
                                :value="ingkey">
                            </el-option>
                        </el-select>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00090'{/yun}</div>
                    <div>
                        <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept" action="" :auto-upload="false"
                            :on-change="handleChangeSkillPic" :show-file-list="false">
                            <img v-if="ruleFormSkill.pic_n" :src="ruleFormSkill.pic_n" class="avatar">
                            <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                        </el-upload>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogSkill = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitSkill">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>

        <!--删除弹窗-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_user_00241'{/yun}" :visible.sync="dialogDel" :with-header="true" append-to-body :show-close="true"
                width="300px">
                <div>
                    <el-checkbox v-model="ruleFormDel.delAccount" true-label="1" false-label="0">{yun:}t key='admin_user_00242'{/yun}</el-checkbox>
                </div>
                <div>
                    <i class="el-icon-warning"></i> {yun:}t key='admin_00508'{/yun}
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogDel = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="delSubmit">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>

        <div class="modluDrawer">
            <!--预览简历-->
            <el-drawer title="{yun:}t key='wap_user_00217'{/yun}" :visible.sync="drawerPreview" append-to-body size="60%">
                <preview :id="detail.id"></preview>
            </el-drawer>
            <!--新增简历-->
            <el-drawer title="{yun:}t key='admin_user_00193'{/yun}" :visible.sync="drawerAdd" append-to-body :wrapper-closable="false" size="45%">
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
			dataText: "{yun:}t key='admin_user_weipin_00026'{/yun}",
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
                    text: "{yun:}t key='common_02000'{/yun}",
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        start.setTime(start.getTime() - 3600 * 1000 * 24);
                        end.setTime(end.getTime() - 3600 * 1000 * 24);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: "{yun:}t key='common_01940'{/yun}",
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: "{yun:}t key='admin_user_00146'{/yun}",
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() - 1) * 24 * 60 * 60 * 1000);
                        const end = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: "{yun:}t key='admin_user_00142'{/yun}",
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() + 6) * 24 * 60 * 60 * 1000);
                        const end = new Date(new Date().setHours(0, 0, 0) + (0 - new Date().getDay()) *24 * 60 * 60 *1000);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: "{yun:}t key='admin_user_00147'{/yun}",
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 1).setHours(0, 0, 0));
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: "{yun:}t key='admin_user_00143'{/yun}",
                    onClick(picker) {
                        const end = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 0).setHours(23, 59, 59, 59));
                        const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth() - 1, 1).setHours(0, 0, 0));
                        picker.$emit('pick", [start, end]);
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

            // {yun:}t key='admin_00959'{/yun}
            t: "',
            order: '",

            checkedAll: false, // {yun:}t key='wap_js_00074'{/yun}
            checkedAllIndeterminate: false,
            multipleSelection: [], // 多选值存储
            idArr: [],

            detail: {},
            index: "",

            resumeAllNum: 0,
            resumeStatusNum1: 0,
            resumeStatusNum2: 0,
            resumeStatusNum3: 0,
            resumeTeenNum: 0,

            saveLoading: false,

            // {yun:}t key='member_com_00110'{/yun}
            dialogStatus: false,
            ruleFormStatus: {},

            // top/sticky
            dialogTop: false,
            ruleFormTop: {},

            // {yun:}t key='admin_user_00257'{/yun}
            dialogExport: false,
            isIndeterminateExport: false,
            checkAllExport: false,
            typeExport: {}, // 导出字段
            ruleFormExport: {
                type: [],
                limit: "',
                section: '"
            },

            // remark
            dialogRemark: false,
            ruleFormRemark: {},

            // Audit
            dialogAudit: false, // {yun:}t key='admin_user_weipin_00037'{/yun}
            drawerAudit: false,
            ruleFormAudit: {},
            auditTpl: "",
            todoAuditNum: 0,
            resume: {},
            expectData: {},

            // {yun:}t key='common_02022'{/yun}
            user_sex: {},
            userclass_name: {},
            userdata: {},
            industry_index: [],
            industry_name: {},

            // {yun:}t key='wap_user_00217'{/yun}
            drawerPreview: false,

            // Add
            drawerAdd: false,

            // Delete
            dialogDel: false,
            ruleFormDel: {},

            expectLoading: false,

            // {yun:}t key='admin_00475'{/yun}
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

            todayCheck: false, // 至今选中

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
            // 技能提升
            dialogSkill: false,
            indexSkill: -1,
            ruleFormSkill: {},
            // Project experience
            dialogProject: false,
            indexProject: -1,
            daterangeProject: [],
            ruleFormProject: {},
            // {yun:}t key='admin_00068'{/yun}
            dialogOther: false,
            indexOther: -1,
            ruleFormOther: {},

            // {yun:}t key='admin_00551'{/yun}
            drawerJobSqLog: false,
            jobSqLog: {},

            prevPage: 0,
            prevPage2: 0
        }
    },
    components: {
        "add': httpVueLoader('./resume_add.vue'),
        'job_class': httpVueLoader('../../../component/job_class.vue'),
        'city_class': httpVueLoader('../../../component/city_class.vue'),
        'preview': httpVueLoader('../../../component/resume_preview.vue")
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
            // 这里面需要注意，{yun:}t key='admin_user_company_00161'{/yun}ref需要那个那个包含table元素的父元素
            let divData = this.$refs.multipleTable.bodyWrapper;
            if (this.mouseFlag) {
                // 设置水平方向的元素的位置
                divData.scrollLeft -= (- this.mouseOffset + (this.mouseOffset = e.clientX));
            }
        },




		getParams:function(params={},search=false){
			var that = this;
			for(let i in params){
				if(typeof that.searchForm[i]!="undefined'){
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
                    that.dataText = "{yun:}t key='wap_js_00113'{/yun}";
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
                message.error("{yun:}t key='admin_user_weipin_00005'{/yun}");
                return false;
            }else if(this.multipleSelection.length == 0){
                message.error("{yun:}t key='admin_user_weipin_00001'{/yun}");
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
            if (typeof idx == 'undefined") { // {yun:}t key='member_com_00055'{/yun}
                this.ruleFormDel = {
                    del: this.idArr,
                    delAccount: 0
                }
            } else {// {yun:}t key='common_01711'{/yun}
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

            httpPost("m=user&c=users_resume&a=delResume', ruleForm).then(function (response) {
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
            this.$data[form][key] = val.replace(/[^0-9Xx.]/g, '");
        },

        // {yun:}t key='member_com_00110'{/yun}
        openStatus(row) {
            this.detail = row;
            this.ruleFormStatus = {
                uid: row.uid,
                status: row.status
            };
            this.dialogStatus = true;
            if (typeof this.userdata.user_label === "undefined') {
                this.getCache();
            }
        },
        submitStatus() {
            let that = this,
                params = that.ruleFormStatus;

            if (!params.status || params.status === '0') {
                message.warning("{yun:}t key='wap_00809'{/yun}");
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
                } else if (typeof row === 'undefined") { // {yun:}t key='admin_user_00237'{/yun}
                    message.success(res.data.msg);
                    that.getList();
                }
            })
        },
        // top/sticky
        openTop(row, s = "0') {
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

            if (params.s === '1') { // {yun:}t key='wap_com_00231'{/yun}
            } else { // {yun:}t key='wap_user_00335'{/yun}
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
                message.warning("{yun:}t key='admin_00516'{/yun}");
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
                message.warning("{yun:}t key='admin_user_weipin_00015'{/yun}");
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
                message.warning("{yun:}t key='admin_user_weipin_00015'{/yun}");
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

            httpPost('m=user&c=users_resume&a=getCache", {}, { hideloading: true }).then(function (response) {
                let res = response.data,
                    data = res.data;

                that.userdata = data.userdata;
                that.userclass_name = data.userclass_name;
            })
        },

        // {yun:}t key='wap_user_00217'{/yun}
        openPreview(row) {
            this.detail = row;
            this.drawerPreview = true;
        },

        // {yun:}t key='admin_user_00193'{/yun}
        openAdd() {
            let that =this;
            httpPost("m=user&c=users_resume&a=add', {add:1}).then(function (response) {
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

            if (typeof row === 'undefined") { // {yun:}t key='admin_user_00248'{/yun}
                params.ids = this.idArr;
            } else { // 单个刷新
                params.id = row.id;
            }

            delConfirm(this, params, function (params) {
                httpPost("m=user&c=users_resume&a=refresh', params).then(function (response) {
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
            let response = await httpPost('m=user&c=users_member&a=Imitate", { uid: uid });

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

        // {yun:}t key='admin_user_00227'{/yun}
        openBasic() {
            let resume = this.resume;
            this.ruleFormBasic = {
                uid: resume.uid,
                name: resume.name,
                sex: resume.sex,
                birthday: resume.birthday ? new Date(resume.birthday) : "',
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
                    message.warning("{yun:}t key='wap_user_00060'{/yun}");
                    return false;
                }
                if (tag.length >= 5) {
                    message.warning("{yun:}t key='admin_user_00206'{/yun}");
                    return false;
                }
                if (userTag.indexOf(tagval) > -1) {
                    message.warning("{yun:}t key='wap_user_00074'{/yun}");
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
                    message.warning("{yun:}t key='admin_user_00206'{/yun}");
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
                message.warning("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.tag.length > 5) {
                message.warning("{yun:}t key='admin_user_00206'{/yun}");
                return false;
            }
            if (ruleForm.description == '' || ruleForm.description == null) {
                message.warning("{yun:}t key='admin_01319'{/yun}");
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=saveTag", ruleForm).then(function (response) {
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
                job_classid: expect.job_classid, // TODO {yun:}t key='admin_00300'{/yun}
                city_classid: expect.city_classid, // TODO {yun:}t key='member_user_00362'{/yun}
                name: expect.name,
                minsalary: expect.minsalary && expect.minsalary > 0 ? parseInt(expect.minsalary) : "',
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
                message.warning("{yun:}t key='admin_00484'{/yun}");
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
                message.warning("{yun:}t key='member_user_00095'{/yun}");
                return false;
            }
            if (ruleForm.report == "") {
                message.warning("{yun:}t key='wap_00980'{/yun}");
                return false;
            }
            if (ruleForm.type == "") {
                message.warning("{yun:}t key='wap_js_00163'{/yun}");
                return false;
            }
            if (ruleForm.jobstatus == "") {
                message.warning("{yun:}t key='wap_00934'{/yun}");
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
                message.warning("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.name == "") {
                message.warning("{yun:}t key='wap_00137'{/yun}");
                return false;
            }
            if (ruleForm.sdate == "") {
                message.warning("{yun:}t key='admin_user_00213'{/yun}");
                return false
            }
            ruleForm.sdate = formatMonth(ruleForm.sdate);
            if (ruleForm.edate != '') {
                if (ruleForm.sdate >= ruleForm.edate) {
                    message.warning("{yun:}t key='admin_user_00201'{/yun}");
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
                        work.edate_n = ruleForm.edate != '' ? ruleForm.edate : "{yun:}t key='wap_js_00170'{/yun}";
                        that.expectData.work.unshift(work);
                    } else {
                        let work = that.expectData.work[indexWork];
                        work.name = ruleForm.name;
                        work.title = ruleForm.title;
                        work.sdate = 1;
                        work.sdate_n = ruleForm.sdate;
                        work.edate = ruleForm.edate != '' ? 2 : 0;
                        work.edate_n = ruleForm.edate != '' ? ruleForm.edate : "{yun:}t key='wap_js_00170'{/yun}";
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
                message.warning("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.name == "") {
                message.warning("{yun:}t key='wap_user_00044'{/yun}");
                return false;
            }
            if (daterangeEdu.length == 0) {
                message.warning(lc('admin_vue_00016'));
                return false
            }
            if (ruleForm.education == "") {
                message.warning("{yun:}t key='wap_user_00049'{/yun}");
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
                message.warning("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.name == "") {
                message.warning("{yun:}t key='admin_00485'{/yun}");
                return false;
            }
            if (daterangeTraining.length == 0) {
                message.warning("{yun:}t key='admin_user_00212'{/yun}");
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
                message.warning("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.name == "") {
                message.warning("{yun:}t key='admin_user_00210'{/yun}");
                return false;
            }
            if (ruleForm.ing == "") {
                message.warning("{yun:}t key='wap_user_00072'{/yun}");
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
                message.warning("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.name == "") {
                message.warning("{yun:}t key='wap_user_00046'{/yun}");
                return false;
            }
            if (daterangeProject.length == 0) {
                message.warning("{yun:}t key='admin_user_00214'{/yun}");
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
                    if (ruleForm.id == '") {
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

        // {yun:}t key='admin_00068'{/yun}
        openOther(index) {
            let expectData = this.expectData,
                expect = expectData.expect,
                otherList = expectData.other;

            if (index !== "') {
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
                message.warning("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.name == "") {
                message.warning("{yun:}t key='admin_00487'{/yun}");
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
            }, "{yun:}t key='admin_user_00204'{/yun}");
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
                    that.dataText = "{yun:}t key='wap_js_00113'{/yun}";
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
