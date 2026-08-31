<template>
<div id="schollfl" class="moduleElenAl">
        <div class="moduleSeachs">
            <div class="">{{ lc('admin_00259') }}</div>
            <div class="nrtopbtn">
                <el-button size="small" type="primary" icon="el-icon-document-add" @click="openAdd('')">{{ lc('admin_00222') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table :data="list" stripe border style="width: 100%;" ref="multipleTable"
                @selection-change="handleSelectionChange"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"> </el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="180">
                </el-table-column>
                <el-table-column :label="lc('member_com_00021')" prop="name">
                </el-table-column>
                <el-table-column :label="lc('admin_00150')" prop="img">
                    <template #default="scope">
                        <img :src="scope.row.pic" width="40" height="40" />
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="140" fixed="right">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" plain @click="openAdd(scope.row)">{{ lc('wap_js_00073') }}</el-button>
                            <el-button type="danger" size="small"  @click="del(scope.$index)">{{ lc('wap_js_00077') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="">
                <div class="">
                    <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
                        @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>
                    <el-button @click="batch('del')" size="small">{{ lc('member_com_00055') }}</el-button>
                </div>
            </div>
        </div>

        <div class="modluDrawer">
            <el-dialog :title="ruleForm.id ? lc('admin_00255') : lc('admin_00256')" width="500px" v-model="dialogAdd"
                :modal-append-to-body="false">
                <div class="toolClasDia">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00260') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input v-model="ruleForm.name" :placeholder="lc('admin_00217')"></el-input>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00261') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input type="textarea" :rows="2" :placeholder="lc('admin_00257')" v-model="ruleForm.content">
                            </el-input>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('default_00254') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-upload class="avatar-uploader" list-type="picture" action="" :auto-upload="false"
                                :on-change="handleChangePic" :show-file-list="false" :accept="pic_accept">
                                <img v-if="ruleForm.pic_n" :src="ruleForm.pic_n" class="avatar">
                                <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                            </el-upload>
                        </div>
                    </div>
                </div>
                <template #footer><div class="dialog-footer">
                    <el-button type="primary" @click="save" :disabled="saveLoading">{{ruleForm.id ? lc('wap_js_00073') : lc('wap_js_00091')}}</el-button>
                </div></template>
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
        data: function () {
            return {
                pic_accept: localStorage.getItem("pic_accept"),
                emptytext: lc('wap_js_00113'),
                loading: false,
                // 列表
                list: [],

                checkedAll: false, // 全选
                checkedAllIndeterminate: false,
                multipleSelection: [], // 多选值存储
                idArr: [],

                detail: {},

                saveLoading: false,

                // 添加
                dialogAdd: false,
                ruleForm: {},
            }
        },
        created() {
            this.getList();
        },
        methods: {
            getList() {
                let that = this;
                that.loading = true;
                that.emptytext = lc('admin_user_weipin_00026');
                httpPost('m=neirong&c=toolbox_class',{}, {hideloading: true}).then(function (response) {
                    let res = response.data || {};
                    let data = res.data || {};
                    that.list = Array.isArray(data.list) ? data.list : [];
                    that.loading = false;
                    if (that.list.length === 0){
                        that.emptytext = lc('wap_js_00113');
                    }
                    if (res.error > 0 && res.msg) {
                        message.error(res.msg);
                    }
                }).catch(function () {
                    that.list = [];
                    that.loading = false;
                    that.emptytext = lc('wap_js_00113');
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
                if (this.multipleSelection.length == 0) {
                    message.error(lc('admin_00136'));
                    return false;
                }

                let idArr = [];
                this.multipleSelection.forEach(function (item) {
                    idArr.push(item.id);
                })
                this.idArr = idArr;

                if (type == 'del') {
                    this.del();
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

                if (typeof idx == 'undefined') { // 批量删除
                    params.del = this.idArr;
                    msg = lc('common_00853');
                } else {// 单个删除
                    params.del = that.list[idx].id;
                    msg = lc('admin_00333');
                }

                delConfirm(this, params, function (params) {
                    httpPost('m=neirong&c=toolbox_class&a=del', params).then(function (res) {
                        if (res.data.error > 0) {
                            message.error(res.data.msg);
                        } else {
                            that.getList();
                            message.success(res.data.msg, function () {
                                that.$refs.multipleTable.clearSelection();
                            });
                        }
                    })
                }, msg)
            },

            openAdd(row) {
                let that = this;

                if (row != '') {
                    that.detail = row;
                    that.ruleForm = {
                        id: row.id,
                        name: row.name,
                        content: row.content,
                        pic_n: row.pic
                    };
                } else {
                    that.detail = {};
                    that.ruleForm = {};
                }

                that.dialogAdd = true;
            },

            // 上传时触发
            handleChangePic(file, fileList) {
                this.$set(this.ruleForm, 'pic', file.raw);
                this.$set(this.ruleForm, 'pic_n', file.url);
            },

            save() {
                let that = this,
                    ruleForm = that.ruleForm,
                    formData = new FormData();

                if (!ruleForm.name) {
                    message.warning(lc('admin_00217'));
                    return false;
                }

                if (!ruleForm.content) {
                    message.warning(lc('admin_00257'));
                    return false;
                }

                if (!ruleForm.pic_n) {
                    message.warning(lc('admin_00258'));
                    return false;
                }

                if (that.saveLoading) {
                    return false;
                }

                that.saveLoading = true;

                $.each(ruleForm, function (key, value) {
                    if (key != 'pic_n') {
                        formData.append(key, value);
                    }
                });

                httpPost('m=neirong&c=toolbox_class&a=save', formData).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        message.error(res.msg, function () {
                            that.saveLoading = false;
                        });
                    } else {
                        that.dialogAdd = false;
                        that.getList();
                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },
        }
    }
</script>
