<template>
    <div class="moduleElHight">
        <div class="moduleElSearchInf">
            <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                <div class="moduleInptList moduleInptWidt">
                    <el-input placeholder="{yun:}t key='admin_user_weipin_00003'{/yun}" @keyup.enter.native="search" size="small" v-model="searchForm.keyword" clearable class="input-with-select">
                        <el-select v-model="searchForm.type" slot="prepend" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                            <el-option label="职位名称" :value="2"></el-option>
                            <el-option label="联系电话" :value="3"></el-option>
                            <el-option label="联系人" :value="4"></el-option>
                            <el-option label="店铺名" :value="5"></el-option>
                        </el-select>
                    </el-input>
                </div>
                <div v-for="(searchItem, searchIndex) in searchList" :key="searchIndex" class="moduleInptList">
                    <el-select size="small" v-model="searchForm[searchItem.param]" slot="prepend" :clearable="true"
                        :placeholder="searchItem.name" @change="search">
                        <el-option v-for="(searchLabel, searchValue) in searchItem.value" :key="searchValue" :label="searchLabel"
                            :value="searchValue"></el-option>
                    </el-select>
                </div>
                <div class="moduleInptList">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                </div>
            </div>
            <div class="moduleSeachButn moduleSeachMart">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="openAdd('')">{yun:}t key='admin_user_weipin_00042'{/yun}</el-button>
            </div>
        </div>
        <div class="admin_datatip">
            <i class="el-icon-document"></i>
            {{ lc("admin_data_stats") }} <span @click="init">{{ lc("admin_total_count", [onceAllNum]) }}</span>
            <span class="admin_datatip_n"><span @click="statusSearch('3')">{{ lc("admin_pending_review_count", [onceStatusNum1 ? onceStatusNum1 :
                0]) }}</span> </span>
            <span class="admin_datatip_n"><span @click="statusSearch('2')">{{ lc("admin_expired_count", [onceStatusNum2 ? onceStatusNum2 :
                0]) }}</span></span>
            <span class="admin_datatip_n">{{ lc("admin_search_results_count", [total]) }}</span>
        </div>
        <div class="moduleElTable" style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 135px);">
            <el-table :data="list" border style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
                @sort-change="sortChange" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" label="编号" width="90" sortable="custom">
                </el-table-column>
                <el-table-column label="职位名称/店铺名" min-width="180">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span><el-link :underline="false" @click="openPage(scope.row.once_url)" type="primary">{{
                                scope.row.title }}</el-link></span>
                            <span>{{ scope.row.companyname }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="店铺形象" width="140" align="center">
                    <template slot-scope="scope">
                        <div class="moduleProps" style="display: flex; align-items: center; justify-content: center;">
                            <el-image v-if="scope.row.pic" :src="scope.row.pic_n" style="width: 50px; height: 50px">
                            </el-image>
                            <span v-else>{yun:}t key='common_02082'{/yun}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="phone" label="联系电话" width="140">
                </el-table-column>
                <el-table-column prop="linkman" label="联系人" width="140">
                </el-table-column>
                <el-table-column prop="ctime" label="发布时间" width="140" sortable="custom">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.status == 2" class="admin_state3">{{ scope.row.ctime_n }}</span>
                            <span v-else>{{ scope.row.ctime_n }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="edate" label="结束时间" width="140" sortable="custom">
                    <template slot-scope="scope">
                        <div>{{ scope.row.edate_n }}</div>
                    </template>
                </el-table-column>
                <el-table-column label="状态" width="120">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class="admin_state">
                                <span v-if="scope.row.status == 1" class="admin_state1">{yun:}t key='wap_user_00165'{/yun}</span>
                                <span v-else-if="scope.row.status == 2" class="admin_state2">{yun:}t key='member_com_00304'{/yun}</span>
                                <span v-else-if="scope.row.status == 0" class="admin_state5">{yun:}t key='wap_user_00166'{/yun}</span>
                            </div>
                            <div v-if="scope.row.pay > 0" class="admin_state">
                                <span v-if="scope.row.pay == 1" class="admin_state5">{yun:}t key='wap_00359'{/yun}</span>
                                <span v-else-if="scope.row.pay == 2" class="admin_state1">{yun:}t key='admin_user_weipin_00045'{/yun}</span>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="站点" width="140">
                    <template slot-scope="scope">
                        <div class="moduleProps moduleTrButn">
                            <span>{{ domainList[scope.row.did] }}</span>
                            <el-button type="text" @click="openDomain(scope.row)">{yun:}t key='admin_user_weipin_00048'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" label="操作" width="140">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <el-button size="mini" style="margin-right: 8px;" @click="openAudit(scope.row)">{yun:}t key='member_user_00152'{/yun}</el-button>
                            <el-popover placement="bottom" width="60" trigger="hover">
                                <div class="moduleMores">
                                    <el-button size="mini" @click="openDetail(scope.row)">{yun:}t key='wap_com_00427'{/yun}</el-button>
                                    <el-button size="mini" @click="openAdd(scope.row)">{yun:}t key='wap_js_00073'{/yun}</el-button>
                                    <el-button size="mini" @click="refresh(scope.row)">{yun:}t key='wap_user_00334'{/yun}</el-button>
                                    <el-button size="mini" @click="del(scope.$index)">{yun:}t key='common.delete'{/yun}</el-button>
                                </div>
                                <el-button size="mini" slot="reference">{yun:}t key='common.more'{/yun}</el-button>
                            </el-popover>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
                    @change="checkAll">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button @click="batch('del')" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
                <el-button @click="batch('audit')" size="mini">{yun:}t key='admin_user_weipin_00037'{/yun}</el-button>
                <el-button @click="batch('audit_cancel')" size="mini">{yun:}t key='admin_user_weipin_00018'{/yun}</el-button>
                <el-button @click="batch('extension')" size="mini">{yun:}t key='admin_user_weipin_00038'{/yun}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page="page" :page-sizes="pageSizes" :page-size="limit"
                    layout="total, sizes, prev, pager, next, jumper" :total="total" :pager-count="pagerCount">
                </el-pagination>
            </div>
        </div>
        <!-- 弹窗 -->
        <div class="modluDrawer">
            <el-drawer title="{yun:}t key='admin_user_weipin_00032'{/yun}" :visible.sync="dialogAudit" :with-header="true" :modal-append-to-body="false"
                :show-close="true" size="640px">
                <div class="tableTancyans">
                    <div class="toolClasDia">
                        <div class="zhaopTacOne">
                            <div class="zhaopTacName">{yun:}t key='admin_user_weipin_00039'{/yun}</div>
                            <div class="zhaopTacTwo">
                                <table border="1">
                                    <tbody>
                                        <tr>
                                            <td width="25%">
                                                <div class="TableTite">{yun:}t key='wap_com_00288'{/yun}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.title }}</div>
                                            </td>
                                            <td width="25%">
                                                <div class="TableTite">{yun:}t key='admin_user_weipin_00044'{/yun}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.salary }}</div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <div class="TableTite">{yun:}t key='wap_00351'{/yun}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.companyname }}</div>
                                            </td>
                                            <td>
                                                <div class="TableTite">{yun:}t key='wap_00347'{/yun}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">
                                                    <el-image :src="detail.yyzz_n"
                                                        style="max-width: 200px; max-height: 200px"
                                                        :preview-src-list="[detail.yyzz_n]">
                                                    </el-image>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <div class="TableTite">{yun:}t key='admin_user_weipin_00030'{/yun}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.ctime_n }}</div>
                                            </td>
                                            <td>
                                                <div class="TableTite">{yun:}t key='member_com_00315'{/yun}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.edate_n }}</div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <div class="TableTite">{yun:}t key='wap_00374'{/yun}</div>
                                            </td>
                                            <td colspan="3">
                                                <div class="TableData">{{ detail.require }}</div>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                                <!-- <div class="tableTancBito">
                                    <span>职位名称：{{ detail.title }}</span>
                                    <span>工 资：{{ detail.salary }}</span>
                                </div>
                                <div class="tableTancBito">
                                    <span>店面名称：{{ detail.companyname }}</span>
                                </div>
                                <div v-if="detail.yyzz" class="tableTancBito">
                                    <span>店面营业执照：
                                        <el-image :src="detail.yyzz_n" style="max-width: 200px; max-height: 200px"
                                            :preview-src-list="[detail.yyzz_n]">
                                        </el-image>
                                    </span>
                                </div>
                                <div class="tableTancBito">
                                    <span>具体要求：{{ detail.require }}</span>
                                </div>
                                <div class="tableTancBito">
                                    <span>发布时间：{{ detail.ctime_n }}</span>
                                    <span>有效期： {{ detail.edate_n }}</span>
                                </div> -->
                            </div>
                        </div>
                        <div class="zhaopTacOne">
                            <div class="zhaopTacName">{yun:}t key='wap_00462'{/yun}</div>
                            <div class="zhaopTacTwo">
                                <table border="1">
                                    <tbody>
                                        <tr>
                                            <td width="25%">
                                                <div class="TableTite">{yun:}t key='wap_user_00265'{/yun}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.phone }}</div>
                                            </td>
                                            <td width="25%">
                                                <div class="TableTite">{yun:}t key='wap_01431'{/yun}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.linkman }}</div>
                                            </td>
                                        </tr>

                                        <tr>
                                            <td>
                                                <div class="TableTite">{yun:}t key='member_user_00198'{/yun}</div>
                                            </td>
                                            <td colspan="3">
                                                <div class="TableData">{{ detail.address }}</div>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>

                                <!-- <div class="tableTancBito">
                                    <span>{{ lc("admin_contact_phone_value", [detail.phone]) }}</span>
                                    <span>{{ lc("admin_contact_person_value", [detail.linkman]) }}</span>
                                </div>
                                <div class="tableTancBito">
                                    <span>工作地点：{{ detail.address }}</span>
                                </div> -->
                            </div>
                        </div>
                        <div class="toolClasList">
                            <div class="toolClasTite">
                                <span>{yun:}t key='admin_user_weipin_00022'{/yun}</span>
                            </div>
                            <div class="toolClasCont">
                                <el-radio v-model="ruleFormAudit.status" :label="1">{yun:}t key='wap_user_00165'{/yun}</el-radio>
                                <el-radio v-model="ruleFormAudit.status" :label="2">{yun:}t key='admin_user_weipin_00008'{/yun}</el-radio>
                            </div>
                        </div>
                    </div>
                    <div class="dialogFooter">
                        <!-- <el-button @click="dialogAudit = false">取 消</el-button> -->
                        <el-button type="primary" @click="saveAudit">{yun:}t key='wap_com_00019'{/yun}</el-button>
                    </div>
                </div>
            </el-drawer>
            <el-dialog title="{yun:}t key='admin_user_weipin_00038'{/yun}" :visible.sync="dialogExtension" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="350px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_user_weipin_00025'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input v-model="ruleFormExtension.endtime" placeholder=""
                                @input="inputIntNumber($event, 'ruleFormExtension', 'endtime')">
                                <template slot="append">{yun:}t key='common_02067'{/yun}</template>
                            </el-input>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogExtension = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="saveExtension">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_user_weipin_00029'{/yun}" :visible.sync="dialogDomain" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="500px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_user_weipin_00035'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <span>{{ detail.companyname }}</span>
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
        </div>
        <div class="modluDrawer">
            <el-drawer title="{yun:}t key='admin_user_weipin_00040'{/yun}" :visible.sync="drawerDetail" :modal-append-to-body="false" size="640px">
                <div class="tableTancyans">
                    <div class="zhaopTacTwo">
                        <table border="1">
                            <tbody>
                                <tr>
                                    <td width="25%">
                                        <div class="TableTite">{yun:}t key='wap_com_00288'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.title }}</div>
                                    </td>
                                    <td width="25%">
                                        <div class="TableTite">{yun:}t key='admin_user_weipin_00034'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.companyname }}</div>
                                    </td>
                                </tr>

                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_user_weipin_00041'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.edate_n }}</div>
                                    </td>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_user_weipin_00044'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.salary }}</div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_user_weipin_00027'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.linkman }}</div>
                                    </td>
                                    <td>
                                        <div class="TableTite">{yun:}t key='wap_user_00265'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.phone }}</div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='wap_00349'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.city_n }}</div>
                                    </td>
                                    <td>
                                        <div class="TableTite">{yun:}t key='member_user_00198'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.address }}</div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_user_weipin_00017'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">
                                            <el-image :src="detail.yyzz_n" style="max-width: 200px; max-height: 200px"
                                                :preview-src-list="[detail.yyzz_n]">
                                            </el-image>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="TableTite">{yun:}t key='wap_00362'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">
                                            <el-image :src="detail.pic_n" style="max-width: 200px; max-height: 200px"
                                                :preview-src-list="[detail.pic_n]">
                                            </el-image>
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='wap_00374'{/yun}</div>
                                    </td>
                                    <td colspan="3">
                                        <div class="TableData">
                                            <div v-html="detail.require_n"></div>
                                        </div>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    <div class="drawerModLis" style="justify-content: initial;">
                        <div class="drawerModTite" style="width: initial;">
                            <span>{yun:}t key='wap_com_00320'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <div class="admin_state">
                                <span v-if="detail.status == 1" class="admin_state1">{yun:}t key='wap_user_00165'{/yun}</span>
                                <span v-else-if="detail.status == 2" class="admin_state2">{yun:}t key='member_com_00304'{/yun}</span>
                                <span v-else-if="detail.status == 0" class="admin_state5">{yun:}t key='wap_user_00166'{/yun}</span>
                            </div>
                        </div>
                    </div>
                </div>
                
            </el-drawer>
            <el-drawer title="{yun:}t key='wap_js_00130'{/yun}" :visible.sync="drawerAdd" :modal-append-to-body="false" size="40%">
                <div class="moduleSchools">
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_com_00288'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.title" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_user_weipin_00034'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.companyname" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_user_weipin_00010'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.edate" placeholder=""
                                @input="inputIntNumber($event, 'ruleFormAdd', 'edate')" maxlength="3"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_user_weipin_00027'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.linkman" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_user_00265'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.phone" placeholder=""
                                @input="inputIntNumber($event, 'ruleFormAdd', 'phone')"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_user_weipin_00044'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.salary" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_00349'{/yun}</span>
                        </div>
                        <div class="drawerModInpt" style="display: flex; align-items: center;">
                            <el-select v-model="ruleFormAdd.provinceid" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="changeCity($event, 1)">
                                <el-option v-for="item in provinceList" :key="item.id" :label="item.name" :value="item.id">
                                </el-option>
                            </el-select>
                            <el-select v-model="ruleFormAdd.cityid" placeholder="{yun:}t key='wap_user_00100'{/yun}" style="margin: 0 10px;"
                                @change="changeCity($event, 2)">
                                <el-option v-for="item in cityList" :key="item.id" :label="item.name" :value="item.id">
                                </el-option>
                            </el-select>
                            <el-select v-model="ruleFormAdd.three_cityid" placeholder="{yun:}t key='wap_user_00100'{/yun}" clearable>
                                <el-option v-for="item in regionList" :key="item.id" :label="item.name" :value="item.id">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='member_user_00198'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.address" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_user_weipin_00017'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-upload class="upload-demo" :accept="pic_accept" list-type="picture" action=""
                                :auto-upload="false" :on-change="handleChangeYyzz" :show-file-list="false">
                                <el-button slot="trigger" size="small" type="primary"
                                    icon="el-icon-document-add">{yun:}t key='wap_00540'{/yun}</el-button>
                                <img class="el-upload-list__item-thumbnail"
                                    style="padding-left: 5px;max-width: 120px;max-height: 120px;" v-if="ruleFormAdd.yyzz_n"
                                    :src="ruleFormAdd.yyzz_n" />
                            </el-upload>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_00362'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-upload class="upload-demo" :accept="pic_accept" list-type="picture" action=""
                                :auto-upload="false" :on-change="handleChangePic" :show-file-list="false">
                                <el-button slot="trigger" size="small" type="primary"
                                    icon="el-icon-document-add">{yun:}t key='wap_00540'{/yun}</el-button>
                                <img class="el-upload-list__item-thumbnail"
                                    style="padding-left: 5px;max-width: 120px;max-height: 120px;" v-if="ruleFormAdd.pic_n"
                                    :src="ruleFormAdd.pic_n" />
                            </el-upload>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_user_weipin_00047'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input type="textarea" :rows="2" placeholder="" v-model="ruleFormAdd.require">
                            </el-input>
                        </div>
                    </div>
                    <div class="drawerModLis" style="align-items: initial;">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_user_00371'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.password" placeholder="" show-password
                                @input="inputPassword($event, 'ruleFormAdd', 'password')"></el-input>
                            <el-alert title="{yun:}t key='admin_user_weipin_00007'{/yun}" :closable="false" type="info" show-icon>
                            </el-alert>
                        </div>
                    </div>
                    <div class="setBasicButn" style="border: none;">
                        <el-button type="primary" size="medium" @click="saveAdd" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
                    </div>
                </div>
            </el-drawer>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props: {
        status: { type: String, default: '' }
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            loading: false,
			pagerCount: 5,
            dataText: "{yun:}t key='admin_user_weipin_00026'{/yun}",
            // 搜索筛选项
            searchList: [],
            searchForm: {
                type: 2,
                status: this.status
            },

            // list
            page: 1,
            limit: 0,
            list: [],
            total: 0,
            pageSizes: [],

            // 列表排序
            t: '',
            order: '",

            checkedAll: false, // {yun:}t key='wap_js_00074'{/yun}
            checkedAllIndeterminate: false,
            multipleSelection: [], // 多选值存储
            idArr: [],

            detail: {},

            onceStatusNum1: 0,
            onceStatusNum2: 0,
            onceAllNum: 0,

            saveLoading: false,

            // 分站切换
            dialogDomain: false,
            ruleFormDomain: {},
            domainList: {},

            // Audit
            dialogAudit: false,
            ruleFormAudit: {},

            // {yun:}t key='admin_user_weipin_00038'{/yun}
            dialogExtension: false,
            ruleFormExtension: {},

            // {yun:}t key='wap_com_00427'{/yun}
            drawerDetail: false,

            // {yun:}t key='wap_js_00091'{/yun}/{yun:}t key='common.edit'{/yun}
            drawerAdd: false,
            ruleFormAdd: {},
            provinceList: [],
            cityList: [],
            regionList: [],

            prevPage: 0
        }
    },

    mounted() {
        var that = this
        setTimeout(function () {
            that.getCacheFun();
        }, 200)
    },
    created() {
        this.init();
    },
    methods: {
        init() {
            // this.resetSearch();
            this.getCountData();
            this.search();
        },
        getCacheFun:function(){
            let that = this;
            httpPost("m=user&c=weipin_once&a=getCache', {},{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.searchList = res.data.search_list;
                    that.domainList = res.data.dname;
                }
            })
        },
        getDnameFun:function(){
            let that = this;
            httpPost('m=common&c=cache&a=getDname', {},{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.domainList = res.data.Dname
                }
            })
        },
        resetSearch() {
            this.searchForm = {
                type: 2
            };
            this.limit = 0;
        },

        statusSearch(status) {
            this.searchForm.status = status;
            this.search();
        },

        getCountData() {
            let that = this;

            httpPost('m=user&c=weipin_once&a=onceNum', {}, { hideloading: true }).then(function (response) {
                let res = response.data;

                that.onceStatusNum1 = res.onceStatusNum1;
                that.onceStatusNum2 = res.onceStatusNum2;
                that.onceAllNum = res.onceAllNum;
            })
        },

        handleSizeChange(val) {
            this.limit = val;
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
            httpPost('m=user&c=weipin_once', { ...params, ...searchForm }, {hideloading: true}).then(function (response) {
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
                if(that.prevPage != that.page){
                    that.prevPage = that.page;
                    that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
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
            }else if (this.multipleSelection.length == 0){
                message.error("{yun:}t key='admin_user_weipin_00001'{/yun}");
                return false;
            }

            let idArr = [];
            this.multipleSelection.forEach(function (item) {
                idArr.push(item.id);
            })
            this.idArr = idArr;

            if (type == 'del') {
                this.del();
            } else if (type == 'audit') {
                delConfirm(this, null, function (params) {
                    that.ruleFormAudit = {
                        id: that.idArr,
                        status: 1,
                    };
                    that.saveAudit();
                }, "{yun:}t key='admin_user_weipin_00011'{/yun}")
            } else if (type == 'audit_cancel') {
                delConfirm(this, null, function (params) {
                    that.ruleFormAudit = {
                        id: that.idArr,
                        status: 0,
                    };
                    that.saveAudit();
                }, "{yun:}t key='admin_user_weipin_00004'{/yun}")
            } else if (type == 'extension') {
                this.openExtension();
            }
        },
        checkAll(val) {
            val ? this.checkedAllIndeterminate = false : '';
            this.$refs.multipleTable.toggleAllSelection();
        },

        del(idx) {
            let that = this,
                params = {},
                msg = '';

            if (typeof idx == 'undefined") { // {yun:}t key='member_com_00055'{/yun}
                params.del = this.idArr;
                msg = "你确定要删除选中项吗？";
            } else {// {yun:}t key='common_01711'{/yun}
                params.del = that.list[idx].id;
                msg = "你确定要删除当前项吗？';
            }

            delConfirm(this, params, function (params) {
                httpPost('m=user&c=weipin_once&a=del', params).then(function (res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        that.getList();
                        that.$refs.multipleTable.clearSelection();
                        message.success(res.data.msg);
                    }
                })
            }, msg)
        },

        openDomain(row) {
            if (typeof row == 'undefined") { // {yun:}t key='admin_yunying_00106'{/yun}
                this.detail = {};
                this.$set(this.ruleFormDomain, "id', this.idArr);
                this.$set(this.ruleFormDomain, 'did', '');
            } else { // 单个操作
                this.detail = row;
                this.$set(this.ruleFormDomain, 'id', row.id);
                this.$set(this.ruleFormDomain, 'did', row.did && this.domainList[row.did] ? '' + row.did : '');
            }
            this.dialogDomain = true;
        },

        saveDomain() {
            let that = this,
                ruleForm = that.ruleFormDomain;

            if (!ruleForm.did) {
                message.error("{yun:}t key='admin_user_weipin_00002'{/yun}");
                return false;
            }

            if (that.saveLoading) {
                return false;
            }

            that.saveLoading = true;

            httpPost('m=user&c=weipin_once&a=checksitedid', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    that.saveLoading = false;
                    message.error(res.msg);
                } else {
                    that.dialogDomain = false;
                    that.getList();
                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    })
                }
            })
        },

        openAudit(row) {
            let that = this;

            if (typeof row != 'undefined") { // 单个审核
                that.detail = row;
                that.ruleFormAudit = {
                    id: row.id,
                    status: row.status,
                };

                that.dialogAudit = true;
            } else { // {yun:}t key='admin_user_weipin_00037'{/yun}
                that.detail = {};
                that.ruleFormAudit = {
                    id: this.idArr,
                    status: "',
                };
            }
        },

        saveAudit() {
            let that = this,
                params = that.ruleFormAudit;

            if (params.status === '') {
                message.error("{yun:}t key='admin_user_weipin_00015'{/yun}");
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=weipin_once&a=status', params).then(function (response) {
                let res = response.data,
                    data = res.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogAudit = false;
                    that.getList();
                    that.$refs.multipleTable.clearSelection();
                    if (data.status === 3) {
                        message.error("{yun:}t key='admin_user_weipin_00023'{/yun}");
                    } else if (data.status === 1) {
                        message.success("{yun:}t key='admin_user_weipin_00031'{/yun}");
                    } else {
                        message.success("{yun:}t key='admin_user_weipin_00016'{/yun}");
                    }
                }
            })
        },

        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g, '');
        },
        inputPassword(val, form, key) {
            this.$data[form][key] = val.replace(/^ +| +$/g, '");
        },

        // {yun:}t key='admin_user_weipin_00038'{/yun}
        openExtension(row) {
            this.ruleFormExtension = {
                id: this.idArr,
                endtime: "',
            };

            this.dialogExtension = true;
        },

        saveExtension() {
            let that = this,
                params = that.ruleFormExtension;

            if (params.endtime === '') {
                message.error("{yun:}t key='admin_user_weipin_00013'{/yun}");
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=weipin_once&a=ctime', params).then(function (response) {
                let res = response.data,
                    data = res.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    that.dialogExtension = false;
                    that.getList();
                    that.$refs.multipleTable.clearSelection();
                    message.success(res.msg);
                }
            })
        },

        openDetail(row) {
            let that = this;

            httpPost('m=user&c=weipin_once&a=edit', { id: row.id }).then(function (response) {
                let res = response.data;

                that.detail = res.data.info;
                that.drawerDetail = true;
            })
        },

        getCity(level) {
            let that = this;

            httpPost('m=common&c=cache&a=getCity', {
                provinceid: that.ruleFormAdd.provinceid,
                cityid: that.ruleFormAdd.cityid,
                level: typeof level !== "undefined" ? level : ''
            }, { hideloading: true }).then(function (response) {
                let res = response.data,
                    data = res.data;

                if (typeof data.provinceList !== "undefined") {
                    that.provinceList = data.provinceList;
                }
                if (typeof data.cityList !== "undefined") {
                    that.cityList = data.cityList;
                }
                if (typeof data.regionList !== "undefined") {
                    that.regionList = data.regionList;
                }
            })
        },

        changeCity(val, level) {
            if (level == 1) {
                this.ruleFormAdd.provinceid = val;
                this.ruleFormAdd.cityid = '';
                this.ruleFormAdd.three_cityid = '';
            } else if (level == 2) {
                this.ruleFormAdd.cityid = val;
                this.ruleFormAdd.three_cityid = '';
            }
            this.getCity(level);
        },

        openAdd(row) {
            let that = this;

            if (row !== '') {
                httpPost('m=user&c=weipin_once&a=edit', { id: row.id }).then(function (response) {
                    let res = response.data,
                        info = res.data.info;

                    if (info) {
                        that.ruleFormAdd = {
                            id: info.id,
                            title: info.title,
                            companyname: info.companyname,
                            edate: info.day_n ? info.day_n : '',
                            linkman: info.linkman,
                            phone: info.phone,
                            salary: info.salary,
                            provinceid: info.provinceid && info.provinceid > 0 ? info.provinceid : '',
                            cityid: info.cityid && info.cityid > 0 ? info.cityid : '',
                            three_cityid: info.three_cityid && info.three_cityid > 0 ? info.three_cityid : '',
                            address: info.address,
                            require: info.require,
                            password: '',
                            pic_n: info.pic_n,
                            yyzz_n: info.yyzz_n
                        };
                    }

                    that.getCity();
                    that.drawerAdd = true;
                })
            } else {
                this.ruleFormAdd = {
                    provinceid: '',
                    cityid: '',
                    three_cityid: '',
                };
                that.getCity();
                this.drawerAdd = true;
            }
        },

        // 上传时触发
        handleChangePic(file, fileList) {
            this.$set(this.ruleFormAdd, 'file', file.raw);
            this.$set(this.ruleFormAdd, 'pic_n', file.url);
        },
        handleChangeYyzz(file, fileList) {
            this.$set(this.ruleFormAdd, 'yyzz', file.raw);
            this.$set(this.ruleFormAdd, 'yyzz_n', file.url);
        },

        saveAdd() {
            let that = this,
                ruleForm = that.ruleFormAdd,
                formData = new FormData();

            if (typeof ruleForm.title === 'undefined' || $.trim(ruleForm.title) == "") {
                message.error("{yun:}t key='admin_user_weipin_00014'{/yun}");
                return false;
            }
            if (typeof ruleForm.companyname === 'undefined' || $.trim(ruleForm.companyname) == "") {
                message.error("{yun:}t key='admin_user_weipin_00012'{/yun}");
                return false;
            }
            if (typeof ruleForm.edate === 'undefined' || $.trim(ruleForm.edate) == "") {
                message.error('请输入有效时间');
                return false;
            }
            if (typeof ruleForm.linkman === 'undefined' || $.trim(ruleForm.linkman) == "") {
                message.error("{yun:}t key='wap_com_00013'{/yun}");
                return false;
            }
            var phoneReg = /^[1][3456789]\d{9}$|^([0-9]{3,4}\-)?[0-9]{7,8}$/;
            if (typeof ruleForm.phone === 'undefined' || $.trim(ruleForm.phone) == "") {
                message.error("{yun:}t key='wap_com_00322'{/yun}");
                return false;
            } else if (!phoneReg.test(ruleForm.phone)) {
                message.error("{yun:}t key='wap_01440'{/yun}");
                return false;
            }
            if (typeof ruleForm.salary === 'undefined' || $.trim(ruleForm.salary) == "") {
                message.error("{yun:}t key='admin_user_weipin_00028'{/yun}");
                return false;
            }
            if ($.trim(ruleForm.provinceid) == "" || (that.cityList.length > 0 && $.trim(ruleForm.cityid) == "")) {
                message.error("{yun:}t key='member_user_00109'{/yun}");
                return false;
            }
            if (typeof ruleForm.address === 'undefined' || $.trim(ruleForm.address) == "") {
                message.error('请输入工作地点');
                return false;
            }
            if (typeof ruleForm.require === 'undefined' || $.trim(ruleForm.require) == "") {
                message.error('请输入招聘要求');
                return false;
            }

            $.each(ruleForm, function (key, value) {
                if (key != 'pic_n' && key != 'yyzz_n') {
                    formData.append(key, value);
                }
            });

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=weipin_once&a=save', formData).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    that.saveLoading = false;
                    message.error(res.msg);
                } else {
                    that.drawerAdd = false;
                    that.getList();
                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
        },

        refresh(row) {
            let that = this;

            message.confirm('确认刷新职位？', function () {
                httpPost('m=user&c=weipin_once&a=refresh_job', { id: row.id }).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        message.error(res.msg);
                    } else {
                        that.getList();
                        message.success(res.msg);
                    }
                })
            })
        },

        openPage(url) {
            window.open(url);
        },
    },
};
</script>
<style scoped>
.drawerModInpt .el-alert {
    background: none;
    padding: 6px 0;
}

.moduleInptWidt .el-select .el-input {
    width: 120px;
}
</style>