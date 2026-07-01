<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{{ lc('member_com_00021') }}</th>
                        <th width="400">{{ lc('member_user_00181') }}</th>
                        <th>{{ lc('member_com_00207') }}</th>
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
                                <span>{{ lc('admin_tool_00639') }}</span>
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
                                 <span>{{ lc('admin_tool_00640') }}</span>
                             </div>
                         </td>
                     </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="post">{{ lc('common.submit') }}</el-button>
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