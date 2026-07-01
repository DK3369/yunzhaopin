<template>
    <div class="moduleElHight">

        <div class=" moduleTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="200">{{ lc('member_com_00021') }}</th>
                        <th width="440">{{ lc('member_user_00181') }}</th>
                        <th>{{ lc('member_com_00207') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01238') }}</div>
                        </td>
                        <td>
                            <div class="TableFlexty">
                                <div class="TableInpt">
                                    <el-autocomplete
                                        v-model="save.username2"
                                        :fetch-suggestions="remoteMethod"
                                        :placeholder="lc('wap_user_00076')"
                                        :trigger-on-focus="false"
                                        @select="usernameChange"
                                        size="small"
                                    ></el-autocomplete>
<!--                                    <el-input v-model="input" placeholder=" "></el-input>-->
                                </div>
                                <div class="TableButn">
<!--                                    <el-button type="primary">检索</el-button>-->
                                </div>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01239') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01240') }}</div>
                        </td>
                        <td>
                            <div class="TableFlexty">
                                <div class="TableInpt">
                                    <el-autocomplete
                                        v-model="save.comname2"
                                        :fetch-suggestions="remoteMethodCom"
                                        :placeholder="lc('wap_user_00076')"
                                        :trigger-on-focus="false"
                                        @select="comnameChange"
                                        size="small"
                                    ></el-autocomplete>
<!--                                    <el-input v-model="input" placeholder=" "></el-input>-->
                                </div>
                                <div class="TableButn">
                                </div>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01240') }}</span>
                            </div>
                        </td>
                    </tr>
                    
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('wap_com_00393') }}</div>
                        </td>
                        <td>
                            <div class="TableSelect" style="display: flex;align-items: center;">
                                <el-select v-model="save.serviceId" :placeholder="lc('wap_user_00100')" @change="handleChange">
                                    <el-option v-for="item in serviceList" :key="item.id" :label="item.name"
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
                    <tr v-if="save.serviceId">
                        <td>
                        </td>
                        <td>
                            <div class="TableSelect" style="align-items: center;">
                                <div class="TableButn" v-for="(item,key) in serviceDetails" :key="key">
                                    <el-radio-group v-model="save.service_package" @change="serviceChange(item)">
                                        <el-radio :label="item.id">
                                            <span v-if="item.resume>0">{{ lc('wap_00451') }}：{{item.resume}}{{ lc('common_02052') }}</span>
                                            <span v-if="item.interview>0">{{ lc('resume_00029') }}：{{item.interview}}{{ lc('common_02052') }}</span>
                                            <span v-if="item.job_num>0">{{ lc('wap_com_00028') }}：{{item.job_num}}{{ lc('common_02052') }}</span>
                                            <span v-if="item.breakjob_num>0">{{ lc('wap_com_00029') }}：{{item.breakjob_num}}{{ lc('common_02052') }}</span>
                                            <span v-if="item.top_num>0">{{ lc('wap_com_00029') }}：{{item.top_num}}{{ lc('common_02052') }}</span>
                                            <span v-if="item.rec_num>0">{{ lc('wap_com_00237') }}：{{item.rec_num}}{{ lc('common_02052') }}</span>
                                            <span v-if="item.urgent_num>0">{{ lc('member_com_00613') }}：{{item.rec_num}}{{ lc('common_02052') }}</span>
                                            <span v-if="item.zph_num>0">{{ lc('admin_user_company_00210') }}：{{item.zph_num}}{{ lc('common_02052') }}</span>
                                            
                                        </el-radio>
                                    </el-radio-group>

                                </div>
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
                            <div class="TableTite">{{ lc('wap_00563') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="save.service_price" placeholder=" " onKeyUp="priceCk(this)">
                                    <span slot="suffix" class="slotspan">{{ lc('common_02056') }}</span>
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
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="saveFun" :disabled="submitLoading">{{ lc('admin_01247') }}</el-button>
            </div>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props:{
        serviceList:Array,
    },
    data: function () {
        return {
            select: '',
            input: '',
            value: '',
            value1: '',
            textarea: '',
            usernameList: [],
            comnameList:[],
            save:{
                username2:'',
                comname2:'',
                serviceid:'',
                service_package:'',
                service_price:'',
                uid:''
            },
            submitLoading:false,
            uri:"m=yunying&c=",
            serviceDetails:[],
        }
    },

    mounted() {

    },
    methods: {
        remoteMethod:function (query,cb){
            let  url = this.uri+"finance_recharge&a=searchname";
            console.log(query);
            this.save.comname2 = ''
            let sendData = {
                username:query
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
                }else {
                    cb([]);
                }
            })
        },
        usernameChange(item){
            this.save.comname2 = item.comname;
            this.save.uid = item.uid;
        },
        remoteMethodCom:function (query,cb){
            let  url = this.uri+"finance_recharge&a=searchcom";
            let sendData = {
                comname:query
            }
            this.save.username2 = ''
            let _this = this;
            httpPost(url, sendData,{hideloading:true}).then(function (response) {
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
                }else {
                    cb([]);
                }
            })
        },
        comnameChange(item){
            this.save.username2 = item.username;
            this.save.uid = item.uid;
        },
        handleChange(){
            let  url = this.uri+"finance_recharge&a=getservice";
            let sendData = {
                type:this.save.serviceId
            }
            let _this = this;
            httpPost(url, sendData).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.serviceDetails = res.data;
                }
            })
        },
        serviceChange:function(detail){
            this.save.service_price = detail.service_price;
        },
        saveFun:function (){
            let url = this.uri+"finance_recharge&a=comservice";

            let _this = this;
            _this.submitLoading = true;
            httpPost(url, _this.save).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(res.msg);
                    _this.save = {
                        username2:'',
                        comname2:'',
                        serviceid:'',
                        service_package:'',
                        service_price:'',
                        uid:''
                    };
                }else{
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