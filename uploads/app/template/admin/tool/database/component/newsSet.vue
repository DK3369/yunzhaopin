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
                        <div class="TableTite">自动提取关键字</div>
                    </td>
                    <td>
                        <div class="TableButn">
							<el-radio-group v-model="locoy_config.locoy_keyword">
								<el-radio label="1">是</el-radio>
								<el-radio label="2">否</el-radio>
							</el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>注：只有在没有参数传输的才起作用</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">浏览数随机范围</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_rand" @input="inputIntNumber($event, 'locoy_config', 'locoy_rand')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>如：0-100，默认为0</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">排序随机范围</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_sort" @input="inputIntNumber($event, 'locoy_config', 'locoy_sort')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>如：0-100，默认为0</span>
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
		},
        data: function () {
            return {
                saveLoading: false
            }
        },
		watch: {
			locoy_config: {
				handler (n, v){
				},
				deep: true
			}
		},
        methods: {
            inputIntNumber(val, form, key) {
                this.$props[form][key] = val.replace(/[^0-9]/g,'');
            },
            submitLocoyConfig: function () {
                let that = this;
                let params = {
                    locoyConfig: 1,
                    locoy_keyword: that.locoy_config.locoy_keyword,
                    locoy_rand: that.locoy_config.locoy_rand,
                    locoy_sort: that.locoy_config.locoy_sort
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
    .moduleTable {max-height: calc(100% - (60px + 10px));}
</style>