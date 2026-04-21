<template>
    <div style="overflow: hidden; position: relative; height: 100%; padding: 0 20px;">
        <div class="moduleTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="20%">名称</th>
                        <th>状态</th>
                        <th width="20%">说明</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">问答标题</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="info.title">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>问答标题</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">问答类别</div>
                        </td>
                        <td>
                            <div class="TableSelect" style="display: flex;align-items: center;">
                                <el-select v-model="info.pid" placeholder="请选择" style="width: 50%;" @change="classchange">
                                    <el-option v-for="item in positionOne" :key="item.id" :label="item.name"
                                        :value="item.id">
                                    </el-option>
                                </el-select>
                                <el-select v-model="info.cid" placeholder="请选择" style="width: 50%;" >
                                    <el-option v-for="item in positionTwo" :key="item.id" :label="item.name"
                                        :value="item.id">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>问答类别</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">访问次数</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="info.visit">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>访问次数</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">是否推荐</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-switch v-model="info.is_recom_n" active-color="#1890FF" inactive-color="#B8BDC9">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>是否推荐</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">问答内容</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <div id="editor—wrapper" style="border: 1px solid #ccc;">
                                    <div id="toolbar-container"><!-- 工具栏 --></div>
                                    <div id="editor-container" style="height: 300px;"><!-- 编辑器 --></div>
                                </div>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>问答内容</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="submitLoading">更新</el-button>
        </div>
    </div>
</template>
    <!-- script -->
    <script>
        let editor = null,editorInterval = null;
        const { createEditor, createToolbar } = window.wangEditor;
		module.exports = {
			props: {
				id_v: {
					type: String,
					default: ''
				}
			},
            data: function () {
                return {
                    id: '',
                    info:[],
                    positionOne:[],
                    positionTwo:[],
					submitLoading:false,
                }
            },
			watch: {
				id_v: {
					handler(n) {
						if (n != '') {
							this.id = n.trim();
							this.getInfo();
						}
					},
					deep: true,
					immediate: true
				},
			},
            mounted() {
                this.initEditor();
                
            },
            beforeDestroy() {
                editor = null; 
                editorInterval = null;
            },
            methods: {
                initEditor: function (content=null) {
                    clearInterval(editorInterval);
                    editorInterval = setInterval(()=>{
                        if (editor !== null){
                            clearInterval(editorInterval);
                            if(content!==null){
                                editor.setHtml(content);
                            }
                        }else{
                            let editorConfig = {
                                MENU_CONF: {
                                    uploadImage: {
                                        server: baseUrl + 'm=index&c=uploadfile',
                                        fieldName: 'file'
                                    }
                                }
                            };
                            editor = createEditor({
                                selector: '#editor-container',
                                html: '',
                                config: editorConfig,
                                mode: 'simple'
                            });
                            
                            let toolbar = createToolbar({
                                editor,
                                selector: '#toolbar-container',
                                config: {
                                    excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                                },
                                mode: 'simple'
                            });
                        }
                    },50);
                    
                },
                save(){
                    let that = this;
                    let formData = new FormData();
                    if (that.info.title == '' ||  that.info.title == undefined) {
                        message.error('请填写问答标题！');
                        return false;
                    }
                    if (that.info.cid == '' ||  that.info.cid == undefined) {
                        message.error('请选择商品类别！');
                        return false;
                    }

                    formData.append('title', that.info.title);
                    formData.append('cid', that.info.cid);
                    formData.append('visit', that.info.visit);
                    formData.append('is_recom', that.info.is_recom?1:0);
                    let content = editor.getHtml();
                    formData.append('content', content);
                    if (that.id > 0) {
                        formData.append('id', that.id);
                    }
					that.submitLoading = true;
                    httpPost('m=yunying&c=report_ask&a=save', formData).then(function (res) {
                        if (res.data.error == 0) {
                            message.success(res.data.msg, function () {
                                that.$emit("child-event");
                            });
                        } else {
                            message.error(res.data.msg);
                        }
                    }).catch(function (error) {
                        console.log(error);
                    }).finally(function () {
						setTimeout(function () {
						    that.submitLoading = false;
						}, 2000);
					});
                },
                getInfo() {
                    let that = this;
                    let params = {};
                    if (this.id !='') {
                        params.id = this.id;
                    }
                    httpPost('m=yunying&c=report_ask&a=edit', params).then(function (response) {
                        let res = response.data,
							data = res.data;
                        if (res.error==0) {
                            that.info = data.info;
                            if (that.info.content !== '') {
                                that.initEditor(that.info.content);
                            }
                        }
                        that.positionOne = data.class;
						that.info.pid = data.pid ? data.pid : '';
						if (data.pid){
							that.getclass(data.pid)
						}
                    }).catch(function (error) {
                        console.log(error);
                    });
                },
                classchange(){
                    this.info.cid = '';
                    this.getclass(this.info.pid);
                },
                getclass:function(pid){
                    let that = this;
                    let params = {};
                    if (pid){
                        params.pid = pid;
                    }
                    httpPost('m=yunying&c=report_ask&a=getclass', params,{hideloading: true}).then(function (response) {
                        let res = response.data,
                            data = res.data;
                        that.positionTwo = data.class;
                    }).catch(function (error) {
                        console.log(error);
                    });
                },

            }
        };
    </script>
