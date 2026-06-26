<template>
    <div class="drawerModlue"  v-loading="addloading" style="display: table;">
        <div class="drawerModInfo">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_tool_00467'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    {{tpl_n}}
                </div>
            </div>
            
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='member_user_00010'{/yun}</span>
                </div>
                <div class="drawerModInpt">
					<el-input type="textarea" rows="2" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="info.content" show-word-limit></el-input>
                </div>
            </div>
            
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="saveinfo" :loading="saveloading">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
        <div>
            <table width="100%" class="table_form">
                <tr>
                    <th colspan="2" class="admin_bold_box">
                        <div class="admin_bold">{yun:}t key='admin_tool_00320'{/yun}</div>
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

module.exports = {
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