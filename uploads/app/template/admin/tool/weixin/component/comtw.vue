<template>
    <div class="moduleElHight">
        <!--新样式-->
        <table class="tableVue">
            <thead>
                <tr align="left">
                    <th width="180">推文选择</th>
                    <th width="260">推文状态</th>
                    <th>推文内容预览</th>
                </tr>
            </thead>
            <tbody>
                <tr align="left" valign="top">
                    <td>
                        <div class="TableTite">推文类型</div>
                    </td>
                    <td>
                        <el-radio-group v-model="temptype" @input="setTemp">
                            <el-radio label="1">社群推文模板</el-radio>
                            <el-radio label="0">公众号文章模板</el-radio>
                        </el-radio-group>
                    </td>
                    <td rowspan="3" vertical-align="top" valign="top">
                        <div v-show="temptype == '1'">
                            <div class="tw_textarea">
                                <el-input type="textarea" v-model="htmlcon" id='twcomtemp_content' name="temp_content" ></el-input>
                            </div>
                            <div class="tw_jobscbox">
                                <input id="twcomcopy" data-clipboard-action="copy"
                                    data-clipboard-target="#twcomtemp_content" type="button" value="一键复制"
                                    class="admin_Filter_bth" />
                                <el-link type="primary" @click="htmlcon = ''">清空内容</el-link>

                            </div>
                        </div>

                        <div v-show="temptype == '0'">
                            <div class="wxpubtool_sj_con" id="twcomgzh_content" v-html="htmlcon"
                                style="border:1px solid #eee; padding: 0 16px;border-radius:5px; min-height:100px;"></div>
                            <div class="tw_jobscbox">
                                <input type="button" class="admin_Filter_bth" id="twcomgzh_copy"
                                    data-clipboard-action="copy" data-clipboard-target="#twcomgzh_content" value="一键复制" />
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">模板类型</div>
                    </td>
                    <td>
                        <div class="TableSelect">
                            <el-select v-model="tempid" size="small" slot="prepend" placeholder="选择模板">
                                <el-option v-for="item in templist" :key="item.id" :label="item.title"
                                    :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">指定企业</div>
                    </td>
                    <td>
                        <div class="TableSelect">
                            <el-select v-model="com_search" @visible-change="comChoosed" multiple filterable remote
                                reserve-keyword placeholder="输入搜索企业" :remote-method="comSearch"
                                :loading="com_searchloading">
                                <el-option v-for="item in com_search_list" :key="item.value" :label="item.name"
                                    :value="item.value">
                                    <span
                                        style="float: left; color: #333; font-size: 14px;font-weight:bold;">{{ item.name }}</span>
                                </el-option>
                            </el-select>
                        </div>
                        <!---->
                        <div id="joblist">
                            <draggable v-model="tw_comlist" handle=".dragctrl" chosen-class="chosen" force-fallback="true"
                                group="conarr" animation="300" @sort="dragSort">
                                <transition-group>
                                    <div v-for="(com, cKey) in tw_comlist" :key="com.cuid" class="tw_job_tag ">
                                        <span class="dragctrl">{{ com.comname }}</span><span @click.stop="removeCom(cKey);"
                                            class="tw_jobscbth"></span>
                                    </div>

                                </transition-group>
                            </draggable>
                        </div>
                        <div class="tool_inputips"><i class="el-icon-question"></i> 拖拽标签可排序 <el-button type="text"
                                @click="clearCom">清除企业</el-button></div>

                        <div class="tw_jobscbox">
                            <el-button type="primary" @click="getTW">生成推文</el-button>
                        </div>
                    </td>
                </tr>
            </tbody>
        </table>






        <div class="moduleSeachs" style="padding-top:10px;">
            <div class="moduleSeachleft" style="display: flex; flex-wrap: wrap; align-items: center;">

                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-input placeholder="企业名称、备注、企业UID" style="width: 300px;" clearable v-model="keyword"
                        prefix-icon="el-icon-search">
                    </el-input>
                </div>
                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-select v-model="auid" size="small" slot="prepend" placeholder="选择管理员" clearable @change="search">
                        <el-option v-for="admin in adminList" :key="admin.uid" :label="admin.name ? admin.name : admin.username" :value="admin.uid"></el-option>
                    </el-select>
                </div>

                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-select v-model="status" size="small" slot="prepend" placeholder="状态" @change="search" clearable>
                        <el-option label="全部" value="3"></el-option>
                        <el-option label="未推送" value="2"></el-option>
                        <el-option label="已推送" value="1"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachkidusye" style="margin-bottom: 0px;">
                    标签：
                    <span @click="tagsearch('urgent')" :class="urgent == 1 ? 'urgent_search_y' : 'urgent_search_n'" title="加急"></span>
                    <span @click="tagsearch('wcmoments')" :class="wcmoments == 1 ? 'wcmoments_search_y' : 'wcmoments_search_n'" title="朋友圈"> </span>
                    <span @click="tagsearch('gzh')" :class="gzh == 1 ? 'gzh_search_y' : 'gzh_search_n'" title="公众号"> </span>
                </div>
                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="search">查询</el-button>
                </div>
            </div>

        </div>

        <div class="moduleElTable">
            <el-table ref="table" :data="tableData" v-loading="list_loading" @selection-change="selectionChange"
                :default-sort="{ prop: 'id', order: 'descending' }" @sort-change="sortChange" border style="width: 100%"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" :empty-text="emptytext">
				<el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="编号" width="80" sortable="custom"> </el-table-column>
                <el-table-column label="标签">
                    <template slot-scope="scope">
                        <span :class="scope.row.urgent == '1' ? 'twtask_xz' : 'twtask_xz_w'"> </span>
                        <span :class="scope.row.wcmoments == '1' ? 'twtask_pyqxz' : 'twtask_pyqxzw'"> </span>
                        <span :class="scope.row.gzh == '1' ? 'twtask_gzhxz' : 'twtask_gzhxzw'"> </span>
                    </template>
                </el-table-column>
                <el-table-column label="企业名称">
                    <template slot-scope="scope">
                        <el-link type="primary" :href="scope.row.comurl" target="_blank">{{ scope.row.comname }}</el-link>
                    </template>
                </el-table-column>

                <el-table-column prop="jobsdate_n" label="企业更新时间"></el-table-column>
                <el-table-column label="状态">
                    <template slot-scope="scope">
                        <span v-if="scope.row.status == '1'">已推送</span>
                        <span v-else>未推送</span>
                    </template>
                </el-table-column>
                <el-table-column prop="admin_username" label="创建人"> </el-table-column>
                <el-table-column prop="ctime_n" label="添加时间" width="180" sortable="custom"></el-table-column>
                <el-table-column prop="content" label=" 备注" width="200"></el-table-column>
                <el-table-column label="操作" width="300" fixed="right">
                    <template slot-scope="scope">
                        <el-button type="success" size="mini" @click="addcom(scope.row.id, scope.row.cuid, scope.row.comname)"
                            :disabled="scope.row.comstatus == '2' || com_uids.indexOf(scope.row.cuid) != -1">生成</el-button>
                        <el-button type="primary" size="mini" @click="taskFinish(scope.row.id)"
                            :disabled="scope.row.status == '1'">{{ scope.row.status == '1' ? '已推送' : '未推送' }}</el-button>
                        <el-button size="mini" type="danger" @click="deleteinfo(scope.row.id)">删除</el-button>
                    </template>
                </el-table-column>
            </el-table>
        </div>


        <div class="modulePaging">
            <div class="modulecz">
                <el-checkbox v-model="allchecked" @change="allcheckChange">全选</el-checkbox>
                <el-button size="mini" @click="deleteAll">批量删除</el-button>
                <el-button size="mini" @click="multiAdd">批量生成</el-button>
                <el-button size="mini" @click="multiFinish">批量推送</el-button>
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
module.exports = {
    data: function () {
        return {
			pagerCount: 5,
            emptytext: '暂无数据',
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
            status: '3',
            sort_t: '',
            order: '',

            tempid: '',
            temptype: '1',
            templist: [],
            com_searchloading: false,
            com_search: [],
            com_search_list: [],
            com_uids: [],
            tw_comlist: [],
            htmlcon: '',
        }
    },

    mounted() {

        var tw_comlist = localStorage.getItem("tw_comid");

        this.tw_comlist = tw_comlist ? JSON.parse(tw_comlist) : [];

        this.retwcomData();

        this.getList();
        this.getBaseData();
        var clipboard = new ClipboardJS("#twcomcopy");
        clipboard.on('success', function (e) {
            message.success('复制成功！');
            e.clearSelection();
        });

        var gzh_clipboard = new ClipboardJS("#twcomgzh_copy");
        gzh_clipboard.on('success', function (e) {
            message.success('复制成功！');
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
            }

            this.list_loading = true;
            that.emptytext = "数据加载中";
            httpPost('m=tool&c=fabutool&a=comtwTask', params).then((result) => {
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
                        that.emptytext = "暂无数据";
                    }
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        getBaseData() {
            let _this = this;
            httpPost('m=tool&c=fabutool&a=comtwTask_base_data', {}, {hideloading: true}).then(function (response) {
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
                message.error('请选择要删除的数据'); return;
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
                    message.success(res.msg, function () { that.getList() }); return;
                } else {
                    message.error(res.msg); return;
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
            delConfirm(_this, params, this.finishPost, '确认已推送？')
        },
        multiFinish: function () {
            var _this = this;
            var idarr = [];
            if (this.choosedata.length > 0) {
                for (let i in this.choosedata) {
                    idarr.push(this.choosedata[i].id);
                }
            } else {
                message.error('请选择要操作的数据项'); return;
            }
            var params = {
                id: idarr
            };

            delConfirm(_this, params, this.finishPost, '确认已推送？')
        },
        async finishPost(params) {

            let that = this;

            httpPost('m=tool&c=fabutool&a=taskFinish', params).then(function (result) {

                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () { that.getList() }); return;
                } else {
                    message.error(res.msg); return;
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        addcom: function (tid, cuid, comname) {
            var that = this;

            var tw_comlist = deepClone(this.tw_comlist);
            var com_uids = deepClone(this.com_uids);

            if (com_uids.indexOf(cuid) == -1) {
                tw_comlist.push({
                    cuid: cuid,
                    comname: comname,
                    tid: tid
                });
            }

            this.tw_comlist = tw_comlist;
            this.retwcomData();
        },
        multiAdd: function () {
            var that = this;
            var tw_comlist = deepClone(this.tw_comlist);
            var com_uids = deepClone(this.com_uids);

            if (this.choosedata.length > 0) {
                for (let i in this.choosedata) {
                    if (com_uids.indexOf(this.choosedata[i].cuid) == -1) {
						com_uids.push(this.choosedata[i].cuid);
                        tw_comlist.push({
                            cuid: this.choosedata[i].cuid,
                            comname: this.choosedata[i].comname,
                            tid: this.choosedata[i].id,
                        });
                    }
                }
            } else {
                message.error('请选择要操作的数据项'); return;
            }

            this.tw_comlist = tw_comlist;
            this.retwcomData();
        },
        dragSort: function () {

            var tw_comlist = deepClone(this.tw_comlist);
            var com_uids = [];

            for (let i in tw_comlist) {
                com_uids.push(tw_comlist[i].cuid);
            }
            this.com_uids = com_uids;
        },
        async getTW() {
            let that = this;

            if (this.tempid == '') {
                message.error('请选择模板'); return;
            }
            if (this.com_uids.length == 0) {
                message.error('请添加要生成的职位'); return;
            }

            var params = {
                tpl: this.tempid,
                cuids: this.com_uids.join(',')
            };

            httpPost('m=tool&c=fabutool&a=getComTW', params).then((result) => {

                var res = result.data;

                that.htmlcon = result.data;
                if (that.temptype == 0) {
                    that.$nextTick(() => {
                        // 将图片转成base64，防止复制到其他地方会跨域
                        $("#twcomgzh_content").find('img').each(function () {
                            var _this = $(this);
                            var backImg = new Image();
                            backImg.src = _this.attr('src');;
                            backImg.onload = function () {
                                // 将是人才网域名的图片转成base64
                                if (backImg.src.indexOf(weburl) > -1) {
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
        async comSearch(query) {

            let that = this;

            var com_search_list = [];


            that.com_searchloading = true;

            var params = {
                keyword: query
            };

            httpPost('m=tool&c=fabutool&a=getComBySearch', params).then((result) => {
                that.com_searchloading = false;
                var res = result.data;

                if (res.error == 0) {
                    com_search_list = res.data;
                }

                that.com_search_list = com_search_list;

            }).catch(function (e) {
                console.log(e)
            })
        },
        clearCom: function () {

            this.tw_comlist = [];

            this.retwcomData();
        },
        removeCom: function (jkey) {

            var tw_comlist = deepClone(this.tw_comlist);

            tw_comlist.splice(jkey, 1);

            this.tw_comlist = tw_comlist;

            this.retwcomData();
        },
        comChoosed: function (visible) {
            if (!visible) {
                var com_search = deepClone(this.com_search);
                var tw_comlist = deepClone(this.tw_comlist);
                var com_search_list = deepClone(this.com_search_list);
                var com_uids = deepClone(this.com_uids);

                for (let i in com_search) {
                    for (let j in com_search_list) {
                        if (com_search_list[j].value == com_search[i] && com_uids.indexOf(com_search[i]) == -1) {
							com_uids.push(com_search_list[j].value);
                            tw_comlist.push({
                                cuid: com_search_list[j].value,
                                comname: com_search_list[j].name,
                                tid: ''
                            });
                        }
                    }
                }
                this.com_search_list = [];
                this.com_search = [];
                this.tw_comlist = tw_comlist;
                this.retwcomData();
            }
        },
        //更新需要生成的职位id  设置本地缓存 
        retwcomData: function () {
            var tw_comlist = deepClone(this.tw_comlist);
            var com_uids = [];

            for (let i in tw_comlist) {
                com_uids.push(tw_comlist[i].cuid);
            }

            this.com_uids = com_uids;

            localStorage.setItem("tw_comid", JSON.stringify(tw_comlist));
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
}

.el-dialog__body {
    padding: 0px 20px;
}

.moduleElHight {
    overflow-y: auto;
}</style>