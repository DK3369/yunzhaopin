<template>
    <div class="moduleElHight">
        <div class="moduleSeachs">
            <div class="moduleSeachInpt">
                <el-input :placeholder="lc('admin_00340')" size="small" style="margin-right: 8px;" v-model="keyword" class="input-with-select" clearable>
                    <el-select v-model="type" slot="prepend" :placeholder="lc('wap_user_00100')">
                        <el-option :label="lc('member_com_00377')" value="1"></el-option>
                        <el-option :label="lc('admin_00321')" value="2"></el-option>
                    </el-select>
                </el-input>
                <el-select v-model="status" size="small" slot="prepend" :placeholder="lc('wap_com_00406')" clearable style="margin-right: 10px; text-align: left;" @change="search">
                    <el-option :label="lc('admin_00319')" value="1"></el-option>
                    <el-option :label="lc('admin_00320')" value="3"></el-option>
                    <el-option :label="lc('wap_user_00259')" value="2"></el-option>
                </el-select>
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
            <div class="moduleSeachButn">
                <div class="tableSeachInpt" style="margin-bottom: 0;">
                    <el-button type="primary" icon="el-icon-document-add" size="mini" @click="addzph({id: ''})">{{ lc('admin_00848') }}</el-button>
                </div>
            </div>
        </div>
        <div class="admin_datatip">
            <i class="el-icon-document"></i> {{ lc('admin_00830') }}
        </div>
        <div class="moduleElTable moduleElMoreLive" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" @selection-change="handleSelectionChange" ref="multipleTable" :default-sort="{ prop: 'id', order: 'descending' }" @sort-change='sortChange' v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="80" sortable="custom"></el-table-column>
                <el-table-column :label="lc('member_com_00377')" min-width="180" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <div class="">
                            <el-link type="primary" :href="scope.row.url" target="_blank"> {{ scope.row.title }}</el-link>
                        </div>
                        <div class="">{{ lc("admin_venue_value", [scope.row.address]) }}</div>
                    </template>
                </el-table-column>
                <el-table-column prop="starttime" :label="lc('admin_company_00005')" width="180" sortable="custom"></el-table-column>
                <el-table-column prop="endtime" :label="lc('admin_company_00006')" width="180" sortable="custom"></el-table-column>
                <el-table-column :label="lc('wap_00559')" min-width="80" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <div style="padding-top: 10px;">
                            <el-button @click="showComList(scope.row)" type="text">
                                <span>{{scope.row.comnum}}</span><br />
                                <span>{{ lc('wap_com_00427') }}</span>
                            </el-button>
                            <el-tooltip v-if="scope.row.booking > 0" :content="lc('admin_00316')" placement="top-end">
                                <el-button @click.native="showComList(scope.row, 3)" type="text">
                                    <el-badge :value="scope.row.booking" :max="99" class="item"></el-badge>
                                </el-button>
                            </el-tooltip>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="zd" :label="lc('admin_user_weipin_00050')" width="75">
                    <template slot-scope="scope">
                        <span>{{ dnamearr[scope.row.did] }}</span>
                        <el-link type="primary" @click="fp(scope.row)">{{ lc('admin_user_weipin_00048') }}</el-link>
                    </template>
                </el-table-column>
                <el-table-column prop="id" :label="lc('admin_00846')" width="180">
                    <template slot-scope="scope">
                        <el-link type="primary" @click="piclist(scope.row)">{{ lc('admin_00586') }}</el-link>
                    </template>
                </el-table-column>
                <el-table-column prop="zt" :label="lc('admin_00285')">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span class="admin_state1">{{ lc('admin_00849') }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" fixed="right" width="140">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini">
                                <el-link style="font-size: 12px;" :href="scope.row.url" target="_blank">{{ lc('wap_00071') }}
                                </el-link>
                            </el-button>
                            <el-button size="mini" @click="statusSet(scope.row)">{{ lc('member_user_00181') }}</el-button>
                        </div>
                        <div class="cz_button" style="margin-top: 10px;">
                            <el-button size="mini" @click="addzph(scope.row)">{{ lc('wap_js_00073') }}</el-button>
                            <el-button type="danger" size="mini" @click="delrow(scope.row.id)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="delAllBottom" size="mini">{{ lc('member_com_00055') }}</el-button>
                <el-button @click="fpAllBottom" size="mini">{{ lc('admin_user_00279') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" :current-page="currentPage" :page-sizes="pageSizes" :page-size="perPage" layout="total, sizes, prev, pager, next, jumper" :total="total" :pager-count="pagerCount">
                </el-pagination>
            </div>
        </div>
        <!--添加招聘会-->
        <el-drawer :title="lc('admin_00317')" :visible.sync="editdrawer" :modal-append-to-body="false" :show-close="true" :with-header="true" size="60%">
            <xczphadd ref="zphadd" :cdata="curr_data" :spacearr="spacearr" :dnamearr="dnamearr" @change="addPlace" @child-event-list="handleCloseList"></xczphadd>
        </el-drawer>
        <!--招聘会状态-->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00855')" :visible.sync="drawerstatus" :with-header="true" :modal-append-to-body="false" :show-close="true" width="450px">
                <div class="wxsettip_small ">{{ lc('member_com_00377') }}</div>
                <el-input v-model="curr_data.title" :disabled="true">
                </el-input>
                <div class="wxsettip_small ">{{ lc('admin_00315') }}</div>
                <template>
                    <el-radio v-model="curr_data.is_open" label="1">{{ lc('member_com_00023') }}</el-radio>
                    <el-radio v-model="curr_data.is_open" label="0">{{ lc('common.close') }}</el-radio>
                </template>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawerstatus = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="statusSave" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--参会企业-->
        <el-drawer :title="curr_data.title + ' - ' + lc('wap_00559')" :visible.sync="comdrawer" :modal-append-to-body="false" append-to-body size="80%" @close="closeComList">
            <div class="moduleElHight" style="padding: 0 20px;">
                <div class="modulemoreSeach">
                    <div class="moduleSeachbig">
                        <div class="tableSeachInpt tableSeachInptsmall newsinput">
                            <el-select v-model="com_status" size="small" slot="prepend" :placeholder="lc('wap_com_00406')" clearable @change="comSearch">
                                <el-option :label="lc('member_user_00042')" value="1"></el-option>
                                <el-option :label="lc('wap_user_00166')" value="3"></el-option>
                                <el-option :label="lc('wap_user_00167')" value="2"></el-option>
                            </el-select>
                        </div>
                        <div class="tableSeachInpt">
                            <el-input v-model="comkeyword" :placeholder="lc('wap_user_00149')" clearable size="small" prefix-icon="el-icon-search">
                            </el-input>
                        </div>
                        <div class="tableSeachInpt">
                            <el-button type="primary" icon="el-icon-search" size="mini" @click="comSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
                        </div>
                    </div>
                    <div class="moduleSeachButn">
                        <div class="tableSeachInpt" style="padding-right:10px; ">
                            <el-button type="primary" icon="el-icon-document-add" size="mini" @click="comadd">{{ lc('admin_00850') }}
                            </el-button>
                            <!--<el-button type="primary" plain icon="el-icon-plus" size="mini">导出参会企业</el-button>-->
                        </div>
                    </div>
                </div>
                <!--<div class="admin_datatip"><i class="el-icon-document"></i> 参会企业共 120家 参会职位 8789</div>-->
                <div class="moduleElTable" style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 100px);">
                    <el-table :data="comTableData" style="width: 100%" stripe :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" @selection-change="handleComSelectionChange" ref="comMultipleTable" :default-sort="{ prop: 'id', order: 'descending' }" @sort-change='comSortChange' :key="comTblKey" v-loading="loading" :empty-text="emptytext">
                        <el-table-column type="selection" width="55"></el-table-column>
                        <el-table-column prop="id" :label="lc('member_com_00345')" sortable="custom"></el-table-column>
                        <el-table-column prop="comname" :label="lc('admin_00299')" min-width="180" show-overflow-tooltip></el-table-column>
                        <el-table-column prop="jobname" :label="lc('wap_user_00154')" min-width="180" show-overflow-tooltip></el-table-column>
                        <el-table-column prop="space_n" :label="lc('admin_00306')" sortable="custom"></el-table-column>
                        <el-table-column prop="sort" :label="lc('member_com_00022')" sortable="custom">
                            <template slot-scope="scope">
                                <el-input v-if="scope.row[scope.column.property + 'isShow']" :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index" v-model="scope.row.sort" @blur="alterData(scope, 1)"></el-input>
                                <span v-else>
                                    {{ scope.row.sort }}
                                    <img src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14" @click="editData(scope, 1)">
                                </span>
                            </template>
                        </el-table-column>
                        <el-table-column prop="zt" :label="lc('member_user_00181')">
                            <template slot-scope="props">
                                <div class="admin_state">
                                    <span v-if="props.row.status == '1'" class="admin_state1"> {{ lc('admin_user_00149') }}</span>
                                    <span v-else-if="props.row.status == '0'" class="admin_state2"> {{ lc('wap_user_00166') }}</span>
                                    <span v-else-if="props.row.status == '2'" class="admin_state2"> {{ lc('wap_user_00167') }}</span>
                                </div>
                            </template>
                        </el-table-column>
                        <el-table-column :label="lc('member_user_00048')" fixed="right" width="150" align="center">
                            <template slot-scope="scope">
                                <div class="cz_button">
                                    <el-button size="" plain @click="cominfo(scope.row)">{{ lc('member_com_00380') }}</el-button>
                                    <el-button type="danger" size="small" @click="comdelrow(scope.row.id)">{{ lc('common.delete') }}
                                    </el-button>
                                </div>
                            </template>
                        </el-table-column>
                    </el-table>
                </div>
                <div class="modulePaging" style="padding-top: 5px">
                    <div class="bottomButnBull">
                        <div class="bottomButnBlak">
                            <el-checkbox v-model="comCheckedAll" @change="comSelectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                            <el-button type="mini" @click="comDelAllBottom">{{ lc('member_com_00055') }}</el-button>
                        </div>
                        <div class="bottomButnNone">
                            <el-popover placement="top-start" width="200" trigger="hover">
                                <div class="bottomButnGend">
                                    <el-button type="mini" @click="multipleComStatus">{{ lc('admin_user_weipin_00037') }}</el-button>
                                    <el-button type="mini" @click="exportExcel">{{ lc('admin_00314') }}</el-button>
                                </div>
                                <div class="bottomButnMore" slot="reference">{{ lc('common.more') }}</div>
                            </el-popover>
                        </div>
                    </div>
                    <div class="modulePagNum">
                        <el-pagination background @size-change="handleComSizeChange" @current-change="handleComCurrentChange" :current-page="comCurrentPage" :page-sizes="comPageSizes" :page-size="comPerPage" layout="total, sizes, prev, pager, next, jumper" :total="comTotal" :pager-count="pagerCount">
                        </el-pagination>
                    </div>
                </div>
            </div>
        </el-drawer>
        <!--参会企业添加-->
        <el-drawer :title="lc('admin_00850')" :visible.sync="drawercomadd" append-to-body :modal-append-to-body="false" size="80%">
            <div class="shbox shboxTianCompay">


                <div class="shboxTianHrigh">
                    <div class="shboxAddCompany" style="text-align: center;">
                        <div class="shboxAddComTite">
                            <span>{{ lc('admin_00318') }}</span>
                        </div>
                        <div class="shboxAddComInt">
                            <el-select v-model="comid" filterable remote :placeholder="lc('admin_user_company_00019')" :remote-method="getComArr" @change="comChange">
                                <el-option v-for="item in com_arr" :key="item.value" :label="item.label" :value="item.value">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                    <div class="shboxAddCompany" style="text-align: center;margin-top: 10px;">
                        <div class="shboxAddComTite">
                            <span>{{ lc('admin_00297') }}</span>
                        </div>
                        <div class="shboxAddComInt">
                            <el-select v-model="jobids" filterable remote :placeholder="lc('admin_00300')" multiple>
                                <el-option v-for="item in job_arr" :key="item.value" :label="item.label" :value="item.value">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                    <div class="" style="text-align: center;">
                        <span class="admin_web_tip">{{ lc('admin_00309') }}</span>
                    </div>
                    <div class="yd_qy">{{ lc("admin_booth_selection_title", [curr_data.title]) }}</div>
                    <div class="yd_qylist" v-for="(item,index) in space_list" :key="index">
                        <el-divider content-position="center">{{item.name}}</el-divider>
                        <div class="yd_ztbox">
                            <div @click="changezw(childit, item, $event)" :class="ydCls(item, childit)" v-for="(childit,index) in item.list" :key="item.id + index + ''">
                                <span v-if="childit.comstatus == '-1'" class="yd_zt_n">{{ lc('admin_00301') }}</span>
                                <span v-if="childit.comstatus == '1'" class="yd_zt_n">{{ lc('admin_00303') }}</span>
                                <span v-if="childit.comstatus == '0'" class="yd_zt_n">{{ lc('wap_user_00174') }}</span>
                                <span v-if="childit.comstatus == '2' || childit.comstatus == '3'" class="yd_zt_n">{{ lc('admin_00298') }}</span>
                                <span class="yd_zt_zw">{{childit.name}}</span>
                            </div>
                        </div>
                    </div>
                </div>

                
                <div class="yd_zt_fot shboxTianButnd">
                    <div class="yd_zt_bth">
                        <div class="yd_zt_bthleft">
                            <div class="">{{sel_comname}}</div>
                            <div class="yd_zt_bthzwbox">{{ lc("admin_booth_value", [sel_zwname]) }}</div>
                        </div>
                        <div class="yd_zt_bthbot">
                            <el-button type="primary" @click="saveAddCom" :disabled="submitLoading">{{ lc('admin_00305') }}</el-button>
                        </div>
                    </div>
                </div>
            </div>
        </el-drawer>
        <el-drawer :title="lc('admin_00856')" :visible.sync="zphdrawerimg" :modal-append-to-body="false" append-to-body size="80%">
            <xczphimg ref="zphimg" :zphid="curr_data.id"></xczphimg>
        </el-drawer>
        <!--参会企业详情-->
        <el-drawer :title="lc('admin_00843')" v-if="dtlislook" :visible.sync="comdrawersh" :modal-append-to-body="false" append-to-body size="80%">
            <div class="shbox">
                <div class="shinfo">
                    <div class="shcomname">{{info.name}}
                        <el-tag type="danger" size="mini">{{info.rating_name}}</el-tag>
                    </div>
                    <div class="sh_zwsz_add">{{ lc("admin_company_booth_value", [curr_comdata.zphname, curr_comdata.space_n]) }}</div>
                    <div class="sh_zwsz">
                        <el-button type="primary" size="mini" plain @click="setZw"><i class="el-icon-edit"></i> {{ lc('admin_00838') }}
                        </el-button>
                        <el-button type="primary" size="mini" @click="showComJob"><i class="el-icon-suitcase-1"></i> {{ lc('wap_00560') }}</el-button>
                    </div>
                    <div class="shcomtel">
                        <span v-if="info.linkman">
                            {{ lc("admin_contact_person_value", [info.linkman]) }}<span v-if="info.linkjob">（{{info.linkjob}}）</span>
                        </span>
                        <span class="shcomtel_n" v-if="info.linktel">
                            {{ lc("admin_contact_phone_value", [info.linktel]) }}
                        </span>
                        <span v-if="info.crm_uid != '0'">
                            {{ lc("admin_salesperson_value", [info.crm_name]) }}
                        </span>
                    </div>
                    <div class="shshowall">
                        <div class="shshow">
                            <div class="shshow_tit"><i class="el-icon-office-building"></i> {{ lc('wap_user_00341') }}</div>
                            <div class="shshow_p">
                                <div class="" v-if="info.welfare">{{ lc('admin_00644') }}
                                    <el-tag style="margin-right: 5px;" v-for="(item,index) in info.welfare_n" size="mini" :key="index">
                                        {{item}}
                                    </el-tag>
                                </div>
                                <div class="" v-if="info.hy">{{ lc("admin_industry_value", [info.hy_n]) }}</div>
                                <div class="" v-if="info.pr">{{ lc("admin_company_nature_value", [info.pr_n]) }}</div>
                                <div class="" v-if="info.mun">{{ lc("admin_company_size_value", [info.mun_n]) }}</div>
                                <div class="" v-if="info.provinceid">{{ lc('admin_00839') }}
                                </div>
                                <div class="" v-if="info.content" v-html="info.content"></div>
                            </div>
                            <div class="shshow_tit" v-if="info.job_list.length > 0"><i class="el-icon-suitcase-1"></i>
                                {{ lc('wap_01536') }}
                            </div>
                            <ul class="shshow_joblist">
                                <li v-for="(item,index) in info.job_list" :key="index">
                                    <el-link type="primary" :underline="false">
                                        <div class="shshow_job">{{item.name}}</div>
                                    </el-link>
                                    <div class="shshow_jobinfo">
                                        <span class="shshow_jobxz">
                                            <span v-if="item.minsalary != '0'">{{item.minsalary}}</span>
                                            <span v-if="item.minsalary != '0' && item.maxsalary != '0'">-</span>
                                            <span v-if="item.maxsalary != '0'">{{item.maxsalary}}</span>
                                        </span>
                                        <span class="shshow_line" v-if="!item.exp_req">{{ lc('admin_00852') }}</span>
                                        <span class="shshow_line" v-if="!item.edu_req">{{ lc('admin_00853') }}</span>
                                    </div>
                                    <span class="shshow_zt" v-if="item.ch_n == 'admin_00302'">{{ lc(item.ch_n) }}</span>
                                    <span class="shshow_zt shshow_ztno" v-else>{{ lc(item.ch_n) }}</span>
                                </li>
                            </ul>
                        </div>
                        <div class="shcz">
                            <div class="wxsettip_small ">{{ lc('admin_00842') }}</div>
                            <template>
                                <el-radio v-model="info.zph.status" label="1">{{ lc('admin_user_00149') }}</el-radio>
                                <el-radio v-model="info.zph.status" label="2">{{ lc('wap_user_00167') }}</el-radio>
                            </template>
                            <div class="wxsettip_small ">{{ lc('admin_user_00365') }}</div>
                            <el-input type="textarea" v-model="info.zph.statusbody" :rows="2" :placeholder="lc('wap_user_00076')">
                            </el-input>
                            <div class=" shczbth">
                                <el-button type="primary" @click="comStatusSave(info.zph.id)" :disabled="submitLoading">{{ lc('member_com_00248') }}</el-button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </el-drawer>
        <!--分配站点弹窗-->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_weipin_00029')" width="300px" :visible.sync="drawerfp" :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite" style="width: 90px;">
                            <span>{{ lc('admin_00854') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <span>{{curr_data.title}}</span>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_user_weipin_00020') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-select v-model="curr_data.did" :placeholder="lc('wap_user_00100')" filterable>
                                <el-option v-for="(item,index) in dnamearr" :key="index" :label="item" :value="index">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawerfp = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="fpSave(1)" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--批量分配站点-->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00162')" width="300px" :visible.sync="drawerfpmultiple" :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_user_weipin_00020') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-select v-model="multipledid" :placeholder="lc('wap_user_00100')" filterable>
                                <el-option v-if="dnamearr.length>0" v-for="(item,index) in dnamearr" :key="index" :label="item" :value="index">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawerfpmultiple = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="fpSave(2)" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--批量审核参会企业-->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00842')" width="300px" :visible.sync="drawercomstatusmultiple" append-to-body :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_user_weipin_00065') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-radio v-model="multiComStatus" label="1">{{ lc('admin_user_00149') }}</el-radio>
                            <el-radio v-model="multiComStatus" label="2">{{ lc('wap_user_00167') }}</el-radio>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('member_user_00450') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input type="textarea" v-model="multiComStatusBody" :rows="2" :placeholder="lc('wap_user_00076')">
                            </el-input>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawercomstatusmultiple = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="multipleComStatusSave">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--参会企业详情 设置展位-->
        <el-drawer :title="lc('admin_00838')" :visible.sync="drawersetzw" :modal-append-to-body="false" append-to-body size="80%">
            <div class="yd_qy">{{ lc("admin_booth_selection_title", [curr_data.title]) }}</div>
            <div class="yd_qylist" v-for="(item,index) in space_list" style="margin-left: 20px;" :key="index">
                <el-divider content-position="center">{{item.name}}</el-divider>
                <div class="yd_ztbox">
                    <div :class="ydCls(item, childit)" @click="changezw(childit, item, $event)" v-for="(childit,index) in item.list" :key="item.id + index + ''">
                        <span v-if="childit.comstatus == '-1'" class="yd_zt_n">{{ lc('admin_00301') }}</span>
                        <span v-if="childit.comstatus == '1'" class="yd_zt_n">{{ lc('admin_00303') }}</span>
                        <span v-if="childit.comstatus == '0'" class="yd_zt_n">{{ lc('wap_user_00174') }}</span>
                        <span v-if="childit.comstatus == '2' || childit.comstatus == '3'" class="yd_zt_n">{{ lc('admin_00298') }}</span>
                        <span class="yd_zt_zw">{{childit.name}}</span>
                    </div>
                </div>
            </div>
            <div class="yd_zt_fot">
                <div class="yd_zt_bth">
                    <div class="yd_zt_bthleft">
                        <div class="yd_zt_bthzwbox">{{ lc("admin_booth_value", [sel_zwname]) }}</div>
                    </div>
                    <div class="yd_zt_bthbot">
                        <el-button type="primary" @click="saveChangeZw">{{ lc('admin_00305') }}</el-button>
                    </div>
                </div>
            </div>
        </el-drawer>
        <!--参会职位-->
        <div class="modluDrawer">
            <el-dialog :title="lc('wap_00560')" width="300px" :visible.sync="drawercomjob" append-to-body :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00297') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-select v-model="jobids" filterable remote :placeholder="lc('admin_00300')" multiple>
                                <el-option v-for="item in job_arr" :key="item.value" :label="item.label" :value="item.value">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawercomjob = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="saveComJob">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
    </div>
</template>
<script>
module.exports = {
    data: function() {
        return {
            loading: false,
            pagerCount: 5,
            emptytext: lc('wap_js_00113'),
            submitLoading: false,
            type: '1',
            keyword: '',
            comkeyword: '',
            checkedAll: false,
            selectedItem: [],
            tableData: [],
            currentPage: 1,
            perPage: 0,
            pageSizes: [],
            total: 0,
            comCheckedAll: false,
            comSelectedItem: [],
            comTableData: [],
            comCurrentPage: 1,
            comPerPage: 0,
            comPageSizes: [],
            comTotal: 0,
            comTblKey: '',
            islook: false,
            sort_type: '',
            sort_col: '',
            dnamearr: {},
            curr_data: {},
            curr_comdata: {},
            drawerfp: false,
            drawerfpmultiple: false,
            multipledid: '',
            preview_url: '',
            zphdrawerimg: false,
            drawerstatus: false,
            editdrawer: false,
            spacearr: [],
            status: '',
            com_status: '',
            oldData: null,
            info: null,
            dtlislook: false,
            drawercomstatusmultiple: false,
            multiComStatus: '',
            multiComStatusBody: '',
            drawercomadd: false,
            comid: '',
            com_arr: [],
            space_list: [],
            sel_comname: '',
            sel_zwid: '',
            sel_cdid: '',
            sel_zwname: lc('admin_00304'),
            job_arr: [],
            jobids: [],
            comdrawer: false,
            comdrawersh: false,
            tableHig: true,
            drawersetzw: false,
            drawercomjob: false,
            pytoken: localStorage.getItem("pytoken"),
            baseUrl: localStorage.getItem("baseUrl"),
            prevPage: 0,
            prevPage1: 0
        }
    },
    mounted() {},
    components: {
        'xczphimg': httpVueLoader('./xczphimg.vue'),
        'xczphadd': httpVueLoader('./xczphadd.vue'),
    },
    methods: {
        closeComList() {
            this.comdrawer = false;
            this.search();
        },
        // 设置参会职位
        showComJob() {
            var that = this
            that.job_arr = []
            httpPost('m=neirong&c=zhaopinhui&a=getjoblist', { comid: that.curr_comdata.uid }).then(function(response) {
                if (response.data.error == 0) {
                    if (response.data.data.length > 0) {
                        that.jobids = that.info.jobid_arr
                        that.job_arr = response.data.data
                        that.drawercomjob = true
                    } else {
                        message.error(lc('admin_00295'));
                    }
                } else {
                    message.error(lc('admin_00295'));
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        // 设置参会职位 保存
        saveComJob() {
            var that = this
            var params = { zcomid: that.curr_comdata.id, zphjob: that.jobids.join(',') }
            httpPost('m=neirong&c=zhaopinhui&a=upjob', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function() {
                        that.drawercomjob = false
                        that.info.jobid_arr = that.jobids
                        that.getComList()
                        that.getList()
                    });
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        ydCls(item, childit) {
            var rt = ['yd_zt']
            if (childit.comstatus == '-1') {
                rt.push('yd_ztkyd')
            } else if (childit.comstatus == '1') {
                rt.push('yd_ztyyd')
            } else if (childit.comstatus == '0') {
                rt.push('yd_ztshz')
            } else if (childit.comstatus == '2' || childit.comstatus == '3') {
                rt.push('yd_ztbkyd')
            }
            if (childit.comstatus == '-1' && this.sel_zwid == childit.id) {
                rt.push('yd_ztkyd_active")
            }
            if (this.sel_zwid == childit.id) {
                this.sel_zwname = childit.name
            }
            return rt
        },
        // {{ lc('admin_00838') }}
        setZw() {
            var that = this
            httpPost("m=neirong&c=zhaopinhui&a=comadd', { id: that.curr_data.id }).then(function(response) {
                if (response.data.error == 0) {
                    that.space_list = response.data.data.spacelist
                    that.sel_zwid = that.curr_comdata.bid
                    that.sel_cdid = that.curr_comdata.cid
                    that.drawersetzw = true
                } else {
                    message.error(lc('admin_user_company_00017'));
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        // 设置展位 保存
        saveChangeZw() {
            var that = this
            var params = { zcomid: that.curr_comdata.id, cid: that.sel_cdid, bid: that.sel_zwid }
            httpPost('m=neirong&c=zhaopinhui&a=upzhanwei', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function() {
                        that.drawersetzw = false
                        that.curr_comdata.bid = that.sel_zwid
                        that.curr_comdata.cid = that.sel_cdid
                        that.getComList()
                        that.getList()
                    });
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        saveAddCom() {
            var that = this
            if (!that.comid) {
                message.error(lc('admin_vue_00075'));
                return false
            }
            if (!that.sel_cdid || !that.sel_zwid) {
                message.error(lc('admin_vue_00076'));
                return false
            }
            this.submitLoading = true
            httpPost('m=neirong&c=zhaopinhui&a=comaddsave', {
                comid: that.comid,
                zphid: that.curr_data.id,
                zphsid: that.curr_data.sid,
                cid: that.sel_cdid,
                bid: that.sel_zwid,
                jobid: that.jobids.join(',')
            }).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function() {
                        that.drawercomadd = false
                        that.getComList()
                        that.getList()
                    });
                } else {
                    message.error(lc('admin_00295'));
                }
            }).catch(function(error) {
                console.log(error);
            }).finally(function() {
                that.submitLoading = false;
            });
        },
        // 选择展位
        changezw(childit, item, event) {
            if (childit.comstatus != '-1') {
                message.error(lc('admin_00296'));
                return false
            }
            this.sel_zwid = childit.id
            this.sel_cdid = item.id
            this.sel_zwname = childit.name
            var curr_active = document.getElementsByClassName('yd_ztkyd_active')
            for (let i = 0; i < curr_active.length; i++) {
                curr_active[i].classList.remove('yd_ztkyd_active')
            }
            event.currentTarget.classList.add('yd_ztkyd_active')
        },
        // 添加参会企业选择企业
        comChange(data) {
            var that = this
            var selOption = this.com_arr.find((item) => item.value === data)
            this.sel_comname = selOption.label
            that.job_arr = []
            that.jobids = []
            httpPost('m=neirong&c=zhaopinhui&a=getjoblist', { comid: selOption.value }).then(function(response) {
                if (response.data.error == 0) {
                    if (response.data.data.length > 0) {
                        that.job_arr = response.data.data
                    } else {
                        message.error(lc('admin_00295'));
                    }
                } else {
                    message.error(lc('admin_00295'));
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        // 添加招聘会参会企业获取展位信息
        comadd() {
            var that = this
            that.comid = ''
            that.com_arr = []
            that.space_list = []
            that.sel_comname = ''
            that.sel_zwid = ''
            that.sel_cdid = ''
            that.sel_zwname = lc('admin_00304')
            that.job_arr = []
            that.jobids = []
            httpPost('m=neirong&c=zhaopinhui&a=comadd', { id: that.curr_data.id }).then(function(response) {
                if (response.data.error == 0) {
                    that.space_list = response.data.data.spacelist
                    that.drawercomadd = true
                } else {
                    message.error(lc('admin_user_company_00017'));
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        // 添加参会企业搜索企业
        getComArr(query) {
            var that = this
            if (query !== '') {
                setTimeout(() => {
                    httpPost('m=neirong&c=zhaopinhui&a=getcomlist', { comname: query }).then(function(response) {
                        if (response.data.error == 0) {
                            that.com_arr = response.data.data
                        } else {
                            message.error(lc('admin_user_company_00017'));
                        }
                    }).catch(function(error) {
                        console.log(error);
                    })
                }, 200);
            } else {
                this.options = [];
            }
        },
        // 参会企业批量审核
        multipleComStatus: function() {
            if (!this.comSelectedItem.length) {
                message.error(lc('admin_00246'));
                return false;
            }
            this.drawercomstatusmultiple = true
        },
        // 参会企业批量审核提交
        multipleComStatusSave() {
            var that = this
            if (!that.comSelectedItem.length) {
                message.error(lc('admin_00246'));
                return false;
            }
            that.comstatus({
                pid: that.comSelectedItem.join(','),
                status: that.multiComStatus,
                status_body: that.multiComStatusBody
            }, 2);
        },
        comStatusSave(id) {
            this.comstatus({ pid: id, status: this.info.zph.status, statusbody: this.info.zph.statusbody }, 1);
        },
        comstatus(params, tp) {
            var that = this
            this.submitLoading = true;
            httpPost('m=neirong&c=zhaopinhui&a=status', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function() {
                        that.getComList()
                    });
                } else {
                    message.error(lc('admin_user_company_00017'));
                }
            }).catch(function(error) {
                console.log(error);
            }).finally(function() {
                that.submitLoading = false;
            });
            if (tp == 1) {
                that.dtlislook = false
                that.comdrawersh = false
            } else {
                that.drawercomstatusmultiple = false
            }
        },
        cominfo(data) {
            var that = this
            that.curr_comdata = data
            httpPost('m=neirong&c=zhaopinhui&a=audit', { id: this.curr_comdata.id }).then(function(response) {
                if (response.data.error == 0) {
                    that.info = response.data.data
                    that.dtlislook = true
                    that.comdrawersh = true
                } else {
                    message.error(lc('admin_user_company_00017'));
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        showComList(data, status) {
            this.curr_data = data
            this.comdrawer = true
            this.comTblKey = Math.random()
            if (status) {
                this.status = '' + status
            } else {
                this.status = ''
            }
            this.comSearch()
        },
        addzph(data) {

            let that = this;
            httpPost('m=neirong&c=zhaopinhui&a=add', {}).then(function(response) {
                var res = response.data
                if (data.id) {
                    that.curr_data = JSON.parse(JSON.stringify(data));
                } else {
                    that.curr_data = { id: '' }
                }
                that.editdrawer = true
                that.$nextTick(function() {
                    that.$refs.zphadd.reserved_arr = [];
                    that.$refs.zphadd.initEditor();
                    that.spacearr = res.data.space
                    if (data.sid) {
                        that.$refs.zphadd.cdChange(data.sid)
                    }
                })
            }).catch(function(error) {
                console.log(error);
            }).finally(function() {
                that.submitLoading = false;
            });
        },
        statusSet(data) {
            this.curr_data = {
                id: data.id,
                title: data.title,
                is_open: data.is_open,
            };
            this.drawerstatus = true
        },
        statusSave() {
            var that = this
            var params = {
                pid: that.curr_data.id,
                is_open: that.curr_data.is_open
            }
            this.submitLoading = true;
            httpPost('m=neirong&c=zhaopinhui&a=upisopen', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, that.getList());
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            }).finally(function() {
                that.submitLoading = false;
            });
            that.drawerstatus = false
        },
        piclist(data) {
            var that = this
            that.curr_data = data
            that.zphdrawerimg = true
            that.$nextTick(function() {
                that.$refs.zphimg.getList()
            })
        },
        fp(data) {
            this.curr_data = JSON.parse(JSON.stringify(data));
            this.curr_data.did = '' + this.curr_data.did
            this.drawerfp = true
        },
        fpAllBottom() {
            if (!this.selectedItem.length) {
                message.error(lc('admin_00312'));
                return false;
            }
            this.multipledid = '"
            this.drawerfpmultiple = true
        },
        fpSave(tp) {
            var that = this
            let params = {}
            if (tp == 1) { // {{ lc('admin_user_weipin_00029') }}
                params.uid = that.curr_data.id
                params.did = that.curr_data.did
            } else { // {{ lc('admin_00162') }}
                if (!that.selectedItem.length) {
                    message.error(lc('admin_00312'));
                    return false;
                }
                params.uid = that.selectedItem.join(',')
                params.did = that.multipledid
            }
            this.submitLoading = true;
            httpPost('m=neirong&c=zhaopinhui&a=checksitedid', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, that.getList());
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            }).finally(function() {
                that.submitLoading = false;
            });
            if (tp == 1) {
                that.drawerfp = false
            } else {
                that.drawerfpmultiple = false
            }
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
            this.getList()
        },
        handleCurrentChange(val) {
            this.currentPage = val;
            this.getList()
        },
        sortChange: function(column) {
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
        async getList() {
            let that = this;
            let params = {
                page: that.currentPage,
                pageSize: that.perPage
            }
            if (that.keyword) {
                params.keyword = that.keyword
            }
            if (that.status) {
                params.status = that.status
            }
            if (that.type) {
                params.type = that.type
            }
            if (that.sort_type && that.sort_col) {
                params.order = that.sort_type
                params.t = that.sort_col
            }
            that.loading = true;
            that.emptytext = lc('admin_user_weipin_00026');
            httpPost('m=neirong&c=zhaopinhui&a=index', params, { hideloading: true }).then(function(result) {
                var res = result.data
                if (res.error == 0) {
                    that.tableData = res.data.list
                    that.perPage = parseInt(res.data.perPage)
                    that.pageSizes = res.data.pageSizes
                    that.total = parseInt(res.data.total)
                    if (that.prevPage != that.currentPage) {
                        that.prevPage = that.currentPage;
                        that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    that.loading = false;
                    if (that.tableData.length === 0) {
                        that.emptytext = lc('wap_js_00113');
                    }
                }
            }).catch(function(e) {
                console.log(e)
            })
        },
        async getGroup() {
            let that = this;
            httpPost('m=neirong&c=zhaopinhui&a=getGroup', {}, { hideloading: true }).then(function(result) {
                var res = result.data
                if (res.error == 0) {
                    that.dnamearr = res.data.Dname
                    that.preview_url = res.data.preview_url
                    that.spacearr = res.data.space
                }
            }).catch(function(e) {
                console.log(e)
            })
        },
        delrow(id) {
            delConfirm(this, id, this.delete);
        },
        delAllBottom() {
            if (!this.selectedItem.length) {
                message.error(lc('admin_00136'));
                return false;
            }
            delConfirm(this, this.selectedItem, this.delete);
        },
        // 导出excel
        exportExcel() {
            var that = this
            if (that.comTableData.length == 0) {
                message.error(lc('admin_00311'));
                return false
            }
            message.confirm(lc('admin_00313'), function() {
                var params = {
                    zid: that.curr_data.id,
                    cid: that.comSelectedItem.join(',')
                }
                httpPost('m=neirong&c=zhaopinhui&a=comxlscheck', params).then(function(response) {
                    let res = response.data;
                    if (res.error > 0) {
                        message.error(res.msg);
                    } else {
                        httpPost('m=neirong&c=zhaopinhui&a=comxls', params).then(function(response2) {
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
            })
        },
        async delete(id) {
            let that = this;
            let params = {
                del: id
            };
            httpPost('m=neirong&c=zhaopinhui&a=del', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg);
                    that.getList();
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            })
        },

        handleComSelectionChange(val) {
            this.comSelectedItem = [];
            let _this = this;
            if (val.length) {
                val.forEach(item => {
                    _this.comSelectedItem.push(item.id);
                });
            }
            if (_this.comSelectedItem.length == 0) {
                _this.comCheckedAll = false;
            } else {
                if (_this.comSelectedItem.length == _this.comTableData.length) {
                    _this.comCheckedAll = true;
                } else {
                    _this.comCheckedAll = false;
                }
            }
        },
        comSelectAllBottom(value) {
            value ? this.$refs.comMultipleTable.toggleAllSelection() : this.$refs.comMultipleTable.clearSelection();
        },
        handleComSizeChange(val) {
            this.comPerPage = val;
            this.getComList()
        },
        handleComCurrentChange(val) {
            this.comCurrentPage = val;
            this.getComList()
        },
        comSortChange: function(column) {
            if (column.order == 'descending') {
                this.sort_type = 'desc';
            } else if (column.order == 'ascending') {
                this.sort_type = 'asc';
            } else {
                this.sort_type = '';
            }
            this.sort_col = column.prop
            if (this.sort_col == 'space_n') {
                this.sort_col = 'sid'
            }
            this.comSearch();
        },
        comSearch() {
            this.comCurrentPage = 1;
            this.getComList();
        },
        async getComList() {
            let that = this;
            let params = {
                type: 2,
                page: that.comCurrentPage,
                pageSize: that.comPerPage
            }
            if (that.comkeyword) {
                params.keyword = that.comkeyword
            }
            if (that.com_status) {
                params.status = that.com_status
            }
            if (that.curr_data.id) {
                params.id = that.curr_data.id
            }
            if (that.sort_type && that.sort_col) {
                params.order = that.sort_type
                params.t = that.sort_col
            }
            that.loading = true;
            that.emptytext = lc('admin_user_weipin_00026');
            httpPost('m=neirong&c=zhaopinhui&a=com', params).then(function(result) {
                var res = result.data
                if (res.error == 0) {
                    that.comTableData = res.data.list
                    that.comPerPage = parseInt(res.data.perPage)
                    that.comPageSizes = res.data.pageSizes
                    that.comTotal = parseInt(res.data.total)
                    that.comTblKey = Math.random()
                    // that.dnamearr = res.data.Dname
                    // that.preview_url = res.data.preview_url
                    // that.spacearr = res.data.space
                    if (that.prevPage1 != that.comCurrentPage) {
                        that.prevPage1 = that.comCurrentPage;
                        that.$refs.comMultipleTable.bodyWrapper.scrollTop = 0;
                    }
                    that.loading = false;
                    if (that.comTableData.length === 0) {
                        that.emptytext = lc('wap_js_00113');
                    }
                }
            }).catch(function(e) {
                console.log(e)
            })
        },
        comdelrow(id) {
            delConfirm(this, id, this.comDelete);
        },
        comDelAllBottom() {
            if (!this.comSelectedItem.length) {
                message.error(lc('admin_00136'));
                return false;
            }
            delConfirm(this, this.comSelectedItem, this.comDelete);
        },
        async comDelete(id) {
            let that = this;
            let params = {
                del: id
            };
            httpPost('m=neirong&c=zhaopinhui&a=delcom', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg);
                    that.getComList();
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        editData(scope) {
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            this.oldData = JSON.parse(JSON.stringify(row));
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = true;
            this.$set(this.comTableData, index, copyRow);
            this.$nextTick(() => {
                let ref = column.property + index;
                $("#" + ref).focus();
            });
        },
        alterData(scope) {
            if (this.oldData == null) {
                return false;
            }
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = false;
            this.$set(this.comTableData, index, copyRow);
            if (row[column.property] === this.oldData[column.property]) {
                return false;
            }
            let _this = this;
            let sendData = { id: row.id };
            sendData[column.property] = row[column.property];
            httpPost('m=neirong&c=zhaopinhui&a=ajaxsort', sendData, { hideloading: true }).then(function(response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(lc('admin_user_company_00208'));
                } else {
                    message.error(lc('admin_00187'));
                }
                _this.oldData = null;
                _this.getList();
            }).catch(function(error) {
                console.log(error);
            });
        },
        addPlace: function($param) {
            this.editdrawer = false
            custoapp.activeName = 'cd';
            custoapp.$refs['cd'].getList()
        },
        handleCloseList() {
            this.editdrawer = false;
            this.getList();
        },
    },

};
</script>
<style>
.el-dialog__body {
    padding: 0px 20px;
}

.wxsettip_small {
    padding: 20px 0
}

.yd_ztkyd_active {
    background-color: #409eff;
    color: #fff;
}

.admin_web_tip {
    display: inline-block;
    line-height: 16px;
    padding-left: 20px;
    color: #999;
    font-size: 12px;
    padding-top: 8px;
}
</style>