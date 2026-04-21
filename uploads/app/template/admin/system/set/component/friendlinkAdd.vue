<template>
    <div class="tableDome" style="top: 40px;">
        <div class="tableDome_tip">
            <el-alert title="添加链接时，请正确选择链接类型！" type="success" :closable="false">
            </el-alert>
        </div>
        <div class="moduleTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="140">名称</th>
                    <th width="450">状态</th>
                    <th width="200">说明</th>
                </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">链接类型</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="infoData.linktype" placeholder="请选择">
                                    <el-option label="文字链接" value="1"></el-option>
                                    <el-option label="图片链接" value="2"></el-option>
                                </el-select>
                            </div>
                        </td>
                        <td></td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">链接标题</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="infoData.linkname"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>例：phpyun</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">链接地址</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="infoData.linkurl"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>例：http://www.phpyun.com</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">显示站点</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="infoData.did" size="small" slot="prepend" placeholder="请选择">
                                    <el-option v-for="item in domainData" :key="item.value" :label="item.label" :value="item.value"></el-option>
                                </el-select>
                            </div>
                        </td>
                        <td></td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">使用范围</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="infoData.didtype" placeholder="请选择">
                                    <el-option label="全站使用" value="1"></el-option>
                                    <el-option label="仅在首页使用" value="2"></el-option>
                                </el-select>
                            </div>
                        </td>
                        <td></td>
                    </tr>
                    <tr v-show="infoData.linktype == 2">
                        <td>
                            <div class="TableTite">缩 略 图</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-radio-group v-model="infoData.imgtype">
                                    <el-radio label="1">上传图片</el-radio>
                                    <el-radio label="2">远程图片</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>尺寸：160*50px</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="infoData.imgtype ==1 && infoData.linktype == 2">
                        <td></td>
                        <td>
                            <div class="TableUpload">
                                <el-upload
                                  class="avatar-uploader"
                                  :auto-upload="false"
                                  action=""
                                  :accept="pic_accept"
                                  :show-file-list="false"
                                  :on-change="uploadChange"
                                >
                                  <img v-if="infoData.imgurl" :src="infoData.imgurl" class="avatar" width="160" height="50">
                                  <el-button size="small" type="primary">重新上传</el-button>
                                </el-upload>
                            </div>
                        </td>
                        <td >
                           
                        </td>
                    </tr>
                    <tr v-show="infoData.imgtype ==2 && infoData.linktype == 2">
                        <td></td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="infoData.picurl"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>例：http://www.hr135.com/yun.jpg</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">排序</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="infoData.linksort"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>例：大前小后</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" :loading="save_load" @click="save">{{btnTis}}</el-button>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props: {
        link_id: Number
    },
    watch: {
        link_id: {
            handler(newValue, oldValue) {
                if (newValue > 0) {
                    this.btnTis = '修 改';
                    this.getInfo();
                }else{
                    this.btnTis = '添 加';
                    this.infoData = {
                        linktype: '',
                        linkname: '',
                        linkurl: '',
                        did: 0,
                        didtype: '',
                        imgtype: '',
                        picurl: '',
                        imgurl: '',
                        linksort: '',
                    }
                }
            },
            deep: true,
            immediate: true
        },
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            infoData: {
                linktype: '',
                linkname: '',
                linkurl: '',
                did: 0,
                didtype: '',
                imgtype: '',
                picurl: '',
                imgurl: '',
                linksort: '',
            },
            domainData: {},
            files: [],
            btnTis: '添 加',
			save_load:false,
        }
    },
    created(){
        this.getCache();
    },
    methods: {
        async getInfo(){
            let that = this;
            that.loading = true;
            httpPost('m=system&c=set_friendlink&a=getInfo', {id: this.link_id},{hideloading: true}).then(function (response) {
                let data = response.data;
                if (data.error == 0) {
                     that.infoData.linktype = data.data.link_type;   
                     that.infoData.linkname = data.data.link_name;   
                     that.infoData.linkurl = data.data.link_url;   
                     that.infoData.did = '' + data.data.did;
                     that.infoData.didtype = data.data.tem_type;   
                     that.infoData.imgtype = data.data.img_type;   
                     that.infoData.imgurl = data.data.pic_n;   
                     that.infoData.picurl = data.data.pic;   
                     that.infoData.linksort = data.data.link_sorting;   
                }
                that.loading = false;
            }).catch(function (error) {
                console.log(error);
            })
        },
        async getCache(){
			httpPost('m=system&c=set_friendlink&a=getCache', {}).then((response)=>{
			    let res = response.data;
			    if (res.error == 0) {
			        this.domainData = res.data.domain
			    }
			}).catch(function (error) {
			    console.log(error);
			})
            
        },
        async save(){
            let that = this;
            let formData = new FormData();
            if (that.infoData.linktype == ''){
                message.error('请选择链接类型');
                return false;
            }
            if (that.infoData.linkname == '') {
                message.error('请填写链接标题');
                return false;
            }
            if (that.infoData.linkurl == '') {
                message.error('请填写链接地址');
                return false;
            }
            if (that.infoData.didtype == '') {
                message.error('请选择站点下使用范围');
                return false;
            }
            formData.append('type', that.infoData.linktype);
            formData.append('title', that.infoData.linkname);
            formData.append('url', that.infoData.linkurl);
            formData.append('tem_type', that.infoData.didtype);
            formData.append('did', that.infoData.did);
            formData.append('phototype', that.infoData.imgtype);
            formData.append('sorting', that.infoData.linksort);
            formData.append('uplocadpic', that.infoData.imgurl);
            if (that.files.length !== 0) {
                formData.append('file', that.files);
            }
            if (that.link_id > 0 ) {
                formData.append('id', that.link_id);
            }
			that.save_load = true;
            httpPost('m=system&c=set_friendlink&a=save', formData).then(function (res) {
				that.save_load = false;
                if (res.data.error == 0) {
                    message.success(res.data.msg, function () {
                        that.$emit("child-event");
                    });
                } else {
                    message.error(res.data.msg);
                }
            });
        },
        uploadChange(file) {
            this.infoData.imgurl = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.files = file.raw;
        },
    },
};
</script>
<style scoped>
    
    .avatar-uploader .el-upload {
    border: 1px dashed #d9d9d9;
    border-radius: 6px;
    cursor: pointer;
    position: relative;
    overflow: hidden;
  }
  .avatar-uploader .el-upload:hover {
    border-color: #409EFF;
  }
  .avatar-uploader-icon {
    font-size: 28px;
    color: #8c939d;
    width: 148px;
    height: 148px;
    line-height: 148px;
    text-align: center;
  }
  .avatar {
    width: 148px;
    height: 148px;
    display: block;
  }
</style>