<template>
  <div class="jobspecial_box">
    <!--添加参会企业-->
    <div class="jobspecial_box" style="padding: 0;">
      <div class="moduleSeachbig">
        <!--关键字搜索和查询在一起-------------------------------------------------------------------->
        <div class="tableSeakusydsg" style="padding: 2px 0;">
          <el-input v-model="searchForm.keyword" placeholder="请输入搜索内容" class="input-with-select" size="small"
            prefix-icon="el-icon-search" clearable>
            <el-select slot="prepend" v-model="searchForm.type" size="small" placeholder="职位名称">
              <el-option label="企业名称/简称" value="1"></el-option>
              <el-option label="用户名称" value="2"></el-option>
              <el-option label="联系人" value="3"></el-option>
              <el-option label="联系电话" value="4"></el-option>
              <el-option label="用户邮箱" value="5"></el-option>
              <el-option label="用户ID" value="6"></el-option>
            </el-select>
          </el-input>
        </div>
        <!--收起部分-->
        <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
          <el-select v-model="searchForm.rating" size="small" slot="prepend" placeholder="会员等级" clearable @change="handleSearch">
            <el-option v-for="(item, index) in ratingList" :label="item.label" :value="item.value"
              :key="index"></el-option>
          </el-select>
        </div>
        <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
          <el-select v-model="searchForm.time" size="small" slot="prepend" placeholder="到期时间" clearable @change="handleSearch">
            <el-option v-for="(item, index) in timeList" :label="item.label" :value="item.value" :key="index"></el-option>
          </el-select>
        </div>
        <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
          <el-select v-model="searchForm.status" size="small" slot="prepend" placeholder="审核状态" clearable @change="handleSearch">
            <el-option v-for="(item, index) in statusList" :label="item.label" :value="item.value"
              :key="index"></el-option>
          </el-select>
        </div>
        <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
          <el-select v-model="searchForm.source" size="small" slot="prepend" placeholder="数据来源" clearable @change="handleSearch">
            <el-option v-for="(item, index) in sourceList" :label="item.label" :value="item.value"
              :key="index"></el-option>
          </el-select>
        </div>
        <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
          <el-select v-model="searchForm.rec" size="small" slot="prepend" placeholder="知名企业" clearable @change="handleSearch">
            <el-option v-for="(item, index) in recList" :label="item.label" :value="item.value" :key="index"></el-option>
          </el-select>
        </div>
        <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
          <el-select v-model="searchForm.gw" size="small" slot="prepend" placeholder="企业顾问" clearable @change="handleSearch">
            <el-option v-for="(item, index) in gwList" :label="item.label" :value="item.value" :key="index"></el-option>
          </el-select>
        </div>
        <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
          <el-select v-model="searchForm.lotime" size="small" slot="prepend" placeholder="最近登录" clearable @change="handleSearch">
            <el-option v-for="(item, index) in lotimeList" :label="item.label" :value="item.value"
              :key="index"></el-option>
          </el-select>
        </div>
        <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
          <el-select v-model="searchForm.adtime" size="small" slot="prepend" placeholder="最近注册" clearable @change="handleSearch">
            <el-option v-for="(item, index) in adtimeList" :label="item.label" :value="item.value"
              :key="index"></el-option>
          </el-select>
        </div>
        <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
          <el-select v-model="searchForm.job" size="small" slot="prepend" placeholder="职位状况" clearable @change="handleSearch">
            <el-option label="有职位" value="1"></el-option>
            <el-option label="无职位" value="2"></el-option>
          </el-select>
        </div>
        <div class="tableSeachInpt">
          <el-button type="primary" icon="el-icon-search" size="mini" @click="handleSearch">查询</el-button>
        </div>
        <div class="tableSeachzk" :class="{ 'searchbutnKai': seachbutn }" style="margin-bottom: 12px;">
          <el-button type="info" class="zhankai" @click="seachbutn = !seachbutn, tableHig = !tableHig"
            aria-disabled="false" size="mini" plain>展开<i class="el-icon-arrow-down el-icon--right"></i></el-button>
          <el-button type="info" class="shouqi" @click="seachbutn = !seachbutn, tableHig = !tableHig"
            aria-disabled="false" size="mini" plain>合并<i class="el-icon-arrow-up el-icon--right"></i></el-button>
        </div>
      </div>


      <div class="admin_datatip"><i class="el-icon-document"></i> 数据统计：企业总数 {{ totalNum }} 条
        <span class="admin_datatip_n">未加入：{{ noNum }} 条</span>
        <span class="admin_datatip_n">已加入：{{ applyNum }} 条 </span>
      </div>
      <div class="moduleElenAlRight" :class="{ 'moduleElenkuandu': seachbutn }">
        <div class="moduleElTable">
          <el-table :data="tableData" border style="width: 100%"
            :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" ref="multipleTable"
            @selection-change="handleSelectionChange" v-loading="loading">
            <template slot="empty">
              <p>{{ dataText }}</p>
            </template>
            <el-table-column type="selection" width="55" :selectable="selectable"></el-table-column>
            <el-table-column label="企业名称" min-width="180">
              <template slot-scope="scope">
                <el-popover trigger="hover" placement="top">
                  <p>{{ scope.row.name }}<span v-if="scope.row.shortname">【简称: {{ scope.row.shortname }}】</span></p>
                  <span slot="reference" class="name-wrapper">
                    <el-link type="primary" :href="scope.row.comUrl" target="_blank">{{ scope.row.name }}</el-link>
                  </span>
                </el-popover>
              </template>
            </el-table-column>
            <el-table-column prop="rating_name" label="会员等级"></el-table-column>
            <el-table-column label="联系电话">
              <template slot-scope="scope">
                <template v-if="scope.row.linktel">{{ scope.row.linktel }}</template>
                <template v-else-if="scope.row.linkphone">{{ scope.row.linkphone }}</template>
              </template>
            </el-table-column>
            <el-table-column label="业务员">
              <template slot-scope="scope">
                <span v-if="scope.row.crm_uid <= 0" style="color:#c1c1c1;">{{ scope.row.crm_uid_n }}</span>
                <span v-else-if="scope.row.crm_uid > 0">{{ scope.row.crm_uid_n }}</span>
              </template>
            </el-table-column>
            <el-table-column prop="zz_jobnum" label="在招职位数" width="150"></el-table-column>
            <el-table-column label="参与状态">
              <template slot-scope="scope">
                <span v-if="scope.row.join == '1'">已参加</span>
                <span v-else style="color: #c1c1c1;">未参加</span>
              </template>
            </el-table-column>
            <el-table-column fixed="right" label="操作" width="210" align="center">
              <template slot-scope="scope">
                <div class="cz_button">
                  <template v-if="scope.row.jobnum > 0 && scope.row.r_status == 1">
                    <el-button size="mini" @click="handleInsert(scope)">加入</el-button>
                  </template>
                  <template v-else>
                    <template v-if="scope.row.r_status != 1">企业未审核</template>
                    <template v-if="scope.row.jobnum <= 0">-</template>
                  </template>
                </div>
              </template>
            </el-table-column>
          </el-table>
        </div>
        <div class="modulePaging">
          <div>
            <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">全选</el-checkbox>
            <el-button @click="handleInsert(null, true)" size="mini">批量加入</el-button>
          </div>
          <div class="modulePagNum">
            <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
              :current-page="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes"
              layout="total, sizes, prev, pager, next, jumper" :total="total">
            </el-pagination>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
module.exports = {
  props: {
    //special_com.sid
    id: { type: [String, Number], default: 0 },
  },
  data: function () {
    return {
      loading: false,
      dataText: '数据加载中',
      pytoken: localStorage.getItem("pytoken"),
      searchForm: {
        page: 1,
        limit: null,
        type: '1',
        keyword: null,//关键字
        rating: null,//会员等级
        time: null,//到期时间
        status: null,//审核状态
        source: null,//数据来源
        rec: null,//知名企业
        gw: null,//企业顾问
        lotime: null,//最近登录
        adtime: null,//最近注册
        job: null,//职位状况
      },
      total: 0,
      tableData: [],
      pageSizes: [],
      checked: false,//全选
      isIndeterminate: false,// checkbox 的不确定状态
      selectedItem: [],
      //搜索条件的基本数据
      ratingList: [],//会员等级
      timeList: [],//到期时间
      statusList: [],//审核状态
      sourceList: [],//数据来源
      recList: [],//知名企业
      gwList: [],//企业顾问
      lotimeList: [],//最近登录
      adtimeList: [],//最近注册
      totalNum: 0,
      applyNum: 0,//参会企业的数量
      noNum: 0,//未加入
      statuscomVisible: false,//批量审核

      tableHig: 100,
      seachbutn: false,
        prevPage:0
    }
  },
  created: function () {
    this.getList();
    this.getBaseData();
  },
  methods: {
    selectable(row, index) {
      if (row.jobnum <= 0 || row.r_status != '1') {
        return false;
      } else {
        return true;
      }
    },
    handleSelectionChange(val) {
      this.selectedItem = val;
      if (this.selectedItem.length == 0) {
        this.isIndeterminate = false;
        this.checked = false;
      } else {
        if (this.selectedItem.length == this.tableData.length) {
          this.isIndeterminate = false;
          this.checked = true;
        } else {
          this.isIndeterminate = true;
          this.checked = false;
        }
      }
    },
    selectAllBottom(value) {
      value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
    },
    handleSizeChange(val) {
      this.searchForm.limit = val;
      this.getList();
    },
    handleCurrentChange(val) {
      this.searchForm.page = val;
      this.getList();
    },
    handleSearch() {
      this.searchForm.page = 1
      this.getList()
    },
    getList() {
      let _this = this;
      let params = JSON.parse(JSON.stringify(this.searchForm));
      params.id = this.id;
      for (let index in params) {
        (params[index] === '') && (params[index] = null);
      }
      _this.loading = true;
      httpPost('m=yunying&c=special_special&a=addlist', params).then(function (response) {
        let res = response.data;
        if (res.error === 0) {
          _this.tableData = res.data.list;
          _this.total = res.data.total;
          _this.searchForm.limit = res.data.perPage;
          _this.pageSizes = res.data.pageSizes;
          _this.totalNum = res.data.totalNum;
          _this.applyNum = res.data.applyNum;
          _this.noNum = res.data.noNum;
          if(_this.prevPage != _this.searchForm.page){
              _this.prevPage = _this.searchForm.page;
              _this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
          }
          _this.loading = false;
          if (_this.tableData.length === 0) {
            _this.dataText = "暂无数据";
          }
        }
      }).catch(function (error) {
        console.log(error);
      });
    },
    getBaseData() {
      let _this = this;
      httpPost('m=yunying&c=special_special&a=set_comaddsearch', {}, { hideloading: true }).then(function (response) {
        let res = response.data;
        if (res.error === 0) {
          _this.ratingList = res.data.ratingList;
          _this.timeList = res.data.timeList;
          _this.statusList = res.data.statusList;
          _this.sourceList = res.data.sourceList;
          _this.recList = res.data.recList;
          _this.gwList = res.data.gwList;
          _this.lotimeList = res.data.lotimeList;
          _this.adtimeList = res.data.adtimeList;
        }
      }).catch(function (error) {
        console.log(error);
      });
    },
    handleInsert(scope, isMore) {
      let params = {};
      if (isMore) {
        if (!this.selectedItem.length) {
          message.error('您还未选择参会企业！');
          return false;
        }
        let list = [];
        for (let item of this.selectedItem) {
          list.push(item.uid);
        }
        params.sid = this.id;
        params.uid = list.join(',');
        params.type = 'more';
      } else {
        // let index = scope.$index;
        // this.tableData.splice(index, 1);
        params.sid = this.id;
        params.uid = scope.row.uid;
        params.type = 'one';
      }

      delConfirm(this, params, this.insert, '确认添加至专题招聘？');
    },
    insert(params) {
      let _this = this;
      let url = params.type === 'one' ? 'm=yunying&c=special_special&a=savespecial' : 'm=yunying&c=special_special&a=mutiAddCom';
      httpPost(url, params).then(function (response) {
        let res = response.data;
        if (res.error === 0) {
          message.success(res.msg);
          _this.$emit("child-event");
        } else {
          message.error(res.msg);
        }
      }).catch(function (error) {
        console.log(error);
      });
    },
  },
  watch: {
    id: {
      handler: function (newValue, oldValue) {
      },
      deep: true,
      immediate: true
    },
  }
};
</script>
<style scoped>
.drawerModInfo::-webkit-scrollbar {
  display: none;
}

.moduleElenAlRight {
  overflow: hidden;
  position: relative;
  width: 100%;
  height: calc(100% - 130px);
}

.jobspecial_box {
  overflow: hidden;
  position: relative;
  width: 100%;
  height: 100%;
}

.moduleElTable {
  margin-top: 0;
  padding: 0;
  width: 100%;
  height: calc(100% - 55px);
}

.moduleElenkuand {
  overflow: hidden;
  position: relative;
  width: 100%;
  height: calc(100% - 5px) !important;
}</style>