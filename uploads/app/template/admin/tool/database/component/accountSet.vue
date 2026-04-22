<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="采集前务必设置自己的接口密码，以免被其他人利用，这里所设置的参数，只作为没有值的情况下使用，若采集软件有值传输，会优先使用传输值" type="success" :closable="false"></el-alert>
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
                        <div class="TableTite">生成用户名长度</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_length" @input="inputIntNumber($event, 'locoy_config', 'locoy_length')" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>如：8</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">生成用户名前缀</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_name" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>如：user 加随机字符</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">生成指定密码</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_pwd" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>如：123456</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">用户状态</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="locoy_config.locoy_user_status">
                                <el-radio label="1">已审核</el-radio>
                                <el-radio label="0">未审核</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span> </span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">企业会员等级</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-select v-model="locoy_config.locoy_rating" placeholder="请选择">
                                <el-option v-for="item in ratingOptions" :key="item.value" :label="item.label" :value="item.value"></el-option>
                            </el-select>

                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span> </span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="submitLocoyConfig" :disabled="saveLoading">提交</el-button>
            </div>
        </div>
    </div>
</template>

<script>
    module.exports = {
        props: {
            locoy_config: Object,
            account_set: Number
        },
        watch: {
            locoy_config: {
                handler (n, v){
                },
                deep: true
            },
            account_set: {
                handler(newValue, oldValue) {
                    if (newValue == 1) {
                        this.getCache();
                    }
                },
                deep: true,
                immediate: true
            }
        },
        data: function () {
            return {

                ratingOptions: [],
                saveLoading: false
            }
        },
        methods: {
            inputIntNumber(val, form, key) {
                this.$props[form][key] = val.replace(/[^0-9]/g,'');
            },
            async getCache() {
                let res = await httpPost('m=tool&c=dataCollection&a=getRating');
                if (res.data.error == 0) {
                    let data = res.data.data;

                    var ratingArr = data.ratingArr;
                    ratingArr.forEach((item) => {
                        this.ratingOptions.push({value: item.id, label: item.name})
                    });
                }
            },
            submitLocoyConfig: function () {
                let that = this;
                let params = {
                    locoyConfig: 1,

                    locoy_length: that.locoy_config.locoy_length,
                    locoy_name: that.locoy_config.locoy_name,
                    locoy_pwd: that.locoy_config.locoy_pwd,
                    locoy_user_status: that.locoy_config.locoy_user_status,
                    locoy_rating: that.locoy_config.locoy_rating
                };
                that.saveLoading = true;
                httpPost('m=tool&c=dataCollection&a=setLocoyConfig', params).then(function (res) {
                    if (res.data.error == 0) {

                        message.success(res.data.msg);
                    } else {

                        message.error(res.data.msg);
                    }
                }).finally(function () {
                    setTimeout(function () {
                        that.saveLoading = false;
                    }, 2000);
                });
            },
        },
    };
</script>
<style scoped>
    .moduleTable {
        max-height: calc(100% - (60px + 10px));
    }
</style>