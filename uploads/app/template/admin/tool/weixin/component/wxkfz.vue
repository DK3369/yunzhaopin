<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                        <th width="400">{yun:}t key='member_user_00181'{/yun}</th>
                        <th>{yun:}t key='member_com_00207'{/yun}</th>
                    </tr>
                </thead>
                <tbody>
                   
                    <tr>
                        <td>
                            <div class="TableTite">AppId</div>
                        </td>
                        <td>
                          <div class="TableInpt">
                              <el-input v-model="configdata.wx_appid"></el-input>
                          </div>
                        </td>
                        <td>
                            <div class="TableShuom" >
                                <span>{yun:}t key='admin_tool_00639'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    
                     <tr>
                         <td>
                             <div class="TableTite">AppSecret</div>
                         </td>
                         <td>
                           <div class="TableInpt">
                               <el-input v-model="configdata.wx_appsecret"></el-input>
                           </div>
                         </td>
                         <td>
                             <div class="TableShuom">
                                 <span>{yun:}t key='admin_tool_00640'{/yun}</span>
                             </div>
                         </td>
                     </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="post">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props:{
        config: {
            type: Object,
            default: {}
        },
    },
    data: function () {
        return {
            configdata:{}
        }
    },

    watch:{
        config:{
            handler(val, oldVal) {
                this.configdata = val;
            },
            immediate: true,
            deep:true,
        }
    },
    methods: {
        post:function(){
            this.$emit('post-set',{type:'wxkfz',config:this.configdata})
        }
    },
};
</script>
<style scoped></style>