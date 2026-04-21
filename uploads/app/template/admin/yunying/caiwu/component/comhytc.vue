<template>
    <div class="moduleElHight">

        <div style="overflow: hidden; position: relative; height: 100%; height: 100%;">
            <div style="overflow-y: auto; position: relative; height: auto; max-height: calc(100% - 80px);">
                <table class="tableVue">
                    <thead>
                        <tr align="left">
                            <th width="200">名称</th>
                            <th width="440">状态</th>
                            <th>说明</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">开通用户名</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-autocomplete v-model="save.username" :fetch-suggestions="remoteMethod"
                                        placeholder="请输入内容" :trigger-on-focus="false" @select="usernameChange"
                                        size="small"></el-autocomplete>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>单次只能为一个会员开通套餐</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">开通企业名称</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-autocomplete v-model="save.comname" :fetch-suggestions="remoteMethodCom"
                                        placeholder="请输入内容" :trigger-on-focus="false" @select="comnameChange"
                                        size="small"></el-autocomplete>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>开通企业名称</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">是否累加</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="save.leijia" @change="leijiaFun">
                                        <el-radio label="1">是 </el-radio>
                                        <el-radio label="2">否</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>是否累加</span>
                                </div>
                            </td>
                        </tr>
                        <tr v-if="leijiaStatus">
                            <td>
                                <div class="TableTite">原本套餐：</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <span style="margin-right: 10px;">套餐名称：<span>{{ package_data.rating_name
                                    }}</span></span>
                                    <span style="margin-right: 10px;">截止日期：
                                        <span v-if="expireTimeStatus" style="color: red">{{ package_data.time_ymd }}</span>
                                        <span v-else>{{ package_data.time_ymd }}</span>
                                    </span>
                                </div>
                            </td>
                            <td>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">开通等级</div>
                            </td>
                            <td>
                                <div class="TableSelect" style="display: flex;align-items: center;">
                                    <el-select v-model="save.ratingid" placeholder="请选择" @change="ratingFun">
                                        <el-option v-for="item in ratinglist" :key="item.id" :label="item.name"
                                            :value="item.id">
                                        </el-option>
                                    </el-select>

                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">套餐时间</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="save.taocanshijian" placeholder=" " disabled>
                                        <span slot="suffix" class="slotspan">天</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">价格</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="save.vipprice" placeholder=" ">
                                        <span slot="suffix" class="slotspan">元</span>
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">到期时间</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-date-picker v-model="save.vipetime" type="date" placeholder="选择日期"
                                        @change="changeDate" :picker-options="pickerOptions">
                                    </el-date-picker>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">备　　注</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="save.remark">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span></span>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="saveFun" :disabled="submitLoading">开通</el-button>
            </div>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props: {
        ratinglist: Array,
        ratingid: String
    },
    watch: {
        ratingid: {
            handler(newValue, oldValue) {
                console.log(newValue)
                this.save.ratingid = newValue;
            },
            deep: true,
            immediate: true
        }
    },
    data: function () {
        return {
            select: '',
            input: '',
            value: '',
            radio: '1',
            options: [],
            comnameList: [],
            usernameList: [],
            save: {
                username: '',
                comname: '',
                leijia: '2',
                ratingid: "0",
                taocanshijian: '0',
                vipprice: 0,
                vipetime: '',
                remark: '',
                uid: '',
            },
            uri: "m=yunying&c=",
            leijiaStatus: false,
            package_data: {
                time: '',
                rating_name: ''
            },
            expireTimeStatus: true,
            pickerOptions: {//el-date-picker 时间限定
                disabledDate(time) {
                    // 今天及今天之前的日期
                    // return time.getTime() > Date.now();
                    // 今天及今天之后的日期
                    return time.getTime() < Date.now() - 8.64e7;
                }
            },
            submitLoading: false,
        }
    },
    mounted() {

    },
    methods: {
        remoteMethod: function (query, cb) {
            let url = this.uri + "finance_recharge&a=searchname";
            this.save.comname = '';
            let sendData = {
                username: query
            }
            let _this = this;
            httpPost(url, sendData).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.usernameList = res.namelist;
                    let callBackArr = []; // 准备一个空数组，此数组是最终返给输入框的数组
                    // 这个res是发请求，从后台获取的数据
                    _this.usernameList.forEach((item) => {
                        // if (item.value.indexOf(queryString) == 0) { // 等于0 以什么什么开头
                        item.value = item.username;
                        if (item.value.indexOf(query) > -1) { // 大于-1，只要包含就行，不再乎位置
                            // 如果有具有关联性的数据
                            callBackArr.push(item); // 就存到callBackArr里面准备返回呈现
                        }
                    });
                    // 经过这么一波查询操作以后，如果这个数组还为空，说明没有查询到具有关联的数据，就直接返回给用户暂无数据
                    if (callBackArr.length == 0) {
                        cb([]);
                    } else {
                        cb(callBackArr);
                    }
                } else {
                    cb([]);
                }
            })
        },
        usernameChange(item) {
            this.comnameList = [];
            this.save.comname = item.comname;
            this.save.uid = item.uid;
            let vipetime = item.vipetime;
            let vipetime_ymd = item.vipetime_ymd;
            let $time = parseInt(Date.now() / 1000);
            if (vipetime_ymd == '不限' || vipetime > $time) {
                this.expireTimeStatus = false;
            } else {
                this.expireTimeStatus = true;
            }
            this.package_data.time = vipetime
            this.package_data.time_ymd = vipetime_ymd
            this.package_data.rating_name = item.rating_name
        },

        remoteMethodCom: function (query, cb) {
            let url = this.uri + "finance_recharge&a=searchcom";
            let sendData = {
                comname: query
            }
            this.save.username = '';
            let _this = this;
            httpPost(url, sendData, { hideloading: true }).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.comnameList = res.namelist;
                    let callBackArr = []; // 准备一个空数组，此数组是最终返给输入框的数组
                    // 这个res是发请求，从后台获取的数据
                    _this.comnameList.forEach((item) => {
                        // if (item.value.indexOf(queryString) == 0) { // 等于0 以什么什么开头
                        item.value = item.comname;
                        if (item.value.indexOf(query) > -1) { // 大于-1，只要包含就行，不再乎位置
                            // 如果有具有关联性的数据
                            callBackArr.push(item); // 就存到callBackArr里面准备返回呈现
                        }
                    });
                    // 经过这么一波查询操作以后，如果这个数组还为空，说明没有查询到具有关联的数据，就直接返回给用户暂无数据
                    if (callBackArr.length == 0) {
                        cb([]);
                    } else {
                        cb(callBackArr);
                    }
                } else {
                    cb([]);
                }
            })
        },
        comnameChange(item) {
            // let key = this.comnameList.findIndex(item => item.comname === e);
            // console.log(key);
            this.save.username = item.username;
            this.save.uid = item.uid;
            let vipetime = item.vipetime;
            let vipetime_ymd = item.vipetime_ymd;
            let $time = parseInt(Date.now() / 1000);
            if (vipetime_ymd == '不限' || vipetime > $time) {
                this.expireTimeStatus = false;
            } else {
                this.expireTimeStatus = true;
            }
            this.package_data.time = vipetime
            this.package_data.time_ymd = vipetime_ymd
            this.package_data.rating_name = item.rating_name;
        },
        ratingFun(e) {
            let key = this.ratinglist.findIndex(item => item.id === e);
            this.save.vipprice = this.ratinglist[key]['service_price'];
            this.save.taocanshijian = this.ratinglist[key]['service_time'];
            let taocanshijian = this.save.taocanshijian;
            var $time = parseInt(Date.now() / 1000);
            var service_timeSec = taocanshijian * 86400;
            let vipetime = '';
            if (this.leijiaStatus) {
                let oldeTime = parseInt(this.package_data.time);
                if (oldeTime < $time) {
                    oldeTime = $time;
                }
                vipetime = (oldeTime + service_timeSec) * 1000;
            } else {
                vipetime = ($time + service_timeSec) * 1000;
            }
            this.save.vipetime = timestampToTime(vipetime);
        },
        leijiaFun: function () {
            let leijia = this.save.leijia;
            if (!this.save.comname) {
                message.error('请选择企业名称');
                return;
            }
            if (!this.save.username) {
                message.error('请选择用户名称');
                return;
            }
            if (leijia == '1') {
                this.leijiaStatus = true;
            } else {
                this.leijiaStatus = false;
            }
        },
        changeDate: function (e) {
            if (e == null) {
                this.save.taocanshijian = 0;
                return;
            }
            let time = (Date.parse(e)) / 1000
            let oldeTime = '';
            if (this.leijiaStatus) {
                oldeTime = this.package_data.time
                var $time = parseInt(Date.now() / 1000);
                if (oldeTime < $time) {
                    oldeTime = $time;
                }
            } else {
                if (!this.save.comname) {
                    message.error('请选择企业名称');
                    return;
                }
                if (!this.save.username) {
                    message.error('请选择用户名称');
                    return;
                }
                oldeTime = (Date.parse(new Date())) / 1000;
            }
            let days = Math.ceil((time - oldeTime) / 86400);
            this.save.taocanshijian = days;
            this.save.vipetime = e;
        },
        saveFun: function () {

            if (!this.save.comname) {
                message.error('请选择企业名称');
                return;
            }
            if (!this.save.username) {
                message.error('请选择用户名称');
                return;
            }

            if (!this.save.ratingid) {
                message.error('请选择开通等级！');
                return;
            }
            let url = this.uri + "finance_recharge&a=comvip";
            let _this = this;
            _this.submitLoading = true;
            httpPost(url, _this.save).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(res.msg);
                    _this.save = {
                        username: '',
                        comname: '',
                        leijia: '2',
                        ratingid: _this.ratingid,
                        taocanshijian: '0',
                        vipprice: 0,
                        vipetime: '',
                        remark: '',
                        uid: ''
                    };
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        }
    },
};
function timestampToTime(timestamp) {
    var date = new Date(timestamp);
    var Y = date.getFullYear() + '-';
    var M = (date.getMonth() + 1 < 10 ? '0' + (date.getMonth() + 1) : date.getMonth() + 1) + '-';
    var D = (date.getDate() < 10 ? '0' + date.getDate() : date.getDate()) + ' ';
    // var h = (date.getHours() < 10 ? '0'+date.getHours() : date.getHours()) + ':';
    // var m = (date.getMinutes() < 10 ? '0'+date.getMinutes() : date.getMinutes()) + ':';
    // var s = (date.getSeconds() < 10 ? '0'+date.getSeconds() : date.getSeconds());
    // strDate = Y+M+D+h+m+s;
    strDate = Y + M + D;
    return strDate;

}
</script>
<style scoped>
.moduleTable {
    max-height: calc(100% - (60px + 10px));
}

.TableFlexty {
    overflow: hidden;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.TableFlexty .TableInpt {
    overflow: hidden;
    position: relative;
    width: calc(100% - 70px);
}
</style>