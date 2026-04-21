<template>
    <div class="drawerModlue">
        <!-- <div class="tableDome_tip">
            <el-alert :title="headTip" type="warning">
            </el-alert>
        </div> -->
        <div class="tableDome_tip tableDoAlert">
            <span>题目统计： 总数：{{ anum }}道 总分：{{ fullscore }}分</span>
        </div>
        <div class="drawerModInfo drawerModInfoOne">
            <div class="drawerModInpt" v-for="(item, index) in list" :key="index">
                <div class="pinyuGuanli">
                    <div class="pinyuGuFensu">
                        <div class="pinyuName">
                            <span>问题{{ index + 1 }}</span>
                        </div>
                        <div class="pinyuFroms">
                            <el-input type="textarea" :rows="2" placeholder="请输入问题" v-model="item.question">
                            </el-input>
                        </div>
                    </div>
                    <div class="pinyuFromfkieu" v-for="(op, k) in item.option" :key="k">
                        <div class="pinyuFromsList">
                            <div class="pinyuName">
                                <span>选项</span>
                            </div>
                            <div class="pinyuFroms">
                                <el-input placeholder="选项" v-model="item.option[k]">
                                </el-input>
                                <span class="spanfez">分值</span>
                                <el-input placeholder="分值" v-model="item.score[k]">
                                </el-input>
                                <el-button type="text" @click="delopt(index, k)">删除</el-button>
                            </div>
                        </div>
                    </div>
                    <el-button type="text" @click="addopt(index)">添加选项</el-button>
                </div>
                <div class="pinyuClose">
                    <el-button type="text" @click="delrow(index)">删除</el-button>
                </div>
            </div>
            <div class="drawerModLis" style="align-items: initial;">
                <div class="drawerModInpt">
                    <el-button type="primary" icon="el-icon-plus" plain size="medium" @click="addrow">添加测评题目</el-button>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save">确定</el-button>
        </div>
    </div>
</template>
<script>
module.exports = {
    props: {
        sjid: {
            type: String,
            default: ''
        },
		askdata: {
			type: Array,
			default: []
		}
    },
    data: function () {
        return {
            id: '',
            list: [],
            fullscore: 0,
            anum: 0,
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
		askdata: {
		    handler(val, oldVal) {
		        this.list = val;
				this.anum = this.list.length;
				this.fullscore = 0;
				this.list.forEach((item,key)=>{
					const max = Math.max.apply(null,this.list[key]['score'])
					this.fullscore += max;
				});
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
            this.list.push({ id: '', question: '', option: ['', ''], score: ['', ''] })
        },
        delrow(index) {
            this.$delete(this.list, index)
        },
        save() {
            var that = this
            var err = false
            that.list.forEach(item => {
                if (item.question == '') {
                    err = true
                }
                item.option.forEach(subitem => {
                    if (subitem == '') {
                        err = true
                    }
                });
                item.score.forEach(subitem => {
                    if (subitem == '') {
                        err = true
                    }
                });
            });
            if (err == true) {
				message.error('问题、选项、分值都不能为空');
                return false;
            }
			that.$parent.$parent.askdata = that.list;
			that.$parent.$parent.drawerquestion = false;
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