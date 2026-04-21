<?php
/* *
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
*/
class index_controller extends wap_controller{

	function waptpl($tpname)
	{
		$this->yuntpl(array('wap/member/user/'.$tpname));
	}

	function get_user()
	{
		$ResumeM   =  $this->MODEL('resume');
		$isresume  =  $ResumeM->getResumeInfo(array('uid'=>$this->uid));

		if (! $isresume['name']) {

		    $this->ACT_msg_wap(Url('wap', array('c' => 'info'), 'member'), '请先完善个人资料', 2, 3);
		}
	}
    //会员中心
	function index_action()
	{

		$backurl  =  Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
		$resumeM		=	$this -> MODEL('resume');

		//判断我是否有简历
		$eData    =   array(
		    'field'   => '`lastupdate`,`jobstatus`,`id`,`name`'
		);
		$rlist  =  $resumeM -> getExpectByUid($this->uid,$eData);
        if($this -> config['resume_sx']==1  && $_COOKIE['amtype'] != '1'){//登录自动简历刷新,在后台配置、管理员登录的，不需要刷新

		    if($rlist['id']){
		        
		        $resumeM -> upInfo(array('id'=>$rlist['id'],'uid'=>$this->uid),array('eData'=>array('lastupdate'=>time())));
		        
		        $resumeM -> upResumeInfo(array('uid'=>$this->uid),array('rData'=>array('lastupdate'=>time()), 'port' => 2));
		    }
		}
		$this->yunset('membernav', 1);
		$this->waptpl('index');
	}
	// 判断用户有没有关注公众号
	function isgzh_action(){
	    
	    $subscribe = 0;
	    $wxloginid = 'weixin_gzhid_'. $this->uid;
	    
	    $userInfoM  =  $this->MODEL('userinfo');
	    $member     =  $userInfoM->getInfo(array('uid'=>$this->uid),array('field'=>'`subscribe`,`wxid`'));
	    // 查询识别记录
	    $weixinM = $this->MODEL('weixin');
	    $log = $weixinM->getWxQrcode(array('wxloginid'=>$wxloginid, 'status'=>2, 'time'=>array('>', strtotime('today')), 'orderby'=>array('id,DESC')),array('field'=>'wxid'));
	    
	    if(!empty($member['wxid'])){
	        // 账号已绑定微信公众号
	        if (!empty($log['wxid']) && $member['wxid'] != $log['wxid']){
	            // 绑定的微信不是识别二维码的微信，提示用户是否要换绑
	            $subscribe = 2;
	        }else{
	            $res = $weixinM->getWxUser($member['wxid']);
                if ($member['subscribe'] == 2){
                    $this->obj->update_once('member', array('subscribe' => $res['subscribe']), array('uid' => $this->uid));
                }
	            if (isset($res['subscribe'])){
	                $subscribe = $res['subscribe'];
	            }
	        }
	    }
	    echo json_encode(array('subscribe'=>$subscribe));
	}
    // 不常用的服务，例如问答等
    function otherservice_action(){

        $backurl  =  Url('wap',array(),'member');
        $this->yunset('backurl',$backurl);
        $this->yunset('headertitle','其他服务');
        $this->waptpl('other_service');
    }
	//上传形象照
	function photo_action(){

	    $backurl  =  Url('wap',array(),'member');
	    $this->yunset('backurl',$backurl);
	    
	    $this->yunset('headertitle',"上传形象照");
	    $this->waptpl('photo');
	}
    //申请的职位
	function sq_action(){

        $backurl	=	Url('wap',array(),'member');
        $this->yunset('backurl',$backurl);
		$this->yunset('headertitle',"申请的职位");
		$this->waptpl('sq');
	}

    function partapply_action()
    {

        $backurl = Url('wap', array(), 'member');
        $this->yunset('backurl', $backurl);
        $this->yunset('headertitle', "兼职管理");
        $this->waptpl('partapply');
    }

	function collect_action(){

		$backurl	=	Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('headertitle',"收藏/关注");
		$this->waptpl('collect');
	}
	
	function password_action(){

		$this->yunset('backurl',Url('wap',array('c'=>'safe'),'member'));

		$this->yunset('headertitle',"密码设置");
		$this->waptpl('password');
	}
	function invitecont_action(){

		$this -> yunset('headertitle',"面试详情");
		$this -> waptpl('invitecont');
	}

	function invite_action(){

        $backurl	=	Url('wap',array(),'member');
        $this->yunset('backurl',$backurl);
		$this->yunset('headertitle',"面试通知");
		$this->waptpl('invite');
	}
    // 谁看了我/我的足迹
	function look_action(){

        $backurl	=	Url('wap',array(),'member');
        $this->yunset('backurl',$backurl);
		$this->yunset('headertitle',"记录");
		$this->waptpl('look');
	}
	// 创建简历
	function addresume_action(){

	    $cacheM	=	$this->MODEL('cache');
	    $cache	=	$cacheM -> GetCache(array('city','job'));
	    
	    $this->yunset($cache);
	    $this->yunset('backurl',Url('wap',array(),'member'));
		$this->waptpl('addresume');
	}
    // 简历附表添加、修改
	function addresumeson_action(){

		switch($_GET['type']){

			case 'work':		$headertitle='工作经历';  break;
			case 'edu':			$headertitle='教育经历';  break;
			case 'project':		$headertitle='项目经历';  break;
			case 'training':	$headertitle='培训经历';  break;
			case 'skill':		$headertitle='职业技能';  break;
			case 'other':		$headertitle='其他信息';  break;
			case 'desc':		$headertitle='自我评价';  break;
			case 'show':		$headertitle='作品案例';  break;
			case 'doc':	        $headertitle='粘贴简历';  break;
		}
		$this->yunset('headertitle',$headertitle);
		$this->waptpl('addresumeson');
	}
	// 基本信息页面
	function info_action(){
		$this->yunset('headertitle',"基本信息");
 		$this->waptpl('info');
	}
	
    function addexpect_action()
    {
        $cacheM	=	$this->MODEL('cache');
        $cache	=	$cacheM -> GetCache(array('city','job'));
        
        $this -> yunset($cache);
        
		$this->yunset('headertitle','意向职位修改');
		$this->waptpl('addexpect');
	}
	function rcomplete_action(){
		$this->yunset('headertitle',"发布成功");
		$backurl	=	Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
        $this->yunset('url',Url('wap',array('c'=>'resume','a'=>'show','id'=>$_GET['id'])));
		$this->waptpl('rcomplete');
	}
	function resume_action(){

		$backurl		=Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('headertitle',"我的简历");
		$this->waptpl('resume');
	}
	function optimize_action(){
        $this->yunset('headertitle',"优化简历");
        
        if (isset($_GET['add'])){
            $backurl  =  Url('wap',array(),'member');
            $this->yunset('backurl',$backurl);
        }

        $this->waptpl('optimize');
    }
	// 简历管理。设置顶部隐私显示cookie
	function setPrivacyCookie_action(){
	    $this->cookie->setcookie('privacy', 1, time() + 3600 * 6);
	}

	function binding_action()
	{

		$this->yunset('headertitle',"社交账号绑定");
		$this->yunset("backurl",Url('wap',array('c'=>'ident'),'member'));
		$this->waptpl('binding');
	}
	function idcard_action(){
		$this->yunset('headertitle',"身份证认证");

		$backurl	=	Url('wap',array('c'=>'ident'),'member');
		$this->yunset('backurl',$backurl);
		$this->waptpl('idcard');
	}
	function bindingbox_action(){
		switch($_GET['type']){
			case 'moblie':$headertitle="手机认证";
			break;
			case 'email':$headertitle="邮箱认证";
			break;
		}
		$this->yunset('headertitle',$headertitle);

		$backurl	=	Url('wap',array('c'=>'ident'),'member');
		$this->yunset('backurl',$backurl);

		$this->waptpl('bindingbox');
	}
	function setname_action(){

		$backurl	=	Url('wap',array('c'=>'safe'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('headertitle',"修改用户名");
		$this->waptpl('setname');
	}
	function reward_list_action(){
		$this->yunset('headertitle',"兑换记录");
		if($_GET['back']){
			$backurl		=	Url('wap',array('c'=>'redeem'));
		}else{
			$backurl		=	Url('wap',array('c'=>'finance'),'member');
		}
		$this->yunset('backurl',$backurl);
		$this->waptpl('reward_list');
	}

	function privacy_action(){
		$this->yunset('headertitle',"隐私设置");

		$this->waptpl('privacy');
	}

	function getOrder_action(){

		if($_POST){

		    $M	=	$this->MODEL('userpay');

			$_POST['uid']		=	$this->uid;
			$_POST['usertype']	=	$this->usertype;
			$_POST['did']		=	$this->userdid;

			if($_POST['server']=='zdresume'){

				$return = $M->buyZdresume($_POST);
			}

			if($return['order']['order_id'] && $return['order']['id']){

				$dingdan	= $return['order']['order_id'];
				$price 		= $return['order']['order_price'];

				$_POST['dingdan']		=	$dingdan;
				$_POST['dingdanname']	=	$dingdan;
				$_POST['alimoney']		=	$price;
				$data['msg']			=	"下单成功，请付款！";
				//多种支付方式并存 进行选择
				if($_POST['paytype']=='alipay'){

					$url	=	$this->config['sy_weburl'].'/api/wapalipay/alipayto.php?dingdan='.$dingdan.'&dingdanname='.$dingdan.'&alimoney='.$price;
				}
 				echo json_encode(array(
 				    'error' => 0,
 				    'url'   => $url,
 				    'msg'   =>  '下单成功，请付款！'
 				));
			}else{
			    echo json_encode(array(
			        'error' => 1,
			        'msg' => '提交失败，请重新提交订单！'
			    ));
			}
 		}else{
 		    echo json_encode(array(
 		        'error' => 1,
 		        'msg' => '参数错误，请重试！'
 		    ));
		}
	}

	function pay_action(){
		$this->yunset('headertitle',"充值");
		$this->waptpl('pay');
	}

	function payment_action(){
		$orderM		=	$this->MODEL('companyorder');


		if($this->config['alipay']=='1' &&  $this->config['alipaytype']=='1'){
			$paytype['alipay']	=	'1';
		}

 		if($paytype){
			if($_GET['id']){
				$order	=	$orderM->getInfo(array('id'=>(int)$_GET['id']));
				if(empty($order)){
					$this->ACT_msg_wap($_SERVER['HTTP_REFERER'],"订单不存在！",2,5);
				}elseif($order['order_state']!='1'){
					header("Location:index.php?c=paylog");
				}else{
					$this->yunset("order",$order);
				}
			}

			$this->yunset("paytype",$paytype);

		}else{
			$data['msg']	=	"暂未开通手机支付，请移步至电脑端充值！";
			$data['url']	=	$_SERVER['HTTP_REFERER'];
			$this->yunset("layer",$data);
		}

		$this->get_user();
		$this->yunset('headertitle',"收银台");
		$this->waptpl('payment');
	}
	/**
	 * 生成订单
	 */
	function dingdan_action(){

		$data['price_int']	   =  intval($_POST['price_int']);
		$data['integralid']	   =  intval($_POST['integralid']);
		$data['uid']		   =  $this->uid;
		$data['did']		   =  $this->userdid;
		$data['usertype']	   =  $this->usertype;
		$data['paytype']	   =  $_POST['paytype'];
		$data['type']		   =  'wap';

		$orderM   =  $this->MODEL('companyorder');
		$return   =  $orderM->addComOrder($data);

		//微信支付、支付宝支付，跳转到相应的链接
		if($return['errcode'] == 9 && !empty($return['url'])){

			header('Location: '.$return['url']);exit();
		}else{
			$this->yunset("layer",$return);
		}

		$backurl  =  Url('wap',array(),'member');
		$this->get_user();
		$this->yunset('backurl',$backurl);
		$this->yunset('headertitle',"订单");

		$this->waptpl('pay');
	}

    function paylog_action(){
        $this->yunset('headertitle',"明细");
        $backurl	=	Url('wap',array('c'=>'finance'),'member');
        $this		->	yunset('backurl',$backurl);
        $this->waptpl('paylog');
    }

	function likejob_action(){
		$this		->	yunset('headertitle',"职位速配");

		$this		->	waptpl('likejob');
	}

	function set_action(){
		$this->yunset('headertitle',"账户设置");
		
		$backurl	=	Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
		$this->waptpl('set');
	}

	function sysnews_action(){

		$this->yunset('headertitle',"消息");
		$this->waptpl('sysnews');

	}
	//私信
	function sxnews_action(){
		$this->yunset('headertitle',"系统消息");

		$backurl	=	Url('wap',array('c'=>'sysnews'),'member');
		$this->yunset('backurl',$backurl);
		$this->waptpl('sxnews');
	}

	function commsg_action(){
		$this->yunset('headertitle',"求职咨询");

		$backurl=Url('wap',array('c'=>'sysnews'),'member');
		$this->yunset('backurl',$backurl);
		$this->waptpl('commsg');
	}
	function finance_action(){

		$this->yunset('headertitle',"财务管理");
        $reg_url = Url('wap',array('c'=>'register','uid'=>$this->uid));
        $this->yunset('reg_url', $reg_url);
		$backurl	=	Url('wap',array(),'member');

		$this->yunset('backurl',$backurl);
		$this->waptpl('finance');
	}
	function integral_action(){
        $this->yunset('headertitle',"全部任务");
        $reg_url = Url('wap',array('c'=>'register','uid'=>$this->uid));
        $this->yunset('reg_url', $reg_url);
        $this->waptpl('alltask');
    }

    function blacklist_action()
    {

        $backurl	=	Url('wap',array(),'member');

        $this->yunset('backurl',$backurl);
        $this->yunset('headertitle', '屏蔽企业');
        $this->waptpl('blacklist');
    }
	function blacklistadd_action(){

		$this->yunset('headertitle',"添加屏蔽");
        $backurl	=	Url('wap',array('c'=>'blacklist'),'member');

        $this->yunset('backurl',$backurl);
		$this->waptpl('blacklistadd');
	}



	function getStatis($type=''){
		$statisM  	= 	$this->MODEL('statis');

		$statis		= 	$statisM->getInfo($this->uid,array('usertype'=>1));

		if($type=='finance'){
			$orderM		=	$this->MODEL('companyorder');
			$orders		=	$orderM->getPayList(array('com_id'=>$this->uid, 'usertype' =>$this->usertype, 'type'=>'1'),array('field'=>'`order_price`'));
            $allprice   =   0;
            foreach($orders as $key=>$val){
				$allprice	+=	$val['order_price'];
			}
			if($allprice<0){
				$statis['allprice']		=	number_format(str_replace('-','', $allprice));
			}else{
				$statis['allprice']		=	'0';
			}

			$statis['freeze'] = sprintf("%.2f", $statis['freeze']);
		}

		if($type=='loglist'){
			$statis['freeze'] = sprintf("%.2f", $statis['freeze']);
		}

		$this->yunset("statis",$statis);
	}

	function transfer_action(){
		$this->yunset('headertitle',"账户分离");
		$this->waptpl('transfer');
	}


	function logout_action()
    {

        $backurl	=	Url('wap',array('c' => 'safe'),'member');
        $this->yunset('backurl',$backurl);

        $this->yunset('headertitle',"账号注销");
        $this->waptpl('logout');
    }
    /****************** gengzs start ************************/
    public function ident_action(){

        $backurl=Url('wap',array('c'=>'set'),'member');

        $this->yunset('backurl',$backurl);

        $this->yunset('header_title',"认证与绑定");

        $this->waptpl('ident');
    }
    public function safe_action(){

        $backurl=Url('wap',array('c'=>'set'),'member');
        $this->yunset('backurl',$backurl);

        $this->yunset('header_title',"账号与安全");

        $this->waptpl('safe');
    }
    /****************** gengzs  end  ************************/
}
?>