<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="运营人员可以针对微信公众号设置“回复关健词”方式后，用户在微信公众号上回复该关健词可以自动返回已设置的关健词或用户解决日常问题" type="success"
                      :closable="false">
            </el-alert>
        </div>
        <div class="moduleSeachs">
            <div class="moduleSeachleft">

                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-input placeholder="请输入搜索内容" v-model="keyword" size="small" prefix-icon="el-icon-search"
                              clearable>
                    </el-input>
                </div>

                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="search">查询</el-button>
                </div>
            </div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="addinfo()">添加规则</el-button>
            </div>
        </div>

        <div class="moduleElTable">
            <el-table ref="table" :data="tableData" v-loading="list_loading" @selection-change="selectionChange" border
                      style="width: 100%" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                      :empty-text="emptytext">
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" label="编号" width="80">
                </el-table-column>
                <el-table-column prop="title" label="规则名">
                </el-table-column>
                <el-table-column prop="keyword" label="关键字">
                </el-table-column>
                <el-table-column label="操作" width="130" fixed="right" header-align="center">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" @click="editinfo(scope.row.id)">修改</el-button>
                            <el-button type="danger" size="small" @click="deleteinfo(scope.row.id)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>


        <div class="modulePaging">
            <div class="modulecz modulePagButn">
                <el-checkbox v-model="allchecked" @change="allcheckChange">全选</el-checkbox>
                <el-button size="mini" @click="deleteAll">批量删除</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                               :current-page="currentPage" :page-size="limit" :page-sizes="page_sizes" :total="total"
                               layout="total, sizes, prev, pager, next, jumper">
                </el-pagination>
            </div>
        </div>

        <!--添加规则-->
        <div>
            <el-drawer :title="row.id==''?'添加规则':'修改规则'" :visible.sync="editshow" v-loading="info_loading"
                       :modal-append-to-body="false"
                       :show-close="true" size="50%">
                <div class="drawerReply">
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>规则名</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="row.title" :maxlength="255" placeholder="请输入规则名"></el-input>
                        </div>

                    </div>
                    <div class="drawerModLis" style="align-items: initial;">
                        <div class="drawerModTite" style="margin-top: 8px;">
                            <span>关键 字</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input type="textarea" v-model="row.keyword" :maxlength="255"
                                      placeholder="请输入关键字"></el-input>
                        </div>
                        <div class="drawerModTips">
                            <el-alert title="设置多个关键字请按英文','分隔" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis" style="align-items: initial;">
                        <div class="drawerModTite" style="margin-top: 10px;">
                            <span>回复消息</span>
                        </div>
                        <div class="drawerModInpt">
                            <div class="ReplyHuifu">
                                <el-tabs v-model="conkey" type="card" @tab-remove="removeTab" @tab-add="tabcon('')"
                                         :closable="true" :addable="addable">
                                    <el-tab-pane v-for="(item, index) in conarr" :key="item.id" :name="'' + index"
                                                 :label="'消息' + (index + 1) + ' ' + item.msgtype_n">
                                        <div class="drawerModLis">
                                            <div class="drawerModTite">
                                                <span>消息类型</span>
                                            </div>
                                            <div class="drawerModInpt">
                                                <el-radio-group v-model="item.msgtype" @input="radio_type">
                                                    <el-radio label="text">文字</el-radio>
                                                    <el-radio label="image">图片</el-radio>
                                                    <el-radio label="xcx">小程序卡片</el-radio>
                                                </el-radio-group>
                                            </div>

                                        </div>

                                        <div class="drawerModLis">
                                            <div class="drawerModTite">
                                                <span>优先级别</span>
                                            </div>
                                            <div class="drawerModInpt">
                                                <el-input v-model="item.sort" placeholder="请输入优先级别"></el-input>
                                            </div>
                                        </div>


                                        <div class="drawerModLis" v-show="item.msgtype == 'text'">
                                            <div class="drawerModTite">
                                                <span>回复内容</span>
                                            </div>
                                            <div class="drawerModInpt">
                                                <el-input type="textarea" :rows="2" placeholder="请输入内容"
                                                          v-model="item.content">
                                                </el-input>
                                            </div>
                                        </div>
                                        <div v-show="item.msgtype == 'xcx'">
                                            <div class="drawerModLis">
                                                <div class="drawerModTite">
                                                    <span>卡片标题</span>
                                                </div>
                                                <div class="drawerModInpt">
                                                    <el-input placeholder="请输入卡片标题" v-model="item.xcx_title"></el-input>
                                                </div>
                                            </div>
                                            <div class="drawerModLis">
                                                <div class="drawerModTite">
                                                    <span>小程序AppID</span>
                                                </div>
                                                <div class="drawerModInpt">
                                                    <el-input placeholder="请输入小程序AppID"
                                                              v-model="item.xcx_appid"></el-input>
                                                </div>
                                            </div>
                                            <div class="drawerModLis">
                                                <div class="drawerModTite">
                                                    <span>小程序路径</span>
                                                </div>
                                                <div class="drawerModInpt">
                                                    <el-input placeholder="请输入跳转小程序页面 实例：pages/index/index"
                                                              v-model="item.xcx_pagepath"></el-input>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="drawerModLis"
                                             v-show="item.msgtype == 'image' || item.msgtype == 'xcx'"
                                             style="align-items: initial;">
                                            <div class="drawerModTite" style="margin-top: 8px;">
                                                <span>{{ item.msgtype == 'image' ? '图片' : '封面图' }}</span>
                                            </div>
                                            <div class="drawerModInpt">
                                                <el-upload :action="uploadAction" class="avatar-uploader" :auto-upload="false"
                                                           :show-file-list="false" :multiple="false"
                                                           :accept="pic_accept"
                                                           :on-change="choosePic" list-type="picture">
                                                    <div v-if="item.image_n">
                                                        <el-image style="width: 88px; height: 88px; margin-top: 8px;"
                                                                  :src="item.image_n"
                                                                  fit="contain"></el-image>
                                                    </div>

                                                    <el-button slot="trigger" size="small" type="primary">选取文件
                                                    </el-button>
                                                    <div slot="tip" class="el-upload__tip"
                                                         v-show="item.msgtype == 'xcx'">
                                                        建议大小520*416，大小2M
                                                    </div>
                                                </el-upload>
                                            </div>
                                        </div>

                                    </el-tab-pane>

                                </el-tabs>
                            </div>
                        </div>
                    </div>
                    <div class="ReplyButn">
                        <el-button type="primary" @click="saveinfo" :loading="post_loading">确 定</el-button>
                    </div>
                </div>
            </el-drawer>
        </div>
    </div>
</template>

<script>
const defaultcon = {
    id: '',
    sort: '',
    msgtype: 'text',
    msgtype_n: '文本',

    content: '',

    media_id: '',
    image_n: '',
    newimage: '',

    xcx_title: '',
    xcx_appid: '',
    xcx_pagepath: '',

    isadd: 1,
    addpic: 0,
};
module.exports = {
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            emptytext: '暂无数据',
            tableData: [],
            total: 0,
            limit: 0,
            currentPage: 1,
			prevPage:0,
            page_sizes: [],
            list_loading: false,
            allchecked: false,
            choosedata: [],
            keyword: '',

            addable: true,
            editshow: false,
            row: {},
            info_loading: false,
            post_loading: false,
            conarr: [],
            conkey: '0',
            con_post: [],
            del_idarr: [],
            newpic: [],
            uploadAction: baseUrl + 'm=common&c=common_upload'
        }
    },
    watch: {
        conarr: {
            handler(val, oldVal) {
                this.addable = this.conarr.length < 3 ? true : false;
            },
            // immediate: true,
            // deep:true,
        },
    },
    mounted() {
        this.getList();
    },
    methods: {

        async getList() {
            let that = this;
            let params = {
                page: that.currentPage,
                limit: that.limit,
                keyword: that.keyword,
            }

            this.list_loading = true;
            that.emptytext = "数据加载中";
            httpPost('m=tool&c=weixinmenu&a=zdkeyword', params).then((result) => {
                this.list_loading = false;
                var res = result.data;

                if (res.error == 0) {
                    that.tableData = res.data.list
                    that.total = parseInt(res.data.total)
                    that.page_sizes = res.data.page_sizes;
                    that.limit = res.data.page_size;
					
					if(that.prevPage != that.currentPage){
						that.prevPage = that.currentPage;
						that.$refs.table.bodyWrapper.scrollTop = 0;
					}
                    if (that.tableData.length === 0) {
                        that.emptytext = "暂无数据";
                    }
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        search: function () {
            this.currentPage = 1;
            this.getList();
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
                message.error('请选择要删除的数据');
                return;
            }
            var params = {
                del: idarr
            };

            delConfirm(_this, params, this.deletePost)
        },
        async deletePost(params) {

            let that = this;

            httpPost('m=tool&c=weixinmenu&a=delkeyword', params).then(function (result) {

                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        location.reload();
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


        editinfo: function (id) {

            var self = this;
            var tabcon_key = '';

            self.info_loading = true;

            httpPost('m=tool&c=weixinmenu&a=getzdkeyword', {id: id}).then(function (result) {

                self.info_loading = false;

                var res = result.data;

                if (res.error == 0) {


                    self.row = res.data.row;
                    var conarr = res.data.row.conarr;
                    for (let i in conarr) {

                        for (let key in defaultcon) {
                            conarr[i][key] = typeof conarr[i][key] != 'undefined' ? conarr[i][key] : defaultcon[key];
                        }
                        conarr[i].isadd = 0;
                        conarr[i].addpic = 0;
                    }

                    self.conarr = conarr;
                    tabcon_key = 0;


                    self.tabcon(tabcon_key);

                    self.editshow = true;
                } else {
                    message.error(res.msg);
                    return;
                }
            }).catch(function (e) {
                console.log(e)
            })

        },
        addinfo: function () {
            this.row = {
                id: '',
                title: '',
                keyword: '',
            };
            this.conarr = [];
            this.con_post = [];
            this.del_idarr = [];
            this.newpic = [];
            this.tabcon('');

            this.editshow = true;
        },
        tabcon: function (ckey) {
            var self = this;

            var msgtype = 'text';
            if (ckey !== '' && self.conarr[ckey]) {
                var con = self.conarr[ckey];
                self.conkey = '' + ckey;
                msgtype = con.msgtype;
            } else {

                if (self.conarr.length >= 3) {
                    message.warning('最多只能添加3条消息！');
                    return;
                }
                var randnum = parseInt(Math.random() * 1000);
                var con = deepClone(defaultcon);
                con.id = randnum;
                self.conarr.push(con);

                self.conkey = '' + (self.conarr.length - 1);
            }

        },
        removeTab: function (key) {
            var self = this;
            key = parseInt(key);

            var conarr = deepClone(self.conarr);
            var del_idarr = [];
            if (conarr[key].isadd == 0) {
                del_idarr.push(conarr[key].id);
            }
            self.del_idarr = del_idarr;
            conarr.splice(key, 1);

            for (let i in self.newpic) {
                if (self.newpic[i] && conarr[key] && (self.newpic[i].id == conarr[key].id)) {
                    self.newpic.splice(i, 1);
                    break;
                }
            }

            self.conarr = conarr;
        },
        radio_type: function (val) {
            var msgtype = val;
            var conkey = parseInt(this.conkey);
            var conarr = deepClone(this.conarr);

            var msgtype_n = '文本';
            if (msgtype == 'image') {
                msgtype_n = '图片';
            } else if (msgtype == 'xcx') {
                msgtype_n = '小程序卡片';
            }
            conarr[conkey].msgtype = msgtype;
            conarr[conkey].msgtype_n = msgtype_n;


            this.conarr = conarr;
        },
        choosePic: function (file, fileList) {

            var conkey = parseInt(this.conkey);
            var conarr = deepClone(this.conarr);
            conarr[conkey].image_n = file.url;
            conarr[conkey].newimage = file.url;
            conarr[conkey].addpic = 1;
            this.newpic.push({id: conarr[conkey].id, file: file.raw});
            this.conarr = conarr;

        },
        checkform: function () {
            var self = this;
            var conarr = deepClone(self.conarr);
            var row = deepClone(self.row);
            var id = '';
            var msgtype = '';
            var errmsg = '';
            var isadd = 0;
            var con_post = [];

            if (row.title == '') {
                message.warning('规则名不能为空！');
                return;
            } else if (row.keyword.trim() == '') {
                message.warning('关键字不能为空！');
                return;
            }

            for (let i in conarr) {

                errmsg = '消息' + parseInt(i + 1) + '的';

                id = conarr[i].id;

                msgtype = conarr[i].msgtype;

                isadd = conarr[i].isadd;

                var conval = {
                    id: id,
                    msgtype: msgtype,
                    isadd: isadd,
                    sort: conarr[i].sort,
                    addpic: conarr[i].addpic,
                };

                if (msgtype == 'image') {

                    if (conarr[i].media_id == '' && conarr[i].newimage == '') {
                        message.warning(errmsg + '图片不能为空！');
                        return false;
                    }

                    conval.newimage = conarr[i].newimage;
                    conval.media_id = conarr[i].media_id;

                } else if (msgtype == 'xcx') {

                    if (conarr[i].xcx_title == '') {
                        message.warning(errmsg + '卡片标题不能为空！');
                        return false;
                    }
                    if (conarr[i].xcx_appid == '') {
                        message.warning(errmsg + '小程序AppID不能为空！');
                        return false;
                    }
                    if (conarr[i].xcx_pagepath == '') {
                        message.warning(errmsg + '小程序路径不能为空！');
                        return false;
                    }
                    if (conarr[i].media_id == '' && conarr[i].newimage == '') {
                        message.warning(errmsg + '小程序封面图不能为空！');
                        return false;
                    }

                    conval.xcx_title = conarr[i].xcx_title;
                    conval.xcx_appid = conarr[i].xcx_appid;
                    conval.xcx_pagepath = conarr[i].xcx_pagepath;
                    conval.newimage = conarr[i].newimage;
                    conval.media_id = conarr[i].media_id;
                    conval.image_n = conarr[i].image_n;

                } else {

                    if (conarr[i].content == '') {
                        message.warning(errmsg + '文本内容不能为空！');
                        return false;
                    }
                    conval.addpic = 0;
                    conval.content = conarr[i].content;
                }

                con_post.push(conval);

                self.con_post = con_post;
            }

            return true;

        },
        saveinfo: function () {

            var self = this;

            var cres = self.checkform();
            if (!cres) {
                return false;
            }

            var params = new FormData();
            var con_post = deepClone(self.con_post);
            var newpic = self.newpic;

            for (let i in con_post) {
                if (con_post[i].msgtype == 'text') {
                    con_post[i].content = con_post[i].content.replace(/"/g, '&quot;').replace(/'/g, '&apos;');
                } else if (con_post[i].msgtype == 'xcx') {
                    con_post[i].xcx_title = con_post[i].xcx_title.replace(/"/g, '&quot;').replace(/'/g, '&apos;');
                }
                if (con_post[i].addpic == 1) {
                    for (let j in newpic) {
                        if (newpic[j].id == con_post[i].id) {

                            params.append('newpic[]', newpic[j].file);
                        }
                    }
                }
            }

            var conpost = JSON.stringify(con_post);

            params.append('id', self.row.id);
            params.append('title', self.row.title);
            params.append('keyword', self.row.keyword);
            params.append('content', conpost);
            params.append('del_idarr', self.del_idarr);


            self.post_loading = true;

            httpPost('m=tool&c=weixinmenu&a=saveZdKeyword'
                , params
                , {
                    headers: {'Content-Type': 'multipart/form-data'},
                }
            ).then((result) => {

                self.post_loading = false;

                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, () => {
                        self.editshow = false;
                        self.getList();
                    });
                } else {
                    message.error(res.msg);
                }

            }).catch(function (e) {
                console.log(e)
            })
        },
        doLayout(){
            if (this.$refs.table) {
                this.$nextTick(() => {
                    this.$refs.table.doLayout();
                })
            }
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
    height: calc(100% - 136px);
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
    padding: 0px 5px;;
}

.el-dialog__body {
    padding: 0px 20px;
}

.el-tag + .el-tag {
    margin-left: 10px;
}

.button-new-tag {
    margin-left: 10px;
    height: 32px;
    line-height: 30px;
    padding-top: 0;
    padding-bottom: 0;
}

.input-new-tag {
    width: 90px;
    margin-left: 10px;
    vertical-align: bottom;
}
</style>