<template>
    <div class="drawerModlue">
        <!-- <div class="tableDome_tip">
            <el-alert :title="headTip" type="warning">
            </el-alert>
        </div> -->
        <div class="tableDome_tip tableDoAlert">
            <span>{yun:}t key='admin_00971'{/yun}<br />{yun:}t key='admin_00972'{/yun}</span>
            <a href="https://ziyuan.baidu.com/college/documentinfo?id=1576&amp;qq-pf-to=pcqq.c2c" target="_blank" style="color:#00F">{yun:}t key='admin_00973'{/yun}</a>
        </div>
        <div class="drawerModInfo drawerModInfoOne drawerModInguding">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00372'{/yun}</span>
                </div>
                <div v-if="call == 'seo'" class="drawerModInpt">
                    <el-select v-model="ruleForm.seomodel" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                        <el-option v-for="(smitem, smindex) in seomodel" :key="smindex" :label="smitem" :value="smindex">
                        </el-option>
                    </el-select>
                </div>
                <div v-if="call == 'module'" class="drawerModInpt">
                    <el-select v-model="ruleForm.seoid" @change="changeSeoid" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                        <el-option v-for="seoItem in seo" :key="seoItem.id" :label="seoItem.seoname" :value="seoItem.id">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00377'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.seoname" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00371'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.ident" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_00126'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="ruleForm.did" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                        <el-option v-for="(ditem, dindex) in Dname" :key="dindex" :label="ditem" :value="dindex">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00373'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.rewrite_url" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_00976'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00375'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.php_url" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_vue_00058'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00364'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.rewrite_wap_url" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_00976'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00370'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.php_wap_url" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_00977'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00362'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ruleForm.title" @blur="textareaBlur($event, 'title')">
                    </el-input>
                    <el-button type="info" @click="openCenterDialog('title')">{yun:}t key='admin_system_00376'{/yun}</el-button>
                </div>
                <!-- <div class="drawerModTips drawerMoAlert">
                    <el-alert :title="titleTip" type="info" show-icon :closable="false">
                    </el-alert>
                </div> -->
                <div class="drawerModTips drawerMoAlert">
                    <i class="el-icon-info"></i>
                    <span>{yun:}t key='admin_00974'{/yun}</span>
                    <a href="https://ziyuan.baidu.com/college/documentinfo?id=1576&amp;qq-pf-to=pcqq.c2c" target="_blank" style="color:#00F">{yun:}t key='admin_00973'{/yun}</a>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00360'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ruleForm.keywords" @blur="textareaBlur($event, 'keywords')">
                    </el-input>
                    <el-button type="info" @click="openCenterDialog('keywords')">{yun:}t key='admin_system_00376'{/yun}</el-button>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_00978'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00359'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ruleForm.description" @blur="textareaBlur($event, 'description')">
                    </el-input>
                    <el-button type="info" @click="openCenterDialog('description')">{yun:}t key='admin_system_00376'{/yun}</el-button>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_00979'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
        <div class="modluDialog">
            <el-drawer title="{yun:}t key='admin_system_00374'{/yun}" :visible.sync="centerDialogVisible" :append-to-body="true" :show-close="true" :with-header="true" size="40%">
                <div style="overflow-y: auto; position: relative; width: 100%; height: calc(100% - 70px); padding: 0 20px;">
                    <div class="tableDome_tip">
                        <el-alert title="{yun:}t key='admin_system_00358'{/yun}" type="warning">
                        </el-alert>
                    </div>
                    <div v-for="(scitem, scindex) in seoconfigList" :key="scindex" v-if="checkSeoconfig(scitem.seomodel)">
                        <el-table ref="multipleTable" :data="scitem.tableData" tooltip-effect="dark" style="width: 100%" @selection-change="handleSelectionChange" v-loading="loading" :empty-text="emptytext">
                            <el-table-column type="selection" width="55">
                            </el-table-column>
                            <el-table-column label="{yun:}t key='member_com_00207'{/yun}" prop="title" width="150">
                            </el-table-column>
                            <el-table-column label="{yun:}t key='admin_system_00379'{/yun}">
                                <template slot-scope="scope">{{ '{' + scope.row.code + '}' }}</template>
                            </el-table-column>
                        </el-table>
                    </div>
                </div>
                <div class="dialofhooter">
                    <el-button type="primary" @click="confirmSelection">{yun:}t key='wap_com_00019'{/yun}</el-button>
                    <el-button @click="centerDialogVisible = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                </div>
            </el-drawer>
        </div>
        <!-- <div class="modluDialog">
            <el-dialog title="{yun:}t key='admin_system_00374'{/yun}" :visible.sync="centerDialogVisible" :append-to-body="true" :modal="false" width="30%" center>
                <div>
                    <div class="tableDome_tip">
                        <el-alert title="{yun:}t key='admin_system_00358'{/yun}" type="warning">
                        </el-alert>
                    </div>
                    <div v-for="(scitem, scindex) in seoconfigList" :key="scindex" v-if="checkSeoconfig(scitem.seomodel)">
                        <el-table ref="multipleTable" :data="scitem.tableData" tooltip-effect="dark" style="width: 100%" @selection-change="handleSelectionChange" v-loading="loading">
                            <el-table-column type="selection" width="55">
                            </el-table-column>
                            <el-table-column label="{yun:}t key='member_com_00207'{/yun}" prop="title" width="150">
                            </el-table-column>
                            <el-table-column label="{yun:}t key='admin_system_00379'{/yun}">
                                <template slot-scope="scope">{{ '{' + scope.row.code + '}' }}</template>
                            </el-table-column>
                        </el-table>
                    </div>
                </div>
                <div slot="footer" class="dialog-footer">
                    <el-button type="primary" @click="confirmSelection" :disabled="saveLoading">确 定</el-button>
                    <el-button @click="centerDialogVisible = false">取 消</el-button>
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
            emptytext: "{yun:}t key='wap_js_00113'{/yun}",
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
            that.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
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
                    that.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
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
                message.warning("{yun:}t key='admin_system_00369'{/yun}");
                return false;
            }
            if (call == 'module' && !ruleForm.seoid) {
                message.warning("{yun:}t key='admin_system_00361'{/yun}");
                return false;
            }
            if (!ruleForm.seoname || ruleForm.seoname == "") {
                message.warning("{yun:}t key='admin_system_00365'{/yun}");
                return false;
            }
            if (!ruleForm.ident || ruleForm.ident == "") {
                message.warning("{yun:}t key='admin_system_00367'{/yun}");
                return false;
            }
            if (!ruleForm.title || ruleForm.title == "") {
                message.warning("{yun:}t key='admin_system_00368'{/yun}");
                return false;
            }
            if (!ruleForm.keywords || ruleForm.keywords == "") {
                message.warning("{yun:}t key='admin_system_00363'{/yun}");
                return false;
            }
            if (!ruleForm.description || ruleForm.description == "") {
                message.warning("{yun:}t key='admin_system_00366'{/yun}");
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
                                custoapp.seotabRefresh(); // {yun:}t key='wap_user_00334'{/yun}TAB{yun:}t key='wap_00316'{/yun}
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