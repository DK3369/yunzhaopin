<template>
  <div class="shbox">
    <div class="shinfo">
      <div class="shcomname">
        <template v-if="info.name">{{ info.name }}</template>
        <el-tag v-if="info.rating_name" type="danger" size="mini">{{ info.rating_name }}</el-tag>
      </div>
      <div class="sh_zwsz_add">
        <template v-if="info.linkman">{{ lc("admin_contact_person_value", [info.linkman]) }}{{ info.linkjob ? '（' + info.linkjob + '）' : '' }}</template>
        <span v-if="info.linktel" class="shcomtel_n">{{ lc('admin_01221') }}</span>
        <template v-if="info.crm_name">{{ lc("admin_salesperson_value", [info.crm_name]) }}</template>
      </div>
      <div class="shcomtel">
        <template v-if="info.reg_date_n">{{ lc('admin_01222') }}</template>
        <span v-if="info.login_date > 0" class="shcomtel_n">{{ lc('admin_00450') }}：{{ info.login_date_n }} </span>
        <template v-if="info.login_ip">IP：{{ info.login_ip }}</template>
      </div>
      <div class="shshowall">
        <div class="shshow" style="margin-top: 12px;">
          <el-tabs v-model="activeName" @tab-click="handleClick">
            <el-tab-pane :label="lc('wap_user_00341')" name="first">
              <div class="shshow_p">
                <div class="" v-if="info.welfare_n">{{ lc('admin_00644') }}
                  <el-tag v-for="(item, index) in info.welfare_n" :key="index" size="mini"
                    class="welfare-margin">{{ item }}</el-tag>
                </div>
                <div class="">{{ lc("admin_industry_value", [info.hy_n ? info.hy_n : '']) }} </div>
                <div class="">{{ lc("admin_company_nature_value", [info.pr_n ? info.pr_n : '']) }} </div>
                <div class="">{{ lc("admin_company_size_value", [info.mun_n ? info.mun_n : '']) }}</div>
                <div class="">{{ lc('admin_01223') }}</div>
                <div class="">{{ lc('wap_com_00158') }}： {{ info.job_city_one ? info.job_city_one : '' }}&nbsp;{{ info.job_city_two ? info.job_city_two : '' }}&nbsp;{{ info.job_city_three ? info.job_city_three : '' }}&nbsp;{{ info.address ? info.address : '' }}</div>
                <div class="" v-html="info.content ? info.content : ''"></div>
              </div>
            </el-tab-pane>
            <el-tab-pane :label="lc('wap_01536')" name="second">
              <div>
                <ul class="shshow_joblist" v-if="total > 0">
                  <li v-for="(item, index) in tableData" :key="index">
                    <el-link type="primary" :underline="false">
                      <div class="shshow_job"><el-link :href="item.url" target="_blank" type="primary">{{item.name}}</el-link></div>
                    </el-link>
                    <div class="shshow_jobinfo"><span class="shshow_jobxz">{{item.job_salary}}</span> <span
                        class="shshow_line">|</span>
                      {{ lc('admin_01224') }} <span class="shshow_line">|</span> {{ lc('admin_01225') }}
                    </div>
                  </li>
                </ul>
                <div v-if="total <= 0" class="firm_tips_no"> {{ lc('wap_00027') }}</div>
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
          <div class="wxsettip_small ">{{ lc('admin_01226') }} </div>
          <template>
            <el-radio v-model="ruleForm.status" label="1">{{ lc('admin_01227') }}</el-radio>
            <el-radio v-model="ruleForm.status" label="2">{{ lc('admin_01228') }}</el-radio>
          </template>
          <div class="wxsettip_small ">{{ lc('admin_01229') }} </div>
          <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')" v-model="ruleForm.statusbody">
          </el-input>
          <div class=" shczbth">
            <el-button type="primary" @click="submitForm('ruleForm')">{{ lc('member_com_00248') }}</el-button>
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
          message.error(lc('wap_js_00113'));
        }
      }).catch(function (error) {
        console.log(error);
      });
    },
    submitForm(formName) {
      let _this = this;
      let params = JSON.parse(JSON.stringify(this.ruleForm));
      if (params.status != '1' && params.status != '2') {
        message.error(lc('admin_01311'));
        return false;
      }

      httpPost('m=yunying&c=special_special&a=statuscom', params).then(function (response) {
        let res = response.data;
        if (res.error === 0) {
          message.success(lc('wap_js_00159'));
          _this.$emit("child-event");
        } else {
          message.error(lc('model_00003'));
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
      console.log(`Page size: ${val}`);
    },
    handleCurrentChange(val) {
      console.log(`Current page: ${val}`);
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