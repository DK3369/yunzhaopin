<template>
    <div class="moduleElHight">
        <!--新样式-->
        <table class="tableVue">
            <thead>
                <tr align="left">
                    <th width="180">{{ lc('admin_tool_00538') }}</th>
                    <th width="320">{{ lc('admin_tool_00536') }}</th>
                    <th>{{ lc('admin_tool_00535') }}</th>
                </tr>
            </thead>
            <tbody>
                <tr align="left" valign="top">
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00537') }}</div>
                    </td>
                    <td>
                        <el-radio-group v-model="temptype" @input="setTemp">
                            <el-radio label="1">{{ lc('admin_tool_00525') }}</el-radio>
                            <el-radio label="0">{{ lc('admin_tool_00524') }}</el-radio>
                        </el-radio-group>
                    </td>
                    <td rowspan="3" vertical-align="top" valign="top">
                        <div v-show="temptype == '1'">
                            <div class="tw_textarea">
                                <el-input type="textarea" id='twjobtemp_content' v-model="htmlcon" name="temp_content"></el-input>
                            </div>
                            <div class="tw_jobscbox">
                                <input id="twjobcopy" data-clipboard-action="copy"
                                    data-clipboard-target="#twjobtemp_content" type="button" value="lc('admin_tool_00526')"
                                    class="admin_Filter_bth" />
                                <el-link type="primary" @click="htmlcon = ''">{{ lc('admin_tool_00528') }}</el-link>

                            </div>
                        </div>

                        <div v-show="temptype == '0'">
                            <div class="wxpubtool_sj_con" id="twjobgzh_content"
                                style="border:1px solid #eee; padding: 0 16px;border-radius:5px; min-height:100px;"
                                v-html="htmlcon"></div>
                            <div class="tw_jobscbox">
                                <input type="button" class="admin_Filter_bth" id="twjobgzh_copy"
                                    data-clipboard-action="copy" data-clipboard-target="#twjobgzh_content" value="lc('admin_tool_00526')" />
                            </div>
                        </div>

                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00541') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect">
                            <el-select v-model="tempid" size="small" :placeholder="lc('admin_tool_00545')">
                                <el-option v-for="item in templist" :key="item.id" :label="item.title"
                                    :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00550') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect">
                            <el-select v-model="job_search" @visible-change="jobChoosed" multiple filterable remote
                                reserve-keyword :placeholder="lc('admin_tool_00556')" :remote-method="jobSearch"
                                :loading="job_searchloading">
                                <el-option v-for="item in job_search_list" :key="item.value" :label="item.name"
                                    :value="item.value">
                                    <span style="float: left; color: #333; font-size: 14px;font-weight:bold;">{{ item.name}}</span>
                                    <span style="float: right; color: #a5a5a5; font-size: x-small;">{{ item.upname }}</span>
                                </el-option>
                            </el-select>
                        </div>
                        <!---->
                        <div id="joblist">
                            <draggable v-model="tw_joblist" handle=".dragctrl" chosen-class="chosen" force-fallback="true"
                                group="conarr" animation="300" @sort="dragSort">
                                <transition-group>
                                    <div v-for="(job, jKey) in tw_joblist" :key="job.jobid" class="tw_job_tag ">
                                        <span class="dragctrl">{{ job.jobname }}</span><span @click.stop="removeJob(jKey);"
                                            class="tw_jobscbth"> </span>
                                    </div>

                                </transition-group>
                            </draggable>
                        </div>
                        <div class="tool_inputips"><i class="el-icon-question"></i> {{ lc('admin_tool_00534') }}
                            <el-button type="text" @click="clearJob">{{ lc('admin_tool_00552') }}</el-button>
                        </div>

                        <div class="tw_jobscbox">
                            <el-button type="primary" @click="getTW">{{ lc('admin_tool_00543') }}</el-button>
                        </div>
                    </td>
                </tr>
            </tbody>
        </table>


        <div class="moduleSeachs" style="padding-top:10px;">
            <div class="moduleSeachleft" style="display: flex; flex-wrap: wrap; align-items: center;">
                <div class="tableSeachInpt" style="margin-bottom: 0px;flex-flow: row wrap;;">
                    <el-input :placeholder="lc('admin_tool_00555')" clearable style="width: 300px;" v-model="keyword" prefix-icon="el-icon-search"></el-input>
                </div>
                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-select v-model="auid" size="small" :placeholder="lc('admin_tool_00546')" clearable @change="search">
                        <el-option v-for="admin in adminList" :key="admin.uid" :label="admin.name ? admin.name : admin.username" :value="admin.uid"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt" style="margin-bottom: 0px;">
                    <el-input :placeholder="lc('admin_tool_00553')" v-model="welfarekeyword" style="width: 400px;" prefix-icon="el-icon-search" clearable></el-input>
                </div>
                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-select v-model="status" size="small" :placeholder="lc('member_user_00181')" @change="search" clearable>
                        <el-option :label="lc('wap_js_00075')" value="3"></el-option>
                        <el-option :label="lc('admin_tool_00539')" value="2"></el-option>
                        <el-option :label="lc('admin_tool_00532')" value="1"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachkidusye" style="margin-bottom: 0px; font-size: 14px; color: #666;">
                    {{ lc('admin_tool_00540') }}
                    <span @click="tagsearch('urgent')" :class="urgent == 1 ? 'urgent_search_y' : 'urgent_search_n'" :title="lc('admin_user_company_00156')"></span>
                    <span @click="tagsearch('wcmoments')" :class="wcmoments == 1 ? 'wcmoments_search_y' : 'wcmoments_search_n'" :title="lc('admin_user_company_00152')"> </span>
                    <span @click="tagsearch('gzh')" :class="gzh == 1 ? 'gzh_search_y' : 'gzh_search_n'" :title="lc('admin_user_company_00148')"> </span>
                </div>
                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>

        </div>

        <div class="moduleElTable">
            <el-table ref="table" :data="tableData" v-loading="list_loading" @selection-change="selectionChange"
                :default-sort="{ prop: 'id', order: 'descending' }" @sort-change="sortChange" border style="width: 100%"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" :empty-text="emptytext">
				<el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="ID" width="80" sortable="custom"></el-table-column>
                <el-table-column :label="lc('admin_user_company_00159')" width="140">
                    <template #default="scope">
                        <span :class="scope.row.urgent == '1' ? 'twtask_xz' : 'twtask_xz_w'"> </span>
                        <span :class="scope.row.wcmoments == '1' ? 'twtask_pyqxz' : 'twtask_pyqxzw'"> </span>
                        <span :class="scope.row.gzh == '1' ? 'twtask_gzhxz' : 'twtask_gzhxzw'"> </span>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('wap_com_00157')" min-width="170">
                    <template #default="scope">
                        <el-link type="primary" :href="scope.row.comurl" target="_blank">{{ scope.row.comname }}
                        </el-link>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('wap_com_00288')" min-width="220">
                    <template #default="scope">
                        <el-link type="primary" :href="scope.row.joburl" target="_blank">{{ scope.row.jobname }}
                        </el-link>
                        <div v-if="scope.row.jobstatus == '1'">{{ lc('admin_tool_00551') }}</div>
                        <div v-else-if="scope.row.jobstatus == '2'">{{ lc('wap_com_00242') }}</div>
                    </template>
                </el-table-column>
                <el-table-column prop="jobsdate_n" :label="lc('admin_tool_00554')" width="180"></el-table-column>
                <el-table-column :label="lc('member_user_00181')">
                    <template #default="scope">
                        <span v-if="scope.row.status == '1'">{{ lc('admin_tool_00532') }}</span>
                        <span v-else>{{ lc('admin_tool_00539') }}</span>
                    </template>
                </el-table-column>
                <el-table-column prop="admin_username" :label="lc('admin_tool_00531')" width="170"></el-table-column>
                <el-table-column prop="ctime_n" :label="lc('member_com_00300')" width="180" sortable="custom"></el-table-column>
                <el-table-column prop="content" :label="lc('admin_vue_00040')" width="200"></el-table-column>
                <el-table-column :label="lc('member_user_00048')" fixed="right" width="300">
                    <template #default="scope">
                        <el-button type="success" size="small"
                            @click="addjob(scope.row.id, scope.row.jobid, scope.row.jobname)"
                            :disabled="scope.row.jobstatus == '1' || job_ids.indexOf(scope.row.jobid) != -1">{{ lc('wap_00215') }}
                        </el-button>
                        <el-button type="primary" size="small" @click="taskFinish(scope.row.id)"
                            :disabled="scope.row.status == '1'">{{ lc('admin_01270') }}
                        </el-button>
                        <el-button type="danger" size="small" @click="deleteinfo(scope.row.id)">{{ lc('common.delete') }}</el-button>
                    </template>
                </el-table-column>
            </el-table>
        </div>


        <div class="modulePaging">
            <div class="modulecz">
                <el-checkbox v-model="allchecked" @change="allcheckChange">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button size="small" @click="deleteAll">{{ lc('member_com_00055') }}</el-button>
                <el-button size="small" @click="multiAdd">{{ lc('admin_tool_00533') }}</el-button>
                <el-button size="small" @click="multiFinish">{{ lc('admin_user_company_00376') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page="currentPage" :page-size="limit" :page-sizes="page_sizes" :total="total"
                    layout="total, sizes, prev, pager, next, jumper" :pager-count="pagerCount">
                </el-pagination>
            </div>

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
    data: function () {
        return {
			pagerCount: 5,
            emptytext: window.yunAdminT(lc('wap_js_00113')),
            temps: [],
            temps2: [],
            adminList: [],
            tableData: [],
            total: 0,
            limit: 0,
            currentPage: 1,
			prevPage:0,
            page_sizes: [],

            list_loading: false,

            allchecked: false,
            choosedata: [],

            urgent: 0,
            wcmoments: 0,
            gzh: 0,
            auid: '',
            keyword: '',
            welfarekeyword: '',
            status: '3',
            sort_t: '',
            order: '',

            tempid: '',
            temptype: '1',
            templist: [],
            job_searchloading: false,
            job_search: [],
            job_search_list: [],
            job_ids: [],
            tw_joblist: [],
            htmlcon: '',
			weburl:localStorage.getItem("sy_weburl")
        }
    },
    mounted() {

        var tw_joblist = localStorage.getItem("tw_jobids");

        this.tw_joblist = tw_joblist ? JSON.parse(tw_joblist) : [];

        this.retwjobData();

        this.getList();
        this.getBaseData();
        var clipboard = new ClipboardJS("#twjobcopy");
        clipboard.on('success', function (e) {
            message.success(window.yunAdminT(lc('wap_com_00254')));
            e.clearSelection();
        });

        var gzh_clipboard = new ClipboardJS("#twjobgzh_copy");
        gzh_clipboard.on('success', function (e) {
            message.success(window.yunAdminT(lc('wap_com_00254')));
            e.clearSelection();
        });

    },
    methods: {
        async getList() {
            let that = this;
            let params = {
                page: that.currentPage,
                limit: that.limit,
                t: that.sort_t,
                order: that.order,
                keyword: that.keyword,
                status: that.status,
                urgent: that.urgent,
                wcmoments: that.wcmoments,
                gzh: that.gzh,
                auid: that.auid,
                welfarekeyword: that.welfarekeyword,
            }

            this.list_loading = true;
            that.emptytext = window.yunAdminT(lc('admin_user_weipin_00026'));
            httpPost('m=tool&c=fabutool&a=twTask', params).then((result) => {
                this.list_loading = false;
                var res = result.data;

                if (res.error == 0) {
                    that.tableData = res.data.list
                    that.total = parseInt(res.data.total)
                    that.page_sizes = res.data.page_sizes
					that.limit = res.data.page_size
					
					if(that.prevPage != that.currentPage){
						that.prevPage = that.currentPage;
						that.$refs.table.bodyWrapper.scrollTop = 0;
					}
                    if (that.tableData.length === 0){
                        that.emptytext = window.yunAdminT(lc('wap_js_00113'));
                    }
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        getBaseData() {
            let _this = this;
            httpPost('m=tool&c=fabutool&a=twTask_base_data', {}, {hideloading: true}).then(function (response) {
                let res = response.data;
                _this.adminList = Object.freeze(res.data.adminList);
                _this.temps = Object.freeze(res.data.temps);
                _this.temps2 = Object.freeze(res.data.temps2);
                _this.setTemp();
            }).catch(function (error) {
                console.log(error);
            });
        },
        search: function () {
            this.currentPage = 1;
            this.getList();
        },
        sortChange: function (e) {
            this.sort_t = e.prop;
            if (e.prop == 'ctime_n') {
                this.sort_t = 'ctime';
            }
            this.order = e.order == 'ascending' ? 'asc' : 'desc';
            this.search();
        },
        tagsearch: function (type) {
            this[type] = this[type] == 1 ? 0 : 1;
        },
        handleCurrentChange(val) {
            this.currentPage = val;
            this.getList()
        },
        handleSizeChange(val) {
            this.currentPage = 1
            this.limit = val
            this.getList()
        },
        allcheckChange: function () {

            this.$refs.table.toggleAllSelection();

        },
        selectionChange: function (e) {
            if (this.tableData.length != e.length) {
                this.allchecked = false;
            } else {
                this.allchecked = true;
            }
            this.choosedata = e;
        },
        deleteinfo: function (id) {
            var _this = this;

            var params = {
                del: id
            };
            delConfirm(_this, params, this.deletePost)
        },
        deleteAll: function () {
            var _this = this;
            var idarr = [];
            if (this.choosedata.length > 0) {
                for (let i in this.choosedata) {
                    idarr.push(this.choosedata[i].id);
                }
            } else {
                message.error(window.yunAdminT(lc('admin_user_weipin_00005')));
                return;
            }
            var params = {
                del: idarr
            };

            delConfirm(_this, params, this.deletePost)
        },
        async deletePost(params) {

            let that = this;

            httpPost('m=tool&c=fabutool&a=delTwTask', params).then(function (result) {

                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList()
                    });
                    return;
                } else {
                    message.error(res.msg);
                    return;
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        taskFinish: function (id) {
            var _this = this;

            var params = {
                id: id
            };
            delConfirm(_this, params, this.finishPost, window.yunAdminT(lc('admin_tool_00547')))
        },
        multiFinish: function () {
            var _this = this;
            var idarr = [];
            if (this.choosedata.length > 0) {
                for (let i in this.choosedata) {
                    idarr.push(this.choosedata[i].id);
                }
            } else {
                message.error(window.yunAdminT(lc('admin_user_weipin_00001')));
                return;
            }
            var params = {
                id: idarr
            };

            delConfirm(_this, params, this.finishPost, window.yunAdminT(lc('admin_tool_00547')))
        },
        async finishPost(params) {

            let that = this;

            httpPost('m=tool&c=fabutool&a=taskFinish', params).then(function (result) {

                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList()
                    });
                    return;
                } else {
                    message.error(res.msg);
                    return;
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        addjob: function (tid, jobid, jobname) {
            var that = this;

            var tw_joblist = deepClone(this.tw_joblist);
            var job_ids = deepClone(this.job_ids);

            if (job_ids.indexOf(jobid) == -1) {
                tw_joblist.push({
                    jobid: jobid,
                    jobname: jobname,
                    tid: tid
                });
            }

            this.tw_joblist = tw_joblist;
            this.retwjobData();
        },
        multiAdd: function () {
            var that = this;
            var tw_joblist = deepClone(this.tw_joblist);
            var job_ids = deepClone(this.job_ids);

            if (this.choosedata.length > 0) {
                for (let i in this.choosedata) {
                    if (job_ids.indexOf(this.choosedata[i].jobid) == -1) {
						job_ids.push(this.choosedata[i].jobid);
                        tw_joblist.push({
                            jobid: this.choosedata[i].jobid,
                            jobname: this.choosedata[i].jobname,
                            tid: this.choosedata[i].id,
                        });
                    }
                }
            } else {
                message.error(window.yunAdminT(lc('admin_user_weipin_00001')));
                return;
            }

            this.tw_joblist = tw_joblist;
            this.retwjobData();
        },
        dragSort: function () {

            var tw_joblist = deepClone(this.tw_joblist);
            var job_ids = [];

            for (let i in tw_joblist) {
                job_ids.push(tw_joblist[i].jobid);
            }
            this.job_ids = job_ids;
        },
        async getTW() {
            let that = this;

            if (this.tempid == '') {
                message.error(window.yunAdminT(lc('admin_tool_00548')));
                return;
            }
            if (this.job_ids.length == 0) {
                message.error(window.yunAdminT(lc('admin_tool_00549')));
                return;
            }

            var params = {
                tpl: this.tempid,
                jobids: this.job_ids.join(',')
            };

            httpPost('m=tool&c=fabutool&a=getTW', params).then((result) => {

                var res = result.data;

                that.htmlcon = result.data;
                if (that.temptype == 0) {
                    that.$nextTick(() => {
                        // 将图片转成base64，防止复制到其他地方会跨域
                        $("#twjobgzh_content").find('img').each(function () {
                            var _this = $(this);
                            var backImg = new Image();
                            backImg.src = _this.attr('src');
                            ;
                            backImg.onload = function () {
                                // 将是人才网域名的图片转成base64
                                if (backImg.src.indexOf(that.weburl) > -1) {
                                    var base = getBase64Image(backImg);
                                    _this.attr('src', base);
                                }
                            };
                        });
                    })
                }


            }).catch(function (e) {
                console.log(e)
            })
        },
        setTemp: function (id = '') {

            var temptype = this.temptype;

            if (temptype == '1') {
                this.templist = this.temps;
            } else {
                this.templist = this.temps2;
            }

            this.tempid = '';

            this.htmlcon = '';

        },
        async jobSearch(query) {

            let that = this;

            var job_search_list = [];


            that.job_searchloading = true;

            var params = {
                keyword: query
            };

            httpPost('m=tool&c=fabutool&a=getWxpubJob', params).then((result) => {
                that.job_searchloading = false;
                var res = result.data;

                if (res.error == 0) {
                    job_search_list = res.data;
                }

                that.job_search_list = job_search_list;

            }).catch(function (e) {
                console.log(e)
            })
        },
        clearJob: function () {

            this.tw_joblist = [];

            this.retwjobData();
        },
        removeJob: function (jkey) {

            var tw_joblist = deepClone(this.tw_joblist);

            tw_joblist.splice(jkey, 1);

            this.tw_joblist = tw_joblist;

            this.retwjobData();
        },
        jobChoosed: function (visible) {
            if (!visible) {
                var job_search = deepClone(this.job_search);
                var tw_joblist = deepClone(this.tw_joblist);
                var job_search_list = deepClone(this.job_search_list);
                var job_ids = deepClone(this.job_ids);

                for (let i in job_search) {
                    for (let j in job_search_list) {
                        if (job_search_list[j].value == job_search[i] && job_ids.indexOf(job_search[i]) == -1) {
							job_ids.push(job_search_list[j].value);
                            tw_joblist.push({
                                jobid: job_search_list[j].value,
                                jobname: job_search_list[j].name,
                                tid: ''
                            });
                        }
                    }
                }
                this.job_search_list = [];
                this.job_search = [];
                this.tw_joblist = tw_joblist;
                this.retwjobData();
            }

        },
        //更新需要生成的职位id  设置本地缓存
        retwjobData: function () {
            var tw_joblist = deepClone(this.tw_joblist);
            var job_ids = [];

            for (let i in tw_joblist) {
                job_ids.push(tw_joblist[i].jobid);
            }

            this.job_ids = job_ids;

            localStorage.setItem("tw_jobids", JSON.stringify(tw_joblist));
        }
    },
};
</script>
<style scoped>
.moduleSeachmore {
    padding: 0px;
}

.moduleSeachs {
    padding: 0px 0px 12px 0px;
    width: 100%;
}

.moduleElTable {
    padding: 0;
    margin: 0;
    height: initial;
    width: 100%;
}

.tableSeachInptsmalltwo {
    margin-bottom: 0px;
    margin-right: 12px;
}

.tableSeachInptsmalltwo .el-input__inner {
    height: 32px;
    line-height: 32px;
    width: 260px;
    padding: 0px 5px;
    ;
}

.el-dialog__body {
    padding: 0px 20px;
}

.moduleElHight {
    overflow-y: auto;
}
</style>