<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="{yun:}t key='admin_tool_00059'{/yun}" type="success" :closable="false"></el-alert>
        </div>
        <div class=" moduleTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{yun:}t key='member_com_00021'{/yun}</th>
                    <th width="400">{yun:}t key='member_user_00181'{/yun}</th>
                    <th>{yun:}t key='member_com_00207'{/yun}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00254'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
							<el-radio-group v-model="locoy_config.locoy_keyword">
								<el-radio label="1">{yun:}t key='common.yes'{/yun}</el-radio>
								<el-radio label="2">{yun:}t key='common.no'{/yun}</el-radio>
							</el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00256'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00252'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_rand" @input="inputIntNumber($event, 'locoy_config', 'locoy_rand')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00255'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00253'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_sort" @input="inputIntNumber($event, 'locoy_config', 'locoy_sort')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00255'{/yun}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="submitLocoyConfig" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
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