<template>
  <div class="shbox">
    <div class="shinfo">
      <div class="shcomname">
        <template v-if="info.name">{{ info.name }}</template>
        <el-tag v-if="info.rating_name" type="danger" size="mini">{{ info.rating_name }}</el-tag>
      </div>
      <div class="sh_zwsz_add">
        <template v-if="info.linkman">联系人：{{ info.linkman }}{{ info.linkjob ? '（' + info.linkjob + '）' : '' }}</template>
        <span v-if="info.linktel" class="shcomtel_n">联系电话：{{ info.linktel }}{{ info.infostatus == '1' ? '（公开）' :
          '（不公开）' }}</span>
        <template v-if="info.crm_name">业务员：{{ info.crm_name }}</template>
      </div>
      <div class="shcomtel">
        <template v-if="info.reg_date_n">注册时间：{{ info.reg_date_n }}</template>
        <span v-if="info.login_date > 0" class="shcomtel_n">最近登录时间：{{ info.login_date_n }} </span>
        <template v-if="info.login_ip">IP：{{ info.login_ip }}</template>
      </div>
      <div class="shshowall">
        <div class="shshow" style="margin-top: 12px;">
          <el-tabs v-model="activeName" @tab-click="handleClick">
            <el-tab-pane label="基本资料" name="first">
              <div class="shshow_p">
                <div class="" v-if="info.welfare_n">企业福利：
                  <el-tag v-for="(item, index) in info.welfare_n" :key="index" size="mini"
                    class="welfare-margin">{{ item }}</el-tag>
                </div>
                <div class="">从事行业：{{ info.hy_n ? info.hy_n : '' }} </div>
                <div class="">企业性质：{{ info.pr_n ? info.pr_n : '' }} </div>
                <div class="">企业规模：{{ info.mun_n ? info.mun_n : '' }}</div>
                <div class="">注册资金：{{ info.money ? info.money : '' }} {{ info.money && info.moneytype_n ? info.moneytype_n : '' }}</div>
                <div class="">企业地址： {{ info.job_city_one ? info.job_city_one : '' }}&nbsp;{{ info.job_city_two ? info.job_city_two : '' }}&nbsp;{{ info.job_city_three ? info.job_city_three : '' }}&nbsp;{{ info.address ? info.address : '' }}</div>
                <div class="" v-html="info.content ? info.content : ''"></div>
              </div>
            </el-tab-pane>
            <el-tab-pane label="招聘岗位" name="second">
              <div>
                <ul class="shshow_joblist" v-if="total > 0">
                  <li v-for="(item, index) in tableData" :key="index">
                    <el-link type="primary" :underline="false">
                      <div class="shshow_job"><el-link :href="item.url" target="_blank" type="primary">{{item.name}}</el-link></div>
                    </el-link>
                    <div class="shshow_jobinfo"><span class="shshow_jobxz">{{item.job_salary}}</span> <span
                        class="shshow_line">|</span>
                      {{item.job_exp}}经验 <span class="shshow_line">|</span> {{item.job_edu}}学历
                    </div>
                  </li>
                </ul>
                <div v-if="total <= 0" class="firm_tips_no"> 该企业还没有发布职位信息!</div>
              </div>
              <div v-if="total > 0" class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                  :current-page.sync="searchForm.page" :page-sizes="pageSizes" layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
              </div>
            </el-tab-pane>
          </el-tabs>
        </div>
        <div class="shcz">
          <div class="wxsettip_small ">企业加入状态 </div>
          <template>
            <el-radio v-model="ruleForm.status" label="1">允许加入</el-radio>
            <el-radio v-model="ruleForm.status" label="2">拒绝加入</el-radio>
          </template>
          <div class="wxsettip_small ">状态说明 </div>
          <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="ruleForm.statusbody">
          </el-input>
          <div class=" shczbth">
            <el-button type="primary" @click="submitForm('ruleForm')">提 交</el-button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script>
module.exports = {
    props: {
        //special_com.id
        id: {type: Number | String, default: ''},
        uid: {type: Number | String, default: ''},
    },
    data: function () {
        return {
            info: {},
            activeName: 'second',
            ruleForm: {
                pid: this.id,
                status: '0',
                statusbody: '',
                single: 1,
            },
            pageSizes: [],
            searchForm: {
                page: 1,
                limit: null,
            },
            tableData: [],
            total: 0,
        }
    },
  mounted() {
  },
  methods: {
    getInfo() {
      let _this = this;
      let params = { id: this.id };
      httpPost('m=yunying&c=special_special&a=audit', params).then(function (response) {
        let res = response.data;
        if (res.error === 0) {
          _this.info = res.data;
          if (_this.info.hasOwnProperty('special')) {
            _this.ruleForm.status = _this.info.special.status;
            _this.ruleForm.statusbody = _this.info.special.statusbody;
          }
        } else {
          message.error('暂无数据');
        }
      }).catch(function (error) {
        console.log(error);
      });
    },
    submitForm(formName) {
      let _this = this;
      let params = JSON.parse(JSON.stringify(this.ruleForm));
      if (params.status != '1' && params.status != '2') {
        message.error('请选择审核状态！');
        return false;
      }

      httpPost('m=yunying&c=special_special&a=statuscom', params).then(function (response) {
        let res = response.data;
        if (res.error === 0) {
          message.success('操作成功！');
          _this.$emit("child-event");
        } else {
          message.error('操作失败！');
        }
      }).catch(function (error) {
        console.log(error);
      });
    },
    getList() {
      let _this = this;
      let params = JSON.parse(JSON.stringify(this.searchForm));
      params.uid = this.uid;
      for (let index in params) {
        (params[index] === '') && (params[index] = null);
      }
      httpPost('m=yunying&c=special_special&a=comjob', params, {hideloading: true}).then(function (response) {
        let res = response.data;
        if (res.error === 0) {
          _this.tableData = res.data.list;
          _this.total = res.data.total;
          _this.pageSizes = res.data.pageSizes;
        }
      }).catch(function (error) {
        console.log(error);
      });
    },
    handleClick(tab, event) {
      console.log(tab, event);
    },
    handleSizeChange(val) {

        this.searchForm.limit = val;
        this.getList();
      console.log(`每页 ${val} 条`);
    },
    handleCurrentChange(val) {
      console.log(`当前页: ${val}`);
    }
  },
  watch: {
    id: {
      handler: function (newValue, oldValue) {
        if (newValue) {
          this.getInfo();
        }
      },
      deep: true,
      immediate: true
    },
    uid: {
      handler: function (newValue, oldValue) {
        if (newValue) {
          this.getList();
        }
      },
      deep: true,
      immediate: true
    },
  }
};
</script>
<style scoped>
.shcomname {
  height: 25px;
}

.sh_zwsz_add {
  height: 20px;
}

.shcomtel {
  height: 20px;
}

.welfare-margin {
  margin-right: 4px;
}
.firm_tips_no{width:100%; text-align:center; padding:40px 0; font-size:16px;}
</style>