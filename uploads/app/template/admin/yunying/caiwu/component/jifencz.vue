<template>
    <div class="moduleElHight">
        <div class="admin_datatip">
            <i class="el-icon-document"></i>
            充值时，请正确填写用户名和要充值的积分。确认用户名的正确性
        </div>
        <div class=" moduleTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="200">名称</th>
                        <th width="400">状态</th>
                        <th>说明</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">充值用户名</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="userarr" placeholder=" "></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>多个用户名用逗号隔开</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">充值方式</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio v-model="fs" label="1">增加</el-radio>
                                <el-radio v-model="fs" label="2">扣除</el-radio>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>充值方式</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{pricename}}(个)</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="integral" size="20" maxlength="16" placeholder=" " onKeyUp="this.value=this.value.replace(/[^0-9.]/g,'')"></el-input>
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
                                <el-input name="order_price" id="order_price" size="20" maxlength="16" v-model="order_price" onKeyUp="priceCk(this)" placeholder=" ">
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
                            <div class="TableTite">备　　注</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="remark">
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
                <el-button type="primary" size="medium" @click="save" :disabled="submitLoading">提交</el-button>
            </div>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props: {
        pricename: String,
    },
    data: function () {
        return {
            submitLoading:false,
            input: '',
            userarr:'',
            fs: '1',
            integral:'',
            order_price: '',
            remark: '',
            uri:"m=yunying&c=",
        }
    },
    mounted() {

    },
    methods: {
        save:function(){
            let userarr = this.userarr;
            let integral = this.integral;
            if (userarr == '') {
                message.error('请输入用户名！');
                return false;
            }
            if (integral < 1) {
                message.error('请输入'+this.pricename+'！');
                return false;
            }
            let _this = this;
            let  url= _this.uri+'finance_recharge&a=jifenSave';
            let sendData ={
                fs:this.fs,
                integral:this.integral,
                order_price:this.order_price,
                remark:this.remark,
                userarr:userarr
            };
            this.submitLoading = true;
            httpPost(url, sendData).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(res.msg);
                    _this.fs =1;
                    _this.userarr='';
                    _this.integral='';
                    _this.order_price= '';
                    _this.remark='';
                }else {
                    message.error(res.msg)
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
</style>