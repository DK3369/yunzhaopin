<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:35:40
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/member/public/resume_city_job.htm" */ ?>
<?php /*%%SmartyHeaderCode:28130481069e8966c37e0f2-68211927%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'cce493a0cd825083eddf1c85b0acc514e2b94ac7' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/member/public/resume_city_job.htm',
      1 => 1708477840,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '28130481069e8966c37e0f2-68211927',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
    'job_type' => 0,
    'job_index' => 0,
    'v' => 0,
    'job_name' => 0,
    'key' => 0,
    'vv' => 0,
    'twok' => 0,
    'city_type' => 0,
    'city_index' => 0,
    'city_name' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8966c391dd9_18533392',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8966c391dd9_18533392')) {function content_69e8966c391dd9_18533392($_smarty_tpl) {?><!-- 公共多职位、多城市弹窗-->
<div id="cityjobVue" style="display: none;">
    <input type="hidden" value="5" id="jobnum" />
    <!--工作职能-->
    <van-popup v-model="jobShow" position="right" :style="{ height: '100%',width:'100%',backgroundColor:'#f2f6f9'}">
        <div class="yzxz_header">
            <!-- 页面头部 -->
            <div class="workplace_header">
                <div class="workplace_header_left" @click="jobShow = false">
                    <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/nav_icon_return.png" alt="" width="100%" height="100%">
                </div>
                <div class="workplace_header_center">{{jobheader?jobheader:'意向岗位选择'}}</div>
                <div class="workplace_header_right"></div>
            </div>
            <div id="jobClassBox">
                <div class="zn_search">
                    <div class="zn_search_text_c">
                        <div class="zn_search_text">
                            <input id="jclass_svalue" type="text" value="" data-type='jobclass' placeholder="搜索意向岗位" class="zn_search_t inputListener">
                        </div>
                    </div>
                </div>
                <div id="jobclass_search" class="zn_searchbox " style="display:none;">
                    <div id="jobclass_searhtml" class="mui-input-row   s_checkbox  classTap"> </div>
                </div>
                <?php if (empty($_smarty_tpl->tpl_vars['job_type']->value)) {?>
                <div class="grade-eject grade-w-roll">
                    <ul class="grade-w" id="jobone">
                        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['job_index']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                        <li class="jobone" data-id="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
">
                            <div class="mui-input-row mui-checkbox">
                                <label for="jobone<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
"><?php echo $_smarty_tpl->tpl_vars['job_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</label>
                                <input class="jobonebox jobcheck<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" id="jobone<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" type="checkbox" />
                            </div>
                        </li>
                        <?php } ?>
                    </ul>
                </div>
                <?php } else { ?>
                <div class="grade-eject grade-w-roll">
                    <ul class="grade-w" id="jobone" style="height:77%;">
                        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['job_index']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                        <li data-id="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
"><?php echo $_smarty_tpl->tpl_vars['job_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</li>
                        <?php } ?>
                    </ul>
                    <ul class="grade-t" id="jobtwo">
                        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['job_type']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                        <?php if (in_array($_smarty_tpl->tpl_vars['key']->value,$_smarty_tpl->tpl_vars['job_index']->value)) {?>
                        <?php  $_smarty_tpl->tpl_vars['vv'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['vv']->_loop = false;
 $_smarty_tpl->tpl_vars['twok'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['v']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['vv']->key => $_smarty_tpl->tpl_vars['vv']->value) {
$_smarty_tpl->tpl_vars['vv']->_loop = true;
 $_smarty_tpl->tpl_vars['twok']->value = $_smarty_tpl->tpl_vars['vv']->key;
?>
                        <?php if (!empty($_smarty_tpl->tpl_vars['job_type']->value[$_smarty_tpl->tpl_vars['vv']->value])) {?>
                        <li class="jobtwo job<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
 none" data-id="<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
"><?php echo $_smarty_tpl->tpl_vars['job_name']->value[$_smarty_tpl->tpl_vars['vv']->value];?>
</li>
                        <?php } else { ?>
                        <li class="jobtwo job<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
 none" data-id="<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
">
                            <div class="mui-input-row mui-checkbox two_check" style="display: flex; ">
                                <label style="padding-top: 2px;"><?php echo $_smarty_tpl->tpl_vars['job_name']->value[$_smarty_tpl->tpl_vars['vv']->value];?>
</label>
                                <input class="two_check_ipt jobtwobox jobcheck<?php echo $_smarty_tpl->tpl_vars['twok']->value;?>
" id="jobtwo<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" value="<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" type="checkbox" style="width: 100%;" />
                            </div>
                        </li>
                        <?php }?>
                        <?php } ?>
                        <?php }?>
                        <?php } ?>
                    </ul>
                    <ul class="grade-s citythreeHinght" id="jobthree">
                        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['job_type']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                        <?php if (in_array($_smarty_tpl->tpl_vars['key']->value,$_smarty_tpl->tpl_vars['job_index']->value)) {?>
                        <?php  $_smarty_tpl->tpl_vars['vv'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['vv']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['v']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['vv']->key => $_smarty_tpl->tpl_vars['vv']->value) {
$_smarty_tpl->tpl_vars['vv']->_loop = true;
?>
                        <?php if (!empty($_smarty_tpl->tpl_vars['job_type']->value[$_smarty_tpl->tpl_vars['vv']->value])) {?>
                        <li id="jobtwo<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" class="jobthree job<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
 none">
                            <div class="mui-input-row mui-checkbox citythree_pd">
                                <label class="label_box" for="jobcheckAll<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
">全部</label>
                                <input type="checkbox" value="<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" id="jobcheckAll<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" class="checkAll" />
                            </div>
                        </li>
                        <?php }?>
                        <?php } ?>
                        <?php }?>
                        <?php } ?>
                        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['job_type']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                        <?php if (!in_array($_smarty_tpl->tpl_vars['key']->value,$_smarty_tpl->tpl_vars['job_index']->value)) {?>
                        <?php  $_smarty_tpl->tpl_vars['vv'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['vv']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['v']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['vv']->key => $_smarty_tpl->tpl_vars['vv']->value) {
$_smarty_tpl->tpl_vars['vv']->_loop = true;
?>
                        <li class="jobthree job<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
 none">
                            <div class="mui-input-row mui-checkbox citythree_pd">
                                <label for="jobthree<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
"><?php echo $_smarty_tpl->tpl_vars['job_name']->value[$_smarty_tpl->tpl_vars['vv']->value];?>
</label>
                                <input class="jobthreebox jobcheck<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
" id="jobthree<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" value="<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" type="checkbox" />
                            </div>
                        </li>
                        <?php } ?>
                        <?php }?>
                        <?php } ?>
                    </ul>
                </div>
                <?php }?>
                <div class="grade_chlose_box">
                    <div class="grade_chlose_box_c">
                        <div id="jobchoosed" class="mui-scroll mui-scrollpouy">
                            <a class="grade_chlose_box_a" v-for="(job, jk) in jobnameArr" :key="jk" :data-id="jk">{{job}}</a>
                        </div>
                    </div>
                    <a class="grade_chlose_bth" @click="jobShow = false">确定 <span id="jobpencent" class="none">1/5</span></a>
                </div>
            </div>
        </div>
    </van-popup>
    <!--城市类别-->
    <van-popup v-model="cityShow" position="right" :style="{ height: '100%',width:'100%',backgroundColor:'#f3f3f3'}">
        <div class="yzxz_header">
            <!-- 页面头部 -->
            <div class="workplace_header">
                <div class="workplace_header_left" @click="cityShow = false">
                    <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/nav_icon_return.png" alt="" width="100%" height="100%">
                </div>
                <div class="workplace_header_center">工作城市选择</div>
                <div class="workplace_header_right"></div>
            </div>
            <div id="cityClassBox">
                <div class="zn_search">
                    <div class="zn_search_text_c">
                        <div class="zn_search_text">
                            <input id="cclass_svalue" type="text" value="" data-type='cityclass' placeholder="请输入城市类别" class="zn_search_t inputListener">
                        </div>
                    </div>
                </div>
                <div id="cityclass_search" class="zn_searchbox " style="display:none;">
                    <div id="cityclass_searhtml" class="mui-input-row s_checkbox classTap">
                    </div>
                </div>
                <div class="grade_tit">选择工作城市(可多选)</div>
                <?php if (empty($_smarty_tpl->tpl_vars['city_type']->value)) {?>
                <div class="grade-eject grade-w-roll">
                    <ul class="grade-w" id="cityone">
                        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['city_index']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                        <li class="cityone" data-id="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
">
                            <div class="mui-input-row mui-checkbox">
                                <label><?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</label>
                                <input class="cityonebox citycheck<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" id="cityone<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" type="checkbox" style="width:10px" />
                            </div>
                        </li>
                        <?php } ?>
                    </ul>
                </div>
                <?php } else { ?>
                <div class="grade-eject grade-w-roll">
                    <ul class="grade-w" id="cityone" style="height:74%;">
                        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['city_index']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                        <li class="cityone" data-id="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
">
                            <div class="mui-input-row mui-checkbox">
                                <label><?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</label>
                                <input class="cityonebox citycheck<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" id="cityone<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" type="checkbox" style="width:10px" />
                            </div>
                        </li>
                        <?php } ?>
                    </ul>
                    <ul class="grade-t" id="citytwo" style="left: 30.48%; height: 74%;">
                        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['city_type']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                        <?php if (in_array($_smarty_tpl->tpl_vars['key']->value,$_smarty_tpl->tpl_vars['city_index']->value)) {?>
                        <?php  $_smarty_tpl->tpl_vars['vv'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['vv']->_loop = false;
 $_smarty_tpl->tpl_vars['twok'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['v']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['vv']->key => $_smarty_tpl->tpl_vars['vv']->value) {
$_smarty_tpl->tpl_vars['vv']->_loop = true;
 $_smarty_tpl->tpl_vars['twok']->value = $_smarty_tpl->tpl_vars['vv']->key;
?>
                        <?php if (!empty($_smarty_tpl->tpl_vars['city_type']->value[$_smarty_tpl->tpl_vars['vv']->value])) {?>
                        <li class="citytwo city<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
 none" data-id="<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
"><?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['vv']->value];?>
</li>
                        <?php } else { ?>
                        <li class="citytwo city<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
 none" data-id="<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
">
                            <div class="mui-input-row mui-checkbox"><label><?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['vv']->value];?>
</label><input class="citytwobox citycheck<?php echo $_smarty_tpl->tpl_vars['twok']->value;?>
" id="citytwo<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" value="<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" type="checkbox" style="width:150px" /></div>
                        </li>
                        <?php }?>
                        <?php } ?>
                        <?php }?>
                        <?php } ?>
                    </ul>
                    <ul class="grade-s citythreeHinght" id="citythree">
                        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['city_type']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                        <?php if (in_array($_smarty_tpl->tpl_vars['key']->value,$_smarty_tpl->tpl_vars['city_index']->value)) {?>
                        <?php  $_smarty_tpl->tpl_vars['vv'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['vv']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['v']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['vv']->key => $_smarty_tpl->tpl_vars['vv']->value) {
$_smarty_tpl->tpl_vars['vv']->_loop = true;
?>
                        <?php if (!empty($_smarty_tpl->tpl_vars['city_type']->value[$_smarty_tpl->tpl_vars['vv']->value])) {?>
                        <li id="citytwo<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" class="citythree city<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
 none">
                            <div class="mui-input-row mui-checkbox citythree_pd"><label for="citycheckAll<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
">全部</label><input type="checkbox" value="<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" id="citycheckAll<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" class="checkAll" /></div>
                        </li>
                        <?php }?>
                        <?php } ?>
                        <?php }?>
                        <?php } ?>
                        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['city_type']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                        <?php if (!in_array($_smarty_tpl->tpl_vars['key']->value,$_smarty_tpl->tpl_vars['city_index']->value)) {?>
                        <?php  $_smarty_tpl->tpl_vars['vv'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['vv']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['v']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['vv']->key => $_smarty_tpl->tpl_vars['vv']->value) {
$_smarty_tpl->tpl_vars['vv']->_loop = true;
?>
                        <li class="citythree city<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
 none">
                            <div class="mui-input-row mui-checkbox citythree_pd"><label for="citythree<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
"><?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['vv']->value];?>
</label><input class="citythreebox citycheck<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
" id="citythree<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" value="<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
" type="checkbox" /></div>
                        </li>
                        <?php } ?>
                        <?php }?>
                        <?php } ?>
                    </ul>
                </div>
                <?php }?>
                <div class="grade_chlose_box">
                    <div class="grade_chlose_box_c">
                        <div class="mui-slider">
                            <div class="mui-scroll-wrapper mui-slider-indicator mui-segmented-control mui-segmented-control-inverted" style="background:#fff; ">
                                <div id="citychoosed" class="mui-scroll" style="background:#fff; ">
                                    <a class="grade_chlose_box_a" v-for="(city, ck) in citynameArr" :key="ck" :data-id='ck'>{{city}}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                    <a class="grade_chlose_bth" @click="cityShow = false">确定<span id="citypencent" class="none">1/5</span></a>
                </div>
            </div>
        </div>
    </van-popup>
</div>
<?php echo '<script'; ?>
>
var cityjobVue = new Vue({
    el: '#cityjobVue',
    data: {
        jobShow: false,
        cityShow: false,

        citynameArr: [],
        jobnameArr: [],
        jobheader:''
    },
    mounted() {
        document.getElementById('cityjobVue').style.display = 'block';
    }
});
<?php echo '</script'; ?>
><?php }} ?>
