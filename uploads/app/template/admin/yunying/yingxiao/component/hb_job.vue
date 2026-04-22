<template>
    <div class="setUpload">
        <div class="uploadTable">
            <div class="playbillHeader">
                <div class="playbillHeaText">
                    <span class="span1">温馨提示：</span>
                    <span>模板勾选后前台才能展示相应的海报，您也可以自定义添加海报。职位海报默认展示当前职位信息和企业信息。</span>
                </div>
                <div class="playbillHeabutn">
                    <el-button type="primary" icon="el-icon-document-add" size="mini"
                        @click="openAdd('')">添加海报</el-button>
                </div>
            </div>
            <div class="playbillHeader">
                <div class="playbillHeaText">
                    <span class="span1">海报参数：</span>
                    <span>海报大小（1080 * 1920）；白色文案区域：高 428 ，距离左右 53，距离底部 86</span>
                </div>
            </div>
            <div class="playbilCont">
                <div class="playbilName">
                    <span>职位海报：</span>
                </div>
                <div class="playbilFlex">
                    <ul>
                        <li v-for="(item, index) in list" :key="index">
                            <div class="playbilXuanz">
                                <el-checkbox v-model="item.checked" :checked="item.isopen == 1">{{item.name}}</el-checkbox>
                            </div>
                            <div class="playbilImge">
                                <img :src="item.pic_n" alt="">
                            </div>
                            <div class="playbilAnNiu">
                                <el-button size="mini" @click="openAdd(item)">管理</el-button>
                                <el-button size="mini" @click="del(index)">删除</el-button>
                            </div>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
        <div v-if="list.length > 0" class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="save">提交</el-button>
        </div>

        <div class="modluDrawer">
            <el-dialog title="职位海报管理" width="500px" :visible.sync="dialogAdd" :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>海报名称：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input v-model="ruleForm.name" placeholder="请输入海报名称"></el-input>
                        </div>
                    </div>
                    <div class="toolClasList" style="align-items: initial;">
                        <div class="toolClasTite" style="margin-top: 15px;">
                            <span>上传图片：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-upload :accept="pic_accept"
                                    class="avatar-uploader"
                                    list-type="picture"
                                    action=""
                                    :auto-upload="false"
                                    :on-change="handleChangePic"
                                    :show-file-list="false">
                                <img v-if="ruleForm.pic_n" :src="ruleForm.pic_n" class="avatar">
                                <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                            </el-upload>
                            <div class="alertTips">
                                <el-alert title="说明：海报的尺寸为1080*1920" type="info" :closable="false" show-icon>
                                </el-alert>
                            </div>
                        </div>
                    </div>
                    <div class="toolClasList" style="align-items: initial;">
                        <div class="toolClasTite" style="margin-top: 6px;">
                            <span>排序：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input v-model="ruleForm.sort" placeholder="" @input="inputIntNumber($event, 'ruleForm', 'sort')"></el-input>
                            <div class="alertTips">
                                <el-alert title="数字越大排序越靠前" type="info" :closable="false" show-icon>
                                </el-alert>
                            </div>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>是否开启：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-switch v-model="ruleForm.isopen" active-value="1" inactive-value="0">
                            </el-switch>
                        </div>
                    </div>
                </div>
                <div slot="footer" class="dialog-footer">
                    <el-button type="primary" @click="saveWhb">提交</el-button>
                </div>
            </el-dialog>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props: {},
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            list: [],
            detail: {},

            // 添加弹窗
            dialogAdd: false,
            ruleForm: {},

            saveLoading: false,
        }
    },

    mounted() {

    },
    created() {
        this.init();
    },
    methods: {
        init() {
            this.getList();
            this.resetData();
        },

        resetData() {
            this.ruleForm = {
                type: 1,
                isopen: 0
            };
        },

        getList() {
            let that = this;
            httpPost('m=yunying&c=yingxiao_hbconfig&a=job').then(function (response) {
                let res = response.data,
                    data = res.data;

                data.list.map(item => item.checked = item.isopen == 1);
                that.list = data.list;
            })
        },

        save() {
            let that = this,
                list = that.list,
                params = {
                    sy_job_hb: ''
                };

            if (that.saveLoading) {
                return false;
            }

            that.saveLoading = true;

            list.forEach(item => {
                if (item.checked) {
                    params.sy_job_hb += params.sy_job_hb ? ',' + item.id : item.id;
                }
            })

            httpPost('m=yunying&c=yingxiao_hbconfig&a=saveWhbConfig', params).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg, function() {
                        that.saveLoading = false;
                    });
                } else {
                    message.success(res.msg, function() {
                        that.saveLoading = false;
                    });
                }
            })
        },

        openAdd(data) {
            this.dialogAdd = true;
            if (data !== '') {
                this.detail = data;

                this.$set(this.ruleForm, 'name', data.name);
                this.$set(this.ruleForm, 'num', data.num > 0 ? data.num : '');
                this.$set(this.ruleForm, 'style', data.style > 0 ? data.style : '');
                this.$set(this.ruleForm, 'pic_n', data.pic_n);
                this.$set(this.ruleForm, 'sort', data.sort > 0 ? data.sort : '');
                this.$set(this.ruleForm, 'isopen', data.isopen);
            } else {
                this.detail = {};
                this.resetData();
            }
        },

        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g,'');
        },

        // 上传时触发
        handleChangePic(file, fileList) {
            this.$set(this.ruleForm, 'pic', file.raw);
            this.$set(this.ruleForm, 'pic_n', file.url);
        },

        saveWhb() {
            let that = this,
                ruleForm = that.ruleForm,
                formData = new FormData();

            if (!ruleForm.name) {
                message.error('请输入海报名称');
                return false;
            }

            if (!ruleForm.pic_n) {
                message.error('请上传图片');
                return false;
            }

            if (that.saveLoading) {
                return false;
            }

            that.saveLoading = true;

            $.each(ruleForm, function(key, value){
                if (key != 'pic_n') {
                    formData.append(key, value);
                }
            });

            if (typeof (that.detail.id) != 'undefined') {
                formData.append('id', that.detail.id);
            }

            httpPost('m=yunying&c=yingxiao_hbconfig&a=saveWhb', formData).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg, function() {
                        that.saveLoading = false;
                    });
                } else {
                    message.success(res.msg, function() {
                        that.dialogAdd = false;
                        that.resetData();
                        that.saveLoading = false;
                        that.getList();
                    });
                }
            })
        },

        del(idx) {
            let that = this,
                params = {};

            params.id = that.list[idx].id;

            delConfirm(this, params, function (params) {
                httpPost('m=yunying&c=yingxiao_hbconfig&a=delWhb', params).then(function(res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.success(res.data.msg, function () {
                            that.list.splice(idx, 1);
                            // that.getList();
                        });
                    }
                })
            }, '确定删除该海报？')
        },
    },
};
</script>
<style scoped>
.fenpeizhand .toolClasCont {
    flex-wrap: wrap;
}
.alertTips{
    overflow: hidden;
    position: relative;
    width: 100%;
}
.alertTips .el-alert{
    padding: 8px 0 0 0;
    background: none;
}

/* 上传样式开始 */
.avatar-uploader .el-upload {
    border: 1px dashed #d9d9d9;
    border-radius: 6px;
    cursor: pointer;
    position: relative;
    overflow: hidden;
}
.avatar-uploader .el-upload:hover {
    border-color: #409EFF;
}
.avatar-uploader-icon {
    font-size: 28px;
    color: #8c939d;
    width: 148px;
    height: 263px;
    line-height: 263px;
    text-align: center;
}
.avatar {
    width: 148px;
    height: 263px;
    display: block;
}
/* 上传样式结束 */
</style>