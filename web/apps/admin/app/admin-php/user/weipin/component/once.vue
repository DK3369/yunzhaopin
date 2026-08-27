<template>
    <div class="moduleElHight">
        <div class="moduleElSearchInf">
            <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                <div class="moduleInptList moduleInptWidt">
                    <el-input :placeholder="lc('admin_user_weipin_00003')" @keyup.enter="search" size="small" v-model="searchForm.keyword" clearable class="input-with-select">
                        <template #prepend><el-select v-model="searchForm.type" :placeholder="lc('wap_user_00100')">
                            <el-option :label="lc('wap_com_00288')" :value="2"></el-option>
                            <el-option :label="lc('admin_company_00023')" :value="3"></el-option>
                            <el-option :label="lc('wap_01431')" :value="4"></el-option>
                            <el-option :label="lc('admin_user_weipin_00046')" :value="5"></el-option>
                        </el-select></template>
                    </el-input>
                </div>
                <div v-for="(searchItem, searchIndex) in searchList" :key="searchIndex" class="moduleInptList">
                    <el-select size="small" v-model="searchForm[searchItem.param]" :clearable="true"
                        :placeholder="searchItem.name" @change="search">
                        <el-option v-for="(searchLabel, searchValue) in searchItem.value" :key="searchValue" :label="searchLabel"
                            :value="searchValue"></el-option>
                    </el-select>
                </div>
                <div class="moduleInptList">
                    <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>
            <div class="moduleSeachButn moduleSeachMart">
                <el-button type="primary" icon="el-icon-document-add" size="small" @click="openAdd('')">{{ lc('admin_user_weipin_00042') }}</el-button>
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
                <template #empty>
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="90" sortable="custom">
                </el-table-column>
                <el-table-column :label="lc('admin_user_weipin_00006')" min-width="180">
                    <template #default="scope">
                        <div class="moduleProps">
                            <span><el-link :underline="false" @click="openPage(scope.row.once_url)" type="primary">{{
                                scope.row.title }}</el-link></span>
                            <span>{{ scope.row.companyname }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('wap_00362')" width="140" align="center">
                    <template #default="scope">
                        <div class="moduleProps" style="display: flex; align-items: center; justify-content: center;">
                            <el-image v-if="scope.row.pic" :src="scope.row.pic_n" style="width: 50px; height: 50px">
                            </el-image>
                            <span v-else>{{ lc('common_02082') }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="phone" :label="lc('admin_company_00023')" width="140">
                </el-table-column>
                <el-table-column prop="linkman" :label="lc('wap_01431')" width="140">
                </el-table-column>
                <el-table-column prop="ctime" :label="lc('admin_user_weipin_00030')" width="140" sortable="custom">
                    <template #default="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.status == 2" class="admin_state3">{{ scope.row.ctime_n }}</span>
                            <span v-else>{{ scope.row.ctime_n }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="edate" :label="lc('admin_company_00006')" width="140" sortable="custom">
                    <template #default="scope">
                        <div>{{ scope.row.edate_n }}</div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00181')" width="120">
                    <template #default="scope">
                        <div class="moduleProps">
                            <div class="admin_state">
                                <span v-if="scope.row.status == 1" class="admin_state1">{{ lc('wap_user_00165') }}</span>
                                <span v-else-if="scope.row.status == 2" class="admin_state2">{{ lc('member_com_00304') }}</span>
                                <span v-else-if="scope.row.status == 0" class="admin_state5">{{ lc('wap_user_00166') }}</span>
                            </div>
                            <div v-if="scope.row.pay > 0" class="admin_state">
                                <span v-if="scope.row.pay == 1" class="admin_state5">{{ lc('wap_00359') }}</span>
                                <span v-else-if="scope.row.pay == 2" class="admin_state1">{{ lc('admin_user_weipin_00045') }}</span>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_user_weipin_00050')" width="140">
                    <template #default="scope">
                        <div class="moduleProps moduleTrButn">
                            <span>{{ domainList[scope.row.did] }}</span>
                            <el-button type="text" @click="openDomain(scope.row)">{{ lc('admin_user_weipin_00048') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
                    <template #default="scope">
                        <div class="moduleElTaCaoz">
                            <el-button size="small" style="margin-right: 8px;" @click="openAudit(scope.row)">{{ lc('member_user_00152') }}</el-button>
                            <el-popover placement="bottom" width="60" trigger="hover">
                                <div class="moduleMores">
                                    <el-button size="small" @click="openDetail(scope.row)">{{ lc('wap_com_00427') }}</el-button>
                                    <el-button size="small" @click="openAdd(scope.row)">{{ lc('wap_js_00073') }}</el-button>
                                    <el-button size="small" @click="refresh(scope.row)">{{ lc('wap_user_00334') }}</el-button>
                                    <el-button size="small" @click="del(scope.$index)">{{ lc('common.delete') }}</el-button>
                                </div>
                                <template #reference><el-button size="small">{{ lc('common.more') }}</el-button></template>
                            </el-popover>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
                    @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="batch('del')" size="small">{{ lc('member_com_00055') }}</el-button>
                <el-button @click="batch('audit')" size="small">{{ lc('admin_user_weipin_00037') }}</el-button>
                <el-button @click="batch('audit_cancel')" size="small">{{ lc('admin_user_weipin_00018') }}</el-button>
                <el-button @click="batch('extension')" size="small">{{ lc('admin_user_weipin_00038') }}</el-button>
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
            <el-drawer :title="lc('admin_user_weipin_00032')" v-model="dialogAudit" :with-header="true" :modal-append-to-body="false"
                :show-close="true" size="640px">
                <div class="tableTancyans">
                    <div class="toolClasDia">
                        <div class="zhaopTacOne">
                            <div class="zhaopTacName">{{ lc('admin_user_weipin_00039') }}</div>
                            <div class="zhaopTacTwo">
                                <table border="1">
                                    <tbody>
                                        <tr>
                                            <td width="25%">
                                                <div class="TableTite">{{ lc('wap_com_00288') }}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.title }}</div>
                                            </td>
                                            <td width="25%">
                                                <div class="TableTite">{{ lc('admin_user_weipin_00044') }}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.salary }}</div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <div class="TableTite">{{ lc('wap_00351') }}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.companyname }}</div>
                                            </td>
                                            <td>
                                                <div class="TableTite">{{ lc('wap_00347') }}</div>
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
                                                <div class="TableTite">{{ lc('admin_user_weipin_00030') }}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.ctime_n }}</div>
                                            </td>
                                            <td>
                                                <div class="TableTite">{{ lc('member_com_00315') }}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.edate_n }}</div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <div class="TableTite">{{ lc('wap_00374') }}</div>
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
                            <div class="zhaopTacName">{{ lc('wap_00462') }}</div>
                            <div class="zhaopTacTwo">
                                <table border="1">
                                    <tbody>
                                        <tr>
                                            <td width="25%">
                                                <div class="TableTite">{{ lc('wap_user_00265') }}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.phone }}</div>
                                            </td>
                                            <td width="25%">
                                                <div class="TableTite">{{ lc('wap_01431') }}</div>
                                            </td>
                                            <td>
                                                <div class="TableData">{{ detail.linkman }}</div>
                                            </td>
                                        </tr>

                                        <tr>
                                            <td>
                                                <div class="TableTite">{{ lc('member_user_00198') }}</div>
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
                                <span>{{ lc('admin_user_weipin_00022') }}</span>
                            </div>
                            <div class="toolClasCont">
                                <el-radio v-model="ruleFormAudit.status" :label="1">{{ lc('wap_user_00165') }}</el-radio>
                                <el-radio v-model="ruleFormAudit.status" :label="2">{{ lc('admin_user_weipin_00008') }}</el-radio>
                            </div>
                        </div>
                    </div>
                    <div class="dialogFooter">
                        <!-- <el-button @click="dialogAudit = false">取 消</el-button> -->
                        <el-button type="primary" @click="saveAudit">{{ lc('wap_com_00019') }}</el-button>
                    </div>
                </div>
            </el-drawer>
            <el-dialog :title="lc('admin_user_weipin_00038')" v-model="dialogExtension" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="350px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_user_weipin_00025') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input v-model="ruleFormExtension.endtime" placeholder=""
                                @input="inputIntNumber($event, 'ruleFormExtension', 'endtime')">
                                <template #append>{{ lc('common_02067') }}</template>
                            </el-input>
                        </div>
                    </div>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogExtension = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="saveExtension">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_weipin_00029')" v-model="dialogDomain" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="500px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_user_weipin_00035') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <span>{{ detail.companyname }}</span>
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
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogDomain = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="saveDomain">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <div class="modluDrawer">
            <el-drawer :title="lc('admin_user_weipin_00040')" v-model="drawerDetail" :modal-append-to-body="false" size="640px">
                <div class="tableTancyans">
                    <div class="zhaopTacTwo">
                        <table border="1">
                            <tbody>
                                <tr>
                                    <td width="25%">
                                        <div class="TableTite">{{ lc('wap_com_00288') }}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.title }}</div>
                                    </td>
                                    <td width="25%">
                                        <div class="TableTite">{{ lc('admin_user_weipin_00034') }}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.companyname }}</div>
                                    </td>
                                </tr>

                                <tr>
                                    <td>
                                        <div class="TableTite">{{ lc('admin_user_weipin_00041') }}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.edate_n }}</div>
                                    </td>
                                    <td>
                                        <div class="TableTite">{{ lc('admin_user_weipin_00044') }}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.salary }}</div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{{ lc('admin_user_weipin_00027') }}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.linkman }}</div>
                                    </td>
                                    <td>
                                        <div class="TableTite">{{ lc('wap_user_00265') }}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.phone }}</div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{{ lc('wap_00349') }}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.city_n }}</div>
                                    </td>
                                    <td>
                                        <div class="TableTite">{{ lc('member_user_00198') }}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">{{ detail.address }}</div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{{ lc('admin_user_weipin_00017') }}</div>
                                    </td>
                                    <td>
                                        <div class="TableData">
                                            <el-image :src="detail.yyzz_n" style="max-width: 200px; max-height: 200px"
                                                :preview-src-list="[detail.yyzz_n]">
                                            </el-image>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="TableTite">{{ lc('wap_00362') }}</div>
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
                                        <div class="TableTite">{{ lc('wap_00374') }}</div>
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
                            <span>{{ lc('wap_com_00320') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <div class="admin_state">
                                <span v-if="detail.status == 1" class="admin_state1">{{ lc('wap_user_00165') }}</span>
                                <span v-else-if="detail.status == 2" class="admin_state2">{{ lc('member_com_00304') }}</span>
                                <span v-else-if="detail.status == 0" class="admin_state5">{{ lc('wap_user_00166') }}</span>
                            </div>
                        </div>
                    </div>
                </div>
                
            </el-drawer>
            <el-drawer :title="lc('wap_js_00130')" v-model="drawerAdd" :modal-append-to-body="false" size="40%">
                <div class="moduleSchools">
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('wap_com_00288') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.title" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_user_weipin_00034') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.companyname" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_user_weipin_00010') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.edate" placeholder=""
                                @input="inputIntNumber($event, 'ruleFormAdd', 'edate')" maxlength="3"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_user_weipin_00027') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.linkman" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('wap_user_00265') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.phone" placeholder=""
                                @input="inputIntNumber($event, 'ruleFormAdd', 'phone')"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_user_weipin_00044') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.salary" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('wap_00349') }}</span>
                        </div>
                        <div class="drawerModInpt" style="display: flex; align-items: center;">
                            <el-select v-model="ruleFormAdd.provinceid" :placeholder="lc('wap_user_00100')" @change="changeCity($event, 1)">
                                <el-option v-for="item in provinceList" :key="item.id" :label="item.name" :value="item.id">
                                </el-option>
                            </el-select>
                            <el-select v-model="ruleFormAdd.cityid" :placeholder="lc('wap_user_00100')" style="margin: 0 10px;"
                                @change="changeCity($event, 2)">
                                <el-option v-for="item in cityList" :key="item.id" :label="item.name" :value="item.id">
                                </el-option>
                            </el-select>
                            <el-select v-model="ruleFormAdd.three_cityid" :placeholder="lc('wap_user_00100')" clearable>
                                <el-option v-for="item in regionList" :key="item.id" :label="item.name" :value="item.id">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('member_user_00198') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.address" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_user_weipin_00017') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-upload class="upload-demo" :accept="pic_accept" list-type="picture" action=""
                                :auto-upload="false" :on-change="handleChangeYyzz" :show-file-list="false">
                                <el-button size="small" type="primary"
                                    icon="el-icon-document-add">{{ lc('wap_00540') }}</el-button>
                                <img class="el-upload-list__item-thumbnail"
                                    style="padding-left: 5px;max-width: 120px;max-height: 120px;" v-if="ruleFormAdd.yyzz_n"
                                    :src="ruleFormAdd.yyzz_n" />
                            </el-upload>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('wap_00362') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-upload class="upload-demo" :accept="pic_accept" list-type="picture" action=""
                                :auto-upload="false" :on-change="handleChangePic" :show-file-list="false">
                                <el-button size="small" type="primary"
                                    icon="el-icon-document-add">{{ lc('wap_00540') }}</el-button>
                                <img class="el-upload-list__item-thumbnail"
                                    style="padding-left: 5px;max-width: 120px;max-height: 120px;" v-if="ruleFormAdd.pic_n"
                                    :src="ruleFormAdd.pic_n" />
                            </el-upload>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_user_weipin_00047') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input type="textarea" :rows="2" placeholder="" v-model="ruleFormAdd.require">
                            </el-input>
                        </div>
                    </div>
                    <div class="drawerModLis" style="align-items: initial;">
                        <div class="drawerModTite">
                            <span>{{ lc('wap_user_00371') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormAdd.password" placeholder="" show-password
                                @input="inputPassword($event, 'ruleFormAdd', 'password')"></el-input>
                            <el-alert :title="lc('admin_user_weipin_00007')" :closable="false" type="info" show-icon>
                            </el-alert>
                        </div>
                    </div>
                    <div class="setBasicButn" style="border: none;">
                        <el-button type="primary" size="medium" @click="saveAdd" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
                    </div>
                </div>
            </el-drawer>
        </div>
    </div>
</template>
    
<script>
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
        status: { type: String, default: '' }
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            loading: false,
			pagerCount: 5,
            dataText: lc('admin_user_weipin_00026'),
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
            order: '',

            checkedAll: false, // {{ lc('wap_js_00074') }}
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

            // {{ lc('admin_user_weipin_00038') }}
            dialogExtension: false,
            ruleFormExtension: {},

            // {{ lc('wap_com_00427') }}
            drawerDetail: false,

            // {{ lc('wap_js_00091') }}/{{ lc('common.edit') }}
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
            httpPost('m=user&c=weipin_once&a=getCache', {},{hideloading: true}).then(function (response) {
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
                    that.dataText = lc('wap_js_00113');
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
                message.error(lc('admin_user_weipin_00005'));
                return false;
            }else if (this.multipleSelection.length == 0){
                message.error(lc('admin_user_weipin_00001'));
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
                }, lc('admin_user_weipin_00011'))
            } else if (type == 'audit_cancel') {
                delConfirm(this, null, function (params) {
                    that.ruleFormAudit = {
                        id: that.idArr,
                        status: 0,
                    };
                    that.saveAudit();
                }, lc('admin_user_weipin_00004'))
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

            if (typeof idx == 'undefined') { // {{ lc('member_com_00055') }}
                params.del = this.idArr;
                msg = lc('common_00853');
            } else {// {{ lc('common_01711') }}
                params.del = that.list[idx].id;
                msg = lc('admin_00333');
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
            if (typeof row == 'undefined') { // {{ lc('admin_yunying_00106') }}
                this.detail = {};
                this.$set(this.ruleFormDomain, 'id', this.idArr);
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
                message.error(lc('admin_user_weipin_00002'));
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

            if (typeof row != 'undefined') { // 单个审核
                that.detail = row;
                that.ruleFormAudit = {
                    id: row.id,
                    status: row.status,
                };

                that.dialogAudit = true;
            } else { // {{ lc('admin_user_weipin_00037') }}
                that.detail = {};
                that.ruleFormAudit = {
                    id: this.idArr,
                    status: '',
                };
            }
        },

        saveAudit() {
            let that = this,
                params = that.ruleFormAudit;

            if (params.status === '') {
                message.error(lc('admin_user_weipin_00015'));
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
                        message.error(lc('admin_user_weipin_00023'));
                    } else if (data.status === 1) {
                        message.success(lc('admin_user_weipin_00031'));
                    } else {
                        message.success(lc('admin_user_weipin_00016'));
                    }
                }
            })
        },

        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g, '');
        },
        inputPassword(val, form, key) {
            this.$data[form][key] = val.replace(/^ +| +$/g, '');
        },

        // {{ lc('admin_user_weipin_00038') }}
        openExtension(row) {
            this.ruleFormExtension = {
                id: this.idArr,
                endtime: '',
            };

            this.dialogExtension = true;
        },

        saveExtension() {
            let that = this,
                params = that.ruleFormExtension;

            if (params.endtime === '') {
                message.error(lc('admin_user_weipin_00013'));
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
                message.error(lc('admin_user_weipin_00014'));
                return false;
            }
            if (typeof ruleForm.companyname === 'undefined' || $.trim(ruleForm.companyname) == "") {
                message.error(lc('admin_user_weipin_00012'));
                return false;
            }
            if (typeof ruleForm.edate === 'undefined' || $.trim(ruleForm.edate) == "") {
                message.error(lc('admin_vue_00036'));
                return false;
            }
            if (typeof ruleForm.linkman === 'undefined' || $.trim(ruleForm.linkman) == "") {
                message.error(lc('wap_com_00013'));
                return false;
            }
            var phoneReg = /^[1][3456789]\d{9}$|^([0-9]{3,4}\-)?[0-9]{7,8}$/;
            if (typeof ruleForm.phone === 'undefined' || $.trim(ruleForm.phone) == "") {
                message.error(lc('wap_com_00322'));
                return false;
            } else if (!phoneReg.test(ruleForm.phone)) {
                message.error(lc('wap_01440'));
                return false;
            }
            if (typeof ruleForm.salary === 'undefined' || $.trim(ruleForm.salary) == "") {
                message.error(lc('admin_user_weipin_00028'));
                return false;
            }
            if ($.trim(ruleForm.provinceid) == "" || (that.cityList.length > 0 && $.trim(ruleForm.cityid) == "")) {
                message.error(lc('member_user_00109'));
                return false;
            }
            if (typeof ruleForm.address === 'undefined' || $.trim(ruleForm.address) == "") {
                message.error(lc('admin_vue_00037'));
                return false;
            }
            if (typeof ruleForm.require === 'undefined' || $.trim(ruleForm.require) == "") {
                message.error(lc('admin_vue_00038'));
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

            message.confirm(lc('admin_vue_00039'), function () {
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