<template>
    <div class="drawerModlue" v-if="islook">
        <!-- <div class="tableDome_tip">
            <el-alert :title="headTip" type="warning">
            </el-alert>
        </div> -->
        <div class="drawerModInfo drawerModInfoOne">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>试卷名称</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.name" placeholder="请输入试卷名称"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>试卷类别选择</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="info.keyid" placeholder="请选择试卷类别">
                        <el-option v-for="group in group_all" :key="group.id" :label="group.name" :value="group.id">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>缩略图</span>
                </div>
                <div class="drawerModInpt">
                    <el-upload class="upload-demo" st :action="''" :auto-upload="false" :show-file-list="false" :accept="pic_accept"
                        :on-change="picChange">
                        <el-button size="small" type="primary" plain icon="el-icon-plus">上传图片</el-button>
                    </el-upload>
                    <img style="width: 208px; height: 167px;padding-left: 5px;" v-if="info.pic_n" :src="info.pic_n">
                </div>
                <div class="drawerModTips">
                    <el-alert title="尺寸：220*140px" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>试卷排序</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.sort" placeholder="请输入试卷排序"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>试卷属性</span>
                </div>
                <div class="drawerModInpt">
                    <el-checkbox v-model="topcheck">首页幻灯片</el-checkbox>
                    <el-checkbox v-model="hotcheck">头条</el-checkbox>
                    <el-checkbox v-model="reccheck">推荐</el-checkbox>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>描　　述</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" placeholder="请输入描述" v-model="info.description">
                    </el-input>
                </div>
            </div>
            <div class="drawerModLis" style="align-items: initial;">
                <div class="drawerModTite"></div>
                <div class="drawerModInpt">
                    <el-button type="primary" icon="el-icon-plus" plain size="medium" @click="addrate">评语管理</el-button>
					<el-button type="primary" icon="el-icon-plus" plain size="medium" @click="addquestion">题目管理</el-button>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="submitLoading">提交</el-button>
        </div>
		<div class="modluDrawer">
			<el-drawer title="评语管理" :visible.sync="drawerrate" :append-to-body="true" :modal-append-to-body="false" :show-close="true"
			    :with-header="true" size="40%">
			    <addrate ref="addrate" :sjid="id" :ratedata="ratedata"></addrate>
			</el-drawer>
		    <el-drawer title="试题管理" :visible.sync="drawerquestion" :append-to-body="true" :modal-append-to-body="false" :show-close="true"
		        :with-header="true" size="42%">
		        <addquestion ref="addask" :sjid="id" :askdata="askdata"></addquestion>
		    </el-drawer>
		</div>
		 
    </div>
</template>
<script>
module.exports = {
    props: {
        currid: {
            type: String,
            default: ''
        },
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            id: '',
            info: {},
            
            fullscore: 0,
            group_all: [],
            
            topcheck: false,
            hotcheck: false,
            reccheck: false,
            
            piclist: [],
            islook: false,
			submitLoading:false,
			
			askdata: [],
			ratedata: [],
			drawerquestion: false,
			drawerrate: false,
        }
    },
	
    watch: {
        currid: {
            handler(val, oldVal) {
                this.id = val;
            },
            immediate: true,
            deep: true,
        }
    },
	components: {
	    'addrate': httpVueLoader('./addrate.vue'),
	    'addquestion': httpVueLoader('./addquestion.vue'),
	},
    mounted() {

    },
    methods: {
        addrate() {
            this.drawerrate = true;
        },
		addquestion() {
			this.drawerquestion = true;
		},
        picChange(file) {
            var tmp = deepClone(this.info);
            // 预览文件处理
            tmp.pic_n = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.piclist[0] = file.raw;
            this.info = tmp
        },
        save() {
            var that = this
            var params = new FormData();
            if (!that.info.name) {
				message.error('请输入试卷名称');
                return false;
            }
            if (!that.info.keyid) {
				message.error('请选择试卷类别');
                return false;
            }
            that.info.top = that.topcheck == true ? '1' : '0'
            that.info.hot = that.hotcheck == true ? '1' : '0'
            that.info.recommend = that.reccheck == true ? '1' : '0'
            that.info.pj_arr = that.ratedata;
            that.info.ask_arr = that.askdata;
			
            params.append('name',  that.info.name);
            params.append('top',  that.info.top);
            params.append('hot',  that.info.hot);
            params.append('recommend',  that.info.recommend);
            params.append('pj_arr',  JSON.stringify(that.info.pj_arr));
            params.append('ask_arr',  JSON.stringify(that.info.ask_arr));
            params.append('keyid',  that.info.keyid);
            params.append('sort',  that.info.sort);
            params.append('description',  that.info.description);
            params.append('id',  that.info.id);
            if (that.piclist.length) {
                params.append('pic[]', this.piclist[0])
            }
			that.submitLoading = true;
            httpPost('m=neirong&c=evaluate&a=add', params).then(function (response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function () {
                        that.$parent.$parent.getList();
                        that.$parent.$parent.draweradd = false;
                        that.$parent.$parent.sjid = response.data.data.nid
                    });
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
				that.submitLoading = false;
			});
        },
        getInfo() {
            let that = this;
            httpPost('m=neirong&c=evaluate&a=add', { id: that.id ,add:1}).then(function (response) {
                let data = response.data.data;
                if (data.info) {
                    that.info = data.info
                } else {
                    that.info = { id: '', name: '' }
                }
                // if (that.info.pic_n) {
                //     that.fileList[0].url = that.info.pic_n
                // }
                that.topcheck = that.info.top == '1'
                that.hotcheck = that.info.hot == '1'
                that.reccheck = that.info.recommend == '1'

                that.fullscore = data.fullscore ? data.fullscore : 0
                
                that.askdata = data.ask ? data.ask : []
                that.group_all = data.group_all ? data.group_all : []
                if (data.info) {
                    that.ratedata = data.info.pj_arr ? data.info.pj_arr : [{ from: '', to: '', content: '' }]
                } else {
                    that.ratedata = [{ from: '', to: '', content: '' }]
                }
                that.islook = true
            })
        },
    },
};
</script>
<style scoped>
.drawerModInfo::-webkit-scrollbar {
    display: none;
}

.el-dialog-s {
    z-index: 11;
}
.drawerMoFlexd{
    overflow: hidden;
    position: relative;
    width: calc(100% - 180px);
}
.drawerMoFlexd .drawerModInpt{
    overflow: hidden;
    position: relative;
    width: 100%;
    margin: 10px 0;
    padding-left: 0;
}
</style>