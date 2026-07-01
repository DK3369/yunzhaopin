<template>
    <div class="drawerModlue">
        <!-- <div class="tableDome_tip">
            <el-alert :title="headTip" type="warning">
            </el-alert>
        </div> -->
        <div class="tableDome_tip tableDoAlert">
            <span>{{ lc('admin_00971') }}<br />{{ lc('admin_00972') }}</span>
            <a href="https://ziyuan.baidu.com/college/documentinfo?id=1576&amp;qq-pf-to=pcqq.c2c" target="_blank" style="color:#00F">{{ lc('admin_00973') }}</a>
        </div>
        <div class="drawerModInfo drawerModInfoOne drawerModInguding">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00372') }}</span>
                </div>
                <div v-if="call == 'seo'" class="drawerModInpt">
                    <el-select v-model="ruleForm.seomodel" :placeholder="lc('wap_user_00100')">
                        <el-option v-for="(smitem, smindex) in seomodel" :key="smindex" :label="smitem" :value="smindex">
                        </el-option>
                    </el-select>
                </div>
                <div v-if="call == 'module'" class="drawerModInpt">
                    <el-select v-model="ruleForm.seoid" @change="changeSeoid" :placeholder="lc('wap_user_00100')">
                        <el-option v-for="seoItem in seo" :key="seoItem.id" :label="seoItem.seoname" :value="seoItem.id">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00377') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.seoname" :placeholder="lc('wap_user_00076')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00371') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.ident" :placeholder="lc('wap_user_00076')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_user_00126') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="ruleForm.did" :placeholder="lc('wap_user_00100')">
                        <el-option v-for="(ditem, dindex) in Dname" :key="dindex" :label="ditem" :value="dindex">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00373') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.rewrite_url" :placeholder="lc('wap_user_00076')"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert :title="lc('admin_00976')" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00375') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.php_url" :placeholder="lc('wap_user_00076')"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert :title="lc('admin_vue_00058')" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00364') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.rewrite_wap_url" :placeholder="lc('wap_user_00076')"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert :title="lc('admin_00976')" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00370') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.php_wap_url" :placeholder="lc('wap_user_00076')"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert :title="lc('admin_00977')" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00362') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')" v-model="ruleForm.title" @blur="textareaBlur($event, 'title')">
                    </el-input>
                    <el-button type="info" @click="openCenterDialog('title')">{{ lc('admin_system_00376') }}</el-button>
                </div>
                <!-- <div class="drawerModTips drawerMoAlert">
                    <el-alert :title="titleTip" type="info" show-icon :closable="false">
                    </el-alert>
                </div> -->
                <div class="drawerModTips drawerMoAlert">
                    <i class="el-icon-info"></i>
                    <span>{{ lc('admin_00974') }}</span>
                    <a href="https://ziyuan.baidu.com/college/documentinfo?id=1576&amp;qq-pf-to=pcqq.c2c" target="_blank" style="color:#00F">{{ lc('admin_00973') }}</a>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00360') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')" v-model="ruleForm.keywords" @blur="textareaBlur($event, 'keywords')">
                    </el-input>
                    <el-button type="info" @click="openCenterDialog('keywords')">{{ lc('admin_system_00376') }}</el-button>
                </div>
                <div class="drawerModTips">
                    <el-alert :title="lc('admin_00978')" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00359') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')" v-model="ruleForm.description" @blur="textareaBlur($event, 'description')">
                    </el-input>
                    <el-button type="info" @click="openCenterDialog('description')">{{ lc('admin_system_00376') }}</el-button>
                </div>
                <div class="drawerModTips">
                    <el-alert :title="lc('admin_00979')" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
        </div>
        <div class="modluDialog">
            <el-drawer :title="lc('admin_system_00374')" :visible.sync="centerDialogVisible" :append-to-body="true" :show-close="true" :with-header="true" size="40%">
                <div style="overflow-y: auto; position: relative; width: 100%; height: calc(100% - 70px); padding: 0 20px;">
                    <div class="tableDome_tip">
                        <el-alert :title="lc('admin_system_00358')" type="warning">
                        </el-alert>
                    </div>
                    <div v-for="(scitem, scindex) in seoconfigList" :key="scindex" v-if="checkSeoconfig(scitem.seomodel)">
                        <el-table ref="multipleTable" :data="scitem.tableData" tooltip-effect="dark" style="width: 100%" @selection-change="handleSelectionChange" v-loading="loading" :empty-text="emptytext">
                            <el-table-column type="selection" width="55">
                            </el-table-column>
                            <el-table-column :label="lc('member_com_00207')" prop="title" width="150">
                            </el-table-column>
                            <el-table-column :label="lc('admin_system_00379')">
                                <template slot-scope="scope">{{ '{' + scope.row.code + '}' }}</template>
                            </el-table-column>
                        </el-table>
                    </div>
                </div>
                <div class="dialofhooter">
                    <el-button type="primary" @click="confirmSelection">{{ lc('wap_com_00019') }}</el-button>
                    <el-button @click="centerDialogVisible = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                </div>
            </el-drawer>
        </div>
        <!-- <div class="modluDialog">
            <el-dialog :title="lc('admin_system_00374')" :visible.sync="centerDialogVisible" :append-to-body="true" :modal="false" width="30%" center>
                <div>
                    <div class="tableDome_tip">
                        <el-alert :title="lc('admin_system_00358')" type="warning">
                        </el-alert>
                    </div>
                    <div v-for="(scitem, scindex) in seoconfigList" :key="scindex" v-if="checkSeoconfig(scitem.seomodel)">
                        <el-table ref="multipleTable" :data="scitem.tableData" tooltip-effect="dark" style="width: 100%" @selection-change="handleSelectionChange" v-loading="loading">
                            <el-table-column type="selection" width="55">
                            </el-table-column>
                            <el-table-column :label="lc('member_com_00207')" prop="title" width="150">
                            </el-table-column>
                            <el-table-column :label="lc('admin_system_00379')">
                                <template slot-scope="scope">{{ '{' + scope.row.code + '}' }}</template>
                            </el-table-column>
                        </el-table>
                    </div>
                </div>
                <div slot="footer" class="dialog-footer">
                    <el-button type="primary" @click="confirmSelection" :disabled="saveLoading">{{ lc('common_02016') }}</el-button>
                    <el-button @click="centerDialogVisible = false">{{ lc('wap_js_00080') }}</el-button>
                </div>
            </el-dialog>
        </div> -->
    </div>
</template>
<script>
module.exports = {
    props: ['call', 'config', 'seoid', 'detail'],
    data: function() {
        return {
            emptytext: lc('wap_js_00113'),
            loading: false,
            Dname: {},
            seo: [],
            ruleForm: {
                did: '0'
            },

            centerDialogVisible: false,
            openType: '',
            openTypeBlur: [],
            seoconfigList: [],
            multipleSelection: [],

            seomodel: {},
            saveLoading: false
        }
    },

    mounted() {

    },
    created: function() {
        this.getInfo();
    },
    methods: {
        getInfo() {
            let that = this,
                call = that.call,
                url = '',
                params = {};
            if (call == 'seo') {
                url = "m=system&c=set_seo&a=seoadd";
                params = { id: that.seoid };
            } else if (call == 'module') {
                url = "m=system&c=set_module&a=seoshezhi";
                params = { config: that.config };
            }
            that.loading = true;
            that.emptytext = lc('admin_user_weipin_00026');
            httpPost(url, params).then(function(response) {
                let data = response.data.data;

                that.Dname = data.Dname;
                if (call == 'seo') {
                    that.seomodel = data.seomodel;
                    if (data.info) {
                        that.ruleForm = data.info;
                    }
                } else if (call == 'module') {
                    that.seo = data.seo;
                }

                let seoconfig = data.seoconfig,
                    tableData = [],
                    seoconfigList = [],
                    publicList = [];

                if (seoconfig.public) {
                    for (let pkey in seoconfig.public) {
                        publicList.push({ code: pkey, title: seoconfig.public[pkey] });
                    }
                }

                for (let key in seoconfig) {
                    tableData = [];
                    if (key == 'public') { // 跳过public部分，避免重复拼接
                        seoconfigList.push({ seomodel: key, tableData: publicList });
                        continue;
                    }

                    for (let key2 in seoconfig[key]) {
                        tableData.push({ code: key2, title: seoconfig[key][key2] });
                    }

                    seoconfigList.push({ seomodel: key, tableData: publicList.concat(tableData) });
                }

                that.seoconfigList = seoconfigList;
                that.loading = false;
                if (that.seoconfigList.length === 0){
                    that.emptytext = lc('wap_js_00113');
                }
            })
        },
        // 模块设置专用
        changeSeoid(val) {
            let that = this;

            httpPost('m=system&c=set_module&a=getseo', { id: val }).then(function(response) {
                let data = response.data.data;

                that.$set(that.ruleForm, 'description', data.description);
                that.$set(that.ruleForm, 'did', data.did);
                that.$set(that.ruleForm, 'ident', data.ident);
                that.$set(that.ruleForm, 'keywords', data.keywords);
                that.$set(that.ruleForm, 'php_url', data.php_url);
                that.$set(that.ruleForm, 'rewrite_url', data.rewrite_url);
                that.$set(that.ruleForm, 'seoname', data.seoname);
                that.$set(that.ruleForm, 'title', data.title);
                that.$set(that.ruleForm, 'rewrite_wap_url', data.rewrite_wap_url);
                that.$set(that.ruleForm, 'php_wap_url', data.php_wap_url);
            })
        },
        checkSeoconfig(seomodel) {
            if (this.call == 'seo') {
                if (this.ruleForm.seomodel && this.ruleForm.seomodel != 'index') {
                    return this.ruleForm.seomodel == seomodel
                } else {
                    return 'public' == seomodel;
                }
            } else if (this.call == 'module') {
                if (this.config && this.config != 'index') {
                    return this.config == seomodel;
                } else {
                    return 'public' == seomodel;
                }
            }
        },
        openCenterDialog(val) {
            this.openType = val;
            this.centerDialogVisible = true;
        },
        // 批量选中
        handleSelectionChange(val) {
            this.multipleSelection = val;
        },
        textareaBlur(e, val) {
            let openTypeBlur = [];
            openTypeBlur[val] = e.srcElement.selectionStart;
            this.openTypeBlur = openTypeBlur;
        },
        confirmSelection() {
            let code = '';
            this.multipleSelection.forEach(function(item, index) {
                code += code == '' ? `{${item.code}}` : `-{${item.code}}`;
            })

            if (this.ruleForm[this.openType]) {
                let content = this.ruleForm[this.openType];
                if (this.openTypeBlur[this.openType]) {
                    let index = this.openTypeBlur[this.openType];
                    this.$set(this.ruleForm, this.openType, content.slice(0, index) + code + content.slice(index)); // 光标位置插入
                } else {
                    this.openTypeBlur = []; // 清空失焦记录
                    this.$set(this.ruleForm, this.openType, content + code);
                }
            } else {
                this.$set(this.ruleForm, this.openType, code);
            }

            this.$refs.multipleTable[0].clearSelection();
            this.multipleSelection = [];
            this.centerDialogVisible = false;
        },
        save() {
            let that = this,
                ruleForm = that.ruleForm,
                call = that.call,
                url = '';

            if (call == 'seo' && !ruleForm.seomodel) {
                message.warning(lc('admin_system_00369'));
                return false;
            }
            if (call == 'module' && !ruleForm.seoid) {
                message.warning(lc('admin_system_00361'));
                return false;
            }
            if (!ruleForm.seoname || ruleForm.seoname == "") {
                message.warning(lc('admin_system_00365'));
                return false;
            }
            if (!ruleForm.ident || ruleForm.ident == "") {
                message.warning(lc('admin_system_00367'));
                return false;
            }
            if (!ruleForm.title || ruleForm.title == "") {
                message.warning(lc('admin_system_00368'));
                return false;
            }
            if (!ruleForm.keywords || ruleForm.keywords == "") {
                message.warning(lc('admin_system_00363'));
                return false;
            }
            if (!ruleForm.description || ruleForm.description == "") {
                message.warning(lc('admin_system_00366'));
                return false;
            }

            if (call == 'seo') {
                url = "m=system&c=set_seo&a=save";
            } else if (call == 'module') {
                ruleForm.id = ruleForm.seoid;
                url = "m=system&c=set_module&a=seoshezhi";
            }
            that.saveLoading = true;
            httpPost(url, ruleForm).then(function(response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    message.success(res.msg, function() {
                        that.$emit("child-event");
                        if (call == 'seo') {
                            if (custoapp.curTab == ruleForm.seomodel) {
                                custoapp.seotabRefresh(); // {{ lc('wap_user_00334') }}TAB{{ lc('wap_00316') }}
                            }
                        }
                    })
                }
            }).finally(function () {
                setTimeout(function () {
                    that.saveLoading = false;
                }, 2000);
            });
        },
    },
    watch: {
        config: function(val, oldVal) {
            this.ruleForm = {
                did: "0'
            };

            if (this.call == 'module') {
                this.getInfo();
            }
        },
        seoid: function(val, oldVal) {
            this.ruleForm = {
                did: '0'
            };

            if (this.call == 'seo') {
                this.getInfo();
            }
        },
    }
};
</script>
<style>
.drawerModInfo::-webkit-scrollbar {
    display: none;
}

.el-dialog-s {
    z-index: 11;
}
.dialofhooter{
    overflow: hidden;
    position: relative;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    padding-top: 20px;
}
.dialofhooter .el-button{
    width: 100px;
}
</style>