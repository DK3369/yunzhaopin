<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:41:07
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/member/user/index.htm" */ ?>
<?php /*%%SmartyHeaderCode:123635849969e897b358fe46-87419023%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '9b470bb3784ed6237250aa1d7016edb879bb4803' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/member/user/index.htm',
      1 => 1708477840,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '123635849969e897b358fe46-87419023',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
    'config' => 0,
    'lunbo' => 0,
    'isweixin' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e897b35b3c93_62472249',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e897b35b3c93_62472249')) {function content_69e897b35b3c93_62472249($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/member/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<link href="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/swiper/swiper.min.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" />
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/swiper/swiper.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>

<div id="yunvue" class="none">
    <!-- 页面头部 -->
	
    <div class="userheader">
      <div class="userheader_nav">
            <div class="userheader_nav_calendar">
                <img v-if="info.signstate==1" src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/comtop2.png" onclick="signok();" alt=""
                    width="100%" height="100%">
                <img v-else src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/comtop22.png" onclick="sign();" alt="" width="100%"
                    height="100%">
            </div>
            <div class="userheader_nav_set" onclick="navigateTo('index.php?c=set')">
                <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/comtop4.png" alt="" width="100%" height="100%">
            </div>
        </div>  
        <!-- 头像级名称简历完整度 -->
        <div class="userheader_datum userheaderToubuds">
            <div class="userheader_datum_logo" onclick="navigateTo('index.php?c=photo')">
                <img :src="info.photo" alt="" width="100%" height="100%">
            </div>
            <div class="userheader_datum_left" onclick="navigateTo('index.php?c=info')">
                <div class="userheader_datum_job_name">
                    <i>{{info.name ? info.name : info.username}}</i>
                    <div class="userheader_datum_job_name_number" v-if="info.rnums != 0">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/dskke.png" alt="">
                        <span>{{info.integrity}}%</span>
                    </div>
                </div>
                <!--提示完善简历-->
                <div class="userheader_datum_job_state">
                    <div v-if="info.exp_n && info.edu_n && info.age" class="userheader_datum_job_data">
                        {{info.exp_n}}经验/{{info.edu_n}}学历/{{info.age}}岁
                    </div>
                    <div v-else class="userheader_datum_job_data"> 编辑基本信息</div>
                </div>
            </div>
            <div class="userheader_datum_right"
                :onclick="info.rnums != 0 ? 'navigateTo(\'index.php?c=resume\')' : 'navigateTo(\'index.php?c=addresume\')'">
              
                <div>
                    <div class="userheader_datum_right_word">
                        <span>{{info.rnums != 0 ? '编辑简历':'创建简历'}}</span>
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/comtop1.png" alt="">
                    </div>
                    <!-- <div class="userheader_datum_right_nav">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/me_next_icon.png" alt="" width="100%" height="100%">
                    </div> -->
                </div>

            </div>
        </div>
        <!-- 公司参数 -->
        <div class="userparticulars">
            <ul>
                <li onclick="navigateTo('index.php?c=invite')">
                    <i class="userparticulars_number">{{info.yqnum}}</i>
                    <i class="userparticulars_word">面试通知</i>
                </li>
                <li onclick="navigateTo('index.php?c=sq')">
                    <i class="userparticulars_number">{{info.sq_jobnum}}</i>
                    <i class="userparticulars_word">已投简历</i>
                </li>
                <li onclick="navigateTo('index.php?c=collect')">
                    <i class="userparticulars_number">{{info.fav_jobnum}}</i>
                    <i class="userparticulars_word">收藏/关注</i>
                </li>
                <li onclick="navigateTo('index.php?c=look')">
                    <i class="userparticulars_number">{{info.looknum}}</i>
                    <i class="userparticulars_word">浏览</i>
                </li>
            </ul>
        </div>

    </div>
    <div class="heiseVipDao">
        <!-- 黑色导航条 -->
        <div class="vip_nav" v-if="info.wstitle&&info.wsts==1">
            <div class="vip_nav_img">
                <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/inform.png" alt="" width="100%" height="100%">
            </div>
            <i class="vip_nav_word">简历缺少{{info.wstitle}}，获得面试几率较低</i>
            <div class="vip_nav_remind" :onclick="'navigateTo(\''+info.wswapurl+'\')'">去完善</div>
        </div>
        <div class="vip_nav" v-if="info.wstitle&&info.wsts==2">
            <div class="vip_nav_img">
                <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/inform.png" alt="" width="100%" height="100%">
            </div>
            <i class="vip_nav_word">完善{{info.wstitle}}，更能获得企业的青睐</i>
            <div class="vip_nav_remind" :onclick="'navigateTo(\''+info.wswapurl+'\')'">去完善</div>
        </div>
        <div class="vip_nav" v-if="info.wstitle&&info.wsts==3">
            <div class="vip_nav_img">
                <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/inform.png" alt="" width="100%" height="100%">
            </div>
            <i class="vip_nav_word" v-if="info.status==2">简历已隐藏，企业无法主动发现你</i>
            <i class="vip_nav_word" v-if="info.status==3">简历仅投递企业可见</i>
            <div class="vip_nav_remind" :onclick="'navigateTo(\''+info.wswapurl+'\')'">取消隐藏</div>
        </div>
    </div>
    <!-- 页面主体部分 -->
    <div class="min_body">
        <div class="user_nav_fast" style="height: auto;" v-if="info.expect_state == '3'">
            <div style=" padding: .22rem">
                <div style="font-weight: 600; font-size: .4rem; color: #181818">温馨提示</div>
                <div style="font-size: .32rem; padding: .22rem 0;">
                    <div>简历审核未通过，请重新完善简历，联系客服快速审核</div>
                    <div style="margin-top: 5px;" v-if="info.statusbody">原因：{{info.statusbody}}</div>
                </div>
            </div>
        </div>
        <!-- 四个功能按钮 -->
        <div class="user_nav_fast mt10">
            <ul>
                <li @click="topCheck">
                    <div class=" user_nav_fast_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/jobhunter_top.png" alt="" width="100%" height="100%">
                    </div>
                    <i class=" user_nav_fast_word" v-if="info.expect_state=='1' && info.expect_top=='1' && info.topdatetime > 0">
                        已置顶
                    </i>
                    <i class=" user_nav_fast_word" v-else>
                        置顶简历
                    </i>
                </li>
                <li @click="refreshResume">
                    <div class=" user_nav_fast_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/jobhunter_refresh.png" alt="" width="100%" height="100%">
                    </div>
                    <i class=" user_nav_fast_word">刷新简历</i>
                </li>
                <li @click="previewResume">
                    <div class=" user_nav_fast_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/jobhunter_preview.png" alt="" width="100%" height="100%">
                    </div>
                    <i class=" user_nav_fast_word">预览简历</i>
                </li>
                <li @click="pageTo('index.php?c=likejob&id='+info.def_job)">
                    <div class=" user_nav_fast_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/userpp.png" alt="" width="100%" height="100%">
                    </div>
                    <i class=" user_nav_fast_word">职位速配</i>
                </li>
            </ul>
        </div>
        <!-- 广告位开始 -->
        <div class="wap_useradvertisement" style="display: none;">
            <div class="swiper-container" id="imgswiper">
                <div class="swiper-wrapper" >
                    <?php  $_smarty_tpl->tpl_vars["lunbo"] = new Smarty_Variable; $_smarty_tpl->tpl_vars["lunbo"]->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"500","item"=>"\"lunbo\"","key"=>"“key“","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[500];if(is_array($add_arr) && !empty($add_arr)){
				$i=0;$limit = 0;$length = 0;
				foreach($add_arr as $key=>$value){
					if($config['did']){
						if(($value['did']==$config['did']|| $value['did']==-1)&&$value['start']<time()&&$value['end']>time()){
							if($i>0 && $limit==$i){
								break;
							}
							if($length>0){
								$value['name'] = mb_substr($value['name'],0,$length);
							}
							if($paramer['type']!=""){
								if($paramer['type'] == $value['type']){
									$AdArr[] = $value;
								}
							}else{
								$AdArr[] = $value;
							}
							$i++;
						}
						
					}else{
						if(($value['did']==-1 || !$value['did']) && $value['start']<time()&&$value['end']>time()){
							if($i>0 && $limit==$i){
								break;
							}
							if($length>0){
								$value['name'] = mb_substr($value['name'],0,$length);
							}
							if($paramer['type']!=""){
								if($paramer['type'] == $value['type']){
									$AdArr[] = $value;
								}
							}else{
								$AdArr[] = $value;
							}
							$i++;
						}
						
					}
				}
				if (isset($attr['random']) && $attr['random'] && count($AdArr) > $attr['random']) {
			        $temp = [];
			        $random_keys = array_rand($AdArr, $attr['random']);

			        if($attr['random'] == 1) {
			            $temp[] = $AdArr[$random_keys];
			        } else {
			            foreach ($AdArr as $key => $value) {
			                if (in_array($key, $random_keys)) {
			                    $temp[$key] = $value;
			                }
			            }
			        }
			        $AdArr = $temp;
		        }
			}$AdArr = $AdArr; if (!is_array($AdArr) && !is_object($AdArr)) { settype($AdArr, 'array');}
foreach ($AdArr as $_smarty_tpl->tpl_vars["lunbo"]->key => $_smarty_tpl->tpl_vars["lunbo"]->value) {
$_smarty_tpl->tpl_vars["lunbo"]->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars["lunbo"]->key;
?>
                    <div class="swiper-slide" style="transform:translate3d(0,0,0);">
                        <a href="<?php echo $_smarty_tpl->tpl_vars['lunbo']->value['src'];?>
">
                            <img src="<?php echo $_smarty_tpl->tpl_vars['lunbo']->value['pic'];?>
"  style="border-radius: 6px;"/>
                        </a>
                    </div>
                    <?php } ?>
                </div>
            </div>            
        </div>
        <!-- 结束 -->
        <!-- 功能任务区域 -->
        <div class="taskbar_box">
            <div class="taskbar_enterprise"
                :onclick="info.rnums != 0 ? 'navigateTo(\'index.php?c=resume\')' : 'navigateTo(\'index.php?c=addresume\')'">
                <div class="taskbar_datum">
                    <div class="taskbar_datum_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/resume_index.png" alt="" width="100%" height="100%">
                    </div>
                    <div class="taskbar_datum_word">我的简历</div>
                </div>
                <div class="taskbar_nav">
                    <div class="taskbar_nav_word" v-if="info.rnums != 0">正在找工作</div>
                    <div class="taskbar_nav_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/my_more.png" alt="" width="100%" height="100%">
                    </div>
                </div>
            </div>
            <div class="taskbar_enterprise" onclick="navigateTo('index.php?c=privacy')">
                <div class="taskbar_datum">
                    <div class="taskbar_datum_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/ys.png" alt="" width="100%" height="100%">
                    </div>
                    <div class="taskbar_datum_word">隐私设置</div>
                </div>
                <div class="taskbar_nav">
                    <div class="taskbar_nav_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/my_more.png" alt="" width="100%" height="100%">
                    </div>
                </div>
            </div>
            <div class="taskbar_enterprise" onclick="navigateTo('index.php?c=spview')" v-if="config.spview == 1">
                <div class="taskbar_datum">
                    <div class="taskbar_datum_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/video.png" alt="" width="100%" height="100%">
                    </div>
                    <div class="taskbar_datum_word">视频面试</div>
                </div>
                <div class="taskbar_nav">
                    <div class="taskbar_nav_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/my_more.png" alt="" width="100%" height="100%">
                    </div>
                </div>
            </div>
            <div class="taskbar_enterprise" onclick="navigateTo('index.php?c=xjhLive')" v-if="config.xjhlive == 1">
                <div class="taskbar_datum">
                    <div class="taskbar_datum_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/Live_speech.png" alt="" width="100%" height="100%">
                    </div>
                    <div class="taskbar_datum_word">直播宣讲会</div>
                </div>
                <div class="taskbar_nav">
                    <div class="taskbar_nav_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/my_more.png" alt="" width="100%" height="100%">
                    </div>
                </div>
            </div>
            <span onclick="navigateTo('index.php?c=otherservice')">
                <div class="taskbar_enterprise">
                    <div class="taskbar_datum">
                        <div class="taskbar_datum_img">
                            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/job_training.png" alt="" width="100%" height="100%">
                        </div>
                        <div class="taskbar_datum_word">其他服务</div>
                    </div>
                    <div class="taskbar_nav">
                        <div class="taskbar_nav_word">{{config.part == 1 ? '兼职 ' : ''}}{{config.ask == 1 ? '问答 ' :
                            ''}}{{config.reward == 1 ? '赏金职位 ' : ''}}等</div>
                        <div class="taskbar_nav_img">
                            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/my_more.png" alt="" width="100%" height="100%">
                        </div>
                    </div>
                </div>
            </span>
            <div class="taskbar_enterprise" onclick="navigateTo('index.php?c=finance')">
                <div class="taskbar_datum">
                    <div class="taskbar_datum_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/financial_management.png" alt="" width="100%"
                            height="100%">
                    </div>
                    <div class="taskbar_datum_word">财务管理</div>
                </div>
                <div class="taskbar_nav">
                    <div class="taskbar_nav_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/my_more.png" alt="" width="100%" height="100%">
                    </div>
                </div>
            </div>
            <div class="taskbar_enterprise" onclick="navigateTo('index.php?c=set')">
                <div class="taskbar_datum">
                    <div class="taskbar_datum_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/sz.png" alt="" width="100%" height="100%">
                    </div>
                    <div class="taskbar_datum_word">账户设置</div>
                </div>
                <div class="taskbar_nav">
                    <div class="taskbar_nav_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/my_more.png" alt="" width="100%" height="100%">
                    </div>
                </div>
            </div>
			<div class="taskbar_enterprise_last" onclick="navigateTo('<?php echo smarty_function_url(array('m'=>'wap','c'=>'advice'),$_smarty_tpl);?>
')">
                <div class="taskbar_datum">
                    <div class="taskbar_datum_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/fk.png" alt="" width="100%" height="100%">
                    </div>
                    <div class="taskbar_datum_word">意见反馈</div>
                </div>
                <div class="taskbar_nav">
                    <div class="taskbar_nav_img">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/my_more.png" alt="" width="100%" height="100%">
                    </div>
                </div>
            </div>
        </div>

        <div class="companyDatapage">
            <div class="companyDataTell" v-if="webtel">
                <span>客服电话  {{webtel}} &nbsp; &nbsp;<span v-if="worktime">({{worktime}})</span></span>
            </div>
            <div class="companyDatazhiao">
                <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'rlzy'),$_smarty_tpl);?>
">人力资源服务许可证</a>
                <span></span>
                <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'yyzz'),$_smarty_tpl);?>
">营业执照</a>
            </div>
        </div>
    </div>
    <van-popup v-model="topResumeShow" :closeable="true" :close-on-click-overlay="false" position="bottom" :style="{ height: '45%', padding: '0.1rem 0 0.5rem 0' }">
        <div class="entrust_box">
            <div class="entrust_box_title">简历置顶</div>
            <div class="entrust_box_now">
                <div class="entrust_box_text">当前简历</div>
                <div class="entrust_box_text">{{info.resume_name}}</div>
            </div>
            <div class="entrust_box_now" v-if="day>0">
                <div class="entrust_box_text">当前简历置顶剩余{{day}}天</div>
            </div>
            <div class="entrust_box_now">
                <div class="entrust_box_text">置顶天数</div>
                <div class="entrust_box_text"><input type="number" maxlength="2" :value="days" @input="dayInput" oninput="value=value.replace(/^0|[\^d]/g,'')" placeholder="请设置置顶天数" style="text-align: right;" /></div>
            </div>
            <div class="zd_pay_box">
                <div v-if="fktype.fkwx" class="zd_pay_list">
                    <div class="zd_pay_icon">
                        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/dredge_wx.png" alt="" width="30" height="30">
                    </div>
                    <div class="zd_pay_name">微信支付</div>
                    <div @click="changefk('fkwx')" class="zd_pay_xz">
                        <div v-if="fk=='fkwx'">
                            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/dredge_affirm.png" alt="" width="100%" height="100%">
                        </div>
                        <div v-if="fk!='fkwx'">
                            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/dredge_To_confirm.png" alt="" width="100%" height="100%">
                        </div>
                    </div>
                </div>
                <div v-if="fktype.fkal && isweixin == 2" class="zd_pay_list">
                    <div class="zd_pay_icon"> <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/dredge_zfb.png" alt="" width="30" height="30"></div>
                    <div class="zd_pay_name">支付宝支付</div>
                    <div @click="changefk('fkal')" class="zd_pay_xz">
                        <div v-if="fk=='fkal'" class=" ">
                            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/dredge_affirm.png" alt="" width="100%" height="100%">
                        </div>
                        <div v-if="fk!='fkal'" class=" ">
                            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/dredge_To_confirm.png" alt="" width="100%" height="100%">
                        </div>
                    </div>
                </div>
            </div>
            <div class="zd_pay_fk">应付金额: <span class="zd_pay_fk_dw">￥<span class="zd_pay_fk_n">{{order_price}}</span></span>
                <div @click="buyResumeTop" class="zd_pay_bth">立即购买</div>
            </div>
        </div>
    </van-popup>
	<!-- 关注公众号弹框 --> 
	<van-popup v-model="gzhshow" :close-on-click-overlay="gzhclose" :closeable="gzhclose" round >
		<div class="gzh_gzbox">
			<div class="gzh_gzbox_n">关注公众号</div>
				<img :src="gzhurl" @touchstart="gzhstart" @touchend="gzhend">
			<div class="gzh_gzbox_p">长按识别二维码</div>
			<div class="gzh_gzbox_p">随时随地查找好工作</div>
		</div>
    </van-popup>
</div>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/user.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
> 

var loopVal = null,
    isweixin = '<?php echo $_smarty_tpl->tpl_vars['isweixin']->value;?>
';
var yunvue = new Vue({
    el: '#yunvue',
    data() {
        return {
            info: {},
            config: {},
            day: '',
            days: '',
            fktype: {},
            fk: '',
            topResumeShow: false,
            order_price: 0,
            price: 0,
			gzhshow: false,
			gzhurl: '',
			gzhclose: false,

            isweixin: isweixin ? isweixin : 2,
            webtel: '',
            worktime: '',
        }

    },
    created() {
        this.getInfo();
        if ($("#imgswiper").has('a').length > 0) { 
            $(".wap_useradvertisement").css('display', 'block');
        }
    },
    methods: {
        getInfo: function() {
            showLoading();
            var self = this;
            $.post('<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'index','c'=>'index'),$_smarty_tpl);?>
', { rand: Math.random() }, function(res) {
                hideLoading();
                $("#yunvue").css('display', 'block');
                self.info = res.data;
                self.config = res.data.config;
                self.fktype = res.data.fktype;
                self.day = res.data.top_day ? res.data.top_day : 0;
				self.gzhurl = res.data.gzhurl ? res.data.gzhurl : '';
                self.webtel = res.data.webtel;
                self.worktime = res.data.worktime;
                if (self.fktype.fkwx) {
                    self.changefk('fkwx');
                } else if (self.fktype.fkal) {
                    self.changefk('fkal');
                }
				if ($("#imgswiper").has('a').length > 0) {
           		 	var mySwiper = new Swiper('#imgswiper', {
           	            direction: 'horizontal',
           	            autoplay: {
           	                disableOnInteraction: false
           	            },
           	            loop: true,
           	        });
            	}
            }, 'json');
        },
        topCheck: function() {
            var that = this;
            if (!that.info.def_job) {
                showToast('请先创建简历');
            } else {
                if (that.info.expect_state != '1') {
                    showToast('简历还未审核，无法置顶');
                } else {
                    showLoading();
                    var eid = that.info.def_job;
                    $.post("<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'resume','c'=>'topCheck'),$_smarty_tpl);?>
", {
                        eid: eid
                    }, function(data) {
                        hideLoading();
                        if (data.msg) {
                            showToast(data.msg);
                        } else {
                            if (that.fktype.fkwx || that.fktype.fkal) {
                                that.topResumeShow = true;
                            } else {
                                showToast('网站已关闭支付接口，请联系管理员');
                            }
                        }
                    }, 'json');
                }
            }
        },
        refreshResume: function() {
            if (!this.info.def_job) {
                showToast('请先创建简历');
            } else {
                showLoading();
                $.post("<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'resume','c'=>'refresh_resume'),$_smarty_tpl);?>
", {
                    id: this.info.def_job,provider: 'wap'
                }, function(data) {
                    hideLoading();
                    if (data.error == 1) {
                        showToast('刷新成功');
                    } else {
                        showToast(data.msg);
                    }
                });
            }
        },
        previewResume: function() {
            if (!this.info.def_job) {
                showToast('请先创建简历');
            } else {
                window.location.href = wapurl + '?c=resume&a=show&id=' + this.info.def_job;
            }
        },
        buyResumeTop: function() {
            var that = this;
            if (that.days == '') {
                return showToast('请输入置顶天数');
            }
            if (that.fk == '') {
                return showToast('请选择支付方式');
            }
            let formData = {
                id: this.info.def_job,
                rdays: that.days,
                fktype: that.fk,
                type: 'wap',
                server: 'zdresume'
            };
            showLoading('购买中');
            $.post('<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'finance','c'=>'setServer'),$_smarty_tpl);?>
', formData, function(data) {
                hideLoading();
                if (data.msg == 'ok') {
                    let res = data.data;
                    wxpayShow = true;
                    if (res.goto) {
                        window.location.href = res.url;
                    } else {

                        showToast('置顶成功', 2, function() {
                            location.reload();
                        });
                    }

                } else {
                    showToast(data.msg);
                }

            }, 'json');
        },
        //乘
        accMul: function(arg1, arg2) {
            var m = 0,
                s1 = arg1.toString(),
                s2 = arg2.toString();
            try {
                m += s1.split(".")[1].length
            } catch (e) {}
            try {
                m += s2.split(".")[1].length
            } catch (e) {}

            return Number(s1.replace(".", "")) * Number(s2.replace(".", "")) / Math.pow(10, m);
        },
        changefk: function(e) {
            this.fk = e;
        },
        dayInput: function(e) {
            var that = this;
            if (e.target.value != '') {
                that.days = e.target.value;
                that.order_price = that.accMul(parseInt(e.target.value), parseFloat(that.config.top_price));
            } else {
                that.days = '';
                that.order_price = 0;
            }
        },
        pageTo: function(url){
            window.location.href = url;
        },
		gzhstart: function(){
			if(loopVal){
				 //再次清空定时器，防止重复注册定时器
				clearTimeout(loopVal);
			}
			var self = this;
			loopVal = setTimeout(function() {
				self.gzhclose = true;
				gzhbd();
			}.bind(this), 1000);
		},
		gzhend: function(){
			 if(loopVal){
				clearTimeout(loopVal);
				loopVal = null;
			}
		}
    }
});
 
<?php echo '</script'; ?>
>
<?php if ($_smarty_tpl->tpl_vars['isweixin']->value&&$_smarty_tpl->tpl_vars['config']->value['user_gzgzh']=='1') {?>
<?php echo '<script'; ?>
>
	var gzhConfirm = false,
		gzhBox = false;
	$(function(){
		isgzh()
	})
	function isgzh(){
		$.post('index.php?c=isgzh', { rand: Math.random() }, function(res) {
			if (res.subscribe == 0 && !gzhConfirm) {
				gzhBox = true;
				yunvue.$data.gzhshow = true;
			}else if(res.subscribe == 2 && !gzhConfirm) {
				if(!gzhBox){
					// 页面首次查询，先弹出二维码弹窗
					yunvue.$data.gzhshow = true;
					gzhBox = true;
				}else{
					gzhConfirm = true;
					yunvue.$data.gzhshow = false;
					showConfirm('本账号绑定的微信号，不是当前微信号，是否要换绑为当前微信号？', function() {
						hbwx();
					},'否','是');
				}
			}else if(res.subscribe == 1 && gzhBox) {
				window.location.reload();
			}
		}, 'json');
	}
	function gzhbd(){
		setInterval(function(){
			isgzh();
		},3000);
	}
	function hbwx(){
		showLoading();
		$.post('<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'index','c'=>'hbwx'),$_smarty_tpl);?>
', { rand: Math.random() }, function(res) {
			hideLoading();
			if(!res.error){
				showToast(res.msg);
			}
		}, 'json');
	}
<?php echo '</script'; ?>
>
<?php }?>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>


<?php }} ?>
