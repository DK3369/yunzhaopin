<template>
    <div class="drawerModlue"  v-loading="addloading">
        <div class="drawerModInfo">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>名称：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.name" placeholder="请输入内容"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>类别：</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="info.nid" placeholder="请选择">
                        <el-option v-for="item in class_arr" :key="item.id" :label="item.name" :value="item.id">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>类型：</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio-group v-model="info.is_type">
                        <el-radio v-for="item in type_arr" :key="item.label" :label="item.label">{{item.name}}</el-radio>
                    </el-radio-group>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>URL链接：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.url" placeholder="请输入内容"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="(可以为多级目录)。例：/about/index.html" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>标题：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.title" placeholder="请输入内容"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>关键词：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.keyword" placeholder="请输入内容"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="(多关键字，请用，隔开，请不要为空)" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>描述：</span>
                </div>
                <div class="drawerModInpt">
					<el-input type="textarea" rows="2" placeholder="请输入内容" v-model="info.descs"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>头部模板：</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="info.top_tpl" placeholder="请选择">
                        <el-option v-for="item in tpl_arr" :key="item.value" :label="item.label" :value="item.value"></el-option>
                    </el-select>
                </div>
                <div class="drawerModTips">
                    <el-alert v-if="info.top_tpl==1" title="头部默认模板为当前模板风格下的header.htm" type="info" show-icon :closable="false"></el-alert>
                    <div v-if="info.top_tpl==3" style="overflow: hidden; position: relative; margin-top: 10px;">
                        <el-input v-model="info.top_tpl_dir" placeholder="请输入目录"></el-input>
                        <el-alert title="例：default/header 注：default为当前风格目录 模板后缀为(.htm)" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>底部模板：</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="info.footer_tpl" placeholder="请选择">
                        <el-option v-for="item in tpl_arr" :key="item.value" :label="item.label" :value="item.value"></el-option>
                    </el-select>
                </div>
                <div class="drawerModTips">
                    <el-alert v-if="info.footer_tpl==1" title="底部默认模板为当前模板风格下的footer.htm" type="info" show-icon :closable="false"></el-alert>
                    <div v-if="info.footer_tpl==3" style="overflow: hidden; position: relative; margin-top: 10px;">
                        <el-input v-model="info.footer_tpl_dir" placeholder="请输入目录"></el-input>
                        <el-alert title="例：default/footer 注：default为当前风格目录 模板后缀为(.htm)" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>页面内容：</span>
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
                    <span>排序：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.sort" @input="inputIntNumber($event, 'info', 'sort')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>左则导航：</span>
                </div>
                <div class="drawerModInpt">
                    <el-switch v-model="info.is_nav" active-text="显示" inactive-text="不显示" active-value="1" inactive-value="0">
                    </el-switch>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="saveinfo" :loading="saveloading">提交</el-button>
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
                {label:'默认模板',value:'1'},
                {label:'空白模板',value:'2'},
                {label:'自定义模板',value:'3'}
            ],
            type_arr:[
                {label:'1',name:'自定义页面'},
                {label:'0',name:'外部链接'},
                {label:'2',name:'站内链接'}
            ],
            addloading:false,
            saveloading:false,
        }
    },
    mounted() {
        ue = UE.getEditor('projectBasis', {
            wordCount: false,           // 关闭字数统计
            elementPathEnabled: false,  //关闭elementPath 元素路径
            autoHeightEnabled: false,   //关闭自适应高度，超出部分以滚动条形式展示
            initialFrameHeight: 480,    //默认的编辑区域高度
            initialFrameWidth: 600      //初始化编辑器宽度,默认1000
        });
    },
    created:function(){
        this.getInfo();
    },

    methods: {
        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g,'');
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
                    ue = UE.getEditor('projectBasis', {
                        wordCount: false,           // 关闭字数统计
                        elementPathEnabled: false,  //关闭elementPath 元素路径
                        autoHeightEnabled: false,   //关闭自适应高度，超出部分以滚动条形式展示
                        initialFrameHeight: 480,    //默认的编辑区域高度
                        initialFrameWidth: 600      //初始化编辑器宽度,默认1000
                    });
                    ue.ready(function () {
                        if (that.info.content) {
                            ue.setContent(that.info.content);
                        } else {
                            ue.setContent('');
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
                message.error('请填写单页面名称');
                return false;
            }
            if (that.info.url == '') {
                message.error('请填写URL链接');
                return false;
            }
            if (that.info.title == '') {
                message.error('请填写标题');
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
                    message.success('单页面设置成功',function(){
                        that.$emit("close-update");
                    });
                } else {
                    message.error('单页面设置失败');
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