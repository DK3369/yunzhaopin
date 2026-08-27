<template>
    <div class="drawerModlue">
        <!-- <div class="tableDome_tip">
            <el-alert :title="headTip" type="warning">
            </el-alert>
        </div> -->
        <!-- <div class="tableDome_tip tableDoAlert">
            <span>{{ lc("admin_question_stats", [anum, fullscore]) }}</span>
        </div> -->
        <div class="drawerModInfo drawerModInfoOne">
            <div class="drawerModInpt" v-for="(item, index) in list" :key="index">
				<div class="pinyuGuanli">
					<div class="pinyuGuFensu">
						<div class="pinyuName">
							<span>{{ lc('admin_00863') }}</span>
						</div>
						<div class="pinyuFroms">
							<el-input v-model="item.from" :placeholder="lc('admin_00866')"></el-input>
							<span class="spantite">{{ lc('admin_vue_00136') }}</span>
							<el-input v-model="item.to" :placeholder="lc('admin_00866')"></el-input>
						</div>
					</div>
					<div class="pinyuGuFensu">
						<div class="pinyuName">
							<span>{{ lc('admin_00864') }}</span>
						</div>
						<div class="pinyuFroms">
							<el-input type="textarea" :rows="2" :placeholder="lc('admin_00867')" v-model="item.content">
							</el-input>
						</div>
					</div>
				</div>
				<div class="pinyuClose">
					<el-button type="text" @click="delrow(index)">{{ lc('common.delete') }}</el-button>
				</div>
			
			</div>
            <div class="drawerModLis" style="align-items: initial;">
                <div class="drawerModInpt">
                    <el-button type="primary" icon="el-icon-plus" plain size="medium" @click="addrow">{{ lc('admin_00865') }}</el-button>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save">{{ lc('common.confirm') }}</el-button>
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
    props: {
        sjid: {
            type: String,
            default: ''
        },
		ratedata: {
			type: Array,
			default: []
		}
    },
    data: function () {
        return {
            id: '',
            list: [],
            fullscore: 0,
        }
    },
    watch: {
        sjid: {
            handler(val, oldVal) {
                this.id = val;
            },
            immediate: true,
            deep: true,
        },
		ratedata: {
			handler(val, oldVal) {
			    this.list = val;
			},
			immediate: true,
			deep: true,
		}
    },
    mounted() {

    },
    methods: {
        addopt(index) {
            var that = this
            that.list[index].option.push('')
            that.list[index].score.push('')
        },
        delopt(index, k) {
            var that = this
            that.$delete(that.list[index].option, k)
            that.$delete(that.list[index].score, k)
        },
        addrow() {
            this.list.push({ from: '', to: '', content: '' })
        },
        delrow(index) {
            this.$delete(this.list, index)
        },
        save() {
			var that = this
			var err = false
			that.list.forEach(item => {
				if (item.from == '' || item.to == '' || item.content == '') {
					err = true
				}
			});
			if (err == true) {
				message.error(lc('admin_vue_00123'));
				return false;
			}
			that.$parent.$parent.ratedata = that.list;
            that.$parent.$parent.drawerrate = false
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

.drawerModInpt {
    width: 100%;
    padding-left: 0;
    margin: 10px 0;
}

.pinyuFromfkieu {
    overflow: hidden;
    position: relative;
    width: 100%;
}

.pinyuFromsList {
    overflow: hidden;
    position: relative;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-bottom: 10px;
}

.pinyuFromsList .spanfez {
    overflow: hidden;
    display: block;
    font-size: 14px;
    color: #333;
}

.pinyuFromsList .el-input {
    overflow: hidden;
    position: relative;
    width: calc((100% - (50px + 50px + 38px)) / 2);
}</style>