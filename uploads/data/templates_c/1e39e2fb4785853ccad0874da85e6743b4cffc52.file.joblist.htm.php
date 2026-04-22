<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:20:09
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/joblist.htm" */ ?>
<?php /*%%SmartyHeaderCode:158972359969e8a0d96359e5-20598872%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '1e39e2fb4785853ccad0874da85e6743b4cffc52' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/joblist.htm',
      1 => 1706496289,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '158972359969e8a0d96359e5-20598872',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'w1' => 0,
    'w0' => 0,
    'w3' => 0,
    'w4' => 0,
    'w5' => 0,
    'addjobnum' => 0,
    'audit' => 0,
    'config' => 0,
    'i_know_job' => 0,
    'statis' => 0,
    'todayStart' => 0,
    'jobNum' => 0,
    'partNum' => 0,
    'ltjobNum' => 0,
    'type' => 0,
    'rows' => 0,
    'job' => 0,
    'isPaused' => 0,
    'pagenav' => 0,
    'hbids' => 0,
    'v' => 0,
    'hbNum' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0d9682b04_41641098',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0d9682b04_41641098')) {function content_69e8a0d9682b04_41641098($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
if (!is_callable('smarty_modifier_date_format')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/modifier.date_format.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<div class="w1000">
    <div class="admin_mainbody">
        <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/left.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

    <div class="right_box">
            <div class="newmember_tit">
                <ul>
                    <li class="newmember_titcur"> <a href="index.php?c=job&w=1" title="职位管理" >职位管理</a></li>
                    <li><a href="index.php?c=partok&w=1" title="兼职管理" class=" ">兼职管理</a></li>
					<li><a href="index.php?c=zhaopin" title="招聘数据" class=" ">招聘数据</a></li>
                </ul>
            </div>
            <div class="newmember_screenbox">
                <div class="newmember_screen">
                    <ul class="">
                        <li <?php if ($_GET['w']=="1") {?> class="job_list_tit_cur" <?php }?>>
                            <a href="index.php?c=job&w=1">招聘中<span class="job_list_tit_n"><?php if ($_smarty_tpl->tpl_vars['w1']->value>0) {?>(<?php echo $_smarty_tpl->tpl_vars['w1']->value;?>
)<?php }?></span></a>
                        </li>
                        <li <?php if ($_GET['w']=="0") {?> class="job_list_tit_cur" <?php }?>>
                            <a href="index.php?c=job&w=0">待审核<span class="job_list_tit_n"><?php if ($_smarty_tpl->tpl_vars['w0']->value>0) {?>(<?php echo $_smarty_tpl->tpl_vars['w0']->value;?>
)<?php }?></span></a>
                        </li>
                        <li <?php if ($_GET['w']=="3") {?> class="job_list_tit_cur" <?php }?>>
                            <a href="index.php?c=job&w=3">未通过<span class="job_list_tit_n"><?php if ($_smarty_tpl->tpl_vars['w3']->value>0) {?>(<?php echo $_smarty_tpl->tpl_vars['w3']->value;?>
)<?php }?></span></a>
                        </li>
                        <li <?php if ($_GET['w']=="4") {?> class="job_list_tit_cur" <?php }?>>
                            <a href="index.php?c=job&w=4">已下架<span class="job_list_tit_n"><?php if ($_smarty_tpl->tpl_vars['w4']->value>0) {?>(<?php echo $_smarty_tpl->tpl_vars['w4']->value;?>
)<?php }?></span></a>
                        </li>
                        <li <?php if ($_GET['w']=="5") {?> class="job_list_tit_cur" <?php }?>>
                            <a href="index.php?c=job&w=5">全部职位<span class="job_list_tit_n"><?php if ($_smarty_tpl->tpl_vars['w5']->value>0) {?>(<?php echo $_smarty_tpl->tpl_vars['w5']->value;?>
)<?php }?></span></a>
                        </li>
                    </ul>

                    <!--搜索-->
                    <div class="com_topbth_box">
                        <a href="javascript:void(0)" onclick="jobadd_url('<?php echo $_smarty_tpl->tpl_vars['addjobnum']->value;?>
');return false;"
                            class="com_topbth">发布职位</a>
<!--
                        <?php if ($_smarty_tpl->tpl_vars['audit']->value>0) {?>
                        <div class="com_topbth_zh">
                            <div class="com_topbth_zh_pd">温馨小提示
                                <div>你有 <font color="#FF0000"><?php echo $_smarty_tpl->tpl_vars['audit']->value;?>
</font>
                                    个待审核职位，我们将在24小时内审核，如需马上审核，请联系客服：
                                    <?php echo $_smarty_tpl->tpl_vars['config']->value['sy_comwebtel'];?>
</div>
                            </div>
                        </div>
                        <?php } elseif ($_smarty_tpl->tpl_vars['i_know_job']->value==0) {?>
                        <?php if ($_smarty_tpl->tpl_vars['statis']->value['rating_type']==1) {?>
                        <div class="com_topbth_zh" id="i_know">
                            <div class="com_topbth_zh_pd">你的账号，可上架 <?php if ($_smarty_tpl->tpl_vars['statis']->value['vip_etime']>$_smarty_tpl->tpl_vars['todayStart']->value||$_smarty_tpl->tpl_vars['statis']->value['vip_etime']=="0") {
echo $_smarty_tpl->tpl_vars['statis']->value['job_num'];
} else { ?>0<?php }?>
                                个职位
                                <?php if (!empty($_smarty_tpl->tpl_vars['jobNum']->value)||!empty($_smarty_tpl->tpl_vars['partNum']->value)||!empty($_smarty_tpl->tpl_vars['ltjobNum']->value)) {?>
                                <div class="">现已上架： <br />
                                    <?php if (!empty($_smarty_tpl->tpl_vars['jobNum']->value)) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['jobNum']->value;?>
 个全职职位<br />
                                    <?php }?>
                                    <?php if (!empty($_smarty_tpl->tpl_vars['partNum']->value)) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['partNum']->value;?>
 个兼职职位<br />
                                    <?php }?>
                                    <?php if (!empty($_smarty_tpl->tpl_vars['ltjobNum']->value)) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['ltjobNum']->value;?>
 个猎头职位
                                    <?php }?>
                                </div>
                                <?php }?>
                            </div>
                            <div class="com_topbth_zh_bot"><a href="javascript:void(0);" onclick="i_know('job')">知道了</a>
                            </div>
                        </div>
                        <?php }?>
                        <?php }?>-->
                    </div>
                    <div class="joblist_search">
                        <form action="index.php" method="get">
                            <div class="joblist_search_box">
                                <input name="c" type="hidden" value="job">
                                <input name="w" type="hidden" value="<?php echo $_GET['w'];?>
">
                                <input name="type" type="hidden" >
                                <input name="keyword" type="text" class="joblist_search_box_text" value="<?php echo $_GET['keyword'];?>
"
                                    placeholder="请输入职位关键字">
                                <input name="" type="submit" class="joblist_search_bth" value=" ">
                            </div>
                        </form>
                    </div>
                    <!--搜索-->
                </div>
            </div>


            <div class="admincont_box" style="padding-top: 10px;;">
                <div class="job_lookmode">查看方式
                    <a href="index.php?c=job&w=1" id="firststyle"
                        class="com_resume_listbox_titlook_zs <?php if ($_smarty_tpl->tpl_vars['type']->value!=2) {?>com_resume_listbox_titlook_cur<?php }?>"><i
                            class="com_resume_listbox_titlook_zslb "></i></a>
                    <a href="index.php?c=job&w=1&type=2" id="twostyle"
                        class="com_resume_listbox_titlook_zs <?php if ($_smarty_tpl->tpl_vars['type']->value==2) {?>com_resume_listbox_titlook_cur<?php }?>"><i
                            class="com_resume_listbox_titlook_zsxx"></i></a>
                </div>

                <div class="com_body">
                    <div class="clear"></div>

                    <iframe id="supportiframe" name="supportiframe" onload="returnmessage('supportiframe');"
                        style="display:none"></iframe>
                    <form name="MyForm" action="index.php?c=job&act=opera" target="supportiframe" method="post"
                        id="myform" class="layui-form">
                        <div class="clear"></div>
                        <div id="jobtwolook" <?php if ($_smarty_tpl->tpl_vars['type']->value!=2) {?>style="display:none;" <?php }?>>
                            <table class="com_table ">
                                <?php if ($_smarty_tpl->tpl_vars['rows']->value) {?>
                                <tr>
                                    <th width="25" align="center">&nbsp;</th>
                                    <th>职位名称</th>
                                    <th>应聘简历</th>
                                    <th>被浏览</th>
                                    <th>曝光量</th>
                                    <th>刷新日期</th>
                                    <th width="180">分享</th>
                                    <th>职位推广</th>
                                    <th>操作</th>
                                </tr>
                                <?php }?>
                                <?php  $_smarty_tpl->tpl_vars['job'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['job']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['rows']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['job']->key => $_smarty_tpl->tpl_vars['job']->value) {
$_smarty_tpl->tpl_vars['job']->_loop = true;
?>
                                <tr>
                                    <td width="25" align="center">
                                        <input type="checkbox" name="checkboxidid[]" value="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
"
                                            class="com_job_list_check" lay-skin="primary" />
                                    </td>
                                    <td>
                                        <div class="job_looklist_namebox">
                                            <a href="<?php echo smarty_function_url(array('m'=>'job','c'=>'comapply','id'=>'`$job.id`'),$_smarty_tpl);?>
"
                                                class="job_looklist_name" target="_blank"><?php echo $_smarty_tpl->tpl_vars['job']->value['name'];?>
</a>
                                        </div>

                                    </td>
                                    <td align="center">
                                        <?php echo $_smarty_tpl->tpl_vars['job']->value['jobnum'];?>
 份<br />
                                        <?php if ($_smarty_tpl->tpl_vars['job']->value['jobnum']>0) {?>
                                        <a href="index.php?c=hr&jobid=<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
-<?php echo $_smarty_tpl->tpl_vars['job']->value['type'];?>
"
                                            class="yun_m_job_r_l">查看</a>
                                        <?php }?>
                                    </td>

                                    <td align="center"><?php echo $_smarty_tpl->tpl_vars['job']->value['jobhits'];?>
次</td>
                                    <td align="center"><?php echo $_smarty_tpl->tpl_vars['job']->value['jobexpoure'];?>
次</td>
                                    <td align="center"><?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['lastupdate'],'%Y-%m-%d');?>
</td>
                                    <td align="center">

                                        <a href="javascript:void(0)" onclick="shareShow('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
');"
                                            class="job_looklist_fx job_share" style="padding-left:0">分享</a>

                                        
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_haibao_isopen']==1) {?>
                                        <a href="javascript:void(0)" onclick="getJobHb(0, '<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
');"
                                            class="job_looklist_hb job_whb">海报</a>
                                        <?php }?>
                                    </td>
                                    <td align="center" width="300">
                                        <div class="job_looklist_tgbox" style="width:300px">
                                            <?php if ($_smarty_tpl->tpl_vars['job']->value['rec_time']>time()&&$_smarty_tpl->tpl_vars['job']->value['rec']==1) {?>
                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['tg_back']==1) {?>
                                            <a href="javascript:void(0);"
                                                onclick="closeJobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['rec_time'],'%Y-%m-%d');?>
' , '<?php echo $_smarty_tpl->tpl_vars['job']->value['rec_day'];?>
', 2)"
                                                class="job_looklist_tg job_looklist_tg_kq">推荐</a>
                                            <?php } else { ?>
                                            <a href="javascript:void(0);"
                                                onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['rec_time'],'%Y-%m-%d');?>
', 2)"
                                                class="job_looklist_tg job_looklist_tg_kq">推荐</a>
                                            <?php }?>
                                            <?php } else { ?>
                                            <a href="javascript:void(0);"
                                                onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','', 2)"
                                                class="job_looklist_tg">推荐</a>
                                            <?php }?>

                                            <?php if ($_smarty_tpl->tpl_vars['job']->value['urgent_time']>time()&&$_smarty_tpl->tpl_vars['job']->value['urgent']==1) {?>
                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['tg_back']==1) {?>
                                            <a href="javascript:void(0);"
                                                onclick="closeJobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['urgent_time'],'%Y-%m-%d');?>
', '<?php echo $_smarty_tpl->tpl_vars['job']->value['urgent_day'];?>
', 3)"
                                                class="job_looklist_tg  job_looklist_tg_kq">紧急</a>
                                            <?php } else { ?>
                                            <a href="javascript:void(0);"
                                                onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['urgent_time'],'%Y-%m-%d');?>
', 3)"
                                                class="job_looklist_tg  job_looklist_tg_kq">紧急</a>
                                            <?php }?>
                                            <?php } else { ?>
                                            <a href="javascript:void(0);"
                                                onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','', 3)"
                                                class="job_looklist_tg">紧急</a>
                                            <?php }?>

                                            <?php if ($_smarty_tpl->tpl_vars['job']->value['xsdate']>time()&&$_smarty_tpl->tpl_vars['job']->value['xsdate']) {?>
                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['tg_back']==1) {?>
                                            <a href="javascript:void(0);"
                                                onclick="closeJobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['xsdate'],'%Y-%m-%d');?>
', '<?php echo $_smarty_tpl->tpl_vars['job']->value['top_day'];?>
', 1);"
                                                class="job_looklist_tg job_looklist_tg_kq">置顶</a>
                                            <?php } else { ?>
                                            <a href="javascript:void(0);"
                                                onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['xsdate'],'%Y-%m-%d');?>
', 1);"
                                                class="job_looklist_tg job_looklist_tg_kq">置顶</a>
                                            <?php }?>
                                            <?php } else { ?>
                                            <a href="javascript:void(0);"
                                                onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','', 1);"
                                                class="job_looklist_tg">置顶</a>
                                            <?php }?>


                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['com_job_reserve']==1) {?>
                                            <a href="javascript:void(0);"
                                                onclick="reserveRefreshJob('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
', '<?php echo $_smarty_tpl->tpl_vars['job']->value['reserve_status'];?>
', '<?php echo $_smarty_tpl->tpl_vars['job']->value['reserve_interval'];?>
', '<?php echo $_smarty_tpl->tpl_vars['job']->value['reserve_end'];?>
' ,'<?php echo $_smarty_tpl->tpl_vars['job']->value['s_time'];?>
', '<?php echo $_smarty_tpl->tpl_vars['job']->value['e_time'];?>
');"
                                                class="job_looklist_tg <?php if ($_smarty_tpl->tpl_vars['job']->value['is_reserve']==1) {?>job_looklist_tg_kq<?php }?>">预约刷新</a>
                                            <?php } else { ?>
                                            <a href="javascript:void(0);"
                                                onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php if ($_smarty_tpl->tpl_vars['job']->value['autotime']>time()) {
echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['autodate'],'%Y-%m-%d');
}?>', 5);"
                                                class="job_looklist_tg <?php if ($_smarty_tpl->tpl_vars['job']->value['autotime']>time()) {?>job_looklist_tg_kq<?php }?>">自动刷新</a>
                                            <?php }?>

                                        </div>
                                    </td>

                                    <td align="center" width="180">
                                        <div style="padding-bottom:10px;width:180px;">
                                        <a href="index.php?c=likeresume&jobid=<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
"
                                            class="com_bth">匹配</a>
                                        <a href="javascript:void(0)"
                                            onclick="refreshJob('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
''<?php echo $_smarty_tpl->tpl_vars['statis']->value['upJobNum'];?>
','', '<?php echo $_smarty_tpl->tpl_vars['isPaused']->value;?>
');"
                                            class="com_bth">刷新</a>
                                            <a href="<?php echo smarty_function_url(array('m'=>'job','c'=>'comapply','id'=>'`$job.id`'),$_smarty_tpl);?>
" target="_blank"
                                                title="预览" class="com_bth">预览</a>
                                                </div>
                                        <?php if ($_smarty_tpl->tpl_vars['job']->value['status']=="1") {?>
                                        <?php if ($_smarty_tpl->tpl_vars['statis']->value['vip_etime']>$_smarty_tpl->tpl_vars['todayStart']->value||$_smarty_tpl->tpl_vars['statis']->value['vip_etime']=="0") {?>
                                        <?php if ($_smarty_tpl->tpl_vars['statis']->value['rating_type']==1) {?>
                                        <a href="javascript:onstatus('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','2', '', '您可上架职位<?php echo $_smarty_tpl->tpl_vars['statis']->value['job_num'];?>
个，确认上架？');"
                                            class="com_bth">上架</a>
                                        <?php } else { ?>
                                        <a href="javascript:onstatus('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','2', '', '确认上架？');"
                                            class="com_bth">上架</a>
                                        <?php }?>
                                        <?php } else { ?>
                                        <a href="javascript:onstatus('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','2');"
                                            class="com_bth">上架</a>
                                        <?php }?>
                                        <?php } else { ?>
                                        <a href="javascript:onstatus('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','1');"
                                            class="com_bth">下架</a>
                                        <?php }?>

                                        <a href="index.php?c=jobadd&act=edit&id=<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
"
                                            class="com_bth">修改</a>
                                        <a href="javascript:void(0)"
                                            onclick="layer_del('确定要删除该职位？', 'index.php?c=job&act=opera&del=<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
');"
                                            class="com_bth">删除</a>
                                    </td>
                                </tr>
                                <?php }
if (!$_smarty_tpl->tpl_vars['job']->_loop) {
?>
                                <tr>
                                    <td colspan="5" class="table_end">
                                        <div class="com_msg_no">
                                            <p class="com_msg_no_name">招人才，没有职位怎么行？</p>
                                            <p class="">快去发布职位，优质人才任你挑</p>
                                            <a href="javascript:;" onclick="jobadd_url('<?php echo $_smarty_tpl->tpl_vars['addjobnum']->value;?>
');"
                                                class="com_msg_no_bth com_submit">发布职位</a>
                                        </div>
                                    </td>
                                </tr>

                                <?php } ?>
                                <?php if (!empty($_smarty_tpl->tpl_vars['rows']->value)) {?>
                                <tr>
                                    <td align="center">
                                        <input type="checkbox" lay-filter="allcomid" lay-skin="primary" />
                                    </td>
                                    <td colspan="8">
                                        <div class="com_Release_job_bot" style="padding-top:0px;">
                                            <span class="com_Release_job_qx"> 全选 </span>

                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['com_job_reserve']!=1) {?>
                                                <input class="c_btn_02 c_btn_02_w110" type="button" value="批量自动刷新" onclick="jobPromote('checkboxidid[]','', 5);">
                                            <?php } else { ?>
                                                <input class="c_btn_02 c_btn_02_w110" type="button" value="批量预约刷新" onclick="return reserveAllJob('checkboxidid[]','<?php echo $_smarty_tpl->tpl_vars['statis']->value['upJobNum'];?>
','<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_reserve_refresh_price'];?>
');">
                                            <?php }?>
                                            <input class="c_btn_02 c_btn_02_w110" type="button" value="批量刷新职位"
                                                onclick="return refreshAllJob('checkboxidid[]','<?php echo $_smarty_tpl->tpl_vars['statis']->value['upJobNum'];?>
','<?php echo $_smarty_tpl->tpl_vars['statis']->value['rating_type'];?>
', '<?php echo $_smarty_tpl->tpl_vars['isPaused']->value;?>
');">
                                            <input class="c_btn_02 c_btn_02_w110" type="button" value="批量下架职位"
                                                onclick="return allonstatusid('checkboxidid[]');">
                                            <input class="c_btn_02 c_btn_02_w110" type="button" value="批量删除职位"
                                                onclick="return really('checkboxidid[]');">
                                        </div>
                                    </td>
                                </tr>
                                <?php }?>
                            </table>
                        </div>
                        <div class="clear"></div>
                        <div id="jobfirstlook" <?php if ($_smarty_tpl->tpl_vars['type']->value==2) {?>style="display:none;" <?php }?>>
                            <table class="com_table">
                                <?php if ($_smarty_tpl->tpl_vars['rows']->value) {?>
                                <tr>
                                    <th width="25" align="center">&nbsp;</th>
                                    <th>职位名称</th>
                                    <th>招聘情况</th>
                                    <th>职位推广</th>
                                    <th>操作</th>
                                </tr>
                                <?php }?>
                                <?php  $_smarty_tpl->tpl_vars['job'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['job']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['rows']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['job']->key => $_smarty_tpl->tpl_vars['job']->value) {
$_smarty_tpl->tpl_vars['job']->_loop = true;
?>
                                <tr>
                                    <td align="left">
                                        <input type="checkbox" name="checkboxid[]" value="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
"
                                            class="com_job_list_check" lay-skin="primary" />
                                    </td>
                                    <td align="top">
                                        <div class="yun_m_jobname">
                                            <a href="<?php echo smarty_function_url(array('m'=>'job','c'=>'comapply','id'=>'`$job.id`'),$_smarty_tpl);?>
"
                                                class="yun_m_jobname_a" target="_blank"><?php echo $_smarty_tpl->tpl_vars['job']->value['name'];?>
</a>
                                        </div>
                                        
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_haibao_isopen']==1) {?>
                                        <div class="yun_m_joblist_right_resume job_whb" style="width:120px;"
                                            onclick="selectHb('1', '1', '<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo $_smarty_tpl->tpl_vars['job']->value['name'];?>
')">
                                            <a style="cursor: pointer;">招聘海报展示</a>
                                        </div>
                                        <?php }?>
                                        <div class="yun_m_jobshare" style="margin-top: 10px;">
                                            <a class="job_share" onclick="shareShow('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
');"
                                                style="cursor: pointer;"><i></i>分享到朋友圈</a>
                                        </div>
                                    </td>

                                    <td>
                                        <div class="yun_m_jobqk">
                                            <div class="yun_m_jobqk_box">
                                                <span class="yun_m_jobqk_box_n">收到简历</span>
                                                <a href="index.php?c=hr&jobid=<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
-<?php echo $_smarty_tpl->tpl_vars['job']->value['type'];?>
"
                                                    class="yun_m_jobqk_n"><?php echo $_smarty_tpl->tpl_vars['job']->value['jobnum'];?>
</a> 份
                                            </div>
                                            <div class="yun_m_jobqk_box">
                                                <span class="yun_m_jobqk_box_n">被浏览</span>
                                                <span class="yun_m_jobqk_n"><?php echo $_smarty_tpl->tpl_vars['job']->value['jobhits'];?>
</span> 次
                                            </div>
                                            <div class="yun_m_jobqk_box">
                                                <span class="yun_m_jobqk_box_n">曝光量 </span>
                                                <span class="yun_m_jobqk_n"><?php echo $_smarty_tpl->tpl_vars['job']->value['jobexpoure'];?>
</span>次
                                            </div>

                                            <div class="yun_m_jobqk_box">
                                                <span class="yun_m_jobqk_box_n">刷新日期 </span>
                                                <?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['lastupdate'],'%Y-%m-%d %H:%M:%S');?>

                                            </div>
                                            <div class="yun_m_job_lookresume"></div>
                                        </div>
                                    </td>

                                    <td>
                                        <div class="yun_m_joblist_cont" style="width:350px; margin:0 auto">

                                            <?php if ($_smarty_tpl->tpl_vars['job']->value['rec_time']>time()&&$_smarty_tpl->tpl_vars['job']->value['rec']==1) {?>
                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['tg_back']==1) {?>
                                            <div class="yun_m_joblist_extension yun_m_joblist_extension_cur yun_m_joblist_extension_cur_hov"
                                                dtype="rec" pid="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                <i class="yun_m_joblist_extension_icon"></i>
                                                <i class="yun_m_joblist_tip_icon"></i>
                                                <div class="yun_m_joblist_extension_p">
                                                    <font color="">推荐</font>
                                                </div>
                                                <a href="javascript:void(0);"
                                                    onclick="closeJobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['rec_time'],'%Y-%m-%d');?>
', '<?php echo $_smarty_tpl->tpl_vars['job']->value['rec_day'];?>
', 2)"
                                                    class="yun_m_joblist_extension_bth"></a>
                                            </div>
                                            <?php } else { ?>
                                            <div class="yun_m_joblist_extension yun_m_joblist_extension_cur yun_m_joblist_extension_cur_hov"
                                                dtype="rec" pid="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                <i class="yun_m_joblist_extension_icon"></i>
                                                <i class="yun_m_joblist_tip_icon"></i>
                                                <div class="yun_m_joblist_extension_p">
                                                    <font color="">推荐</font>
                                                </div>
                                                <a href="javascript:void(0);"
                                                    onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['rec_time'],'%Y-%m-%d');?>
', 2)"
                                                    class="yun_m_joblist_extension_bth"></a>
                                            </div>
                                            <?php }?>
                                            <?php } else { ?>
                                            <div class="yun_m_joblist_extension yun_m_joblist_extension_cur_hov"
                                                dtype="rec" pid="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                <i class="yun_m_joblist_extension_icon"></i>
                                                <i class="yun_m_joblist_tip_icon"></i>
                                                <div class="yun_m_joblist_extension_p">
                                                    <font color="">推荐</font>
                                                </div>
                                                <a href="javascript:void(0);"
                                                    onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','', 2)"
                                                    class="yun_m_joblist_extension_bth"></a>
                                            </div>
                                            <?php }?>

                                            <?php if ($_smarty_tpl->tpl_vars['job']->value['urgent_time']>time()&&$_smarty_tpl->tpl_vars['job']->value['urgent']==1) {?>
                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['tg_back']==1) {?>
                                            <div class="yun_m_joblist_extension  yun_m_joblist_extension_cur"
                                                dtype="urgent" pid="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                <i
                                                    class="yun_m_joblist_extension_icon yun_m_joblist_extension_icon_urgent"></i>
                                                <i class="yun_m_joblist_tip_icon"></i>
                                                <div class="yun_m_joblist_extension_p">
                                                    <font color="">紧急</font>
                                                </div>
                                                <a href="javascript:void(0);"
                                                    onclick="closeJobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['urgent_time'],'%Y-%m-%d');?>
' , '<?php echo $_smarty_tpl->tpl_vars['job']->value['urgent_day'];?>
', 3)"
                                                    class="yun_m_joblist_extension_bth"></a>
                                            </div>
                                            <?php } else { ?>
                                            <div class="yun_m_joblist_extension  yun_m_joblist_extension_cur"
                                                dtype="urgent" pid="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                <i
                                                    class="yun_m_joblist_extension_icon yun_m_joblist_extension_icon_urgent"></i>
                                                <i class="yun_m_joblist_tip_icon"></i>
                                                <div class="yun_m_joblist_extension_p">
                                                    <font color="">紧急</font>
                                                </div>
                                                <a href="javascript:void(0);"
                                                    onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['urgent_time'],'%Y-%m-%d');?>
', 3)"
                                                    class="yun_m_joblist_extension_bth"></a>
                                            </div>
                                            <?php }?>
                                            <?php } else { ?>
                                            <div class="yun_m_joblist_extension" dtype="urgent"
                                                pid="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                <i
                                                    class="yun_m_joblist_extension_icon yun_m_joblist_extension_icon_urgent"></i>
                                                <i class="yun_m_joblist_tip_icon"></i>
                                                <div class="yun_m_joblist_extension_p">
                                                    <font color="">紧急</font>
                                                </div>
                                                <a href="javascript:void(0);"
                                                    onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','', 3)"
                                                    class="yun_m_joblist_extension_bth"></a>
                                            </div>
                                            <?php }?>
                                            <?php if ($_smarty_tpl->tpl_vars['job']->value['xsdate']>time()&&$_smarty_tpl->tpl_vars['job']->value['xsdate']) {?>
                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['tg_back']==1) {?>
                                            <div class="yun_m_joblist_extension yun_m_joblist_extension_cur"
                                                dtype="jingjia" pid="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                <i
                                                    class="yun_m_joblist_extension_icon yun_m_joblist_extension_icon_zd"></i>
                                                <i class="yun_m_joblist_tip_icon"></i>
                                                <div class="yun_m_joblist_extension_p">
                                                    <font color="">置顶</font>
                                                </div>
                                                <a href="javascript:void(0);"
                                                    onclick="closeJobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['xsdate'],'%Y-%m-%d');?>
' , '<?php echo $_smarty_tpl->tpl_vars['job']->value['top_day'];?>
', 1);"
                                                    class="yun_m_joblist_extension_bth"></a>
                                            </div>
                                            <?php } else { ?>
                                            <div class="yun_m_joblist_extension yun_m_joblist_extension_cur"
                                                dtype="jingjia" pid="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                <i
                                                    class="yun_m_joblist_extension_icon yun_m_joblist_extension_icon_zd"></i>
                                                <i class="yun_m_joblist_tip_icon"></i>
                                                <div class="yun_m_joblist_extension_p">
                                                    <font color="">置顶</font>
                                                </div>
                                                <a href="javascript:void(0);"
                                                    onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['xsdate'],'%Y-%m-%d');?>
', 1);"
                                                    class="yun_m_joblist_extension_bth"></a>
                                            </div>
                                            <?php }?>
                                            <?php } else { ?>
                                            <div class="yun_m_joblist_extension" dtype="jingjia"
                                                pid="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                <i
                                                    class="yun_m_joblist_extension_icon yun_m_joblist_extension_icon_zd"></i>
                                                <i class="yun_m_joblist_tip_icon"></i>
                                                <div class="yun_m_joblist_extension_p">
                                                    <font color="">置顶</font>
                                                </div>
                                                <a href="javascript:void(0);"
                                                    onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','', 1);"
                                                    class="yun_m_joblist_extension_bth"></a>
                                            </div>
                                            <?php }?>

                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['com_job_reserve']==1) {?>
                                            <div class="yun_m_joblist_extension <?php if ($_smarty_tpl->tpl_vars['job']->value['is_reserve']==1) {?>yun_m_joblist_extension_cur<?php }?>"
                                                dtype="reserve" pid="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                <i
                                                    class="yun_m_joblist_extension_icon yun_m_joblist_extension_icon_sx"></i>
                                                <i class="yun_m_joblist_tip_icon"></i>
                                                <div class="yun_m_joblist_extension_p">
                                                    <font color="">预约刷新</font>
                                                </div>
                                                <a href="javascript:void(0);"
                                                    onclick="reserveRefreshJob('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
', '<?php echo $_smarty_tpl->tpl_vars['job']->value['reserve_status'];?>
', '<?php echo $_smarty_tpl->tpl_vars['job']->value['reserve_interval'];?>
', '<?php echo $_smarty_tpl->tpl_vars['job']->value['reserve_end'];?>
' ,'<?php echo $_smarty_tpl->tpl_vars['job']->value['s_time'];?>
', '<?php echo $_smarty_tpl->tpl_vars['job']->value['e_time'];?>
');"
                                                    class="yun_m_joblist_extension_bth"></a>
                                            </div>
                                            <?php } else { ?>
                                            <?php if ($_smarty_tpl->tpl_vars['job']->value['autotime']>time()) {?>
                                            <div class="yun_m_joblist_extension <?php if ($_smarty_tpl->tpl_vars['job']->value['autotime']>time()) {?>yun_m_joblist_extension_cur<?php }?>"
                                                dtype="autojob" pid="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                <i
                                                    class="yun_m_joblist_extension_icon yun_m_joblist_extension_icon_sx"></i>
                                                <i class="yun_m_joblist_tip_icon"></i>
                                                <div class="yun_m_joblist_extension_p">
                                                    <font color="">自动刷新</font>
                                                </div>
                                                <a href="javascript:void(0);"
                                                    onclick="autoJobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['autodate'],'%Y-%m-%d');?>
', '<?php echo $_smarty_tpl->tpl_vars['job']->value['auto_day'];?>
');"
                                                    class="yun_m_joblist_extension_bth"></a>
                                            </div>
                                            <?php } else { ?>
                                            <div class="yun_m_joblist_extension <?php if ($_smarty_tpl->tpl_vars['job']->value['autotime']>time()) {?>yun_m_joblist_extension_cur<?php }?>"
                                                dtype="autojob" pid="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                <i
                                                    class="yun_m_joblist_extension_icon yun_m_joblist_extension_icon_sx"></i>
                                                <i class="yun_m_joblist_tip_icon"></i>
                                                <div class="yun_m_joblist_extension_p">
                                                    <font color="">自动刷新</font>
                                                </div>
                                                <a href="javascript:void(0);"
                                                    onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php if ($_smarty_tpl->tpl_vars['job']->value['autotime']>time()) {
echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['job']->value['autodate'],'%Y-%m-%d');
}?>', 5);"
                                                    class="yun_m_joblist_extension_bth"></a>
                                            </div>
                                            <?php }?>
                                            <?php }?>

                                            <div class="yun_m_joblist_tip" id="tip<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
">
                                                展示在列表右侧黄金位置，带来精确的投递、高质的效果
                                            </div>
                                        </div>
                                    </td>
                                    <td align="center" width="150">
                                        <div style="padding-bottom:10px;width:150px;">

                                            <a href="javascript:void(0)"
                                                onclick="refreshJob('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','<?php echo $_smarty_tpl->tpl_vars['statis']->value['upJobNum'];?>
','', '<?php echo $_smarty_tpl->tpl_vars['isPaused']->value;?>
');"
                                                class="com_bth">刷新</a>
                                            <a href="<?php echo smarty_function_url(array('m'=>'job','c'=>'comapply','id'=>'`$job.id`'),$_smarty_tpl);?>
" target="_blank"
                                                title="预览" class="com_bth">预览</a>
                                                </div>
                                                <div style="padding-bottom:10px;width:150px;">
                                            <?php if ($_smarty_tpl->tpl_vars['job']->value['status']=="1") {?>
                                            <?php if ($_smarty_tpl->tpl_vars['statis']->value['vip_etime']>$_smarty_tpl->tpl_vars['todayStart']->value||$_smarty_tpl->tpl_vars['statis']->value['vip_etime']=="0") {?>
                                            <?php if ($_smarty_tpl->tpl_vars['statis']->value['rating_type']==1) {?>
                                            <a href="javascript:onstatus('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','2', '', '您可上架职位<?php echo $_smarty_tpl->tpl_vars['statis']->value['job_num'];?>
个，确认上架？');"
                                                class="com_bth">上架</a>
                                            <?php } else { ?>
                                            <a href="javascript:onstatus('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','2', '', '确认上架？');"
                                                class="com_bth">上架</a>
                                            <?php }?>
                                            <?php } else { ?>
                                            <a href="javascript:onstatus('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','2');"
                                                class="com_bth">上架</a>
                                            <?php }?>
                                            <?php } else { ?>
                                            <a href="javascript:onstatus('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
','1');"
                                                class="com_bth">下架</a>
                                            <?php }?>

                                            <a href="index.php?c=jobadd&act=edit&id=<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
"
                                                class="com_bth">修改</a>
                                                </div>
                                            <a href="index.php?c=likeresume&jobid=<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
"
                                                class="com_bth">匹配</a>
                                            <a href="javascript:void(0)"
                                                onclick="layer_del('确定要删除该职位？', 'index.php?c=job&act=opera&del=<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
');"
                                                class="com_bth">删除</a>
                                        </div>
                                    </td>
                                </tr>
                                <?php }
if (!$_smarty_tpl->tpl_vars['job']->_loop) {
?>
                                <tr>
                                    <td colspan="5" class="table_end">
                                        <div class="com_msg_no">
                                            <p class="com_msg_no_name">招人才，没有职位怎么行？</p>
                                            <p class="">快去发布职位，优质人才任你挑</p>
                                            <a href="javascript:;" onclick="jobadd_url('<?php echo $_smarty_tpl->tpl_vars['addjobnum']->value;?>
');"
                                                class="com_msg_no_bth com_submit">发布职位</a>
                                        </div>
                                    </td>
                                </tr>
                                <?php } ?>

                                <?php if (!empty($_smarty_tpl->tpl_vars['rows']->value)) {?>
                                <tr>
                                    <td align="center">
                                        <input type="checkbox" lay-filter="allcom" lay-skin="primary" />
                                    </td>

                                    <td colspan="4">
                                        <div class="com_Release_job_bot" style="padding-top:0px;">
                                            <span class="com_Release_job_qx"> 全选 </span>
                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['com_job_reserve']!=1) {?>
                                            <input class="c_btn_02 c_btn_02_w110" type="button" value="批量自动刷新" onclick="jobPromote('checkboxid[]','', 5);">
                                            <?php } else { ?>
                                            <input class="c_btn_02 c_btn_02_w110" type="button" value="批量预约刷新" onclick="return reserveAllJob('checkboxid[]','<?php echo $_smarty_tpl->tpl_vars['statis']->value['upJobNum'];?>
','<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_reserve_refresh_price'];?>
');">
                                            <?php }?>
                                            <input class="c_btn_02 c_btn_02_w110" type="button" value="批量刷新职位" onclick="return refreshAllJob('checkboxid[]','<?php echo $_smarty_tpl->tpl_vars['statis']->value['upJobNum'];?>
','<?php echo $_smarty_tpl->tpl_vars['statis']->value['rating_type'];?>
', '<?php echo $_smarty_tpl->tpl_vars['isPaused']->value;?>
');">
                                            <input class="c_btn_02 c_btn_02_w110" type="button" value="批量下架职位" onclick="return allonstatus('checkboxid[]');">
                                            <input class="c_btn_02 c_btn_02_w110" type="button" value="批量删除职位" onclick="return really('checkboxid[]');">
                                        </div>
                                    </td>
                                </tr>
                                <?php }?>
                            </table>
                        </div>
                        <div class="diggg" style="<?php if ($_smarty_tpl->tpl_vars['type']->value==2) {?>margin-top: -38px;<?php } else { ?>margin-top: -40px;<?php }?>"><?php echo $_smarty_tpl->tpl_vars['pagenav']->value;?>
</div>
                    </form>
                    <div class="clear"></div>

                </div>
            </div>
            <div class="com_tip_bottom  ">
                <div class="wxts_box wxts_box_mt30">
                    <div class="wxts">温馨提示：</div>
                    <?php if ($_smarty_tpl->tpl_vars['statis']->value['vip_etime']>$_smarty_tpl->tpl_vars['todayStart']->value||$_smarty_tpl->tpl_vars['statis']->value['vip_etime']=="0") {?>
                    <?php if ($_smarty_tpl->tpl_vars['statis']->value['rating_type']==1) {?>
                    1、贵公司共可上架（<span class="f60"><?php echo $_smarty_tpl->tpl_vars['statis']->value['job_num'];?>
</span>）条职位信息<br>
                    <?php } else { ?>
                    1、贵公司可发布（<span class="f60">不限</span>）条职位信息<br>
                    <?php }?>
                    <?php } else { ?>
                    1、贵公司共可上架（<span class="f60">0</span>）条职位信息<br>
                    <?php }?>
                    2、如贵公司要快速招聘人才，建议成为我们的会员，获取更多的展示机会，以帮助您快速找到满意的人才。 <a href="index.php?c=right" class="wxts_sj" style="color:red;">立即升级</a>。<br>
                    3、请贵公司保证职位信息的真实性、合法性，并及时更新职位信息，如被举报或投诉，确认发布的信息违反相关规定后，本站将会关闭贵公司的招聘服务，敬请谅解！ <br>
                    4、参加紧急的招聘职位;我们将在首页紧急招聘 模块显示，并有紧急图标显示。<br>
                    5、参加自动刷新的招聘职位;使招聘职位信息置于列表前端，更有利于吸引客户的注意 <br>
                    6、参加置顶服务的招聘职位；我们将在首页列表模块显示 ！
                </div>

            </div>
        </div>
    </div>
</div>
<input type="hidden" id="refreshjobids" value="" />

<div id="wxShare" style="display:none;">
    <div class="yun_wxbd_box">
        <div class="yun_wxbd_img_c">
            <div id="wx_share_qrcode" class="yun_wxbd_img" style="border:1px solid #eee; line-height:180px;">
                正在获取二维码...
            </div>
        </div>
        <div class="yun_wxbd_p"> 请使用微信扫描二维码分享职位</div>
    </div>
</div>



<?php echo '<script'; ?>
>
    layui.use(['form', 'layer', 'laydate'], function() {
        var $ = layui.$,
            form = layui.form,
            laydate = layui.laydate,
            layer = layui.layer;

        form.on('checkbox(allcom)', function(data) {
            $("input[name='checkboxid[]']").each(function() {
                this.checked = data.elem.checked;
            });
            form.render('checkbox');
        });
        form.on('checkbox(allcomid)', function(data) {
            $("input[name='checkboxidid[]']").each(function() {
                this.checked = data.elem.checked;
            });
            form.render('checkbox');
        });
    });

    function allonstatusid() {
        var allid = [];
        var i = 0;
        $('input[name="checkboxidid[]"]:checked').each(function() {
            allid.push($(this).val());
            i++;
        });
        if (allid.length == 0) {
            layer.msg("请选择要下架的职位！", 2, 8);
            return false;
        } else {
            onstatus(allid, 1);
        }
    }

    function allonstatus() {
        var allid = [];
        var i = 0;
        $('input[name="checkboxid[]"]:checked').each(function() {
            allid.push($(this).val());
            i++;
        });
        if (allid.length == 0) {
            layer.msg("请选择要下架的职位！", 2, 8);
            return false;
        } else {
            onstatus(allid, 1);
        }
    }

    var weburl = '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
';
    var hbids = [];
    '<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['hbids']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>'
    hbids.push('<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
')
    '<?php } ?>'

    function getJobHb(hb, id) {

        layer.closeAll();

        var hbNum = '<?php echo $_smarty_tpl->tpl_vars['hbNum']->value;?>
';

        var url = weburl + '/index.php?m=ajax&c=getJobHb&id=' + id + '&hb=' + hbids[hb];

        if (hb < (parseInt(hbNum) - 1)) {
            var next = hb + 1;
        } else {
            var next = 0;
        }

        var loading = layer.load('生成中...', 0);

        var image = new Image();
        image.src = url;
        image.onload = function() {
            layer.closeAll();
            layer.open({
                type: 1,
                title: false,
                content: '<div class="hb_tc"><img src="' + url +
                    '" style="max-width: 100%;"><div class="hb_tc_bth"><a href="javascript:;" onclick="getJobHb(' +
                    next + ', ' + id +
                    ');" class="hb_tc_hyz">换一张</a><a href="javascript:;" onclick="downWhb(' + hb + ', ' + id +
                    ');" class="hb_tc_xz">下载海报</a></div></div>',
                area: ['360px', 'auto'],
                offset: '55px',
                closeBtn: 0,
                shadeClose: true
            });
        }
    }

    function downWhb(hb, id) {
        var loading = layer.load('下载中...', 0);
        var url = weburl + '/index.php?m=ajax&c=getJobHb&id=' + id + '&hb=' + +hbids[hb];
        var image = new Image();
        image.src = url;
        image.onload = function() {
            layer.closeAll();
            var a = document.createElement('a'); // 创建一个a节点插入的document
            var event = new MouseEvent('click') // 模拟鼠标click点击事件
            a.download = 'whb' + id + '_' + hbids[hb]; // 设置a节点的download属性值
            a.href = url; // 将图片的src赋值给a节点的href
            a.dispatchEvent(event);
        }
    }
<?php echo '</script'; ?>
>

<?php echo '<script'; ?>
>
    function shareShow(id) {
        var h5 = '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_h5_share'];?>
';
        if (h5 == '1') {
            var data = weburl + '/index.php?m=ajax&c=pubqrcode&toc=job&toa=share&toid=' + id;
        } else {
            var data = weburl + '/index.php?m=ajax&c=pubqrcode&toc=job&toa=view&toid=' + id;
        }

        $('#wx_share_qrcode').html('<img src="' + data + '" width="180" height="180" />');

        $.layer({
            type: 1,
            title: '微信分享',
            border: [10, 0.3, '#000', true],
            area: ['400px', 'auto'],
            page: {
                dom: "#wxShare"
            },
            close: function() {
                if (setval) {
                    clearInterval(setval);
                    setval = null;
                }
                if (setwout) {
                    clearTimeout(setwout);
                    setwout = null;
                }
            }
        });

    }

    $(document).ready(function() {

        $('.yun_m_joblist_extension').hover(function() {
            var type = $(this).attr('dtype');
            var pid = $(this).attr('pid');

            $('.yun_m_joblist_extension').removeClass('yun_m_joblist_extension_cur_hov');

            $(this).addClass('yun_m_joblist_extension_cur_hov');

            if (type == 'rec') {

                $('#tip' + pid).html('展示在列表右侧黄金位置，带来精确的投递、高质的效果');
            } else if (type == 'urgent') {

                $('#tip' + pid).html('紧急招聘，强烈提升职位曝光度');
            } else if (type == 'jingjia') {

                $('#tip' + pid).html('将职位信息固定排在页面第一页，不会被其他信息挤下去');
            } else if (type == 'autojob') {

                $('#tip' + pid).html('自动刷新，让职位管理轻松更高效');
            } else if (type == 'reserve') {

                $('#tip' + pid).html('预约刷新，让职位管理轻松更高效');
            }
        });
    });
    
<?php echo '</script'; ?>
>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/public_search/hb.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo '<script'; ?>
 type="text/javascript">
	// 用来判断预约刷新时间段选项
	var sy_reserve_refresh_interval = parseInt("<?php if ($_smarty_tpl->tpl_vars['config']->value['com_job_reserve']==1) {
echo $_smarty_tpl->tpl_vars['config']->value['sy_reserve_refresh_interval'];
} else { ?>0<?php }?>");
<?php echo '</script'; ?>
>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/jobserver.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
