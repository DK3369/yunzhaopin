<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="{yun:}t key='admin_tool_00641'{/yun}" type="success"
                      :closable="false">
            </el-alert>
        </div>
        <div class="moduleSeachs">
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="navsync"
                           plain>{yun:}t key='admin_tool_00659'{/yun}
                </el-button>
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="addinfo">{yun:}t key='admin_tool_00660'{/yun}</el-button>
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
                <el-table-column prop="name" label="{yun:}t key='admin_tool_00654'{/yun}">
                    <template slot-scope="scope">
				        <span v-if="editname_id==scope.row.id">
                            <el-input id="inputref" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="editname" :data-preval="scope.row.name"
                                      data-type="name" @blur="editChange" clearable></el-input>
				        </span>
                        <div class="moduleElTaPax" v-else>
                            <span>{{ scope.row.name }}</span>
                            <img src="../../../admin/images/bine.png"
                                 @click="editcolumn('name',scope.row.name,scope.row.id)" alt="">
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="type" label="{yun:}t key='admin_tool_00655'{/yun}" width="180">
                </el-table-column>
                <el-table-column prop="key" label="{yun:}t key='admin_tool_00650'{/yun}">
                </el-table-column>
                <el-table-column prop="url" label="{yun:}t key='admin_tool_00656'{/yun}">
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_vue_00044'{/yun}" width="180">
                    <template slot-scope="scope">
                        <div class="moduleElTaPax" v-if="editsort_id==scope.row.id">
                            <el-input id="inputref" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="editsort" :data-preval="scope.row.sort"
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
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="130" fixed="right" header-align="center">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" @click="editinfo(scope.row)">{yun:}t key='wap_js_00073'{/yun}</el-button>
                            <el-button size="mini" type="danger" @click="deleteinfo(scope.row.id)">{yun:}t key='common.delete'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>

        </div>
        <div class="otherPageButn">
            <div class="modulePaging">
                <div class="modulecz modulePagButn">
                    <el-checkbox v-model="allchecked" @change="allcheckChange">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                    <el-button size="mini" @click="deleteAll">{yun:}t key='member_com_00055'{/yun}</el-button>
                </div>
            </div>
        </div>
        <!--新增微信菜单-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_tool_00661'{/yun}" :visible.sync="editshow" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="440px">
                <div>
                    <div class="wxsettip_small ">{yun:}t key='admin_tool_00654'{/yun}</div>
                    <el-input v-model="einfo.name"></el-input>
                    <div class="wxsettip_small ">{yun:}t key='admin_tool_00653'{/yun}</div>
                    <div class="wxsettip_smallselect ">
                        <el-select v-model="einfo.keyid">
                            <el-option key="0" label="{yun:}t key='admin_tool_00651'{/yun}" value="0"></el-option>
                            <el-option v-for="item in tableData" :key="item.id" :label="item.name"
                                       :value="item.id"></el-option>
                        </el-select>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_tool_00655'{/yun}</div>
                    <div class="wxsettip_smallselect ">
                        <el-select v-model="einfo.type" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                            <el-option label="{yun:}t key='admin_tool_00652'{/yun}" value="click"></el-option>
                            <el-option label="{yun:}t key='admin_tool_00657'{/yun}" value="view"></el-option>
                            <el-option label="{yun:}t key='admin_tool_00658'{/yun}" value="miniprogram"></el-option>
                        </el-select>
                    </div>
                    <div v-show="einfo.type=='click'">
                        <div class="wxsettip_small ">{yun:}t key='admin_tool_00650'{/yun}</div>
                        <el-input v-model="einfo.key"></el-input>
                    </div>
                    <div v-show="einfo.type=='view'">
                        <div class="wxsettip_small ">{yun:}t key='admin_tool_00656'{/yun}</div>
                        <el-input v-model="einfo.url"></el-input>
                    </div>
                    <div v-show="einfo.type=='miniprogram'">
                        <div class="wxsettip_small ">{yun:}t key='admin_tool_00656'{/yun}</div>
                        <el-input v-model="einfo.url"></el-input>
                        <div class="wxsettip_small ">{yun:}t key='admin_tool_00648'{/yun}</div>
                        <el-input v-model="einfo.appid"></el-input>
                        <div class="wxsettip_small ">{yun:}t key='admin_tool_00649'{/yun}</div>
                        <el-input v-model="einfo.apppage"></el-input>
                    </div>

                    <div class="wxsettip_small ">{yun:}t key='member_com_00022'{/yun}</div>
                    <el-input v-model="einfo.sort"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="editshow = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
					<el-button type="primary" @click="saveinfo" :loading="post_loading">{yun:}t key='wap_com_00019'{/yun}</el-button>
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
            emptytext: window.yunAdminT("{yun:}t key='wap_js_00113'{/yun}"),
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
            that.emptytext = window.yunAdminT("{yun:}t key='admin_user_weipin_00026'{/yun}");
            httpPost('m=tool&c=weixinmenu&a=wxnav', params).then((result) => {
                this.list_loading = false;
                var res = result.data
                if (res.error == 0) {
                    that.tableData = res.data.list;
                    if (that.tableData.length === 0) {
                        that.emptytext = window.yunAdminT("{yun:}t key='wap_js_00113'{/yun}");
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
                message.error(window.yunAdminT("{yun:}t key='admin_user_weipin_00005'{/yun}"));
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
                message.warning(window.yunAdminT("{yun:}t key='admin_tool_00646'{/yun}"));
                return;
            }
            if (this.einfo.keyid != '0' && this.einfo.type == 'click' && this.einfo.key == '') {
                message.warning(window.yunAdminT("{yun:}t key='admin_tool_00642'{/yun}"));
                return;
            }
            if (this.einfo.keyid != '0' && this.einfo.type == 'view' && this.einfo.url == '') {
                message.warning(window.yunAdminT("{yun:}t key='admin_tool_00644'{/yun}"));
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
                    message.error(window.yunAdminT("{yun:}t key='admin_tool_00647'{/yun}"));
                    return false;
                } else if (res.error == 2) {
                    message.error(window.yunAdminT("{yun:}t key='admin_tool_00645'{/yun}"));
                    return false;
                } else if (res.error == 3) {
                    message.success(window.yunAdminT("{yun:}t key='wap_js_00159'{/yun}"), () => {
                        that.editshow = false;
                        that.getList();
                    });
                    return false;
                } else if (res.error == 4) {
                    message.success(window.yunAdminT("{yun:}t key='wap_js_00159'{/yun}"), () => {
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
                    message.error(window.yunAdminT("{yun:}t key='admin_00208'{/yun}"));
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
                    message.success(window.yunAdminT("{yun:}t key='admin_user_company_00208'{/yun}"), function () {
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
            }, window.yunAdminT("{yun:}t key='admin_tool_00643'{/yun}"));
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