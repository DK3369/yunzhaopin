<template>
<div id="moduapp" class="moduleDome">
    <div class="setDomeAll setDomeInte">
        <el-tabs v-model="activeName" @tab-click="handleClick">
            <el-tab-pane :label="lc('admin_user_company_00064')" name="renzheng" :lazy="true">
                <companyrz_renzheng :status="status.renzheng"></companyrz_renzheng>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_company_00059')" name="logo" :lazy="true">
                <companyrz_pic_logo :status="status.logo"></companyrz_pic_logo>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_company_00060')" name="show" :lazy="true">
                <companyrz_pic_show :status="status.show"></companyrz_pic_show>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_company_00058')" name="banner" :lazy="true">
                <companyrz_pic_banner :status="status.banner"></companyrz_pic_banner>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_company_00061')" name="product" :lazy="true">
                <companyrz_product :status="status.product"></companyrz_product>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_company_00057')" name="news" :lazy="true">
                <companyrz_news :status="status.news"></companyrz_news>
            </el-tab-pane>
        </el-tabs>
    </div>
</div>
</template>

<script>
import CompanyrzRenzheng from './component/companyrz_renzheng.vue'
import CompanyrzPicLogo from './component/companyrz_pic_logo.vue'
import CompanyrzPicShow from './component/companyrz_pic_show.vue'
import CompanyrzPicBanner from './component/companyrz_pic_banner.vue'
import CompanyrzProduct from './component/companyrz_product.vue'
import CompanyrzNews from './component/companyrz_news.vue'

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
        data: function () {
            return {
                activeName: 'renzheng',
                status: {
                    renzheng: '',
                    logo: '',
                    show: '',
                    banner: '',
                    product: '',
                    news: '',
                },
            }
        },
        components: {
            'companyrz_renzheng': CompanyrzRenzheng,
            'companyrz_pic_logo': CompanyrzPicLogo,
            'companyrz_pic_show': CompanyrzPicShow,
            'companyrz_pic_banner': CompanyrzPicBanner,
            'companyrz_product': CompanyrzProduct,
            'companyrz_news': CompanyrzNews,
        },
        created: function () {
            var that = this
            let query = window.parent.homeapp.$route.query;


            if (query.tabs) {
                that.activeName = query.tabs;
            }
            if (query.status) {
                that.status[that.activeName] = query.status;
            }
        },
        methods: {
            handleClick(tab, event) {
                console.log(tab, event);
            }
        }
    }
</script>
