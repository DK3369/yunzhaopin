<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">{{ lc('member_com_00021') }}</th>
                    <th width="360">{{ lc('member_user_00181') }}</th>
                    <th>{{ lc('member_com_00207') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00297') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="list.ismemcache">
                                <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                                <el-radio label="2">{{ lc('common.close') }}</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00908') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00295') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="list.memcachehost" :placeholder="lc('wap_user_00076')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_system_00293') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00296') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="list.memcacheport" @input="inputIntNumber($event, 'list', 'memcacheport')" :placeholder="lc('wap_user_00076')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00909') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00294') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="list.memcachetime" @input="inputIntNumber($event, 'list', 'memcachetime')" :placeholder="lc('wap_user_00076')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00910') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00301') }}</div>
                    </td>
                    <td>
                        <div class="TableButn" style="display: flex;align-items: center;">
                            <div>
                                <el-radio-group v-model="list.webcache">
                                    <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                                    <el-radio label="2">{{ lc('common.close') }}</el-radio>
                                </el-radio-group>
                            </div>
                            <div class="huncmokuai">
                                <a href="#" @click="setCache">{{ lc('admin_system_00299') }}</a>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00911') }}</span>{{ lc('admin_system_00299') }}
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00300') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="list.webcachetime" @input="inputIntNumber($event, 'list', 'webcachetime')" :placeholder="lc('wap_user_00076')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00910') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00912') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="list.issmartycache">
                                <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                                <el-radio label="2">{{ lc('common.close') }}</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00912') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00913') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="list.smartycachetime" @input="inputIntNumber($event, 'list', 'smartycachetime')" :placeholder="lc('wap_user_00076')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00910') }}</span>
                        </div>
                    </td>
                </tr>

                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
        </div>
        <el-drawer :title="lc('admin_00914')" :visible.sync="cacheDrawer" :modal-append-to-body="false" :show-close="true"
                   :with-header="true" size="70%">
            <div class="tableDome_tip">
                <el-alert :title="lc('admin_system_00298')" type="success" show-icon>
                </el-alert>
            </div>
            <div class="moduleTable">
                <div class="moduleHuancun">
                    <ul>
                        <li>
                            <div class="moduleHcTite">
                                <span>{{ lc('common.home') }}</span>
                            </div>
                            <div class="moduleHcKg">
                                <el-radio-group v-model="sy_index_cache">
                                    <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                                    <el-radio label="2">{{ lc('common.close') }}</el-radio>
                                </el-radio-group>
                            </div>
                        </li>
                        <li v-for="(item,index) in newModel" :key="index">
                            <div class="moduleHcTite">
                                <span>{{ item.value }}</span>
                            </div>
                            <div class="moduleHcKg">
                                <el-radio-group v-model="item.cache">
                                    <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                                    <el-radio label="2">{{ lc('common.close') }}</el-radio>
                                </el-radio-group>
                            </div>
                        </li>
                    </ul>
                </div>
            </div>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="saveCache" :disabled="saveLoading">{{ lc('common.save') }}</el-button>
            </div>
        </el-drawer>
    </div>

</template>

<script>
module.exports = {
    props: {
        list: Object
    },
    data: function () {
        return {
            input: '',
            value1: true,
            value: '',
            textarea: '',
            radio: '1',
            uri: "m=system&c=",
            title: lc('admin_00914'),
            cacheDrawer: false,
            newModel:[],
            sy_index_cache:'',
            saveLoading: false
        }
    },
    mounted() {
    },
    methods: {
        inputIntNumber(val, form, key) {
            this.$props[form][key] = val.replace(/[^0-9]/g,'');
        },
        save: function () {
            let _this = this;
            let url = this.uri + "set_config&a=save";
            let ruleForm = {
                ismemcache: _this.list.ismemcache == 1 ? 1 : 2,
                issmartycache: _this.list.issmartycache == 1 ? 1 : 2,
                memcachehost: _this.list.memcachehost,
                memcacheport: _this.list.memcacheport,
                memcachetime: _this.list.memcachetime,
                smartycachetime: _this.list.smartycachetime,
                webcache: _this.list.webcache == 1 ? 1 : 2,
                webcachetime: _this.list.webcachetime
            };
            _this.saveLoading = true;
            httpPost(url, ruleForm).then(function (response) {
                var res = response.data;
                if (res.error == 0) {
                    message.success(lc('wap_user_00264'));
                    _this.$emit('get-list', true)
                } else {
                    message.error(res.msg);
                }
            }).finally(function () {
                setTimeout(function () {
                    _this.saveLoading = false;
                }, 2000);
            });
        },
        setCache() {
            this.cacheDrawer = true;
            this.getSettplcache();
        },
        getSettplcache() {
            const _this = this;
            let url = this.uri + "set_config&a=settplcache";
            httpPost(url, {}).then(function (response) {
                var res = response.data;
                if (res.error == 0) {
                    _this.newModel = res.data.newModel;
                    _this.sy_index_cache = res.data.sy_index_cache;
                } else {
                    message.error(res.msg);
                }
            })
        },
        saveCache: function () {
            const _this = this;
            let url = this.uri + "set_config&a=savetplcache";
            let ruleForm = {
                sy_index_cache: this.sy_index_cache
            }
            for (const ruleFormKey in this.newModel) {
                // console.log(ruleFormKey.cache);
                let key = 'sy_' + ruleFormKey + "_cache"
                ruleForm[key] = this.newModel[ruleFormKey].cache;
            }
            _this.saveLoading = true;
            httpPost(url, ruleForm).then(function (response) {
                var res = response.data;
                if (res.error == 0) {
                    message.success(res.msg);
                    _this.getSettplcache();
                    _this.cacheDrawer = false;

                } else {
                    message.error(res.msg);
                }
            }).finally(function () {
                setTimeout(function () {
                    _this.saveLoading = false;
                }, 2000);
            });
        }
    },
};
</script>
<style scoped></style>