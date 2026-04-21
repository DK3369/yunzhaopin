<template>
    <div class="tabseoTemps">
        <div class="moduleHeadrcz">
            <div class="moduleHeadrButn">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="openSeo('')">添加SEO</el-button>
            </div>
        </div>
        <el-table :data="list" border style="width: 100%" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
            :height="tableHeight" v-loading="loading" :empty-text="emptytext" height="calc(100% - 40px)">
            <el-table-column type="selection" width="55">
            </el-table-column>
            <el-table-column prop="seoname" label="名称" width="160">
            </el-table-column>
            <el-table-column prop="ident" label="SEO标识符">
            </el-table-column>
            <el-table-column prop="title" label="网页标题">
            </el-table-column>
            <el-table-column prop="time_n" label="更新时间">
            </el-table-column>

            <el-table-column fixed="right" label="操作" width="140">
                <template slot-scope="scope">
                    <div class="cz_button">
                        <el-button size="mini" @click="openSeo(scope.row)">修改</el-button>
                        <el-button size="mini" type="danger" @click="del(scope.$index)">删除</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
        <div class="modluDrawer">
            <el-dialog title="信息" :visible.sync="drawer" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="30%">
                <span>确定要删除？</span>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawer = false">取 消</el-button>
                    <el-button type="primary" @click="drawer = false">确 定</el-button>
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
            emptytext: '暂无数据',
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
            this.emptytext = "数据加载中";
            let listlen = this.list.length
            if (listlen > 0) {
                let height = 48 + (60 * listlen);
                this.tableHeight = height > 750 ? '750px' : height + 'px';
            }
            if (this.list.length === 0){
                this.emptytext = "暂无数据";
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