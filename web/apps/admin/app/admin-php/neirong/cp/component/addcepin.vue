<template>
    <div class="drawerModlue" v-if="islook">
        <!-- <div class="tableDome_tip">
            <el-alert :title="headTip" type="warning">
            </el-alert>
        </div> -->
        <div class="drawerModInfo drawerModInfoOne">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00113') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.name" :placeholder="lc('admin_00108')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00110') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="info.keyid" :placeholder="lc('admin_00857')">
                        <el-option v-for="group in group_all" :key="group.id" :label="group.name" :value="group.id">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00119') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-upload class="upload-demo" st :action="''" :auto-upload="false" :show-file-list="false" :accept="pic_accept"
                        :on-change="picChange">
                        <el-button size="small" type="primary" plain icon="el-icon-plus">{{ lc('wap_00540') }}</el-button>
                    </el-upload>
                    <img style="width: 208px; height: 167px;padding-left: 5px;" v-if="info.pic_n" :src="info.pic_n">
                </div>
                <div class="drawerModTips">
                    <el-alert :title="lc('admin_00107')" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00115') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="info.sort" :placeholder="lc('admin_00109')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00114') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-checkbox v-model="topcheck">{{ lc('admin_00111') }}</el-checkbox>
                    <el-checkbox v-model="hotcheck">{{ lc('admin_00120') }}</el-checkbox>
                    <el-checkbox v-model="reccheck">{{ lc('common.recommended') }}</el-checkbox>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00118') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" :placeholder="lc('wap_00936')" v-model="info.description">
                    </el-input>
                </div>
            </div>
            <div class="drawerModLis" style="align-items: initial;">
                <div class="drawerModTite"></div>
                <div class="drawerModInpt">
                    <el-button type="primary" icon="el-icon-plus" plain size="medium" @click="addrate">{{ lc('admin_00112') }}</el-button>
					<el-button type="primary" icon="el-icon-plus" plain size="medium" @click="addquestion">{{ lc('admin_00117') }}</el-button>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="submitLoading">{{ lc('common.submit') }}</el-button>
        </div>
		<div class="modluDrawer">
			<el-drawer :title="lc('admin_00112')" v-model="drawerrate" :append-to-body="true" :modal-append-to-body="false" :show-close="true"
			    :with-header="true" size="40%">
			    <addrate ref="addrate" :sjid="id" :ratedata="ratedata"></addrate>
			</el-drawer>
		    <el-drawer :title="lc('admin_00116')" v-model="drawerquestion" :append-to-body="true" :modal-append-to-body="false" :show-close="true"
		        :with-header="true" size="42%">
		        <addquestion ref="addask" :sjid="id" :askdata="askdata"></addquestion>
		    </el-drawer>
		</div>
		 
    </div>
</template>
<script>
import Addrate from './addrate.vue'
import Addquestion from './addquestion.vue'

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
	    'addrate': Addrate,
	    'addquestion': Addquestion,
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
				message.error(lc('admin_00108'));
                return false;
            }
            if (!that.info.keyid) {
				message.error(lc('admin_00857'));
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