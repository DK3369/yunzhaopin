<template>
    <div class="moduleElHight" :class="searchClass == 'drawer' ? 'pad_lr_20' : ''">
        <div class="moduleSeachbig">
            <!--关键字搜索和查询在一起-------------------------------------------------------------------->
            <div class="tableSeachInpt tableSeachInptsmall tableSeacFromer" style="padding: 2px 0;">
                <el-input v-model="search_params.keyword" @keyup.enter.native="search" placeholder="请输入搜索内容" class="input-with-select" size="small"
                     clearable>
                    <el-select slot="prepend" v-model="search_params.type" size="small" placeholder="用户名">
                        <el-option label="名称/简称" value="1"></el-option>
                        <el-option label="用户名称" value="2"></el-option>
                        <el-option label="联系人" value="3"></el-option>
                        <el-option label="联系电话" value="4"></el-option>
                        <el-option label="用户邮箱" value="5"></el-option>
                        <el-option label="用户ID" value="6"></el-option>
                        <el-option label="IP" value="7"></el-option>
                        <el-option label="企业地址" value="8"></el-option>
                    </el-select>
                </el-input>
            </div>
            <!--收起部分-->
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="search_params.time_type" size="small" slot="prepend" placeholder="筛选日期" clearable @change="handleTimeChange">
                    <el-option label="注册时间" value="adtime"></el-option>
                    <el-option label="登录时间" value="lotime"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmalltwo" :class="{ 'searchbutnOnff': seachbutn }">
                <el-date-picker v-model="search_params.times" type="daterange" align="right" unlink-panels range-separator="至" start-placeholder="开始日期" end-placeholder="结束日期" :picker-options="timeOptions" value-format="yyyy-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall" v-for="(searchitem, searchidx) in searchlist" :key="searchidx" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="search_params[searchidx]" size="small" slot="prepend" :placeholder="searchitem.name" clearable @change="search()">
                    <el-option v-for="(item, index) in searchitem.value" :label="item" :key="index" :value="index"></el-option>
                </el-select>
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
                <el-button type="primary" plain icon="el-icon-document-add" size="mini" @click="addcom">新增企业</el-button>
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
        <div class="admin_datatip">
            <i class="el-icon-document"></i> 数据统计
            <span class="admin_datatip_n">共：{{allNum }} 条</span>
            <span class="admin_datatip_n">未审核：{{status1Num }} 条</span>
            <span class="admin_datatip_n">未通过：{{status2Num }} 条</span>
            <span class="admin_datatip_n">已锁定：{{status3Num }} 条</span>
            <span class="admin_datatip_n">搜索结果：{{total }} 条</span>
        </div>
        <div class="moduleElTable" :class="{ 'modulElTableGai': tableHig }"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe @sort-change='sortChange'
                @mousedown.native="mouseDownHandler"
                @mouseup.native="mouseUpHandler"
                @mousemove.native="mouseMoveHandler"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                :default-sort="{ prop: 'uid', order: 'descending' }" @selection-change="handleSelectionChange"
                ref="multipleTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="50"></el-table-column>
                <el-table-column prop="uid" label="用户ID" width="90" sortable="custom"></el-table-column>
                <el-table-column label="名称/用户名/认证" min-width="220" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class="username">
                                <el-link target="_blank" :href="scope.row.comUrl">{{ scope.row.name }}</el-link>
                                <span v-if="scope.row.shortname">【{{ scope.row.shortname }}】</span>
                            </div>
                            <div class="yhm">
                                <el-link @click="memberCheck(scope.row.uid, scope.row.usertype)" :underline="false">{{ scope.row.username
                                }}
                                </el-link>
                                <el-tooltip v-if="scope.row.r_status == '2'" class="item" effect="dark" content="已锁定"
                                            placement="top-start">
                                    <i class="el-icon-lock" style="color: orange"></i>
                                </el-tooltip>
                            </div>
                        </div>
                        <div class="rz_box" style="padding-top: 5px;">
                            <el-tooltip v-if="scope.row.moblie_status == '1'" class="item" effect="dark" content="手机已认证"
                                placement="top-start">
                                <div slot="content">
                                    <span style="line-height: 20px;">手机已绑定</span><br />
                                    <span style="line-height: 20px;">点击图标，可弹出绑定手机号</span>
                                </div>
                                <el-button type="text"
                                    @click="sjrz(scope.row.linktel, scope.row.moblie_status, scope.row.uid)">
                                    <i class="rzicon rzicon_sjyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" placement="top-start">
                                <div slot="content">
                                    <span style="line-height: 20px;">手机未绑定</span><br />
                                    <span style="line-height: 20px;">点击图标，可弹出绑定手机号</span>
                                </div>
                                <el-button type="text"
                                    @click="sjrz(scope.row.linktel, scope.row.moblie_status, scope.row.uid)">
                                    <i class="rzicon rzicon_sjwrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-if="scope.row.wxid != '' || scope.row.wxopenid != ''"
                                class="item" effect="dark" placement="top-start">
                                <div slot="content">
                                    <span style="line-height: 20px;" v-html="scope.row.wxBindmsg"></span><br />
                                    <span style="line-height: 20px;">点击图标，可弹出绑定微信二维码</span>
                                </div>
                                <el-button type="text" @click="showQrcode(scope.row.uid, scope.row.wxid)">
                                    <i class="rzicon rzicon_wxyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" placement="top-start">
                                <div slot="content">
                                    <span style="line-height: 20px;" v-html="scope.row.wxBindmsg"></span><br />
                                    <span style="line-height: 20px;">点击图标，可弹出绑定微信二维码</span>
                                </div>
                                <el-button type="text" @click="showQrcode(scope.row.uid, scope.row.wxid)">
                                    <i class="rzicon rzicon_wxwrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-if="scope.row.yyzz_status == '1'" class="item" effect="dark"
                                placement="top-start">
                                <div slot="content">
                                    <span style="line-height: 20px;">企业资质已认证</span><br />
                                    <span style="line-height: 20px;">点击图标，可查看企业资质认证</span>
                                </div>
                                <el-button type="text" @click="yyzzrz(scope.row)">
                                    <i class="rzicon rzicon_zzyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" placement="top-start">
                                <div slot="content">
                                    <span style="line-height: 20px;">企业资质未认证</span><br />
                                    <span style="line-height: 20px;">点击图标，可查看企业资质认证</span>
                                </div>
                                <el-button type="text" @click="yyzzrz(scope.row)">
                                    <i class="rzicon rzicon_zzwrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-if="scope.row.email_status == '1'" class="item" effect="dark"
                                placement="top-start">
                                <div slot="content">
                                    <span style="line-height: 20px;">邮箱已认证</span><br />
                                    <span style="line-height: 20px;">点击图标，可弹出绑定邮箱</span>
                                </div>
                                <el-button type="text" @click="yxrz(scope.row.linkmail, scope.row.email_status, scope.row.uid)">
                                    <i class="rzicon rzicon_yxyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" placement="top-start">
                                <div slot="content">
                                    <span style="line-height: 20px;">邮箱未认证</span><br />
                                    <span style="line-height: 20px;">点击图标，可弹出绑定邮箱</span>
                                </div>
                                <el-button type="text"
                                    @click="yxrz(scope.row.linkmail, scope.row.email_status, scope.row.uid)">
                                    <i class="rzicon rzicon_yxwrz"></i>
                                </el-button>
                            </el-tooltip>
                            <!--实地-->
                            <el-tooltip v-if="scope.row.fact_status == '1'" class="item" effect="dark" placement="top-start">
                                <div slot="content">
                                    <span style="line-height: 20px;">实地已核验</span><br />
                                    <span style="line-height: 20px;">点击图标，可弹出实地核验</span>
                                </div>
                                <el-button type="text" @click="factrz(scope.row)">
                                    <i class="rzicon rzicon_sdyrz"></i>
                                </el-button>
                            </el-tooltip>
                            <el-tooltip v-else class="item" effect="dark" placement="top-start">
                                <div slot="content">
                                    <span style="line-height: 20px;">实地未核验</span><br />
                                    <span style="line-height: 20px;">点击图标，可弹出实地核验</span>
                                </div>
                                <el-button type="text" @click="factrz(scope.row)">
                                    <i class="rzicon rzicon_sdwrz"></i>
                                </el-button>
                            </el-tooltip>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="等级/到期时间" min-width="180">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class="username">
                                <span
                                    v-if="today <= scope.row.vipetime || (scope.row.vipetime == '0' && scope.row.rating > '0')">
                                    {{ scope.row.rating_name }}
                                    <img src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;"
                                        @click="editRating(scope.row.uid, scope.row.r_status)" width="14" height="14">
                                </span>
                                <span v-else style="color: red;">
                                    {{ scope.row.oldrating_name ? scope.row.oldrating_name : scope.row.rating_name }}
                                    <img src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;"
                                        @click="editRating(scope.row.uid, scope.row.r_status)" width="14" height="14">
                                </span>
                            </div>
                            <div v-if="scope.row.vipetime == '0'" class="mt5">不限</div>
                            <div v-else :style="todayetime >= scope.row.vipetime ? 'color: red;' : ''">
                                {{ scope.row.vip_etime_n }}{{ todayetime >= scope.row.vipetime ? '到期' : '' }}
                            </div>
                            <span class="gsd">
                                {{ scope.row.crm_uid > '0' ? '业务员：' + scope.row.crm_name : '未分配' }}
                                <el-button type="text" size="mini"
                                    @click="fpgw(1, scope.row.username, scope.row.crm_uid,scope.row.uid)">分配</el-button>
                            </span>
                        </div>
                    </template>
                </el-table-column>
				<el-table-column prop=" " label="企业LOGO" width="100">
				    <template slot-scope="props">
				        <div class="layuiSmallImg">
				            <a href="javascript:;" class="layuiSmallImgUp" @click="makeLogo(props.row.uid, props.row.shortname, props.row.name, 2)">
				                <el-image :src="props.row.logo" style="width:48px;height:48px">
				                    <div slot="error" class="image-slot">
									  <img src="../../../admin/images/imaheg.png" alt="">
				                    </div>
				                </el-image>
				                <div v-if="!props.row.logo">
				                    <a href="javascript:;" class="layui-btn layui-btn-small layuiSmallImgDwon" style="font-size: 12px;;" @click="makeLogo(props.row.uid, props.row.shortname, props.row.name, 1)">生成LOGO</a>
				                </div>
				            </a>
				        </div>
				    </template>
				</el-table-column>
                <el-table-column label="手机号/归属地" min-width="130">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <span v-if="props.row.linktel || props.row.linkphone">{{ props.row.linktel ? props.row.linktel : props.row.linkphone }}</span>
                            <span class="gsd" v-if="props.row.moblie_address"> {{ props.row.moblie_address }}</span>
                            <span class="gsd" v-else-if="props.row.linktel || props.row.linkphone">
                                <el-link type="primary" size="mini" @click="getmobileaddress(props.row.uid, props.row.linktel ? props.row.linktel : props.row.linkphone)">查询归属地</el-link>
                            </span>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column prop="login_date" label="注册/登录" min-width="150" sortable="custom">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <span class="gsd">{{ props.row.reg_date_n }}</span>
                            <span>{{ props.row.login_date > 0 ? props.row.login_date_n : '未登录' }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="ip" label="来源/IP/归属地" min-width="180">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <span>{{ source[props.row.source] }}</span>
                            <span>{{ props.row.login_ip }}</span>
                            <span class="gsd" v-if="props.row.login_address">{{ props.row.login_address }}</span>
                            <span class="gsd" v-else-if="props.row.login_ip">
                                <el-link @click="getipaddress(props.row.uid, props.row.login_ip)" size="mini" type="primary">查询归属地</el-link>
                            </span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="ip" label="职位数" min-width="100">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <span>总数：<el-button type="text" @click="JumpComJob(props.row, 0)">{{ props.row.jobnum ? props.row.jobnum : 0 }}</el-button></span>
                            <span>在招：<el-button type="text" @click="JumpComJob(props.row, 2)">{{ props.row.zz_jobnum ? props.row.zz_jobnum : 0 }}</el-button></span>
                            <span class="jobtj">
                                <el-link icon="el-icon-document-add" size="mini" @click="addjob(props.row.uid)">添加职位</el-link>
                            </span>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column prop="zt" label="状态" fixed="right">
                    <template slot-scope="props">
                        <div class="admin_state">
                            <span class="admin_state1" v-if="props.row.r_status == '1'">已审核</span>
                            <div v-else-if="props.row.r_status == '3'">
                                <span class="admin_state2">未通过</span>
                                <div style="display:inline-block">
                                    <el-popover trigger="hover" placement="right" v-if="props.row.lock_info" >
                                        <p >{{ props.row.lock_info }}</p>
                                        <div slot="reference" class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div>
                                    </el-popover>
                                </div>
                            </div>
                            <div v-else-if="props.row.r_status == '2'">
                                <span class="admin_state3">已锁定</span>
                                <div style="display:inline-block">
                                    <el-popover trigger="hover" placement="right" v-if="props.row.lock_info" >
                                        <p >{{ props.row.lock_info }}</p>
                                        <div slot="reference" class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div>
                                    </el-popover>
                                </div>
                            </div>
                            <span class="admin_state5" v-else-if="props.row.r_status == '4'">已暂停</span>
                            <span class="admin_state4" v-else>未审核</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="操作" width="140" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="comaudit(scope.row.uid)">审核</el-button>
                            <el-button size="mini" plain @click="comrz(scope.row)">日志</el-button>
                        </div>
                        <div class="cz_button" style="margin-top: 10px;">
                            <el-button size="mini" plain
                                @click="addTuiWenTask(scope.row.uid, scope.row.name, scope.row.lastupdate, scope.row.tw_num)">
                                推文
                            </el-button>
                            <el-button size="mini" plain @click="cominfo(scope.$index, scope.row.uid)">详情</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging" style="height: initial; flex-wrap: wrap; padding-top: 10px;">
            <div class="bottomButnBull" style="width:100%;">
                <div class="bottomButnBlak">
                    <el-checkbox v-model="checkedAll" @change="selectAllBottom">全选</el-checkbox>
                    <el-button size="mini" @click="multipleStatus">批量审核</el-button>
                    <el-button size="mini" @click="openDel(undefined)">批量删除</el-button>
                    <el-button size="mini" @click="exportdrawer = true">导出</el-button>
                    <el-button size="mini" @click="multiFpzd">批量选择分站</el-button>
                    <el-button size="mini" @click="fpgw(2, '')">批量分配顾问</el-button>
                    <el-button size="mini" @click="multirz">批量认证</el-button>
                    <el-button size="mini" @click="twtaskall">推文任务</el-button>
                </div>
                <!-- <div class="bottomButnNone">
                    <el-popover placement="top-start" width="520" trigger="hover">
                        <div class="bottomButnGend">
                            <el-button size="mini" @click="exportdrawer = true">导出</el-button>
                            <el-button size="mini" @click="multiFpzd">批量选择分站</el-button>
                            <el-button size="mini" @click="fpgw(2, '')">批量分配顾问</el-button>
                            <el-button size="mini" @click="multirz">批量认证</el-button>
                            <el-button size="mini" @click="twtaskall">推文任务</el-button>
                        </div>
                        <div class="bottomButnMore" slot="reference">更多</div>
                    </el-popover>
                </div> -->

                <!-- <el-button size="mini" @click="exportdrawer = true">导出</el-button>
                <el-button size="mini" @click="multiFpzd">批量选择分站</el-button>
                <el-button size="mini" @click="fpgw(2, '')">批量分配顾问</el-button>
                <el-button size="mini" @click="multirz">批量认证</el-button>
                <el-button size="mini" @click="twtaskall">推文任务</el-button> -->
            </div>
            <div class="modulePagNum" style="padding-top: 8px;">
                <el-pagination background @size-change="handleSizeChange"
                    @current-change="handleCurrentChange" :current-page="currentPage" :page-sizes="pageSizes"
                    :page-size="perPage" layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <!--企业详情审核 ---------------------------------------------------------------------->
        <el-drawer title="企业审核" :visible.sync="comdrawersh" append-to-body :modal-append-to-body="false" :close-on-click-modal="false" size="80%">
            <div class="shbox" v-if="comdrawersh">
                <div class="shinfo">
                    <div class="shcomname">{{ curr_com.name }}
                        <el-tag type="danger" size="mini">{{ curr_com.rating_name }}</el-tag>
                    </div>
                    <div class="sh_zwsz_add">
                        <span v-if="curr_com.linkman">联系人：{{ curr_com.linkman }}<span v-if="curr_com.linkjob">({{
                            curr_com.linkjob }})</span></span>
                        <span class="shcomtel_n" v-if="curr_com.linktel">联系手机：{{ curr_com.linktel }}
                            <span>{{ curr_com.infostatus == 1 ? '（公开）' : '（不公开）' }}</span>
                            <span v-if="curr_com.moblie_address">归属地：{{ curr_com.moblie_address }}</span>
                        </span>
                        <span class="shcomtel_n" v-if="curr_com.linkphone">固定电话：{{ curr_com.linkphone }}</span>
                        <span v-if="curr_com.crm_uid != '0'">业务员：{{ curr_com.crm_name }}</span>
                    </div>
                    <div class="shcomtel">
                        <span v-if="curr_com.reg_date_n">注册时间：{{ curr_com.reg_date_n }}</span>
                        <span v-if="curr_com.login_date_n" class="shcomtel_n">最近登录时间：{{ curr_com.login_date_n }} </span>
                        <span v-if="curr_com.login_ip">IP：{{ curr_com.login_ip }}</span>
                    </div>
                    <div class="shshowall">
                        <div class="shshow">
                            <div class="shshow_tit"><i class="el-icon-office-building"></i> 基本资料</div>
                            <div class="shshow_p">
                                <div v-if="curr_com.welfare_n">企业福利：
                                    <el-tag v-for="(item, key) in curr_com.welfare_n" :key="key" size="mini">{{ item
                                    }}</el-tag>
                                </div>
                                <div v-if="curr_com.hy_n">从事行业：{{ curr_com.hy_n }}</div>
                                <div v-if="curr_com.pr_n">企业性质：{{ curr_com.pr_n }}</div>
                                <div v-if="curr_com.mun_n">企业规模：{{ curr_com.mun_n }}</div>
                                <div v-if="curr_com.provinceid">企业地址：{{ curr_com.job_city_one }} {{ curr_com.job_city_two }}
                                    {{ curr_com.job_city_three }} {{ curr_com.address }}
                                </div>
                                <div v-if="curr_com.content" v-html="curr_com.content"></div>
                            </div>
                            <div class="shshow_tit" v-if="curr_com.job_list.length > 0"><i class="el-icon-suitcase-1"></i>
                                招聘岗位
                            </div>
                            <ul class="shshow_joblist" v-if="curr_com.job_list.length > 0">
                                <li v-for="(item, index) in curr_com.job_list" :key="index">
                                    <el-link type="primary"  :href="item.joburl" target="_blank"  :underline="false">
                                        <div class="shshow_job">{{ item.name }}</div>
                                    </el-link>
                                    <div class="shshow_jobinfo">
                                        <span class="shshow_jobxz">{{ item.job_salary }}</span>
                                        <span class="shshow_line" v-if="item.job_exp">|</span> {{ item.job_exp == '不限' ?
                                            '不限经验' : item.job_exp }}
                                        <span class="shshow_line" v-if="item.job_edu">|</span> {{ item.job_exp == '不限' ?
                                            '不限学历' : item.job_edu }}
                                    </div>
                                </li>
                            </ul>
                        </div>
                        <div class="shcz">
                            <div class="wxsettip_small">企业审核</div>
                            <div v-if="islock" style="margin-bottom: 10px;">
                                <template>
                                    <el-radio v-model="member_status" label="1">正常</el-radio>
                                    <el-radio v-model="member_status" label="2">锁定</el-radio>
                                </template>
                                <div class="wxsettip_small ">锁定状态说明</div>
                                <el-alert :title="lockdesc" type="warning" show-icon :closable="false">
                                </el-alert>
                            </div>
                            <div v-if="member_status != '2'">
                                <template>
                                    <el-radio v-model="r_status" label="0">未审核</el-radio>
                                    <el-radio v-model="r_status" label="1">通过</el-radio>
                                    <el-radio v-model="r_status" label="3">未通过</el-radio>
                                </template>
                                <div class="wxsettip_small ">审核状态说明</div>
                                <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="statusbody">
                                </el-input>
                            </div>
                            <div class="shczbth">
                                <el-button type="primary" @click="comSh(1)">提 交</el-button>
                            </div>
                            <div class=" shczbth" v-if="snum > '0' && member_status != '2'">
                                <el-button type="primary" @click="comSh(2)" plain>提交，并审核下一个</el-button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </el-drawer>
        <!--批量审核企业-->
        <div class="modluDrawer">
            <el-dialog title="企业批量审核" width="400px" :visible.sync="drawerauditmultiple" append-to-body
                :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>审核操作：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-radio v-model="multiStatus" label="0">未审核</el-radio>
                            <el-radio v-model="multiStatus" label="1">已审核</el-radio>
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
                    <el-button type="primary" @click="multipleStatusSave">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--企业日志 ---------------------------------------------------------------------->
        <el-drawer title="企业日志" :visible.sync="qyrz" append-to-body :modal-append-to-body="false" size="80%">
            <div class="elTabQanCompany">
                <comlog :typelist="typeArr" :time="time" :type="'3'" :keyword="curr_com.uid"></comlog>
            </div>
        </el-drawer>
        <!--企业详情 ---------------------------------------------------------------------->
        <el-drawer title="企业详情" :visible.sync="qyxqdrawer" append-to-body :modal-append-to-body="false" size="95%">
            <div class="shbox">
                <div class="shinfo">
                    <div class="shinfotop">
                        <div class="shinfologo"><img :src="curr_com.logo_n" width="60" height="60"></div>
                        <div class="shcomname">{{ curr_com.name ? curr_com.name : '企业尚未完善资料' }}</div>
                        <div class="shcomdj">会员等级：
                            <el-tag type="danger" size="mini">{{ curr_com.rating_name }}</el-tag>
                            <span class="cominfo_dq">{{ curr_com.vipetime>0 ? curr_com.vipetime_n + '到期' : '不限'}}</span>
                            <el-button type="text" @click="editRating(curr_com.uid, curr_com.r_status)"><i
                                    class="el-icon-edit"></i>修改等级
                            </el-button>
                            <el-button type="text" v-if="curr_com.r_status != 4" @click="comzt(curr_com.r_status)"><i
                                    class="el-icon-thumb"></i>暂停会员
                            </el-button>
                            <el-button type="text" v-else @click="comunzt(curr_com.zt_days)"><i
                                    class="el-icon-thumb"></i>解除暂停
                            </el-button>
                        </div>
                        <div class="sh_zwsz">
                            <el-button type="primary" size="mini" @click="memberCheck(curr_com.uid, 2)"><i
                                    class="el-icon-school"></i> 进入企业中心
                            </el-button>
                        </div>
                    </div>
                    <div class="shcomtel" style="padding-bottom:15px; padding-top:10px;;border:none;font-size: 13px;">
                        <span class=" " v-if="curr_com.reg_date_n">注册时间：{{ curr_com.reg_date_n }} </span>
                        <span class="shcomtel_n" v-if="curr_com.login_date_n">最近登录 ：{{ curr_com.login_date_n }} </span>
                        <span class=" " v-if="curr_com.reg_ip"> IP：{{ curr_com.reg_ip }} </span>
                        <span class=" ">
                            <span class="shcomtel_n" v-if="curr_com.source_n != ''">来源：{{ curr_com.source_n }}</span>
                            <span class=" ">站点：{{ dnameArr[curr_com.did] }}</span>
                            <el-button type="text" @click="drawerfpzd = true"><i
                                    class="el-icon-map-location"></i>分配站点</el-button>
                        </span>
                    </div>
                    <div class="cominfocz">
                        <el-button type="primary" size="mini" @click="openAccount"> 账户信息</el-button>
                        <el-button v-if="curr_com.hottime && curr_com.rec == '1'" type="primary" size="mini" @click="qxmq">
                            取消名企
                        </el-button>
                        <el-button v-else type="primary" size="mini" @click="setmq"> 设为名企</el-button>
                        <el-button type="primary" size="mini" @click="createhb"> 生成海报</el-button>
                        <el-button type="primary" size="mini" @click="commb"> 企业模板</el-button>
                        <el-button type="primary" size="mini" @click="sendmsg"> 发短信</el-button>
                        <el-button type="primary" size="mini" @click="sendmail"> 发邮件</el-button>
                        <el-button type="primary" size="mini" @click="jumpToMember(curr_com.uid, 'tongji')"> 招聘效果
                        </el-button>
                        <el-button type="primary" size="mini" @click="resetpass"> 重置密码</el-button>
                        <el-button type="primary" size="mini" @click="bindPackage"> 绑定套餐</el-button>
                        <el-button type="danger" size="mini" @click="openDel(comindex, curr_com.uid)"> 删除该企业</el-button>
                    </div>
                    <!--企业详情切换-->
                    <el-tabs v-model="activeName" type="card" @tab-click="handleClick">
                        <el-tab-pane label="基本资料" name="first" :lazy="true">
                            <div class="shshow_tit">
                                <i class="el-icon-mobile"></i> 联系方式
                                <span class="shshow_cz">
                                    <el-button type="text" @click="editcom"><i class="el-icon-edit"></i>编辑资料</el-button>
                                </span>
                            </div>
                            <div class="shshow_p">
                                <div class="cominfo">联系人：{{ curr_com.linkman ? curr_com.linkman : '暂未填写' }}</div>
                                <div class="cominfo">联系电话：{{ curr_com.phone ? curr_com.phone : '暂未填写' }}</div>
                                <div class="cominfo" v-if="curr_com.crm_uid > 0">业务员：{{ curr_com.crm_name }}</div>
                            </div>
                            <div class="shshow_tit"><i class="el-icon-office-building"></i> 基本资料</div>
                            <div class="shshow_p">
                                <div class="cominfo cominforz">
                                    <div>企业认证：</div>
                                    <div class="rz_box">
                                        <el-tooltip v-if="curr_com.yyzz_status == '1'" class="item" effect="dark"
                                            content="营业执照已认证" placement="top-start">
                                            <el-button type="text">
                                                <i class="rzicon rzicon_zzyrz"></i>
                                            </el-button>
                                        </el-tooltip>
                                        <el-tooltip v-else class="item" effect="dark" content="营业执照未认证"
                                            placement="top-start">
                                            <el-button type="text">
                                                <i class="rzicon rzicon_zzwrz"></i>
                                            </el-button>
                                        </el-tooltip>
                                        <el-tooltip v-if="curr_com.moblie_status == '1'" class="item" effect="dark"
                                            content="手机已认证" placement="top-start">
                                            <el-button type="text">
                                                <i class="rzicon rzicon_sjyrz"></i>
                                            </el-button>
                                        </el-tooltip>
                                        <el-tooltip v-else class="item" effect="dark" content="手机未认证" placement="top-start">
                                            <el-button type="text">
                                                <i class="rzicon rzicon_sjwrz"></i>
                                            </el-button>
                                        </el-tooltip>
                                        <el-tooltip v-if="curr_com.email_status == '1'" class="item" effect="dark"
                                            content="邮箱已认证" placement="top-start">
                                            <el-button type="text">
                                                <i class="rzicon rzicon_yxyrz"></i>
                                            </el-button>
                                        </el-tooltip>
                                        <el-tooltip v-else class="item" effect="dark" content="邮箱未认证" placement="top-start">
                                            <el-button type="text">
                                                <i class="rzicon rzicon_yxwrz"></i>
                                            </el-button>
                                        </el-tooltip>
                                        <el-tooltip
                                            v-if="curr_com.wxid != '' || curr_com.wxopenid != ''"
                                            class="item" effect="dark" content="微信已认证" placement="top-start">
                                            <el-button type="text">
                                                <i class="rzicon rzicon_wxyrz"></i>
                                            </el-button>
                                        </el-tooltip>
                                        <el-tooltip v-else class="item" effect="dark" content="微信未认证" placement="top-start">
                                            <el-button type="text">
                                                <i class="rzicon rzicon_wxwrz"></i>
                                            </el-button>
                                        </el-tooltip>
                                        <el-tooltip v-if="curr_com.fact_status == '1'" class="item" effect="dark" placement="top-start">
                                            <div slot="content">
                                                <span style="line-height: 20px;">实地已核验</span><br />
                                                <span style="line-height: 20px;">点击图标，可弹出实地核验</span>
                                            </div>
                                            <el-button type="text">
                                                <i class="rzicon rzicon_sdyrz"></i>
                                            </el-button>
                                        </el-tooltip>
                                        <el-tooltip v-else class="item" effect="dark" placement="top-start">
                                            <div slot="content">
                                                <span style="line-height: 20px;">实地未核验</span><br />
                                                <span style="line-height: 20px;">点击图标，可弹出实地核验</span>
                                            </div>
                                            <el-button type="text">
                                                <i class="rzicon rzicon_sdwrz"></i>
                                            </el-button>
                                        </el-tooltip>
                                    </div>
                                </div>
                                <div class="cominfo" v-if="curr_com.welfare != ''">
                                    企业福利：
                                    <el-tag v-for="(item, index) in curr_com.welfare_n" :key="index" size="mini">
                                        {{ item }}
                                    </el-tag>
                                </div>
                                <div class="cominfo" v-if="curr_com.hy_n != ''">从事行业：{{ curr_com.hy_n }}</div>
                                <div class="cominfo" v-if="curr_com.pr_n != ''">企业性质：{{ curr_com.pr_n }}</div>
                                <div class="cominfo" v-if="curr_com.mun_n != ''">企业规模：{{ curr_com.mun_n }}</div>
                                <div class="cominfo">企业地址：{{ curr_com.job_city_one }} {{ curr_com.job_city_two }}
                                    {{ curr_com.job_city_three }} {{ curr_com.address }}
                                </div>
                                <div class="cominfo" v-if="curr_com.content" v-html="curr_com.content"></div>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="招聘岗位" name="second">
                            <div style="overflow: hidden;position: relative;height:calc(100% - 5px);">
                                <company_job  v-if="activeName == 'second'" :searchuid="curr_com.uid"></company_job>
                            </div>
                        </el-tab-pane>
                        
                        <el-tab-pane label="企业日志" name="xwth" :lazy="true">
                            <div class="eltdraKanOmpany">
                                <comlog ref="comlog"  v-if="activeName == 'xwth'" :typelist="typeArr" :time="time" :type="'3'"
                                    :keyword="curr_com.uid"></comlog>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="下载简历" name="fiveth" :lazy="true">
                            <div style="overflow: hidden;position: relative;height:calc(100% - 5px);">
                                <resumedown v-if="activeName == 'fiveth'" :searchcomid="curr_com.uid"></resumedown>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="收到简历" name="sixth" :lazy="true">
                            <div style="overflow: hidden;position: relative;height:calc(100% - 5px);">
                                <comlog_index v-if="activeName == 'sixth'"  :searchcomid="curr_com.uid" :searchtype="'2'" :searchable="false">
                                </comlog_index>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="面试邀请" name="msth" :lazy="true">
                            <div style="overflow: hidden;position: relative;height:calc(100% - 5px);">
                                <comlog_useridmsg v-if="activeName == 'msth'" :searchcomid="curr_com.uid" :searchtype="'2'" :searchable="false">
                                </comlog_useridmsg>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="积分管理" name="severth" :lazy="true">
                            <div style="overflow: hidden;position: relative;height:calc(100% - 5px);">
                                <company_pay ref="jfgl" v-if="activeName == 'severth'" :cuid="curr_com.uid" :searchable="false"></company_pay>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="充值订单" name="company_order" :lazy="true">
                            <div style="overflow: hidden;position: relative;height:calc(100% - 5px);">
                                <company_order v-if="activeName == 'company_order'" ref="company_order" :cuid="curr_com.uid" :searchable="false"></company_order>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="套餐记录" name="company_tcjl" :lazy="true">
                            <div style="overflow: hidden;position: relative;height:calc(100% - 5px);">
                                <company_tcjl ref="company_tcjl" v-if="activeName == 'company_tcjl'" :cuid="curr_com.uid" :searchable="false"></company_tcjl>
                            </div>
                        </el-tab-pane>
                    </el-tabs>
                </div>
            </div>
        </el-drawer>
        <!-- 企业基本信息弹出框-->
        <el-drawer title="企业基本信息" v-if="hascache" :append-to-body="true" :visible.sync="infoDrawer" :wrapper-closable="false" size="60%">
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
                                <div class="TableTite">企业全称</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="curr_editcom.name" placeholder="请输入企业全称"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">企业简称</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="curr_editcom.shortname" placeholder="请输入企业简称"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">从事行业</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="curr_editcom.hy" placeholder="请选择">
                                        <el-option v-for="(item, index) in cache.industry_index" :key="index"
                                            :label="cache.industry_name[item]" :value="item">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">企业性质</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="curr_editcom.pr" placeholder="请选择">
                                        <el-option v-for="(item, index) in cache.comdata.job_pr" :key="index"
                                            :label="cache.comclass_name[item]" :value="item">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">企业规模</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="curr_editcom.mun" placeholder="请选择">
                                        <el-option v-for="(item, index) in cache.comdata.job_mun" :key="index"
                                            :label="cache.comclass_name[item]" :value="item">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">联系人</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="curr_editcom.linkman" placeholder="请输入联系人"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">所属职位</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="curr_editcom.linkjob" placeholder="请输入所属职位"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">联系手机</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="curr_editcom.linktel" placeholder="请输入联系手机"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">固定电话</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="curr_editcom.linkphone" placeholder="请输入内容"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">联系邮箱</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="curr_editcom.linkmail" placeholder="请输入联系邮箱"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">所在地区</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-cascader v-model="sel_city" :options="cache.cities" @change="citychange" filterable
                                        collapse-tags clearable></el-cascader>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">详细地址</div>
                            </td>
                            <td>
                                <div class="TableInpt">
									<el-autocomplete style="width: 100%;"
										popper-class="my-autocomplete"
										:debounce="1000"
										v-model="curr_editcom.address"
										:fetch-suggestions="addressKeyup"
										placeholder="请输入内容"
										@select="poiSearchClick">
										<i class="el-icon-location-outline el-input__icon" slot="suffix" @click="localsearch('全国')"></i>
										<template slot-scope="{ item }">
											<div class="autocompLtite">
												<div class="name">{{ item.name }}</div>
												<span class="addr">{{ item.address }}</span>
											</div>
											
										</template>
									</el-autocomplete>
								</div>
								<div class="yunyinDiaList" v-if="mapurl">
									<div class="yunyinDiaInpt" style="width:100%;">
										<div class="TableInpt TableInptCoor" style="position:relative; width: 100%;">
											<div id="com_conrtainer" style="width:100%;height:300px; position:relative; z-index:1"></div>
										</div>
									</div>
								</div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">注册资金</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-select v-model="curr_editcom.moneytype" clearable placeholder="请选择">
                                        <el-option v-for="item in moneytypeoptions" :key="item.value" :label="item.label"
                                            :value="item.value">
                                        </el-option>
                                    </el-select>
                                    <el-input v-model="curr_editcom.money" type="number" placeholder="请输入注册资金"
                                        style="margin-left: 10px;">
                                        <template slot="append">{{ curr_editcom.moneytype == '1' ? '万元' : '万美元'
                                        }}</template>
                                    </el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">联系QQ</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="curr_editcom.linkqq" placeholder="请输入联系QQ"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">企业网址</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="curr_editcom.website" placeholder="请输入企业网址"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">企业简介</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <div id="editor—wrapper" style="border: 1px solid #ccc;">
                                        <div id="toolbar-container">
                                            <!-- 工具栏 -->
                                        </div>
                                        <div id="editor-container" style="height: 300px;">
                                            <!-- 编辑器 -->
                                        </div>
                                    </div>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">福利待遇</div>
                            </td>
                            <td>
                                <el-checkbox-group v-model="checkedwelfare">
                                    <el-checkbox v-for="(item, index) in curr_editcom.all_welfare" :label="item"
                                        :key="index">
                                        {{ item }}
                                    </el-checkbox>
                                </el-checkbox-group>
                                <el-input style="margin-left: 0px;" class="input-new-tag" v-if="inputVisible"
                                    v-model="inputValue" ref="saveTagInput" size="small"
                                    @keyup.enter.native="welfareInputConfirm(1)" @blur="welfareInputConfirm(1)">
                                </el-input>
                                <el-button v-else style="margin-left: 0px;" class="button-new-tag" size="small"
                                    @click="showInput">+
                                    新增
                                </el-button>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">公司二维码</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-upload class="avatar-uploader" :action="''" :show-file-list="false"
                                        :on-change="comqcodeChange" :accept="pic_accept">
                                        <img v-if="curr_editcom.comqcode" :src="curr_editcom.comqcode" class="comavatar">
                                        <i v-else class="el-icon-plus comavatar-uploader-icon"></i>
                                    </el-upload>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">公交站点</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" v-model="curr_editcom.busstops"
                                        placeholder="请输入公交站点"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">联系方式</div>
                            </td>
                            <td>
                                <div class="job_set_list">
                                    <el-radio v-model="curr_editcom.infostatus" label="1">公开</el-radio>
                                    <el-radio v-model="curr_editcom.infostatus" label="2">不公开</el-radio>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">审核状态</div>
                            </td>
                            <td>
                                <div class="job_set_list">
                                    <el-radio v-model="curr_editcom.r_status" label="0">未审核</el-radio>
                                    <el-radio v-model="curr_editcom.r_status" label="1">正常</el-radio>
                                </div>
                                <!--<el-input type="textarea" :rows="2" placeholder="请输入理由" v-model="textarea"></el-input>-->
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="setBasicButn" style="border: none; height: 80px;">
                <el-button type="primary" size="medium" @click="comeditsave" :loading="edit_loading">提交</el-button>
            </div>
        </el-drawer>
        <!--职位基本信息弹出框-->
        <el-drawer title="职位基本信息" :visible.sync="drawerEditJob" append-to-body :wrapper-closable="false" size="60%">
            <addjob ref="jobedit" :jid="jobid" :jtypes="job_types" :ctypes="city_types"></addjob>
        </el-drawer>
        <!--执照认证弹窗-->
        <div class="modluDrawer">
            <el-dialog title="营业执照认证" :visible.sync="zzrztc" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">企业名称</div>
                    <el-input placeholder="" v-model="yy_comname"></el-input>
                    <div class="wxsettip_small " v-if="com_social_credit == '1'">统一社会信用代码</div>
                    <el-input v-if="com_social_credit == '1'" v-model="yy_scredit" :disabled="true"></el-input>
                    <div class="wxsettip_small ">认证图片</div>
                    <div class="zzrz_img">
                        <div class="zzrz_imgpreview">
                            <el-image style="width: 80px; height: 80px" :src="yy_picurl" :preview-src-list="yySrcList">
                            </el-image>
                            <div> 执照/代码证</div>
                        </div>
                        <div class="zzrz_imgpreview" v-if="com_cert_owner == '1'">
                            <el-image style="width: 80px; height: 80px" :src="yy_owner_picurl"
                                :preview-src-list="yySrcList">
                            </el-image>
                            <div> 经办人身份证</div>
                        </div>
                        <div class="zzrz_imgpreview" v-if="com_cert_wt && com_cert_wt == '1'">
                            <el-image style="width: 80px; height: 80px" :src="yy_wt_picurl" :preview-src-list="yySrcList">
                            </el-image>
                            <div>委托书/承诺函</div>
                        </div>
                        <div class="zzrz_imgpreview" v-if="com_cert_other == '1'">
                            <el-image style="width: 80px; height: 80px" :src="yy_other_picurl"
                                :preview-src-list="yySrcList">
                            </el-image>
                            <div>其他材料</div>
                        </div>
                    </div>
                    <div class="wxsettip_small ">审核操作</div>
                    <el-radio v-model="yy_status" label="1">正常</el-radio>
                    <el-radio v-model="yy_status" label="2">未通过</el-radio>
                    <div class="wxsettip_small " v-if="com_free_status == '1'">同步操作</div>
                    <el-checkbox v-if="com_free_status == '1'" v-model="yy_job_status">所有未审核职位同步审核成功</el-checkbox>
                    <div class="wxsettip_small ">审核说明</div>
                    <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="yy_sbody"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="zzrztc = false">取 消</el-button>
                    <el-button type="primary" @click="yyzzrzSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--手机认证弹窗-->
        <div class="modluDrawer">
            <el-dialog title="手机认证" :visible.sync="sjrztc" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">手机号</div>
                    <el-input v-model="sj_mobile"></el-input>
                    <div class="wxsettip_small ">审核操作</div>
                    <el-checkbox v-model="sj_status">已认证</el-checkbox>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="sjrztc = false">取 消</el-button>
                    <el-button type="primary" @click="sjrzSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--邮箱认证弹窗-->
        <div class="modluDrawer">
            <el-dialog title="邮箱认证" :visible.sync="yxrztc" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">邮箱号</div>
                    <el-input v-model="yx_email"></el-input>
                    <div class="wxsettip_small ">审核操作</div>
                    <el-checkbox v-model="yx_status">已认证</el-checkbox>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="yxrztc = false">取 消</el-button>
                    <el-button type="primary" @click="yxrzSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--微信认证弹窗-->
        <div class="modluDrawer" v-if="wxrztc">
            <el-dialog title="微信扫码绑定" :visible.sync="wxrztc" :with-header="true" append-to-body
                :modal-append-to-body="false" :show-close="true" width="300px">
                <div class="codeFldex">
                    <div>
                        <div class="code_img">
                            <img :src="code_img" width="200" height="200">
                        </div>
                        <div class="code_p">将二维码发给企业，企业扫码后绑定</div>
                    </div>
                </div>
            </el-dialog>
        </div>
        <!--企业发送至推文弹窗-->
        <div class="modluDrawer">
            <el-dialog :close-on-click-modal="false" title="企业发送至推文" :visible.sync="twdrawer" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="tw_tip" v-if="tw_tip != ''">
                        <el-alert :title="tw_tip" type="warning" show-icon :closable="false">
                        </el-alert>
                    </div>
                    <div class="wxsettip_small " v-if="multitw == false">企业名称</div>
                    <el-input v-if="multitw == false" placeholder="宿迁鑫潮信息技术有限公司" v-model="tw_data.name"
                        :disabled="true"></el-input>
                    <div class="wxsettip_small">标签</div>
                    <el-checkbox v-model="jj">加急</el-checkbox>
                    <el-checkbox v-model="pyq">朋友圈</el-checkbox>
                    <el-checkbox v-model="gzh">公众号</el-checkbox>
                    <div class="wxsettip_small ">备注</div>
                    <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="tw_desc"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="twdrawer = false">取 消</el-button>
                    <el-button type="primary" @click="addTwTask">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--企业会员等级修改-->
        <div class="modluDrawer">
            <el-dialog title="企业会员等级修改" :visible.sync="drawerrating" :with-header="true" append-to-body
                :modal-append-to-body="false" :show-close="true" width="630px">
                <div class="huiyuanDeng">
                    <div class="huiyuanList">
                        <div class="huiyuanTite">
                            <span>会员等级</span>
                        </div>
                        <div class="huiyuanFrom">
                            <el-select v-model="rid" placeholder="请选择" @change="rateChange">
                                <el-option v-for="(item, index) in ratingarr" :key="index" :label="item" :value="index">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                    <div class="huiyuanList">
                        <div class="huiyuanTite">
                            <span>账户{{ pricename }}</span>
                        </div>
                        <div class="huiyuanFrom">
                            <el-input v-model="integral" placeholder="请输入内容" @input="inputIntNumber($event, 'integral')"></el-input>
                        </div>
                    </div>
                    <div class="huiyuanList">
                        <div class="huiyuanTite">
                            <span>会员到期时间</span>
                        </div>
                        <div class="huiyuanFrom">
                            <el-date-picker v-model="vip_etime" type="date" placeholder="不限" :readonly="!vip_etime">
                            </el-date-picker>
                        </div>
                    </div>
                    <div class="huiyuanList">
                        <div class="huiyuanTite">
                            <span>企业推广</span>
                        </div>
                        <div class="huiyuanFrom">
                            <el-switch v-model="hotjob" active-text="设为名企">
                            </el-switch>
                        </div>
                    </div>
					<div class="huiyuanList">
					    <div class="huiyuanTite">
					        <span>最长有效期</span>
					    </div>
					    <div class="huiyuanFrom">
					        <el-date-picker v-model="max_time" type="date" value-format="yyyy-MM-dd" placeholder="不限" :readonly="!vip_etime && !max_time">
					        </el-date-picker>
					    </div>
					</div>
					<div class="huiyuanList">
					    <div class="huiyuanTite">
					        <span>可暂停次数</span>
					    </div>
					    <div class="huiyuanFrom">
					        <el-input v-model="suspend_num" placeholder="请输入内容" @input="inputIntNumber($event, 'suspend_num')"></el-input>
					    </div>
					</div>
                    <div class="huiyuanList">
                        <div class="huiyuanTite">
                            <span>上架职位数</span>
                        </div>
                        <div class="huiyuanFrom">
                            <el-input v-model="job_num" placeholder="请输入内容" @input="inputIntNumber($event, 'job_num')"></el-input>
                        </div>
                    </div>
                    <div class="huiyuanList">
                        <div class="huiyuanTite">
                            <span>刷新职位数</span>
                        </div>
                        <div class="huiyuanFrom">
                            <el-input v-model="breakjob_num" placeholder="请输入内容" @input="inputIntNumber($event, 'breakjob_num')"></el-input>
                        </div>
                    </div>
                    <div class="huiyuanList">
                        <div class="huiyuanTite">
                            <span>剩余下载数</span>
                        </div>
                        <div class="huiyuanFrom">
                            <el-input v-model="down_resume" placeholder="请输入内容" @input="inputIntNumber($event, 'down_resume')"></el-input>
                        </div>
                    </div>
                    <div class="huiyuanList">
                        <div class="huiyuanTite">
                            <span>邀请面试</span>
                        </div>
                        <div class="huiyuanFrom">
                            <el-input v-model="invite_resume" placeholder="请输入内容" @input="inputIntNumber($event, 'invite_resume')"></el-input>
                        </div>
                    </div>
                    <div class="huiyuanList">
                        <div class="huiyuanTite">
                            <span>招聘会报名次数</span>
                        </div>
                        <div class="huiyuanFrom">
                            <el-input v-model="zph_num" placeholder="请输入内容" @input="inputIntNumber($event, 'zph_num')"></el-input>
                        </div>
                    </div>
                    <div class="huiyuanList">
                        <div class="huiyuanTite">
                            <span>置顶天数</span>
                        </div>
                        <div class="huiyuanFrom">
                            <el-input v-model="top_num" placeholder="请输入内容" @input="inputIntNumber($event, 'top_num')"></el-input>
                        </div>
                    </div>
                    <div class="huiyuanList">
                        <div class="huiyuanTite">
                            <span>紧急天数</span>
                        </div>
                        <div class="huiyuanFrom">
                            <el-input v-model="urgent_num" placeholder="请输入内容" @input="inputIntNumber($event, 'urgent_num')"></el-input>
                        </div>
                    </div>
                    <div class="huiyuanList">
                        <div class="huiyuanTite">
                            <span>推荐天数</span>
                        </div>
                        <div class="huiyuanFrom">
                            <el-input v-model="rec_num" placeholder="请输入内容" @input="inputIntNumber($event, 'rec_num')"></el-input>
                        </div>
                    </div>
                    
                    
                </div>
                <span slot="footer" class="dialog-footer">
                    <div v-if="curr_rstatus == '4'"
                        style="font-size: 13px;color: red;text-align: center; border-right: 0px;">当前客户已设置暂停招聘，请解除暂停后调整</div>
                    <div v-else>
                        <el-button @click="drawerrating = false">取 消</el-button>
                        <el-button type="primary" @click="ratingSubmit" :loading="saveLoading">确 定</el-button>
                    </div>
                </span>
            </el-dialog>
        </div>
        <!--企业logo-->
        <div class="modluDrawer">
            <el-dialog title="企业LOGO" :visible.sync="drawerlogo" :with-header="true" append-to-body
                :modal-append-to-body="false" :show-close="true" width="530px">
                <el-tabs v-model="logoActiveName" @tab-click="handleLogoTabClick">
                    <el-tab-pane label="智能生成logo" name="autologo">
                        <div>
                            <div class="tw_tip" v-if="tw_tip != ''">
                                <el-alert :title="tw_tip" type="warning" show-icon :closable="false">
                                </el-alert>
                            </div>
                            <div class="wxsettip_small ">文字说明</div>
                            <el-input placeholder="在此输入名称最多四个汉字" v-model="logocname"></el-input>
                            <div class="wxsettip_small">背景模板</div>
                            <ul style="display: flex;">
                                <li style="margin-right: 10px" v-for="(item, index) in hbBgA" :key="index"
                                    @click="logobg = index + 1">
                                    <img :class="logobg == index + 1 ? 'logo_box_img_cur' : 'logo_box_img'" :src="item"
                                        width="40" height="40" />
                                </li>
                            </ul>
                             <div style="padding:18px 0;">
							<el-alert
							    title="可点击颜色选择背景模板"
							    type="info"
							    show-icon  :closable="false">
							  </el-alert>
							  </div>
                        </div>
                        <div
                            style="display: flex;align-content: center;justify-content: center;text-align: center;padding-bottom: 18px;">
                            <el-button type="primary"  @click="makeLogoHb" style="width:200px;">{{ logobt }}</el-button>
                            <el-button type="text"  icon="el-icon-thumb"  @click="previewLogoHb">预览LOGO</el-button>
                        </div>
                    </el-tab-pane>
                    <el-tab-pane label="上传logo图片" name="uplogo">
                        <div class="center">
                            <div>
                                <el-upload class="avatar-uploader" :action="upurl" :show-file-list="false" :data="logodata"
                                    :on-success="handleAvatarSuccess" :accept="pic_accept">
                                    <img v-if="uplogopic" :src="uplogopic" class="logoavatar">
                                    <i v-else class="el-icon-plus logoavatar-uploader-icon"></i>
                                </el-upload>
								<div style="padding:18px 0;">
								<el-alert
								    title="直接上传企业logo图片即可,正方形效果最佳"
								    type="info"
								    show-icon  :closable="false">
								  </el-alert>
								  </div>

                            </div>
                        </div>
                    </el-tab-pane>
                </el-tabs>
            </el-dialog>
        </div>
        <!--企业logo预览-->
        <div class="modluDrawer">
            <el-dialog title="企业LOGO预览" :visible.sync="drawerlogopreview" :with-header="true" append-to-body
                :modal-append-to-body="false" :show-close="true" width="260px">
                <div class="center">
                    <el-image style="width: 200px; height: 200px" :src="logopreview" fit="fill"></el-image>
                </div>
            </el-dialog>
        </div>
        <!--企业取消暂停弹窗-->
        <div class="modluDrawer">
            <el-dialog title="企业取消暂停" :visible.sync="drawerqyzt" :with-header="true" append-to-body
                :modal-append-to-body="false" :show-close="true" width="350px">
                <div>已暂停{{ ztdays ? ztdays : 0 }}天，是否为客户延续会员有效期？</div>
                <span slot="footer" class="dialog-footer">
                    <el-button size="mini" type="primary" @click="setupcom(1)">是</el-button>
                    <el-button size="mini" @click="setupcom(0)">否</el-button>
                    <el-button size="mini" @click="drawerqyzt = false">取 消</el-button>
                </span>
            </el-dialog>
        </div>
        <!--批量分配站点弹窗-->
        <div class="modluDrawer">
            <el-dialog title="批量选择分站" :visible.sync="drawerfpzdmulti" append-to-body width="450px">
                <div>
                    <div class="wxsettip_small ">切换站点</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="comdid" size="small" slot="prepend" placeholder="使用范围" filterable>
                            <el-option v-for="(item, index) in dnameArr" :key="index" :label="item" :value="index">
                            </el-option>
                        </el-select>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawerfpzdmulti = false">取 消</el-button>
                    <el-button type="primary" @click="fpzdSubmit(2)" :loading="saveLoading">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--分配站点弹窗-->
        <div class="modluDrawer">
            <el-dialog title="分配站点" :visible.sync="drawerfpzd" append-to-body width="450px">
                <div class="wxsettip_small ">用户名</div>
                <el-input placeholder="企业用户" v-model="curr_com.name" :disabled="true"></el-input>
                <div>
                    <div class="wxsettip_small ">切换站点</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="comdid" size="small" slot="prepend" placeholder="使用范围" filterable>
                            <el-option v-for="(item, index) in dnameArr" :key="index" :label="item" :value="index">
                            </el-option>
                        </el-select>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawerfpzd = false">取 消</el-button>
                    <el-button type="primary" @click="fpzdSubmit(1)" :loading="saveLoading">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--设为名企弹窗-->
        <div class="modluDrawer" v-if="hotcom">
            <el-drawer title="名企招聘" :visible.sync="drawermq" :modal-append-to-body="false" append-to-body :wrapper-closable="false" size="600px">
                <addhotjob :hotinfo="hotcom" :hascom="true" :cindex="comindex"></addhotjob>
            </el-drawer>
        </div>
        <!--生成海报弹窗-->
        <div class="modluDrawer">
            <el-drawer :title="'生成海报'" :visible.sync="drawerhb" :modal-append-to-body="false" append-to-body
                :show-close="true" :with-header="true" size="95%">
                <div class="waixunHaib">
                    <ul>
                        <li class="" v-for="(item, index) in hbarr" :key="index">
                            <div class="hb_listbox">
                                <div class="poster_pic"><img :src="item.pic_n"></div>
                                <div class="hb_listbox_name" style="background:#fff;">
                                    <div class="hb_cz">
                                        <el-button @click="showHb(item.style)" size="mini">预览</el-button>
                                        <el-button @click="downHb(item.style)" size="mini">下载</el-button>
                                        <!-- <a href="javascript:;" @click="showHb(item.style)">预览</a>
                                        <a href="javascript:;" @click="downHb(item.style)">下载</a> -->
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
            <el-dialog title="海报预览" :visible.sync="showhb" :with-header="true" append-to-body :show-close="true" width="300px">
                <div class="code_img" style="display:flex;justify-content: center;margin-bottom: 20px;">
                    <img :src="hburl" :key="hbkey" width="260">
                </div>
            </el-dialog>
        </div>
        <!--企业模板弹窗-->
        <div class="modluDrawer">
            <el-drawer :title="'企业模板'" :visible.sync="drawercommb" :modal-append-to-body="false" append-to-body
                :show-close="true" :with-header="true" size="95%">
                <div class="companyMobans">
                    <ul>
                        <li class="" v-for="(item, index) in comtpllist" :key="index">
                            <div class="hb_listbox">
                                <div class="poster_pic"><img :src="item.pic_n"></div>
                                <div class="hb_listbox_name" style="background:#fff;">
                                    <div class="hb_cz">
                                        <div class="namneyulan">
                                            <span class="bmanmic">模板名称：{{ item.name }}</span>
                                            <el-link target="_blank" :href="item.preview_url">[预览]</el-link>
                                        </div>

                                        <div class="shiyonbutn">
                                            <el-button disabled v-if="comtplstatis.comtpl == item.url" size="mini"
                                                type="info">使用中
                                            </el-button>
                                            <el-button size="mini" v-else type="primary" @click="checktpl(item.id)">使用
                                            </el-button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </li>
                    </ul>
                </div>
            </el-drawer>
        </div>
        <!--发送短信弹窗-->
        <div class="modluDrawer">
            <el-dialog title="发送短信" :visible.sync="drawersendmsg" append-to-body width="450px">
                <div class="wxsettip_small">短信内容</div>
                <el-input type="textarea" maxlength="200" placeholder="短信内容" v-model="msgcontent"></el-input>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawersendmsg = false">取 消</el-button>
                    <el-button type="primary" @click="dosend">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--发送邮件弹窗-->
        <div class="modluDrawer">
            <el-dialog title="发送邮件" :visible.sync="drawersendmail" append-to-body width="450px">
                <div class="wxsettip_small">邮件标题</div>
                <el-input maxlength="200" placeholder="邮件标题" v-model="mailtit"></el-input>
                <div class="wxsettip_small">邮件内容</div>
                <el-input type="textarea" rows="5" maxlength="200" placeholder="邮件内容" v-model="mailcontent"></el-input>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawersendmail = false">取 消</el-button>
                    <el-button type="primary" @click="dosendmail">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--绑定套餐-->
        <div class="modluDrawer">
            <el-dialog title="绑定套餐" :visible.sync="dialogPackage" append-to-body width="650px">
                <div class="tck_setname">
                    <el-checkbox-group v-model="ruleFormPackage.package">
                        <el-checkbox :label="ratingkey" v-for="(ratingItem, ratingkey) in ratingarr" :key="ratingkey">{{
                            ratingItem }}</el-checkbox>
                    </el-checkbox-group>
                </div>
                <div>
                    <el-alert title="绑定套餐后，企业只能看见绑定的会员套餐，其他套餐不可见" :closable="false" type="info" show-icon></el-alert>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogPackage = false">取 消</el-button>
                    <el-button type="primary" @click="submitPackage" :disabled="saveLoading">确 认</el-button>
                </span>
            </el-dialog>
        </div>
        <!--删除弹窗-->
        <div class="modluDrawer">
            <el-dialog title="删除企业数据" :visible.sync="dialogDel" :with-header="true" append-to-body :show-close="true"
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
        <!--职位推广弹窗-->
        <div class="modluDrawer">
            <el-dialog :title="jobtgtit" :visible.sync="jobtgdrawer" :with-header="true" append-to-body :show-close="true"
                width="400px">
                <div class="wxsettip_small" v-if="jobtgtype == 1">置顶天数</div>
                <div class="wxsettip_small" v-else-if="jobtgtype == 2">推荐天数</div>
                <div class="wxsettip_small" v-else-if="jobtgtype == 3">紧急天数</div>
                <el-input type="number" placeholder="请输入天数" v-model="jobtgdays">
                    <template slot="append">天</template>
                </el-input>
                <div class="wxsettip_small" v-if="jobtgetime != ''">当前结束日期</div>
                <el-input v-if="jobtgetime != ''" v-model="jobtgetime" disabled>
                </el-input>
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
                    <el-button type="primary" @click="jobTgSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--新增企业-->
        <div class="modluDrawer">
            <el-drawer title="新增企业" :visible.sync="comadddrawer" :modal-append-to-body="false" append-to-body :wrapper-closable="false" size="65%">
                <company_add ref="comadd" :rates="ratingarr" :pricename="pricename"></company_add>
            </el-drawer>
        </div>
        <!--导出字段弹窗-->
        <div class="modluDrawer">
            <el-dialog title="选择导出字段" :visible.sync="exportdrawer" :with-header="true" append-to-body :show-close="true"
                width="740px">
                <div style="">
                    <el-checkbox-group v-model="checkedCols" @change="handleColCheckedChange">
                        <el-checkbox style="width:110px;margin-bottom: 5px;margin-left:0" size="small" border
                            v-for="(item, index) in cols" :key="index" :label="item.value">{{ item.label }}</el-checkbox>
                        <el-checkbox style="width:110px;margin-left:0" size="small" border :indeterminate="isIndeterminate"
                            v-model="colCheckAll" @change="handleColCheckAllChange">全选</el-checkbox>
                    </el-checkbox-group>
                </div>
                <div class="wxsettip_small">导出数量</div>
                <el-input type="number" placeholder="请输入导出数量" v-model="exp_num">
                </el-input>
                <el-alert style="margin-top: 10px;" title="数字太大会导致运行缓慢，请慎重填写。" type="warning" show-icon :closable="false">
                </el-alert>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="exportdrawer = false">取 消</el-button>
                    <el-button type="primary" @click="submitExport">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--分配顾问弹窗-->
        <div class="modluDrawer">
            <el-dialog :title="fpgwmulti ? '批量选择顾问' : '分配顾问'" :visible.sync="drawerfpgw" append-to-body width="450px">
                <div v-if="fpgwmulti == false">
                    <div class="wxsettip_small ">企业用户名</div>
                    <div class="wxsettip_Sealect">
                        <el-input :value="fpgwcname" disabled></el-input>
                    </div>
                </div>
                <div>
                    <div class="wxsettip_small ">选择顾问</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="gwid" size="small" slot="prepend" placeholder="选择顾问" filterable>
                            <el-option v-for="(item, index) in gwArr" :key="index"
                                :label="item.name ? item.name : item.username" :value="item.uid">
                            </el-option>
                        </el-select>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawerfpgw = false">取 消</el-button>
                    <el-button type="primary" @click="fpgwSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--批量认证弹窗-->
        <div class="modluDrawer">
            <el-dialog title="批量认证" :visible.sync="drawerrzmulti" append-to-body width="450px">
                <div>
                    <div class="wxsettip_small ">认证类型</div>
                    <div class="wxsettip_Sealect">
                        <el-checkbox v-model="email_rz">邮箱</el-checkbox>
                        <el-checkbox v-model="mobile_rz">手机</el-checkbox>
                        <el-checkbox v-model="yyzz_rz">企业资质</el-checkbox>
                    </div>
                </div>
                <div>
                    <div class="wxsettip_small ">认证操作</div>
                    <div class="wxsettip_Sealect">
                        <el-radio-group v-model="plstatus">
                            <el-radio :label="0">待认证</el-radio>
                            <el-radio :label="1">已认证</el-radio>
                        </el-radio-group>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawerrzmulti = false">取 消</el-button>
                    <el-button type="primary" @click="multirzSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--新增职位提示-->
        <el-drawer title="新增职位" :visible.sync="drawerAddJob" append-to-body :wrapper-closable="false" size="60%">
            <addjob ref="jobadd" style="margin-right:10px;" :comid="curr_comid"></addjob>
        </el-drawer>
        <!--账户信息弹窗-->
        <div class="modluDrawer">
            <el-dialog title="账户信息" :visible.sync="dialogAccount" :with-header="true" :modal-append-to-body="false" :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">用户名</div>
                    <el-input placeholder="请输入用户名" v-model="ruleFormAccount.username"></el-input>
                    <div class="wxsettip_small ">登录密码</div>
                    <el-input @mousedown.native="pwdMousedown" @input="pwdchange" @focus="readonlyCtl(false)" @blur="readonlyCtl(true)" :readonly="pwdreadonly" placeholder="请输入登录密码" v-model="ruleFormAccount.password"></el-input>
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
        <!--实地核验-->
        <div class="modluDrawer" v-if="factshow">
            <el-dialog title="实地核验" :visible.sync="factshow" :with-header="true" append-to-body
                :modal-append-to-body="false" :show-close="true" width="43%">
                <el-upload action="#" :auto-upload="false" multiple :limit="3" list-type="picture-card" :accept="pic_accept"
                    :on-change="factChange" :file-list="factfileList" :on-exceed="factexceedFun"
                    :on-preview="factPreview"
                    :on-remove="factRemove"
                    >
                    <i slot="default" class="el-icon-plus"></i>
                </el-upload>
                
                <div style="font-size: 12px;color: #8c939d; margin-top:10px;">
                    <i class="el-icon-warning-outline"></i>
                    <span>只能上传jpg/png/jpeg/gif文件</span>
                </div>
                
                <div class="sdhy_tg">
                    <el-checkbox v-model="fact_status">通过实地核验</el-checkbox>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="factshow = false">取 消</el-button>
                    <el-button type="primary" @click="saveFact" :disabled="submitLoading">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <el-dialog :visible.sync="factImageVisible" :with-header="true" append-to-body :modal-append-to-body="false" :show-close="true">
            <img width="100%" :src="factImageUrl" alt="">
        </el-dialog>
    </div>
</template>
<script>
var setval = null;
let editor = null,
    toolbar = null,editorInterval = null;
const { createEditor, createToolbar } = window.wangEditor;

/**
 * @desc 邮箱格式验证
 */
function check_email(strEmail) {
    var emailReg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
    if (emailReg.test(strEmail))
        return true;
    else
        return false;
}

/**
 * @desc 手机号码验证
 */
function isjsMobile(obj) {
    var reg = /^[1][3456789]\d{9}$/;
    if (obj.length != 11) return false;
    else if (!reg.test(obj)) return false;
    else if (isNaN(obj)) return false;
    else return true;
}

/**
 * @desc 电话验证
 */
function isjsTell(str) {
    //var result = str.match(/^((0\d{2,3})-)(\d{7,8})(-(\d{3,}))?$/);
    var result = str.match(/^[0-9-]+?$/);
    if (result == null) return false;
    return true;
}
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
module.exports = {
    props: {
        status: { type: String, default: '' },
        jump_params: {type: Object, default: ()=>{
            return {
                reg_days: 0,
                reg_time:'',
            }
        }}
    },
    data: function () {
        return {
                pwdreadonly: true,
            mouseFlag: false,
            mouseOffset: 0,
            pic_accept: localStorage.getItem("pic_accept"),
            loading: false,
			dataText: '数据加载中',
            drawerAddJob: false,
            multitw: false,
            input3: '',
            email_rz: '',
            mobile_rz: '',
            yyzz_rz: '',
            plstatus: '',
            drawerrzmulti: false,
            gwArr: [],
            fpgwcname: '',
            drawerfpgw: false,
            fpgwmulti: false,
            gwid: '',
            drawerfpzdmulti: false,
            exp_num: '',
            isIndeterminate: true,
            checkedCols: [],
            colCheckAll: false,
            exportdrawer: false,
            comadddrawer: false,
            searchlist: null,
            search_params: {
                type: '1',
                keyword: '',
                rating: '',
                rec: '',
                time: '',
                status: this.status,
                source: '',
                gw: '',
                    time_type: 'lotime',
                    times: [],
                has_job: '',
				city_class:'',
                reg_days: 0,
                reg_time: [],
                login_days: 0,
                login_time: [],
                fact_status:''
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
            jumpClass: '',

            checkedAll: false,
            selectedItem: [],
            tableData: [],
            currentPage: 1,
            perPage: 0,
            pageSizes: [],
            total: 0,
            today: 0,
            source: [],
            todayetime: '',
            curr_com: { uid: '' },
            r_status: '',
            member_status: '',
            islock: false,
            lockdesc: '',
            statusbody: '',
            snum: '0',
            tw_data: { name: '', uid: '' },
            jj: false,
            pyq: false,
            gzh: false,
            tw_tip: '',
            tw_desc: '',
            sj_mobile: '',
            sj_status: '',
            sj_uid: '',
            yx_email: '',
            yx_status: '',
            yx_uid: '',
            com_social_credit: '',
            com_cert_owner: '',
            com_cert_wt: '',
            com_cert_other: '',
            com_free_status: '',
            wxrztc: false,
            code_img: '',
            yy_picurl: '',
            yy_owner_picurl: '',
            yy_wt_picurl: '',
            yy_other_picurl: '',
            yy_uid: '',
            yy_status: '',
            yy_scredit: '',
            yy_comname: '',
            yySrcList: [],
            yy_job_status: false,
            yy_sbody: '',
            allNum: 0,
            status1Num: 0,
            status2Num: 0,
            status3Num: 0,
            drawerrating: false,
            pricename: '',
            chatname: '',
            ratingarr: [],
            rid: '',
            integral: '',
            vip_etime: '',
            oldetime: '',
            hotjob: '',
			max_time:'',
			suspend_num:'',
            job_num: '',
            breakjob_num: '',
            down_resume: '',
            invite_resume: '',
            zph_num: '',
            top_num: '',
            urgent_num: '',
            rec_num: '',
            chat_num: '',
            
            curr_uid: '',
            curr_rstatus: '',
            drawerlogo: false,
            logocname: '',
            logobg: '',
            logobt: '',
            logopreview: '',
            drawerlogopreview: false,
            hbBgA: [],
            upurl: baseUrl + 'm=index&c=layui_upload',
            uplogopic: '',
            logodata: null,
            logoActiveName: 'autologo',
            typeArr: [{
                value: 1,
                label: "用户名"
            }, {
                value: 3,
                label: "用户ID"
            }],
            time: [{
                value: 1,
                label: "今天"
            }, {
                value: 3,
                label: "最近三天"
            }, {
                value: 7,
                label: "最近七天"
            }, {
                value: 15,
                label: "最近半月"
            }, {
                value: 30,
                label: "最近一个月"
            }],
            ztdays: '',
            drawerqyzt: false,
            dnameArr: {},
            comdid: '0',
            drawerfpzd: false,
            cache: [],
            hascache: false,
            sel_city: [],
            moneytypeoptions: [
                { value: '1', label: '人民币' },
                { value: '2', label: '美元' },
            ],
            comqcodelist: [],
            checkedwelfare: [],
            drawermq: false,
            hotcom: null,
            mqlogolist: [],
            drawerhb: false,
            hbarr: [],
            basehburl: '',
            hburl: '',
            hbkey: '',
            showhb: false,
            drawercommb: false,
            drawersendmsg: false,
            msgcontent: '',
            drawersendmail: false,
            mailtit: '',
            mailcontent: '',
            comtpllist: [],
            comtplstatis: null,
            curr_editcom: null,
            curr_comid: '',
            dialogDel: false,
            ruleFormDel: {},
            comindex: {},
            jobTableData: [],
            currentJobPage: 1,
            jobPerPage: 0,
            jobPageSizes: [],
            jobTotal: 0,
            curr_time: 0,
            dsh: 0,
            wtg: 0,
            xj: 0,
            drawerEditJob: false,
            curr_job: null,
            job_types: [],
            city_types: [],
            jobcomdatacache: [],
            jobcomclassnamecache: [],
            mychecked: false,
            jobcache: null,
            showJob: false,
            jobCompany: null,
            jobAddressList: [],
            cache_userdata: null,
            cache_userclassname: null,
            cache_com_sexreq: null,
            jobtgtype: '',
            jobtgtit: '',
            jobtgdrawer: false,
            jobtgdays: '',
            jobtgetime: '',
            qxtgchecked: '0',
            tgjid: '',
            jionly: 0,
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
            drawerauditmultiple: false,
            multiStatus: '',
            multiStatusBody: '',
            cols: [
                { label: '企业UID', value: 'uid' },
                { label: '企业名称', value: 'name' },
                { label: '从事行业', value: 'hy' },
                { label: '企业性质', value: 'pr' },
                { label: '会员等级', value: 'rating' },
                { label: '省', value: 'provinceid' },
                { label: '市', value: 'cityid' },
                { label: '规模', value: 'mun' },
                { label: '创办时间', value: 'sdate' },
                { label: '注册资金', value: 'money' },
                { label: '公司地址', value: 'address' },
                { label: '联系人', value: 'linkman' },
                { label: '所属职位', value: 'linkjob' },
                { label: '联系QQ', value: 'linkqq' },
                { label: '固定电话', value: 'linkphone' },
                { label: '联系手机', value: 'linktel' },
                { label: '邮件', value: 'linkmail' },
                { label: '网址', value: 'website' },
                { label: '知名企业', value: 'rec' },
                { label: '更新时间', value: 'lastdate' },
                { label: '会员开始时间', value: 'vip_stime' },
                { label: '会员到期时间', value: 'vip_etime' },
                { label: '当前业务员', value: 'crm_salesman' },
            ],
            islook: false,
            comdrawersh: false,
            qyxqdrawer: false,
            infoDrawer: false,
            twdrawer: false,
            zzrztc: false,
            sjrztc: false,
            yxrztc: false,
            qyrz: false,
            seachbutn: true,
            tableHig: true,
            activeName: 'first',
            inputVisible: false,
            inputValue: '',
            sort_type: '',
            sort_col: '',
            jobid: '',
            edit_loading: false,

            saveLoading: false,

            // 绑定套餐
            dialogPackage: false,
            ruleFormPackage: {},

            emptytext: '暂无数据',

			// 账户信息
			dialogAccount: false,
			ruleFormAccount: {},

            prevPage: 0,
            factshow:false,
            factuploadAction: baseUrl + 'm=user&c=company&a=upfactpic',
            factfileList: [],
            fact_picurl:[],
            fact_delid:[],
            fact_status:false,
            fact_uid:'',
            submitLoading:false,
            factImageVisible:false,
            factImageUrl:'',
			address:'',
			x:'',
			y:'',
			address_v:'',
			mapurl: '',
			mapsecret: '',
        }
    },
    components: {
        'comlog': httpVueLoader('./comlog.vue'),
        'comlog_index': httpVueLoader('./comlog_index.vue'),
        'comlog_useridmsg': httpVueLoader('./comlog_useridmsg.vue'),
        'company_pay': httpVueLoader('./company_pay.vue'),
        'company_order': httpVueLoader('./company_order.vue'),
        'company_tcjl': httpVueLoader('./company_tcjl.vue'),
        'company_add': httpVueLoader('./company_add.vue'),
        'resumedown': httpVueLoader('./resume_down.vue'),
        'addhotjob': httpVueLoader('./addhotjob.vue'),
        'addjob': httpVueLoader('./addjob.vue'),
        'company_job': httpVueLoader('./company_job.vue'),
		'city_class': httpVueLoader('../../../component/city_class.vue'),
    },
    mounted() {
        var that = this
        setTimeout(function () {
            that.getTjNum();
            that.getCacheFun();
        }, 200)
    },
    beforeDestroy() {
        editor = null; 
        editorInterval = null;
    },
    watch: {
        jump_params:{
            handler(val) {
                if (parseInt(val.reg_days) > 0){

                    this.search_params.reg_days = val.reg_days;
                }else if (val.reg_time){

                    this.search_params.reg_time = val.reg_time;
                }
                if (parseInt(val.login_days) > 0){

                    this.search_params.login_days = val.login_days;
                }else if (val.login_time){

                    this.search_params.login_time = val.login_time;
                }
                if (val.search_class){

                    this.searchClass = val.search_class;
                } else {

                    this.search_params.reg_days = '';
                    this.search_params.reg_time = '';
                    this.search_params.login_days = '';
                    this.search_params.login_time = '';
                    this.searchClass = '';
                }
            },
            deep: true,
            immediate: true
        },
		vip_etime:{
            handler(val) {
				if(val===null){
					this.max_time = null;
				}
            },
            immediate: true
        },
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
        this.getList();
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



        inputIntNumber(val, form, key) {
            this.$data[form] = val.replace(/[^0-9]/g,'');
        },
        getParams: function (params = {}, search = false) {
            var that = this;
            for (let i in params) {
                if(typeof that.search_params[i]!='undefined'){
                	that.search_params[i] = params[i];
                }
            }
            if (search) {
                this.search();
            }
        },
        // 新增职位
        addjob: function (id) {
            var that = this
            this.curr_comid = id
            this.drawerAddJob = true
            this.$nextTick(function () {
                that.$refs.jobadd.edit()
            })
        },
        // 批量认证
        multirz: function () {
            if (this.selectedItem.length == 0) {
                message.error('请选择要操作的数据项');
                return;
            }
            this.email_rz = ''
            this.mobile_rz = ''
            this.yyzz_rz = ''
            this.plstatus = ''
            this.drawerrzmulti = true
        },
        // 批量认证保存
        multirzSubmit: function () {
            if (this.selectedItem.length == 0) {
                message.error('请选择要操作的数据项');
                return false
            }
            var that = this,
                params = {
                    uid: this.selectedItem.join(','),
                    plstatus: that.plstatus,
                    batchfirm: 1
                };
            if (this.email_rz) {
                params.comname_email = 'on'
            }
            if (this.mobile_rz) {
                params.comname_moblie = 'on'
            }
            if (this.yyzz_rz) {
                params.comname_yyzz = 'on'
            }
            httpPost('m=user&c=company&a=comcert', params).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList();
                        that.drawerrzmulti = false
                    })
                } else {
                    message.error(res.msg)
                    return false
                }
            }).catch(function (e) {

            })
        },
        // 批量分配顾问
        fpgw: function (type, cuname, crm_uid = '',uid = '') {
            this.gwid = ''
            if (type == 2) {
                if (this.selectedItem.length == 0) {
                    message.error('请选择要操作的数据项');
                    return;
                } else {
                    this.fpgwmulti = true
                    this.fpgwcname = ''
                }
            } else {
                this.fpgwcname = cuname
                this.curr_uid  = uid;
                if (crm_uid != '' && parseInt(crm_uid) > 0) {
                        this.gwid = crm_uid + ''
                }
                this.fpgwmulti = false
            }
            this.drawerfpgw = true
        },
        // 分配顾问保存
        fpgwSubmit: function () {
            var that = this,
                params = {
                    gid: that.gwid
                }
            if (that.fpgwmulti == false) { // 单个操作
                params.comid = that.curr_uid
            } else { // 批量分配
                if (that.selectedItem.length == 0) {
                    message.error('请选择要操作的数据项');
                    return false
                }
                params.comid = that.selectedItem.join(',')
            }
            httpPost('m=user&c=company&a=checkguwen', params).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList();
                        that.drawerfpgw = false
                    })
                } else {
                    message.error(res.msg)
                    return false
                }
            }).catch(function (e) {

            })
        },
        // 批量分配站点
        multiFpzd: function () {
            if (this.selectedItem.length == 0) {
                message.error('请选择要操作的数据项')
                return;
            }
            this.drawerfpzdmulti = true
        },
        submitExport() {
            let that = this
            if (that.checkedCols.length == 0) {
                message.error('请选择要操作的数据项')
                return;
            }
            params = {
                uid: that.selectedItem.join(','),
                type: that.checkedCols,
                limit: that.exp_num
            }
            httpPost('m=user&c=company&a=export_check', params).then(function (response) {
                let res = response.data;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    httpPost('m=user&c=company&a=xls', {}).then(function (response2) {
                        let res2 = response2.data;
                        if (res2.error > 0) {
                            message.error(res2.msg);
                        } else {
                            that.exportdrawer = false;
                            utilFile.downloadFileByByte(res2.data.file, res2.data.file_name);
                        }
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
        // 企业批量审核
        multipleStatus() {
            var that = this
            if (!that.selectedItem.length) {
                message.error('请选择要操作的数据项');
                return false;
            }
            that.drawerauditmultiple = true
        },
        // 企业批量审核保存
        multipleStatusSave() {
            var that = this
            if (!that.selectedItem.length) {
                message.error('请选择要操作的数据项');
                return false;
            }
            that.comShSubmit({
                uid: that.selectedItem.join(','),
                status: that.multiStatus,
                status_body: that.multiStatusBody,
                atype: 1
            });
        },
        addcom: function () {
            var that = this
            that.comadddrawer = true
            that.$nextTick(function () {
                that.$refs.comadd.getinfo()
            })
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
            httpPost(url, params).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getComJobList()
                        that.jobtgdrawer = false
                        that.jobtgdays = ''
                        that.qxtgchecked = false
                    })
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            })
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
        // 职位招聘状态修改
        zpstatuschange: function (val, id) {
            var that = this
            httpPost('m=user&c=company_job&a=checkstate', { id: id, state: val ? 2 : 1 }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.getComJobList()
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        initEditor: function (content=null) {
            var that = this
            clearInterval(editorInterval);
            editorInterval = setInterval(()=>{
                if (editor !== null){
                    clearInterval(editorInterval);
                    if(content!==null){
                        editor.setHtml(content);
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
                    if (!editor) {
                        editor = createEditor({
                            selector: '#editor-container',
                            html: '',
                            config: editorConfig,
                            mode: 'simple'
                        });
                    }
                    if (!toolbar) {
                        toolbar = createToolbar({
                            editor,
                            selector: '#toolbar-container',
                            config: {
                                // excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                                excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'color', 'bgColor', 'insertLink', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                    
                            },
                            mode: 'simple'
                        });
                    }
                }
            },300);
            
        },
        jobedit: function (row) {
            var that = this
            that.jobid = row.id
            that.drawerEditJob = true
            that.$nextTick(function () {
                that.$refs.jobedit.edit()
            })
        },
        // 获取企业招聘岗位
        getComJobList: function () {
            let that = this;
            let params = {
                page: that.currentJobPage,
                pageSize: that.jobPerPage,
                type: 1,
                keyword: that.curr_com.name
            }
            httpPost('m=user&c=company_job&a=index', params).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.jobTableData = res.data.list
                    let dsh = 0,
                        wtg = 0,
                        xj = 0;
                    that.jobTableData.forEach(function (e) {
                        if (e.state == '0') {
                            dsh++
                        } else if (e.state == '3') {
                            wtg++
                        } else if (e.status == '1') {
                            xj++
                        }
                    })
                    that.dsh = dsh
                    that.wtg = wtg
                    that.xj = xj
                    that.jobPerPage = parseInt(res.data.perPage)
                    that.jobPageSizes = res.data.pageSizes
                    that.jobTotal = parseInt(res.data.total)
                    that.curr_rime = res.data.curr_time
                    that.job_types = res.data.job_types
                    that.city_types = res.data.city_types
                    that.jionly = res.data.jionly
                    that.jobcomdatacache = res.data.comdata
                    that.jobcomclassnamecache = res.data.comclass_name
                    that.jobcache = res.data.cache
                }
            }).catch(function (e) {
                console.log(e)
            })
        },

        // 重置密码
        resetpass: function () {
            var that = this;
            let params = {uid: that.curr_com.uid};
            delConfirm(this, params, function(params) {
                httpPost('m=user&c=company&a=reset_companypassword', params).then(function (result) {
                    var res = result.data;
                    if (res.error == 0) {
                        message.success("企业会员：" + that.curr_com.name + "密码已经重置为123456！")
                    } else {
                        message.error('重置失败')
                        return false
                    }
                }).catch(function (e) {

                })

            }, '确定要重置密码？')
        },
        // 发送邮件弹窗
        sendmail: function () {
            if (this.curr_com.linkmail == '') {
                message.error('该企业未填写联系邮箱！')
                return false
            }
            this.drawersendmail = true
        },
        // 邮件发送
        dosendmail: function () {
            var that = this
            if (that.mailtit == '') {
                message.error('邮件标题不能为空！')
                return false
            }
            if (that.mailcontent == '') {
                message.error('邮件内容不能为空！')
                return false
            }
            var params = {
                utype: 5,
                email_title: that.mailtit,
                content: that.mailcontent,
                email_user: this.curr_com.linkmail,
                pagelimit: 50,
                value: 0,
                sendok: 0,
                sendno: 0
            }
            httpPost('m=user&c=admin_member&a=send', params).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.drawersendmail = false
                    })
                } else {
                    message.error(res.msg)
                    return false
                }
            }).catch(function (e) {

            })
        },
        // 发送短信弹窗
        sendmsg: function () {
            if (this.curr_com.linktel == '') {
                message.error('该企业未填写联系手机！')
                return false
            }
            this.drawersendmsg = true
        },
        // 短信发送
        dosend: function () {
            var that = this
            if (that.msgcontent == '') {
                message.error('短信内容不能为空！')
                return false
            }
            var params = {
                utype: 5,
                content: that.msgcontent,
                userarr: this.curr_com.linktel,
                pagelimit: 50,
                value: 0,
                sendok: 0,
                sendno: 0
            }
            httpPost('m=user&c=admin_member&a=msgsave', params).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.drawersendmsg = false
                    })
                } else {
                    message.error(res.msg)
                    return false
                }
            }).catch(function (e) {

            })
        },
        // 使用模板
        checktpl: function (id) {
            var that = this
            let params = {id: id, comid: that.curr_com.uid};
            delConfirm(this, params, function(params) {
                httpPost('m=user&c=company&a=msettpl', params).then(function (result) {
                    var res = result.data;
                    if (res.error == 0) {
                        message.success(res.msg, function () {
                            that.drawercommb = false
                        })
                    } else {
                        message.error(res.msg)
                        return false
                    }
                }).catch(function (e) {

                })

            }, '确定使用该模板？')
        },
        // 企业模板
        commb: function () {
            var that = this
            httpPost('m=user&c=company&a=mcomtpl', { comid: that.curr_com.uid }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    that.comtpllist = res.data.list
                    that.comtplstatis = res.data.statis
                    that.drawercommb = true
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
            that.hburl = that.basehburl + '&uid=' + that.curr_com.uid + '&style=' + style
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
            this.hburl = this.basehburl + '&uid=' + this.curr_com.uid + '&style=' + style
            this.hbkey = Math.random()
            this.showhb = true
        },
        // 生成海报
        createhb: function () {
            var that = this
            httpPost('m=user&c=company&a=mwhb', {}).then(function (result) {
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
        // 取消名企
        qxmq: function () {
            var that = this

            let params = {del: that.curr_com.uid};
            delConfirm(this, params, function(params) {
                httpPost('m=user&c=hotjob&a=del', params).then(function (result) {
                    var res = result.data;
                    if (res.error == 0) {
                        message.success(res.msg, function () {
                            that.cominfo(that.comindex, that.curr_com.uid)
                            that.getList();
                        })
                    } else {
                        message.error(res.msg)
                        return false
                    }
                }).catch(function (e) {

                })

            }, '确定要取消该名企？')
        },
        // 设为名企
        setmq: function () {
            var that = this
            if (that.curr_com.name == '') {
                message.error('请先完善企业资料！');
                return false;
            }
            httpPost('m=user&c=hotjob&a=hotjobinfo', { uid: that.curr_com.uid }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.hotcom = res.data
                    that.drawermq = true
                } else {
                    message.error(res.msg);
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        welfareInputConfirm(type) {
            let inputValue = this.inputValue;
            if (inputValue) {
                if (type == 1) { // 企业编辑福利修改
                    this.curr_editcom.all_welfare.push(inputValue);
                } else { // 职位编辑福利修改
                    this.curr_job.all_welfare.push(inputValue);
                }
                this.checkedwelfare.push(inputValue)
            }
            this.inputVisible = false;
            this.inputValue = '';
        },
        // 企业基本信息保存
        comeditsave: function () {
            var that = this
            if (that.curr_editcom.linkmail && check_email(that.curr_editcom.linkmail) == false) {
                message.error('邮箱格式错误！');
                return false;
            }
            if (that.curr_editcom.linktel && isjsMobile(that.curr_editcom.linktel) == false) {
                message.error('手机格式错误！');
                return false;
            }
            if (that.curr_editcom.linkphone && isjsTell(that.curr_editcom.linkphone) == false) {
                message.error('固话格式错误！');
                return false;
            }
            var linkman = that.curr_editcom.linkman;
            if (linkman) {
                linkman = linkman.replace(/[-_ ]/g, ''); // 去掉空格
                if (!linkman) {
                    return;
                }
                var test = linkman.replace(/[0-9]/g, '');
                if (!test) {
                    message.error('联系人不支持全数字');
                    return false;
                } else {
                    if (/\d/.test(linkman)) {
                        if (linkman.length > 8) {
                            message.error('联系人填写字数不能超过8个');
                            return false;
                        }
                    }
                }
            }
            var params = new FormData();
            if (that.sel_city[0]) {
                that.curr_editcom.provinceid = that.sel_city[0]
            }
            if (that.sel_city[1]) {
                that.curr_editcom.cityid = that.sel_city[1]
            }
            if (that.sel_city[2]) {
                that.curr_editcom.three_cityid = that.sel_city[2]
            }
            that.curr_editcom.checked_welfare = that.checkedwelfare.join(',')
            that.curr_editcom.content = editor.getHtml()
			if(that.x){
				that.curr_editcom.x = that.x
			}
			if(that.y){
				that.curr_editcom.y = that.y
			}
            for(let i in that.curr_editcom){
                params.append(i,that.curr_editcom[i])
            }
            if (this.comqcodelist.length) {
                params.append('comqcode[]', this.comqcodelist[0])
            }
            that.edit_loading = true;
            httpPost('m=user&c=company&a=comeditsave', params).then(function (result) {
                that.edit_loading = false;
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.infoDrawer = false
                        that.cominfo(that.comindex, that.curr_editcom.uid)
                        that.getList();
                    })
                } else {
                    message.error(res.msg);
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        comqcodeChange(file) {
            var tmp = deepClone(this.curr_editcom)
            // 预览文件处理
            tmp.comqcode = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.comqcodelist[0] = file.raw;
            this.curr_editcom = tmp
        },
        citychange: function (data) {
            this.sel_city = data
        },
        editcom: function () {
            var that = this
            httpPost('m=user&c=company&a=edit', { id: that.curr_uid }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    that.curr_editcom = res.data.row
                    if (res.data.row.arraywelfare) {
                        that.checkedwelfare = res.data.row.arraywelfare
                    } else {
                        that.checkedwelfare = []
                    }
                    that.sel_city = []
					that.curr_editcom.hy = parseInt(that.curr_editcom.hy)>0?that.curr_editcom.hy:'';
					that.curr_editcom.pr = parseInt(that.curr_editcom.pr)>0?that.curr_editcom.pr:'';
					that.curr_editcom.mun = parseInt(that.curr_editcom.mun)>0?that.curr_editcom.mun:'';
                    if (that.curr_editcom.provinceid > 0) {
                        that.sel_city.push(that.curr_editcom.provinceid)
                    }
                    if (that.curr_editcom.cityid > 0) {
                        that.sel_city.push(that.curr_editcom.cityid)
                    }
                    if (that.curr_editcom.three_cityid > 0) {
                        that.sel_city.push(that.curr_editcom.three_cityid)
                    }
                    that.cache = res.data.cache
					that.x = that.curr_editcom.x
					that.y = that.curr_editcom.y
                    that.initEditor(that.curr_editcom.content);
                    that.hascache = true
                    that.infoDrawer = true
					that.$nextTick(_ => {
						var map_url = that.mapurl+"&callback=onAMapCallback";
						that.writeJs(map_url, that.mapsecret).then(value => {
							that.openMap();
						});
					});
                } else {
                    message.error('获取企业信息失败')
                    return false
                }
            }).catch(function (e) {

            })
        },
		writeJs: function(url, secret) {
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
		getMap:function (){
			var map = new AMap.Map('com_conrtainer', {
			  zoom: 15,
			  center: [this.x, this.y]
			});
			var marker = new AMap.Marker({
				position: new AMap.LngLat(this.x, this.y)
			});
			map.add(marker);
			return map;
		},
		openMap:function (){
			var that = this;
			var data=get_map_config();
			if(data && data.indexOf('map_x')>-1){
				var config=eval('('+data+')');
				var rating,map_control_type,map_control_anchor;
				if (!that.x && !that.y) {
					that.x = config.map_x;
					that.y = config.map_y;
				}
				var map = that.getMap();
				map.on("click",function(e){
					var lngLat = e.lnglat;
					that.x = lngLat.lng
					that.y = lngLat.lat
					map.clearMap();
					var marker = new AMap.Marker({
						position: new AMap.LngLat(lngLat.lng, lngLat.lat)
					});
					map.add(marker);
				});
			}
		},
		addressKeyup:function(queryString, cb){
			
			this.localsearch(queryString, cb)
		},
		localsearch:function(city='',cb){
			var that = this;
			var address = city;
			
			if (this.mapurl && address.length > 3) {
				var map = this.getMap();
				map.clearMap();
				
				var params = {address: address};
				httpPost('m=common&c=cache&a=poi', params).then(function (result) {
					var res = result.data
					if (res.error == 0) {
						var e = res.data;
						if(e.status == '1' && e.tips.length > 0){
							that.poiSearchArr = e.tips;
							setTimeout(function(){
								map.clearMap();
								
							},200);
						}
						cb(that.poiSearchArr);
						that.address_v = that.curr_editcom.address;
					}
				}).catch(function (e) {
					console.log(e)
				});
				
			}
				
		},
		poiSearchClick:function(item){
			
			var that = this;
			that.curr_editcom.address = that.address_v;
			var location = item.location;
			var c = location.split(',');
			
			this.x = c[0];
			this.y = c[1];
			var map = that.getMap();
			// 设置marker
			var lngLat = new AMap.LngLat(c[0],c[1]);
			map.setZoomAndCenter(17,lngLat);
			var marker = new AMap.Marker({
				position: lngLat
			});
			map.add(marker);
			// 地图监听点击事件
			map.on("click",function(e){
				var lngLat = e.lnglat;
				this.x = lngLat.lng;
				this.y = lngLat.lat;
				map.clearMap();
				var marker = new AMap.Marker({
					position: new AMap.LngLat(lngLat.lng, lngLat.lat)
				});
				map.add(marker);
			});
			this.poiSearchArr = [];
		},
        // 分配站点保存
        fpzdSubmit: function (type) {
            var that = this,
                params = {
                    did: that.comdid
                }
            if (type == 1) { // 单个操作
                params.uid = that.curr_uid
            } else if (type == 2) { // 批量分配站点
                if (that.selectedItem.length == 0) {
                    message.error('请选择要批量操作的企业')
                    return false
                }
                params.uid = that.selectedItem.join(',')
            }
            that.saveLoading = true;
            httpPost('m=user&c=company&a=checksitedid', params).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        if (type == 1) {
                            that.curr_com.did = that.comdid
                            that.drawerfpzd = false
                        } else if (type == 2) {
                            that.drawerfpzdmulti = false
                        }
                        that.getList();
                    })
                } else {
                    message.error('站点分配失败')
                    return false
                }
            }).catch(function (e) {

            }).finally(function() {
                setTimeout(function() {
                    that.saveLoading = false;
                }, 2000);
            });
        },
        // 企业暂停
        comzt: function (rstatus) {
            var that = this
            if (rstatus == '2') {
                message.error('该企业已被锁定，请先解锁后再暂停！')
                return false
            }

            let params = {uid: that.curr_uid};
            delConfirm(this, params, function(params) {
                httpPost('m=user&c=company&a=suspend', params).then(function (result) {
                    var res = result.data;
                    if (res.error == 0) {
                        message.success('暂停成功！', function () {
                            that.curr_com.r_status = 4
                            that.getList();
                        })
                    } else {
                        message.error(res.msg)
                        return false
                    }
                }).catch(function (e) {

                })
            }, '确定要暂停该企业吗？')
        },
        // 解除暂停
        comunzt: function (ztdays) {
            this.ztdays = ztdays
            this.drawerqyzt = true
        },
        //解除暂停
        setupcom: function (addzttime) {
            var that = this
            httpPost('m=user&c=company&a=setupcom', {
                uid: that.curr_uid,
                addzttime: addzttime
            }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    message.success('企业解除暂停成功！', function () {
                        that.curr_com.r_status = 1
                        that.drawerqyzt = false
                        that.getList();
                    })
                } else {
                    message.error('操作失败请重试！')
                    return false
                }
            }).catch(function (e) {

            })
        },
        // 获取企业信息
        cominfo: function (index, uid) {
            var that = this
            that.comindex = index
            that.curr_uid = uid
            httpPost('m=user&c=company&a=getinfo', { comid: that.curr_uid }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    that.curr_com = res.data
                    that.comdid = '' + that.curr_com.did;
                    that.activeName = 'first'
                    that.qyxqdrawer = true
                } else {
                    message.error('获取企业信息失败')
                    return false
                }
            }).catch(function (e) {

            })
        },
		// 账户信息
		openAccount() {
		    let member = this.curr_com;
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
		    httpPost('m=user&c=company&a=saveUser', ruleForm).then(function(response) {
		        let res = response.data;

		        if (res.error > 0) {
		            message.error(res.msg);
		        } else {
		            message.success(res.msg,function(){
						that.dialogAccount = false;
						that.cominfo(that.comindex, that.curr_com.uid)
						that.getList();
					});
		        }
		    }).finally(function() {
		        setTimeout(function() {
		            that.saveLoading = false;
		        }, 2000);
		    });
		},
        // 企业日志
        comrz: function (data) {
            this.curr_com = data
            this.qyrz = true
        },
        handleLogoTabClick(tab) {
            if (tab.name == 'uplogo') {
                this.logodata = {
                    path: 'company',
                    imgid: 'logo',
                    uid: this.curr_uid,
                    usertype: 2,
                    pytoken: localStorage.getItem("pytoken")
                }
            }
        },
        handleAvatarSuccess(res) {
            var that = this
            if (res.code == 0) {
                message.success(res.msg, function () {
                    that.uplogopic = res.data.url
                    that.getList()
                })
            } else {
                message.error(res.msg)
            }
        },
        // 企业logo弹窗
        makeLogo: function (uid, short, name, type) {
            var that = this
            that.uplogopic = '';
            that.curr_uid = uid
            if (short != '' && short != undefined) {
                that.logocname = short
            } else {
                that.logocname = name
            }
            if (type == 1) {
                that.logobt = '生成'
            } else if (type == 2) {
                that.logobt = '修改'
            }
            that.logoActiveName = 'autologo'
            that.drawerlogo = true
        },
        previewLogoHb: function () {
            var that = this
            if (that.logocname == '') {
                message.error('文字描述不能为空')
                return false
            } else if (that.logocname.length < 2 || that.logocname.length > 4) {
                message.error('请填写2-4个字')
                return false;
            } else if (that.logobg == '') {
                message.error('请选择背景模板')
                return false;
            }
            that.logopreview = baseUrl + 'm=user&c=company&a=adminLogoHb&name=' + that.logocname + '&hb=' + that.logobg
            that.drawerlogopreview = true
        },
        makeLogoHb: function () {
            var that = this
            if (that.logocname == '') {
                message.error('文字描述不能为空')
                return false
            } else if (that.logocname.length < 2 || that.logocname.length > 4) {
                message.error('请填写2-4个字')
                return false;
            } else if (that.logobg == '') {
                message.error('请选择背景模板')
                return false;
            }
            httpPost('m=user&c=company&a=adminLogoHb', {
                name: that.logocname,
                hb: that.logobg,
                out: 1
            }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    that.setLogo(res.data)
                } else {
                    message.error(res.msg)
                    return false
                }
            }).catch(function (e) {

            })
        },
        setLogo: function (logo) {
            var that = this
            httpPost('m=user&c=company&a=setLogo', { logo: logo, uid: that.curr_uid }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList()
                        that.drawerlogo = false
                    })
                } else {
                    message.error(res.msg)
                    return false
                }
            }).catch(function (e) {

            })
        },
        // 企业会员等级切换
        rateChange: function (e) {
            var that = this
            httpPost('m=user&c=company&a=getrating', { id: e, uid: that.curr_uid }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    if (res.data) {
                        that.initrating(res.data)
                    } else {
                        message.error('请选择会员等级！')
                        return false
                    }
                }
            }).catch(function (e) {

            })
        },
        // 会员等级修改弹窗会员信息处理
        initrating: function (rate) {
            this.integral = rate.integral
            if (rate.vipetime == '不限') {
                this.vip_etime = ''
            } else {
                this.vip_etime = rate.vipetime
            }
			if (rate.max_time_n == '不限') {
			    this.max_time = ''
			} else {
			    this.max_time = rate.max_time_n
			}
            this.oldetime = rate.vip_etime
            this.hotjob = rate.hotjob == '1'
            this.job_num = rate.job_num
            this.breakjob_num = rate.breakjob_num
            this.down_resume = rate.down_resume
            this.invite_resume = rate.invite_resume
            this.zph_num = rate.zph_num
            this.top_num = rate.top_num
            this.urgent_num = rate.urgent_num
            this.rec_num = rate.rec_num
            
            
			this.suspend_num = rate.suspend_num
        },
        // 会员等级修改
        editRating: function (uid, r_status) {
            var that = this
            httpPost('m=user&c=company&a=getstatis', { uid: uid }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    if (res.data) {
                        that.curr_rstatus = r_status
                        that.curr_uid = uid
                        that.rid = res.data.rating
                        that.initrating(res.data)
                        that.drawerrating = true
                    } else {
                        message.error('用户信息获取失败！')
                        return false
                    }
                } else {
                    message.error('用户信息获取失败！')
                    return false
                }
            }).catch(function (e) {

            })
        },
        // 会员等级修改提交
        ratingSubmit: function () {
            var that = this
            var params = {
                rating: that.rid,
                integral: that.integral,
                vipetime: that.vip_etime == '' ? '不限' : that.vip_etime,
                job_num: that.job_num,
                breakjob_num: that.breakjob_num,
                down_resume: that.down_resume,
                invite_resume: that.invite_resume,
                zph_num: that.zph_num,
                top_num: that.top_num,
                urgent_num: that.urgent_num,
                rec_num: that.rec_num,
                
                
                oldetime: that.oldetime,
                ratuid: that.curr_uid,
                rating_name: that.ratingarr[that.rid],
				suspend_num: that.suspend_num,
				max_time: that.max_time,
            }
            if (that.hotjob) {
                params.hotjob = 1
            }

			if(that.vip_etime && that.max_time){
				var vip_etime_v = new Date(that.vip_etime).getTime();
				var max_time_v = new Date(that.max_time).getTime();
				if(vip_etime_v>max_time_v){
					message.error('最长有效期不能短于会员到期时间')
					return false
				}
			}else if(!that.vip_etime && that.max_time){
				message.error('最长有效期不能短于会员到期时间')
				return false
			}
            that.saveLoading = true;
            httpPost('m=user&c=company&a=uprating', params).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList()
                        that.drawerrating = false
                    })
                } else {
                    message.error(res.msg)
                    return false
                }
            }).catch(function (e) {

            }).finally(function () {
                setTimeout(function () {
                    that.saveLoading = false;
                }, 2000);
            });
        },
        // 跳转会员中心前检测
        memberCheck: function (uid, usertype) {
            var that = this
            var tip = ''
            if (usertype != '2') {
                if (usertype == '0') {
                    tip = '该账户当前没有设置身份，以招聘者身份模拟进入可能导致部分功能无法正常使用，是否确认进入？'
                } else {
                    if (usertype == '1') {
                        var u = '求职者';
                    }
                    tip = "该账户当前身份为" + u + "，以招聘者身份模拟进入可能导致部分功能无法正常使用，是否确认进入？"
                }
            }
            if (tip) {
                delConfirm(this, {}, function(params) {
                    that.jumpToMember(uid);
                }, tip)
            } else {
                that.jumpToMember(uid);
            }
        },
        // 跳转到会员中心
        jumpToMember: function (uid, type = '') {
            let tmpWin = window.open('', '_blank')
            var params = { uid: uid }
            if (type != '') {
                params.type = type
            }
            httpPost('m=user&c=company&a=Imitate', params).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    tmpWin.location = res.data
                }
            }).catch(function (e) {
                tmpWin.close()
            })
        },
        // 查询手机归属地
        getmobileaddress: function (uid, moblie) {
            var that = this
            httpPost('m=index&c=getMobileAddress', { uid: uid, moblie: moblie }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList()
                    });
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 查询ip归属地
        getipaddress: function (uid, ip) {
            var that = this
            httpPost('m=index&c=getIpAddress', { uid: uid, ip: ip }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList()
                    });
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 获取企业数量统计
        getTjNum: function () {
            var that = this;
            httpPost('m=user&c=company&a=companyNum', {}, { hideloading: true }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    that.allNum = res.data.companyAllNum ? res.data.companyAllNum : 0
                    that.status1Num = res.data.companyStatusNum1 ? res.data.companyStatusNum1 : 0
                    that.status2Num = res.data.companyStatusNum2 ? res.data.companyStatusNum2 : 0
                    that.status3Num = res.data.companyStatusNum3 ? res.data.companyStatusNum3 : 0
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        getCacheFun:function(){
            let that = this;
            httpPost('m=user&c=company&a=getCache', {},{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.dnameArr = res.data.dname;

                    that.com_social_credit = res.data.config.com_social_credit;
                    that.com_cert_owner = res.data.config.com_cert_owner;
                    that.com_cert_wt = res.data.config.com_cert_wt;
                    that.com_cert_other = res.data.config.com_cert_other;
                    that.com_free_status = res.data.config.com_free_status;
                    that.pricename = res.data.config.pricename;
                    
                    that.todayetime = res.data.config.today_etime;
                    that.today = res.data.config.today;

                    that.hbBgA = res.data.hbBgA;
                    that.searchlist = res.data.search_list;
                    that.ratingarr = res.data.ratingarr;
                    that.source = res.data.source;
                    that.gwArr = res.data.gwinfo;
					that.mapurl = res.data.mapurl;
					that.mapsecret = res.data.mapsecret;
                }
            })
        },
        // 微信绑定绑定检测
        async wxbindstatus(comid) {
            var that = this;
            httpPost('m=user&c=company&a=getacbindstatus', { comid: comid }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    message.success('绑定成功', function () {
                        clearInterval(setval);
                        that.user.wxid = res.data.wxid;
                    });
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 获取微信绑定二维码，显示微信绑定弹窗
        showQrcode: function (comid, wxid) {
            var that = this
            if (wxid != '') {
                message.error('企业已绑定微信');
                return false;
            }
            httpPost('m=user&c=company&a=comcert', { acwxbind: 1, comid: comid }).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    that.code_img = res.data.code_url;
                    that.wxrztc = true;
                    setval = setInterval(function () {
                        that.wxbindstatus(comid)
                    }, 2000);
                } else {
                    message.error(res.msg);
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        factrz:function(data){
            var that = this;
            let factpics = data.factpics;
            this.fact_delid = [];
            this.factfileList = [];
            for (let key in factpics) {
                this.factfileList.push({name:'',url:factpics[key]['pic_n'],id:factpics[key]['id']});
            }
            that.fact_uid = data.uid;
            that.fact_status = data.fact_status=='1'?true:false;
            this.factshow = true;
        },
        factRemove(file, fileList) {
            if(typeof file.id!='undefined' && file.id){
                this.fact_delid.push(file.id);
            }
            this.fact_picurl = fileList
        },
        factChange(file, fileList) {
            this.fact_picurl = fileList
        },
        factexceedFun(files, fileList) {
            this.$message.error('最多只能上传3张图片!');
        },
        factPreview(file){
            this.factImageUrl = file.url;
            this.factImageVisible = true;
        },
        saveFact: function () {
            let _this = this;
            var fact_status = _this.fact_status?1:0;
            var fact_picurl = this.fact_picurl;
            var params = new FormData();
            params.append('uid',_this.fact_uid);
            params.append('fact_status',fact_status);
            params.append('fact_delid',_this.fact_delid);
            
            for (let j in fact_picurl) {
                if (fact_picurl[j].raw) {
                    params.append('newpic[]', fact_picurl[j].raw);
                }
            }
            _this.submitLoading = true;
            httpPost('m=user&c=company&a=savefact', params, {
                headers: { 'Content-Type': 'multipart/form-data' },
            }).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.submitLoading = false;
                    _this.factshow = false;
                    _this.fact_picurl = [];
                    _this.factfileList = [];
                    _this.fact_delid = [];
                    _this.fact_uid = '';
                    
                    message.success(res.msg,_this.getList())
                    
                } else {
                    message.error(res.msg);
                    _this.submitLoading = false;
                }
            }).catch(function (error) {
                console.log(error);
            })
        },
        // 营业执照认证弹窗
        yyzzrz: function (data) {
            var that = this
            that.yy_comname = data.name
            if (data.yyzzurl) {
                that.yy_picurl = data.yyzzurl
            }
            if (data.owner_cert_url) {
                that.yy_owner_picurl = data.owner_cert_url
            }
            if (data.wt_cert_url) {
                that.yy_wt_picurl = data.wt_cert_url
            }
            if (data.other_cert_url) {
                that.yy_other_picurl = data.other_cert_url
            }
            that.yySrcList = []
            if (that.yy_picurl) {
                that.yySrcList.push(that.yy_picurl)
            }
            if (that.yy_owner_picurl) {
                that.yySrcList.push(that.yy_owner_picurl)
            }
            if (that.yy_wt_picurl) {
                that.yySrcList.push(that.yy_wt_picurl)
            }
            if (that.yy_other_picurl) {
                that.yySrcList.push(that.yy_other_picurl)
            }
            that.yy_uid = data.uid
            that.yy_status = data.yyzz_status
            that.yy_scredit = data.social_credit
            httpPost('m=user&c=company&a=comcert', { uid: data.uid, sbody: 1 }).then(function (result) {
                if (result.data.error == 0) {
                    that.yy_sbody = result.data.data.sbody
                    that.zzrztc = true
                } else {
                    message.error('获取审核说明失败');
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 营业执照弹窗提交
        yyzzrzSubmit: function () {
            var that = this
            var params = {
                r_status: that.yy_status,
                uid: that.yy_uid,
                statusbody: that.yy_sbody,
                name:that.yy_comname
            }
            if (that.com_free_status == '1') {
                params.job_status = that.yy_job_status == true ? 1 : 0
            }
            httpPost('m=user&c=company&a=comcert', params).then(function (result) {
                if (result.data.error == 0) {
                    message.success(result.data.msg, function () {
                        that.getList()
                        that.zzrztc = false;
                    });
                } else {
                    message.error(result.data.msg);
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 邮箱认证弹窗
        yxrz: function (email, status, uid) {
            var that = this
            that.yx_email = email
            that.yx_status = status == '1' ? true : false
            that.yx_uid = uid
            that.yxrztc = true
        },
        // 邮箱认证弹窗提交
        yxrzSubmit: function () {
            var that = this
            var params = {
                comemail: that.yx_email,
                estatus: that.yx_status == true ? 1 : 0,
                uid: that.yx_uid
            }
            httpPost('m=user&c=company&a=comcert', params).then(function (result) {
                if (result.data.error == 0) {
                    message.success(result.data.msg, function () {
                        that.getList()
                        that.yxrztc = false;
                    });
                } else {
                    message.error(result.data.msg);
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 手机认证弹窗
        sjrz: function (mobile, status, uid) {
            var that = this
            that.sj_mobile = mobile
            that.sj_status = status == '1' ? true : false
            that.sj_uid = uid
            that.sjrztc = true
        },
        // 手机认证弹窗提交
        sjrzSubmit: function () {
            var that = this
            var params = {
                comlinktel: that.sj_mobile,
                mstatus: that.sj_status == true ? 1 : 0,
                uid: that.sj_uid
            }
            httpPost('m=user&c=company&a=comcert', params).then(function (result) {
                if (result.data.error == 0) {
                    message.success(result.data.msg, function () {
                        that.getList()
                        that.sjrztc = false;
                    });
                } else {
                    message.error(result.data.msg);
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 批量推文任务
        twtaskall: function () {
            var that = this
            var nowTime = parseInt(new Date().getTime() / 1000);
            var codearr = "";
            var lastupdate = '';
            var twnum = 0;
            var twTip = '';
            var statusMsg = '';
            this.tableData.forEach(function (item) {
                if (that.selectedItem.includes(item.uid)) {
                    if (codearr == "") {
                        codearr = item.uid;
                    } else {
                        codearr = codearr + "," + item.uid;
                    }
                    if (twnum == 0) {
                        twnum = Number(item.tw_num);
                    }
                    lastupdate = Number(item.lastupdate);
                    if (twTip == '' && nowTime - lastupdate > 60 * 60 * 24 * 7) {
                        twTip = '有部分企业已超过7天没有更新，请确认无误后添加';
                    }
                    if (item.r_status == '0') {
                        statusMsg = '未审核';
                    }
                    if (item.r_status == '2') {
                        statusMsg = '已锁定';
                    }
                    if (item.r_status == '3') {
                        statusMsg = '未通过';
                    }
                    if (item.r_status == '4') {
                        statusMsg = '已暂停';
                    }
                }
            })
            if (statusMsg != '') {
                var msg = '部分所选企业非审核状态，请重新选择。';
                message.error(msg)
                return false;
            }
            if (codearr == "") {
                message.error('请选择要生成推文任务的企业！')
                return false;
            } else if (twnum > 0) {

                delConfirm(this, {}, function(params) {
                    that.addTwAll(twTip, codearr)
                }, '有部分企业已添加推文未发送，是否继续添加？')

            } else {
                that.addTwAll(twTip, codearr)
            }
        },
        addTwAll: function (twTip, codearr) {
            var that = this
            that.tw_tip = twTip
            that.tw_data = {
                uid: codearr
            }
            that.tw_desc = ''
            that.multitw = true
            that.twdrawer = true
        },
        // 创建单个企业推文任务
        addTuiWenTask: function (id, name, lastupdate, num) {
            var that = this
            if (num > 0) {
                delConfirm(this, {}, function(params) {
                    that.addTW(id, name, lastupdate)
                }, '该企业已有推文未发送，是否继续添加？')
            } else {
                that.addTW(id, name, lastupdate)
            }
        },
        addTW: function (id, name, lastupdate) {
            var that = this
            var nowTime = parseInt(new Date().getTime() / 1000);
            lastupdate = Number(lastupdate);
            if (nowTime - lastupdate > 60 * 60 * 24 * 7) {
                that.tw_tip = '企业已超过7天没有更新，请确认无误后添加';
            } else {
                that.tw_tip = ''
            }
            that.tw_data = {
                uid: id,
                name: name
            }
            that.tw_desc = ''
            that.multitw = false
            that.twdrawer = true
        },
        // 提交创建推文任务信息
        addTwTask: function () {
            var that = this
            var params = {
                twtask_uid: that.tw_data.uid,
                twtask_content: that.tw_desc,
                twtask_urgent: that.jj ? 1 : 0,
                twtask_wcmoments: that.pyq == true ? 1 : 0,
                twtask_gzh: that.gzh == true ? 1 : 0
            };
            httpPost('m=user&c=company&a=addTuiWenTask', params).then(function (result) {
                if (result.data.error == 0) {
                    message.success(result.data.msg, function () {
                        that.getList()
                        that.twdrawer = false;
                        that.tableData.forEach(item => {
                            if (item.uid == that.tw_data.uid){
                                item.tw_num++;
                            }
                        });
                    });
                } else {
                    message.error(result.data.msg);
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 企业审核提交
        comSh: function (atype) {
            var that = this
            var params = {
                single: 1,
                status: that.r_status,
                uid: that.curr_com.uid,
                statusbody: that.statusbody,
                atype: atype
            };
            if (that.member_status == '1') {
                params.lock_status = that.member_status;
            } else {
                message.error("锁定操作异常！");
                return false;
            }
            that.comShSubmit(params)
        },
        comShSubmit: function (params) {
            var that = this
            httpPost('m=user&c=company&a=status', params).then(function (result) {
                if (result.data.error == 0) {
                    message.success(result.data.msg, function () {
                        if (result.data.hasOwnProperty("data") && result.data.data.hasOwnProperty("uid") && result.data.data.uid) {
                            that.comaudit(result.data.data.uid)
                        } else {
                            that.getList();
                            if (params.single == 1) {
                                that.comdrawersh = false;
                            } else {
                                that.drawerauditmultiple = false;
                            }
                        }
                    });
                } else {
                    message.error(result.data.msg);
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 获取企业审核信息
        comaudit: function (uid) {
            var that = this
            httpPost('m=user&c=company&a=companyAudit', { uid: uid }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.comdrawersh = true
                    that.curr_com = res.data.Info
                    that.r_status = that.curr_com.r_status
                    that.member_status = that.curr_com.member_status
                    that.islock = that.r_status == '2' ? true : false
                    that.statusbody = that.curr_com.statusbody
                    that.lockdesc = that.curr_com.statusbody
                    that.snum = res.data.snum
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
                    _this.selectedItem.push(item.uid);
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
		// 搜索城市选择
		confirmCitySearch(data) {
			this.search_params.city_class = data.cityId.join(',');
		},
        search() {
            this.currentPage = 1;
            this.getList();
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
            if (that.search_params.rating) {
                params.rating = that.search_params.rating
            }
            if (that.search_params.rec) {
                params.rec = that.search_params.rec
            }
            if (that.search_params.time) {
                params.time = that.search_params.time
            }
            if (that.search_params.status) {
                params.status = that.search_params.status
            }
            if (that.search_params.source) {
                params.source = that.search_params.source
            }
            if (that.search_params.gw) {
                params.gw = that.search_params.gw
            }
                if (that.search_params.time_type != '') {
                    params.time_type = that.search_params.time_type;
                }
                if (Array.isArray(that.search_params.times) && that.search_params.times.length == 2) {
                    params.times = that.search_params.times;
            }
            if (that.search_params.has_job) {
                params.has_job = that.search_params.has_job
            }
            if (that.search_params.reg_days) {
                params.reg_days = that.search_params.reg_days
            }
            if (that.search_params.reg_time) {
                params.reg_time = that.search_params.reg_time
            }
            if (that.search_params.login_days) {
                params.login_days = that.search_params.login_days
            }
            if (that.search_params.login_time) {
                params.login_time = that.search_params.login_time
            }
            if (that.sort_type && that.sort_col) {
                params.order = that.sort_type
                params.t = that.sort_col
            }
			if (that.search_params.city_class) {
				params.city_class = that.search_params.city_class
			}
            if (that.search_params.fact_status) {
                params.fact_status = that.search_params.fact_status
            }
                if (that.search_params.map_status) {
                    params.map_status = that.search_params.map_status;
                }
            that.loading = true;
            that.emptytext = "数据加载中";
            httpPost('m=user&c=company&a=index', params, { hideloading: true }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.tableData = res.data.list;
                    if (that.tableData.length === 0) {
                        that.emptytext = "暂无数据";
                    }
                    that.perPage = parseInt(res.data.perPage)
                    that.pageSizes = res.data.pageSizes
                    that.total = parseInt(res.data.total)
                    if(that.prevPage != that.currentPage){
                        that.prevPage = that.currentPage;
                        that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                        scrollToTop()
                    }
                    that.loading = false;
                }
            }).catch(function (e) {
                console.log(e)
            })
        },

        // 删除
        openDel(idx) {
            if (typeof idx == 'undefined') { // 批量删除
                if (!this.selectedItem.length) {
                    message.error('请选择要删除的数据');
                    return false;
                }
                this.ruleFormDel = {
                    del: this.selectedItem,
                    delAccount: 0
                }
            } else { // 单个删除
                this.ruleFormDel = {
                    del: this.curr_uid,
                    delAccount: 0
                }
            }
            this.dialogDel = true;
        },
        delSubmit() {
            let that = this,
                ruleForm = this.ruleFormDel;
            httpPost('m=user&c=company&a=del', ruleForm).then(function (response) {
                let res = response.data;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogDel = false;
                    that.qyxqdrawer = false;
                    that.getList();
                    that.$refs.multipleTable.clearSelection();
                    message.success(res.msg)
                }
            })
        },
        showInput() {
            this.inputVisible = true;
            this.$nextTick(_ => {
                this.$refs.saveTagInput.$refs.input.focus();
            });
        },
        handleClick(tab) {
            if (this.activeName == 'second') {
                //this.getComJobList()
            }
            // else if (this.activeName == 'severth') {
            //     this.$refs.jfgl.search()
            // } else if (this.activeName == 'zzhth') {
            //     this.$refs.zzh.search()
            // } else if (this.activeName == 'company_order') {
            //     this.$refs.company_order.search()
            // } else if (this.activeName == 'company_tcjl') {
            //     this.$refs.company_tcjl.search()
            // }
        },
        
        bindPackage() {
            this.ruleFormPackage = {
                uid: this.curr_com.uid,
                package: this.curr_com.package
            }
            this.dialogPackage = true;
        },
        submitPackage() {
            let that = this,
                params = that.ruleFormPackage;

            // if (params.package.length == 0) {
            //     message.warning('请选择绑定套餐');
            //     return;
            // }

            that.saveLoading = true;
            httpPost('m=user&c=company&a=bindPackage', params).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    that.saveLoading = false;
                    message.error(res.msg);
                } else {
                    that.dialogPackage = false;
                    message.success(res.msg, function () {
                        that.curr_com.package = params.package;
                        that.saveLoading = false;
                    });
                }
            });
        },
        JumpComJob(row,status){
            let params ={
                keyword:row.name,
                type:'1',
                status:status?status:''
            }
            window.parent.homeapp.checkMenuTwo(1, 6, 40, '职位管理', '/companyjob', params);
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
            }
        }
};
</script>
<style>
    .pad_lr_20{padding: 0 20px;}
    .el-tag+.el-tag{margin-left:10px}
    .button-new-tag{margin-left:10px;height:32px;line-height:30px;padding-top:0;padding-bottom:0}
    .input-new-tag{width:90px;margin-left:10px;vertical-align:bottom}
    .job_set_list{margin-bottom:10px;margin-top:10px;font-size:14px;color:#606266}
    .job_set_pd{padding-left:20px}
    .TableInptline{line-height:35px;padding:0 10px}
    .jobchecked{padding:8px 0 0 10px}
    .cominfocz{padding:15px 0;position:fixed;overflow:hidden;right:0;bottom:0;width:calc(95% - 20px);background:#fff;z-index:222;border-top:1px solid #eee}
    .el-dialog__body{padding:0 20px}
    .logo_box_img{opacity:.5}
    .logo_box_img_cur{opacity:1}
    .avatar-uploader .el-upload{border:1px dashed #d9d9d9;border-radius:6px;cursor:pointer;position:relative;overflow:hidden}
    .avatar-uploader .el-upload:hover{border-color:#409eff}
    .logoavatar-uploader-icon{font-size:28px;color:#8c939d;width:200px;height:200px;line-height:200px;text-align:center}
    .logoavatar{width:200px;height:200px;display:block}
    .comavatar-uploader-icon{font-size:28px;color:#8c939d;width:100px;height:100px;line-height:100px;text-align:center}
    .comavatar{width:100px;height:100px;display:block}
    .center{display:flex;align-content:center;justify-content:center;text-align:center;padding-bottom:18px}
    .waixunHaib{overflow:hidden;position:relative;padding:0 20px;width:100%}
    .waixunHaib ul{overflow:hidden;position:relative;width:calc(100% - 16px);display:flex;padding:0 8px;flex-wrap:wrap;align-items:center;justify-content:space-between}
    .waixunHaib ul::after{overflow:hidden;position:relative;display:block;content:"";width:calc(19% - 8px)}
    .waixunHaib ul li{overflow:hidden;position:relative;width:calc(19% - 12px);margin-bottom:15px}
    .hb_listbox{overflow:hidden;position:relative}
    .poster_pic{width:100%}
    .poster_pic img{width:100%;border-radius:3px;box-shadow:0 5px 10px 0 rgba(111,116,132,.1)}
    .hb_listbox_name{font-size:15px;width:100%;text-align:center;padding-top:10px}
    .hb_cz{padding-top:10px}
    .tableSeachInptsmall .el-input{width:initial; }
    .tableSeacFromer{margin-right:8px}
    .tableSeacFromer .el-input-group__prepend{padding:0;background:0 0}
    .tableSeacFromer .el-select{margin-right:0;width:120px; padding-left:20px;}
    .tableSeacFromer .el-input{margin-right:0}
    .el-upload--picture-card{width:98px;height:98px; line-height: 96px;;}
    .el-upload-list--picture-card .el-upload-list__item{width: 98px;
    height: 98px;}
    .sdhy_tg{ padding-top:20px;}
</style>