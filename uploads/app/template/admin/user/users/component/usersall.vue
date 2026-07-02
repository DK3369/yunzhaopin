<template>
    <div class="moduleElHight" :class="searchClass == 'drawer' ? 'pad_lr_20' : ''">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall" style="padding: 2px 0;">
                <el-input v-model="searchForm.keyword" @keyup.enter.native="search" :placeholder="lc('admin_user_00158')" size="small"
                    clearable>
                    <el-select v-model="searchForm.type" size="small" slot="prepend" :placeholder="lc('admin_user_00140')" style="padding-left:20px;">
                        <el-option :label="lc('admin_user_00140')" :value="1"></el-option>
                        <el-option :label="lc('wap_00529')" :value="2"></el-option>
                        <el-option :label="lc('wap_01619')" :value="3"></el-option>
                        <el-option label="EMAIL" :value="4"></el-option>
                        <el-option :label="lc('admin_user_00130')" :value="5"></el-option>
                        <el-option label="IP" :value="6"></el-option>
                    </el-select>
                </el-input>
            </div>
            <!-- Collapsed section -->
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="searchForm.time_type" size="small" slot="prepend" :placeholder="lc('admin_user_00135')" clearable @change="handleTimeChange">
                    <el-option :label="lc('admin_user_00129')" value="adtime"></el-option>
                    <el-option :label="lc('admin_user_00134')" value="lotime"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmalltwo" :class="{ 'searchbutnOnff': seachbutn }">
                <el-date-picker v-model="searchForm.times" type="daterange" align="right" unlink-panels :range-separator="lc('admin_company_00019')" :start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')" :picker-options="timeOptions" value-format="yyyy-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
            </div>
            <div v-for="(searchItem, searchIndex) in searchList" :key="searchIndex"
                class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="searchForm[searchItem.param]" slot="prepend" :clearable="true"
                    :placeholder="searchItem.name" size="small" @change="search">
                    <el-option v-for="(searchLabel, searchValue) in searchItem.value" :key="searchValue"
                        :label="searchLabel" :value="searchValue"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" plain icon="el-icon-document-add" size="mini" @click="openAdd">{{ lc('admin_user_00305') }}</el-button>
            </div>
            <div class="tableSeachzk" :class="{ 'searchbutnKai': seachbutn }" style="margin-bottom: 11px;">
                <el-button type="info" class="zhankai" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>{{ lc('admin_user_00145') }}
                    <i class="el-icon-arrow-down el-icon--right"></i>
                </el-button>
                <el-button type="info" class="shouqi" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>{{ lc('admin_user_00144') }}
                    <i class="el-icon-arrow-up el-icon--right"></i>
                </el-button>
            </div>
        </div>
        <div class="admin_datatip">
            <i class="el-icon-document"></i> {{ lc("admin_data_stats") }} <span @click="init">{{ lc("admin_total_count", [userAllNum]) }}</span>
            <span class="admin_datatip_n" @click="statusSearch('2')">{{ lc("admin_locked_count", [userStatusNum3 ? userStatusNum3 : 0]) }}</span>
            <span class="admin_datatip_n">{{ lc("admin_search_results_count", [total]) }}</span>
        </div>
        <div class="moduleElTable" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="list" style="width: 100%" stripe ref="multipleTable" @selection-change="handleSelectionChange"

                @mousedown.native="mouseDownHandler"
                @mouseup.native="mouseUpHandler"
                @mousemove.native="mouseMoveHandler"

                @sort-change="sortChange" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                v-loading="loading">
                <template slot="empty">
                    <p>{{ dataText }}</p>
                </template>
                <el-table-column type="selection" width="50"> </el-table-column>
                <el-table-column prop="uid" :label="lc('admin_user_00130')" width="100" sortable="custom"></el-table-column>
                <el-table-column :label="lc('admin_00545')" min-width="110" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class="username">{{ scope.row.username_n }}</div>
                        </div>
                        <div class="yhm">
                            <el-link @click="memberCheck(scope.row.uid, scope.row.usertype)" :underline="false">{{
                                scope.row.username
                            }}
                            </el-link>
                            <el-tooltip v-if="scope.row.r_status == '2'" class="item" effect="dark" :content="lc('admin_user_00138')"
                                placement="top-start">
                                <i class="el-icon-lock" style="color: orange"></i>
                            </el-tooltip>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_user_00117')" min-width="130">
                    <template slot-scope="scope">
                        <div class="moduleProps" v-if="scope.row.telphone">
                            <span>{{ scope.row.telphone }}</span>
                            <span v-if="scope.row.moblie_address" class="gsd">
                                {{ scope.row.moblie_address }}
                            </span>
                            <el-link v-else type="primary" :underline="false"
                                @click="getMobileAddress(scope.$index)">{{ lc('admin_00433') }}</el-link>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_user_00287')" min-width="110" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <div class="rz_box">
                            <el-tooltip v-if="scope.row.idcard_status == 1" class="item" effect="dark" :content="lc('resume_00008')"
                                placement="top-start">
                                <el-button type="text" @click="idcardRz(scope.row)">
                                    <i class="rzicon rzicon_zzyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" :content="lc('member_user_00502')" placement="top-start">
                                <el-button type="text" @click="idcardRz(scope.row)">
                                    <i class="rzicon rzicon_zzwrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-if="scope.row.moblie_status == 1" class="item" effect="dark" :content="lc('member_user_00117')"
                                placement="top-start">
                                <el-button type="text" @click="moblieRz(scope.row)">
                                    <i class="rzicon rzicon_sjyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" :content="lc('wap_01245')" placement="top-start">
                                <el-button type="text" @click="moblieRz(scope.row)">
                                    <i class="rzicon rzicon_sjwrz"></i>
                                </el-button>
                            </el-tooltip>
                        </div>
                        <div class="rz_box">
                            <el-tooltip v-if="scope.row.email_status_n == 1" class="item" effect="dark" :content="lc('admin_user_00286')"
                                placement="top-start">
                                <el-button type="text" @click="emailRz(scope.row)">
                                    <i class="rzicon rzicon_yxyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" :content="lc('wap_01246')" placement="top-start">
                                <el-button type="text" @click="emailRz(scope.row)">
                                    <i class="rzicon rzicon_yxwrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-if="scope.row.wxid != '' || scope.row.wxopenid != ''"
                                class="item" effect="dark" placement="top-start">
                                <div slot="content" v-html="lc('admin_user_company_00129') + '<br/>' + scope.row.wxBindmsg"></div>
                                <el-button type="text">
                                    <i class="rzicon rzicon_wxyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" placement="top-start">
                                <div slot="content" v-html="lc('member_user_00504') + '<br/>' + scope.row.wxBindmsg"></div>
                                <el-button type="text">
                                    <i class="rzicon rzicon_wxwrz"></i>
                                </el-button>
                            </el-tooltip>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_00510')" min-width="130" align="center">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class="username">{{ scope.row.sq_num > 0 ? scope.row.sq_num : 0 }}</div>
                            <el-link v-if="scope.row.sq_num > 0" type="primary" :underline="false"
                                @click="openSqLog(scope.$index, scope.row)">{{ lc('wap_com_00427') }}</el-link>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="login_date" :label="lc('admin_user_00121')" min-width="150" sortable="custom">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span class="gsd">{{ scope.row.reg_date_n }}</span>
                            <span v-if="scope.row.login_date_n">{{ scope.row.login_date_n }}</span>
                            <span v-else>{{ lc('admin_user_00139') }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_00546')" min-width="150">
                    <template slot-scope="scope">
                        <div class="moduleProps" v-if="scope.row.def_job != '0'">
                            <el-link type="primary" :underline="false"
                                @click="openDetail(scope.$index, scope.row)">{{ lc('wap_user_00217') }}</el-link>
                        </div>
                        <div class="moduleProps" v-else>
                            <el-link type="primary" :underline="false" @click="openResume(scope.row)">{{ lc('admin_user_00296') }}</el-link>
                        </div>
                        <span class="gsd">{{ source[scope.row.source] }}</span>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_vue_00026')" min-width="130">
                    <template slot-scope="scope">
                        <div class="moduleProps">

                            <div v-if="scope.row.login_ip">
                                <span>{{ scope.row.login_ip }}</span>
                                <span v-if="scope.row.login_address" class="gsd"> {{ scope.row.login_address }}</span>
                                <el-link v-else type="primary" :underline="false"
                                    @click="getIpAddress(scope.$index)">{{ lc('admin_00433') }}</el-link>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00181')" fixed="right" width="60">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.r_status == '2'" class="admin_state3">{{ lc('admin_user_00138') }}</span>
                            <span v-else class="admin_state1">{{ lc('admin_user_00149') }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="80" fixed="right" align="center">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="openDetail(scope.$index, scope.row)">{{ lc('member_com_00380') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging" style="height: initial; flex-wrap: wrap; padding-top: 10px;">
            <div  style="width:100%;">
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
                    @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="batch('del')" size="mini">{{ lc('member_com_00055') }}</el-button>
                <el-button @click="batch('domain')" size="mini">{{ lc('admin_user_00279') }}</el-button>
                <el-button @click="batch('auth')" size="mini">{{ lc('admin_user_00292') }}</el-button>
            </div>
            <div class="modulePagNum" style="padding-top: 8px;">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page="page" :page-sizes="pageSizes" :page-size="limit"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <!-- Delete dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00547')" :visible.sync="dialogDel" :with-header="true" append-to-body :show-close="true"
                width="300px">
                <div>
                    <el-checkbox v-model="ruleFormDel.delAccount" true-label="1" false-label="0">{{ lc('admin_user_00242') }}</el-checkbox>
                </div>
                <div>
                    <i class="el-icon-warning"></i> {{ lc('admin_00508') }}
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogDel = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="delSubmit">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Identity verification dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_01030')" :visible.sync="dialogIdcardRz" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_00529') }}</div>
                    <el-input :value="detail.username_n" :disabled="true"></el-input>
                    <div class="wxsettip_small ">{{ lc('admin_user_00285') }}</div>
                    <el-input :value="detail.idcard" :disabled="true"></el-input>
                    <div class="wxsettip_small ">{{ lc('admin_00533') }}</div>
                    <div class="zzrz_img">
                        <div class="zzrz_imgpreview">
                            <el-image v-if="detail.idcard_pic" style="width: 80px; height: 80px" :src="detail.idcard_pic"
                                :preview-src-list="detail.idcard_pic">
                            </el-image>
                            <span v-else>{{ lc('admin_user_00277') }}</span>
                        </div>
                    </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_weipin_00032') }}</div>
                    <el-radio v-model="ruleFormIdcardRz.r_status" label="0">{{ lc('admin_user_00300') }}</el-radio>
                    <el-radio v-model="ruleFormIdcardRz.r_status" label="1">{{ lc('wap_user_00128') }}</el-radio>
                    <div class="wxsettip_small ">{{ lc('member_user_00062') }}</div>
                    <el-input type="textarea" :rows="2" placeholder="" v-model="ruleFormIdcardRz.statusbody"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogIdcardRz = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="idcardRzSubmit">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Mobile verification dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('member_com_00071')" :visible.sync="dialogMoblieRz" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_01619') }}</div>
                    <el-input placeholder="" v-model="ruleFormMobileRz.moblie"></el-input>
                    <div class="wxsettip_small ">{{ lc('admin_user_weipin_00032') }}</div>
                    <el-radio v-model="ruleFormMobileRz.mstatus" label="1">{{ lc('wap_user_00128') }}</el-radio>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogMoblieRz = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="moblieRzSubmit">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Email verification dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_com_00186')" :visible.sync="dialogEmailRz" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00303') }}</div>
                    <el-input placeholder="" v-model="ruleFormEmailRz.email"></el-input>
                    <div class="wxsettip_small ">{{ lc('admin_user_weipin_00032') }}</div>
                    <el-radio v-model="ruleFormEmailRz.estatus" label="1">{{ lc('wap_user_00128') }}</el-radio>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogEmailRz = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="emailRzSubmit">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Applied jobs dialog -->
        <!-- Add individual user dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00548')" :visible.sync="dialogAdd" :append-to-body="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00140') }}</div>
                    <el-input :placeholder="lc('wap_00208')" v-model="ruleFormAdd.username"></el-input>
                    <div class="wxsettip_small ">{{ lc('wap_00702') }}</div>
                    <el-input @mousedown.native="pwdMousedown" @input="pwdchange" @focus="readonlyCtl(false)" @blur="readonlyCtl(true)" :readonly="pwdreadonly" :placeholder="lc('wap_00703')" v-model="ruleFormAdd.password" ></el-input>
                    <div class="wxsettip_small ">{{ lc('member_user_00282') }}</div>
                    <el-input :placeholder="lc('wap_00697')" v-model="ruleFormAdd.email"></el-input>
                    <div class="wxsettip_small ">{{ lc('wap_01619') }}</div>
                    <el-input :placeholder="lc('wap_js_00119')" v-model="ruleFormAdd.moblie"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAdd = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="saveAdd">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
            <el-dialog :title="lc('admin_user_weipin_00029')" :visible.sync="dialogDomain" append-to-body :show-close="true" width="500px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList" v-if="detail.id">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00534') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <span>{{ detail.username }}</span>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_user_weipin_00020') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-select v-model="ruleFormDomain.did" filterable :placeholder="lc('wap_user_00100')">
                                <el-option v-for="(item, key) in domainList" :key="key" :label="item" :value="key">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogDomain = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="saveDomain">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
            <el-dialog :title="lc('admin_user_00292')" :visible.sync="dialogAuth" append-to-body :show-close="true" width="500px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00535') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-checkbox-group v-model="ruleFormAuth.type">
                                <el-checkbox label="email">{{ lc('member_user_00282') }}</el-checkbox>
                                <el-checkbox label="moblie">{{ lc('member_user_00163') }}</el-checkbox>
                                <el-checkbox label="idcard">{{ lc('member_com_00014') }}</el-checkbox>
                            </el-checkbox-group>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00536') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-radio v-model="ruleFormAuth.status" label="0">{{ lc('admin_user_00300') }}</el-radio>
                            <el-radio v-model="ruleFormAuth.status" label="1">{{ lc('wap_user_00128') }}</el-radio>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAuth = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="authSubmit">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Account merge dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_00280')" :visible.sync="dialogAccountMerge" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div style="overflow: hidden; position: relative; width: 100%;">
                    <div class="wxsettip_small ">{{ lc('default_00330') }}</div>
                    <div class="">{{ lc("admin_name_value", [detail.username_n]) }} {{ lc("admin_account_value", [detail.username]) }}</div>
                    <div class="wxsettip_small ">{{ lc('wap_com_00157') }}</div>
                    <!--<el-input v-model="ruleFormAccountMerge.com_uid" placeholder="Enter company name"></el-input>-->
                    <el-autocomplete style="width: 100%;" v-model="AccountMergeComname" :fetch-suggestions="querySearchCom"
                        value-key="name" :placeholder="lc('wap_user_00149')" @select="handleSelectCom"></el-autocomplete>
                    <el-divider content-position="left">{{ lc('admin_user_00270') }}</el-divider>
                    <div class="wxsettip_small ">{{ lc('wap_user_00241') }}</div>
                    <el-radio v-model="ruleFormAccountMerge.mobile" :label="1">{{ lc('common.company') }}</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.mobile" :label="2">{{ lc('admin_user_00304') }}</el-radio>
                    <div class="wxsettip_small ">{{ lc('wap_com_00016') }}</div>
                    <el-radio v-model="ruleFormAccountMerge.email" :label="1">{{ lc('common.company') }}</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.email" :label="2">{{ lc('admin_user_00304') }}</el-radio>
                    <div class="wxsettip_small ">{{ lc('admin_00537') }}</div>
                    <el-radio v-model="ruleFormAccountMerge.QQ" :label="1">{{ lc('common.company') }}</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.QQ" :label="2">{{ lc('admin_user_00304') }}</el-radio>
                    <div class="wxsettip_small ">{{ lc('member_user_00056') }}</div>
                    <el-radio v-model="ruleFormAccountMerge.wx" :label="1">{{ lc('common.company') }}</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.wx" :label="2">{{ lc('admin_user_00304') }}</el-radio>
                    <div class="wxsettip_small ">{{ lc('admin_user_00297') }}</div>
                    <el-radio v-model="ruleFormAccountMerge.sina" :label="1">{{ lc('common.company') }}</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.sina" :label="2">{{ lc('admin_user_00304') }}</el-radio>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAccountMerge = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitAccountMerge">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Delete dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00549')" :visible.sync="scdrawer" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                {{ lc('admin_00538') }}
                <span slot="footer" class="dialog-footer">
                    <el-button @click="scdrawer = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="scdrawer = false">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Resume preview dialog -->
        <!-- Account information dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_00191')" :visible.sync="dialogAccount" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00140') }}</div>
                    <el-input :placeholder="lc('wap_00208')" v-model="ruleFormAccount.username"></el-input>
                    <div class="wxsettip_small ">{{ lc('wap_00702') }}</div>
                    <el-input @mousedown.native="pwdMousedown" @input="pwdchange" @focus="readonlyCtl(false)" @blur="readonlyCtl(true)" :readonly="pwdreadonly" :placeholder="lc('wap_00703')" v-model="ruleFormAccount.password" ></el-input>
                    <div class="wxsettip_small ">{{ lc('member_user_00181') }}</div>
                    <el-radio-group v-model="ruleFormAccount.status">
                        <el-radio label="1">{{ lc('admin_user_00149') }}</el-radio>
                        <el-radio label="2">{{ lc('admin_user_00150') }}</el-radio>
                    </el-radio-group>
                    <template v-if="ruleFormAccount.status == 2">
                        <div class="wxsettip_small ">{{ lc('admin_00438') }}</div>
                        <el-input type="textarea" :rows="2" v-model="ruleFormAccount.lock_info">
                        </el-input>
                    </template>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAccount = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitAccount" :loading="saveLoading">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Individual details dialog -->
        <el-drawer :title="lc('admin_user_00291')" :visible.sync="drawerDetail" @closed="closedDetail" :modal-append-to-body="false" size="95%"
            :append-to-body="true">
            <div class="shbox">
                <div class="shinfo">
                    <div class="sh_zwsz">
                        <el-button type="primary" size="mini" @click="toMember(detail)"><i class="el-icon-school"></i>
                            {{ lc('admin_00539') }}</el-button>
                    </div>
                    <div class="shcomdj">
                        {{ lc("admin_name_value", [resume.name]) }}
                        <span class="shcomtel_n">{{ lc("admin_username_value", [member.username]) }}</span>
                        {{ lc("admin_mobile_value", [resume.telphone]) }}
                    </div>
                    <div class="shcomtel" style="padding-bottom:15px; padding-top:10px;;border:none;font-size: 13px;">
                        <span class=" ">{{ lc('admin_00540') }} </span>
                        <span class="shcomtel_n" v-if="member.logion_date != ''">{{ lc('admin_00541') }} </span>
                        <span class="shcomtel_n" v-else>{{ lc('admin_user_00288') }} </span>
                        {{ lc("admin_login_count_value", [member.login_hits]) }}
                        <span class=" shcomtel_n"> IP：{{ member.login_ip }} </span>
                        <span class=" "></span>
                        <span class="shcomtel_n">{{ lc("admin_source_value", [source[member.source]]) }}</span>
                        <span class=" ">{{ lc("admin_site_value", [domainList[resume.did]]) }}</span>
                        <div class="cominfocz">
                            <el-button type="primary" @click="openAccount" size="mini">
                                <i class="el-icon-edit"></i>{{ lc('admin_user_00191') }}
                            </el-button>
                            <el-button type="primary" @click="openAccountMerge" size="mini">
                                <i class="el-icon-document-add"></i>{{ lc('admin_00542') }}
                            </el-button>
                            <el-button type="primary" @click="resetPassword(detail)" size="mini">
                                <i class="el-icon-thumb"></i>{{ lc('admin_user_00137') }}
                            </el-button>
                            <el-button type="primary" size="mini" @click="openDomain(resume)">
                                <i class="el-icon-map-location"></i>{{ lc('admin_user_weipin_00029') }}
                            </el-button>
                            <el-button type="primary" @click="openDel(index)" size="mini">
                                <i class="el-icon-close"></i>{{ lc('admin_00543') }}
                            </el-button>
                        </div>
                    </div>
                    <!-- Individual details tab switch -->
                    <el-tabs v-model="activeName" type="card" @tab-click="handleClick">
                        <el-tab-pane :label="lc('admin_00550')" name="resume">
                            <div v-loading="expectLoading">
                                <div class="shshow_tit">
                                    <i class="el-icon-office-building"></i> {{ lc('wap_user_00341') }}
                                    <span class="shshow_cz">
                                        <el-button type="text" @click="openBasic">
                                            <i class="el-icon-edit"></i>{{ lc('admin_user_00227') }}
                                        </el-button>
                                    </span>
                                </div>
                                <div class="userinfo_box">
                                    <div class="userinfo_l"><img :src="resume.photo" width="70" height="70"> </div>
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
                                        <el-tag size="mini" v-for="(tagItem, tagIndex) in resume.arrayTag" :key="tagIndex">
                                            {{ tagItem }}
                                        </el-tag>
                                        <div class="cominfo">{{ resume.description }}</div>
                                    </div>
                                    <div class="user_resume_add">
                                        <div class="">{{ lc('admin_user_00196') }}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openTag">
                                                <i class="el-icon-circle-plus-outline"></i> {{ (resume.arrayTag &&
                                                    resume.arrayTag.length > 0) || resume.description ? lc('common.edit') : lc('wap_js_00091') }}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!-- Job intention -->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"><i class="el-icon-notebook-2"></i> {{ lc('wap_00460') }} </div>
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
                                    <div class="user_resume_add">
                                        <div class="">{{ lc('admin_user_00205') }}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openJob">
                                                <i class="el-icon-circle-plus-outline"></i> {{ lc('admin_00472') }}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!-- Work experience -->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"> <i class="el-icon-suitcase-1"></i> {{ lc('wap_00457') }} </div>
                                    <!-- Loop -->
                                    <div class="user_resume_show" v-for="(work, workkey) in expectData.work">
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
                                    <div class="user_resume_add">
                                        <div class="">{{ lc('admin_user_00195') }}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openWork('')">
                                                <i class="el-icon-circle-plus-outline"></i> {{ lc('wap_js_00091') }}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!-- Education experience -->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"> <i class="el-icon-school"></i> {{ lc('wap_00459') }} </div>
                                    <!-- Loop -->
                                    <div class="user_resume_show" v-for="(edu, edukey) in expectData.edu">
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
                                                    v-if="edu.specialty && edu.education_n">|</span>{{ edu.education_n }}
                                            </div>
                                            <div class="user_resume_time">{{ edu.sdate_n }}-{{ edu.edate_n }}</div>
                                        </div>
                                    </div>
                                    <!-- Loop -->
                                    <div class="user_resume_add">
                                        <div class="">{{ lc('admin_user_00202') }} </div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openEdu('')">
                                                <i class="el-icon-circle-plus-outline"></i> {{ lc('wap_js_00091') }}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!-- Training experience -->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"> <i class="el-icon-data-analysis"></i> {{ lc('wap_00455') }} </div>
                                    <!-- Loop -->
                                    <div class="user_resume_show" v-for="(training, trainingKey) in expectData.training">
                                        <div class="user_resume_addname ">{{ training.name }}
                                            <el-button type="text" @click="openTraining(trainingKey)">
                                                <i class="el-icon-edit"></i> {{ lc('wap_js_00073') }}
                                            </el-button>
                                            <el-button type="text"
                                                @click="delResumeFb('training', trainingKey, training.id)">
                                                <i class="el-icon-delete"></i> {{ lc('common.delete') }}
                                            </el-button>
                                        </div>
                                        <div class="user_resume_addjy">
                                            <div class=" ">{{ training.title }} </div>
                                            <div class="user_resume_time">{{ training.sdate_n }}-{{ training.edate_n }}
                                            </div>
                                        </div>
                                        <div class="user_resume_ms">{{ training.content }}</div>
                                    </div>
                                    <!-- Loop -->
                                    <div class="user_resume_add">
                                        <div class="">{{ lc('admin_user_00197') }}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openTraining('')">
                                                <i class="el-icon-circle-plus-outline"></i> {{ lc('wap_js_00091') }}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!-- Professional skills -->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"><i class="el-icon-reading"></i> {{ lc('wap_00461') }}</div>
                                    <!-- Loop -->
                                    <div class="user_resume_show" v-for="(skill, skillkey) in expectData.skill">
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
                                    <div class="user_resume_add">
                                        <div class="">{{ lc('admin_user_00199') }}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openSkill('')">
                                                <i class="el-icon-circle-plus-outline"></i> {{ lc('wap_js_00091') }}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!-- Project experience -->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"><i class="el-icon-wallet"></i> {{ lc('wap_00465') }} </div>
                                    <!-- Loop -->
                                    <div class="user_resume_show" v-for="(project, projectkey) in expectData.project">
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
                                    <div class="user_resume_add">
                                        <div class="">{{ lc('admin_user_00194') }}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openProject('')">
                                                <i class="el-icon-circle-plus-outline"></i> {{ lc('wap_js_00091') }}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!-- Other description -->
                                <div class="user_resume_list" style="padding-bottom:80px; ;">
                                    <div class="shshow_tit"> <i class="el-icon-mic"></i> {{ lc('admin_00068') }} </div>
                                    <!-- Loop -->
                                    <div class="user_resume_show" v-for="(other, otherkey) in expectData.other">
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
                                    <div class="user_resume_add">
                                        <div class="">{{ lc('admin_user_00215') }}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openOther('')">
                                                <i class="el-icon-circle-plus-outline"></i> {{ lc('wap_js_00091') }}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane :label="lc('admin_00551')" name="sqlog">
                            <div class="moduleElHight">
                                <div class="moduleElTable"
                                    style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 45px);">
                                    <el-table :data="jobSqLog.list" style="width: 100%" height="100%" ref="table2" stripe
                                        :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                                        v-loading="loading">
                                        <template slot="empty">
                                            <p>{{ dataText }}</p>
                                        </template>
                                        <el-table-column prop="job_name" :label="lc('wap_01596')">
                                            <template slot-scope="scope">
                                                <div class="moduleProps">
                                                    <el-link type="primary" :underline="false"
                                                        @click="openPage(scope.row.job_comapply)">{{ scope.row.job_name
                                                        }}</el-link>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="com_name" :label="lc('admin_user_00247')">
                                            <template slot-scope="scope">
                                                <div class="moduleProps">
                                                    <el-link type="primary" :underline="false"
                                                        @click="openPage(scope.row.company_show)">{{ scope.row.com_name
                                                        }}</el-link>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="datetime_n_n" :label="lc('member_user_00431')"></el-table-column>
                                        <el-table-column :label="lc('admin_user_00250')">
                                            <template slot-scope="scope">
                                                <div class="admin_state">
                                                    <span class="admin_state1" v-if="scope.row.is_browse == 2">{{ lc('wap_user_00258') }}</span>
                                                    <span class="admin_state2"
                                                        v-else-if="scope.row.is_browse == 3">{{ lc('admin_user_00252') }}</span>
                                                    <span class="admin_state3"
                                                        v-else-if="scope.row.is_browse == 4">{{ lc('wap_user_00354') }}</span>
                                                    <span class="admin_state4"
                                                        v-else-if="scope.row.is_browse == 5">{{ lc('member_com_00108') }}</span>
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
                                            :hide-on-single-page="true" @current-change="handleCurrentChangeJobSqlLog"
                                            :current-page="jobSqLog.page" :page-sizes="jobSqLog.pageSizes"
                                            :page-size="jobSqLog.limit" layout="total, sizes, prev, pager, next, jumper"
                                            :total="jobSqLog.total">
                                        </el-pagination>
                                    </div>
                                </div>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane :label="lc('wap_com_00046')" name="yqms">
                            <div class="moduleElHight">
                                <div class="moduleElTable"
                                    style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 55px);">
                                    <el-table :data="yqmsLog.list" style="width: 100%" ref="table3" stripe
                                        :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%">
                                        <template slot="empty">
                                            <p>{{ dataText }}</p>
                                        </template>
                                        <el-table-column prop="fname" :label="lc('wap_01403')" min-width="200">
                                            <template slot-scope="scope">
                                                <div class="moduleProps">
                                                    <el-link type="primary" :underline="false"
                                                        @click="openPage(scope.row.company_show)">{{ scope.row.fname
                                                        }}</el-link>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="jobname" :label="lc('admin_00552')" min-width="200">
                                            <template slot-scope="scope">
                                                <div class="moduleProps">
                                                    <el-link type="primary" :underline="false"
                                                        @click="openPage(scope.row.job_comapply)">{{ scope.row.jobname
                                                        }}</el-link>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="title" :label="lc('admin_00553')" width="150"></el-table-column>
                                        <el-table-column prop="content" :label="lc('admin_00554')" min-width="170"></el-table-column>
                                        <el-table-column prop="datetime_n" :label="lc('member_user_00170')" width="170"></el-table-column>
                                        <el-table-column :label="lc('admin_user_00250')" width="150">
                                            <template slot-scope="scope">
                                                <div class="admin_state">
                                                    <span class="admin_state1" v-if="scope.row.is_browse == 2">{{ lc('wap_user_00258') }}</span>
                                                    <span class="admin_state2"
                                                        v-else-if="scope.row.is_browse == 3">{{ lc('wap_com_00190') }}</span>
                                                    <span class="admin_state3"
                                                        v-else-if="scope.row.is_browse == 4">{{ lc('wap_user_00257') }}</span>
                                                    <span class="admin_state5" v-else>{{ lc('wap_user_00260') }}</span>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="isdel_n" :label="lc('member_user_00181')" width="100"></el-table-column>
                                    </el-table>
                                </div>
                                <div class="modulePaging">
                                    <div></div>
                                    <div class="modulePagNum">
                                        <el-pagination background @size-change="handleSizeChangeYqmsLog"
                                            :hide-on-single-page="true" @current-change="handleCurrentChangeYqmsLog"
                                            :current-page="yqmsLog.page" :page-sizes="yqmsLog.pageSizes"
                                            :page-size="yqmsLog.limit" layout="total, sizes, prev, pager, next, jumper"
                                            :total="yqmsLog.total">
                                        </el-pagination>
                                    </div>
                                </div>
                            </div>
                        </el-tab-pane>
                        
                        <el-tab-pane :label="lc('admin_00555')" name="log">
                            <div v-if="userLog.list">
                                <template v-for="(ulogitem, ulogkey) in userLog.list">
                                    <el-divider content-position="left">{{ ulogitem.week }} {{ ulogkey }}</el-divider>
                                    <div class="dt_list">
                                        <ul>
                                            <li v-for="ulog in ulogitem.list">
                                                <div class="dt_time">{{ ulog.time_n }}</div>
                                                <div class="dt_name" v-if="ulog.opera_n">{{ ulog.opera_n }}</div>
                                                <div class="dt_mx">{{ ulog.content }}</div>
                                            </li>
                                            <!--<li>-->
                                            <!--	<div class="dt_time">08:35</div>-->
                                            <!--	<div class="dt_name">Viewed job</div>-->
                                            <!--	<div class="dt_mx">Sample user viewed a sample company job</div>-->
                                            <!--</li>-->
                                            <!--<li>-->
                                            <!--	<div class="dt_time">10:39</div>-->
                                            <!--	<div class="dt_name">Visit behavior</div>-->
                                            <!--	<div class="dt_mx">Sample user visited the job assistant</div>-->
                                            <!--</li>-->
                                        </ul>
                                    </div>
                                </template>
                                <div style="height: 100px">
                                    <div v-if="userLog.page == userLog.last_page">{{ lc('admin_user_00283') }}</div>
                                    <h3 v-else @click="handleCurrentChangeUserLog">{{ lc('admin_user_00276') }}</h3>
                                </div>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane :label="lc('admin_00556')" name="pay">
                            <!--<div class="admin_datatip">-->
                            <!--	<i class="el-icon-document"></i> {{ lc("admin_data_stats") }} Current points: 3526-->
                            <!--	<span class="admin_datatip_n">Total points spent: 13625 </span>-->
                            <!--</div>-->
                            <div class="moduleElHight">
                                <div class="moduleElTable"
                                    style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 45px);">
                                    <el-table :data="payLog.list" style="width: 100%" ref="table4" stripe
                                        :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                                        v-loading="loading" height="100%">
                                        <template slot="empty">
                                            <p>{{ dataText }}</p>
                                        </template>
                                        <el-table-column prop="order_id" :label="lc('admin_user_00295')"></el-table-column>
                                        <el-table-column prop="consume_price_n" :label="lc('member_user_00254')"></el-table-column>
                                        <el-table-column prop="consume_remark" :label="lc('admin_user_00290')"></el-table-column>
                                        <el-table-column prop="pay_time_n" :label="lc('wap_com_00344')"></el-table-column>
                                        <el-table-column prop="consume_state_n" :label="lc('member_user_00181')"></el-table-column>
                                    </el-table>
                                </div>
                                <div class="modulePaging">
                                    <div></div>
                                    <div class="modulePagNum">
                                        <el-pagination background @size-change="handleSizeChangePayLog"
                                            :hide-on-single-page="true" @current-change="handleCurrentChangePayLog"
                                            :current-page="payLog.page" :page-sizes="payLog.pageSizes"
                                            :page-size="payLog.limit" layout="total, sizes, prev, pager, next, jumper"
                                            :total="payLog.total">
                                        </el-pagination>
                                    </div>
                                </div>
                            </div>
                        </el-tab-pane>
                    </el-tabs>
                </div>
            </div>
            <!-- Edit resume basic information -->
            <el-drawer :title="lc('admin_00475')" :append-to-body="true" :visible.sync="drawerBasic" :wrapper-closable="false"
                size="60%">
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
                                            <el-radio v-for="(sex, sexkey) in user_sex" :label="sexkey" :key="sexkey">
                                                {{ sex }}
                                            </el-radio>
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
                                            @input="inputFloatNumber($event, 'ruleFormBasic', 'height')"><template
                                                slot="append">CM</template></el-input>
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
                                            @input="inputFloatNumber($event, 'ruleFormBasic', 'weight')"><template
                                                slot="append">KG</template></el-input>
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
                                        <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept"
                                            action="" :auto-upload="false" :on-change="handleChangeWxewm"
                                            :show-file-list="false">
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
            <el-drawer :title="lc('admin_00483')" :append-to-body="true" :visible.sync="drawerJob" :wrapper-closable="false" size="60%">
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
                                        <job_class multiple :max="5" @confirm="confirmJob" :selected="jobSelected">
                                        </job_class>
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
                                        <city_class multiple :max="5" @confirm="confirmCity" :selected="citySelected">
                                        </city_class>
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <div class="TableTite">{{ lc('wap_user_00016') }}</div>
                                </td>
                                <td>
                                    <div class="TableInpt" style="max-width: 700px;">
                                        <el-select v-model="ruleFormJob.minsalary" :placeholder="lc('wap_user_00100')" @change="salaryChange"
                                            style="margin-right:8px;">
                                            <el-option v-for="maxsalary1Val in minsalaryList" :key="maxsalary1Val"
                                                :label="maxsalary1Val" :value="maxsalary1Val">
                                            </el-option>
                                        </el-select>
                                        <el-select v-model="ruleFormJob.maxsalary" :placeholder="lc('wap_user_00100')">
                                            <el-option v-for="maxsalary2Val in maxsalaryList" :key="maxsalary2Val"
                                                :label="maxsalary2Val" :value="maxsalary2Val">
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
        </el-drawer>
        <!-- Edit personal strengths -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_user_00326')" :visible.sync="dialogTag" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00219') }}</div>
                    <div class="">
                        <el-tag :key="tagkey" v-for="(tag, tagkey) in userTag" :disable-transitions="false"
                            @click="checkTag(tag)" :effect="ruleFormTag.tag.indexOf(tag) > -1 ? 'dark' : 'light'">
                            {{ tag }}
                        </el-tag>
                        <el-input style="margin-bottom: 10px;" class="input-new-tag" v-if="inputTag" v-model="tagval"
                            autofoucs size="small" @keyup.enter.native="confirmTag">
                        </el-input>
                        <el-button v-else class="button-new-tag" size="small" @click="showTag">{{ lc('admin_00474') }}
                        </el-button>
                    </div>
                    <div class="wxsettip_small ">{{ lc('wap_00463') }}</div>
                    <el-input type="textarea"
                        :placeholder=lc('admin_vue_00011')
                        v-model="ruleFormTag.description" :autosize="{ minRows: 3, maxRows: 6 }">
                    </el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogTag = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitTag">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Edit work experience -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_00457')" :visible.sync="dialogWork" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_01403') }}</div>
                    <div class="">
                        <el-input v-model="ruleFormWork.name" :placeholder="lc('wap_00137')"></el-input>
                    </div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00091') }}</div>
                    <div class="">
                        <el-input v-model="ruleFormWork.title" :placeholder="lc('wap_user_00045')"></el-input>
                    </div>
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
                    <el-input type="textarea"
                        :placeholder=lc('admin_vue_00012')
                        v-model="ruleFormWork.content" :autosize="{ minRows: 3, maxRows: 6 }">
                    </el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogWork = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitWork">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Edit education -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_00459')" :visible.sync="dialogEdu" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00085') }}</div>
                    <div class="">
                        <el-input v-model="ruleFormEdu.name" :placeholder="lc('wap_user_00044')"></el-input>
                    </div>
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
                    <div class="">
                        <el-input v-model="ruleFormEdu.specialty" :placeholder="lc('wap_user_00042')"></el-input>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogEdu = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitEdu">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Edit training experience -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_00455')" :visible.sync="dialogTraining" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00221') }}</div>
                    <div class="">
                        <el-input v-model="ruleFormTraining.name" :placeholder="lc('admin_00485')"></el-input>
                    </div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00083') }}</div>
                    <div class="">
                        <el-input v-model="ruleFormTraining.title" :placeholder="lc('admin_user_00209')"></el-input>
                    </div>
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
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogTraining = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitTraining">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Edit project experience -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_00465')" :visible.sync="dialogProject" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00099') }}</div>
                    <div class="">
                        <el-input v-model="ruleFormProject.name" :placeholder="lc('wap_user_00046')"></el-input>
                    </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00225') }}</div>
                    <div class="">
                        <el-input v-model="ruleFormProject.title" :placeholder="lc('admin_00486')"></el-input>
                    </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00229') }}</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeProject" type="monthrange" :range-separator="lc('admin_company_00019')"
                            :start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00228') }}</div>
                    <el-input type="textarea" :placeholder=lc('admin_vue_00012') v-model="ruleFormProject.content" :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogProject = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitProject">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Edit other details -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_00216')" :visible.sync="dialogOther" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00103') }}</div>
                    <div class="">
                        <el-input v-model="ruleFormOther.name" :placeholder="lc('admin_00487')"></el-input>
                    </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_00231') }}</div>
                    <el-input type="textarea" v-model="ruleFormOther.content" :placeholder="lc('admin_user_00203')"
                        :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogOther = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitOther">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Edit skills -->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_00461')" :visible.sync="dialogSkill" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_user_00089') }}</div>
                    <div class="">
                        <el-input v-model="ruleFormSkill.name" :placeholder="lc('admin_user_00210')"></el-input>
                    </div>
                    <div class="wxsettip_small ">{{ lc('wap_00458') }}</div>
                    <div class="wxsettip_Sealect">
                        <el-input v-model="ruleFormSkill.longtime" :placeholder="lc('admin_user_00211')">
                            <template slot="append">{{ lc('common_02077') }}</template>
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
                        <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept" action=""
                            :auto-upload="false" :on-change="handleChangeSkillPic" :show-file-list="false">
                            <img v-if="ruleFormSkill.pic_n" :src="ruleFormSkill.pic_n" class="avatar">
                            <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                        </el-upload>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogSkill = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitSkill">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Add resume -->
        <div class="modluDrawer">
            <el-drawer :title="lc('admin_user_00193')" :visible.sync="drawerResume" append-to-body :wrapper-closable="false" size="45%">
                <add :uid="detail.uid" @child-event="closeResume"></add>
            </el-drawer>
        </div>
    </div>
</template>
<script>
module.exports = {
    props: {
        jump_params: {
            type: Object,
            default: () => {
                return {
                    reg_days: '',
                    reg_time: '',
                    login_days: '',
                    login_time: '',
                    search_class: ''
                }
            }
        }
    },
    data: function () {
        return {
            mouseFlag: false,
            mouseOffset: 0,
            loading: false,
            dataText: lc('admin_user_weipin_00026'),
            props: {},
            options: [],
            radio: 1,
            input3: '',
            input: '',
            select: '',
            value: true,
            value1: '',
            checked: '',
            activeName: 'resume',
            drawer: false,
            drawer2: false,
            pxDrawer: false,
            qtDrawer: false,
            jnDrawer: false,
            xmDrawer: false,
            tdjobDrawer: false,
            xqdrawer: false,
            xzdrawer: false,
            zhhbdrawer: false,
            czdrawer: false,
            userysDrawer: false,
            innerDrawer: false,
            gzjlDrawer: false,
            scdrawer: false,
            zzrztc: false,
            wxrztc: false,
            sjrztc: false,
            yxrztc: false,
            xlDrawer: false,
            qyrz: false,
            jobDrawer: false,
            seachbutn: true,
            tableHig: true,
            textarea: '',
            currentPage4: 4,
            dynamicTags: [lc('admin_user_00289'), lc('admin_user_00293'), lc('admin_user_00278'), lc('admin_user_00302'), lc('admin_user_00282'), lc('admin_user_00281'), lc('admin_user_00284'), lc('admin_user_00299'), lc('admin_user_00298')],
            inputVisible: false,
            inputValue: '',
            tableData: [],
            items: [{
                type: '',
                label: lc('admin_user_00149')
            },],
            // Source
            source: {},

            // Search filters
            searchList: [],
            searchForm: {
                type: 1,
                time_type: 'lotime',
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
            searchParams: {},

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

            userStatusNum3: 0,
            userAllNum: 0,

            saveLoading: false,

            // ID card verification
            dialogIdcardRz: false,
            ruleFormIdcardRz: {},
            // mobileverification
            dialogMoblieRz: false,
            ruleFormMobileRz: {},
            // Email verification
            dialogEmailRz: false,
            ruleFormEmailRz: {},

            // {{ lc('admin_user_00292') }}
            dialogAuth: false,
            ruleFormAuth: {},

            // Subsite switch
            dialogDomain: false,
            ruleFormDomain: {},
            domainList: {},

            // {{ lc('wap_com_00427') }}
            drawerDetail: false,
            member: {},
            resume: {},
            expectData: {},

            // {{ lc('common_02022') }}
            user_sex: {},
            userclass_name: {},
            userdata: {},
            industry_index: [],
            industry_name: {},

            // Add
            dialogAdd: false,
            ruleFormAdd: {},
            provinceList: [],
            cityList: [],
            regionList: [],

            // {{ lc('admin_user_00191') }}
            dialogAccount: false,
            ruleFormAccount: {},
            // {{ lc('admin_00542') }}
            dialogAccountMerge: false,
            AccountMergeComname: "",
            ruleFormAccountMerge: {},

            // Delete
            dialogDel: false,
            ruleFormDel: {},

            expectLoading: true,

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
            jobSqLog: {
                page: 1,
                limit: 0,
                total: 0
            },
            // InterviewInvite
            yqmsLog: {
                page: 1,
                limit: 0,
                total: 0
            },
            // Behavior analysis
            behavior: {
                reverseone: true,
                daterange: '',
                times: '',
                activeClass: '',
                fenxiDetail: {},
                dataCount: {},
                logList: [],
                pagenav: 0,
                pageCode: '',
                xialaStatus: true
            },
            // Individual activity
            userLog: {
                page: 1,
                limit: 0,
                list: null
            },
            // Points management
            payLog: {
                page: 1,
                limit: 0,
                total: 0
            },

            // Add resume
            drawerResume: false,

            pic_accept: localStorage.getItem("pic_accept"),

            prevPage: 0,
            prevPage2: 0,
            prevPage3: 0,
            prevPage4: 0,
            pwdreadonly: true
        }
    },
    components: {
        'add': httpVueLoader('./resume_add.vue'),
        'job_class': httpVueLoader('../../../component/job_class.vue'),
        'city_class': httpVueLoader('../../../component/city_class.vue'),
    },
    watch: {
        jump_params: {
            handler(val) {
                if (parseInt(val.reg_days) > 0) {

                    this.searchParams.reg_days = val.reg_days;
                } else if (val.reg_time) {

                    this.searchParams.reg_time = val.reg_time;
                }
                if (parseInt(val.login_days) > 0) {

                    this.searchParams.login_days = val.login_days;
                } else if (val.login_time) {

                    this.searchParams.login_time = val.login_time;
                }
                if (val.search_class) {

                    this.searchClass = val.search_class;
                } else {

                    this.searchParams.reg_days = '';
                    this.searchParams.reg_time = '';
                    this.searchParams.login_days = '';
                    this.searchParams.login_time = '';
                    this.searchClass = '';
                }
            },
            deep: true,
            immediate: true
        }
    },
    created() {
		var that = this;
		let params = window.parent.homeapp.$route.params;
		let query = window.parent.homeapp.$route.query;
		
		if (!$.isEmptyObject(query)) {
			params = {...query,...params};
		}
		
		if (!$.isEmptyObject(params)) {
			delete params.activeName;
			this.getParams(params);
		}
        this.init();
    },
    mounted() {
        var that = this
        setTimeout(function () {
            that.getCountData();
            that.getConfigData();
        }, 200)
    },
    methods: {
        // Prevent password dropdown flicker after repeated password-field clicks
        pwdMousedown(){
            var that = this
            this.pwdreadonly = true
            setTimeout(function(){ that.pwdreadonly = false, 100})
        },
        // {{ lc('common_00444') }}
        pwdchange: function(val){
            var that = this
            if (val == "") {
                this.pwdreadonly = true
                setTimeout(function(){ that.pwdreadonly = false, 100})
            }
        },
        // Temporarily change password readonly state to prevent browser password history suggestions
        readonlyCtl: function(res){
            var that = this
            setTimeout(function(){
                that.pwdreadonly = res
            }, 200)
        },
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
        // Check before jumping to member center
        memberCheck: function (uid, usertype) {
            var that = this
            var tip = ''
            if (usertype != '1') {
                if (usertype == '0') {
                    tip = lc('admin_user_00267')
                } else {
                    if (usertype == '2') {
                        var u = lc('admin_user_00301');
                    }
                    tip = lc('admin_user_00275') + u + lc('admin_user_00268')
                }
            }
            if (tip) {
                delConfirm(this, {}, function (params) {
                    that.jumpToMember(uid);
                }, tip)
            } else {
                that.jumpToMember(uid);
            }
        },
        // Jump to member center
        jumpToMember: function (uid) {
            let tmpWin = window.open('', '_blank')
            var params = { uid: uid }
            httpPost('m=user&c=users_member&a=Imitate', params).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    tmpWin.location = res.data.url
                }
            }).catch(function (e) {
                tmpWin.close()
            })
        },
        init() {
            this.search();
        },
        getParams: function (params = {}, search = false) {
            var that = this;
            for (let i in params) {
                if(typeof that.searchForm[i]!='undefined'){
					that.searchForm[i] = params[i];
				}
            }

            if (search) {
                this.search();
            }
        },
        resetSearch() {
            this.searchForm = {
                type: 1
            };
            this.limit = 0;
        },

        statusSearch(status) {
            this.resetSearch();
            this.searchForm.status = status;
            this.search();
        },

        getCountData() {
            let that = this;

            httpPost('m=user&c=users_member&a=userNum', {}, { hideloading: true }).then(function (response) {
                let res = response.data;

                that.userStatusNum3 = res.userStatusNum3;
                that.userAllNum = res.userAllNum;
            })
        },
        getConfigData() {
            let that = this;

            httpPost('m=user&c=users_member&a=getConfigData', {}, { hideloading: true }).then(function (response) {
                let res = response.data;
                that.searchList = res.data.search_list;
                that.source = res.data.source;
                that.domainList = res.data.domainList;
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

            if (that.searchParams.reg_days) {
                searchForm.reg_days = that.searchParams.reg_days;
            } else if (that.searchParams.reg_time) {
                searchForm.reg_time = that.searchParams.reg_time;
            } else if (that.searchParams.login_days) {
                searchForm.login_days = that.searchParams.login_days;
            } else if (that.searchParams.login_time) {
                searchForm.login_time = that.searchParams.login_time;
            }
            httpPost('m=user&c=users_member', { ...params, ...searchForm }, { hideloading: true }).then(function (response) {
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
                if (that.prevPage != that.page) {
                    that.prevPage = that.page;
                    that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    scrollToTop()
                }
                that.loading = false;
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
            } else if (this.multipleSelection.length == 0) {
                message.error(lc('admin_user_weipin_00001'));
                return false;
            }

            let idArr = [];
            this.multipleSelection.forEach(function (item) {
                idArr.push(item.uid);
            })
            this.idArr = idArr;

            if (type == 'del') {
                this.openDel();
            } else if (type == 'domain') {
                this.openDomain();
            } else if (type == 'auth') {
                this.openAuth();
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
            } else { // {{ lc('common_01711') }}
                this.ruleFormDel = {
                    del: this.list[idx].uid,
                    delAccount: 0
                }
            }

            this.dialogDel = true;
        },
        delSubmit() {
            let that = this,
                ruleForm = this.ruleFormDel;

            that.saveLoading = true;

            httpPost('m=user&c=users_member&a=del', ruleForm).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogDel = false;
                    that.refreshList = false; // Close details dialog on delete without triggering close-refresh event
                    that.drawerDetail = false;
                    that.getList();
                    that.$refs.multipleTable.clearSelection();
                    message.success(res.msg)
                }
            })
        },

        // ID card verification
        idcardRz(row) {
            this.detail = row;
            this.ruleFormIdcardRz = {
                uid: row.uid,
                r_status: row.idcard_status,
                statusbody: ''
            }
            this.dialogIdcardRz = true;
        },
        idcardRzSubmit() {
            let that = this,
                ruleForm = this.ruleFormIdcardRz;

            if (typeof ruleForm.r_status == 'undefined' || ruleForm.r_status === '') {
                message.error(lc('admin_user_weipin_00015'));
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_member&a=usercert', ruleForm).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogIdcardRz = false;
                    that.getList();
                    that.$refs.multipleTable.clearSelection();
                    message.success(res.msg)
                }
            })
        },
        // mobileverification
        moblieRz(row) {
            this.detail = row;
            this.ruleFormMobileRz = {
                uid: row.uid,
                moblie: row.telphone,
                mstatus: row.moblie_status
            }
            this.dialogMoblieRz = true;
        },
        moblieRzSubmit() {
            let that = this,
                ruleForm = this.ruleFormMobileRz;

            if (!ruleForm.moblie) {
                message.error(lc('wap_js_00119'));
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_member&a=usercert', ruleForm).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogMoblieRz = false;
                    that.getList();
                    that.$refs.multipleTable.clearSelection();
                    message.success(res.msg)
                }
            })
        },
        // Email verification
        emailRz(row) {
            this.detail = row;
            this.ruleFormEmailRz = {
                uid: row.uid,
                email: row.email,
                estatus: row.email_status,
            };
            this.dialogEmailRz = true;
        },
        emailRzSubmit() {
            let that = this,
                ruleForm = this.ruleFormEmailRz;

            if (!ruleForm.email) {
                message.error(lc('wap_00697'));
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_member&a=usercert', ruleForm).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogEmailRz = false;
                    that.getList();
                    that.$refs.multipleTable.clearSelection();
                    message.success(res.msg)
                }
            })
        },

        // Query mobile location
        getMobileAddress(index) {
            let that = this,
                row = that.list[index];

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=index&c=getMobileAddress', {
                uid: row.uid,
                moblie: row.telphone
            }).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.list[index].moblie_address = res.msg;
                    message.success(lc('admin_user_00294'));
                }
            })
        },
        // Query IP location
        getIpAddress(index) {
            let that = this,
                row = that.list[index];

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=index&c=getIpAddress', {
                uid: row.uid,
                ip: row.login_ip
            }).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.list[index].login_address = res.msg;
                    message.success(lc('admin_user_00294'));
                }
            })
        },

        openDomain(row) {
            if (typeof row == 'undefined') { // {{ lc('admin_yunying_00106') }}
                this.detail = {};
                this.$set(this.ruleFormDomain, 'uid', this.idArr);
                this.$set(this.ruleFormDomain, 'did', '');
            } else { // Single operation
                this.detail = row;
                this.$set(this.ruleFormDomain, 'uid', row.uid);
                this.$set(this.ruleFormDomain, 'did', row.did && this.domainList[row.did] ? '' + row.did : '');
            }

            this.dialogDomain = true;
        },

        saveDomain() {
            let that = this,
                ruleForm = that.ruleFormDomain;
            if (ruleForm.did === '') {
                message.error(lc('admin_user_weipin_00002'));
                return false;
            }

            that.saveLoading = true;

            httpPost('m=user&c=users_member&a=checksitedid', ruleForm).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogDomain = false;
                    if (typeof ruleForm.uid == 'object') { // {{ lc('member_com_00055') }}
                        that.getList();
                    } else { // {{ lc('common_01711') }}
                        that.refreshList = true;
                        // Reload details
                        that.getDetail(ruleForm.uid);
                    }
                    message.success(res.msg)
                }
            })
        },
        // {{ lc('admin_user_00292') }}
        openAuth() {
            this.dialogAuth = true;
            this.ruleFormAuth = {
                batchfirm: true,
                uid: this.idArr,
                type: [],
                status: ''
            };
        },
        authSubmit() {
            let that = this,
                ruleForm = this.ruleFormAuth;

            if (typeof ruleForm.type == 'undefined' || ruleForm.type.length == 0) {
                message.error(lc('admin_01288'));
                return false;
            }

            if (typeof ruleForm.status == 'undefined' || ruleForm.status === '') {
                message.error(lc('admin_01289'));
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_member&a=usercert', ruleForm).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogAuth = false;
                    that.getList();
                    that.$refs.multipleTable.clearSelection();
                    message.success(res.msg)
                }
            })
        },

        // {{ lc('admin_00551') }}
        openSqLog(index, row) {
            this.activeName = 'sqlog';
            this.openDetail(index, row);
        },

        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g, '');
        },
        inputPassword(val, form, key) {
            this.$data[form][key] = val.replace(/^ +| +$/g, '');
        },
        inputFloatNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9.]/g, '');
        },
        inputIdcard(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9Xx.]/g, '');
        },

        // Open details
        openDetail(index, row) {
            this.expectLoading = true;
            this.index = index;
            this.detail = row;
            this.getDetail();

            // Load default tag data when a default tag exists
            if (this.activeName == 'sqlog') {
                this.getJobSqLog();
            }

            this.drawerDetail = true;
        },
        // Close details
        closedDetail() {
            if (this.refreshList) {
                this.getList();
            }
            this.resetDetail();
        },
        // Reset loaded detail data
        resetDetail() {
            this.activeName = 'resume';
            // {{ lc('admin_00551') }}
            this.$set(this.$data, 'jobSqLog', {
                page: 1,
                limit: 0,
                total: 0
            });
            // InterviewInvite
            this.$set(this.$data, 'yqmsLog', {
                page: 1,
                limit: 0,
                total: 0
            });
            // Behavior analysis
            this.behavior = {
                reverseone: true,
                daterange: '',
                times: '',
                activeClass: '',
                fenxiDetail: {},
                dataCount: {},
                logList: [],
                pagenav: 0,
                pageCode: '',
                xialaStatus: true
            };
            // {{ lc('admin_00555') }}
            this.userLog = {
                page: 1,
                limit: 0,
                list: null
            };
            // {{ lc('admin_00556') }}
            this.$set(this.$data, 'payLog', {
                page: 1,
                limit: 0,
                total: 0
            });
        },
        // Get details
        async getDetail() {
            let response = await httpPost('m=user&c=users_member&a=edit', { uid: this.detail.uid });
            let that = this,
                res = response.data,
                data = res.data;

            this.member = data.member;
            this.member.username = this.member.username ? this.member.username : '';
            this.resume = data.resume ? data.resume : {};
            this.expectData = data.expectData;


            this.user_sex = data.user_sex;
            this.userclass_name = data.userclass_name;
            this.userdata = data.userdata;
            this.industry_index = data.industry_index;
            this.industry_name = data.industry_name;
            this.expectLoading = false;
        },

        openAdd() {
            let that = this;
            httpPost('m=user&c=users_member&a=add', {}).then(function (response) {
                let res = response.data;

                that.ruleFormAdd = {};
                that.dialogAdd = true;
            })
        },

        saveAdd() {
            let that = this,
                ruleForm = that.ruleFormAdd;

            if (typeof ruleForm.username === 'undefined' || $.trim(ruleForm.username) == "") {
                message.error(lc('wap_00208'));
                return false;
            }
            if (typeof ruleForm.password === 'undefined' || $.trim(ruleForm.password) == "") {
                message.error(lc('wap_00703'));
                return false;
            }
            if (typeof ruleForm.moblie === 'undefined' || $.trim(ruleForm.moblie) == "") {
                message.error(lc('wap_js_00119'));
                return false;
            } else if (!isjsMobile(ruleForm.moblie)) {
                message.error(lc('wap_js_00117'));
                return false;
            }

            httpPost('m=user&c=users_member&a=add', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogAdd = false;
                    that.getList();
                    message.success(res.msg);
                }
            })
        },

        toMember(row) {
            let that = this;

            if (row.usertype != '1') {
                if (row.usertype == '0') {
                    delConfirm(that, params, function (params) {
                        that.getMemberUrl(row.uid);
                    }, lc('admin_user_00267'))
                } else {
                    var usertype = '';
                    if (row.usertype == '2') {
                        usertype = lc('admin_user_00301');
                    }

                    delConfirm(that, params, function (params) {
                        that.getMemberUrl(row.uid);
                    }, lc('admin_user_00275') + usertype + lc('admin_user_00268'))
                }
            } else {
                that.getMemberUrl(row.uid);
            }
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


        handleClick(tab, event) {
            if (tab.name == 'sqlog') {
                if (typeof this.jobSqLog.list === 'undefined') {
                    this.getJobSqLog();
                }
            } else if (tab.name == 'yqms') {
                if (typeof this.yqmsLog.list === 'undefined') {
                    this.getYqmsLog();
                }
            } else if (tab.name == 'log') {
                if (!this.userLog.list) {
                    this.getUserLog();
                }
            } else if (tab.name == 'pay') {
                if (!this.payLog.list) {
                    this.getPayLog();
                }
            }
        },

        // {{ lc('admin_user_00191') }}
        openAccount() {
            let member = this.member;
            this.ruleFormAccount = {
                uid: member.uid,
                username: member.username,
                password: '',
                status: member.status,
                lock_info: member.lock_info
            };
            this.dialogAccount = true;
        },
        submitAccount() {
            let that = this,
                ruleForm = that.ruleFormAccount;
            that.saveLoading = true;
            httpPost('m=user&c=users_member&a=saveUser', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogAccount = false;
                    that.refreshList = true;
                    // Reload details
                    that.getDetail(ruleForm.uid);
                    message.success(res.msg);
                }
            }).finally(function () {
                setTimeout(function () {
                    that.saveLoading = false;
                }, 2000);
            });
        },

        // {{ lc('admin_00542') }}
        openAccountMerge() {
            let member = this.member;
            this.AccountMergeComname = '';
            this.ruleFormAccountMerge = {
                uid: member.uid,
                com_uid: '',
                mobile: 1,
                email: 1,
                QQ: 1,
                wx: 1,
                sina: 1,
            };
            this.dialogAccountMerge = true;
        },
        querySearchCom(queryString, cb) {
            if (queryString === '') {
                cb([]);
                return true;
            }
            httpPost('m=user&c=users_member&a=searchCom', { com_name: queryString }).then(function (response) {
                let res = response.data,
                    data = res.data;

                cb(data.companyList);
            })
        },
        handleSelectCom(item) {
            this.ruleFormAccountMerge.com_uid = item.uid;
        },
        submitAccountMerge() {
            let that = this,
                ruleForm = that.ruleFormAccountMerge;

            if (that.AccountMergeComname == '' || ruleForm.com_uid == '') {
                message.error(lc('admin_user_00271'));
                return false;
            }

            httpPost('m=user&c=users_member&a=merge', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {

                    // Reload details
                    //that.getDetail();
                    message.success(res.msg, function () {
                        that.dialogAccountMerge = false;
                        that.refreshList = true;
                        that.drawerDetail = false;
                        that.getList();
                    });
                }
            })
        },

        // reset password
        resetPassword(row) {
            let that = this;
            delConfirm(that, { uid: row.uid }, function (params) {
                httpPost('m=user&c=users_member&a=reset_pw', params).then(function (res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.alert(lc('admin_user_00141') + row.username + " {{ lc('admin_user_00115') }}");
                    }
                })
            }, lc('admin_user_00274'))
        },

        // Edit profile
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
                    that.saveLoading = false;
                    message.error(res.msg);
                } else {
                    that.drawerBasic = false;
                    that.refreshList = true;
                    // Reload details
                    that.getDetail(ruleForm.uid);
                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
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
                    message.error(lc('wap_user_00060'));
                    return false;
                }
                if (tag.length >= 5) {
                    message.error(lc('admin_user_00206'));
                    return false;
                }
                if (userTag.indexOf(tagval) > -1) {
                    message.error(lc('wap_user_00074'));
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
                    message.error(lc('admin_user_00206'));
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
                message.error(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.tag.length > 5) {
                message.error(lc('admin_user_00206'));
                return false;
            }
            if (ruleForm.description == '' || ruleForm.description == null) {
                message.error(lc('admin_01319'));
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=saveTag', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    that.saveLoading = false;
                    message.error(res.msg);
                } else {
                    that.dialogTag = false;
                    that.refreshList = true;
                    that.resume.arrayTag = ruleForm.tag;
                    that.resume.description = ruleForm.description;
                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
        },
        // Job intention
        openJob() {
            let resume = this.resume,
                expect = this.expectData.expect;

            this.jobSelected = expect.jobnameArr;
            this.citySelected = expect.citynameArr;

            let salaryList = deepClone(this.expectData.salary),
                maxsalaryList = [];
            salaryList.forEach(function (item, index) {
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
            salaryList.splice(salaryList.length - 1, 1);
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
            this.expectData.salary.forEach(function (item, index) {
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
                message.error(lc('admin_00484'));
                return false;
            }
            if (ruleForm.job_classid == "") {
                message.error(lc('admin_vue_00013'));
                return false;
            }
            if (ruleForm.city_classid == '') {
                message.error(lc('admin_vue_00014'));
                return false;
            }
            if (ruleForm.minsalary == "" || ruleForm.minsalary == "0") {
                message.error(lc('admin_vue_00015'));
                return false;
            }
            if (ruleForm.maxsalary && parseInt(ruleForm.maxsalary) <= parseInt(ruleForm.minsalary)) {
                message.error(lc('member_user_00095'));
                return false;
            }
            if (ruleForm.report == "") {
                message.error(lc('wap_00980'));
                return false;
            }
            if (ruleForm.type == "") {
                message.error(lc('wap_js_00163'));
                return false;
            }
            if (ruleForm.jobstatus == "") {
                message.error(lc('wap_00934'));
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=saveExpect', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    that.saveLoading = false;
                    message.error(res.msg);
                } else {
                    that.drawerJob = false;
                    that.refreshList = true;
                    // Reload details
                    that.getDetail(ruleForm.uid);
                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
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
                message.error(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.name == "") {
                message.error(lc('wap_00137'));
                return false;
            }
            if (ruleForm.sdate == "") {
                message.error(lc('admin_user_00213'));
                return false
            }
            ruleForm.sdate = formatMonth(ruleForm.sdate);
            if (ruleForm.edate != '') {
                if (ruleForm.sdate >= ruleForm.edate) {
                    message.error(lc('admin_user_00201'));
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
                    that.saveLoading = false;
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

                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
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
                message.error(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.name == "") {
                message.error(lc('wap_user_00044'));
                return false;
            }
            if (daterangeEdu.length == 0) {
                message.error(lc('admin_vue_00016'));
                return false
            }
            if (ruleForm.education == "") {
                message.error(lc('wap_user_00049'));
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
                    that.saveLoading = false;
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

                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
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
                message.error(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.name == "") {
                message.error(lc('admin_00485'));
                return false;
            }
            if (daterangeTraining.length == 0) {
                message.error(lc('admin_user_00212'));
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
                    that.saveLoading = false;
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

                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
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
                message.error(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.name == "") {
                message.error(lc('admin_user_00210'));
                return false;
            }
            if (ruleForm.ing == "") {
                message.error(lc('wap_user_00072'));
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
                    that.saveLoading = false;
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

                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
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
                message.error(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.name == "") {
                message.error(lc('wap_user_00046'));
                return false;
            }
            if (daterangeProject.length == 0) {
                message.error(lc('admin_user_00214'));
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
                    that.saveLoading = false;
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

                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
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
                message.error(lc('admin_user_00207'));
                return false;
            }
            if (ruleForm.name == "") {
                message.error(lc('admin_00487'));
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=other', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    that.saveLoading = false;
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

                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
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

        // Application records
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
                    uid: that.detail.uid
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
                if (that.prevPage2 != jobSqLog.page) {
                    that.prevPage2 = jobSqLog.page;
                    that.$refs.table2.bodyWrapper.scrollTop = 0;
                }
                that.jobSqLog = jobSqLog;
                // that.$set(that.$data, 'jobSqLog', jobSqLog);
                that.loading = false;

                if (that.jobSqLog.list.length === 0) {
                    that.dataText = lc('wap_js_00113');
                }
            })
        },
        // InterviewInvite
        handleSizeChangeYqmsLog(val) {
            this.yqmsLog.limit = val;
            this.getYqmsLog();
        },
        handleCurrentChangeYqmsLog(val) {
            this.yqmsLog.page = val;
            this.getYqmsLog();
        },
        getYqmsLog() {
            let that = this,
                yqmsLog = deepClone(that.yqmsLog),
                params = {
                    page: yqmsLog.page,
                    limit: yqmsLog.limit,
                    uid: that.resume.uid
                };

            httpPost('m=user&c=users_member&a=yqmsLog', params).then(function (response) {
                let res = response.data,
                    data = res.data;

                yqmsLog.list = data.list;
                yqmsLog.total = parseInt(data.total);
                yqmsLog.pageSizes = data.page_sizes;
                if (yqmsLog.limit === 0) {
                    yqmsLog.limit = parseInt(data.limit); // Use default count from system config
                }
                if (yqmsLog.page > data.page) {
                    yqmsLog.page = parseInt(data.page); // Use latest page after the last page is deleted
                }
                if (that.prevPage3 != yqmsLog.page) {
                    that.prevPage3 = yqmsLog.page;
                    that.$refs.table3.bodyWrapper.scrollTop = 0;
                }
                that.yqmsLog = yqmsLog;

                if (that.yqmsLog.list.length === 0) {
                    that.dataText = lc('wap_js_00113');
                }
            })
        },
        
        // Individual activity
        getUserLog() {
            let that = this,
                userLog = deepClone(that.userLog),
                params = {
                    page: userLog.page,
                    limit: userLog.limit,
                    uid: that.resume.uid
                };

            httpPost('m=user&c=users_member&a=log', params).then(function (response) {
                let res = response.data,
                    data = res.data,
                    list = userLog.list ? userLog.list : {};

                data.list.forEach(function (item) {
                    if (typeof list[item.date_n] === 'undefined') {
                        list[item.date_n] = {
                            week: item.week,
                            list: [item]
                        };
                    } else {
                        list[item.date_n].list.push(item);
                    }
                });

                userLog.list = list;
                userLog.total = parseInt(data.total);
                userLog.last_page = parseInt(data.last_page);
                userLog.pageSizes = data.page_sizes;
                if (userLog.limit === 0) {
                    userLog.limit = parseInt(data.limit); // Use default count from system config
                }
                if (userLog.page > data.page) {
                    userLog.page = parseInt(data.page); // Use latest page after the last page is deleted
                }

                that.userLog = userLog;
                that.saveLoading = false;
            })
        },
        handleCurrentChangeUserLog() {
            if (this.saveLoading) {
                return false;
            }
            this.saveLoading = true;
            this.userLog.page++;
            this.getUserLog();
        },
        // {{ lc('admin_00556') }}
        handleSizeChangePayLog(val) {
            this.payLog.limit = val;
            this.getPayLog();
        },
        handleCurrentChangePayLog(val) {
            this.payLog.page = val;
            this.getPayLog();
        },
        getPayLog() {
            let that = this,
                payLog = deepClone(that.payLog),
                params = {
                    page: payLog.page,
                    limit: payLog.limit,
                    uid: that.resume.uid
                };
            that.loading = true;
            httpPost('m=user&c=users_member&a=payLog', params).then(function (response) {
                let res = response.data,
                    data = res.data;

                payLog.list = data.list;
                payLog.total = parseInt(data.total);
                payLog.pageSizes = data.page_sizes;
                if (payLog.limit === 0) {
                    payLog.limit = parseInt(data.limit); // Use default count from system config
                }
                if (payLog.page > data.page) {
                    payLog.page = parseInt(data.page); // Use latest page after the last page is deleted
                }
                if (that.prevPage4 != payLog.page) {
                    that.prevPage4 = payLog.page;
                    that.$refs.table4.bodyWrapper.scrollTop = 0;
                }
                that.payLog = payLog;
                that.loading = false;

                if (that.payLog.list.length === 0) {
                    that.dataText = lc('wap_js_00113');
                }
            })
        },

        // Add resume
        openResume(row) {
            this.detail = row;
            this.detail.uid = parseInt(row.uid);
            this.drawerResume = true;
        },
        closeResume() {
            this.drawerResume = false;
            this.getList();
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
.pad_lr_20 {
    padding: 0 20px;
}

.moduleElTableHig {
    height: calc(100% - 140px) !important
}

.tableSeachInptsmall .el-input {
    width: initial
}

.tableSeachInptsmall .el-select {
    margin-right: 0 !important;
    /*padding-left: 20px;*/
}

.el-input-group__prepend {
    background-color: #fff;
    padding: 0 0 0 5px
}

.el-tag {
    margin-right: 10px;
    margin-bottom: 10px
}

.button-new-tag {
    margin-left: 10px;
    height: 32px;
    line-height: 30px;
    padding-top: 0;
    padding-bottom: 0
}

.input-new-tag {
    width: 90px;
    margin-left: 10px;
    vertical-align: bottom
}

.el-dialog__body {
    padding: 0 20px
}

.cominfocz {
    padding: 15px 0;
    position: fixed;
    overflow: hidden;
    right: 0;
    bottom: 0;
    width: calc(95% - 20px);
    background: #fff;
    z-index: 222;
    border-top: 1px solid #eee
}

.el-upload--picture-card {
    width: 80px;
    height: 80px;
    line-height: 80px
}

.el-upload-list--picture-card .el-upload-list__item {
    width: 80px;
    height: 80px;
    line-height: 76px
}

/* Upload styles start */
.avatar-uploader .el-upload {
    border: 1px dashed #d9d9d9;
    border-radius: 6px;
    cursor: pointer;
    position: relative;
    overflow: hidden
}

.avatar-uploader .el-upload:hover {
    border-color: #409eff
}

.avatar-uploader-icon {
    font-size: 28px;
    color: #8c939d;
    width: 100px;
    height: 100px;
    line-height: 100px;
    text-align: center
}

.avatar {
    width: 100px;
    height: 100px;
    display: block
}

.fenpeizhand .toolClasList {
    flex-wrap: wrap
}

.toolClasTipse {
    overflow: hidden;
    position: relative;
    padding-left: 75px;
    width: calc(100% - 75px)
}

.toolClasTipse .el-alert {
    overflow: hidden;
    position: relative;
    padding: 6px 0;
    background: 0 0
}

.moduleElTabUserall {
    padding: 0;
    margin: 0;
    height: calc(100% - 134px) !important;
    width: 100%
}

.modulElTableGaiUsall {
    height: calc(100% - 134px) !important;
}

/* Upload styles end */

.shinfo .el-tab-pane {
    height: 100%;
}

@media (max-width: 1480px) {
    .moduleElTabUserall {
        height: calc(100% - 180px) !important;
    }

    .modulElTableGaiUsall {
        height: calc(100% - 134px) !important;
    }
}</style>