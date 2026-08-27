<template>
    <div class="moduleElHight">
        <div class="moduleSeachs">
            <div class="moduleSeachInpt">
                <el-input :placeholder="lc('admin_00340')" size="small" style="margin-right: 8px;" v-model="keyword" clearable class="input-with-select">
                    <template #prepend><el-select v-model="type" :placeholder="lc('wap_user_00100')">
                        <el-option :label="lc('member_com_00377')" value="1"></el-option>
                        <el-option :label="lc('wap_com_00157')" value="2"></el-option>
                    </el-select></template>
                </el-input>
                <el-select v-model="status" size="small" style="margin-right: 8px;" :placeholder="lc('wap_com_00406')" clearable @change="search">
                    <el-option :label="lc('member_user_00042')" value="1"></el-option>
                    <el-option :label="lc('wap_user_00166')" value="3"></el-option>
                    <el-option :label="lc('wap_user_00167')" value="2"></el-option>
                </el-select>
                <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
        <div class="admin_datatip"><i class="el-icon-document"></i> {{ lc('admin_00830') }}
        </div>
        <div class="moduleElTable moduleElMoreLive" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" @selection-change="handleSelectionChange" ref="multipleTable" :default-sort="{ prop: 'id', order: 'descending' }" @sort-change='sortChange' v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" sortable="custom"></el-table-column>
                <el-table-column prop="zphname" :label="lc('member_com_00377')" min-width="180"></el-table-column>
                <el-table-column prop="comname" :label="lc('admin_00299')" min-width="180" show-overflow-tooltip></el-table-column>
                <el-table-column prop="jobname" :label="lc('wap_user_00154')" min-width="180" show-overflow-tooltip></el-table-column>
                <el-table-column prop="space_n" :label="lc('admin_00306')" sortable="custom"></el-table-column>
                <el-table-column prop="sort" :label="lc('member_com_00022')" sortable="custom">
                    <template #default="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']" :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index" v-model="scope.row.sort" @blur="alterData(scope, 1)"></el-input>
                        <span v-else>
                            {{ scope.row.sort }}
                            <img src="/admin/php-admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14" @click="editData(scope, 1)">
                        </span>
                    </template>
                </el-table-column>
                <el-table-column prop="zt" :label="lc('member_user_00181')">
                    <template #default="props">
                        <div class="admin_state">
                            <span v-if="props.row.status == '1'" class="admin_state1"> {{ lc('admin_user_00149') }}</span>
                            <span v-else-if="props.row.status == '0'" class="admin_state2"> {{ lc('wap_user_00166') }}</span>
                            <span v-else-if="props.row.status == '2'" class="admin_state2"> {{ lc('wap_user_00167') }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" fixed="right" width="150" align="center">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" plain @click="cominfo(scope.row)">{{ lc('member_com_00380') }}</el-button>
                            <el-button type="danger" size="small" @click="delrow(scope.row.id)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="delAllBottom" size="small">{{ lc('member_com_00055') }}</el-button>
                <el-button @click="multiAudit" size="small">{{ lc('admin_user_weipin_00037') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" :current-page="currentPage" :page-sizes="pageSizes" :page-size="perPage" layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <!--参会企业详情-->
        <el-drawer :title="lc('admin_00843')" v-if="dtlislook" v-model="comdrawersh" :modal-append-to-body="false" append-to-body size="80%">
            <div class="shbox">
                <div class="shinfo">
                    <div class="shcomname">{{info.name}}
                        <el-tag type="danger" size="small">{{info.rating_name}}</el-tag>
                    </div>
                    <div class="sh_zwsz_add">{{ lc("admin_company_booth_value", [curr_comdata.zphname, curr_comdata.space_n]) }}</div>
                    <div class="sh_zwsz" style="top: 0;">
                        <el-button type="primary" size="small" plain @click="setZw"><i class="el-icon-edit"></i> {{ lc('admin_00838') }}
                        </el-button>
                        <el-button type="primary" size="small" @click="showComJob"><i class="el-icon-suitcase-1"></i> {{ lc('wap_00560') }}</el-button>
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
                    <div class="shshowall" style="height: calc(100% - 105px);">
                        <div class="shshow" style="overflow-y: auto; position: relative; height: 100%;">
                            <div class="shshow_tit"><i class="el-icon-office-building"></i> {{ lc('wap_user_00341') }}</div>
                            <div class="shshow_p">
                                <div class="" v-if="info.welfare">{{ lc('admin_00644') }}
                                    <el-tag style="margin-right: 5px;" v-for="(item,index) in info.welfare_n" :key="index" size="small">
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
                                            {{item.job_salary}}
                                        </span>
                                        <span class="shshow_line" v-if="item.job_exp">{{ lc('admin_00840') }}</span>
                                        <span class="shshow_line" v-if="item.job_edu">{{ lc('admin_00841') }}</span>
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
        <!--批量审核参会企业-->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00842')" width="300px" v-model="drawercomstatusmultiple" append-to-body :modal-append-to-body="false">
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
                <template #footer><span class="dialog-footer">
                    <el-button @click="drawercomstatusmultiple = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="multipleComStatusSave">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!--参会企业详情 设置展位-->
        <el-drawer :title="lc('admin_00838')" v-model="drawersetzw" :modal-append-to-body="false" append-to-body size="80%">
            <div class="yd_qy">{{ lc("admin_booth_selection_title", [curr_zphtitle]) }}</div>
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
            <el-dialog :title="lc('wap_00560')" width="300px" v-model="drawercomjob" append-to-body :modal-append-to-body="false">
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
                <template #footer><span class="dialog-footer">
                    <el-button @click="drawercomjob = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="saveComJob">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
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
        shstatus: {type: String, default: ''}
    },
    data: function() {
        return {
            emptytext: lc('wap_js_00113'),
            loading: false,
            submitLoading: false,
            status: this.shstatus,
            type: '1',
            keyword: '',
            checkedAll: false,
            selectedItem: [],
            tableData: [],
            currentPage: 1,
            perPage: 0,
            pageSizes: [],
            total: 0,
            curr_data: {},
            curr_comdata: {},
            sort_type: '',
            sort_col: '',
            job_arr: [],
            jobids: [],
            tableHig: true,
            comdrawersh: false,
            drawercomjob: false,
            drawersetzw: false,
            comid: '',
            com_arr: [],
            space_list: [],
            sel_comname: '',
            sel_zwid: '',
            sel_cdid: '',
            sel_zwname: lc('admin_00304'),
            drawercomstatusmultiple: false,
            multiComStatus: '',
            multiComStatusBody: '',
            info: null,
            dtlislook: false,
            curr_zphtitle: '',
            oldData: null,
            islook: false,
            prevPage:0
        }
    },
    mounted() {

    },
    methods: {
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
                rt.push('yd_ztkyd_active')
            }
            if (this.sel_zwid == childit.id) {
                this.sel_zwname = childit.name
            }
            return rt
        },
        // {{ lc('admin_00838') }}
        setZw() {
            var that = this
            httpPost('m=neirong&c=zhaopinhui&a=comadd', { id: that.info.zph.zid }).then(function(response) {
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
                        that.getList()
                    });
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            })
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
        cominfo(data) {
            var that = this
            that.curr_comdata = data
            httpPost('m=neirong&c=zhaopinhui&a=audit', { id: this.curr_comdata.id, zph_info: 1 }).then(function(response) {
                if (response.data.error == 0) {
                    that.info = response.data.data
                    that.curr_zphtitle = that.info.zph.title
                    that.dtlislook = true
                    that.comdrawersh = true
                } else {
                    message.error(lc('admin_user_company_00017'));
                }
            }).catch(function(error) {
                console.log(error);
            })
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
        async delete(id) {
            let that = this;
            let params = {
                del: id
            };
            httpPost('m=neirong&c=zhaopinhui&a=delcom', params).then(function(response) {
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
        editData(scope) {
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            this.oldData = JSON.parse(JSON.stringify(row));
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = true;
            this.$set(this.tableData, index, copyRow);
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
            this.$set(this.tableData, index, copyRow);
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
        sortChange: function(column) {
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
            if (that.type) {
                params.type = that.type
            }
            if (that.status) {
                params.status = that.status
            }
            if (that.sort_type && that.sort_col) {
                params.order = that.sort_type
                params.t = that.sort_col
            }
            that.loading = true;
            that.emptytext = lc('admin_user_weipin_00026');
            httpPost('m=neirong&c=zhaopinhui&a=com', params, {hideloading: true}).then(function(result) {
                var res = result.data
                if (res.error == 0) {
                    that.tableData = res.data.list
                    that.perPage = parseInt(res.data.perPage)
                    that.pageSizes = res.data.pageSizes
                    that.total = parseInt(res.data.total)
                    if(that.prevPage != that.currentPage){
                        that.prevPage = that.currentPage;
                        that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    that.loading = false;
                    if (that.tableData.length === 0){
                        that.emptytext = lc('wap_js_00113');
                    }
                }
            }).catch(function(e) {
                console.log(e)
            })
        },
        // 参会企业批量审核
        multiAudit: function(){
            if (!this.selectedItem.length) {
                message.error(lc('admin_00246'));
                return false;
            }
            this.drawercomstatusmultiple = true
        },
        // 参会企业批量审核
        multipleComStatusSave() {
            var that = this
            if (!that.selectedItem.length) {
                message.error(lc('admin_00246'));
                return false;
            }
            that.comstatus({
                pid: that.selectedItem.join(','),
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
                        if (tp == 1) {
                            that.dtlislook = false
                            that.comdrawersh = false
                        } else {
                            that.drawercomstatusmultiple = false
                        }
                        that.getList()
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
    },
};
</script>
<style scoped></style>