<template>
<div id="yemanapp" class="moduleDome">
        <div class="moduleHeadr">
            <el-page-header @back="goBack" :content="lc('wap_js_00073')">
            </el-page-header>
            <!-- <span>设置前台导航菜单，可以修改增加排序导航，点击此处</span> -->
        </div>
        <div class="moduleTable">
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
                            <div class="TableTite">{{ lc('admin_00219') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="input3">
                                    <!-- <template #suffix><span class="slotspan">天</span></template> -->
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00219') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00140') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :placeholder="lc('wap_user_00076')" v-model="input3">
                                    <!-- <template #suffix><span class="slotspan">天</span></template> -->
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00140') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00105') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="input3">
                                    <!-- <template #suffix><span class="slotspan">天</span></template> -->
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00218') }}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium">{{ lc('wap_user_00176') }}</el-button>
        </div>
    </div>
</template>

<script>
import Jibenset from './component/jibenset.vue'

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
                    radio: '1',
                    options: [{
                        value: lc('admin_demo_00001'),
                        label: window.lc('admin_00327')
                    }, {
                        value: lc('admin_demo_00002'),
                        label: window.lc('admin_00328')
                    }, {
                        value: lc('admin_demo_00003'),
                        label: window.lc('admin_00329')
                    }, {
                        value: lc('admin_demo_00004'),
                        label: window.lc('admin_00330')
                    }, {
                        value: lc('admin_demo_00005'),
                        label: window.lc('admin_00331')
                    }],
                    value: '',
                    input3: '',
                    fileList: [
                        {
                            name: 'food.jpeg',
                            url: 'https://fuss10.elemecdn.com/3/63/4e7f3a15429bfda99bce42a18cdd1jpeg.jpeg?imageMogr2/thumbnail/360x360/format/webp/quality/100'
                        },
                    ]
                }
            },
            components: {
                // 'jibenset': Jibenset,
            },
            methods: {
                handleClick(tab, event) {
                    console.log(tab, event);


                },
                goBack() {
                    history.go(-1);
                },
                handleRemove(file, fileList) {
                    console.log(file, fileList);
                },
                handlePreview(file) {
                    console.log(file);
                },
            }
        }
</script>
