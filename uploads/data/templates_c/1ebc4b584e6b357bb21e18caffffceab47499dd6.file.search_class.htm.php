<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:34:38
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/publichtm/search_class.htm" */ ?>
<?php /*%%SmartyHeaderCode:129595589169e8962e7ec9f2-73337490%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '1ebc4b584e6b357bb21e18caffffceab47499dd6' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/publichtm/search_class.htm',
      1 => 1700725936,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '129595589169e8962e7ec9f2-73337490',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8962e7ed235_65207297',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8962e7ed235_65207297')) {function content_69e8962e7ed235_65207297($_smarty_tpl) {?><div class="zwfl" v-if="classShow">
    <div class="zwfl_top classShowNew">
        <div class="goBlacks">
            <a href="javascript:;" @click="classShow = false"><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/return.png"></a>
        </div>
        <div class="zwfl_inputbox">
            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/searchicon.png" class="ss_img" alt="">
            <input class="zwfl_input" type="text" confirm-type="done" :value="searchClassVal" placeholder="搜索岗位" @input="bindSearch" />
            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/input_close.png" class="input_closeimg" alt="" @click="clearSearch" v-show="searchClassVal!=''">
        </div>
    </div>
    <div class="zwfl_content">

        <div class="flex_row" v-if="!searchClassShow">
            <div class="nav_left">
                <div class="zwfl_cont_left">
                    <div class="zwfl_left_itembox">
                        <template v-if="category == 'job'" v-for="(jv, jk) in jobIndexArr" :key="jk">
                            <div class="zwfl_left_item" :class="job1==jv ? 'left_active' : ''" @click="checkClassOne(jv);">
                                <div class="left_active_line" v-if="job1==jv"></div>
                                {{jobNameArr[jv]}}
                            </div>
                        </template>
                        <template v-if="category == 'city'" v-for="(cv, ck) in cityIndexArr" :key="ck">
                            <div class="zwfl_left_item" :class="provinceid==cv ? 'left_active' : ''" @click="checkClassOne(cv);">
                                <div class="left_active_line" v-if="provinceid==cv"></div>
                                {{cityNameArr[cv]}}
                            </div>
                        </template>
                    </div>
                </div>
            </div>
            <div class="nav_right">
                <div class="zwfl_cont_right">
                    <div class="zwfl_right_itembox">
                        <template v-if="category == 'job'" v-for="(j2_v, j2_k) in jobTypeArr[job1]" :key="j2_k">
                            <template v-if="jobTypeArr[j2_v]">
                                <div class="zwfl_right_item">
                                    <div class="zwfl_right_item_tit">
                                        {{jobNameArr[j2_v]}}
                                    </div>
                                    <div class="zwfl_right_item_tagbox">
                                        <template v-for="(j3_v, j3_k) in jobTypeArr[j2_v]" :key="j3_k">
                                        <div class="zwfl_right_item_tag" @click="setSearchJobClass(job1, j2_v, j3_v);">
                                            {{jobNameArr[j3_v]}}
                                        </div>
                                        </template>
                                    </div>
                                </div>
                            </template>
                            <template v-else>
                                <div class="zwfl_right_item">
                                    <div class="zwfl_right_item_tagbox">
                                        <div class="zwfl_right_item_tag" @click="setSearchJobClass(job1, j2_v);">
                                            {{jobNameArr[j2_v]}}
                                        </div>
                                    </div>
                                </div>
                            </template>
                        </template>
                        <template v-if="category == 'city'" v-for="(c2_v, c2_k) in cityTypeArr[provinceid]" :key="c2_k">
                            <template v-if="cityTypeArr[c2_v]">
                                <div class="zwfl_right_item">
                                    <div class="zwfl_right_item_tit">
                                        {{cityNameArr[c2_v]}}
                                    </div>
                                    <div class="zwfl_right_item_tagbox">
                                        <template v-for="(c3_v, c3_k) in cityTypeArr[c2_v]" :key="c3_k">
                                            <div class="zwfl_right_item_tag" @click="setSearchJobClass(provinceid, c2_v, c3_v);">
                                                {{cityNameArr[c3_v]}}
                                            </div>
                                        </template>
                                    </div>
                                </div>
                            </template>
                            <template v-else>
                                <div class="zwfl_right_item">
                                    <div class="zwfl_right_item_tagbox">
                                        <div class="zwfl_right_item_tag" @click="setSearchJobClass(provinceid, c2_v);">
                                            {{cityNameArr[c2_v]}}
                                        </div>
                                    </div>
                                </div>
                            </template>
                        </template>
                    </div>
                </div>
            </div>
        </div>

        <!-- 搜索列表 -->
        <div class="search_list" v-if="searchClassShow">
            <div class="search_list_itembox">
                <template v-for="(item, sk) in searchClassList" :key="sk">
                <div class="search_list_item" @click="setSearchJobClass(item.one, item.two, item.value);">
                    <div class="search_list_item_name">
                        {{item.name}}
                    </div>
                    <div class="search_list_item_info">
                        {{item.fname}}
                    </div>
                </div>
                </template>
            </div>
        </div>
    </div>
</div><?php }} ?>
