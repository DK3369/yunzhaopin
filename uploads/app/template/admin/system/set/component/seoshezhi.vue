<template>
    <div class="drawerModlue">
        <!-- <div class="tableDome_tip">
            <el-alert :title="headTip" type="warning">
            </el-alert>
        </div> -->
        <div class="tableDome_tip tableDoAlert">
            <span>提示：如果您不是程序员或不清楚用法，请不要随意修改SEO标识符！网站的SEO部分是网站重要部分，请不要经常修改！<br />根据百度新算法，网站标题请勿堆积关键字，具体请参考</span>
            <a href="https://ziyuan.baidu.com/college/documentinfo?id=1576&amp;qq-pf-to=pcqq.c2c" target="_blank" style="color:#00F">教程</a>
        </div>
        <div class="drawerModInfo drawerModInfoOne drawerModInguding">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>SEO模块</span>
                </div>
                <div v-if="call == 'seo'" class="drawerModInpt">
                    <el-select v-model="ruleForm.seomodel" placeholder="请选择">
                        <el-option v-for="(smitem, smindex) in seomodel" :key="smindex" :label="smitem" :value="smindex">
                        </el-option>
                    </el-select>
                </div>
                <div v-if="call == 'module'" class="drawerModInpt">
                    <el-select v-model="ruleForm.seoid" @change="changeSeoid" placeholder="请选择">
                        <el-option v-for="seoItem in seo" :key="seoItem.id" :label="seoItem.seoname" :value="seoItem.id">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>页面名称</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.seoname" placeholder="请输入内容"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>SEO标识符</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.ident" placeholder="请输入内容"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>使用范围</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="ruleForm.did" placeholder="请选择">
                        <el-option v-for="(ditem, dindex) in Dname" :key="dindex" :label="ditem" :value="dindex">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>伪静态规则</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.rewrite_url" placeholder="请输入内容"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="如：/job/{id}.html 多数用于详情页" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>原始路径</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.php_url" placeholder="请输入内容"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="如：/job/index.php?c=comapply （只需模块链接 无需参数 与上对应）" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>WAP伪静态规则</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.rewrite_wap_url" placeholder="请输入内容"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="如：/job/{id}.html 多数用于详情页" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>WAP原始路径</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.php_wap_url" placeholder="请输入内容"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="如：?c=job&a=comapply （只需模块链接 无需具体参数 与上对应）" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>网站标题（title）</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="ruleForm.title" @blur="textareaBlur($event, 'title')">
                    </el-input>
                    <el-button type="info" @click="openCenterDialog('title')">选择参数</el-button>
                </div>
                <!-- <div class="drawerModTips drawerMoAlert">
                    <el-alert :title="titleTip" type="info" show-icon :closable="false">
                    </el-alert>
                </div> -->
                <div class="drawerModTips drawerMoAlert">
                    <i class="el-icon-info"></i>
                    <span>一般不超过80个字符，根据百度新算法，请勿堆积关键字，具体请参考</span>
                    <a href="https://ziyuan.baidu.com/college/documentinfo?id=1576&amp;qq-pf-to=pcqq.c2c" target="_blank" style="color:#00F">教程</a>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>网站关键字（keywords）</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="ruleForm.keywords" @blur="textareaBlur($event, 'keywords')">
                    </el-input>
                    <el-button type="info" @click="openCenterDialog('keywords')">选择参数</el-button>
                </div>
                <div class="drawerModTips">
                    <el-alert title="一般不超过100个字符" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>网站描述（description）</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="ruleForm.description" @blur="textareaBlur($event, 'description')">
                    </el-input>
                    <el-button type="info" @click="openCenterDialog('description')">选择参数</el-button>
                </div>
                <div class="drawerModTips">
                    <el-alert title="一般不超过200个字符" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">提交</el-button>
        </div>
        <div class="modluDialog">
            <el-drawer title="可选择参数" :visible.sync="centerDialogVisible" :append-to-body="true" :show-close="true" :with-header="true" size="40%">
                <div style="overflow-y: auto; position: relative; width: 100%; height: calc(100% - 70px); padding: 0 20px;">
                    <div class="tableDome_tip">
                        <el-alert title="提示：下面的标识符请根据页面对应添加，否则无法正常显示！" type="warning">
                        </el-alert>
                    </div>
                    <div v-for="(scitem, scindex) in seoconfigList" :key="scindex" v-if="checkSeoconfig(scitem.seomodel)">
                        <el-table ref="multipleTable" :data="scitem.tableData" tooltip-effect="dark" style="width: 100%" @selection-change="handleSelectionChange" v-loading="loading" :empty-text="emptytext">
                            <el-table-column type="selection" width="55">
                            </el-table-column>
                            <el-table-column label="说明" prop="title" width="150">
                            </el-table-column>
                            <el-table-column label="代码">
                                <template slot-scope="scope">{{ '{' + scope.row.code + '}' }}</template>
                            </el-table-column>
                        </el-table>
                    </div>
                </div>
                <div class="dialofhooter">
                    <el-button type="primary" @click="confirmSelection">确 定</el-button>
                    <el-button @click="centerDialogVisible = false">取 消</el-button>
                </div>
            </el-drawer>
        </div>
        <!-- <div class="modluDialog">
            <el-dialog title="可选择参数" :visible.sync="centerDialogVisible" :append-to-body="true" :modal="false" width="30%" center>
                <div>
                    <div class="tableDome_tip">
                        <el-alert title="提示：下面的标识符请根据页面对应添加，否则无法正常显示！" type="warning">
                        </el-alert>
                    </div>
                    <div v-for="(scitem, scindex) in seoconfigList" :key="scindex" v-if="checkSeoconfig(scitem.seomodel)">
                        <el-table ref="multipleTable" :data="scitem.tableData" tooltip-effect="dark" style="width: 100%" @selection-change="handleSelectionChange" v-loading="loading">
                            <el-table-column type="selection" width="55">
                            </el-table-column>
                            <el-table-column label="说明" prop="title" width="150">
                            </el-table-column>
                            <el-table-column label="代码">
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
            emptytext: '暂无数据',
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
            headTip: '提示：如果您不是程序员或不清楚用法，请不要随意修改SEO标识符！网站的SEO部分是网站重要部分，请不要经常修改！<br/>根据百度新算法，网站标题请勿堆积关键字，具体请参考<a href="https://ziyuan.baidu.com/college/documentinfo?id=1576&amp;qq-pf-to=pcqq.c2c" target="_blank" style="color:#00F">教程</a>。',
            titleTip: '一般不超过80个字符，根据百度新算法，请勿堆积关键字，具体请参考<a href="https://ziyuan.baidu.com/college/documentinfo?id=1576&amp;qq-pf-to=pcqq.c2c" target="_blank" style="color:#00F">教程</a>。',
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
            if (call == 'seo') { // SEO设置
                url = 'm=system&c=set_seo&a=seoadd';
                params = { id: that.seoid };
            } else if (call == 'module') { // 模块设置
                url = 'm=system&c=set_module&a=seoshezhi';
                params = { config: that.config };
            }
            that.loading = true;
            that.emptytext = "数据加载中";
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
                    that.emptytext = "暂无数据";
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
                message.warning('请选择SEO模块');
                return false;
            }
            if (call == 'module' && !ruleForm.seoid) {
                message.warning('请选择需要修改的SEO模块');
                return false;
            }
            if (!ruleForm.seoname || ruleForm.seoname == "") {
                message.warning('请填写SEO名称');
                return false;
            }
            if (!ruleForm.ident || ruleForm.ident == "") {
                message.warning('请填写SEO标识');
                return false;
            }
            if (!ruleForm.title || ruleForm.title == "") {
                message.warning('请填写SEO标题');
                return false;
            }
            if (!ruleForm.keywords || ruleForm.keywords == "") {
                message.warning('请填写SEO关键词');
                return false;
            }
            if (!ruleForm.description || ruleForm.description == "") {
                message.warning('请填写SEO描述');
                return false;
            }

            if (call == 'seo') { // SEO设置
                url = 'm=system&c=set_seo&a=save';
            } else if (call == 'module') { // 模块设置
                ruleForm.id = ruleForm.seoid;
                url = 'm=system&c=set_module&a=seoshezhi';
            }
            that.saveLoading = true;
            httpPost(url, ruleForm).then(function(response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    message.success(res.msg, function() {
                        that.$emit("child-event");
                        if (call == 'seo') { // SEO设置
                            if (custoapp.curTab == ruleForm.seomodel) { // tab标签相等
                                custoapp.seotabRefresh(); // 刷新TAB列表
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
                did: '0'
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