<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_tool_00059')" type="success" :closable="false"></el-alert>
        </div>
        <div class=" moduleTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{{ lc('member_com_00021') }}</th>
                    <th width="400">{{ lc('member_user_00181') }}</th>
                    <th>{{ lc('member_com_00207') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00254') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
							<el-radio-group v-model="locoy_config.locoy_keyword">
								<el-radio label="1">{{ lc('common.yes') }}</el-radio>
								<el-radio label="2">{{ lc('common.no') }}</el-radio>
							</el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00256') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00252') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_rand" @input="inputIntNumber($event, 'locoy_config', 'locoy_rand')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00255') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00253') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_sort" @input="inputIntNumber($event, 'locoy_config', 'locoy_sort')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00255') }}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="submitLocoyConfig" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
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