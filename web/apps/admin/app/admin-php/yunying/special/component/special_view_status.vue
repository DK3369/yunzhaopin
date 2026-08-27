<template>
  <div style="padding: 10px 20px;">
    <div class="wxsettip_small">{{ lc('admin_user_weipin_00032') }}</div>
    <template>
      <el-radio v-model="ruleForm.status" label="1">{{ lc('member_user_00042') }}</el-radio>
      <el-radio v-model="ruleForm.status" label="2">{{ lc('wap_user_00167') }}</el-radio>
    </template>
    <div class="wxsettip_small">{{ lc('member_user_00062') }} </div>
    <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')" v-model="ruleForm.statusbody" style="margin-bottom: 10px;"></el-input>
    <span class="dialog-footer">
      <el-button @click="handleCancel">{{ lc('admin_user_weipin_00043') }}</el-button>
      <el-button type="primary" @click="submitForm('ruleForm')">{{ lc('wap_com_00019') }}</el-button>
    </span>
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
    pid: {type: String, default: ''},
  },
  data: function () {
    return {
      ruleForm: {
        pid: '',
        status: null,
        statusbody: '',
      },
    }
  },
  mounted() {
  },
  methods: {
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
    handleCancel(){
      this.$emit("child-event-close");
    }
  },
  watch: {
    pid: {
      handler: function (newValue, oldValue) {
        if (newValue) {
          this.ruleForm.pid = newValue;
        }
      },
      deep: true,
      immediate: true
    },
  }
};
</script>
<style scoped>

</style>