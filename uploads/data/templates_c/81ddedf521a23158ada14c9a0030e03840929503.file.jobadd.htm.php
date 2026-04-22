<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:44:48
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/jobadd.htm" */ ?>
<?php /*%%SmartyHeaderCode:106475518969e8989021c3b1-71988256%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '81ddedf521a23158ada14c9a0030e03840929503' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/jobadd.htm',
      1 => 1706496289,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '106475518969e8989021c3b1-71988256',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'style' => 0,
    'row' => 0,
    'companycert' => 0,
    'job_ms_rating' => 0,
    'company' => 0,
    'referer' => 0,
    'statis' => 0,
    'todayStart' => 0,
    'jobnum' => 0,
    'jionly' => 0,
    'comdata' => 0,
    'v' => 0,
    'j' => 0,
    'comclass_name' => 0,
    'defLink' => 0,
    'addressList' => 0,
    'key' => 0,
    'linkid' => 0,
    'com_sex' => 0,
    'industry_index' => 0,
    'industry_name' => 0,
    'tv' => 0,
    'userdata' => 0,
    'userclass_name' => 0,
    'jobState' => 0,
    'uid' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89890254829_76276699',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89890254829_76276699')) {function content_69e89890254829_76276699($_smarty_tpl) {?><?php if (!is_callable('smarty_modifier_date_format')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/modifier.date_format.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/public_search/index_search.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<div class="w1000">
    <div class="admin_mainbody">
        <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/left.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

        <?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/data/plus/job.cache.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/javascript"><?php echo '</script'; ?>
>
        <?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/data/plus/jobparent.cache.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/javascript"><?php echo '</script'; ?>
>
        <link rel="stylesheet" href="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/style/newclass.public.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/css"/>
        <?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/newclass.public.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/javascript"><?php echo '</script'; ?>
>
        <?php echo '<script'; ?>
 language="javascript">
            function setLink(type) {

                if ($("#is" + type).hasClass('com_job_tel_setlist_zt_cur')) {
                    $("#is" + type).removeClass('com_job_tel_setlist_zt_cur');
                    if (type == 'link') {

                        var link_id = $('#link_id').val();
                        var selLinkId = 'dd[lay-value=' + link_id + ']';
                        $('#link_id').siblings("div.layui-form-select").find('dl').find(selLinkId).click();
                        $("input[name='is_link']").val(1);
                    } else {

                        $("input[name='is_" + type + "']").val(2);
                    }
                } else {
                    $("#is" + type).addClass('com_job_tel_setlist_zt_cur');
                    if (type == 'link') {

                        $("input[name='is_link']").val(3);
                    } else {

                        $("input[name='is_" + type + "']").val(1);
                    }
                }
            }

            function returnmessagejob(frame_id) {
                if (frame_id == '' || frame_id == undefined) {
                    frame_id = 'supportiframe';
                }
                var message = $(window.frames[frame_id].document).find("#layer_msg").val();
                if (message != null) {
                    var url = $(window.frames[frame_id].document).find("#layer_url").val();
                    var layer_time = $(window.frames[frame_id].document).find("#layer_time").val();
                    var layer_st = $(window.frames[frame_id].document).find("#layer_st").val();
                    var layer_url = $(window.frames[frame_id].document).find("#layer_url").val();
                    var h5share = '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_h5_share'];?>
';
                    layer.closeAll('loading');
                    if (layer_st == '9') {
                        //判断是否开启职位推广并且是新增职位
                        $('#jobid').val(layer_url);
                        $('#jobshow').attr('href', weburl + '/job/?c=comapply&id=' + layer_url);
                        if (h5share == 1) {
                            $('#sharecode').attr('src', weburl + '/index.php?m=ajax&c=pubqrcode&toc=job&toa=share&toid=' + layer_url);
                        } else {
                            $('#sharecode').attr('src', weburl + '/index.php?m=ajax&c=pubqrcode&toc=job&toa=view&toid=' + layer_url);
                        }
                        var jid = '<?php echo $_smarty_tpl->tpl_vars['row']->value['id'];?>
';
                        
                            $('#jobrec').attr('onclick', 'jobPromote(\'' + layer_url + '\',\'\', 2)');
                            $('#joburgent').attr('onclick', 'jobPromote(\'' + layer_url + '\',\'\', 3)');
                            $('#jobtop').attr('onclick', 'jobPromote(\'' + layer_url + '\',\'\', 1)');
                            $('#jobauto').attr('onclick', 'jobPromote(\'' + layer_url + '\',\'\', 5)');
                            
                        $.layer({
                            type: 1,
                            move: false,
                            fix: true,
                            zIndex: 666,
                            title: '系统提示',
                            offset: ['100px', ''],
                            border: [10, 0.3, '#000', true],
                            area: ['710px', '330px'],
                            page: {dom: '#addjob'},
                            close: function () {
                                <?php if (($_smarty_tpl->tpl_vars['config']->value['com_free_status']==1&&$_smarty_tpl->tpl_vars['companycert']->value&&$_smarty_tpl->tpl_vars['companycert']->value['status']==1)||($_smarty_tpl->tpl_vars['job_ms_rating']->value&&in_array($_smarty_tpl->tpl_vars['company']->value['rating'],$_smarty_tpl->tpl_vars['job_ms_rating']->value))||$_smarty_tpl->tpl_vars['config']->value['com_job_status']=='1') {?>
                                    <?php if ($_smarty_tpl->tpl_vars['row']->value&&$_smarty_tpl->tpl_vars['referer']->value) {?>
                                        window.location.href = "<?php echo $_smarty_tpl->tpl_vars['referer']->value;?>
";
                                    <?php } else { ?>
                                        window.location.href = "index.php?c=job&w=1";
                                    <?php }?>
                                <?php } else { ?>
                                    window.location.href = "index.php?c=job&w=0";
                                <?php }?>
                            }
                        });
                    } else {
                        if (url == '1') {
                            layer.msg(message, layer_time, Number(layer_st), function () {
                                window.location.reload();
                                window.event.returnValue = false;
                                return false;
                            });
                        } else if (url == '') {

                            layer.msg(message, layer_time, Number(layer_st));
                            $('#submitBtn').attr("disabled", false);
                        } else {
                            layer.msg(message, layer_time, Number(layer_st), function () {
                                window.location.href = url;
                                window.event.returnValue = false;
                                return false;
                            });
                        }
                    }
                }
            }
        <?php echo '</script'; ?>
>

        <input type="hidden" id="comname" value="<?php echo $_smarty_tpl->tpl_vars['company']->value['name'];?>
">
        
        <div class=right_box>
            <div class="newmember_screenbox">
                <div class="newmember_screen">
                    <div class="newmember_screenname"><a href="index.php?c=job&w=1" class="newcom_fh">返回 </a><span
                            class="newcom_fh_line">|</span>新增职位</div>
                </div>
            </div>
            
             
            <div class=admincont_box>
                <div class="com_body">
                    <div class="com_new_tip">
                        <span class="com_new_tip_h">温馨小提示</span>
                        <?php if ($_smarty_tpl->tpl_vars['statis']->value['vip_etime']>$_smarty_tpl->tpl_vars['todayStart']->value||$_smarty_tpl->tpl_vars['statis']->value['vip_etime']=="0") {?>
                            <?php if ($_smarty_tpl->tpl_vars['statis']->value['rating_type']==2) {?>
                                您当前是 <span class="com_new_tip_v"><?php echo $_smarty_tpl->tpl_vars['statis']->value['rating_name'];?>
</span>，到期时间是<?php if ($_smarty_tpl->tpl_vars['statis']->value['vip_etime']!="0") {
echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['statis']->value['vip_etime'],'%Y-%m-%d');
} else { ?>永久<?php }?>，在此之前您可以任意发布职位&nbsp;&nbsp;&nbsp;
                            <?php } else { ?>
                                您当前是 <span class="com_new_tip_v"><?php echo $_smarty_tpl->tpl_vars['statis']->value['rating_name'];?>
 </span><?php if ($_smarty_tpl->tpl_vars['jobnum']->value) {?>，现已上架 <?php echo $_smarty_tpl->tpl_vars['jobnum']->value;?>
 个职位<?php }?>， 您共可上架：<?php echo $_smarty_tpl->tpl_vars['statis']->value['job_num'];?>
个职位。超出后发布的职位需要手动上架&nbsp;&nbsp;&nbsp;
                            <?php }?>
                        <?php } else { ?>
                            您的会员已到期，为了更好的招聘人才，请先<a href="index.php?c=right" class="cblue"> 升级会员</a>！&nbsp;&nbsp;&nbsp;
                        <?php }?>
                        <span class="add_tit_bz">（注 带 <i class="ff0">*</i> 号为必填项）</span>
                    </div>
                    <iframe id="supportiframejob" name="supportiframejob" onload="returnmessagejob('supportiframejob');" style="display:none"></iframe>
                    <form name="MyForm" id="myform" target="supportiframejob" method="post" action="index.php?c=jobadd&act=save" class="layui-form">

                        <div class="com_release_box" id="comjob">
                            <ul>
                                <li>
                                    <div class="com_release_name"><i class="ff0">*</i> 职位名称：</div>
                                    <div class="com_release_cont">
                                        <?php if ($_GET['id']&&$_smarty_tpl->tpl_vars['config']->value['joblock']==1) {?>
                                        <label>
                                            <div class="info_comname_text"><?php echo $_smarty_tpl->tpl_vars['row']->value['name'];?>
</div>
                                        </label>
                                        <input type="hidden" id="name" name="name" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['name'];?>
">
                                        <?php } else { ?>
                                        <div class="com_release_cont_text">
                                            <input type="text" size="45" lay-verify="required" name="name" id='name' value="<?php echo $_smarty_tpl->tpl_vars['row']->value['name'];?>
" class="layui-input" placeholder="请填写职位名称信息，可填写职位属性，如：货运司机包吃住">
                                        </div>
                                        <?php }?>
                                        <span id="by_name" class="errordisplay">职位名不能为空</span>
                                    </div>
                                    <?php if ($_smarty_tpl->tpl_vars['config']->value['joblock']==1) {?>
                                    <div class="" style="color:#f00; padding-top:10px;font-size:12px;">提示：职位名称确认发布后将无法修改</div>
                                    <?php }?>
                                </li>
                                <li>
                                    <div class="com_release_name"><i class="ff0">*</i> 职位类别：</div>
                                    <div class="com_release_cont com_release_cont_xlyc">
                                        <div class="com_release_cont_text" style="position:relative">
                                            <input type="hidden" name="job_post" id="job_post" value="<?php if ($_smarty_tpl->tpl_vars['row']->value['job_post']) {
echo $_smarty_tpl->tpl_vars['row']->value['job_post'];
} elseif ($_smarty_tpl->tpl_vars['row']->value['job1_son']) {
echo $_smarty_tpl->tpl_vars['row']->value['job1_son'];
} elseif ($_smarty_tpl->tpl_vars['row']->value['job1']) {
echo $_smarty_tpl->tpl_vars['row']->value['job1'];
}?>"/>
                                            <select id="jobclass_search" name="jobclass_search" xm-select-type="jobclass" xm-select="jobclass_search" xm-select-search="" xm-select-radio="" xm-select-skin="default" xm-select-direction="down">
                                                <option value="">输入职位类别</option>
                                            </select>
                                            <?php if ($_smarty_tpl->tpl_vars['jionly']->value!='1') {?>
                                            <div onclick="index_job_new(1,'#workadds_job','#job_post','left:100px;top:100px; position:absolute;','1');" class="news_expect_text_new_sxzw" title="选择职位类别">类别筛选</div>
                                            <?php }?>
                                        </div>
                                    </div>
                                </li>
                                <li>
                                    <div class="com_release_name"><i class="ff0">*</i> 薪资待遇：</div>
                                    <div class="com_release_cont">
                                        <div class="layui-input-inline com_release_selectw145">
                                            <input type="text" size="5" id="minsalary" name="minsalary" <?php if ($_smarty_tpl->tpl_vars['row']->value['minsalary']) {?>value="<?php echo $_smarty_tpl->tpl_vars['row']->value['minsalary'];?>
"<?php }?> onkeyup="this.value=this.value.replace(/[^0-9]/g,'')" class="layui-input" placeholder="最低薪资"<?php if (!$_smarty_tpl->tpl_vars['row']->value['minsalary']&&!$_smarty_tpl->tpl_vars['row']->value['maxsalary']&&$_smarty_tpl->tpl_vars['row']->value['id']&&$_smarty_tpl->tpl_vars['config']->value['com_job_myswitch']=="1") {?> disabled="disabled"<?php }?>>
                                            <span class="com_release_cont_dw">元/月</span>
                                        </div>
                                        <div class="layui-input-inline com_release_selectw145">
                                            <input type="text" size="5" id="maxsalary" name="maxsalary" <?php if ($_smarty_tpl->tpl_vars['row']->value['maxsalary']) {?>value="<?php echo $_smarty_tpl->tpl_vars['row']->value['maxsalary'];?>
"<?php }?> onkeyup="this.value=this.value.replace(/[^0-9]/g,'')" class="layui-input" placeholder="最高薪资"<?php if (!$_smarty_tpl->tpl_vars['row']->value['minsalary']&&!$_smarty_tpl->tpl_vars['row']->value['maxsalary']&&$_smarty_tpl->tpl_vars['row']->value['id']&&$_smarty_tpl->tpl_vars['config']->value['com_job_myswitch']=="1") {?> disabled="disabled"<?php }?>>
                                            <span class="com_release_cont_dw">元/月</span>
                                        </div>
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['com_job_myswitch']=="1") {?>
                                        <input type="checkbox" id="salary_type" name="salary_type" title="面议" value="1" <?php if (!$_smarty_tpl->tpl_vars['row']->value['minsalary']&&!$_smarty_tpl->tpl_vars['row']->value['maxsalary']&&$_smarty_tpl->tpl_vars['row']->value['id']) {?> checked="checked"<?php }?> lay-filter="salary_type" lay-skin="primary" />
                                        <?php }?>
                                    </div>
                                </li>
                                <li>
                                    <div class="com_release_name"><i class="ff0">*</i> 招聘人数：</div>
                                    <div class="com_release_cont">
                                        <div class="layui-input-inline com_release_selectw145">
                                            <input name="zp_num" id="zp_num" type="text" <?php if ($_smarty_tpl->tpl_vars['row']->value['zp_num']) {?>value="<?php echo $_smarty_tpl->tpl_vars['row']->value['zp_num'];?>
"<?php }?> onkeyup="this.value=this.value.replace(/[^0-9]/g,'')" class="layui-input" />
                                            <span class="com_release_cont_dw">人</span>
                                        </div>
                                    </div>
                                </li>
                                <li>
                                    <div class="com_release_name"> 招聘要求：</div>
                                    <div class="com_release_cont">
                                        <div class="layui-input-inline com_release_selectw145">
                                            <select name="edu" lay-filter="edu">
                                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_edu']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                                                <option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['edu']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php if ($_smarty_tpl->tpl_vars['j']->value==0) {?>学历<?php }
echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
                                                <?php } ?>
                                            </select>
                                        </div>
                                        <div class="layui-input-inline com_release_selectw145">
                                            <select name="exp" lay-filter="exp">
                                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_exp']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                                                <option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['exp']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php if ($_smarty_tpl->tpl_vars['j']->value==0&&stripos($_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value],'经验')===false) {?>经验<?php }
echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
                                                <?php } ?>
                                            </select>
                                        </div>
                                        &nbsp;&nbsp;
                                        <input name="is_graduate" type="checkbox" value="1" title="可接受应届生" <?php if ($_smarty_tpl->tpl_vars['row']->value['is_graduate']) {?>checked<?php }?> lay-skin="primary" />
                                    </div>
                                </li>
                                <li>
                                    <div class="com_release_name"><i class="ff0">*</i> 职位描述：</div>
                                    <div class="com_release_cont">
                                        <div class="Description" style="display:none;">
                                            <div class="Description_icon">
                                                <i class="Description_icon_i"></i>
                                                <div class="Description_box" style="display:none;">
                                                    <i class="Description_icon_i_j"></i>
                                                    点击职位链接，为你推荐的职位要求模板复制到编辑区域内！<br>您也可以编辑，直至完美！
                                                </div>
                                            </div>
                                            <div class="Description_box_mb">样本：<span id="JobRequInfoTemplate"></span>
                                            </div>
                                        </div>
                                        <div class="clear"></div>
                                        <?php echo '<script'; ?>
 id="description" name="description" type="text/plain" style="width:500px; height:100px;"> <?php echo $_smarty_tpl->tpl_vars['row']->value['description'];?>
<?php echo '</script'; ?>
>
                                        <span id="by_description" class="errordisplay">不能为空</span>
                                    </div>
                                </li>
                                <li>
                                    <div class="com_release_name"><i class="ff0">*</i> 工作地址：</div>
                                    <div class="com_release_cont" style="height:38px;">
                                        <div class="layui-input-inline" style="width:675px; float: left;">
                                            <select name="link_id" lay-filter="link_id" id="link_id">
                                                <option id="link_id_-1" data-provinceid="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['provinceid'];?>
" data-cityid="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['cityid'];?>
" data-three_cityid="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['three_cityid'];?>
" data-address="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['address'];?>
" data-x="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['x'];?>
" data-y="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['y'];?>
" data-link_man="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['link_man'];?>
" data-link_moblie="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['link_moblie'];?>
" data-link_phone="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['link_phone'];?>
" data-email="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['email'];?>
" value="-1" <?php if ($_smarty_tpl->tpl_vars['row']->value['is_link']==1) {?>selected<?php }?>><?php echo $_smarty_tpl->tpl_vars['defLink']->value['link_man'];?>
 - <?php echo $_smarty_tpl->tpl_vars['defLink']->value['link_moblie'];?>
 - <?php echo $_smarty_tpl->tpl_vars['defLink']->value['city'];?>
 - <?php echo $_smarty_tpl->tpl_vars['defLink']->value['address'];?>
 （默认企业地址）</option>
                                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['addressList']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                                                <?php if ($_smarty_tpl->tpl_vars['row']->value['link_id']==$_smarty_tpl->tpl_vars['v']->value['id']) {?>
                                                <?php $_smarty_tpl->tpl_vars["linkid"] = new Smarty_variable($_smarty_tpl->tpl_vars['key']->value, null, 0);?>
                                                <?php }?>
                                                <option id="link_id_<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
" data-provinceid="<?php echo $_smarty_tpl->tpl_vars['v']->value['provinceid'];?>
" data-cityid="<?php echo $_smarty_tpl->tpl_vars['v']->value['cityid'];?>
" data-three_cityid="<?php echo $_smarty_tpl->tpl_vars['v']->value['three_cityid'];?>
" data-address="<?php echo $_smarty_tpl->tpl_vars['v']->value['link_address'];?>
" data-x="<?php echo $_smarty_tpl->tpl_vars['v']->value['x'];?>
" data-y="<?php echo $_smarty_tpl->tpl_vars['v']->value['y'];?>
" data-link_man="<?php echo $_smarty_tpl->tpl_vars['v']->value['link_man'];?>
" data-link_moblie="<?php echo $_smarty_tpl->tpl_vars['v']->value['link_moblie'];?>
" data-link_phone="<?php echo $_smarty_tpl->tpl_vars['v']->value['link_phone'];?>
" data-email="<?php echo $_smarty_tpl->tpl_vars['v']->value['email'];?>
"  value="<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['is_link']!=1&&$_smarty_tpl->tpl_vars['row']->value['link_id']==$_smarty_tpl->tpl_vars['v']->value['id']) {?>selected<?php }?>><?php echo $_smarty_tpl->tpl_vars['v']->value['linkmsg'];?>
</option>
                                                <?php } ?>
                                            </select>
                                        </div>
                                        <?php if ($_smarty_tpl->tpl_vars['row']->value['is_link']&&$_smarty_tpl->tpl_vars['row']->value['is_link']!=1) {?>
                                        <a id="editAddr" href="javascript:;" data-id="<?php echo $_smarty_tpl->tpl_vars['addressList']->value[$_smarty_tpl->tpl_vars['linkid']->value]['id'];?>
" class="add_icon   newAddressBtn"   data-link="2" data-provinceid="<?php echo $_smarty_tpl->tpl_vars['addressList']->value[$_smarty_tpl->tpl_vars['linkid']->value]['provinceid'];?>
" data-cityid="<?php echo $_smarty_tpl->tpl_vars['addressList']->value[$_smarty_tpl->tpl_vars['linkid']->value]['cityid'];?>
" data-three_cityid="<?php echo $_smarty_tpl->tpl_vars['addressList']->value[$_smarty_tpl->tpl_vars['linkid']->value]['three_cityid'];?>
" data-address="<?php echo $_smarty_tpl->tpl_vars['addressList']->value[$_smarty_tpl->tpl_vars['linkid']->value]['link_address'];?>
" data-x="<?php echo $_smarty_tpl->tpl_vars['addressList']->value[$_smarty_tpl->tpl_vars['linkid']->value]['x'];?>
" data-y="<?php echo $_smarty_tpl->tpl_vars['addressList']->value[$_smarty_tpl->tpl_vars['linkid']->value]['y'];?>
" data-link_man="<?php echo $_smarty_tpl->tpl_vars['addressList']->value[$_smarty_tpl->tpl_vars['linkid']->value]['link_man'];?>
" data-link_moblie="<?php echo $_smarty_tpl->tpl_vars['addressList']->value[$_smarty_tpl->tpl_vars['linkid']->value]['link_moblie'];?>
" data-link_phone="<?php echo $_smarty_tpl->tpl_vars['addressList']->value[$_smarty_tpl->tpl_vars['linkid']->value]['link_phone'];?>
" data-email="<?php echo $_smarty_tpl->tpl_vars['addressList']->value[$_smarty_tpl->tpl_vars['linkid']->value]['email'];?>
"><span class="add_xg"></span>修改 </a>
                                        <?php } else { ?>
                                        <a id="editAddr" href="javascript:;" data-id="-1" class="add_icon   newAddressBtn"   data-link="2" data-provinceid="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['provinceid'];?>
" data-cityid="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['cityid'];?>
" data-three_cityid="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['three_cityid'];?>
" data-address="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['address'];?>
" data-x="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['x'];?>
" data-y="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['y'];?>
" data-link_man="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['link_man'];?>
" data-link_moblie="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['link_moblie'];?>
" data-link_phone="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['link_phone'];?>
" data-email="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['email'];?>
"><span class="add_xg"></span>修改 </a>
                                        <?php }?>
                                       
                                        <a href="javascript:;" class="add_icon add_xj  newAddressBtn"  data-link="2" data-provinceid="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['provinceid'];?>
" data-cityid="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['cityid'];?>
" data-three_cityid="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['three_cityid'];?>
" data-address="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['address'];?>
" data-x="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['x'];?>
" data-y="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['y'];?>
"><span class="add_xg"></span>新建 </a>
                                    </div>
                                </li>
                                <li>
                                    <div class="com_release_name">其他要求：</div>
                                    <div class="com_release_cont">
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['com_job_sexswitch']==1) {?>
                                        <div class="layui-input-inline com_release_selectw145">
                                            <select name="sex" lay-filter="sex">
                                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['com_sex']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                                                <option value="<?php echo $_smarty_tpl->tpl_vars['j']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['sex']==$_smarty_tpl->tpl_vars['j']->value) {?> selected<?php }?>><?php if ($_smarty_tpl->tpl_vars['j']->value==3) {?>性别<?php }
echo $_smarty_tpl->tpl_vars['v']->value;?>
</option>
                                                <?php } ?>
                                            </select>
                                        </div>
                                        <?php }?>
                                        <div class="layui-input-inline com_release_selectw145">
                                            <select name="marriage" lay-filter="marriage">
                                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_marriage']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                                                <option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['marriage']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php if ($_smarty_tpl->tpl_vars['j']->value==0) {?>婚况<?php }
echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
                                                <?php } ?>
                                            </select>
                                        </div>
                                        <div class="layui-input-inline com_release_selectw145">
                                            <select name="report" lay-filter="report">
                                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_report']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                                                <option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['report']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php if ($_smarty_tpl->tpl_vars['j']->value==0) {?>到岗时间<?php }
echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
                                                <?php } ?>
                                            </select>
                                        </div>
                                        <div class="layui-input-inline com_release_selectw145">
                                            <input type="text" size="2" id="zp_minage" name="zp_minage" placeholder="最小年龄" <?php if ($_smarty_tpl->tpl_vars['row']->value['zp_minage']) {?>value="<?php echo $_smarty_tpl->tpl_vars['row']->value['zp_minage'];?>
"<?php }?> onkeyup="this.value=this.value.replace(/[^0-9]/g,'')" maxlength="2" class="layui-input" />
                                            <span class="com_release_cont_dw">岁</span>
                                        </div>
                                        <div class="layui-input-inline com_release_selectw145">
                                            <input type="text" size="2" id="zp_maxage" name="zp_maxage" placeholder="最大年龄" <?php if ($_smarty_tpl->tpl_vars['row']->value['zp_maxage']) {?>value="<?php echo $_smarty_tpl->tpl_vars['row']->value['zp_maxage'];?>
"<?php }?> onkeyup="this.value=this.value.replace(/[^0-9]/g,'')" maxlength="2" class="layui-input" />
                                            <span class="com_release_cont_dw">岁</span>
                                        </div>
                                    </div>
                                </li>

                                <li>
                                    <div class="com_release_name">从事行业：</div>
                                    <div class="com_release_cont">
                                        <div class="layui-input-inline" style="width:500px;">
                                            <select name="hy" lay-filter="hy">
                                                <option value="">请选择</option>
                                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['industry_index']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                                                <option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['hy']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php echo $_smarty_tpl->tpl_vars['industry_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
                                                <?php } ?>
                                            </select>
                                        </div>
                                    </div>
                                </li>
                                <li class="jobadd_list_fl">
                                    <div class="com_release_name">福利待遇：</div>
                                    <div class="layui-form-item" style=" margin-bottom: 0px;;">
                                        <div class="layui-input-block">
                                            <span class="" id="addwelfarelist">
                                                <?php  $_smarty_tpl->tpl_vars['tv'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['tv']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['row']->value['arraywelfare']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['tv']->key => $_smarty_tpl->tpl_vars['tv']->value) {
$_smarty_tpl->tpl_vars['tv']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['tv']->key;
?>
                                                    <input name="welfare[]" id="welfare<?php echo $_smarty_tpl->tpl_vars['tv']->value;?>
" value="<?php echo $_smarty_tpl->tpl_vars['tv']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['arraywelfare']&&in_array($_smarty_tpl->tpl_vars['tv']->value,$_smarty_tpl->tpl_vars['row']->value['arraywelfare'])) {?> checked="checked" <?php }?> type="checkbox" title="<?php echo $_smarty_tpl->tpl_vars['tv']->value;?>
" data-tag="<?php echo $_smarty_tpl->tpl_vars['tv']->value;?>
" class="changewelfare" lay-skin="primary">
                                                <?php } ?>
                                            </span>
                                            <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_welfare']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                                            <?php ob_start();?><?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
<?php $_tmp1=ob_get_clean();?><?php if (!in_array($_tmp1,$_smarty_tpl->tpl_vars['row']->value['arraywelfare'])) {?>
                                            <input name="welfare[]" id="welfare<?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
" value="<?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['arraywelfare']&&in_array($_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value],$_smarty_tpl->tpl_vars['row']->value['arraywelfare'])) {?> checked="checked" <?php }?> type="checkbox" title="<?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
" data-tag="<?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
" class="changewelfare" lay-skin="primary" />
                                            <?php }?>
                                            <?php } ?>
                                            <div class="">
                                                <div class="addwelfare_b">
                                                    <input class="addwelfare_text" type="text" tabindex="1000" placeholder="自定义" id="addwelfare"><a class="addwelfarebox">添加福利</a>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </li>
                                <?php if ($_smarty_tpl->tpl_vars['comdata']->value['job_lang']!=''&&count($_smarty_tpl->tpl_vars['comdata']->value['job_lang'])>0) {?>
                                <li>
                                    <div class="com_release_name">语言要求：</div>
                                    <div class="layui-form-item" style=" margin-bottom: 0px;;">
                                        <div class="layui-input-block">
                                            <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_lang']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                                            <input name="lang[]" id="lang<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" title="<?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
" value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if (in_array($_smarty_tpl->tpl_vars['v']->value,$_smarty_tpl->tpl_vars['row']->value['lang'])) {?> checked="checked" <?php }?> type="checkbox" lay-skin="primary" />
                                            <?php } ?>
                                        </div>
                                    </div>
                                </li>
                                <?php }?>
                                <li>
                                    <div class="com_release_name">招聘设置：</div>
                                    <div class="jobadd_set">
                                        <div class="jobadd_set_tit">
                                            <a href="javascript:void(0)" data-id="spshow_xx1" class="jobadd_set_a spshow_yq_div_a_curr"><i></i>联系方式设置</a>
                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['sqjob_req']!=2) {?>
                                            <a href="javascript:void(0)" data-id="spshow_xx2" class="jobadd_set_a"><i></i>投递要求设置</a>
                                            <?php }?>
                                        </div>
                                    </div>
                                    <div class="jobadd_setbox" id="spshow_xx1">
                                        <div class="jobadd_setlist jobadd_setlist_line ">
                                            <div class="jobadd_setname">同步联系方式</div>
                                            <div class="jobadd_set_p">同步联系方式，工作地址到其他职位</div>
                                            <span class="jobadd_set_kg" onclick="setLink('tblink');" id="istblink"><i class="jobadd_set_kgicon"></i></span>
                                            <input type="hidden" name="is_tblink" id="is_tblink" value="0"/>
                                        </div>
                                        <div class="jobadd_setlist jobadd_setlist_line jobadd_setlist_linepv">
                                            <i class="jobadd_setlist_line_l"></i>
                                            <div class="jobadd_setname">投递短信通知</div>
                                            <div class="jobadd_set_p">求职者投递简历将会收到短信通知</div>
                                            <span class="jobadd_set_kg <?php if ($_smarty_tpl->tpl_vars['row']->value['is_message']==1||$_smarty_tpl->tpl_vars['row']->value['is_message']=='') {?>com_job_tel_setlist_zt_cur<?php }?>" onclick="setLink('message');" id="ismessage"><i class="jobadd_set_kgicon "></i></span>
                                            <input type="hidden" name="is_message" id="is_message" value="<?php if ($_smarty_tpl->tpl_vars['row']->value['is_message']) {
echo $_smarty_tpl->tpl_vars['row']->value['is_message'];
} else { ?>1<?php }?>"/>
                                        </div>
                                        <div class="jobadd_setlist">
                                            <div class="jobadd_setname">隐藏联系方式</div>
                                            <div class="jobadd_set_p">是否对求职者隐藏联系方式</div>
                                            <span class="jobadd_set_kg <?php if ($_smarty_tpl->tpl_vars['row']->value['is_link']==3) {?>com_job_tel_setlist_zt_cur<?php }?>" onclick="setLink('link');" id="islink"><i class="jobadd_set_kgicon "></i></span>
                                            <input id="is_link" name="is_link" value="<?php if ($_smarty_tpl->tpl_vars['row']->value['is_link']) {
echo $_smarty_tpl->tpl_vars['row']->value['is_link'];
} else { ?>1<?php }?>" type="hidden"/>
                                        </div>
                                        <div class="jobadd_setlist jobadd_setlist_linepv">
                                            <i class="jobadd_setlist_line_l"></i>
                                            <div class="jobadd_setname">邮件通知</div>
                                            <div class="jobadd_set_p">求职者投递简历将会通过邮件通知</div>
                                            <span class="jobadd_set_kg <?php if ($_smarty_tpl->tpl_vars['row']->value['is_email']==1||$_smarty_tpl->tpl_vars['row']->value['is_email']=='') {?>com_job_tel_setlist_zt_cur<?php }?>" onclick="setLink('email');" id="isemail"><i class="jobadd_set_kgicon"></i></span>
                                            <input type="hidden" name="is_email" id="is_email" value="<?php if ($_smarty_tpl->tpl_vars['row']->value['is_email']) {
echo $_smarty_tpl->tpl_vars['row']->value['is_email'];
} else { ?>1<?php }?>"/>
                                        </div>
                                    </div>
                                    <?php if ($_smarty_tpl->tpl_vars['config']->value['sqjob_req']!=2) {?>
                                    <div class="jobadd_setbox none " id="spshow_xx2">
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['sqjob_req']==0) {?>
                                        <div class="jobadd_settip">不符合以下面试条件的求职者投递简历，系统将自动标记为不合适。</div>
                                        <?php } else { ?>
                                        <div class="jobadd_settip">不符合面试条件的求职者，将无法投递简历。</div>
                                        <?php }?>
                                        <div class="jobadd_settd_list">
                                            <div class="jobadd_settd_list_name">工作经验</div>
                                            <div class="layui-input-inline jobadd_settd_listw160">
                                                <select name="exp_req" lay-filter="exp_req">
                                                    <option value="0" <?php if ($_smarty_tpl->tpl_vars['row']->value['exp_req']==0) {?>selected<?php }?>>不限</option>
                                                    <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['userdata']->value['user_word']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                                                    <?php if ($_smarty_tpl->tpl_vars['userclass_name']->value[$_smarty_tpl->tpl_vars['v']->value]!='不限') {?>
                                                    <option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['exp_req']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php echo $_smarty_tpl->tpl_vars['userclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
                                                    <?php }?>
                                                    <?php } ?>
                                                </select>
                                            </div>
                                        </div>
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['com_job_sexswitch']==1) {?>
                                        <div class="jobadd_settd_list">
                                            <div class="jobadd_settd_list_name">性别要求</div>
                                            <div class="layui-input-block">
                                                <input type="radio" name="sex_req" value="3" <?php if ($_smarty_tpl->tpl_vars['row']->value['sex_req']==3) {?>checked="checked"<?php }?> title="不限"/>
                                                <input type="radio" name="sex_req" value="2" <?php if ($_smarty_tpl->tpl_vars['row']->value['sex_req']==2) {?>checked="checked"<?php }?> title="女"/>
                                            </div>
                                        </div>
                                        <?php }?>
                                        <div class="jobadd_settd_list">
                                            <div class="jobadd_settd_list_name">学历要求</div>
                                            <div class="layui-input-inline jobadd_settd_listw160">
                                                <select name="edu_req" lay-filter="edu_req">
                                                    <option value="0" <?php if ($_smarty_tpl->tpl_vars['row']->value['edu_req']==0) {?>selected<?php }?>>不限</option>
                                                    <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['userdata']->value['user_edu']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                                                    <?php if ($_smarty_tpl->tpl_vars['userclass_name']->value[$_smarty_tpl->tpl_vars['v']->value]!='不限') {?>
                                                    <option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['edu_req']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php echo $_smarty_tpl->tpl_vars['userclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
                                                    <?php }?>
                                                    <?php } ?>
                                                </select>
                                            </div>
                                        </div>

                                        <div class="jobadd_settd_list">
                                            <div class="jobadd_settd_list_name">年龄要求</div>
                                            <div class="layui-input-block">
                                                <div class="layui-input-inline com_release_selectw145">
                                                    <input type="text" size="2" id="minage_req" name="minage_req" placeholder="最小年龄" <?php if ($_smarty_tpl->tpl_vars['row']->value['minage_req']) {?>value="<?php echo $_smarty_tpl->tpl_vars['row']->value['minage_req'];?>
"<?php }?> onkeyup="this.value=this.value.replace(/[^0-9]/g,'')" maxlength="2" class="layui-input" />
                                                    <span class="com_release_cont_dw">岁</span>
                                                </div>
                                                <div class="layui-input-inline com_release_selectw145">
                                                    <input type="text" size="2" id="maxage_req" name="maxage_req" placeholder="最大年龄" <?php if ($_smarty_tpl->tpl_vars['row']->value['maxage_req']) {?>value="<?php echo $_smarty_tpl->tpl_vars['row']->value['maxage_req'];?>
"<?php }?> onkeyup="this.value=this.value.replace(/[^0-9]/g,'')" maxlength="2" class="layui-input" />
                                                    <span class="com_release_cont_dw">岁</span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    <?php }?>
                                </li>
                            </ul>
                        </div>
                        <div class="clear"></div>
                        <div class=admin_submit>
                            <div class="admin_job_js_list_ft  fl"><span style="width:100%;">&nbsp;</span></div>
                            <div class=sub_btn>
                                <input type="hidden" id="r_status" name="r_status" value="<?php echo $_smarty_tpl->tpl_vars['company']->value['r_status'];?>
">
                                <input class="btn_01" id="submitBtn" type="button" name="submitBtn" value=" 提 交 ">
                                <?php if ($_GET['id']) {?>
                                <input id="id" name="id" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['id'];?>
" type="hidden"/>
                                <?php }?>
                                <input id="save" name="save" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['name'];?>
" type="hidden"/>
                            </div>
                        </div>
                        <div class="clear"></div>
                    </form>
                </div>
            </div>
        </div>
    </div>
</div>
<input type='hidden' id='jobid' value=''>
<div class="job_tck_box" id="addjob" style="display:none;">
    <div style="width:710px; background:#fff;">
        <div class="yun_prompt_writingicon"><i class="yun_prompt_writingicon_right"></i></div>
        <div class="yun_prompt_writing">职位发布成功！</div>
        <?php if ($_smarty_tpl->tpl_vars['jobState']->value==1) {?>
            <div class="yun_prompt_writing_tip">职位信息已审核，您可进行以下操作</div>
        <?php } else { ?>
            <div class="yun_prompt_writing_tip">管理员审核通过后才可展示职位信息，您可进行以下操作</div>
        <?php }?>
        <div class="yun_prompt_writing_operation">
            <a href="javascript:void(0)" onclick="addjob_continue('<?php echo $_smarty_tpl->tpl_vars['uid']->value;?>
');return false;" class="yun_prompt_jobtgbth">再发一条</a>
            <a id="jobshow" href="javascript:;" s class="yun_prompt_jobtgbth">查看信息</a>
            <a href="index.php?c=job&w=5" class="yun_prompt_jobtgbth">管理我的信息</a>
            <?php if ($_smarty_tpl->tpl_vars['jobState']->value==1) {?>
            <a href="javascript:;" class="yun_prompt_jobtgbth yun_prompt_jobtgbth_sm">
                扫码分享职位
                <div class="yun_prompt_jobtgbth_smbox none">
                    <div class="yun_prompt_jobtgbth_smboxpic">
                        <img id="sharecode" src="" width="60" height="60">
                    </div>
                    使用微信、手机QQ扫描二维码，
                    <div>让微信和QQ好友推荐招人更靠谱</div>
                </div>
            </a>
            <?php }?>
        </div>
        
        <div class="yun_prompt_jobtg_box">
            <div class="yun_prompt_jobtg">开启职位推广提升招聘效果</div>
            <ul class="yun_prompt_jobtglist">
                <li>
                    <span class="">职位推荐</span>
                    <em class="">展示在列表右侧黄金位置，带来精确的投递、高质的效果</em>
                    <a id="jobrec" href="javascript:void(0);" onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['row']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['row']->value['rec_time'],'%Y-%m-%d');?>
', 2)">开启</a>
                </li>
                <li>
                    <span class="">职位紧急</span>
                    <em class="">紧急招聘，强烈提升职位曝光度</em>
                    <a id="joburgent" href="javascript:void(0)" onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['row']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['row']->value['urgent_time'],'%Y-%m-%d');?>
', 3)">开启</a>
                </li>
                <li>
                    <span class="">职位置顶</span>
                    <em class="">将职位信息固定排在页面第一页，不会被其他信息挤下去</em>
                    <a id="jobtop" href="javascript:void(0)" onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['row']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['row']->value['xsdate'],'%Y-%m-%d');?>
', 1);">开启</a>
                </li>
                <?php if ($_smarty_tpl->tpl_vars['config']->value['com_job_reserve']==0) {?>
                <li>
                    <span class="">自动刷新</span>
                    <em class="">自动刷新， 职位管理轻松更高效</em>
                    <a id="jobauto" href="javascript:void(0)" onclick="jobPromote('<?php echo $_smarty_tpl->tpl_vars['row']->value['id'];?>
','<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['row']->value['autodate'],'%Y-%m-%d');?>
', 5);">开启</a>
                </li>
				<?php }?>
            </ul>
            <div class="yun_prompt_jobgzh">
                <div class="yun_prompt_jobgzh_img">
                    <img src='<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wx_qcode'];?>
' width='130' height='130'>
                </div>
                关注公众号<br>随时接受简历投递
            </div>
        </div>
        
    </div>
</div>

<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/ueditor/ueditor.config.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/ueditor/ueditor.all.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<link href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui/css/formSelects-v4.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" type="text/css"/>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui/formSelects-v4.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
>
    var urgentjob = '<?php echo $_smarty_tpl->tpl_vars['config']->value['com_urgent'];?>
';
    var online = '<?php echo $_smarty_tpl->tpl_vars['config']->value['com_integral_online'];?>
';
    var pro = '<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
';

    $(document).ready(function() {
        var is_link = '<?php echo $_smarty_tpl->tpl_vars['row']->value['is_link'];?>
';
        var link_id = '<?php echo $_smarty_tpl->tpl_vars['row']->value['link_id'];?>
';

        if (link_id == 0){
            $('#editAddr').hide();
        }
    });
    layui.use(['form', 'layer', 'laydate'], function () {
        var form = layui.form,
            layer = layui.layer,
            laydate = layui.laydate,
            formSelects = layui.formSelects,
            $ = layui.$;

        form.on('select(link_id)', function (data) {
            $('#editAddr').attr('data-id', data.value);
            var sel_provinceid = $('#link_id_'+data.value).attr('data-provinceid');
            var sel_cityid = $('#link_id_'+data.value).attr('data-cityid');
            var sel_threecityid = $('#link_id_'+data.value).attr('data-three_cityid');
            var sel_address = $('#link_id_'+data.value).attr('data-address');
            var sel_x = $('#link_id_'+data.value).attr('data-x');
            var sel_y = $('#link_id_'+data.value).attr('data-y');
            var sel_link_man = $('#link_id_'+data.value).attr('data-link_man');
            var sel_link_mobile = $('#link_id_'+data.value).attr('data-link_moblie');
            var sel_link_phone = $('#link_id_'+data.value).attr('data-link_phone');
            var sel_email = $('#link_id_'+data.value).attr('data-email');
            $('#editAddr').attr('data-provinceid', sel_provinceid);
            $('#editAddr').attr('data-cityid', sel_cityid);
            $('#editAddr').attr('data-three_cityid', sel_threecityid);
            $('#editAddr').attr('data-address', sel_address);
            $('#editAddr').attr('data-x', sel_x);
            $('#editAddr').attr('data-y', sel_y);
            $('#editAddr').attr('data-link_man', sel_link_man);
            $('#editAddr').attr('data-link_moblie', sel_link_mobile);
            $('#editAddr').attr('data-link_phone', sel_link_phone);
            $('#editAddr').attr('data-email', sel_email);
            if (data.value != '-1'){
                $('#editAddr').show();
            }else{
                $('#editAddr').hide();
            }

            if (data.value > 0) {

                $('#is_link').val(2);
            }else{
                $('#is_link').val(1);
            }
            $("#islink").removeClass('com_job_tel_setlist_zt_cur');



            form.render('select');
        });

        form.on('select(moneytype)', function (data) {
            if (data.value == 1) {
                $("#money_1").show();
                $("#money_2").hide();
            } else {
                $("#money_2").show();
                $("#money_1").hide();
            }
        });

        form.on('switch(type_switch)', function (data) {
            var v = this.checked ? 1 : 2;
            $("#tblink").val(v);
        });

        form.on('checkbox(salary_type)', function (data) {
            if (data.elem.checked) {
                $("#minsalary").attr("disabled", "disabled");
                $("#maxsalary").attr("disabled", "disabled");
                $("#minsalary").val(0);
                $("#maxsalary").val(0);
            } else if (!data.elem.checked) {
                $("#minsalary").removeAttr("disabled", "disabled");
                $("#maxsalary").removeAttr("disabled", "disabled");
                $("#minsalary").val('<?php echo $_smarty_tpl->tpl_vars['row']->value['minsalary'];?>
');
                $("#maxsalary").val('<?php echo $_smarty_tpl->tpl_vars['row']->value['maxsalary'];?>
');
            }
        });
        formSelects.btns('jobclass_search', []);

        formSelects.on('jobclass_search', function (id, vals, val, isAdd, isDisabled) {
            var jobvalue = [];
            vals.forEach(function (item, index) {
                jobvalue.push(item.value);
            })
            $('#job_post').val(jobvalue.join(','));
            confirm_selected_jobclass_itemss(jobvalue.join(","));
        }, true);

        jobSearchReset();
    });

    function jobSearchReset() {

        var formSelects = layui.formSelects,
            jobclassArr = $("#job_post").val() != '' ? $("#job_post").val().split(",") : [],
            jarr = [];

        for (var i = 0; i < jobclassArr.length; i++) {
            jarr.push({"name": jn[jobclassArr[i]], "value": jobclassArr[i], "selected": 'selected'});
        }
        formSelects.data('jobclass_search', 'local', {
            arr: jarr
        });
    }

    //ajax提交表单
    $(function () {

        $('#submitBtn').click(function () {
            var joblock = '<?php echo $_smarty_tpl->tpl_vars['config']->value['joblock'];?>
';
            var jobid = $("#id").val();

            if ($.trim($("#name").val()) == '') {
                layer.msg('职位名称不能为空！', 2, 8);
                return false;
            }
            if ($("#job_post").val() == '') {
                layer.msg('职位类别不能为空！', 2, 8);
                return false;
            }

            var minsalary = $.trim($("#minsalary").val());
            var maxsalary = $.trim($("#maxsalary").val());
            if ($("#salary_type").attr("checked") != 'checked') {
                if (minsalary == '' || minsalary == '0') {
                    layer.msg('请填写薪资待遇！', 2, 8);
                    return false;
                }
                if (maxsalary) {
                    if (parseInt(maxsalary) < parseInt(minsalary)) {
                        layer.msg('最高工资必须大于最低工资！', 2, 8);
                        return false;
                    }else if (parseInt(maxsalary) == parseInt(minsalary)) {
                        layer.msg('最高工资必须大于最低工资，如是固定工资只需填写最低工资！', 2, 8);
                        return false;
                    }
                }
            }
            var zp_num = $.trim($("#zp_num").val());
            if (zp_num == 0 || zp_num == '') {
                layer.msg('请填写招聘人数', 2, 8);
                return false;
            }
            var zp_minage = $.trim($("#zp_minage").val());
            var zp_maxage = $.trim($("#zp_maxage").val());
            if (zp_minage != '' ) {
                if (zp_minage < 16) {
                    layer.msg('法律规定：禁止招收未满16周岁未成年人！', 2, 8);
                    return false;
                } 
                
            }
            if (zp_maxage != '') {
                if (zp_maxage<16 || zp_maxage > 99) {
                    layer.msg('请设置合理的年龄区间！', 2, 8);
                    return false;
                }
            }
            var minage_req = $.trim($("#minage_req").val());
            var maxage_req = $.trim($("#maxage_req").val());
            if (minage_req != '') {
                if (minage_req < 16) {
                    layer.msg('法律规定：禁止招收未满16周岁未成年人！', 2, 8);
                    return false;
                } 
                
            }
            if ( maxage_req != '') {
                if (maxage_req<16 || maxage_req > 99) {
                    layer.msg('请设置合理的年龄区间！', 2, 8);
                    return false;
                }
            }
            var description = editor.getContent();
            if ($.trim(description) == '') {
                layer.msg('职位描述不能为空！', 2, 8);
                return false;
            }

            var link_id = $("#link_id").val();
            if (link_id == '') {
                layer.msg('工作地址不能为空！', 2, 8);
                return false;
            }

            if (joblock != 1 || jobid > 0) {
                loadlayer();
                $('#submitBtn').attr("disabled","disabled");
                $('#myform').submit();
            } else {

                var i = layer.confirm('发布后,职位名称将不可修改是否继续？',
                    {btn: ['继续发布', '取消']},
                    function () {
                        setTimeout(function () {
                            loadlayer();
                            $('#submitBtn').attr("disabled","disabled");
                            $('#myform').submit()
                        }, 0);
                        layer.close(i);
                    },
                    function () {
                        layer.closeAll();
                        return false;
                    }
                );
            }

        })
        //添加福利
        $('.addwelfarebox').click(function () {
            var welfare = $.trim($('#addwelfare').val());
            var error = 0;

            if (welfare.length >= 2 && welfare.length <= 8) {
                //判断信息是否已经存在
                $('.changewelfare').each(function () {
                    var otag = $(this).attr('data-tag');
                    if (welfare == otag) {
                        layer.msg('相同福利已存在，请选择或重新填写！', 2, 8);
                        error = 1;
                    }
                });
                if (error == 0) {
                    $('#addwelfarelist').append('<input name="welfare[]" value="' + welfare + '" checked="checked"  type="checkbox" title="' + welfare + '" data-tag="' + welfare + '" class="changewelfare" lay-skin="primary">');
                    layui.use(['layer', 'form'], function () {
                        var layer = layui.layer
                            , form = layui.form
                            , $ = layui.$;
                        form.render('checkbox');
                    });
                }
                $('#addwelfare').val('');
            } else {
                layer.msg('请输入2-8个福利字符！', 2, 8);
            }
        });

        $('.yun_prompt_jobtgbth_sm').hover(function () {
            $('.yun_prompt_jobtgbth_smbox').toggle();
        });
        //投递要求tab
        $('.jobadd_set_a').each(function () {
            $(this).click(function () {

                $('.jobadd_set_a').removeClass("spshow_yq_div_a_curr") //点击时先移除所以选中样式
                $(this).addClass("spshow_yq_div_a_curr") //当前点击的添加选中样式

                var id = $(this).attr("data-id"); //获取data-id的值
                $('.jobadd_setbox').addClass("none") //点击时每个内容都隐藏
                $('#' + id).removeClass("none") //职业data-id和内容id相同的显示
            })
        })
    });

    function addjob_continue(uid) {
        var gourl = 'index.php?c=jobadd';
        var url = weburl + '/index.php?m=ajax&c=ajax_day_action_check';

        loadlayer();

        $.post(url, {'type': 'jobnum'}, function (data) {

            layer.closeAll('loading');
            data = eval('(' + data + ')');
            if (data.status == -1) {

                layer.msg(data.msg, 2, 8);
            } else if (data.status == 1) {

                var addurl = 'index.php?c=jobadd&act=getJobNum';
                loadlayer();
                $.post(addurl, {uid: uid}, function (data1) {

                    layer.closeAll('loading');
                    if (data1 == 1) {

                        window.location.href = gourl;
                        window.event.returnValue = false;
                        return false;
                    } else if (data1 == 2) {
                        layer.confirm('当前会员套餐可上架职位数已达上限，新发布职位将无法直接上架哦~', function () {
                            window.location.href = gourl;
                            window.event.returnValue = false;
                            return false;
                        });
                    } else if (data1 == 0) {
                        var msg = '会员已到期，您可以<a href="index.php?c=right" style="color:red">购买会员</a>，是否继续？';

                        layer.confirm(msg, function () {
                            window.location.href = "index.php?c=right";
                        });
                    }
                });
            }
        });
    }

    var editor = UE.getEditor('description', {
        toolbars: [['Source', '|', 'Undo', 'Redo', 'Bold', 'italic', 'underline', 'fontborder', 'strikethrough', 'fontfamily', 'fontsize', 'forecolor', 'backcolor', 'removeformat', 'autotypeset', 'pasteplain', '|', 'insertorderedlist', 'insertunorderedlist', 'selectall', 'cleardoc', '|', 'simpleupload', '|', 'indent', '|', 'justifyleft', 'justifycenter', 'justifyright', 'justifyjustify']],
        wordCount: false,
        elementPathEnabled: false,
        initialFrameHeight: 200
    });
<?php echo '</script'; ?>
>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/newAddress.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/jobserver.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
