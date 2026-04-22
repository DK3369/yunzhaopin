<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:20:08
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/hr.htm" */ ?>
<?php /*%%SmartyHeaderCode:199266048569e8a0d8755a44-62659266%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '854216435ef560c134204f806b535897e0c8e261' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/hr.htm',
      1 => 1706496289,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '199266048569e8a0d8755a44-62659266',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'one' => 0,
    'StateList' => 0,
    'current' => 0,
    'JobList' => 0,
    'v' => 0,
    'now_url' => 0,
    'rows' => 0,
    'freenum' => 0,
    'statis' => 0,
    'pagenav' => 0,
    'comdata' => 0,
    'comclass_name' => 0,
    'config' => 0,
    'resumestate' => 0,
    'rstate' => 0,
    'userdata' => 0,
    'userclass_name' => 0,
    'user_sex' => 0,
    'key' => 0,
    'uptime' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0d877fb04_59620020',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0d877fb04_59620020')) {function content_69e8a0d877fb04_59620020($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
if (!is_callable('smarty_function_Url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<div class="w1000">
    <div class="admin_mainbody">
        <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/left.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

        
    <div class="clear"></div>
     <div class='right_box'>
                <div class="newmember_tit">
                <ul>
                 <li class="newmember_titcur"><a href="index.php?c=hr" title="看过我的职位" class="   ">应聘简历</a></li>
                <li><a href="index.php?c=down" title="关注我的简历" class=" ">我的下载</a></li>
                <li> <a href="index.php?c=look_job" title="看过我的职位" class=" ">谁看过我</a></li>
                 <li> <a href="index.php?c=attention_me" title="关注我的简历" class=" ">对我感兴趣</a></li>
                <li><a href="index.php?c=look_resume" title="您浏览简历的记录" class=" ">我看过的简历</a></li>
                <li><a href="index.php?c=talent_pool" title="加入人才库的简历" class=" ">我的收藏</a></li>
                 
                 </ul>
                </div>
                
            <div class="newmember_screenbox">
            <div class="newmember_screen">  
            <ul>
                <li <?php if ($_GET['state']==$_smarty_tpl->tpl_vars['one']->value['id']) {?>class="job_list_tit_cur" <?php }?>>
                    <a href="index.php?c=hr">全部</a>
                </li>
                <?php  $_smarty_tpl->tpl_vars['one'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['one']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['StateList']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['one']->key => $_smarty_tpl->tpl_vars['one']->value) {
$_smarty_tpl->tpl_vars['one']->_loop = true;
?>
                <li <?php if ($_GET['state']==$_smarty_tpl->tpl_vars['one']->value['id']) {?>class="job_list_tit_cur" <?php }?>>
                    <a href="index.php?c=hr&state=<?php echo $_smarty_tpl->tpl_vars['one']->value['id'];?>
"><?php echo $_smarty_tpl->tpl_vars['one']->value['name'];?>
(<?php echo $_smarty_tpl->tpl_vars['one']->value['num'];?>
)</a>
                </li>
                <?php } ?>
            </ul>   
        
            <div class="joblist_search">
                <form class='layui-form' action="index.php" method='get' id='MyForm'>
                    <div class="joblist_search_box">
                        <input type="text" placeholder="在当前条件下筛选" class="joblist_search_box_text" name='keyword' value='<?php echo $_GET['keyword'];?>
'/>
                        <input type="submit" value="" class="joblist_search_bth"/>
                    </div>
                    <input name='c' value='hr' type='hidden'/>
                    <input name='rstate' id="rstate" value='' type='hidden'/>
                    <input name='jobid' id="jobid" value='' type='hidden'/>
                    <input type="hidden" id="resumetype" name="resumetype" value=""/>
                    <input type="hidden" id="exp" name="exp" value=""/>
                    <input type="hidden" id="edu" name="edu" value=""/>
                    <input type="hidden" id="sex" name="sex" value=""/>
                    <input type="hidden" id="uptime" name="uptime" value=""/>
                    <a id="joblist_search_more" href="javascript:void(0);" onclick="$('#jlsx').show();" class="joblist_search_more">更多筛选</a>
                </form>
            </div>
            
            <div class="ypjob" id="yp_jobname"  onclick="yp_jobshow();">
                <div class="ypjob_name"><?php if ($_smarty_tpl->tpl_vars['current']->value['name']) {
echo $_smarty_tpl->tpl_vars['current']->value['name'];
} else { ?>应聘职位筛选<?php }?></div>
                <div id="yp_joblist" class="ypjob_box" style="display: none;">
                    <a href="javascript:void(0);" onclick="sxjob('',this);">全部</a>
                    <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['JobList']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                    <a href="javascript:void(0);" onclick="sxjob('<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
',this);"><?php echo $_smarty_tpl->tpl_vars['v']->value['name'];?>
</a>
                    <?php } ?>
                </div>
            </div>
             </div> 
             </div>
         

       
            <div class='admincont_box'>
                <div class='com_body'>
                    <div class="admin_textbox_04">

                        <div class="clear"></div>

                        <!-- 筛选部分 -->
                        <!-- 收到简历列表 -->
                        <iframe id="supportiframe" name="supportiframe" onload="returnmessage('supportiframe');" style="display:none"></iframe>
                        <form action='<?php echo $_smarty_tpl->tpl_vars['now_url']->value;?>
&act=hrset' target="supportiframe" method="post" id='myform' class='layui-form'>
                            <table class="com_table">
                                <?php if ($_smarty_tpl->tpl_vars['rows']->value) {?>
                                <tr>
                                    <th style="border-radius:6px 0 0 6px ;">基本信息</th>
                                    <th>投递职位</th>
                                    <th>联系电话</th>
                                    <th>状态</th>
                                    <th>查看简历</th>
                                    <th style="border-radius:0 6px 6px 0 ;" width="212">操作</th>
                                </tr>
                                <?php }?>
                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['rows']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                                <tr>
                                    <td>
                                        <div class="newcom_user_info">
                                            <span class="newcom_user_infoheckb">
                                                <input type="checkbox" name="delid[]" value="<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
" class="newcom_user_infoheck" lay-skin="primary"/>
                                            </span>
                                            <div class="newcom_user_pic"><img src="<?php echo $_smarty_tpl->tpl_vars['v']->value['photo'];?>
"></div>
                                            <div>
                                                <?php if ($_smarty_tpl->tpl_vars['v']->value['state']==1) {?>
                                                    <a href="javascript:void(0)" onclick="com_lookresume_check('<?php echo $_smarty_tpl->tpl_vars['v']->value['eid'];?>
','<?php echo $_smarty_tpl->tpl_vars['v']->value['resume_status'];?>
','<?php echo $_smarty_tpl->tpl_vars['v']->value['state'];?>
')" class="newcom_user_name"><?php echo $_smarty_tpl->tpl_vars['v']->value['username_n'];?>
</a>
                                                    <!--<a href="javascript:void(0);" onclick="rstateTip('<?php echo $_smarty_tpl->tpl_vars['v']->value['state'];?>
');" class="newcom_user_name"><?php echo $_smarty_tpl->tpl_vars['v']->value['name'];?>
</a>-->
                                                <?php }?>
                                                <?php if ($_smarty_tpl->tpl_vars['v']->value['state_n']) {?>
                                                    <span class="newcom_user_zt"><?php echo $_smarty_tpl->tpl_vars['v']->value['state_n'];?>
</span>
                                                <?php }?>
                                                <?php if ($_smarty_tpl->tpl_vars['v']->value['userid_msg']==1) {?><span class="hr_yyy">已邀约</span><?php }?>
                                                <?php if ($_smarty_tpl->tpl_vars['v']->value['islink']==1) {?><span class="hr_yxz">已下载</span><?php }?>
                                            </div>

                                            <div class="newcom_user_infop">
                                                <?php if ($_smarty_tpl->tpl_vars['v']->value['sex']) {
echo $_smarty_tpl->tpl_vars['v']->value['sex'];
} else { ?>保密<?php }?> · <?php if ($_smarty_tpl->tpl_vars['v']->value['exp']) {
echo $_smarty_tpl->tpl_vars['v']->value['exp'];?>
经验 · <?php }
if ($_smarty_tpl->tpl_vars['v']->value['edu']) {
echo $_smarty_tpl->tpl_vars['v']->value['edu'];?>
学历 · <?php }
echo $_smarty_tpl->tpl_vars['v']->value['age'];?>
岁
                                            </div>
                                            <div>期望薪资：<?php echo $_smarty_tpl->tpl_vars['v']->value['salary'];?>
</div>
                                        </div>
                                    </td>
                                    <td>
                                        <div>
                                            投递了<a href="<?php if ($_smarty_tpl->tpl_vars['v']->value['type']==1) {
echo smarty_function_url(array('m'=>'job','c'=>'comapply','id'=>$_smarty_tpl->tpl_vars['v']->value['job_id']),$_smarty_tpl);
} elseif ($_smarty_tpl->tpl_vars['v']->value['type']==2) {
echo smarty_function_url(array('m'=>'lietou','c'=>'jobcomshow','id'=>$_smarty_tpl->tpl_vars['v']->value['job_id']),$_smarty_tpl);
}?>" target="_blank" class="newcom_user_td"><?php echo $_smarty_tpl->tpl_vars['v']->value['job_name'];?>
</a>
                                        </div>
                                        <div class="com_received_tdtime">投递时间 <?php echo $_smarty_tpl->tpl_vars['v']->value['datetime_n'];?>
</div>
                                    </td>

                                    <td align="center">
                                        <div>
                                            <?php if ($_smarty_tpl->tpl_vars['v']->value['state']==1) {?>
                                                <?php if ($_smarty_tpl->tpl_vars['v']->value['islink']==1) {?>
                                                    <?php if ($_smarty_tpl->tpl_vars['v']->value['is_browse']==1) {?>
                                                    <!--状态是未查看的，不展示联系方式，需要先查看简历-->
                                                    <a href="javascript:void(0)" onclick="com_lookresume('<?php echo $_smarty_tpl->tpl_vars['v']->value['eid'];?>
')" class="newcom_user_tel">查看联系方式</a>
                                                    <?php } else { ?>
                                                    <a class="newcom_user_tel"><?php echo $_smarty_tpl->tpl_vars['v']->value['telphone'];?>
</a>
                                                    <?php }?>
                                                <?php } elseif ($_smarty_tpl->tpl_vars['freenum']->value) {?>
                                                    <a class="newcom_user_tel" href="javascript:void(0);" onclick="downResume('<?php echo $_smarty_tpl->tpl_vars['v']->value['eid'];?>
','<?php echo smarty_function_Url(array('m'=>'ajax','c'=>'for_link'),$_smarty_tpl);?>
')">查看联系方式</a>   
                                                <?php } elseif ($_smarty_tpl->tpl_vars['statis']->value['down_resume']>0) {?>
                                                    <a class="newcom_user_tel" href="javascript:void(0);" onclick="isDownResume('<?php echo $_smarty_tpl->tpl_vars['v']->value['eid'];?>
','<?php echo smarty_function_Url(array('m'=>'ajax','c'=>'for_link'),$_smarty_tpl);?>
', '<?php echo $_smarty_tpl->tpl_vars['statis']->value['down_resume'];?>
')">查看联系方式</a>
                                                <?php } else { ?>
                                                    <a class="newcom_user_tel" href="javascript:void(0);" onclick="downResume('<?php echo $_smarty_tpl->tpl_vars['v']->value['eid'];?>
','<?php echo smarty_function_Url(array('m'=>'ajax','c'=>'for_link'),$_smarty_tpl);?>
')">查看联系方式</a>
                                                <?php }?>
                                            <?php } else { ?>
                                                <a class="newcom_user_tel" href="javascript:void(0);" onclick="rstateTip('<?php echo $_smarty_tpl->tpl_vars['v']->value['state'];?>
');">查看联系方式</a>
                                            <?php }?>
                                        </div>
                                    </td>

                                    <td align="center">
                                        <div>
                                            <?php if ($_smarty_tpl->tpl_vars['v']->value['is_browse']=='1') {?>

                                                <?php if ($_smarty_tpl->tpl_vars['v']->value['body']!='') {?>
                                                    <span class="com_received_zt com_received_zt_bhs"><i class="com_received_zt_icon "></i>取消申请</span>
                                                    <span onclick="lookreason('<?php echo $_smarty_tpl->tpl_vars['v']->value['body'];?>
')" style=" display:block; padding-top:5px;color:#4a89e8; text-decoration:underline; cursor:pointer">查看原因</span>
                                                <?php } else { ?>
                                                    <span class="com_received_zt com_received_zt_dcl"><i class="com_received_zt_icon"></i>未查看</span>
                                                <?php }?>
                                            <?php } elseif ($_smarty_tpl->tpl_vars['v']->value['is_browse']=='2') {?>

                                                <span class="com_received_zt com_received_zt_yck"><i class="com_received_zt_icon"></i>已查看</span>
                                            <?php } elseif ($_smarty_tpl->tpl_vars['v']->value['is_browse']=='3') {?>

                                                <span class="com_received_zt com_received_zt_dtz"><i class="com_received_zt_icon"></i>已面试</span>
                                            <?php } elseif ($_smarty_tpl->tpl_vars['v']->value['is_browse']=='4') {?>

                                                <span class="com_received_zt com_received_zt_bhs"><i class="com_received_zt_icon"></i>不合适</span>
                                            <?php } elseif ($_smarty_tpl->tpl_vars['v']->value['is_browse']=='5') {?>

                                                <span class="com_received_zt com_received_zt_bhw"><i class="com_received_zt_icon"></i>未接通</span>
                                            <?php } elseif ($_smarty_tpl->tpl_vars['v']->value['is_browse']=='7') {?>

                                                <span class="com_received_zt com_received_zt_wjt"><i class="com_received_zt_icon"></i>已入职</span>
                                            <?php }?>
                                        </div>

                                    </td>
                                    <td align="center">
                                        <a href="javascript:void(0)" style="background: #3d7dfd; color: #fff;" onclick="com_lookresume_check('<?php echo $_smarty_tpl->tpl_vars['v']->value['eid'];?>
','<?php echo $_smarty_tpl->tpl_vars['v']->value['resume_status'];?>
')" class=" com_bth cblue">查看简历</a>
                                    </td>
                                    <td align="center">
                                        <div class="">
                                            <div class="com_received_username_bjbox bj_<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
" onmouseover='bjAddClass(<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
);' onmouseout='bjRemoveClass(<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
);'>
                                                <span class="com_received_username_bj" style="cursor:pointer;">标记</span>
                                                <div class="com_received_username_bjbox_show">
                                                    <!--<a href="javascript:void(0);" onclick='changeBrowse(1,<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
)' class="com_received_username_bjbox_show_a"><i class="com_received_username_dclicon"></i>未查看</a>-->
                                                    <a href="javascript:void(0);" onclick='changeBrowse(2,<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
)' class="com_received_username_bjbox_show_a"><i class="com_received_username_yckicon"></i>已查看</a>
                                                    <a href="javascript:void(0);" onclick='changeBrowse(3,<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
)' class="com_received_username_bjbox_show_a"><i class="com_received_username_dtzicon"></i>已面试</a>
                                                    <a href="javascript:void(0);" onclick='changeBrowse(4,<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
)' class="com_received_username_bjbox_show_a"><i class="com_received_username_bhgicon"></i>不合适</a>
                                                    <a href="javascript:void(0);" onclick='changeBrowse(7,<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
)' class="com_received_username_bjbox_show_a"><i class="com_received_username_wjticon"></i>已入职</a>
                                                </div>
                                            </div>
                                            <div class="com_received_username_bjbox ">
                                                <a href="javascript:;" class="com_received_username_bj cblue" onclick="remark('<?php echo $_smarty_tpl->tpl_vars['v']->value['uid'];?>
','<?php echo $_smarty_tpl->tpl_vars['v']->value['eid'];?>
');">备注</a>
                                            </div>
                                            <div class="com_received_username_bjbox ">
                                                <a href="javascript:void(0);" onclick="layer_del('确定要删除该条职位申请吗？', 'index.php?c=hr&act=hrset&delid=<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
');" class="com_received_username_bj">删除</a>
                                            </div>
                                        </div>
                                    </td>
                                </tr>
                                <?php if ($_smarty_tpl->tpl_vars['v']->value['remark']) {?>
                                <tr><td colspan="9" style="background-color: #f5faff; "><div style="padding-left: 20px;;">↑ 备注状态：<?php echo $_smarty_tpl->tpl_vars['v']->value['status_n'];?>
&nbsp;&nbsp;&nbsp;&nbsp;备注说明：<?php echo $_smarty_tpl->tpl_vars['v']->value['remark'];?>
</div></td></tr>
                                <?php }?>

                                <?php } ?>

                                <?php if ($_smarty_tpl->tpl_vars['rows']->value) {?>
                                <tr>
                                    <td colspan="6">
                                        <div class="newcom_user_info" style="width: 500px; padding-left: 30px;float: left;">
                                            <span class="newcom_user_infoheckb" style="top:5px;"><input type="checkbox" lay-filter='allcom' lay-skin="primary"/></span>
                                            全选
                                            <input class="c_btn_02" type="button" name="subdel" value="批量删除" onclick="return really('delid[]');">
                                            <input class="c_btn_02" type="button" name="subdel" value="批量标记" onclick="return changeBrowseAll('1','delid[]');" style="margin-left: 10px;">
                                        </div>
                                        <div class="diggg"><?php echo $_smarty_tpl->tpl_vars['pagenav']->value;?>
</div>
                                    </td>
                                </tr>
                                <?php } else { ?>
                                <tr>
                                    <td colspan="7" class="table_end">
                                        <div class="msg_no">
                                            <p class="com_msg_no_name">人才库空空如也~</p>
                                            <p>快去主动出击挑选心仪人才</p>
                                            <a href="<?php echo smarty_function_url(array('m'=>'resume'),$_smarty_tpl);?>
" class="com_msg_no_bth com_submit">我要主动找人才</a>
                                        </div>
                                    </td>
                                </tr>
                                <?php }?>
                            </table>
                        </form>

                        <div class="clear"></div>

<!--                    -->
                        <div class="infoboxp22" id="remarkbox" style="display:none; ">
                            <div>
                                <form action="index.php?c=hr&act=remark" method="post" id="formstatus2"
                                      target="supportiframe" class="layui-form">
                                    <input name="uid" value="0" type="hidden">
                                    <input name="eid" value="0" type="hidden">
                                    <div class="newbz_list"><span class="newbz_list_name">当前状态：</span> <div class="layui-input-inline">
                                                <select name="status" id="status" lay-filter="status">
                                                    <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_remark']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                                                    <option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
"><?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
                                                    <?php } ?>
                                                </select>
                                            </div></div>
                                    <div class="newbz_list"><span class="newbz_list_name">备注说明：</span> <div class="newbz_textarea">
                                        <textarea id="remark"  name="remark" cols="30" rows="9" class="hr_textarea"></textarea>
                                    </div></div>
                                    <div class="newbz_list"><div class="newbz_bthbox">
                                        <button type="submit" name='submit' value='1' class="newbz_bth"  >确认</button>
                                        </div>
                                    </div>
                                 
                                </form>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

    </div>
</div>


<!--筛选弹出-->
<div id="jlsx" style="display: none;">

    <div id="jlsx_box" class="jlsx_box">
        <div class="jlsx_box_tit">筛选 <a href="javascript:void(0);" onclick="$('#jlsx').hide();" class="jlsx_box_tit_c"></a></div>
        <div class="jlsx_boxallpd">
            <div class="jlsx_boxall">
                <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_shresume_applyjob']==1) {?>
                <div class="jlsx_boxname">简历状态</div>
                <div class="jlsx_box_p">
                    <a href="javascript:void(0);" onclick="sxset('rstate','',this);" class="rstateCtrl jlsx_boxjy <?php if (!isset($_GET['rstate'])||$_GET['rstate']=='') {?>jlsx_boxjy_cur<?php }?>">全部</a>
                    <?php  $_smarty_tpl->tpl_vars['rstate'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['rstate']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['resumestate']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['rstate']->key => $_smarty_tpl->tpl_vars['rstate']->value) {
$_smarty_tpl->tpl_vars['rstate']->_loop = true;
?>
                    <a href="javascript:void(0);" onclick="sxset('rstate','<?php echo $_smarty_tpl->tpl_vars['rstate']->value['val'];?>
',this);" class="rstateCtrl jlsx_boxjy <?php if (isset($_GET['rstate'])&&$_GET['rstate']!=''&&$_GET['rstate']==$_smarty_tpl->tpl_vars['rstate']->value['val']) {?>jlsx_boxjy_cur<?php }?>"><?php echo $_smarty_tpl->tpl_vars['rstate']->value['name'];?>
</a>
                    <?php } ?>
                </div>
                <?php }?>
                <div class="jlsx_boxname">经验要求</div>
                <div class="jlsx_box_p">
                    <a href="javascript:void(0);" onclick="sxset('exp','',this);" class="expCtrl jlsx_boxjy <?php if (!$_GET['exp']) {?>jlsx_boxjy_cur<?php }?>">全部</a>
                    <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['userdata']->value['user_word']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                    <a href="javascript:void(0);" onclick="sxset('exp','<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
',this);" class="expCtrl jlsx_boxjy <?php if ($_GET['exp']==$_smarty_tpl->tpl_vars['v']->value) {?>jlsx_boxjy_cur<?php }?>"><?php echo $_smarty_tpl->tpl_vars['userclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a>
                    <?php } ?>
                </div>
                <div class="jlsx_boxname">学历要求</div>
                <div class="jlsx_box_p">
                    <a href="javascript:void(0);" onclick="sxset('edu','',this);" class="eduCtrl jlsx_boxjy <?php if (!$_GET['edu']) {?>jlsx_boxjy_cur<?php }?>">全部</a>
                    <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['userdata']->value['user_edu']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                    <a href="javascript:void(0);" onclick="sxset('edu','<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
',this);" class="eduCtrl jlsx_boxjy <?php if ($_GET['edu']==$_smarty_tpl->tpl_vars['v']->value) {?>jlsx_boxjy_cur<?php }?>"><?php echo $_smarty_tpl->tpl_vars['userclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a>
                    <?php } ?>
                </div>
                <div class="jlsx_boxname">性别要求</div>
                <div class="jlsx_box_p">
                    <a href="javascript:void(0);" onclick="sxset('sex','',this);" class="sexCtrl jlsx_boxjy <?php if (!$_GET['sex']) {?>jlsx_boxjy_cur<?php }?>">全部</a>
                    <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['user_sex']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                    <a href="javascript:void(0);" onclick="sxset('sex','<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
',this);" class="sexCtrl jlsx_boxjy  <?php if ($_GET['sex']==$_smarty_tpl->tpl_vars['key']->value) {?>jlsx_boxjy_cur<?php }?>"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</a>
                    <?php } ?>
                </div>
                <div class="jlsx_boxname">更新时间</div>
                <div class="jlsx_box_p">
                    <a href="javascript:void(0);" onclick="sxset('uptime','',this);" class="uptimeCtrl jlsx_boxjy <?php if (!$_GET['uptime']) {?>jlsx_boxjy_cur<?php }?>">全部</a>
                    <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['uptime']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                    <a href="javascript:void(0);" onclick="sxset('uptime','<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
',this);" class="uptimeCtrl jlsx_boxjy <?php if ($_GET['uptime']==$_smarty_tpl->tpl_vars['key']->value) {?>jlsx_boxjy_cur<?php }?>"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</a>
                    <?php } ?>
                </div>
                <div class="jlsx_boxname">简历类型</div>
                <div class="jlsx_box_p">
                    <a href="javascript:void(0);" onclick="sxset('resumetype','',this);" class="resumetypeCtrl jlsx_boxjy <?php if (!$_GET['resumetype']) {?>jlsx_boxjy_cur<?php }?>">全部</a>
                    <a href="javascript:void(0);" onclick="sxset('resumetype','1',this);" class="resumetypeCtrl jlsx_boxjy <?php if ($_GET['resumetype']==1) {?>jlsx_boxjy_cur<?php }?>">普通简历</a>
                    <a href="javascript:void(0);" onclick="sxset('resumetype','2',this);" class="resumetypeCtrl jlsx_boxjy <?php if ($_GET['resumetype']==2) {?>jlsx_boxjy_cur<?php }?>">优质简历</a>
                </div>
            </div>
        </div>
        <div class="jlsx_box_bth"><a href="javascript:void(0);" onclick="$('#MyForm').submit();" class="jlsx_box_bth_a">确定</a></div>
    </div>
    <div class="jlsx_bg"></div>
</div>
<!--筛选弹出 end-->

<?php echo '<script'; ?>
>
    $(function () {

        $(document).bind("click", function (e) {
            //目标对象
            var object = $(e.target);
            if (object.closest("#yp_jobname").length == 0) {
                $('#yp_joblist').hide();
            }

            if (object.closest("#jlsx_box").length == 0 && object.closest("#joblist_search_more").length == 0) {
                $('#jlsx').hide();
            }

        })

        $(".bodyreturn").hover(function () {
            var msg = $(this).attr('msg');
            if ($.trim(msg) == '') {
                msg = '没有内容';
            }
            layer.tips(msg, this, {
                guide: 1,
                style: ['background-color:#5EA7DC; color:#fff;top:-7px', '#5EA7DC'],
                area: ['auto', 'auto'],
                time: 5000
            });
            $(".xubox_layer").addClass("xubox_tips_border");
        }, function () {

            layer.closeAll('tips');

        });
    })
    layui.use(['form'], function () {
        var form = layui.form,
            $ = layui.$;

        form.on('checkbox(allcom)', function (data) {
            $("input[name='delid[]']").each(function () {
                this.checked = data.elem.checked;
            });
            form.render('checkbox');
        });
    });



    function sxjob(val, obj) {
        $('#jobid').val(val);
        $('#MyForm').submit();
    }

    function sxset(id, val, obj) {

        $('.' + id + 'Ctrl').removeClass('jlsx_boxjy_cur');
        $(obj).addClass('jlsx_boxjy_cur');
        $('#' + id).val(val);
    }

    function bjAddClass(id) {
        $('.bj_' + id).addClass("com_received_username_bjboxcur");
    }

    function bjRemoveClass(id) {
        $('.bj_' + id).removeClass("com_received_username_bjboxcur");
    }

    function changeBrowse(browse, id) {
        $.post("index.php?c=hr&act=hrset", {
            id: id,
            browse: browse
        }, function (data) {
            location.reload();
        });
    }

    function changeBrowseAll(browse, name) {
        var chk_value = [];
        $('input[name="' + name + '"]:checked').each(function () {
            chk_value.push($(this).val());
        });
        if (chk_value.length == 0) {
            layer.msg("请选择要标记的数据！", 2, 8);
            return false;
        } else {
            layer.confirm("确定标记为已读吗？", function () {
                $.post("index.php?c=hr&act=hrset", {
                    ids: chk_value,
                    ajax: 1
                }, function (data) {
                    var data = eval('(' + data + ')');
                    if (data.errcode == '9') {
                        parent.layer.msg(data.msg, 2, 9, function () {
                            window.location.reload();
                            window.event.returnValue = false;
                            return false;
                        });
                        return false;
                    } else {
                        parent.layer.msg(data.msg, 2, 8);
                        return false;
                    }
                })
            });
        }
    }

    function rstateTip(rstate) {

        var reason = "";
        if (rstate == '0') {
            reason = '该简历为未审核状态，暂时无法对其查看、邀请面试等操作，我们将尽快完成该简历的审核，对您的工作造成不便请谅解';
        } else if (rstate == '3') {
            reason = '该简历未通过审核';
        }
        layer.alert(reason);
    }

    function lookreason(reason) {
        layer.alert(reason);
    }

    function remark(uid,eid) {
        $.post('index.php?c=hr&act=remarkhr',{uid:uid,eid:eid},function(res){
            if(res){
                $("#remark").val(res.remark);
                $('#status').val(res.status);
            }else{
                $("#remark").val('');
                $('#status').val(0);
            }
            layui.use(['form'], function() {
                var form = layui.form,
                    $ = layui.$;
                form.render('select');
            });

            var layindex = $.layer({
                type: 1,
                title: '备注简历',
                closeBtn: [0, true],
                border: [10, 0.3, '#000', true],
                area: ['420px', 'auto'],
                page: {
                    dom: "#remarkbox"
                }
            });
            $("#layindex").val(layindex);
        },'json');
        $("input[name=uid]").val(uid);
        $("input[name=eid]").val(eid);
    }

    function yp_jobshow() {
        $('#yp_joblist').toggle();
    }
    function remarkSubmit(){
        if($("#remark").val() == ''){
            layer.msg("请填写备注内容", 2, 8);
            return false;
        }
        loadlayer();
    }
<?php echo '</script'; ?>
>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/yqms.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
