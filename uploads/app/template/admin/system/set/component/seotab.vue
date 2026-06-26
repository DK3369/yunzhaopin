<template>
    <div class="tabseoTemps">
        <div class="moduleHeadrcz">
            <div class="moduleHeadrButn">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="openSeo('')">{yun:}t key='admin_system_00380'{/yun}</el-button>
            </div>
        </div>
        <el-table :data="list" border style="width: 100%" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
            :height="tableHeight" v-loading="loading" :empty-text="emptytext" height="calc(100% - 40px)">
            <el-table-column type="selection" width="55">
            </el-table-column>
            <el-table-column prop="seoname" label="{yun:}t key='member_com_00021'{/yun}" width="160">
            </el-table-column>
            <el-table-column prop="ident" label="{yun:}t key='admin_system_00371'{/yun}">
            </el-table-column>
            <el-table-column prop="title" label="{yun:}t key='admin_00980'{/yun}">
            </el-table-column>
            <el-table-column prop="time_n" label="{yun:}t key='wap_00326'{/yun}">
            </el-table-column>

            <el-table-column fixed="right" label="{yun:}t key='member_user_00048'{/yun}" width="140">
                <template slot-scope="scope">
                    <div class="cz_button">
                        <el-button size="mini" @click="openSeo(scope.row)">{yun:}t key='wap_js_00073'{/yun}</el-button>
                        <el-button size="mini" type="danger" @click="del(scope.$index)">{yun:}t key='common.delete'{/yun}</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_00308'{/yun}" :visible.sync="drawer" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="30%">
                <span>{yun:}t key='wap_user_00001'{/yun}</span>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawer = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="drawer = false">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props: ['action'],
    data: function () {
        return {
            emptytext: "{yun:}t key='wap_js_00113'{/yun}",
            loading: false,
            drawer: false,

            tableHeight: '100%',
            list: [],
            
        }
    },
    mounted() {
    },
    created: function () {
        this.getList();
    },
    methods: {
        async getList() {
            this.loading = true;
            let res = await httpPost('m=system&c=set_seo', { action: this.action });
            let data = res.data.data;

            this.list = data.seolist ? data.seolist : [];
            this.loading = false;
            this.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
            let listlen = this.list.length
            if (listlen > 0) {
                let height = 48 + (60 * listlen);
                this.tableHeight = height > 750 ? '750px' : height + 'px';
            }
            if (this.list.length === 0){
                this.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
            }
        },
        openSeo(data) {
            custoapp.openSeoshezhi(data ? data : {});
        },
        del(idx) {
            let that = this;
            delConfirm(this, { id: that.list[idx].id }, function (params) {
                httpPost('m=system&c=set_seo&a=del', params).then(function (res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.success(res.data.msg, function () {
                            that.list.splice(idx, 1);
                        });
                    }
                })
            })
        },
    },
    watch: {
        action: function (val, oldVal) {
            this.getList();
        }
    }
};
</script>
<style scoped></style>