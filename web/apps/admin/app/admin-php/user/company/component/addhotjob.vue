<template>
    <div class="minqizhapin">
        <div class="minqizhaFromd minqizhaXizeng">
            <div class="xinzenLite">
                <div class="wxsettip_small ">{{ lc('wap_com_00157') }}</div>
                <div class="wxsetokdsu">
                    <el-input v-if="isedit" v-model="info.username" disabled :placeholder="lc('wap_com_00157')"></el-input>
                    <el-select v-else v-model="info.uid" filterable remote :placeholder="lc('admin_user_company_00019')" :remote-method="getComArr"
                        @change="comChange">
                        <el-option v-for="item in com_arr" :key="item.value" :label="item.label" :value="item.value">
                        </el-option>
                    </el-select>
                </div>

            </div>
            <div class="xinzenLite">
                <div class="wxsettip_small ">{{ lc('admin_user_company_00018') }}</div>
                <div class="wxsetokdsu">
                    <el-input v-model="info.rating" disabled :placeholder="lc('admin_user_company_00018')"></el-input>
                </div>

            </div>
            <div class="xinzenLite">
                <div class="wxsettip_small ">{{ lc('admin_user_company_00022') }}</div>
                <div class="wxsetokdsu">
                    <el-date-picker v-model="start" value-format="YYYY-MM-dd" :placeholder="lc('admin_00343')">
                    </el-date-picker>
                    <el-date-picker style="margin-left: 10px;" v-model="end" value-format="YYYY-MM-dd" :placeholder="lc('admin_00344')">
                    </el-date-picker>
                </div>
            </div>
            <div class="xinzenLite">
                <div class="wxsettip_small ">{{ lc('admin_user_company_00021') }}</div>
                <div class="wxsetokdsu">
                    <el-input v-model="info.service_price" :placeholder="lc('admin_user_company_00021')"
                        @input="inputIntNumber($event, 'info', 'service_price')"><template #append><template
                           >{{ lc('common_02056') }}</template></template></el-input>
                </div>

            </div>
            <div class="xinzenLite">
                <div class="wxsettip_small ">{{ lc('admin_user_company_00020') }}</div>
                <div class="wxsetokdsu">
                    <el-input v-model="info.sort" :placeholder="lc('admin_user_company_00020')"
                        @input="inputIntNumber($event, 'info', 'sort')"></el-input>
                </div>

            </div>
            <div class="xinzenLite">
                <div class="wxsettip_small ">{{ lc('admin_user_company_00016') }}</div>
                <div class="wxsetokdsu">
                    <el-upload class="avatar-uploader" :action="''" :show-file-list="false" :on-change="mqlogoChange"
                        :accept="pic_accept">
                        <img v-if="info.hot_pic_n" :src="info.hot_pic_n" class="avatar">
                        <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                    </el-upload>
                </div>

            </div>
            <div class="xinzenLite">
                <div class="wxsettip_small ">{{ lc('admin_00573') }}</div>
                <div class="wxsetokdsu">
                    <el-input type="textarea" :rows="2" :placeholder="lc('admin_00574')" v-model="info.beizhu"></el-input>
                </div>

            </div>

        </div>
        <div class="xinzdialoGooter">
            <div>
                <el-button @click="closedrawermq" size="small">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="mqsave" :loading="saveLoading" size="small">{{ lc('wap_com_00019') }}</el-button>
            </div>
            
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
            comindex: -1,// Company index when setting featured company from company details
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
        // Search company when adding featured company
        getComArr(query) {
            var that = this
            if (query !== '') {
                setTimeout(() => {
                    httpPost('m=user&c=hotjob&a=getComList', { name: query }).then(function (response) {
                        if (response.data.error == 0) {
                            that.com_arr = response.data.data
                        } else {
                            message.error(lc('admin_user_company_00017'));
                        }
                    }).catch(function (error) {
                        console.log(error);
                    })
                }, 200);
            } else {
                this.com_arr = [];
            }
        },
        // Select company when adding participant company
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
                message.error(lc('admin_company_00037'))
                return false
            } else {
                that.info.time_start_n = that.start
            }
            if (that.end == "") {
                message.error(lc('admin_company_00038'))
                return false
            } else {
                that.info.time_end_n = that.end
            }
            if (that.mqlogolist.length == 0 && that.info.hot_pic_n == '') {
                message.error(lc('admin_company_00046'))
                return false
            }
            if (that.info.beizhu && that.info.beizhu.length > 200) {
                message.error(lc('admin_company_00039'))
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
            // Preview file handling
            tmp.hot_pic_n = URL.createObjectURL(file.raw);
            // Clone file metadata
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