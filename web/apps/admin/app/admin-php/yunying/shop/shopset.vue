<template>
<div id="daohaapp" class="moduleDome">
        <div class="moduleHeadr">
            <h3>{{ lc('admin_01204') }}</h3>
            <span>{{ lc('admin_yunying_00119') }}</span>
        </div>
        <div class="scImgbox">
            <el-upload
              :accept="pic_accept"
              class="avatar-uploader"
              :auto-upload="false"
              action=""
              :show-file-list="false"
              :on-change="uploadChange"
            >
            <div class="scImgbimg">
                <el-image :src="sy_imgsc_mr" ></el-image>
            </div>
            <div class="scImgboxInfo">
                <div class="scImgboxTxt">
                    <span>{{ lc('admin_yunying_00120') }}</span>
                    <span>{{ lc('admin_yunying_00121') }}</span>
                </div>
                <div class="scImgboxburn">
                    <el-button type="primary" plain>{{ lc('member_com_00059') }}</el-button>
                </div>
            </div>
            </el-upload>
        </div>
        <div class="scImgboxAnNiu">
            <el-button type="primary" :loading="save_load" @click="save">{{ lc('wap_user_00176') }}</el-button>
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
                    sy_imgsc_mr: '',
                    files: [],
					save_load:false
                }
            },
            created: function () {
                this.getInfo();
            },
            methods: {
                getInfo() {
                    let that = this;

                    httpPost('m=yunying&c=shop_set').then(function (response) {
                        let res = response.data,
                            data = res.data;

                        that.sy_imgsc_mr = data.sy_imgsc_mr;
                    })
                },
                save(){
                    let that = this;
                    let formData = new FormData();
                    formData.append('name', 'sy_imgsc_mr');
                    formData.append('path', 'logo');
                    if (that.files.length !== 0) {
                        formData.append('file', that.files);
                    }
					that.save_load = true;
                    httpPost('m=yunying&c=shop_set&a=saveset', formData).then(function (res) {
						that.save_load = false;
                        if (res.data.error == 0) {
                            message.success(res.data.msg, function () {
                                that.getInfo();
                            });
                        } else {
                            message.error(res.data.msg);
                        }
                    });
                },
                uploadChange(file) {
                    this.sy_imgsc_mr = URL.createObjectURL(file.raw);
                    // 复刻文件信息
                    this.files = file.raw;
                },
            }
        }
</script>
