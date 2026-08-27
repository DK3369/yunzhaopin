<template>
    <div class="drawerModlue"  v-loading="addloading" style="display: table;">
        <div class="drawerModInfo">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_tool_00467') }}</span>
                </div>
                <div class="drawerModInpt">
                    {{tpl_n}}
                </div>
            </div>
            
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('member_user_00010') }}</span>
                </div>
                <div class="drawerModInpt">
					<el-input type="textarea" rows="2" :placeholder="lc('wap_user_00076')" v-model="info.content" show-word-limit></el-input>
                </div>
            </div>
            
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="saveinfo" :loading="saveloading">{{ lc('common.submit') }}</el-button>
        </div>
        <div>
            <table width="100%" class="table_form">
                <tr>
                    <th colspan="2" class="admin_bold_box">
                        <div class="admin_bold">{{ lc('admin_tool_00320') }}</div>
                    </th>
                </tr>
                <tr v-for="(item,index) in tpl_temp" :key="index">
                    <th width="150" height="36">{{item}}</th>
                    <td>{{ lc("admin_code_value", [index]) }}</td>
                </tr> 
                
            </table>
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
    props: {
        tpl: {
            type: String,
            default: ''
        },
    },
    data: function () {
        return {
            tpl_n:'',
            info:{},
            tpl_temp:[],
            addloading:false,
            saveloading:false,
        }
    },
    created:function(){
        this.getInfo();
    },
	
    methods: {
        async getInfo() {
            let that = this;
            let params = {
                name:that.tpl
            }
            
            this.addloading = true;

            httpPost('m=tool&c=messageset&a=gettpl', params).then((result)=>{
                
                this.addloading = false;

                var res = result.data;
                if (res.error == 0) {
                    that.info = res.data.info;
                    that.tpl_temp = res.data.tpl_temp;
                    that.tpl_n = res.data.tpl_n;
                }
            }).catch(function(e){
                console.log(e)
            })
        },
        saveinfo: function () {
            var that = this;
            
            var param = {
                name:that.tpl,
                content:that.info.content
            };
            
            this.saveloading = true;
            
            httpPost('m=tool&c=messageset&a=savetpl', param).then((res)=>{

                this.saveloading = false;
                
                if (res.data.error == 0) {
                    message.success(res.data.msg,()=>{

                        this.$emit("close-update");
                    });
                } else {
                    message.error(res.data.msg);
                }
            });
        }
    },
};
</script>
<style scoped>
.drawerModInfo::-webkit-scrollbar {
    display: none;
}

</style>