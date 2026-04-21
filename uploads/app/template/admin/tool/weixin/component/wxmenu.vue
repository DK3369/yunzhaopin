<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="最多创建3个一级菜单，一级菜单名称不多于4个汉字或8个字母。一级菜单下的子菜单最多可创建5个，子菜单名称不多于8个汉字或16个字母。" type="success"
                      :closable="false">
            </el-alert>
        </div>
        <div class="moduleSeachs">
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="navsync"
                           plain>同步微信菜单
                </el-button>
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="addinfo">增加微信菜单</el-button>
            </div>
        </div>

        <div class="moduleElTable">
            <el-table ref="table" :data="tableData" v-loading="list_loading" @selection-change="selectionChange"
                      style="width: 100%" row-key="id" border default-expand-all
                      :tree-props="{ children: 'list'}"
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                      :empty-text="emptytext">
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="name" label="菜单标题">
                    <template slot-scope="scope">
				        <span v-if="editname_id==scope.row.id">
                            <el-input id="inputref" placeholder="请输入内容" v-model="editname" :data-preval="scope.row.name"
                                      data-type="name" @blur="editChange" clearable></el-input>
				        </span>
                        <div class="moduleElTaPax" v-else>
                            <span>{{ scope.row.name }}</span>
                            <img src="../../../admin/images/bine.png"
                                 @click="editcolumn('name',scope.row.name,scope.row.id)" alt="">
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="type" label="菜单类型" width="180">
                </el-table-column>
                <el-table-column prop="key" label="菜单关键字">
                </el-table-column>
                <el-table-column prop="url" label="菜单链接">
                </el-table-column>
                <el-table-column label="	排序	" width="180">
                    <template slot-scope="scope">
                        <div class="moduleElTaPax" v-if="editsort_id==scope.row.id">
                            <el-input id="inputref" placeholder="请输入内容" v-model="editsort" :data-preval="scope.row.sort"
                                      onKeyUp="this.value=this.value.replace(/[^0-9.]/g,'')" data-type="sort"
                                      @blur="editChange" clearable></el-input>
                        </div>
                        <div class="moduleElTaPax" v-else>
                            <span>{{ scope.row.sort }}</span>
                            <img src="../../../admin/images/bine.png"
                                 @click="editcolumn('sort',scope.row.sort,scope.row.id)" alt="">
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="操作" width="130" fixed="right" header-align="center">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" @click="editinfo(scope.row)">修改</el-button>
                            <el-button size="mini" type="danger" @click="deleteinfo(scope.row.id)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>

        </div>
        <div class="otherPageButn">
            <div class="modulePaging">
                <div class="modulecz modulePagButn">
                    <el-checkbox v-model="allchecked" @change="allcheckChange">全选</el-checkbox>
                    <el-button size="mini" @click="deleteAll">批量删除</el-button>
                </div>
            </div>
        </div>
        <!--新增微信菜单-->
        <div class="modluDrawer">
            <el-dialog title="新增微信菜单" :visible.sync="editshow" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="440px">
                <div>
                    <div class="wxsettip_small ">菜单标题</div>
                    <el-input v-model="einfo.name"></el-input>
                    <div class="wxsettip_small ">菜单数组</div>
                    <div class="wxsettip_smallselect ">
                        <el-select v-model="einfo.keyid">
                            <el-option key="0" label="一级菜单" value="0"></el-option>
                            <el-option v-for="item in tableData" :key="item.id" :label="item.name"
                                       :value="item.id"></el-option>
                        </el-select>
                    </div>
                    <div class="wxsettip_small ">菜单类型</div>
                    <div class="wxsettip_smallselect ">
                        <el-select v-model="einfo.type" placeholder="请选择">
                            <el-option label="点击事件" value="click"></el-option>
                            <el-option label="链接事件" value="view"></el-option>
                            <el-option label="小程序" value="miniprogram"></el-option>
                        </el-select>
                    </div>
                    <div v-show="einfo.type=='click'">
                        <div class="wxsettip_small ">菜单关键字</div>
                        <el-input v-model="einfo.key"></el-input>
                    </div>
                    <div v-show="einfo.type=='view'">
                        <div class="wxsettip_small ">菜单链接</div>
                        <el-input v-model="einfo.url"></el-input>
                    </div>
                    <div v-show="einfo.type=='miniprogram'">
                        <div class="wxsettip_small ">菜单链接</div>
                        <el-input v-model="einfo.url"></el-input>
                        <div class="wxsettip_small ">小程序APPID</div>
                        <el-input v-model="einfo.appid"></el-input>
                        <div class="wxsettip_small ">小程序入口文件</div>
                        <el-input v-model="einfo.apppage"></el-input>
                    </div>

                    <div class="wxsettip_small ">排序</div>
                    <el-input v-model="einfo.sort"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="editshow = false">取 消</el-button>
					<el-button type="primary" @click="saveinfo" :loading="post_loading">确 定</el-button>
				</span>
            </el-dialog>
        </div>
    </div>
</template>

<script>
var timer = null;
module.exports = {
    data: function () {
        return {
            emptytext: '暂无数据',
            tableData: [],
            list_loading: false,
            choosedata: [],
            allchecked: false,
            editshow: false,
            einfo: {},
            post_loading: false,

            editname_id: '',
            editsort_id: '',
            editname: '',
            editsort: '',
        }
    },

    mounted() {
        this.getList();
    },
    methods: {
        async getList() {
            let that = this;
            let params = {};

            this.list_loading = true;
            that.emptytext = "数据加载中";
            httpPost('m=tool&c=weixinmenu&a=wxnav', params).then((result) => {
                this.list_loading = false;
                var res = result.data
                if (res.error == 0) {
                    that.tableData = res.data.list;
                    if (that.tableData.length === 0) {
                        that.emptytext = "暂无数据";
                    }
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        selectionChange: function (e) {

            this.choosedata = e;
        },
        allcheckChange: function () {

            this.$refs.table.toggleAllSelection();

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

            httpPost('m=tool&c=weixinmenu&a=delnav', params).then(function (result) {

                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList();
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
        editinfo: function (row) {

            this.einfo = deepClone(row);
            this.editshow = true;
        },
        addinfo: function () {
            var that = this;
            this.einfo = {
                id: '',
                appid: '',
                apppage: '',
                key: '',
                keyid: '0',
                name: '',
                type: '',
                url: '',
                sort: '',
            };
            this.editshow = true;
        },
        saveinfo: function () {

            var that = this,
                param = {};
            if (this.einfo.name == '') {
                message.warning('菜单名称不能为空！');
                return;
            }
            if (this.einfo.keyid != '0' && this.einfo.type == 'click' && this.einfo.key == '') {
                message.warning('点击事件，菜单关键字不得为空！');
                return;
            }
            if (this.einfo.keyid != '0' && this.einfo.type == 'view' && this.einfo.url == '') {
                message.warning('链接事件，菜单链接不得为空！');
                return;
            }

            param.navid = this.einfo.id;
            param.name = this.einfo.name;
            param.keyid = this.einfo.keyid;
            param.type = this.einfo.type;
            param.key = this.einfo.key;
            param.url = this.einfo.url;
            param.sort = this.einfo.sort;
            param.appid = this.einfo.appid;
            param.apppage = this.einfo.apppage;
            param.apppage = this.einfo.apppage;

            that.post_loading = true;

            httpPost('m=tool&c=weixinmenu&a=savenav', param).then((result) => {

                that.post_loading = false;

                var res = result.data;

                if (res.error == 1) {
                    message.error('请按要求填写信息！');
                    return false;
                } else if (res.error == 2) {
                    message.error('相同名称或关键字已存在！');
                    return false;
                } else if (res.error == 3) {
                    message.success('操作成功！', () => {
                        that.editshow = false;
                        that.getList();
                    });
                    return false;
                } else if (res.error == 4) {
                    message.success('操作成功！', () => {
                        that.editshow = false;
                        that.getList();
                    });
                    return false;
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        editcolumn: function (type, def, id) {

            this[`edit${type}_id`] = id;
            this[`edit${type}`] = def;

            this.$nextTick(() => {
                if (timer) {
                    clearTimeout(timer);
                }
                timer = setTimeout(() => {
                    document.getElementById('inputref').focus();
                }, 100);
            })

        },
        async editChange(e) {

            var that = this;

            var preval = e.target.dataset.preval;
            var type = e.target.dataset.type;

            var val = this[`edit${type}`];
            var id = this[`edit${type}_id`];

            if (val == preval) {

                this[`edit${type}_id`] = '';
                this[`edit${type}`] = '';

            } else {
                if (type == 'name' && val == '') {
                    this[`edit${type}_id`] = '';
                    message.error('类别名称不能为空！');
                    return;
                }
                var param = {id: id};
                param[`${type}`] = val;

                httpPost('m=tool&c=weixinmenu&a=ajaxnav', param).then(function (result) {

                    for (let i in that.tableData) {
                        if (that.tableData[i].id == id) {
                            that.tableData[i][`${type}`] = val;
                            break;
                        }
                    }

                    that[`edit${type}_id`] = '';
                    that[`edit${type}`] = '';
                    message.success('修改成功', function () {
                        that.getList()
                    });
                }).catch(function (e) {
                    console.log(e)
                })
            }

        },

        async navsync(params) {

            let that = this;
            delConfirm(this, {}, function () {
                httpPost('m=tool&c=weixinmenu&a=creatnav', {}).then(function (response) {
                    let res = response.data;

                    if (res.error == 0) {
                        message.success(res.msg);
                    } else {
                        message.error(res.msg);
                    }
                })
            }, '确定要同步菜单至微信服务器？');
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

.el-table .cell {
    display: flex;
    align-items: center;
}

.el-dialog__body {
    padding: 0px 20px;
}
</style>