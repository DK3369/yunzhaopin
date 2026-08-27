<template>
    <div class="tabseoTemps">
        <div class="moduleHeadrcz">
            <div class="moduleHeadrButn">
                <el-button type="primary" icon="el-icon-document-add" size="small" @click="openSeo('')">{{ lc('admin_system_00380') }}</el-button>
            </div>
        </div>
        <el-table :data="list" border style="width: 100%" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
            :height="tableHeight" v-loading="loading" :empty-text="emptytext" height="calc(100% - 40px)">
            <el-table-column type="selection" width="55">
            </el-table-column>
            <el-table-column prop="seoname" :label="lc('member_com_00021')" width="160">
            </el-table-column>
            <el-table-column prop="ident" :label="lc('admin_system_00371')">
            </el-table-column>
            <el-table-column prop="title" :label="lc('admin_00980')">
            </el-table-column>
            <el-table-column prop="time_n" :label="lc('wap_00326')">
            </el-table-column>

            <el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
                <template #default="scope">
                    <div class="cz_button">
                        <el-button size="small" @click="openSeo(scope.row)">{{ lc('wap_js_00073') }}</el-button>
                        <el-button size="small" type="danger" @click="del(scope.$index)">{{ lc('common.delete') }}</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00308')" v-model="drawer" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="30%">
                <span>{{ lc('wap_user_00001') }}</span>
                <template #footer><span class="dialog-footer">
                    <el-button @click="drawer = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="drawer = false">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
    </div>
</template>
    
<script>
const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }
const delConfirm = (...a) => window.delConfirm(...a)
const formatDate = (...a) => window.formatDate(...a)
const formatMonth = (...a) => window.formatMonth(...a)
const formatDatetime = (...a) => window.formatDatetime(...a)
const deepClone = (...a) => window.deepClone(...a)
const scrollToTop = (...a) => window.scrollToTop(...a)
const isEmpty = (...a) => window.isEmpty(...a)
const isArray = (...a) => window.isArray(...a)
const $ = typeof window !== 'undefined' && window.$ ? window.$ : Object.assign(function(){ return { length: 0 } }, {})
const echarts = typeof window !== 'undefined' && window.echarts ? window.echarts : { init(){ return { setOption(){}, resize(){} } }, graphic: { LinearGradient: function(){} } }

export default {
    props: ['action'],
    data: function () {
        return {
            emptytext: lc('wap_js_00113'),
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
            this.emptytext = lc('admin_user_weipin_00026');
            let listlen = this.list.length
            if (listlen > 0) {
                let height = 48 + (60 * listlen);
                this.tableHeight = height > 750 ? '750px' : height + 'px';
            }
            if (this.list.length === 0){
                this.emptytext = lc('wap_js_00113');
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