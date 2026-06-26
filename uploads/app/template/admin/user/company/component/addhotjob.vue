<template>
    <div class="minqizhapin">
        <div class="minqizhaFromd minqizhaXizeng">
            <div class="xinzenLite">
                <div class="wxsettip_small ">{yun:}t key='wap_com_00157'{/yun}</div>
                <div class="wxsetokdsu">
                    <el-input v-if="isedit" v-model="info.username" disabled placeholder="{yun:}t key='wap_com_00157'{/yun}"></el-input>
                    <el-select v-else v-model="info.uid" filterable remote placeholder="{yun:}t key='admin_user_company_00019'{/yun}" :remote-method="getComArr"
                        @change="comChange">
                        <el-option v-for="item in com_arr" :key="item.value" :label="item.label" :value="item.value">
                        </el-option>
                    </el-select>
                </div>

            </div>
            <div class="xinzenLite">
                <div class="wxsettip_small ">{yun:}t key='admin_user_company_00018'{/yun}</div>
                <div class="wxsetokdsu">
                    <el-input v-model="info.rating" disabled placeholder="{yun:}t key='admin_user_company_00018'{/yun}"></el-input>
                </div>

            </div>
            <div class="xinzenLite">
                <div class="wxsettip_small ">{yun:}t key='admin_user_company_00022'{/yun}</div>
                <div class="wxsetokdsu">
                    <el-date-picker v-model="start" value-format="yyyy-MM-dd" placeholder="{yun:}t key='admin_00343'{/yun}">
                    </el-date-picker>
                    <el-date-picker style="margin-left: 10px;" v-model="end" value-format="yyyy-MM-dd" placeholder="{yun:}t key='admin_00344'{/yun}">
                    </el-date-picker>
                </div>
            </div>
            <div class="xinzenLite">
                <div class="wxsettip_small ">{yun:}t key='admin_user_company_00021'{/yun}</div>
                <div class="wxsetokdsu">
                    <el-input v-model="info.service_price" placeholder="{yun:}t key='admin_user_company_00021'{/yun}"
                        @input="inputIntNumber($event, 'info', 'service_price')"><template
                            slot="append">{yun:}t key='common_02056'{/yun}</template></el-input>
                </div>

            </div>
            <div class="xinzenLite">
                <div class="wxsettip_small ">{yun:}t key='admin_user_company_00020'{/yun}</div>
                <div class="wxsetokdsu">
                    <el-input v-model="info.sort" placeholder="{yun:}t key='admin_user_company_00020'{/yun}"
                        @input="inputIntNumber($event, 'info', 'sort')"></el-input>
                </div>

            </div>
            <div class="xinzenLite">
                <div class="wxsettip_small ">{yun:}t key='admin_user_company_00016'{/yun}</div>
                <div class="wxsetokdsu">
                    <el-upload class="avatar-uploader" :action="''" :show-file-list="false" :on-change="mqlogoChange"
                        :accept="pic_accept">
                        <img v-if="info.hot_pic_n" :src="info.hot_pic_n" class="avatar">
                        <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                    </el-upload>
                </div>

            </div>
            <div class="xinzenLite">
                <div class="wxsettip_small ">{yun:}t key='admin_00573'{/yun}</div>
                <div class="wxsetokdsu">
                    <el-input type="textarea" :rows="2" placeholder="{yun:}t key='admin_00574'{/yun}" v-model="info.beizhu"></el-input>
                </div>

            </div>

        </div>
        <div slot="footer" class="xinzdialoGooter">
            <div>
                <el-button @click="closedrawermq" size="small">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                <el-button type="primary" @click="mqsave" :loading="saveLoading" size="small">{yun:}t key='wap_com_00019'{/yun}</el-button>
            </div>
            
        </div>
    </div>
</template>
<script>
module.exports = {
    props: {
        hotinfo: {
            type: Object,
            default: function () {
                return {}
            }
        },
        hascom: {
            type: Boolean,
            default: function () {
                return false
            }
        },
        cindex: {
            type: Number,
            default: function () {
                return -1
            }
        }
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            info: {
                uid: '',
                username: '',
                service_price: '',
                rating: '',
                rating_id: '',
                sort: '',
                beizhu: '',
                hot_pic_n: '',
                time_start_n: '',
                time_end_n: ''
            },
            start: '',
            end: '',
            isedit: false,
            com_arr: [],
            mqlogolist: [],
            comindex: -1,// 企业详情点击设为名企企业index
            saveLoading: false
        }
    },
    mounted() {

    },
    watch: {
        hotinfo: {
            handler(val) {
                this.info = val;
                if (val.time_start_n) {
                    this.start = val.time_start_n
                }
                if (val.time_end_n) {
                    this.end = val.time_end_n
                }
                this.mqlogolist = []
                this.com_arr = []
            },
            immediate: true,
            deep: true,
        },
        hascom: {
            handler(val) {
                this.isedit = val;
            },
            immediate: true,
            deep: true,
        },
        cindex: {
            handler(val) {
                this.comindex = val;
            },
            immediate: true,
            deep: true,
        },
    },
    methods: {
        closedrawermq() {
            this.$parent.$parent.drawermq = false
        },
        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g, '');
        },
        // 添加名企业搜索企业
        getComArr(query) {
            var that = this
            if (query !== '') {
                setTimeout(() => {
                    httpPost('m=user&c=hotjob&a=getComList', { name: query }).then(function (response) {
                        if (response.data.error == 0) {
                            that.com_arr = response.data.data
                        } else {
                            message.error("{yun:}t key='admin_user_company_00017'{/yun}");
                        }
                    }).catch(function (error) {
                        console.log(error);
                    })
                }, 200);
            } else {
                this.com_arr = [];
            }
        },
        // 添加参会企业选择企业
        comChange(data) {
            var that = this
            var selOption = this.com_arr.find((item) => item.value === data)
            httpPost('m=user&c=hotjob&a=gethotjob', { uid: selOption.value }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.info = res.data
                    if (!that.info.time_start_n) {
                        that.start = ''
                    } else {
                        that.start = that.info.time_start_n
                    }
                    if (that.info.time_end_n == undefined || !that.info.time_end_n) {
                        that.end = ''
                    } else {
                        that.end = that.info.time_end_n
                    }
                } else {
                    message.error(res.msg)
                }
            }).catch(function (error) {
                console.log(error);
            })
        },
        mqsave: function () {
            var that = this
            if (that.start == "") {
                message.error("{yun:}t key='admin_company_00037'{/yun}")
                return false
            } else {
                that.info.time_start_n = that.start
            }
            if (that.end == "") {
                message.error("{yun:}t key='admin_company_00038'{/yun}")
                return false
            } else {
                that.info.time_end_n = that.end
            }
            if (that.mqlogolist.length == 0 && that.info.hot_pic_n == '') {
                message.error("{yun:}t key='admin_company_00046'{/yun}")
                return false
            }
            if (that.info.beizhu && that.info.beizhu.length > 200) {
                message.error("{yun:}t key='admin_company_00039'{/yun}")
                return false
            }
            var params = new FormData();
            for(let i in that.info){
                params.append(i,that.info[i])
            }
            if (that.mqlogolist.length) {
                params.append('mqlogo[]', that.mqlogolist[0])
            }
            httpPost('m=user&c=hotjob&a=save', params).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.$parent.$parent.drawermq = false
                        that.$parent.$parent.getList();
                        if (that.comindex >= 0) {
                            that.$parent.$parent.cominfo(that.comindex, that.info.uid)
                        }
                    })
                } else {
                    message.error(res.msg);
                }
            }).catch(function (e) {
                console.log(e)
            }).finally(function () {
                setTimeout(function () {
                    that.saveLoading = false;
                }, 2000);
            });
        },
        mqlogoChange(file) {
            var tmp = deepClone(this.info)
            // 预览文件处理
            tmp.hot_pic_n = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.mqlogolist[0] = file.raw;
            this.info = tmp
        },
    },
};
</script>
<style>
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
    font-size: 14px;
    color: #8c939d;
    width: 48px;
    height: 48px;
    line-height: 48px;
    text-align: center;
}

.avatar {
    width: 48px;
    height: 48px;
    display: block;
}

.minqizhapin {
    overflow: hidden;
    position: relative;
    padding: 0;
    height: 100%;
}

.minqizhaFromd {
    overflow-y: auto;
    position: relative;
    width: 100%;
    height: calc(100% - 80px);
}
</style>