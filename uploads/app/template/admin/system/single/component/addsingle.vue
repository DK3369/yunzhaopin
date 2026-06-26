<template>
    <div class="drawerModlue"  v-loading="addloading">
        <div class="drawerModInfo">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='wap_00468'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.name" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='default_00321'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="info.nid" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                        <el-option v-for="item in class_arr" :key="item.id" :label="item.name" :value="item.id">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00671'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio-group v-model="info.is_type">
                        <el-radio v-for="item in type_arr" :key="item.label" :label="item.label">{{item.name}}</el-radio>
                    </el-radio-group>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00666'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.url" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_system_00664'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='wap_js_00099'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.title" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00662'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.keyword" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_system_00665'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='wap_00476'{/yun}</span>
                </div>
                <div class="drawerModInpt">
					<el-input type="textarea" rows="2" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="info.descs"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00660'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="info.top_tpl" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                        <el-option v-for="item in tpl_arr" :key="item.value" :label="item.label" :value="item.value"></el-option>
                    </el-select>
                </div>
                <div class="drawerModTips">
                    <el-alert v-if="info.top_tpl==1" title="{yun:}t key='admin_system_00667'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    <div v-if="info.top_tpl==3" style="overflow: hidden; position: relative; margin-top: 10px;">
                        <el-input v-model="info.top_tpl_dir" placeholder="{yun:}t key='admin_system_00672'{/yun}"></el-input>
                        <el-alert title="{yun:}t key='admin_system_00659'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00669'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="info.footer_tpl" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                        <el-option v-for="item in tpl_arr" :key="item.value" :label="item.label" :value="item.value"></el-option>
                    </el-select>
                </div>
                <div class="drawerModTips">
                    <el-alert v-if="info.footer_tpl==1" title="{yun:}t key='admin_system_00670'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    <div v-if="info.footer_tpl==3" style="overflow: hidden; position: relative; margin-top: 10px;">
                        <el-input v-model="info.footer_tpl_dir" placeholder="{yun:}t key='admin_system_00672'{/yun}"></el-input>
                        <el-alert title="{yun:}t key='admin_system_00658'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00673'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <textarea type="textarea" id="projectBasis" class="editor" name="projectBasis" cols="150" rows="30">
                    </textarea>
                    <!-- <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="info.content">
                    </el-input> -->
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00103'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.sort" @input="inputIntNumber($event, 'info', 'sort')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00668'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-switch v-model="info.is_nav" active-text="显示" inactive-text="不显示" active-value="1" inactive-value="0">
                    </el-switch>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="saveinfo" :loading="saveloading">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
    </div>
</template>

<script>
var ue = null;

module.exports = {
    props: {
        sid: {
            type: [String, Number],
            default: ''
        },
    },
    data: function () {
        return {
            info:{
                name:'',
                is_nav:'0',
                sort:'',
                content:'',
                footer_tpl:'1',
                footer_tpl_dir:'',
                top_tpl:'1',
                top_tpl_dir:'',
                descs:'',
                keyword:'',
                title:'',
                url:'',
                nid:'',
                is_type:'1',
            },
            class_arr:[],
            tpl_arr:[
                {label:window.yunAdminT("{yun:}t key='member_user_00283'{/yun}"),value:'1'},
                {label:window.yunAdminT("{yun:}t key='admin_system_00674'{/yun}"),value:'2'},
                {label:window.yunAdminT("{yun:}t key='admin_system_00675'{/yun}"),value:'3'}
            ],
            type_arr:[
                {label:'1',name:window.yunAdminT("{yun:}t key='admin_system_00661'{/yun}")},
                {label:'0',name:window.yunAdminT("{yun:}t key='admin_system_00663'{/yun}")},
                {label:'2',name:window.yunAdminT("{yun:}t key='admin_00198'{/yun}")}
            ],
            addloading:false,
            saveloading:false,
        }
    },
    mounted() {
        ue = UE.getEditor('projectBasis", {
            wordCount: false,           // 关闭字数统计
            elementPathEnabled: false,  //{yun:}t key='common.close'{/yun}elementPath {yun:}t key='common_05704'{/yun}
            autoHeightEnabled: false,   //关闭自适应高度，超出部分以滚动条形式展示
            initialFrameHeight: 480,    //默认的编辑区域高度
            initialFrameWidth: 600      //初始化编辑器宽度,{yun:}t key='wap_js_00098'{/yun}1000
        });
    },
    created:function(){
        this.getInfo();
    },

    methods: {
        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g,"');
        },
        async getInfo() {
            let that = this;
            let params = {
                id:that.sid
            }
            this.addloading = true;
            httpPost('m=system&c=singlepage&a=add', params).then((result)=>{

                this.addloading = false;
                var res = result.data;
                if (res.error == 0) {
                    that.class_arr = res.data.class
                    if(that.sid!=''){
                        that.info = res.data.info;
                    }
                    ue = UE.getEditor('projectBasis", {
                        wordCount: false,           // 关闭字数统计
                        elementPathEnabled: false,  //{yun:}t key='common.close'{/yun}elementPath {yun:}t key='common_05704'{/yun}
                        autoHeightEnabled: false,   //关闭自适应高度，超出部分以滚动条形式展示
                        initialFrameHeight: 480,    //默认的编辑区域高度
                        initialFrameWidth: 600      //初始化编辑器宽度,{yun:}t key='wap_js_00098'{/yun}1000
                    });
                    ue.ready(function () {
                        if (that.info.content) {
                            ue.setContent(that.info.content);
                        } else {
                            ue.setContent("');
                        }
                    });

                }
            }).catch(function(e){
                console.log(e)
            })
        },
        saveinfo: function () {
            var that = this;

            if (that.info.name == '') {
                message.error(window.yunAdminT("{yun:}t key='admin_system_00676'{/yun}"));
                return false;
            }
            if (that.info.url == '') {
                message.error(window.yunAdminT("{yun:}t key='admin_system_00677'{/yun}"));
                return false;
            }
            if (that.info.title == '') {
                message.error(window.yunAdminT("{yun:}t key='wap_user_00075'{/yun}"));
                return false;
            }

            var param = {
                id:that.sid,
                name:that.info.name,
                is_nav:that.info.is_nav,
                sort:that.info.sort,
                content:UE.getEditor('projectBasis').getContent(),
                footer_tpl:that.info.footer_tpl,
                footer_tpl_dir:that.info.footer_tpl_dir,
                top_tpl:that.info.top_tpl,
                top_tpl_dir:that.info.top_tpl_dir,
                description:that.info.descs,
                keyword:that.info.keyword,
                title:that.info.title,
                url:that.info.url,
                nid:that.info.nid,
                is_type:that.info.is_type,
            };

            this.saveloading = true;

            httpPost('m=system&c=singlepage&a=save', param).then(function(res) {
                if (res.data.error == 0) {
                    message.success(window.yunAdminT("{yun:}t key='admin_system_00678'{/yun}"),function(){
                        that.$emit("close-update");
                    });
                } else {
                    message.error(window.yunAdminT("{yun:}t key='admin_system_00679'{/yun}"));
                }
            }).finally(function () {
                setTimeout(function () {
                    that.saveloading = false;
                }, 2000);
            });
        }
    },
    watch: {
        sid: function (val, oldVal) {
            this.info = {};
            console.log('val',val)
            this.getInfo();
        },
    }
};
</script>
<style scoped>
.drawerModInfo::-webkit-scrollbar {
    display: none;
}
.drawerModlue{
    overflow: hidden;
    position: relative;
    width: 100%;
    height: 100%;
}
.drawerModInfo{
    overflow-y: auto;
    height: calc(100% - 80px);
}
</style>