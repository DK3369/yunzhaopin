<?php

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

		    $this->ACT_msg_wap(Url('wap', array('c' => 'info'), 'member'), yun_auto_t('请先完善个人资料'), 2, 3);
		}
	}

	function index_action()
	{

		$backurl  =  Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
		$resumeM		=	$this -> MODEL('resume');


		$eData    =   array(
		    'field'   => '`lastupdate`,`jobstatus`,`id`,`name`'
		);
		$rlist  =  $resumeM -> getExpectByUid($this->uid,$eData);
        if($this -> config['resume_sx']==1  && $_COOKIE['amtype'] != '1'){

		    if($rlist['id']){
		        
		        $resumeM -> upInfo(array('id'=>$rlist['id'],'uid'=>$this->uid),array('eData'=>array('lastupdate'=>time())));
		        
		        $resumeM -> upResumeInfo(array('uid'=>$this->uid),array('rData'=>array('lastupdate'=>time()), 'port' => 2));
		    }
		}
		$this->yunset('membernav', 1);
		$this->waptpl('index');
	}

	function isgzh_action(){
	    
	    $subscribe = 0;
	    $wxloginid = 'weixin_gzhid_'. $this->uid;
	    
	    $userInfoM  =  $this->MODEL('userinfo');
	    $member     =  $userInfoM->getInfo(array('uid'=>$this->uid),array('field'=>'`subscribe`,`wxid`'));

	    $weixinM = $this->MODEL('weixin');
	    $log = $weixinM->getWxQrcode(array('wxloginid'=>$wxloginid, 'status'=>2, 'time'=>array('>', strtotime('today')), 'orderby'=>array('id,DESC')),array('field'=>'wxid'));
	    
	    if(!empty($member['wxid'])){

	        if (!empty($log['wxid']) && $member['wxid'] != $log['wxid']){

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

    function otherservice_action(){

        $backurl  =  Url('wap',array(),'member');
        $this->yunset('backurl',$backurl);
        $this->yunset('headertitle',yun_at('wap_user_00196'));
        $this->waptpl('other_service');
    }

	function photo_action(){

	    $backurl  =  Url('wap',array(),'member');
	    $this->yunset('backurl',$backurl);
	    
	    $this->yunset('headertitle',yun_at('admin_00015'));
	    $this->waptpl('photo');
	}

	function sq_action(){

        $backurl	=	Url('wap',array(),'member');
        $this->yunset('backurl',$backurl);
		$this->yunset('headertitle',yun_at('wap_user_00270'));
		$this->waptpl('sq');
	}

    function partapply_action()
    {

        $backurl = Url('wap', array(), 'member');
        $this->yunset('backurl', $backurl);
        $this->yunset('headertitle', yun_at('wap_user_00271'));
        $this->waptpl('partapply');
    }

	function collect_action(){

		$backurl	=	Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('headertitle',yun_at('wap_user_00193'));
		$this->waptpl('collect');
	}
	
	function password_action(){

		$this->yunset('backurl',Url('wap',array('c'=>'safe'),'member'));

		$this->yunset('headertitle',yun_at('wap_user_00337'));
		$this->waptpl('password');
	}
	function invitecont_action(){

		$this -> yunset('headertitle',yun_auto_t('面试详情'));
		$this -> waptpl('invitecont');
	}

	function invite_action(){

        $backurl	=	Url('wap',array(),'member');
        $this->yunset('backurl',$backurl);
		$this->yunset('headertitle',yun_at('wap_user_00216'));
		$this->waptpl('invite');
	}

	function look_action(){

        $backurl	=	Url('wap',array(),'member');
        $this->yunset('backurl',$backurl);
		$this->yunset('headertitle',yun_at('member_com_00322'));
		$this->waptpl('look');
	}

	function addresume_action(){

	    $cacheM	=	$this->MODEL('cache');
	    $cache	=	$cacheM -> GetCache(array('city','job'));
	    
	    $this->yunset($cache);
	    $this->yunset('backurl',Url('wap',array(),'member'));
		$this->waptpl('addresume');
	}

	function addresumeson_action(){

		switch($_GET['type']){

			case 'work':		$headertitle=yun_at('wap_00457');  break;
			case 'edu':			$headertitle=yun_at('wap_00459');  break;
			case 'project':		$headertitle=yun_at('wap_00465');  break;
			case 'training':	$headertitle=yun_at('wap_00455');  break;
			case 'skill':		$headertitle=yun_at('wap_00461');  break;
			case 'other':		$headertitle=yun_at('wap_00493');  break;
			case 'desc':		$headertitle=yun_at('wap_00463');  break;
			case 'show':		$headertitle=yun_at('wap_00492');  break;
			case 'doc':	        $headertitle=yun_at('member_user_00101');  break;
		}
		$this->yunset('headertitle',$headertitle);
		$this->waptpl('addresumeson');
	}

	function info_action(){
		$this->yunset('headertitle',yun_at('wap_00456'));
 		$this->waptpl('info');
	}
	
    function addexpect_action()
    {
        $cacheM	=	$this->MODEL('cache');
        $cache	=	$cacheM -> GetCache(array('city','job'));
        
        $this -> yunset($cache);
        
		$this->yunset('headertitle',yun_auto_t('意向职位修改'));
		$this->waptpl('addexpect');
	}
	function rcomplete_action(){
		$this->yunset('headertitle',yun_auto_t('发布成功'));
		$backurl	=	Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
        $this->yunset('url',Url('wap',array('c'=>'resume','a'=>'show','id'=>$_GET['id'])));
		$this->waptpl('rcomplete');
	}
	function resume_action(){

		$backurl		=Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('headertitle',yun_at('wap_user_00204'));
		$this->waptpl('resume');
	}
	function optimize_action(){
        $this->yunset('headertitle',yun_auto_t('优化简历'));
        
        if (isset($_GET['add'])){
            $backurl  =  Url('wap',array(),'member');
            $this->yunset('backurl',$backurl);
        }

        $this->waptpl('optimize');
    }

	function setPrivacyCookie_action(){
	    $this->cookie->setcookie('privacy', 1, time() + 3600 * 6);
	}

	function binding_action()
	{

		$this->yunset('headertitle',yun_auto_t('社交账号绑定'));
		$this->yunset("backurl",Url('wap',array('c'=>'ident'),'member'));
		$this->waptpl('binding');
	}
	function idcard_action(){
		$this->yunset('headertitle',yun_auto_t('身份证认证'));

		$backurl	=	Url('wap',array('c'=>'ident'),'member');
		$this->yunset('backurl',$backurl);
		$this->waptpl('idcard');
	}
	function bindingbox_action(){
		switch($_GET['type']){
			case 'moblie':$headertitle=yun_at('member_com_00071');
			break;
			case 'email':$headertitle=yun_at('wap_com_00186');
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
		$this->yunset('headertitle',yun_at('member_user_00220'));
		$this->waptpl('setname');
	}
	function reward_list_action(){
		$this->yunset('headertitle',yun_at('wap_user_00170'));
		if($_GET['back']){
			$backurl		=	Url('wap',array('c'=>'redeem'));
		}else{
			$backurl		=	Url('wap',array('c'=>'finance'),'member');
		}
		$this->yunset('backurl',$backurl);
		$this->waptpl('reward_list');
	}

	function privacy_action(){
		$this->yunset('headertitle',yun_at('wap_user_00215'));

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
				$data['msg']			=	yun_at('common_01106');

				if($_POST['paytype']=='alipay'){

					$url	=	$this->config['sy_weburl'].'/api/wapalipay/alipayto.php?dingdan='.$dingdan.'&dingdanname='.$dingdan.'&alimoney='.$price;
				}
 				echo json_encode(array(
 				    'error' => 0,
 				    'url'   => $url,
 				    'msg'   =>  yun_at('common_01106')
 				));
			}else{
			    echo json_encode(array(
			        'error' => 1,
			        'msg' => yun_at('common_00717')
			    ));
			}
 		}else{
 		    echo json_encode(array(
 		        'error' => 1,
 		        'msg' => yun_at('wap_00203')
 		    ));
		}
	}

	function pay_action(){
		$this->yunset('headertitle',yun_at('common_01946'));
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
					$this->ACT_msg_wap($_SERVER['HTTP_REFERER'],yun_at('wap_01291'),2,5);
				}elseif($order['order_state']!='1'){
					header("Location:index.php?c=paylog");
				}else{
					$this->yunset("order",$order);
				}
			}

			$this->yunset("paytype",$paytype);

		}else{
			$data['msg']	=	yun_at('wap_01286');
			$data['url']	=	$_SERVER['HTTP_REFERER'];
			$this->yunset("layer",$data);
		}

		$this->get_user();
		$this->yunset('headertitle',yun_at('member_user_00251'));
		$this->waptpl('payment');
	}
	/**
	 * Create order
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


		if($return['errcode'] == 9 && !empty($return['url'])){

			header('Location: '.$return['url']);exit();
		}else{
			$this->yunset("layer",$return);
		}

		$backurl  =  Url('wap',array(),'member');
		$this->get_user();
		$this->yunset('backurl',$backurl);
		$this->yunset('headertitle',yun_at('common_02029'));

		$this->waptpl('pay');
	}

    function paylog_action(){
        $this->yunset('headertitle',yun_at('wap_com_00068'));
        $backurl	=	Url('wap',array('c'=>'finance'),'member');
        $this		->	yunset('backurl',$backurl);
        $this->waptpl('paylog');
    }

	function likejob_action(){
		$this		->	yunset('headertitle',yun_at('wap_user_00211'));

		$this		->	waptpl('likejob');
	}

	function set_action(){
		$this->yunset('headertitle',yun_at('wap_user_00214'));
		
		$backurl	=	Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
		$this->waptpl('set');
	}

	function sysnews_action(){

		$this->yunset('headertitle',yun_at('wap_user_00365'));
		$this->waptpl('sysnews');

	}

	function sxnews_action(){
		$this->yunset('headertitle',yun_at('wap_user_00363'));

		$backurl	=	Url('wap',array('c'=>'sysnews'),'member');
		$this->yunset('backurl',$backurl);
		$this->waptpl('sxnews');
	}

	function commsg_action(){
		$this->yunset('headertitle',yun_at('wap_com_00408'));

		$backurl=Url('wap',array('c'=>'sysnews'),'member');
		$this->yunset('backurl',$backurl);
		$this->waptpl('commsg');
	}
	function finance_action(){

		$this->yunset('headertitle',yun_at('wap_user_00213'));
        $reg_url = Url('wap',array('c'=>'register','uid'=>$this->uid));
        $this->yunset('reg_url', $reg_url);
		$backurl	=	Url('wap',array(),'member');

		$this->yunset('backurl',$backurl);
		$this->waptpl('finance');
	}
	function integral_action(){
        $this->yunset('headertitle',yun_at('wap_user_00171'));
        $reg_url = Url('wap',array('c'=>'register','uid'=>$this->uid));
        $this->yunset('reg_url', $reg_url);
        $this->waptpl('alltask');
    }

    function blacklist_action()
    {

        $backurl	=	Url('wap',array(),'member');

        $this->yunset('backurl',$backurl);
        $this->yunset('headertitle', yun_at('wap_01060'));
        $this->waptpl('blacklist');
    }
	function blacklistadd_action(){

		$this->yunset('headertitle',yun_at('wap_01125'));
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
		$this->yunset('headertitle',yun_at('wap_user_00339'));
		$this->waptpl('transfer');
	}


	function logout_action()
    {

        $backurl	=	Url('wap',array('c' => 'safe'),'member');
        $this->yunset('backurl',$backurl);

        $this->yunset('headertitle',yun_at('member_com_00538'));
        $this->waptpl('logout');
    }
    /****************** gengzs start ************************/
    public function ident_action(){

        $backurl=Url('wap',array('c'=>'set'),'member');

        $this->yunset('backurl',$backurl);

        $this->yunset('header_title',yun_at('wap_user_00340'));

        $this->waptpl('ident');
    }
    public function safe_action(){

        $backurl=Url('wap',array('c'=>'set'),'member');
        $this->yunset('backurl',$backurl);

        $this->yunset('header_title',yun_at('wap_00817'));

        $this->waptpl('safe');
    }
    /****************** gengzs  end  ************************/
}
?>