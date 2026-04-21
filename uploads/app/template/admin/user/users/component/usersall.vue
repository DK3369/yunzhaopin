<template>
    <div class="moduleElHight" :class="searchClass == 'drawer' ? 'pad_lr_20' : ''">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall" style="padding: 2px 0;">
                <el-input v-model="searchForm.keyword" @keyup.enter.native="search" placeholder="请输入你要搜索的关键字" size="small"
                    clearable>
                    <el-select v-model="searchForm.type" size="small" slot="prepend" placeholder="用户名" style="padding-left:20px;">
                        <el-option label="用户名" :value="1"></el-option>
                        <el-option label="姓名" :value="2"></el-option>
                        <el-option label="手机号" :value="3"></el-option>
                        <el-option label="EMAIL" :value="4"></el-option>
                        <el-option label="用户ID" :value="5"></el-option>
                        <el-option label="IP" :value="6"></el-option>
                    </el-select>
                </el-input>
            </div>
            <!--收起部分-->
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="searchForm.time_type" size="small" slot="prepend" placeholder="筛选日期" clearable @change="handleTimeChange">
                    <el-option label="注册时间" value="adtime"></el-option>
                    <el-option label="登录时间" value="lotime"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmalltwo" :class="{ 'searchbutnOnff': seachbutn }">
                <el-date-picker v-model="searchForm.times" type="daterange" align="right" unlink-panels range-separator="至" start-placeholder="开始日期" end-placeholder="结束日期" :picker-options="timeOptions" value-format="yyyy-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
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
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">查询</el-button>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" plain icon="el-icon-document-add" size="mini" @click="openAdd">新增个人</el-button>
            </div>
            <div class="tableSeachzk" :class="{ 'searchbutnKai': seachbutn }" style="margin-bottom: 11px;">
                <el-button type="info" class="zhankai" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>展开
                    <i class="el-icon-arrow-down el-icon--right"></i>
                </el-button>
                <el-button type="info" class="shouqi" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                    aria-disabled="false" size="mini" plain>合并
                    <i class="el-icon-arrow-up el-icon--right"></i>
                </el-button>
            </div>
        </div>
        <div class="admin_datatip">
            <i class="el-icon-document"></i> 数据统计：共 <span @click="init">{{ userAllNum }}</span> 条
            <span class="admin_datatip_n">已锁定：<span @click="statusSearch('2')">{{ userStatusNum3 ? userStatusNum3 :
                0 }}</span> 条</span>
            <span class="admin_datatip_n">已锁定：</span>
            <span @click="statusSearch('2')">{{ userStatusNum3 ? userStatusNum3 : 0 }}条</span>
            <span class="admin_datatip_n">搜索结果： {{ total }} 条</span>
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
                <el-table-column prop="uid" label="用户ID" width="100" sortable="custom"></el-table-column>
                <el-table-column label="姓名/用户名" min-width="110" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class="username">{{ scope.row.username_n }}</div>
                        </div>
                        <div class="yhm">
                            <el-link @click="memberCheck(scope.row.uid, scope.row.usertype)" :underline="false">{{
                                scope.row.username
                            }}
                            </el-link>
                            <el-tooltip v-if="scope.row.r_status == '2'" class="item" effect="dark" content="已锁定"
                                placement="top-start">
                                <i class="el-icon-lock" style="color: orange"></i>
                            </el-tooltip>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="手机号/归属地" min-width="130">
                    <template slot-scope="scope">
                        <div class="moduleProps" v-if="scope.row.telphone">
                            <span>{{ scope.row.telphone }}</span>
                            <span v-if="scope.row.moblie_address" class="gsd">
                                {{ scope.row.moblie_address }}
                            </span>
                            <el-link v-else type="primary" :underline="false"
                                @click="getMobileAddress(scope.$index)">查询归属地</el-link>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="个人认证" min-width="110" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <div class="rz_box">
                            <el-tooltip v-if="scope.row.idcard_status == 1" class="item" effect="dark" content="身份已认证"
                                placement="top-start">
                                <el-button type="text" @click="idcardRz(scope.row)">
                                    <i class="rzicon rzicon_zzyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" content="身份未认证" placement="top-start">
                                <el-button type="text" @click="idcardRz(scope.row)">
                                    <i class="rzicon rzicon_zzwrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-if="scope.row.moblie_status == 1" class="item" effect="dark" content="手机已认证"
                                placement="top-start">
                                <el-button type="text" @click="moblieRz(scope.row)">
                                    <i class="rzicon rzicon_sjyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" content="手机未认证" placement="top-start">
                                <el-button type="text" @click="moblieRz(scope.row)">
                                    <i class="rzicon rzicon_sjwrz"></i>
                                </el-button>
                            </el-tooltip>
                        </div>
                        <div class="rz_box">
                            <el-tooltip v-if="scope.row.email_status_n == 1" class="item" effect="dark" content="邮箱已认证"
                                placement="top-start">
                                <el-button type="text" @click="emailRz(scope.row)">
                                    <i class="rzicon rzicon_yxyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" content="邮箱未认证" placement="top-start">
                                <el-button type="text" @click="emailRz(scope.row)">
                                    <i class="rzicon rzicon_yxwrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-if="scope.row.wxid != '' || scope.row.wxopenid != ''"
                                class="item" effect="dark" placement="top-start">
                                <div slot="content" v-html="'微信已认证<br/>' + scope.row.wxBindmsg"></div>
                                <el-button type="text">
                                    <i class="rzicon rzicon_wxyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" placement="top-start">
                                <div slot="content" v-html="'微信未认证<br/>' + scope.row.wxBindmsg"></div>
                                <el-button type="text">
                                    <i class="rzicon rzicon_wxwrz"></i>
                                </el-button>
                            </el-tooltip>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="投递岗位" min-width="130" align="center">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class="username">{{ scope.row.sq_num > 0 ? scope.row.sq_num : 0 }}</div>
                            <el-link v-if="scope.row.sq_num > 0" type="primary" :underline="false"
                                @click="openSqLog(scope.$index, scope.row)">查看</el-link>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="login_date" label="注册/登录" min-width="150" sortable="custom">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span class="gsd">{{ scope.row.reg_date_n }}</span>
                            <span v-if="scope.row.login_date_n">{{ scope.row.login_date_n }}</span>
                            <span v-else>未登录</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="简历/来源" min-width="150">
                    <template slot-scope="scope">
                        <div class="moduleProps" v-if="scope.row.def_job != '0'">
                            <el-link type="primary" :underline="false"
                                @click="openDetail(scope.$index, scope.row)">预览简历</el-link>
                        </div>
                        <div class="moduleProps" v-else>
                            <el-link type="primary" :underline="false" @click="openResume(scope.row)">添加简历</el-link>
                        </div>
                        <span class="gsd">{{ source[scope.row.source] }}</span>
                    </template>
                </el-table-column>
                <el-table-column label=" IP/归属地" min-width="130">
                    <template slot-scope="scope">
                        <div class="moduleProps">

                            <div v-if="scope.row.login_ip">
                                <span>{{ scope.row.login_ip }}</span>
                                <span v-if="scope.row.login_address" class="gsd"> {{ scope.row.login_address }}</span>
                                <el-link v-else type="primary" :underline="false"
                                    @click="getIpAddress(scope.$index)">查询归属地</el-link>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="状态" fixed="right" width="60">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.r_status == '2'" class="admin_state3">已锁定</span>
                            <span v-else class="admin_state1">正常</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="操作" width="80" fixed="right" align="center">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="openDetail(scope.$index, scope.row)">详情</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging" style="height: initial; flex-wrap: wrap; padding-top: 10px;">
            <div  style="width:100%;">
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
                    @change="checkAll">全选</el-checkbox>
                <el-button @click="batch('del')" size="mini">批量删除</el-button>
                <el-button @click="batch('domain')" size="mini">批量选择分站</el-button>
                <el-button @click="batch('auth')" size="mini">批量认证</el-button>
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
            <el-dialog title="删除个人数据" :visible.sync="dialogDel" :with-header="true" append-to-body :show-close="true"
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
        <!--身份认证弹窗-->
        <div class="modluDrawer">
            <el-dialog title="身份认证" :visible.sync="dialogIdcardRz" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">姓名</div>
                    <el-input :value="detail.username_n" :disabled="true"></el-input>
                    <div class="wxsettip_small ">身份证号码</div>
                    <el-input :value="detail.idcard" :disabled="true"></el-input>
                    <div class="wxsettip_small ">身份证图片</div>
                    <div class="zzrz_img">
                        <div class="zzrz_imgpreview">
                            <el-image v-if="detail.idcard_pic" style="width: 80px; height: 80px" :src="detail.idcard_pic"
                                :preview-src-list="detail.idcard_pic">
                            </el-image>
                            <span v-else>暂无身份证图片</span>
                        </div>
                    </div>
                    <div class="wxsettip_small ">审核操作</div>
                    <el-radio v-model="ruleFormIdcardRz.r_status" label="0">待认证</el-radio>
                    <el-radio v-model="ruleFormIdcardRz.r_status" label="1">已认证</el-radio>
                    <div class="wxsettip_small ">审核说明</div>
                    <el-input type="textarea" :rows="2" placeholder="" v-model="ruleFormIdcardRz.statusbody"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogIdcardRz = false">取 消</el-button>
                    <el-button type="primary" @click="idcardRzSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--手机认证弹窗-->
        <div class="modluDrawer">
            <el-dialog title="手机认证" :visible.sync="dialogMoblieRz" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">手机号</div>
                    <el-input placeholder="" v-model="ruleFormMobileRz.moblie"></el-input>
                    <div class="wxsettip_small ">审核操作</div>
                    <el-radio v-model="ruleFormMobileRz.mstatus" label="1">已认证</el-radio>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogMoblieRz = false">取 消</el-button>
                    <el-button type="primary" @click="moblieRzSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--邮箱认证弹窗-->
        <div class="modluDrawer">
            <el-dialog title="邮箱认证" :visible.sync="dialogEmailRz" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">邮箱号</div>
                    <el-input placeholder="" v-model="ruleFormEmailRz.email"></el-input>
                    <div class="wxsettip_small ">审核操作</div>
                    <el-radio v-model="ruleFormEmailRz.estatus" label="1">已认证</el-radio>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogEmailRz = false">取 消</el-button>
                    <el-button type="primary" @click="emailRzSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--投递岗位弹窗-->
        <!--新增个人用户弹窗-->
        <div class="modluDrawer">
            <el-dialog title="新增个人用户" :visible.sync="dialogAdd" :append-to-body="true" width="450px">
                <div>
                    <div class="wxsettip_small ">用户名</div>
                    <el-input placeholder="请输入用户名" v-model="ruleFormAdd.username"></el-input>
                    <div class="wxsettip_small ">登录密码</div>
                    <el-input @mousedown.native="pwdMousedown" @input="pwdchange" @focus="readonlyCtl(false)" @blur="readonlyCtl(true)" :readonly="pwdreadonly" placeholder="请输入登录密码" v-model="ruleFormAdd.password" ></el-input>
                    <div class="wxsettip_small ">邮箱</div>
                    <el-input placeholder="请输入邮箱" v-model="ruleFormAdd.email"></el-input>
                    <div class="wxsettip_small ">手机号</div>
                    <el-input placeholder="请输入手机号" v-model="ruleFormAdd.moblie"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAdd = false">取 消</el-button>
                    <el-button type="primary" @click="saveAdd">确 定</el-button>
                </span>
            </el-dialog>
            <el-dialog title="分配站点" :visible.sync="dialogDomain" append-to-body :show-close="true" width="500px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList" v-if="detail.id">
                        <div class="toolClasTite">
                            <span>用户名：</span>
                        </div>
                        <div class="toolClasCont">
                            <span>{{ detail.username }}</span>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>切换站点：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-select v-model="ruleFormDomain.did" filterable placeholder="请选择">
                                <el-option v-for="(item, key) in domainList" :key="key" :label="item" :value="key">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogDomain = false">取 消</el-button>
                    <el-button type="primary" @click="saveDomain">确 定</el-button>
                </span>
            </el-dialog>
            <el-dialog title="批量认证" :visible.sync="dialogAuth" append-to-body :show-close="true" width="500px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>认证类型：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-checkbox-group v-model="ruleFormAuth.type">
                                <el-checkbox label="email">邮箱</el-checkbox>
                                <el-checkbox label="moblie">手机</el-checkbox>
                                <el-checkbox label="idcard">身份证</el-checkbox>
                            </el-checkbox-group>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>认证操作：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-radio v-model="ruleFormAuth.status" label="0">待认证</el-radio>
                            <el-radio v-model="ruleFormAuth.status" label="1">已认证</el-radio>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAuth = false">取 消</el-button>
                    <el-button type="primary" @click="authSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--账户合并弹窗-->
        <div class="modluDrawer">
            <el-dialog title="账户合并弹窗" :visible.sync="dialogAccountMerge" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div style="overflow: hidden; position: relative; width: 100%;">
                    <div class="wxsettip_small ">个人信息</div>
                    <div class="">姓名：{{ detail.username_n }} 账号：{{ detail.username }}</div>
                    <div class="wxsettip_small ">企业名称</div>
                    <!--<el-input v-model="ruleFormAccountMerge.com_uid" placeholder="请输入企业名称"></el-input>-->
                    <el-autocomplete style="width: 100%;" v-model="AccountMergeComname" :fetch-suggestions="querySearchCom"
                        value-key="name" placeholder="请输入企业名称" @select="handleSelectCom"></el-autocomplete>
                    <el-divider content-position="left">合并账号后，默认选择企业数据</el-divider>
                    <div class="wxsettip_small ">手机号码</div>
                    <el-radio v-model="ruleFormAccountMerge.mobile" :label="1">企业</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.mobile" :label="2">个人</el-radio>
                    <div class="wxsettip_small ">联系邮箱</div>
                    <el-radio v-model="ruleFormAccountMerge.email" :label="1">企业</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.email" :label="2">个人</el-radio>
                    <div class="wxsettip_small ">绑定Q Q</div>
                    <el-radio v-model="ruleFormAccountMerge.QQ" :label="1">企业</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.QQ" :label="2">个人</el-radio>
                    <div class="wxsettip_small ">绑定微信</div>
                    <el-radio v-model="ruleFormAccountMerge.wx" :label="1">企业</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.wx" :label="2">个人</el-radio>
                    <div class="wxsettip_small ">绑定微博</div>
                    <el-radio v-model="ruleFormAccountMerge.sina" :label="1">企业</el-radio>
                    <el-radio v-model="ruleFormAccountMerge.sina" :label="2">个人</el-radio>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAccountMerge = false">取 消</el-button>
                    <el-button type="primary" @click="submitAccountMerge">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--删除弹窗-->
        <div class="modluDrawer">
            <el-dialog title="删除个人用户" :visible.sync="scdrawer" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                确定要删除个人用户？
                <span slot="footer" class="dialog-footer">
                    <el-button @click="scdrawer = false">取 消</el-button>
                    <el-button type="primary" @click="scdrawer = false">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--预览简历弹窗-->
        <!--账户信息弹窗-->
        <div class="modluDrawer">
            <el-dialog title="账户信息" :visible.sync="dialogAccount" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">用户名</div>
                    <el-input placeholder="请输入用户名" v-model="ruleFormAccount.username"></el-input>
                    <div class="wxsettip_small ">登录密码</div>
                    <el-input @mousedown.native="pwdMousedown" @input="pwdchange" @focus="readonlyCtl(false)" @blur="readonlyCtl(true)" :readonly="pwdreadonly" placeholder="请输入登录密码" v-model="ruleFormAccount.password" ></el-input>
                    <div class="wxsettip_small ">状态</div>
                    <el-radio-group v-model="ruleFormAccount.status">
                        <el-radio label="1">正常</el-radio>
                        <el-radio label="2">锁定</el-radio>
                    </el-radio-group>
                    <template v-if="ruleFormAccount.status == 2">
                        <div class="wxsettip_small ">锁定说明</div>
                        <el-input type="textarea" :rows="2" v-model="ruleFormAccount.lock_info">
                        </el-input>
                    </template>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAccount = false">取 消</el-button>
                    <el-button type="primary" @click="submitAccount" :loading="saveLoading">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--个人详情弹窗-->
        <el-drawer title="平台信息" :visible.sync="drawerDetail" @closed="closedDetail" :modal-append-to-body="false" size="95%"
            :append-to-body="true">
            <div class="shbox">
                <div class="shinfo">
                    <div class="sh_zwsz">
                        <el-button type="primary" size="mini" @click="toMember(detail)"><i class="el-icon-school"></i>
                            进入个人中心</el-button>
                    </div>
                    <div class="shcomdj">
                        姓名：{{ resume.name }}
                        <span class="shcomtel_n">用户名：{{ member.username }}</span>
                        手机号：{{ resume.telphone }}
                    </div>
                    <div class="shcomtel" style="padding-bottom:15px; padding-top:10px;;border:none;font-size: 13px;">
                        <span class=" ">注册时间：{{ member.reg_date_n }} </span>
                        <span class="shcomtel_n" v-if="member.logion_date != ''">最近登录 ：{{ member.login_date_n }} </span>
                        <span class="shcomtel_n" v-else>从未登录 </span>
                        登录次数：{{ member.login_hits }}
                        <span class=" shcomtel_n"> IP：{{ member.login_ip }} </span>
                        <span class=" "></span>
                        <span class="shcomtel_n">来源：{{ source[member.source] }}</span>
                        <span class=" ">站点：{{ domainList[resume.did] }}</span>
                        <div class="cominfocz">
                            <el-button type="primary" @click="openAccount" size="mini">
                                <i class="el-icon-edit"></i>账户信息
                            </el-button>
                            <el-button type="primary" @click="openAccountMerge" size="mini">
                                <i class="el-icon-document-add"></i>账户合并
                            </el-button>
                            <el-button type="primary" @click="resetPassword(detail)" size="mini">
                                <i class="el-icon-thumb"></i>重置密码
                            </el-button>
                            <el-button type="primary" size="mini" @click="openDomain(resume)">
                                <i class="el-icon-map-location"></i>分配站点
                            </el-button>
                            <el-button type="primary" @click="openDel(index)" size="mini">
                                <i class="el-icon-close"></i>删除该用户
                            </el-button>
                        </div>
                    </div>
                    <!--个人详情详情切换-->
                    <el-tabs v-model="activeName" type="card" @tab-click="handleClick">
                        <el-tab-pane label="用户简历" name="resume">
                            <div v-loading="expectLoading">
                                <div class="shshow_tit">
                                    <i class="el-icon-office-building"></i> 基本资料
                                    <span class="shshow_cz">
                                        <el-button type="text" @click="openBasic">
                                            <i class="el-icon-edit"></i>编辑资料
                                        </el-button>
                                    </span>
                                </div>
                                <div class="userinfo_box">
                                    <div class="userinfo_l"><img :src="resume.photo" width="70" height="70"> </div>
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
                                        <el-tag size="mini" v-for="(tagItem, tagIndex) in resume.arrayTag" :key="tagIndex">
                                            {{ tagItem }}
                                        </el-tag>
                                        <div class="cominfo">{{ resume.description }}</div>
                                    </div>
                                    <div class="user_resume_add">
                                        <div class="">总结优势，突出亮点，个人优势将突出展示给HR</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openTag">
                                                <i class="el-icon-circle-plus-outline"></i> {{ (resume.arrayTag &&
                                                    resume.arrayTag.length > 0) || resume.description ? '编辑' : '添加' }}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--求职意向-->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"><i class="el-icon-notebook-2"></i> 求职意向 </div>
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
                                    <div class="user_resume_add">
                                        <div class="">建议完善求职偏好</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openJob">
                                                <i class="el-icon-circle-plus-outline"></i> {{ expectData.expect ? '编辑' :
                                                    '添加'
                                                }}
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--工作经历-->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"> <i class="el-icon-suitcase-1"></i> 工作经历 </div>
                                    <!--循环-->
                                    <div class="user_resume_show" v-for="(work, workkey) in expectData.work">
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
                                    <div class="user_resume_add">
                                        <div class="">展示工作经验、工作能力否符合岗位要求的重要依据</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openWork('')">
                                                <i class="el-icon-circle-plus-outline"></i> 添加
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--教育经历-->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"> <i class="el-icon-school"></i> 教育经历 </div>
                                    <!--循环-->
                                    <div class="user_resume_show" v-for="(edu, edukey) in expectData.edu">
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
                                                    v-if="edu.specialty && edu.education_n">|</span>{{ edu.education_n }}
                                            </div>
                                            <div class="user_resume_time">{{ edu.sdate_n }}-{{ edu.edate_n }}</div>
                                        </div>
                                    </div>
                                    <!--循环-->
                                    <div class="user_resume_add">
                                        <div class="">补足HR对学历背景的了解 </div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openEdu('')">
                                                <i class="el-icon-circle-plus-outline"></i> 添加
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--培训经历-->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"> <i class="el-icon-data-analysis"></i> 培训经历 </div>
                                    <!--循环-->
                                    <div class="user_resume_show" v-for="(training, trainingKey) in expectData.training">
                                        <div class="user_resume_addname ">{{ training.name }}
                                            <el-button type="text" @click="openTraining(trainingKey)">
                                                <i class="el-icon-edit"></i> 修改
                                            </el-button>
                                            <el-button type="text"
                                                @click="delResumeFb('training', trainingKey, training.id)">
                                                <i class="el-icon-delete"></i> 删除
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
                                        <div class="">展示培训经验否符合岗位要求的重要依据</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openTraining('')">
                                                <i class="el-icon-circle-plus-outline"></i> 添加
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--职业技能-->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"><i class="el-icon-reading"></i> 职业技能</div>
                                    <!--循环-->
                                    <div class="user_resume_show" v-for="(skill, skillkey) in expectData.skill">
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
                                    <div class="user_resume_add">
                                        <div class="">技能专长建议填写职业技能为简历加分</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openSkill('')">
                                                <i class="el-icon-circle-plus-outline"></i> 添加
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--项目经历-->
                                <div class="user_resume_list">
                                    <div class="shshow_tit"><i class="el-icon-wallet"></i> 项目经历 </div>
                                    <!--循环-->
                                    <div class="user_resume_show" v-for="(project, projectkey) in expectData.project">
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
                                    <div class="user_resume_add">
                                        <div class="">展示工作经验、能力，这也是HR判断是否符合岗位要求的重要依据。</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openProject('')">
                                                <i class="el-icon-circle-plus-outline"></i> 添加
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                                <!--其他描述-->
                                <div class="user_resume_list" style="padding-bottom:80px; ;">
                                    <div class="shshow_tit"> <i class="el-icon-mic"></i> 其他描述 </div>
                                    <!--循环-->
                                    <div class="user_resume_show" v-for="(other, otherkey) in expectData.other">
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
                                    <div class="user_resume_add">
                                        <div class="">其他加分补充</div>
                                        <div class="user_resume_addbth">
                                            <el-button type="text" @click="openOther('')">
                                                <i class="el-icon-circle-plus-outline"></i> 添加
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="投递记录" name="sqlog">
                            <div class="moduleElHight">
                                <div class="moduleElTable"
                                    style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 45px);">
                                    <el-table :data="jobSqLog.list" style="width: 100%" height="100%" ref="table2" stripe
                                        :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                                        v-loading="loading">
                                        <template slot="empty">
                                            <p>{{ dataText }}</p>
                                        </template>
                                        <el-table-column prop="job_name" label="投递职位">
                                            <template slot-scope="scope">
                                                <div class="moduleProps">
                                                    <el-link type="primary" :underline="false"
                                                        @click="openPage(scope.row.job_comapply)">{{ scope.row.job_name
                                                        }}</el-link>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="com_name" label="所属企业">
                                            <template slot-scope="scope">
                                                <div class="moduleProps">
                                                    <el-link type="primary" :underline="false"
                                                        @click="openPage(scope.row.company_show)">{{ scope.row.com_name
                                                        }}</el-link>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="datetime_n_n" label="投递时间"></el-table-column>
                                        <el-table-column label="是否查看">
                                            <template slot-scope="scope">
                                                <div class="admin_state">
                                                    <span class="admin_state1" v-if="scope.row.is_browse == 2">已查看</span>
                                                    <span class="admin_state2"
                                                        v-else-if="scope.row.is_browse == 3">待通知</span>
                                                    <span class="admin_state3"
                                                        v-else-if="scope.row.is_browse == 4">不合适</span>
                                                    <span class="admin_state4"
                                                        v-else-if="scope.row.is_browse == 5">未接通</span>
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
                                            :hide-on-single-page="true" @current-change="handleCurrentChangeJobSqlLog"
                                            :current-page="jobSqLog.page" :page-sizes="jobSqLog.pageSizes"
                                            :page-size="jobSqLog.limit" layout="total, sizes, prev, pager, next, jumper"
                                            :total="jobSqLog.total">
                                        </el-pagination>
                                    </div>
                                </div>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="面试邀请" name="yqms">
                            <div class="moduleElHight">
                                <div class="moduleElTable"
                                    style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 55px);">
                                    <el-table :data="yqmsLog.list" style="width: 100%" ref="table3" stripe
                                        :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%">
                                        <template slot="empty">
                                            <p>{{ dataText }}</p>
                                        </template>
                                        <el-table-column prop="fname" label="公司名称" min-width="200">
                                            <template slot-scope="scope">
                                                <div class="moduleProps">
                                                    <el-link type="primary" :underline="false"
                                                        @click="openPage(scope.row.company_show)">{{ scope.row.fname
                                                        }}</el-link>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="jobname" label="面试岗位" min-width="200">
                                            <template slot-scope="scope">
                                                <div class="moduleProps">
                                                    <el-link type="primary" :underline="false"
                                                        @click="openPage(scope.row.job_comapply)">{{ scope.row.jobname
                                                        }}</el-link>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="title" label="邀请标题" width="150"></el-table-column>
                                        <el-table-column prop="content" label="邀请内容" min-width="170"></el-table-column>
                                        <el-table-column prop="datetime_n" label="邀请时间" width="170"></el-table-column>
                                        <el-table-column label="是否查看" width="150">
                                            <template slot-scope="scope">
                                                <div class="admin_state">
                                                    <span class="admin_state1" v-if="scope.row.is_browse == 2">已查看</span>
                                                    <span class="admin_state2"
                                                        v-else-if="scope.row.is_browse == 3">已同意</span>
                                                    <span class="admin_state3"
                                                        v-else-if="scope.row.is_browse == 4">已拒绝</span>
                                                    <span class="admin_state5" v-else>未查看</span>
                                                </div>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="isdel_n" label="状态" width="100"></el-table-column>
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
                        
                        <el-tab-pane label="个人动态" name="log">
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
                                    <div v-if="userLog.page == userLog.last_page">没有更多了</div>
                                    <h3 v-else @click="handleCurrentChangeUserLog">加载更多...</h3>
                                </div>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="积分管理" name="pay">
                            <!--<div class="admin_datatip">-->
                            <!--	<i class="el-icon-document"></i> 数据统计：目前拥有积分 3526-->
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
                                        <el-table-column prop="order_id" label="消费单号"></el-table-column>
                                        <el-table-column prop="consume_price_n" label="金额"></el-table-column>
                                        <el-table-column prop="consume_remark" label="备注说明"></el-table-column>
                                        <el-table-column prop="pay_time_n" label="消费时间"></el-table-column>
                                        <el-table-column prop="consume_state_n" label="状态"></el-table-column>
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
            <el-drawer title="编辑基本资料" :append-to-body="true" :visible.sync="drawerBasic" :wrapper-closable="false"
                size="60%">
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
                                            <el-radio v-for="(sex, sexkey) in user_sex" :label="sexkey" :key="sexkey">
                                                {{ sex }}
                                            </el-radio>
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
                                            @input="inputFloatNumber($event, 'ruleFormBasic', 'height')"><template
                                                slot="append">CM</template></el-input>
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
                                            @input="inputFloatNumber($event, 'ruleFormBasic', 'weight')"><template
                                                slot="append">KG</template></el-input>
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
                                        <job_class multiple :max="5" @confirm="confirmJob" :selected="jobSelected">
                                        </job_class>
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
                                        <city_class multiple :max="5" @confirm="confirmCity" :selected="citySelected">
                                        </city_class>
                                    </div>
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <div class="TableTite">期望薪资</div>
                                </td>
                                <td>
                                    <div class="TableInpt" style="max-width: 700px;">
                                        <el-select v-model="ruleFormJob.minsalary" placeholder="请选择" @change="salaryChange"
                                            style="margin-right:8px;">
                                            <el-option v-for="maxsalary1Val in minsalaryList" :key="maxsalary1Val"
                                                :label="maxsalary1Val" :value="maxsalary1Val">
                                            </el-option>
                                        </el-select>
                                        <el-select v-model="ruleFormJob.maxsalary" placeholder="请选择">
                                            <el-option v-for="maxsalary2Val in maxsalaryList" :key="maxsalary2Val"
                                                :label="maxsalary2Val" :value="maxsalary2Val">
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
        </el-drawer>
        <!---编辑个人优势-->
        <div class="modluDrawer">
            <el-dialog title="个人优势" :visible.sync="dialogTag" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">优势标签</div>
                    <div class="">
                        <el-tag :key="tagkey" v-for="(tag, tagkey) in userTag" :disable-transitions="false"
                            @click="checkTag(tag)" :effect="ruleFormTag.tag.indexOf(tag) > -1 ? 'dark' : 'light'">
                            {{ tag }}
                        </el-tag>
                        <el-input style="margin-bottom: 10px;" class="input-new-tag" v-if="inputTag" v-model="tagval"
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
                    <div class="">
                        <el-input v-model="ruleFormWork.name" placeholder="请输入公司名称"></el-input>
                    </div>
                    <div class="wxsettip_small ">担任职位</div>
                    <div class="">
                        <el-input v-model="ruleFormWork.title" placeholder="请输入担任职位"></el-input>
                    </div>
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
                    <el-input type="textarea"
                        :placeholder="'请简短介绍公司与自己负责的任务，分条罗列在什么项目中，通过某些动作或技能达到可量化的结果。\n示例：\n1、主要负责新员工入职培训；\n2、分析制定员工每月个人销售业绩；\n3、帮助员工提高每日客单价，整体店面管理工作等；'"
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
                    <div class="">
                        <el-input v-model="ruleFormEdu.name" placeholder="请输入学校名称"></el-input>
                    </div>
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
                    <div class="">
                        <el-input v-model="ruleFormEdu.specialty" placeholder="请输入专业名称"></el-input>
                    </div>
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
                    <div class="">
                        <el-input v-model="ruleFormTraining.name" placeholder="请输入培训中心"></el-input>
                    </div>
                    <div class="wxsettip_small ">培训方向</div>
                    <div class="">
                        <el-input v-model="ruleFormTraining.title" placeholder="请输入培训方向"></el-input>
                    </div>
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
                    <div class="">
                        <el-input v-model="ruleFormProject.name" placeholder="请输入项目名称"></el-input>
                    </div>
                    <div class="wxsettip_small ">担任职务</div>
                    <div class="">
                        <el-input v-model="ruleFormProject.title" placeholder="请输入担任职务"></el-input>
                    </div>
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
                    <div class="">
                        <el-input v-model="ruleFormOther.name" placeholder="请输入标题名称"></el-input>
                    </div>
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
                    <div class="">
                        <el-input v-model="ruleFormSkill.name" placeholder="请输入技能名称"></el-input>
                    </div>
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
                        <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept" action=""
                            :auto-upload="false" :on-change="handleChangeSkillPic" :show-file-list="false">
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
        <!--新增简历-->
        <div class="modluDrawer">
            <el-drawer title="新增简历" :visible.sync="drawerResume" append-to-body :wrapper-closable="false" size="45%">
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
            dataText: '数据加载中',
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
            dynamicTags: ['吃苦耐劳', '服从安排', '有工厂工作经验', '无纹身', '无不良嗜好', '适应工厂工作', '熟悉流水线', '适应夜班', '适应加班'],
            inputVisible: false,
            inputValue: '',
            tableData: [],
            items: [{
                type: '',
                label: '正常'
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
            searchParams: {},

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

            userStatusNum3: 0,
            userAllNum: 0,

            saveLoading: false,

            // 身份证认证
            dialogIdcardRz: false,
            ruleFormIdcardRz: {},
            // 手机认证
            dialogMoblieRz: false,
            ruleFormMobileRz: {},
            // 邮件认证
            dialogEmailRz: false,
            ruleFormEmailRz: {},

            // 批量认证
            dialogAuth: false,
            ruleFormAuth: {},

            // 分站切换
            dialogDomain: false,
            ruleFormDomain: {},
            domainList: {},

            // 查看
            drawerDetail: false,
            member: {},
            resume: {},
            expectData: {},

            // 缓存
            user_sex: {},
            userclass_name: {},
            userdata: {},
            industry_index: [],
            industry_name: {},

            // 添加
            dialogAdd: false,
            ruleFormAdd: {},
            provinceList: [],
            cityList: [],
            regionList: [],

            // 账户信息
            dialogAccount: false,
            ruleFormAccount: {},
            // 账户合并
            dialogAccountMerge: false,
            AccountMergeComname: '',
            ruleFormAccountMerge: {},

            // 删除
            dialogDel: false,
            ruleFormDel: {},

            expectLoading: true,

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
            jobSqLog: {
                page: 1,
                limit: 0,
                total: 0
            },
            // 面试邀请
            yqmsLog: {
                page: 1,
                limit: 0,
                total: 0
            },
            // 行为分析
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
        //用来阻止第二次或更多次点击密码输入框时下拉用户密码清单的框一闪而过的问题
        pwdMousedown(){
            var that = this
            this.pwdreadonly = true
            setTimeout(function(){ that.pwdreadonly = false, 100})
        },
        // 防止密码框内容清楚后展示用户密码清单
        pwdchange: function(val){
            var that = this
            if (val == '') {
                this.pwdreadonly = true
                setTimeout(function(){ that.pwdreadonly = false, 100})
            }
        },
        // 修改密码框readonly属性，防止密码框展示浏览器记录的密码信息
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
            // 这里面需要注意，通过ref需要那个那个包含table元素的父元素
            let divData = this.$refs.multipleTable.bodyWrapper;
            if (this.mouseFlag) {
                // 设置水平方向的元素的位置
                divData.scrollLeft -= (- this.mouseOffset + (this.mouseOffset = e.clientX));
            }
        },
        // 跳转会员中心前检测
        memberCheck: function (uid, usertype) {
            var that = this
            var tip = ''
            if (usertype != '1') {
                if (usertype == '0') {
                    tip = '该账户当前没有设置身份，以求职者身份模拟进入可能导致部分功能无法正常使用，是否确认进入？'
                } else {
                    if (usertype == '2') {
                        var u = '招聘者';
                    }
                    tip = "该账户当前身份为" + u + "，以求职者身份模拟进入可能导致部分功能无法正常使用，是否确认进入？"
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
            } else if (this.multipleSelection.length == 0) {
                message.error('请选择要操作的数据项');
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

        // 删除
        openDel(idx) {
            if (typeof idx == 'undefined') { // 批量删除
                this.ruleFormDel = {
                    del: this.idArr,
                    delAccount: 0
                }
            } else { // 单个删除
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
                message.error('请选择审核状态');
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
        // 手机认证
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
                message.error('请输入手机号');
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
                message.error('请输入邮箱');
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
                    message.success('查询成功');
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
                    message.success('查询成功');
                }
            })
        },

        openDomain(row) {
            if (typeof row == 'undefined') { // 批量操作
                this.detail = {};
                this.$set(this.ruleFormDomain, 'uid', this.idArr);
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
                message.error('请选择需要切换的站点');
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
                    if (typeof ruleForm.uid == 'object') { // 批量删除
                        that.getList();
                    } else { // 单个删除
                        that.refreshList = true;
                        // 重新拉取详情
                        that.getDetail(ruleForm.uid);
                    }
                    message.success(res.msg)
                }
            })
        },
        // 批量认证
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
                message.error('请选择认证类型');
                return false;
            }

            if (typeof ruleForm.status == 'undefined' || ruleForm.status === '') {
                message.error('请选择认证状态');
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

        // 投递记录
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
            this.activeName = 'resume';
            // 投递记录
            this.$set(this.$data, 'jobSqLog', {
                page: 1,
                limit: 0,
                total: 0
            });
            // 面试邀请
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
                pageCode: '',
                xialaStatus: true
            };
            // 个人动态
            this.userLog = {
                page: 1,
                limit: 0,
                list: null
            };
            // 积分管理
            this.$set(this.$data, 'payLog', {
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
                message.error('请输入用户名');
                return false;
            }
            if (typeof ruleForm.password === 'undefined' || $.trim(ruleForm.password) == "") {
                message.error('请输入登录密码');
                return false;
            }
            if (typeof ruleForm.moblie === 'undefined' || $.trim(ruleForm.moblie) == "") {
                message.error('请输入手机号');
                return false;
            } else if (!isjsMobile(ruleForm.moblie)) {
                message.error('手机号格式错误');
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
                    }, "该账户当前没有设置身份，以求职者身份模拟进入可能导致部分功能无法正常使用，是否确认进入？")
                } else {
                    var usertype = '';
                    if (row.usertype == '2') {
                        usertype = '招聘者';
                    }

                    delConfirm(that, params, function (params) {
                        that.getMemberUrl(row.uid);
                    }, "该账户当前身份为" + usertype + "，以求职者身份模拟进入可能导致部分功能无法正常使用，是否确认进入？")
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

        // 账户信息
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

        // 账户合并
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
                message.error('请输入企业名称并选择企业');
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

        // 重置密码
        resetPassword(row) {
            let that = this;
            delConfirm(that, { uid: row.uid }, function (params) {
                httpPost('m=user&c=users_member&a=reset_pw', params).then(function (res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.alert("用户：" + row.username + " 密码已经重置为123456！");
                    }
                })
            }, '确定要重置密码吗？')
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
                    message.error('请输入2-8个标签字符');
                    return false;
                }
                if (tag.length >= 5) {
                    message.error('标签最多选择5个');
                    return false;
                }
                if (userTag.indexOf(tagval) > -1) {
                    message.error('标签已存在');
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
                    message.error('标签最多选择5个');
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
                message.error('请先完善求职意向');
                return false;
            }
            if (ruleForm.tag.length > 5) {
                message.error('标签最多选择5个');
                return false;
            }
            if (ruleForm.description == '' || ruleForm.description == null) {
                message.error('请输入自我评价');
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
        // 求职意向
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
                message.error('请输入期望职位');
                return false;
            }
            if (ruleForm.job_classid == "") {
                message.error('请选择从事职位');
                return false;
            }
            if (ruleForm.city_classid == '') {
                message.error('请选择期望地点');
                return false;
            }
            if (ruleForm.minsalary == "" || ruleForm.minsalary == "0") {
                message.error('请选择期望薪资');
                return false;
            }
            if (ruleForm.maxsalary && parseInt(ruleForm.maxsalary) <= parseInt(ruleForm.minsalary)) {
                message.error('最高薪资必须大于最低薪资');
                return false;
            }
            if (ruleForm.report == "") {
                message.error('请选择到岗时间');
                return false;
            }
            if (ruleForm.type == "") {
                message.error('请选择工作性质');
                return false;
            }
            if (ruleForm.jobstatus == "") {
                message.error('请选择求职状态');
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
                message.error('请先完善求职意向');
                return false;
            }
            if (ruleForm.name == "") {
                message.error('请输入公司名称');
                return false;
            }
            if (ruleForm.sdate == "") {
                message.error('请选择工作时间');
                return false
            }
            ruleForm.sdate = formatMonth(ruleForm.sdate);
            if (ruleForm.edate != '') {
                if (ruleForm.sdate >= ruleForm.edate) {
                    message.error('结束日期必须大于开始日期');
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

                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
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
                message.error('请先完善求职意向');
                return false;
            }
            if (ruleForm.name == "") {
                message.error('请输入学校名称');
                return false;
            }
            if (daterangeEdu.length == 0) {
                message.error('请选择在校时间');
                return false
            }
            if (ruleForm.education == "") {
                message.error('请选择最高学历');
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
                message.error('请先完善求职意向');
                return false;
            }
            if (ruleForm.name == "") {
                message.error('请输入培训中心');
                return false;
            }
            if (daterangeTraining.length == 0) {
                message.error('请选择培训时间');
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
                message.error('请先完善求职意向');
                return false;
            }
            if (ruleForm.name == "") {
                message.error('请输入技能名称');
                return false;
            }
            if (ruleForm.ing == "") {
                message.error('请选择熟练程度');
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
                message.error('请先完善求职意向');
                return false;
            }
            if (ruleForm.name == "") {
                message.error('请输入项目名称');
                return false;
            }
            if (daterangeProject.length == 0) {
                message.error('请选择项目时间');
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
                message.error('请先完善求职意向');
                return false;
            }
            if (ruleForm.name == "") {
                message.error('请输入标题名称');
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
            }, '确定要删除该项内容？');
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
                    that.dataText = "暂无数据";
                }
            })
        },
        // 面试邀请
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
                    that.dataText = "暂无数据";
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
        // 积分管理
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
                    that.dataText = "暂无数据";
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