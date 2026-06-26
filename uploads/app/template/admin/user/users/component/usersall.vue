<template>
    <div class="moduleElHight" :class="searchClass == 'drawer' ? 'pad_lr_20' : ''">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall" style="padding: 2px 0;">
                <el-input v-model="searchForm.keyword" @keyup.enter.native="search" placeholder="{yun:}t key='admin_user_00158'{/yun}" size="small"
                    clearable>
                    <el-select v-model="searchForm.type" size="small" slot="prepend" placeholder="{yun:}t key='admin_user_00140'{/yun}" style="padding-left:20px;">
                        <el-option label="{yun:}t key='admin_user_00140'{/yun}" :value="1"></el-option>
                        <el-option label="{yun:}t key='wap_00529'{/yun}" :value="2"></el-option>
                        <el-option label="{yun:}t key='wap_01619'{/yun}" :value="3"></el-option>
                        <el-option label="EMAIL" :value="4"></el-option>
                        <el-option label="{yun:}t key='admin_user_00130'{/yun}" :value="5"></el-option>
                        <el-option label="IP" :value="6"></el-option>
                    </el-select>
                </el-input>
            </div>
            <!--收起部分-->
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="searchForm.time_type" size="small" slot="prepend" placeholder="{yun:}t key='admin_user_00135'{/yun}" clearable @change="handleTimeChange">
                    <el-option label="{yun:}t key='admin_user_00129'{/yun}" value="adtime"></el-option>
                    <el-option label="{yun:}t key='admin_user_00134'{/yun}" value="lotime"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmalltwo" :class="{ 'searchbutnOnff': seachbutn }">
                <el-date-picker v-model="searchForm.times" type="daterange" align="right" unlink-panels range-separator="{yun:}t key='admin_company_00019'{/yun}" start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}" :picker-options="timeOptions" value-format="yyyy-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
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
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" plain icon="el-icon-document-add" size="mini" @click="openAdd">{yun:}t key='admin_user_00305'{/yun}</el-button>
            </div>
            <div class="tableSeachzk" :class="{ 'searchbutnKai': seachbutn }" style="margin-bottom: 11px;">
                <el-button type="info" class="zhankai" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>{yun:}t key='admin_user_00145'{/yun}
                    <i class="el-icon-arrow-down el-icon--right"></i>
                </el-button>
                <el-button type="info" class="shouqi" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>{yun:}t key='admin_user_00144'{/yun}
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
                <el-table-column prop="uid" label="{yun:}t key='admin_user_00130'{/yun}" width="100" sortable="custom"></el-table-column>
                <el-table-column label="{yun:}t key='admin_00545'{/yun}" min-width="110" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class="username">{{ scope.row.username_n }}</div>
                        </div>
                        <div class="yhm">
                            <el-link @click="memberCheck(scope.row.uid, scope.row.usertype)" :underline="false">{{
                                scope.row.username
                            }}
                            </el-link>
                            <el-tooltip v-if="scope.row.r_status == '2'" class="item" effect="dark" content="{yun:}t key='admin_user_00138'{/yun}"
                                placement="top-start">
                                <i class="el-icon-lock" style="color: orange"></i>
                            </el-tooltip>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_user_00117'{/yun}" min-width="130">
                    <template slot-scope="scope">
                        <div class="moduleProps" v-if="scope.row.telphone">
                            <span>{{ scope.row.telphone }}</span>
                            <span v-if="scope.row.moblie_address" class="gsd">
                                {{ scope.row.moblie_address }}
                            </span>
                            <el-link v-else type="primary" :underline="false"
                                @click="getMobileAddress(scope.$index)">{yun:}t key='admin_00433'{/yun}</el-link>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_user_00287'{/yun}" min-width="110" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <div class="rz_box">
                            <el-tooltip v-if="scope.row.idcard_status == 1" class="item" effect="dark" content="{yun:}t key='resume_00008'{/yun}"
                                placement="top-start">
                                <el-button type="text" @click="idcardRz(scope.row)">
                                    <i class="rzicon rzicon_zzyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" content="{yun:}t key='member_user_00502'{/yun}" placement="top-start">
                                <el-button type="text" @click="idcardRz(scope.row)">
                                    <i class="rzicon rzicon_zzwrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-if="scope.row.moblie_status == 1" class="item" effect="dark" content="{yun:}t key='member_user_00117'{/yun}"
                                placement="top-start">
                                <el-button type="text" @click="moblieRz(scope.row)">
                                    <i class="rzicon rzicon_sjyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" content="{yun:}t key='wap_01245'{/yun}" placement="top-start">
                                <el-button type="text" @click="moblieRz(scope.row)">
                                    <i class="rzicon rzicon_sjwrz"></i>
                                </el-button>
                            </el-tooltip>
                        </div>
                        <div class="rz_box">
                            <el-tooltip v-if="scope.row.email_status_n == 1" class="item" effect="dark" content="{yun:}t key='admin_user_00286'{/yun}"
                                placement="top-start">
                                <el-button type="text" @click="emailRz(scope.row)">
                                    <i class="rzicon rzicon_yxyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" content="{yun:}t key='wap_01246'{/yun}" placement="top-start">
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
                <el-table-column label="{yun:}t key='admin_00510'{/yun}" min-width="130" align="center">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class="username">{{ scope.row.sq_num > 0 ? scope.row.sq_num : 0 }}</div>
                            <el-link v-if="scope.row.sq_num > 0" type="primary" :underline="false"
                                @click="openSqLog(scope.$index, scope.row)">{yun:}t key='wap_com_00427'{/yun}</el-link>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="login_date" label="{yun:}t key='admin_user_00121'{/yun}" min-width="150" sortable="custom">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span class="gsd">{{ scope.row.reg_date_n }}</span>
                            <span v-if="scope.row.login_date_n">{{ scope.row.login_date_n }}</span>
                            <span v-else>{yun:}t key='admin_user_00139'{/yun}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_00546'{/yun}" min-width="150">
                    <template slot-scope="scope">
                        <div class="moduleProps" v-if="scope.row.def_job != '0'">
                            <el-link type="primary" :underline="false"
                                @click="openDetail(scope.$index, scope.row)">{yun:}t key='wap_user_00217'{/yun}</el-link>
                        </div>
                        <div class="moduleProps" v-else>
                            <el-link type="primary" :underline="false" @click="openResume(scope.row)">{yun:}t key='admin_user_00296'{/yun}</el-link>
                        </div>
                        <span class="gsd">{{ source[scope.row.source] }}</span>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_vue_00026'{/yun}" min-width="130">
                    <template slot-scope="scope">
                        <div class="moduleProps">

                            <div v-if="scope.row.login_ip">
                                <span>{{ scope.row.login_ip }}</span>
                                <span v-if="scope.row.login_address" class="gsd"> {{ scope.row.login_address }}</span>
                                <el-link v-else type="primary" :underline="false"
                                    @click="getIpAddress(scope.$index)">{yun:}t key='admin_00433'{/yun}</el-link>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='member_user_00181'{/yun}" fixed="right" width="60">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.r_status == '2'" class="admin_state3">{yun:}t key='admin_user_00138'{/yun}</span>
                            <span v-else class="admin_state1">{yun:}t key='admin_user_00149'{/yun}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80" fixed="right" align="center">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="openDetail(scope.$index, scope.row)">{yun:}t key='member_com_00380'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging" style="height: initial; flex-wrap: wrap; padding-top: 10px;">
            <div  style="width:100%;">
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
                    @change="checkAll">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button @click="batch('del')" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
                <el-button @click="batch('domain')" size="mini">{yun:}t key='admin_user_00279'{/yun}</el-button>
                <el-button @click="batch('auth')" size="mini">{yun:}t key='admin_user_00292'{/yun}</el-button>
            </div>
            <div class="modulePagNum" style="padding-top: 8px;">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page="page" :page-sizes="pageSizes" :page-size="limit"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <!--删除弹窗-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_00547'{/yun}" :visible.sync="dialogDel" :with-header="true" append-to-body :show-close="true"
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
        <!--身份认证弹窗-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_01030'{/yun}" :visible.sync="dialogIdcardRz" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_00529'{/yun}</div>
                    <el-input :value="detail.username_n" :disabled="true"></el-input>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00285'{/yun}</div>
                    <el-input :value="detail.idcard" :disabled="true"></el-input>
                    <div class="wxsettip_small ">{yun:}t key='admin_00533'{/yun}</div>
                    <div class="zzrz_img">
                        <div class="zzrz_imgpreview">
                            <el-image v-if="detail.idcard_pic" style="width: 80px; height: 80px" :src="detail.idcard_pic"
                                :preview-src-list="detail.idcard_pic">
                            </el-image>
                            <span v-else>{yun:}t key='admin_user_00277'{/yun}</span>
                        </div>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_weipin_00032'{/yun}</div>
                    <el-radio v-model="ruleFormIdcardRz.r_status" label="0">{yun:}t key='admin_user_00300'{/yun}</el-radio>
                    <el-radio v-model="ruleFormIdcardRz.r_status" label="1">{yun:}t key='wap_user_00128'{/yun}</el-radio>
                    <div class="wxsettip_small ">{yun:}t key='member_user_00062'{/yun}</div>
                    <el-input type="textarea" :rows="2" placeholder="" v-model="ruleFormIdcardRz.statusbody"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogIdcardRz = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="idcardRzSubmit">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--手机认证弹窗-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='member_com_00071'{/yun}" :visible.sync="dialogMoblieRz" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_01619'{/yun}</div>
                    <el-input placeholder="" v-model="ruleFormMobileRz.moblie"></el-input>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_weipin_00032'{/yun}</div>
                    <el-radio v-model="ruleFormMobileRz.mstatus" label="1">{yun:}t key='wap_user_00128'{/yun}</el-radio>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogMoblieRz = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="moblieRzSubmit">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--邮箱认证弹窗-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_com_00186'{/yun}" :visible.sync="dialogEmailRz" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00303'{/yun}</div>
                    <el-input placeholder="" v-model="ruleFormEmailRz.email"></el-input>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_weipin_00032'{/yun}</div>
                    <el-radio v-model="ruleFormEmailRz.estatus" label="1">{yun:}t key='wap_user_00128'{/yun}</el-radio>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogEmailRz = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="emailRzSubmit">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--投递岗位弹窗-->
        <!--新增个人用户弹窗-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_00548'{/yun}" :visible.sync="dialogAdd" :append-to-body="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00140'{/yun}</div>
                    <el-input placeholder="{yun:}t key='wap_00208'{/yun}" v-model="ruleFormAdd.username"></el-input>
                    <div class="wxsettip_small ">{yun:}t key='wap_00702'{/yun}</div>
                    <el-input @mousedown.native="pwdMousedown" @input="pwdchange" @focus="readonlyCtl(false)" @blur="readonlyCtl(true)" :readonly="pwdreadonly" placeholder="{yun:}t key='wap_00703'{/yun}" v-model="ruleFormAdd.password" ></el-input>
                    <div class="wxsettip_small ">{yun:}t key='member_user_00282'{/yun}</div>
                    <el-input placeholder="{yun:}t key='wap_00697'{/yun}" v-model="ruleFormAdd.email"></el-input>
                    <div class="wxsettip_small ">{yun:}t key='wap_01619'{/yun}</div>
                    <el-input placeholder="{yun:}t key='wap_js_00119'{/yun}" v-model="ruleFormAdd.moblie"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAdd = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="saveAdd">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
            <el-dialog title="{yun:}t key='admin_user_weipin_00029'{/yun}" :visible.sync="dialogDomain" append-to-body :show-close="true" width="500px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList" v-if="detail.id">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_00534'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <span>{{ detail.username }}</span>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_user_weipin_00020'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-select v-model="ruleFormDomain.did" filterable placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                <el-option v-for="(item, key) in domainList" :key="key" :label="item" :value="key">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogDomain = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="saveDomain">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
            <el-dialog title="{yun:}t key='admin_user_00292'{/yun}" :visible.sync="dialogAuth" append-to-body :show-close="true" width="500px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_00535'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-checkbox-group v-model="ruleFormAuth.type">
                                <el-checkbox label="email">{yun:}t key='member_user_00282'{/yun}</el-checkbox>
                                <el-checkbox label="moblie">{yun:}t key='member_user_00163'{/yun}</el-checkbox>
                                <el-checkbox label="idcard">{yun:}t key='member_com_00014'{/yun}</el-checkbox>
                            </el-checkbox-group>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_00536'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-radio v-model="ruleFormAuth.status" label="0">{yun:}t key='admin_user_00300'{/yun}</el-radio>
                            <el-radio v-model="ruleFormAuth.status" label="1">{yun:}t key='wap_user_00128'{/yun}</el-radio>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAuth = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="authSubmit">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--账户合并弹窗-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_user_00280'{/yun}" :visible.sync="dialogAccountMerge" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div style="overflow: hidden; position: relative; width: 100%;">
                    <div class="wxsettip_small ">{yun:}t key='default_00330'{/yun}</div>
                    <div class="">{{ lc("admin_name_value", [detail.username_n]) }} {{ lc("admin_account_value", [detail.username]) }}</div>
                    <div class="wxsettip_small ">{yun:}t key='wap_com_00157'{/yun}</div>
                    <!--<el-input v-model="ruleFormAccountMerge.com_uid" placeholder="请输入企业名称"></el-input>-->
                    <el-autocomplete style="width: 100%;" v-model="AccountMergeComname" :fetch-suggestions="querySearchCom"
                        value-key="name" placeholder="{yun:}t key='wap_user_00149'{/yun}" @select="handleSelectCom"></el-autocomplete>
                    <el-divider content-position="left">{yun:}t key='admin_user_00270'{/yun}</el-divider>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00241'{/yun}</div>
                    <el-radio v-model="ruleFormAccountMerge.mobile" :label="1">{yun:}t key='common.company'{/yun}</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.mobile" :label="2">{yun:}t key='admin_user_00304'{/yun}</el-radio>
                    <div class="wxsettip_small ">{yun:}t key='wap_com_00016'{/yun}</div>
                    <el-radio v-model="ruleFormAccountMerge.email" :label="1">{yun:}t key='common.company'{/yun}</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.email" :label="2">{yun:}t key='admin_user_00304'{/yun}</el-radio>
                    <div class="wxsettip_small ">{yun:}t key='admin_00537'{/yun}</div>
                    <el-radio v-model="ruleFormAccountMerge.QQ" :label="1">{yun:}t key='common.company'{/yun}</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.QQ" :label="2">{yun:}t key='admin_user_00304'{/yun}</el-radio>
                    <div class="wxsettip_small ">{yun:}t key='member_user_00056'{/yun}</div>
                    <el-radio v-model="ruleFormAccountMerge.wx" :label="1">{yun:}t key='common.company'{/yun}</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.wx" :label="2">{yun:}t key='admin_user_00304'{/yun}</el-radio>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00297'{/yun}</div>
                    <el-radio v-model="ruleFormAccountMerge.sina" :label="1">{yun:}t key='common.company'{/yun}</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.sina" :label="2">{yun:}t key='admin_user_00304'{/yun}</el-radio>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAccountMerge = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitAccountMerge">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--删除弹窗-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_00549'{/yun}" :visible.sync="scdrawer" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                {yun:}t key='admin_00538'{/yun}
                <span slot="footer" class="dialog-footer">
                    <el-button @click="scdrawer = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="scdrawer = false">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--预览简历弹窗-->
        <!--账户信息弹窗-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_user_00191'{/yun}" :visible.sync="dialogAccount" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00140'{/yun}</div>
                    <el-input placeholder="{yun:}t key='wap_00208'{/yun}" v-model="ruleFormAccount.username"></el-input>
                    <div class="wxsettip_small ">{yun:}t key='wap_00702'{/yun}</div>
                    <el-input @mousedown.native="pwdMousedown" @input="pwdchange" @focus="readonlyCtl(false)" @blur="readonlyCtl(true)" :readonly="pwdreadonly" placeholder="{yun:}t key='wap_00703'{/yun}" v-model="ruleFormAccount.password" ></el-input>
                    <div class="wxsettip_small ">{yun:}t key='member_user_00181'{/yun}</div>
                    <el-radio-group v-model="ruleFormAccount.status">
                        <el-radio label="1">{yun:}t key='admin_user_00149'{/yun}</el-radio>
                        <el-radio label="2">{yun:}t key='admin_user_00150'{/yun}</el-radio>
                    </el-radio-group>
                    <template v-if="ruleFormAccount.status == 2">
                        <div class="wxsettip_small ">{yun:}t key='admin_00438'{/yun}</div>
                        <el-input type="textarea" :rows="2" v-model="ruleFormAccount.lock_info">
                        </el-input>
                    </template>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAccount = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitAccount" :loading="saveLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--个人详情弹窗-->
        <el-drawer title="{yun:}t key='admin_user_00291'{/yun}" :visible.sync="drawerDetail" @closed="closedDetail" :modal-append-to-body="false" size="95%"
            :append-to-body="true">
            <div class="shbox">
                <div class="shinfo">
                    <div class="sh_zwsz">
                        <el-button type="primary" size="mini" @click="toMember(detail)"><i class="el-icon-school"></i>
                            {yun:}t key='admin_00539'{/yun}</el-button>
                    </div>
                    <div class="shcomdj">
                        {{ lc("admin_name_value", [resume.name]) }}
                        <span class="shcomtel_n">{{ lc("admin_username_value", [member.username]) }}</span>
                        {{ lc("admin_mobile_value", [resume.telphone]) }}
                    </div>
                    <div class="shcomtel" style="padding-bottom:15px; padding-top:10px;;border:none;font-size: 13px;">
                        <span class=" ">{yun:}t key='admin_00540'{/yun} </span>
                        <span class="shcomtel_n" v-if="member.logion_date != ''">{yun:}t key='admin_00541'{/yun} </span>
                        <span class="shcomtel_n" v-else>{yun:}t key='admin_user_00288'{/yun} </span>
                        {{ lc("admin_login_count_value", [member.login_hits]) }}
                        <span class=" shcomtel_n"> IP：{{ member.login_ip }} </span>
                        <span class=" "></span>
                        <span class="shcomtel_n">{{ lc("admin_source_value", [source[member.source]]) }}</span>
                        <span class=" ">{{ lc("admin_site_value", [domainList[resume.did]]) }}</span>
                        <div class="cominfocz">
                            <el-button type="primary" @click="openAccount" size="mini">
                                <i class="el-icon-edit"></i>{yun:}t key='admin_user_00191'{/yun}
                            </el-button>
                            <el-button type="primary" @click="openAccountMerge" size="mini">
                                <i class="el-icon-document-add"></i>{yun:}t key='admin_00542'{/yun}
                            </el-button>
                            <el-button type="primary" @click="resetPassword(detail)" size="mini">
                                <i class="el-icon-thumb"></i>{yun:}t key='admin_user_00137'{/yun}
                            </el-button>
                            <el-button type="primary" size="mini" @click="openDomain(resume)">
                                <i class="el-icon-map-location"></i>{yun:}t key='admin_user_weipin_00029'{/yun}
                            </el-button>
                            <el-button type="primary" @click="openDel(index)" size="mini">
                                <i class="el-icon-close"></i>{yun:}t key='admin_00543'{/yun}
                            </el-button>
                        </div>
                    </div>
                    <!--个人详情详情切换-->
                    <el-tabs v-model="activeName" type="card" @tab-click="handleClick">
                        <el-tab-pane label="{yun:}t key='admin_00550'{/yun}" name="resume">
                            <div v-loading="expectLoading">
                                <div class="shshow_tit">
                                    <i class="el-icon-office-building"></i> {yun:}t key='wap_user_00341'{/yun}
                                    <span class="shshow_cz">
                                        <el-button type="text" @click="openBasic">
                                            <i class="el-icon-edit"></i>{yun:}t key='admin_user_00227'{/yun}
                                        </el-button>
                                    </span>
                                </div>
                                <div class="userinfo_box">
                                    <div class="userinfo_l"><img :src="resume.photo" width="70" height="70"> </div>
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
                                        <el-tag size="mini" v-for="(tagItem, tagIndex) in resume.arrayTag" :key="tagIndex">
                                            {{ tagItem }}
                                        </el-tag>
                                        <div class="cominfo">{{ resume.description }}</div>
                                    </div>
                                    <div class="user_resume_add">
                                        <div class="">{yun:}t key='admin_user_00196'{/yun}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openTag">
                                                <i class="el-icon-circle-plus-outline"></i> {{ (resume.arrayTag &&
                                                    resume.arrayTag.length > 0) || resume.description ? '{yun:}t key='common.edit'{/yun}' : '{yun:}t key='wap_js_00091'{/yun}' }}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--求职意向-->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"><i class="el-icon-notebook-2"></i> {yun:}t key='wap_00460'{/yun} </div>
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
                                        <div class="">{yun:}t key='admin_user_00205'{/yun}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openJob">
                                                <i class="el-icon-circle-plus-outline"></i> {yun:}t key='admin_00472'{/yun}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--工作经历-->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"> <i class="el-icon-suitcase-1"></i> {yun:}t key='wap_00457'{/yun} </div>
                                    <!--循环-->
                                    <div class="user_resume_show" v-for="(work, workkey) in expectData.work">
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
                                    <div class="user_resume_add">
                                        <div class="">{yun:}t key='admin_user_00195'{/yun}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openWork('')">
                                                <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--教育经历-->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"> <i class="el-icon-school"></i> {yun:}t key='wap_00459'{/yun} </div>
                                    <!--循环-->
                                    <div class="user_resume_show" v-for="(edu, edukey) in expectData.edu">
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
                                                    v-if="edu.specialty && edu.education_n">|</span>{{ edu.education_n }}
                                            </div>
                                            <div class="user_resume_time">{{ edu.sdate_n }}-{{ edu.edate_n }}</div>
                                        </div>
                                    </div>
                                    <!--循环-->
                                    <div class="user_resume_add">
                                        <div class="">{yun:}t key='admin_user_00202'{/yun} </div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openEdu('')">
                                                <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--培训经历-->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"> <i class="el-icon-data-analysis"></i> {yun:}t key='wap_00455'{/yun} </div>
                                    <!--循环-->
                                    <div class="user_resume_show" v-for="(training, trainingKey) in expectData.training">
                                        <div class="user_resume_addname ">{{ training.name }}
                                            <el-button type="text" @click="openTraining(trainingKey)">
                                                <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                                            </el-button>
                                            <el-button type="text"
                                                @click="delResumeFb('training', trainingKey, training.id)">
                                                <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
                                            </el-button>
                                        </div>
                                        <div class="user_resume_addjy">
                                            <div class=" ">{{ training.title }} </div>
                                            <div class="user_resume_time">{{ training.sdate_n }}-{{ training.edate_n }}
                                            </div>
                                        </div>
                                        <div class="user_resume_ms">{{ training.content }}</div>
                                    </div>
                                    <!--循环-->
                                    <div class="user_resume_add">
                                        <div class="">{yun:}t key='admin_user_00197'{/yun}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openTraining('')">
                                                <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--职业技能-->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"><i class="el-icon-reading"></i> {yun:}t key='wap_00461'{/yun}</div>
                                    <!--循环-->
                                    <div class="user_resume_show" v-for="(skill, skillkey) in expectData.skill">
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
                                    <div class="user_resume_add">
                                        <div class="">{yun:}t key='admin_user_00199'{/yun}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openSkill('')">
                                                <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--项目经历-->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"><i class="el-icon-wallet"></i> {yun:}t key='wap_00465'{/yun} </div>
                                    <!--循环-->
                                    <div class="user_resume_show" v-for="(project, projectkey) in expectData.project">
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
                                    <div class="user_resume_add">
                                        <div class="">{yun:}t key='admin_user_00194'{/yun}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openProject('')">
                                                <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--其他描述-->
                                <div class="user_resume_list" style="padding-bottom:80px; ;">
                                    <div class="shshow_tit"> <i class="el-icon-mic"></i> {yun:}t key='admin_00068'{/yun} </div>
                                    <!--循环-->
                                    <div class="user_resume_show" v-for="(other, otherkey) in expectData.other">
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
                                    <div class="user_resume_add">
                                        <div class="">{yun:}t key='admin_user_00215'{/yun}</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openOther('')">
                                                <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="{yun:}t key='admin_00551'{/yun}" name="sqlog">
                            <div class="moduleElHight">
                                <div class="moduleElTable"
                                    style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 45px);">
                                    <el-table :data="jobSqLog.list" style="width: 100%" height="100%" ref="table2" stripe
                                        :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                                        v-loading="loading">
                                        <template slot="empty">
                                            <p>{{ dataText }}</p>
                                        </template>
                                        <el-table-column prop="job_name" label="{yun:}t key='wap_01596'{/yun}">
                                            <template slot-scope="scope">
                                                <div class="moduleProps">
                                                    <el-link type="primary" :underline="false"
                                                        @click="openPage(scope.row.job_comapply)">{{ scope.row.job_name
                                                        }}</el-link>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="com_name" label="{yun:}t key='admin_user_00247'{/yun}">
                                            <template slot-scope="scope">
                                                <div class="moduleProps">
                                                    <el-link type="primary" :underline="false"
                                                        @click="openPage(scope.row.company_show)">{{ scope.row.com_name
                                                        }}</el-link>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="datetime_n_n" label="{yun:}t key='member_user_00431'{/yun}"></el-table-column>
                                        <el-table-column label="{yun:}t key='admin_user_00250'{/yun}">
                                            <template slot-scope="scope">
                                                <div class="admin_state">
                                                    <span class="admin_state1" v-if="scope.row.is_browse == 2">{yun:}t key='wap_user_00258'{/yun}</span>
                                                    <span class="admin_state2"
                                                        v-else-if="scope.row.is_browse == 3">{yun:}t key='admin_user_00252'{/yun}</span>
                                                    <span class="admin_state3"
                                                        v-else-if="scope.row.is_browse == 4">{yun:}t key='wap_user_00354'{/yun}</span>
                                                    <span class="admin_state4"
                                                        v-else-if="scope.row.is_browse == 5">{yun:}t key='member_com_00108'{/yun}</span>
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
                                            :hide-on-single-page="true" @current-change="handleCurrentChangeJobSqlLog"
                                            :current-page="jobSqLog.page" :page-sizes="jobSqLog.pageSizes"
                                            :page-size="jobSqLog.limit" layout="total, sizes, prev, pager, next, jumper"
                                            :total="jobSqLog.total">
                                        </el-pagination>
                                    </div>
                                </div>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="{yun:}t key='wap_com_00046'{/yun}" name="yqms">
                            <div class="moduleElHight">
                                <div class="moduleElTable"
                                    style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 55px);">
                                    <el-table :data="yqmsLog.list" style="width: 100%" ref="table3" stripe
                                        :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%">
                                        <template slot="empty">
                                            <p>{{ dataText }}</p>
                                        </template>
                                        <el-table-column prop="fname" label="{yun:}t key='wap_01403'{/yun}" min-width="200">
                                            <template slot-scope="scope">
                                                <div class="moduleProps">
                                                    <el-link type="primary" :underline="false"
                                                        @click="openPage(scope.row.company_show)">{{ scope.row.fname
                                                        }}</el-link>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="jobname" label="{yun:}t key='admin_00552'{/yun}" min-width="200">
                                            <template slot-scope="scope">
                                                <div class="moduleProps">
                                                    <el-link type="primary" :underline="false"
                                                        @click="openPage(scope.row.job_comapply)">{{ scope.row.jobname
                                                        }}</el-link>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="title" label="{yun:}t key='admin_00553'{/yun}" width="150"></el-table-column>
                                        <el-table-column prop="content" label="{yun:}t key='admin_00554'{/yun}" min-width="170"></el-table-column>
                                        <el-table-column prop="datetime_n" label="{yun:}t key='member_user_00170'{/yun}" width="170"></el-table-column>
                                        <el-table-column label="{yun:}t key='admin_user_00250'{/yun}" width="150">
                                            <template slot-scope="scope">
                                                <div class="admin_state">
                                                    <span class="admin_state1" v-if="scope.row.is_browse == 2">{yun:}t key='wap_user_00258'{/yun}</span>
                                                    <span class="admin_state2"
                                                        v-else-if="scope.row.is_browse == 3">{yun:}t key='wap_com_00190'{/yun}</span>
                                                    <span class="admin_state3"
                                                        v-else-if="scope.row.is_browse == 4">{yun:}t key='wap_user_00257'{/yun}</span>
                                                    <span class="admin_state5" v-else>{yun:}t key='wap_user_00260'{/yun}</span>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="isdel_n" label="{yun:}t key='member_user_00181'{/yun}" width="100"></el-table-column>
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
                        
                        <el-tab-pane label="{yun:}t key='admin_00555'{/yun}" name="log">
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
                                            <!--	<div class="dt_name">浏览职位</div>-->
                                            <!--	<div class="dt_mx">黄灿灿 浏览了 红河州勤行设计装饰工程有限公司 的 行政资料员</div>-->
                                            <!--</li>-->
                                            <!--<li>-->
                                            <!--	<div class="dt_time">10:39</div>-->
                                            <!--	<div class="dt_name">访问行为</div>-->
                                            <!--	<div class="dt_mx">黄灿灿 访问了求职小助手</div>-->
                                            <!--</li>-->
                                        </ul>
                                    </div>
                                </template>
                                <div style="height: 100px">
                                    <div v-if="userLog.page == userLog.last_page">{yun:}t key='admin_user_00283'{/yun}</div>
                                    <h3 v-else @click="handleCurrentChangeUserLog">{yun:}t key='admin_user_00276'{/yun}</h3>
                                </div>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="{yun:}t key='admin_00556'{/yun}" name="pay">
                            <!--<div class="admin_datatip">-->
                            <!--	<i class="el-icon-document"></i> {{ lc("admin_data_stats") }}目前拥有积分 3526-->
                            <!--	<span class="admin_datatip_n">共消费积分：13625 </span>-->
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
                                        <el-table-column prop="order_id" label="{yun:}t key='admin_user_00295'{/yun}"></el-table-column>
                                        <el-table-column prop="consume_price_n" label="{yun:}t key='member_user_00254'{/yun}"></el-table-column>
                                        <el-table-column prop="consume_remark" label="{yun:}t key='admin_user_00290'{/yun}"></el-table-column>
                                        <el-table-column prop="pay_time_n" label="{yun:}t key='wap_com_00344'{/yun}"></el-table-column>
                                        <el-table-column prop="consume_state_n" label="{yun:}t key='member_user_00181'{/yun}"></el-table-column>
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
            <!---编辑简历 基本资料-->
            <el-drawer title="{yun:}t key='admin_00475'{/yun}" :append-to-body="true" :visible.sync="drawerBasic" :wrapper-closable="false"
                size="60%">
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
                                            <el-radio v-for="(sex, sexkey) in user_sex" :label="sexkey" :key="sexkey">
                                                {{ sex }}
                                            </el-radio>
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
                                            @input="inputFloatNumber($event, 'ruleFormBasic', 'height')"><template
                                                slot="append">CM</template></el-input>
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
                                            @input="inputFloatNumber($event, 'ruleFormBasic', 'weight')"><template
                                                slot="append">KG</template></el-input>
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
                                        <job_class multiple :max="5" @confirm="confirmJob" :selected="jobSelected">
                                        </job_class>
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
                                        <city_class multiple :max="5" @confirm="confirmCity" :selected="citySelected">
                                        </city_class>
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <div class="TableTite">{yun:}t key='wap_user_00016'{/yun}</div>
                                </td>
                                <td>
                                    <div class="TableInpt" style="max-width: 700px;">
                                        <el-select v-model="ruleFormJob.minsalary" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="salaryChange"
                                            style="margin-right:8px;">
                                            <el-option v-for="maxsalary1Val in minsalaryList" :key="maxsalary1Val"
                                                :label="maxsalary1Val" :value="maxsalary1Val">
                                            </el-option>
                                        </el-select>
                                        <el-select v-model="ruleFormJob.maxsalary" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                            <el-option v-for="maxsalary2Val in maxsalaryList" :key="maxsalary2Val"
                                                :label="maxsalary2Val" :value="maxsalary2Val">
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
        </el-drawer>
        <!---编辑个人优势-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_user_00326'{/yun}" :visible.sync="dialogTag" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00219'{/yun}</div>
                    <div class="">
                        <el-tag :key="tagkey" v-for="(tag, tagkey) in userTag" :disable-transitions="false"
                            @click="checkTag(tag)" :effect="ruleFormTag.tag.indexOf(tag) > -1 ? 'dark' : 'light'">
                            {{ tag }}
                        </el-tag>
                        <el-input style="margin-bottom: 10px;" class="input-new-tag" v-if="inputTag" v-model="tagval"
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
                    <div class="">
                        <el-input v-model="ruleFormWork.name" placeholder="{yun:}t key='wap_00137'{/yun}"></el-input>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00091'{/yun}</div>
                    <div class="">
                        <el-input v-model="ruleFormWork.title" placeholder="{yun:}t key='wap_user_00045'{/yun}"></el-input>
                    </div>
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
                    <el-input type="textarea"
                        :placeholder="{yun:}t key='admin_vue_00012'{/yun}"
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
                    <div class="">
                        <el-input v-model="ruleFormEdu.name" placeholder="{yun:}t key='wap_user_00044'{/yun}"></el-input>
                    </div>
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
                    <div class="">
                        <el-input v-model="ruleFormEdu.specialty" placeholder="{yun:}t key='wap_user_00042'{/yun}"></el-input>
                    </div>
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
                    <div class="">
                        <el-input v-model="ruleFormTraining.name" placeholder="{yun:}t key='admin_00485'{/yun}"></el-input>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00083'{/yun}</div>
                    <div class="">
                        <el-input v-model="ruleFormTraining.title" placeholder="{yun:}t key='admin_user_00209'{/yun}"></el-input>
                    </div>
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
                    <div class="">
                        <el-input v-model="ruleFormProject.name" placeholder="{yun:}t key='wap_user_00046'{/yun}"></el-input>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00225'{/yun}</div>
                    <div class="">
                        <el-input v-model="ruleFormProject.title" placeholder="{yun:}t key='admin_00486'{/yun}"></el-input>
                    </div>
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
                    <div class="">
                        <el-input v-model="ruleFormOther.name" placeholder="{yun:}t key='admin_00487'{/yun}"></el-input>
                    </div>
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
                    <div class="">
                        <el-input v-model="ruleFormSkill.name" placeholder="{yun:}t key='admin_user_00210'{/yun}"></el-input>
                    </div>
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
                        <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept" action=""
                            :auto-upload="false" :on-change="handleChangeSkillPic" :show-file-list="false">
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
        <!--新增简历-->
        <div class="modluDrawer">
            <el-drawer title="{yun:}t key='admin_user_00193'{/yun}" :visible.sync="drawerResume" append-to-body :wrapper-closable="false" size="45%">
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
            dataText: "{yun:}t key='admin_user_weipin_00026'{/yun}",
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
            dynamicTags: ["{yun:}t key='admin_user_00289'{/yun}", "{yun:}t key='admin_user_00293'{/yun}", "{yun:}t key='admin_user_00278'{/yun}", "{yun:}t key='admin_user_00302'{/yun}", "{yun:}t key='admin_user_00282'{/yun}", "{yun:}t key='admin_user_00281'{/yun}", "{yun:}t key='admin_user_00284'{/yun}", "{yun:}t key='admin_user_00299'{/yun}", "{yun:}t key='admin_user_00298'{/yun}"],
            inputVisible: false,
            inputValue: '',
            tableData: [],
            items: [{
                type: '',
                label: "{yun:}t key='admin_user_00149'{/yun}"
            },],
            // 来源
            source: {},

            // 搜索筛选项
            searchList: [],
            searchForm: {
                type: 1,
                time_type: 'lotime',
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
            searchParams: {},

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

            userStatusNum3: 0,
            userAllNum: 0,

            saveLoading: false,

            // 身份证认证
            dialogIdcardRz: false,
            ruleFormIdcardRz: {},
            // mobileverification
            dialogMoblieRz: false,
            ruleFormMobileRz: {},
            // 邮件认证
            dialogEmailRz: false,
            ruleFormEmailRz: {},

            // {yun:}t key='admin_user_00292'{/yun}
            dialogAuth: false,
            ruleFormAuth: {},

            // 分站切换
            dialogDomain: false,
            ruleFormDomain: {},
            domainList: {},

            // {yun:}t key='wap_com_00427'{/yun}
            drawerDetail: false,
            member: {},
            resume: {},
            expectData: {},

            // {yun:}t key='common_02022'{/yun}
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

            // {yun:}t key='admin_user_00191'{/yun}
            dialogAccount: false,
            ruleFormAccount: {},
            // {yun:}t key='admin_00542'{/yun}
            dialogAccountMerge: false,
            AccountMergeComname: "",
            ruleFormAccountMerge: {},

            // Delete
            dialogDel: false,
            ruleFormDel: {},

            expectLoading: true,

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
            // 行为分析
            behavior: {
                reverseone: true,
                daterange: "',
                times: '',
                activeClass: '',
                fenxiDetail: {},
                dataCount: {},
                logList: [],
                pagenav: 0,
                pageCode: '',
                xialaStatus: true
            },
            // 个人动态
            userLog: {
                page: 1,
                limit: 0,
                list: null
            },
            // 积分管理
            payLog: {
                page: 1,
                limit: 0,
                total: 0
            },

            // 新增简历
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
                    this.searchClass = '";
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
        //用来阻止第二次或更多次点击密码输入框时下拉用户密码清单的框一闪而过的问题
        pwdMousedown(){
            var that = this
            this.pwdreadonly = true
            setTimeout(function(){ that.pwdreadonly = false, 100})
        },
        // {yun:}t key='common_00444'{/yun}
        pwdchange: function(val){
            var that = this
            if (val == "") {
                this.pwdreadonly = true
                setTimeout(function(){ that.pwdreadonly = false, 100})
            }
        },
        // 修改密码框readonly{yun:}t key='wap_js_00085'{/yun}，防止密码框展示浏览器记录的密码信息
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
            // 这里面需要注意，{yun:}t key='admin_user_company_00161'{/yun}ref需要那个那个包含table元素的父元素
            let divData = this.$refs.multipleTable.bodyWrapper;
            if (this.mouseFlag) {
                // 设置水平方向的元素的位置
                divData.scrollLeft -= (- this.mouseOffset + (this.mouseOffset = e.clientX));
            }
        },
        // 跳转会员中心前检测
        memberCheck: function (uid, usertype) {
            var that = this
            var tip = "'
            if (usertype != '1') {
                if (usertype == '0') {
                    tip = "{yun:}t key='admin_user_00267'{/yun}"
                } else {
                    if (usertype == '2') {
                        var u = "{yun:}t key='admin_user_00301'{/yun}";
                    }
                    tip = "{yun:}t key='admin_user_00275'{/yun}" + u + "{yun:}t key='admin_user_00268'{/yun}"
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
        // 跳转到会员中心
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
                    that.limit = parseInt(data.limit); // 取系统配置默认数量
                }
                if (that.page > data.page) {
                    that.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
                }
                if (that.prevPage != that.page) {
                    that.prevPage = that.page;
                    that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    scrollToTop()
                }
                that.loading = false;
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
            } else if (this.multipleSelection.length == 0) {
                message.error("{yun:}t key='admin_user_weipin_00001'{/yun}");
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
            if (typeof idx == 'undefined") { // {yun:}t key='member_com_00055'{/yun}
                this.ruleFormDel = {
                    del: this.idArr,
                    delAccount: 0
                }
            } else { // {yun:}t key='common_01711'{/yun}
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

            httpPost("m=user&c=users_member&a=del', ruleForm).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogDel = false;
                    that.refreshList = false; // 删除时关闭详情弹窗，不触发关闭事件的刷新
                    that.drawerDetail = false;
                    that.getList();
                    that.$refs.multipleTable.clearSelection();
                    message.success(res.msg)
                }
            })
        },

        // 身份证认证
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
                message.error("{yun:}t key='admin_user_weipin_00015'{/yun}");
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
                message.error("{yun:}t key='wap_js_00119'{/yun}");
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
        // 邮件认证
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

        // 查询手机归属地
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
                    message.success("{yun:}t key='admin_user_00294'{/yun}");
                }
            })
        },
        // 查询IP归属地
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
                    message.success("{yun:}t key='admin_user_00294'{/yun}");
                }
            })
        },

        openDomain(row) {
            if (typeof row == 'undefined") { // {yun:}t key='admin_yunying_00106'{/yun}
                this.detail = {};
                this.$set(this.ruleFormDomain, "uid', this.idArr);
                this.$set(this.ruleFormDomain, 'did', '');
            } else { // 单个操作
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
                message.error("{yun:}t key='admin_user_weipin_00002'{/yun}");
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
                    if (typeof ruleForm.uid == 'object") { // {yun:}t key='member_com_00055'{/yun}
                        that.getList();
                    } else { // {yun:}t key='common_01711'{/yun}
                        that.refreshList = true;
                        // 重新拉取详情
                        that.getDetail(ruleForm.uid);
                    }
                    message.success(res.msg)
                }
            })
        },
        // {yun:}t key='admin_user_00292'{/yun}
        openAuth() {
            this.dialogAuth = true;
            this.ruleFormAuth = {
                batchfirm: true,
                uid: this.idArr,
                type: [],
                status: "'
            };
        },
        authSubmit() {
            let that = this,
                ruleForm = this.ruleFormAuth;

            if (typeof ruleForm.type == 'undefined' || ruleForm.type.length == 0) {
                message.error("{yun:}t key='admin_01288'{/yun}");
                return false;
            }

            if (typeof ruleForm.status == 'undefined' || ruleForm.status === '') {
                message.error("{yun:}t key='admin_01289'{/yun}");
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_member&a=usercert", ruleForm).then(function (response) {
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

        // {yun:}t key='admin_00551'{/yun}
        openSqLog(index, row) {
            this.activeName = "sqlog';
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

        // 打开详情
        openDetail(index, row) {
            this.expectLoading = true;
            this.index = index;
            this.detail = row;
            this.getDetail();

            // 存在默认标签，加载默认标签数据
            if (this.activeName == 'sqlog') {
                this.getJobSqLog();
            }

            this.drawerDetail = true;
        },
        // 关闭详情
        closedDetail() {
            if (this.refreshList) {
                this.getList();
            }
            this.resetDetail();
        },
        // 重置详情加载的数据
        resetDetail() {
            this.activeName = 'resume";
            // {yun:}t key='admin_00551'{/yun}
            this.$set(this.$data, "jobSqLog', {
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
            // 行为分析
            this.behavior = {
                reverseone: true,
                daterange: '',
                times: '',
                activeClass: '',
                fenxiDetail: {},
                dataCount: {},
                logList: [],
                pagenav: 0,
                pageCode: '",
                xialaStatus: true
            };
            // {yun:}t key='admin_00555'{/yun}
            this.userLog = {
                page: 1,
                limit: 0,
                list: null
            };
            // {yun:}t key='admin_00556'{/yun}
            this.$set(this.$data, "payLog', {
                page: 1,
                limit: 0,
                total: 0
            });
        },
        // 获取详情
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
                message.error("{yun:}t key='wap_00208'{/yun}");
                return false;
            }
            if (typeof ruleForm.password === 'undefined' || $.trim(ruleForm.password) == "") {
                message.error("{yun:}t key='wap_00703'{/yun}");
                return false;
            }
            if (typeof ruleForm.moblie === 'undefined' || $.trim(ruleForm.moblie) == "") {
                message.error("{yun:}t key='wap_js_00119'{/yun}");
                return false;
            } else if (!isjsMobile(ruleForm.moblie)) {
                message.error("{yun:}t key='wap_js_00117'{/yun}");
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
                    }, "{yun:}t key='admin_user_00267'{/yun}")
                } else {
                    var usertype = '';
                    if (row.usertype == '2') {
                        usertype = "{yun:}t key='admin_user_00301'{/yun}";
                    }

                    delConfirm(that, params, function (params) {
                        that.getMemberUrl(row.uid);
                    }, "{yun:}t key='admin_user_00275'{/yun}" + usertype + "{yun:}t key='admin_user_00268'{/yun}")
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
            } else if (tab.name == 'pay") {
                if (!this.payLog.list) {
                    this.getPayLog();
                }
            }
        },

        // {yun:}t key='admin_user_00191'{/yun}
        openAccount() {
            let member = this.member;
            this.ruleFormAccount = {
                uid: member.uid,
                username: member.username,
                password: "',
                status: member.status,
                lock_info: member.lock_info
            };
            this.dialogAccount = true;
        },
        submitAccount() {
            let that = this,
                ruleForm = that.ruleFormAccount;
            that.saveLoading = true;
            httpPost('m=user&c=users_member&a=saveUser", ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogAccount = false;
                    that.refreshList = true;
                    // 重新拉取详情
                    that.getDetail(ruleForm.uid);
                    message.success(res.msg);
                }
            }).finally(function () {
                setTimeout(function () {
                    that.saveLoading = false;
                }, 2000);
            });
        },

        // {yun:}t key='admin_00542'{/yun}
        openAccountMerge() {
            let member = this.member;
            this.AccountMergeComname = "';
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
                message.error("{yun:}t key='admin_user_00271'{/yun}");
                return false;
            }

            httpPost('m=user&c=users_member&a=merge', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {

                    // 重新拉取详情
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
                        message.alert("{yun:}t key='admin_user_00141'{/yun}" + row.username + " {yun:}t key='admin_user_00115'{/yun}");
                    }
                })
            }, "{yun:}t key='admin_user_00274'{/yun}")
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
                    that.saveLoading = false;
                    message.error(res.msg);
                } else {
                    that.drawerBasic = false;
                    that.refreshList = true;
                    // 重新拉取详情
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
                    message.error("{yun:}t key='wap_user_00060'{/yun}");
                    return false;
                }
                if (tag.length >= 5) {
                    message.error("{yun:}t key='admin_user_00206'{/yun}");
                    return false;
                }
                if (userTag.indexOf(tagval) > -1) {
                    message.error("{yun:}t key='wap_user_00074'{/yun}");
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
                    message.error("{yun:}t key='admin_user_00206'{/yun}");
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
                message.error("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.tag.length > 5) {
                message.error("{yun:}t key='admin_user_00206'{/yun}");
                return false;
            }
            if (ruleForm.description == '' || ruleForm.description == null) {
                message.error("{yun:}t key='admin_01319'{/yun}");
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=users_resume&a=saveTag", ruleForm).then(function (response) {
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
                message.error("{yun:}t key='admin_00484'{/yun}");
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
                message.error("{yun:}t key='member_user_00095'{/yun}");
                return false;
            }
            if (ruleForm.report == "") {
                message.error("{yun:}t key='wap_00980'{/yun}");
                return false;
            }
            if (ruleForm.type == "") {
                message.error("{yun:}t key='wap_js_00163'{/yun}");
                return false;
            }
            if (ruleForm.jobstatus == "") {
                message.error("{yun:}t key='wap_00934'{/yun}");
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
                    // 重新拉取详情
                    that.getDetail(ruleForm.uid);
                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
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
                message.error("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.name == "") {
                message.error("{yun:}t key='wap_00137'{/yun}");
                return false;
            }
            if (ruleForm.sdate == "") {
                message.error("{yun:}t key='admin_user_00213'{/yun}");
                return false
            }
            ruleForm.sdate = formatMonth(ruleForm.sdate);
            if (ruleForm.edate != '') {
                if (ruleForm.sdate >= ruleForm.edate) {
                    message.error("{yun:}t key='admin_user_00201'{/yun}");
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
                message.error("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.name == "") {
                message.error("{yun:}t key='wap_user_00044'{/yun}");
                return false;
            }
            if (daterangeEdu.length == 0) {
                message.error(lc('admin_vue_00016'));
                return false
            }
            if (ruleForm.education == "") {
                message.error("{yun:}t key='wap_user_00049'{/yun}");
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
                message.error("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.name == "") {
                message.error("{yun:}t key='admin_00485'{/yun}");
                return false;
            }
            if (daterangeTraining.length == 0) {
                message.error("{yun:}t key='admin_user_00212'{/yun}");
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
                message.error("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.name == "") {
                message.error("{yun:}t key='admin_user_00210'{/yun}");
                return false;
            }
            if (ruleForm.ing == "") {
                message.error("{yun:}t key='wap_user_00072'{/yun}");
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
                message.error("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.name == "") {
                message.error("{yun:}t key='wap_user_00046'{/yun}");
                return false;
            }
            if (daterangeProject.length == 0) {
                message.error("{yun:}t key='admin_user_00214'{/yun}");
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

                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
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
                message.error("{yun:}t key='admin_user_00207'{/yun}");
                return false;
            }
            if (ruleForm.name == "") {
                message.error("{yun:}t key='admin_00487'{/yun}");
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

                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
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

        // 投递记录
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
                    jobSqLog.limit = parseInt(data.limit); // 取系统配置默认数量
                }
                if (jobSqLog.page > data.page) {
                    jobSqLog.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
                }
                if (that.prevPage2 != jobSqLog.page) {
                    that.prevPage2 = jobSqLog.page;
                    that.$refs.table2.bodyWrapper.scrollTop = 0;
                }
                that.jobSqLog = jobSqLog;
                // that.$set(that.$data, 'jobSqLog', jobSqLog);
                that.loading = false;

                if (that.jobSqLog.list.length === 0) {
                    that.dataText = "{yun:}t key='wap_js_00113'{/yun}";
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
                    yqmsLog.limit = parseInt(data.limit); // 取系统配置默认数量
                }
                if (yqmsLog.page > data.page) {
                    yqmsLog.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
                }
                if (that.prevPage3 != yqmsLog.page) {
                    that.prevPage3 = yqmsLog.page;
                    that.$refs.table3.bodyWrapper.scrollTop = 0;
                }
                that.yqmsLog = yqmsLog;

                if (that.yqmsLog.list.length === 0) {
                    that.dataText = "{yun:}t key='wap_js_00113'{/yun}";
                }
            })
        },
        
        // 个人动态
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
                    if (typeof list[item.date_n] === 'undefined") {
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
                    userLog.limit = parseInt(data.limit); // 取系统配置默认数量
                }
                if (userLog.page > data.page) {
                    userLog.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
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
        // {yun:}t key='admin_00556'{/yun}
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
            httpPost("m=user&c=users_member&a=payLog', params).then(function (response) {
                let res = response.data,
                    data = res.data;

                payLog.list = data.list;
                payLog.total = parseInt(data.total);
                payLog.pageSizes = data.page_sizes;
                if (payLog.limit === 0) {
                    payLog.limit = parseInt(data.limit); // 取系统配置默认数量
                }
                if (payLog.page > data.page) {
                    payLog.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
                }
                if (that.prevPage4 != payLog.page) {
                    that.prevPage4 = payLog.page;
                    that.$refs.table4.bodyWrapper.scrollTop = 0;
                }
                that.payLog = payLog;
                that.loading = false;

                if (that.payLog.list.length === 0) {
                    that.dataText = "{yun:}t key='wap_js_00113'{/yun}";
                }
            })
        },

        // 新增简历
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

/* 上传样式开始 */
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

/* 上传样式结束 */

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