<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class com_controller extends wap_controller
{

    function get_user()
    {
        if (!$_GET['c']) {
            if ($this->comInfo['hy'] == '') {
                if ($_COOKIE['indextip'] == '1') {

                    $indextip   =   0;
                } else {

                    $this->cookie->SetCookie('indextip', '1', (strtotime('today') + 86400));
                    $indextip   =   1;
                }
                $this->comInfo['base']   =   0;
                $this->yunset('indextip', $indextip);
            } else {

                $this->comInfo['base']   =   1;
                $this->cookie->SetCookie('indextip', '', (strtotime('today') - 86400));
            }
        }
        $this->yunset('company', $this->comInfo);
        return $this->comInfo;
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
        $error = 1;

        echo json_encode(array('error'=>$error,'subscribe'=>$subscribe));
    }
	function waptpl($tpname){

        $this->yuntpl(array('wap/member/com/'.$tpname));
	}

	function index_action(){
		
        $this->yunset('backurl',Url('wap',array()));
        $this->yunset('membernav', 1);
 		$this->waptpl('index');
	}
	// 招聘数据
	function zhaopin_action(){

        $backurl = Url('wap', array(), 'member');
        $this->yunset('backurl',$backurl);
	    $this->waptpl('zhaopin');
	}
	// 招聘周数据
	function zhaopinzhou_action(){

        $backurl = Url('wap', array(), 'member');
        $this->yunset('backurl',$backurl);
	    $this->waptpl('zhaopinzhou');
	}
	function com_action()
    {

		$backurl  =   Url('wap', array('c'=>'finance'), 'member');
		$this -> yunset('backurl',$backurl);
		$this -> yunset('header_title', '我的服务');
		$this -> waptpl('com');
	}

	function reportlist_action()
    {
		$backurl  =   Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',"举报简历");
		$this->waptpl('reportlist');
	}

	function info_action()
    {
		$this -> yunset('header_title','基本信息');
		$this -> waptpl('info');
	}

	function jobadd_action(){
		$cacheM	=	$this->MODEL('cache');
        $cache	=	$cacheM -> GetCache(array('job'));
        
        $this -> yunset($cache);
		$this -> yunset('header_title',"发布职位");
		$this -> waptpl('jobadd');
	}
	function saveJobSuccess_action(){
		$backurl = Url('wap', array('c'=>'job'), 'member');
        $this -> yunset('backurl', $backurl);
        $this -> yunset('header_title', '职位发布成功');

        $this -> waptpl('savejobsuccess');
	}
	function job_action()
    {
        $backurl = Url('wap', array(), 'member');
        $this -> yunset('backurl', $backurl);
        $this -> yunset('header_title', '职位管理');

        $this -> waptpl('job');
    }
	/**
	 * @desc 兼职报名
	 */
	function partapply_action(){

        $backurl  =  Url('wap', array('c' => 'part'), 'member');
        $this->yunset('backurl', $backurl);
        $this->yunset('header_title', '兼职报名');
        $this->waptpl('partapply');
    }

	function hr_action(){

		$this->yunset('header_title',"应聘简历");
		$this->get_user();
		$this->waptpl('hr');
	}

	function password_action(){
		$backurl=Url('wap',array('c'=>'safe'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',"修改密码");
		$this->waptpl('password');
	}

	function pay_action(){

	    $orderM		=	$this	->	MODEL('companyorder');
	    $paytype	=	array(
	        'alipay'	=>	$this->config['alipay']=='1' && $this->config['alipaytype']=='1'	?	'1'	:	''
	    );


	    if($paytype){
	        $this	->	yunset("paytype",$paytype);
	        $this	->	yunset("js_def",4);
	    }else{
	        $data['msg']	=	"暂未开通手机支付，请移步至电脑端充值！";
	        $data['url']	=	$_SERVER['HTTP_REFERER'];
	        $this	->	yunset("layer",$data);
	    }
	    $nopayorder	=	$orderM	->	getCompanyOrderNum(array('uid'=>$this->uid,'usertype' => $this->usertype,'order_state'=>'1'));
	    $this		->	yunset('nopayorder',$nopayorder);

	    $this		->	yunset($this->MODEL('cache')->GetCache(array('integralclass')));
	    $this		->	yunset('header_title',"充值".$this->config['integral_pricename']);
	    $this		->	waptpl('pay');
	}

	function payment_action(){
		if($this->config['alipay']=='1' &&  $this->config['alipaytype']=='1'){
			$paytype['alipay']	=	'1';
		}

		if($paytype){
			if($_GET['id']){//订单
				$orderM	=	$this	->	MODEL('companyorder');
				$order	=	$orderM	->	getInfo(array('uid'=>$this->uid,'id'=>(int)$_GET['id']),array('bank'=>1));
				if(empty($order)){
					$this->ACT_msg_wap($_SERVER['HTTP_REFERER'],"订单不存在！",2,5);
				}elseif($order['order_state']!='1'){
					header("Location:index.php?c=paylog");
				}else{
					$this	->	yunset("order",$order);
				}
			}
 			$this	->	yunset("paytype",$paytype);
 			$this	->	yunset("js_def",4);
		}else{
			$data['msg']	=	"暂未开通手机支付，请移步至电脑端充值！";
			$data['url']	=	$_SERVER['HTTP_REFERER'];
			$this	->	yunset("layer",$data);
		}
		$this	->	yunset('header_title',"订单确认");
		$this	->	waptpl('payment');
	}

	//会员统计信息调用
	function company_satic(){

		$statisM  =  $this->MODEL('statis');
		// 会员套餐过期检测，并处理

		$suid     =  $this->uid;
		$statis   =  $statisM -> vipOver($suid, 2);

		$this->yunset('addjobnum', $statis['addjobnum']);

		if($statis['integral'] == ''){
		    $statis['integral']   =   0;
		}
		$this->yunset('statis',$statis);

		return $statis;
	}

	function getOrder_action()
    {

	    $_POST				=	$this -> post_trim($_POST);

	    if (empty($_POST)) {
	        echo json_encode(array('error' => 1, 'msg' => '参数错误，请重试！'));die();
	    }

	    $data				=	$_POST;
	    $data['uid']		=   $this -> uid;
	    $data['username']	=   $this -> username;
	    $data['usertype']	=   $this -> usertype;
	    $data['did']		=   $this -> userdid;

	    $compayM            =   $this->MODEL('compay');
	    $return				=	$compayM->orderBuy($data);

	    if($return['error'] == 0){
	        $dingdan	=	$return['orderid'];
	        $price		=	$return['order_price'];
	        $id			=	$return['id'];

	        //多种支付方式并存 进行选择
	        if($_POST['paytype']=='alipay'){

	            $url = $this->config['sy_weburl'].'/api/wapalipay/alipayto.php?dingdan='.$dingdan.'&dingdanname='.$dingdan.'&alimoney='.$price;
	        }
	        echo json_encode(array(
	            'error' => 0,
	            'url'   => $url,
	            'msg'   =>  '下单成功，请付款！'
	        ));

	    }else{
	        echo json_encode($return);
	    }
	}

	/**
	 * 充值、购买会员、购买增值包生成订单
	 */
	function dingdan_action()
	{

		$rdata['price']			=  $_POST['price'];
		$rdata['comvip']		=  $_POST['comvip'];
		$rdata['comservice']	=  $_POST['comservice'];
		$rdata['dkjf']			=  $_POST['dkjf'];
		$rdata['price_int']		=  $_POST['price_int'];
		$rdata['integralid']	=  $_POST['integralid'];

		$rdata['uid']			=  $this->uid;
		$rdata['usertype']		=  $this->usertype;
		$rdata['did']			=  $this->userdid;
		$rdata['paytype']	    =  $_POST['paytype'];
		$rdata['type']		    =  'wap';
		$rdata['port']		    =  '2';

		$orderM	 =  $this	->	MODEL('companyorder');
		$return	 =  $orderM	->	addComOrder($rdata);
		//微信支付、支付宝支付，跳转到相应的链接
		if($return['errcode'] == 9 && !empty($return['url'])){

		    header('Location: '.$return['url']);exit();
		}else{
		    $this->yunset("layer",$return);
		}

		$backurl  =  Url('wap',array(),'member');
		$this -> yunset('backurl',$backurl);
		$this -> yunset('headertitle','订单');
		$this -> get_user();
		$this -> waptpl('pay');
	}
	
	function look_job_action(){

		$this->yunset('header_title',"谁看过我");
		$this->get_user();
		$this->waptpl('look_job');
	}

	function invite_action(){
		$this->yunset('header_title',"面试邀请");
		$this->waptpl('invite');
	}

	/**
	 * @desc 兼职列表
	 */
	function part_action()
    {
        $backurl = Url('wap', array('c' => 'jobcolumn'), 'member');
        $this -> yunset('backurl', $backurl);
        $this -> yunset('header_title', '兼职管理');
        $this -> waptpl('part');
    }

    // 发布兼职
	function partadd_action()
    {
        $this->yunset('header_title', "发布兼职");
        $this->waptpl('partadd');
    }

	function photo_action(){
		
	    if($_GET['t']){
	        $backurl	=	Url('wap',array(),'member');
	    }else if($_GET['type']){
	        $backurl	=	Url('wap',array('c'=>'integral'),'member');
	    }else{
	        $backurl	=	Url('wap',array('c'=>'info'),'member');
	    }
	    
	    $this->yunset('backurl',$backurl);
	    $this->yunset('header_title',"企业LOGO");
	    $this->waptpl('photo');
	}
	
	function comcert_action(){

		if(!isset($_GET['certbox'])){
			$backurl = Url('wap',array('c'=>'ident'),'member');
			$this->yunset('backurl',$backurl);
		}

		$this->yunset('header_title', '企业资质');
		$this->waptpl('comcert');
	}

	function binding_action(){

        if (!isset($_GET['certbox'])){
            $backurl = Url('wap',array('c'=>'ident'),'member');
            $this->yunset('backurl',$backurl);
        }
		$this->yunset('header_title',"社交账号绑定");
		$this->waptpl('binding');
	}

	/**
	 * @desc 手机绑定页面
	 */
	function bindingbox_action(){

	    if (!isset($_GET['certbox'])){
	        $backurl = Url('wap', array('c' => 'ident'), 'member');
	        $this->yunset('backurl', $backurl);
	    }
        $this->yunset('header_title', "账户绑定");
        $this->waptpl('bindingbox');
    }

    function setname_action()
    {
        $backurl = Url('wap', array('c' => 'safe'), 'member');
        $this->yunset('backurl', $backurl);
        $this->yunset('header_title', "修改用户名");
        $this->waptpl('setname');
    }

    function reward_list_action()
    {

		$backurl	=	Url('wap',array('c'=>'integral'),'member');
		$this		->	yunset('backurl',$backurl);
		$this		->	yunset('header_title',"兑换记录");

		$this		->	waptpl('reward_list');
	}

	function delreward_action(){
		$redeemM	=	$this		->	MODEL('redeem');
		$return		=	$redeemM	->	delChange(
			array(
				'uid'		=>	$this->uid,
				'id'		=>	(int)$_GET['id']
			),
			array(
				'member'	=>	'com',
				'uid'		=>	$this->uid,
				'usertype'	=>	$this->usertype,
				'id'		=>	(int)$_GET['id']
			)
		);
		$this		->	waplayer_msg($return['msg']);

	}
	function paylog_action(){
	    
		$this	->	yunset('header_title',"明细");
        $backurl  =  Url('wap',array('c'=>'finance'),'member');
        $this->yunset('backurl',$backurl);
		$this	->	waptpl('paylog');
	}



    function special_action(){
        
		$backurl=Url('wap',array('c' => 'jobcolumn'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset("header_title","专题招聘");
        $this->waptpl('special');
    }
    
	function zhaopinhui_action(){
		
		$backurl=Url('wap',array('c' => 'jobcolumn'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset("header_title","招聘会记录");
		$this->waptpl('zhaopinhui');
	}
	

	function set_action(){

	    $backurl  =  Url('wap', array(), 'member');
	    $this->yunset('backurl', $backurl);
	    $this->yunset('header_title', '账户设置');
	    $this->waptpl('set');
	}

	function sysnews_action(){

        $this->yunset('header_title',"消息");
		$this->waptpl('sysnews');
	}
	//求职咨询
	function msg_action(){
		
        $backurl = Url('wap',array('c'=>'sysnews'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',"求职咨询");
        $this->waptpl('msg');
	}
    //私信
	function sxnews_action(){

		$backurl = Url('wap',array('c'=>'sysnews'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',"系统消息");
		$this->waptpl('sxnews');
	}

	function attention_me_action(){
	    
	    $backurl=Url('wap',array('c'=>'sysnews'),'member');
	    $this->yunset('backurl',$backurl);

		$this->yunset('header_title',"对我感兴趣");
	    $this->waptpl('attention_me');
	}



	function finance_action(){
        $reg_url =Url('wap',array('c'=>'register','uid'=>$this->uid));
        $this->yunset('reg_url', $reg_url);
		$backurl =	Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',"财务管理");
		$this->waptpl('finance');
	}
	function integral_action(){

		if($_GET['type']){
			$backurl	=	Url('wap',array('c'=>'finance'),'member');
		}else{
			$backurl	=	Url('wap',array(),'member');
		}

		$reg_url = Url('wap',array('c'=>'register','uid'=>$this->uid));
		$this->yunset('reg_url', $reg_url);
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',"全部任务");
		$this->waptpl('integral');
	}

	function resumecolumn_action(){

		$backurl=Url('wap',array(),'member');

		$this->yunset('backurl',$backurl);

		$this->yunset('header_title',"简历管理");

		$this->waptpl('resumecolumn');
	}

    function jobcolumn_action(){

		$backurl=Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
 		$this->yunset("header_title","其他服务");
		$this->waptpl('jobcolumn');
	}

	function integral_reduce_action(){
		$backurl	=	Url('wap',array('c'=>'integral'),'member');
		$this		->	yunset('backurl',$backurl);
		$this		->	yunset('header_title',"消费规则");
		$this		->	waptpl('integral_reduce');
	}


	function banner_action(){

		$companyM	=	$this -> MODEL('company');

		if($_POST['submit']){

			$data			=	array(

				'base'	=>	$_POST['preview'],

				'uid'		=>	$this->uid,

				'usertype'	=>	$this->usertype

			);

			$row			 =	$companyM-> getBannerInfo('',array('where'=>array('uid'=>$this->uid)));

			if($row['id']){

				$data['type']='update';

			}else{

				$data['type']='add';

			}

			$return			 =	$companyM	->	setBanner($data);

		}

		$banner		=	$companyM-> getBannerInfo('',array('where'=>array('uid'=>$this->uid)));

		$backurl	=	Url('wap',array('c'=>'integral'),'member');

		$this->yunset("layer",$return);
		$this->yunset("banner",$banner);
		$this->yunset("backurl",$backurl);
		$this->yunset('header_title',"企业横幅");
		$this->waptpl('banner');
	}

	function show_action(){

		$backurl = Url('wap',array('c'=>'set'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',"企业环境");
		$this->waptpl('show');
	}

    /**
     * @desc 会员套餐、增值服务、单项购买页面
     */
    function server_action(){

        $this->yunset('header_title', '优选服务');
        $this->waptpl('server');
    }

	/**
	 * 邀请模板列表
	 */
	function yqmb_action(){

		$backurl	=   Url('wap',array('c'=>'set'), 'member');
		$this -> yunset('backurl', $backurl);
		$this -> yunset('header_title', '管理邀请模板');
		$this -> waptpl('yqmb');
	}

	/**
	 * 创建邀请模板
	 */
	function yqmbedit_action(){

		$backurl	=   Url('wap',array('c' => 'yqmb'), 'member');
		$this -> yunset('backurl', $backurl);
		$this -> yunset('header_title', '创建修改模板');
		$this -> waptpl('yqmbedit');
	}

    /**
     * 预约刷新
     */
    function reserveUp_action()
    {

        if ($_POST) {


            $jobM   =   $this->MODEL('job');

            $data   =   array(

                'job_id'    =>  $_POST['job_id'],
                'end_time'  =>  strtotime($_POST['end_time']),
                'interval'  =>  $_POST['interval'],
                'status'    =>  $_POST['status']
            );
            $return =   $jobM->reserveUpJob($data, array('uid' => $this->uid));

            echo json_encode($return);
            die;
        } else {

            echo json_encode(array('error' => 0, 'msg' => '参数错误'));
            die;
        }
    }

    function logout_action()
    {

        $backurl	=	Url('wap',array('c' => 'safe'),'member');
        $this->yunset('backurl',$backurl);

        $this->yunset('header_title',"账号注销");
        $this->waptpl('logout');
    }

    /**
     * 工作地址管理
     */
    function address_action()
    {

        $backurl    =   Url('wap', array('c' => 'set'), 'member');
        $this->yunset('backurl', $backurl);
        $this->yunset('header_title', '地址管理');
        $this->waptpl('address');
    }

    /**
     * 新增工作地址
     */
    function newAddress_action()
    {

        $backurl    =   Url('wap', array('c' => 'address'), 'member');
        $this->yunset('backurl', $backurl);
        $this->yunset('header_title', '新建工作地址');
        $this->waptpl('address_new');
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
    function poi_action(){
        
        echo getPoi($_POST);
    }
}

?>