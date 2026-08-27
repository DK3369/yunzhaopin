<template>
    <div>
        <div class="homeTopRigDuax">
            <div class="homeTopRiTite">
                <span>{{ lc('admin_00094') }}</span>
            </div>
            <div class="homeTopRigDumao">
                <div class="homeTopumaod">
                    <span class="spannum">{{numdata.msgnum}}</span>
                    <!-- <b>剩余短信数量</b> -->
                    <el-button type="primary" size="small" @click="tomsg">{{ lc('wap_user_00172') }}</el-button>
                </div>
            </div>
        </div>
        
        <div class="indexLetSixan">
            <div class="homeTopRiTite">
                <span>{{ lc('admin_00093') }}</span>
            </div>
            <div class="indexLetshicConts">
                <el-carousel :interval="5000" arrow="always" :autoplay="false" v-if="msgNumData.length>0">
                    <el-carousel-item v-for="(item,key) in msgNumData" :key="key">
                        <div class="indexLetshItem">
                            <div class="indexLetshList" v-for="(val,index) in item" :key="index" @click="toMsgPath(val)">
                                <span class="spannaen">{{val.name}}</span>
                                <b class="nubsuz">{{val.num}}</b>
                            </div>
                        </div>
                    </el-carousel-item>
                </el-carousel>
				<!-- 无内容显示部分 -->
				<div class="indexLetEmpty" v-else>
				    <el-empty :description="lc('admin_00092')"></el-empty>
				</div>
            </div>
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

    data: function() {
        return {
            numdata: {},
            timer: null,
            msgNumData: [],
        }
    },

    mounted() {
        this.getMsgnum();
        this.getData();
		
		
		window.addEventListener("message",(e)=>{
			if(e.data.type=='msgnum' && e.data.data){
				this.getMsgnum();
			}
		})
    },
    methods: {
        format(percentage) {
            let tex = lc('admin_00095')
            return tex + percentage + '%'
        },
        getData: function() {
            var that = this;
            httpPost('m=index&c=ajax_right', {},{hideloading: true}).then(function(response) {
                let res = response.data;
                if (res.error == 0) {
                    that.numdata = res.data;

                }
            })
        },
        getMsgnum: function() {
            var that = this;
            if (that.timer !== null) {
                clearTimeout(this.timer);
            }

            if (window.parent.homeapp.msgNumLoad) {
                that.timer = setTimeout(() => {
                    this.getMsgnum();
                }, 50)
            } else {
                var msgNumData = window.parent.homeapp.msgNumData;
                msgNumData.sort((a, b) => { return parseInt(b.num) - parseInt(a.num) });

                let newList = [];

                for (var i = 0; i < msgNumData.length; i += 6) {
                    newList.push(msgNumData.slice(i, i + 6));
                }
                that.msgNumData = newList;

            }
		},
        toMsgPath: function(item) {
            window.parent.homeapp.checkMenuTwo(item.menudata.nval, item.menudata.oval, item.menudata.tval, item.menudata.name, item.menudata.path, item.menudata.query)
        },
        tomsg: function() {
            window.open('https://u.ov6.com/');
        },

    },
};
</script>
<style scoped></style>