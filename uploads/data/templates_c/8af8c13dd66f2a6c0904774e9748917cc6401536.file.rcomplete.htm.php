<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:38:03
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/member/user/rcomplete.htm" */ ?>
<?php /*%%SmartyHeaderCode:164418372269e896fb546143-09341915%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '8af8c13dd66f2a6c0904774e9748917cc6401536' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/member/user/rcomplete.htm',
      1 => 1700725935,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '164418372269e896fb546143-09341915',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
    'url' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e896fb55bf34_89966066',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e896fb55bf34_89966066')) {function content_69e896fb55bf34_89966066($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/member/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<div id="yunvue" class="none">
    
<!--有简历的情况-->
    <div class="issue_post_body">
        <div class="Resume_success_card">
            <div class="issue_card_box">
                <div class="issue_box_logo">
                    <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/issue_hint.png" alt="" width="100%" height="100%">
                </div>
                <div class="issue_box_hint">恭喜！简历发布成功</div>
                <div class="issue_box_btns">
                   <div class="box_btns_preview" onclick="navigateTo('<?php echo $_smarty_tpl->tpl_vars['url']->value;?>
')">预览简历</div>
                   <div class="box_btns_seek" onclick="navigateTo('<?php echo smarty_function_url(array('m'=>'wap','c'=>'job'),$_smarty_tpl);?>
')">去找工作</div>
                </div>
            </div>
            <div class="issue_card_matchingrate" v-if="joblist && joblist.length > 0">以下职位与你匹配度很高，投递试试吧</div>
            <div class="issue_card_recommendbox">
                <div class="issue_card_recommend" v-for="(job,jkey) in joblist" :key="jkey" @click="pageTo(job.wapjob_url)">
                    <div class="Posted_card_top">
                        <div class="Posted_card_name">{{job.name}}</div>
                        <div class="Posted_card_pay">{{job.job_salary}}</div>
                    </div>
                    <div class="Posted_card_cen">
                        <ul>
                            <li v-if="job.citystr">{{job.citystr}}</li>
                            <li v-if="job.job_edu">{{job.job_edu}}学历</li>
                            <li v-if="job.job_exp">{{job.job_exp}}经验</li>
                        </ul>
                    </div>
                    <div class="Posted_card_bom">
                        <div class="Posted_bom_box">
                            <div class="Posted_box_logo">
                                <img :src="job.com_logo_n" alt="" width="100%" height="100%">
                            </div>
                            <div class="Posted_box_name">{{job.com_name}}</div>
                        </div>
                        <div class="Posted_bom_time">{{job.lastupdate_n}}</div>
                    </div>
                </div>
            </div>
        </div>
    </div>

</div>
<?php echo '<script'; ?>
 type="text/javascript">
    var eid = '<?php echo $_GET['id'];?>
';
    var yunvue =  new Vue({
        el:"#yunvue",
        data:{
           eid:eid,
           joblist: []
        },
        created() {
            this.getExpect();
        },
        methods: {
            getExpect: function(){
                var that = this;
                showLoading();
                $.post('<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'resume','c'=>'rcomplete'),$_smarty_tpl);?>
', {eid: that.eid}, function(data){
                    hideLoading();
                    if (data.error==1) {                        
                        that.joblist = data.data;
                    } 
                    $("#yunvue").css('display', 'block');
                });
                            
            },
            pageTo:function(url){
                window.location.href = url;
            }
        }
    });
<?php echo '</script'; ?>
>
</body>
</html><?php }} ?>
