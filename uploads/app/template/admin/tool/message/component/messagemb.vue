<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="设置通知之前，请先配置好短信设置,否则将无法收到短信" type="success" :closable="false">
            </el-alert>
        </div>
        <div class=" moduleTable">
            <div style="overflow-y: auto;position: relative; height: calc(100% - 80px);">
                <div v-for="con in allconfig" :key="con.name">
                    <el-divider content-position="left">{{con.name}}</el-divider>
                    <table class="tableVue">
                        <thead>
                            <tr align="left">
                                <th width="200">名称</th>
                                <th>状态</th>
                                <th width="100">操作</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="item in con.configarr" :key="item.tpl">
                                <td>
                                    <div class="TableTite">{{item.name}}</div>
                                </td>
                                <td>
                                    <div class="TableButn">
                                        <el-switch v-model="item.config_val" active-text="通知" inactive-text="不通知" active-value="1"></el-switch>
                                    </div>
                                </td>
                                <td>
                                    <div class="TableLink">
                                        <el-link type="primary" @click="settpl(item.tpl)">设置模板</el-link>
                                    </div>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" :loading='post_loading' size="medium" @click="postSet">提交</el-button>
            </div>
        </div>
        <!-- 弹窗 -->
        <div class="modluDrawer">
            <el-drawer title="短信模板设置" :visible.sync="addshow" :modal-append-to-body="false" :show-close="true" :with-header="true" size="50%">
                <addtpl :tpl="tpl" :key="timer" @close-update="addshow=false"></addtpl>
            </el-drawer>
        </div>
    </div>
</template>
<script>
var timer = null;
module.exports = {
    data: function() {
        return {
            islook: false,
            allconfig: [],

            post_loading: false,

            addshow: false,
            tpl: '',
            timer: ''
        }
    },
    components: {
        'addtpl': httpVueLoader('./addtpl.vue'),
    },
    mounted() {
        this.getInfo();
    },
    methods: {
        async getInfo() {
            let that = this;

            httpPost('m=tool&c=messageset&a=tplswitch', {}).then((result) => {

                var res = result.data;
                if (res.error == 0) {
                    that.allconfig = res.data;

                }
                that.islook = true;
            }).catch(function(e) {
                console.log(e)
            })
        },
        async postSet() {

            let that = this;

            var param = {};

            var allconfig = that.allconfig;
            var configarr = [];
            var config_name = '';

            for (let i in allconfig) {
                configarr = allconfig[i]['configarr'];
                for (let j in configarr) {
                    config_name = configarr[j]['config_name'];
                    param[config_name] = configarr[j]['config_val'] == 1 ? 1 : 2;
                }
            }

            that.post_loading = true;

            httpPost('m=tool&c=messageset&a=save', param).then((result) => {

                that.post_loading = false;

                var res = result.data;

                message.success(res.msg, that.getInfo);

            }).catch(function(e) {
                console.log(e)
            })
        },
        async settpl(tpl) {

            this.tpl = tpl;

            this.timer = new Date().getTime();

            this.addshow = true;

        }
    },
};
</script>
<style scoped>
.moduleTable {
    max-height: calc(100% - 45px);
    height: 100%;
}
</style>