<?php



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

	function zhaopin_action(){

        $backurl = Url('wap', array(), 'member');
        $this->yunset('backurl',$backurl);
	    $this->waptpl('zhaopin');
	}

	function zhaopinzhou_action(){

        $backurl = Url('wap', array(), 'member');
        $this->yunset('backurl',$backurl);
	    $this->waptpl('zhaopinzhou');
	}
	function com_action()
    {

		$backurl  =   Url('wap', array('c'=>'finance'), 'member');
		$this -> yunset('backurl',$backurl);
		$this -> yunset('header_title', yun_at('wap_com_00064'));
		$this -> waptpl('com');
	}

	function reportlist_action()
    {
		$backurl  =   Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',yun_at('wap_com_00349'));
		$this->waptpl('reportlist');
	}

	function info_action()
    {
		$this -> yunset('header_title',yun_at('wap_00456'));
		$this -> waptpl('info');
	}

	function jobadd_action(){
		$cacheM	=	$this->MODEL('cache');
        $cache	=	$cacheM -> GetCache(array('job'));
        
        $this -> yunset($cache);
		$this -> yunset('header_title',yun_at('wap_00322'));
		$this -> waptpl('jobadd');
	}
	function saveJobSuccess_action(){
		$backurl = Url('wap', array('c'=>'job'), 'member');
        $this -> yunset('backurl', $backurl);
        $this -> yunset('header_title', yun_at('wap_01173'));

        $this -> waptpl('savejobsuccess');
	}
	function job_action()
    {
        $backurl = Url('wap', array(), 'member');
        $this -> yunset('backurl', $backurl);
        $this -> yunset('header_title', yun_at('wap_com_00106'));

        $this -> waptpl('job');
    }
	/**
	 * @desc Part-time job applications
	 */
	function partapply_action(){

        $backurl  =  Url('wap', array('c' => 'part'), 'member');
        $this->yunset('backurl', $backurl);
        $this->yunset('header_title', yun_at('wap_user_00302'));
        $this->waptpl('partapply');
    }

	function hr_action(){

		$this->yunset('header_title',yun_at('wap_00794'));
		$this->get_user();
		$this->waptpl('hr');
	}

	function password_action(){
		$backurl=Url('wap',array('c'=>'safe'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',yun_at('member_user_00226'));
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
	        $data['msg']	=	yun_at('wap_01286');
	        $data['url']	=	$_SERVER['HTTP_REFERER'];
	        $this	->	yunset("layer",$data);
	    }
	    $nopayorder	=	$orderM	->	getCompanyOrderNum(array('uid'=>$this->uid,'usertype' => $this->usertype,'order_state'=>'1'));
	    $this		->	yunset('nopayorder',$nopayorder);

	    $this		->	yunset($this->MODEL('cache')->GetCache(array('integralclass')));
	    $this		->	yunset('header_title',yun_at('common_01946').$this->config['integral_pricename']);
	    $this		->	waptpl('pay');
	}

	function payment_action(){
		if($this->config['alipay']=='1' &&  $this->config['alipaytype']=='1'){
			$paytype['alipay']	=	'1';
		}

		if($paytype){
			if($_GET['id']){// order
				$orderM	=	$this	->	MODEL('companyorder');
				$order	=	$orderM	->	getInfo(array('uid'=>$this->uid,'id'=>(int)$_GET['id']),array('bank'=>1));
				if(empty($order)){
					$this->ACT_msg_wap($_SERVER['HTTP_REFERER'],yun_at('wap_01291'),2,5);
				}elseif($order['order_state']!='1'){
					header("Location:index.php?c=paylog");
				}else{
					$this	->	yunset("order",$order);
				}
			}
 			$this	->	yunset("paytype",$paytype);
 			$this	->	yunset("js_def",4);
		}else{
			$data['msg']	=	yun_at('wap_01286');
			$data['url']	=	$_SERVER['HTTP_REFERER'];
			$this	->	yunset("layer",$data);
		}
		$this	->	yunset('header_title',yun_at('wap_com_00430'));
		$this	->	waptpl('payment');
	}


	function company_satic(){

		$statisM  =  $this->MODEL('statis');


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
	        echo json_encode(array('error' => 1, 'msg' => yun_at('wap_00203')));die();
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


	        if($_POST['paytype']=='alipay'){

	            $url = $this->config['sy_weburl'].'/api/wapalipay/alipayto.php?dingdan='.$dingdan.'&dingdanname='.$dingdan.'&alimoney='.$price;
	        }
	        echo json_encode(array(
	            'error' => 0,
	            'url'   => $url,
	            'msg'   =>  yun_at('common_01106')
	        ));

	    }else{
	        echo json_encode($return);
	    }
	}

	/**
	 * Create order for recharge, membership, or addon purchase
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

		if($return['errcode'] == 9 && !empty($return['url'])){

		    header('Location: '.$return['url']);exit();
		}else{
		    $this->yunset("layer",$return);
		}

		$backurl  =  Url('wap',array(),'member');
		$this -> yunset('backurl',$backurl);
		$this -> yunset('headertitle',yun_at('common_02029'));
		$this -> get_user();
		$this -> waptpl('pay');
	}
	
	function look_job_action(){

		$this->yunset('header_title',yun_at('wap_user_00276'));
		$this->get_user();
		$this->waptpl('look_job');
	}

	function invite_action(){
		$this->yunset('header_title',yun_at('wap_com_00046'));
		$this->waptpl('invite');
	}

	/**
	 * @desc Part-time job list
	 */
	function part_action()
    {
        $backurl = Url('wap', array('c' => 'jobcolumn'), 'member');
        $this -> yunset('backurl', $backurl);
        $this -> yunset('header_title', yun_at('wap_user_00271'));
        $this -> waptpl('part');
    }


	function partadd_action()
    {
        $this->yunset('header_title', yun_at('wap_00321'));
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
	    $this->yunset('header_title',yun_at('wap_com_00148'));
	    $this->waptpl('photo');
	}
	
	function comcert_action(){

		if(!isset($_GET['certbox'])){
			$backurl = Url('wap',array('c'=>'ident'),'member');
			$this->yunset('backurl',$backurl);
		}

		$this->yunset('header_title', yun_at('wap_com_00075'));
		$this->waptpl('comcert');
	}

	function binding_action(){

        if (!isset($_GET['certbox'])){
            $backurl = Url('wap',array('c'=>'ident'),'member');
            $this->yunset('backurl',$backurl);
        }
		$this->yunset('header_title',yun_at('wap_user_00379'));
		$this->waptpl('binding');
	}

	/**
	 * @desc Mobile binding page
	 */
	function bindingbox_action(){

	    if (!isset($_GET['certbox'])){
	        $backurl = Url('wap', array('c' => 'ident'), 'member');
	        $this->yunset('backurl', $backurl);
	    }
        $this->yunset('header_title', yun_at('wap_com_00431'));
        $this->waptpl('bindingbox');
    }

    function setname_action()
    {
        $backurl = Url('wap', array('c' => 'safe'), 'member');
        $this->yunset('backurl', $backurl);
        $this->yunset('header_title', yun_at('member_user_00220'));
        $this->waptpl('setname');
    }

    function reward_list_action()
    {

		$backurl	=	Url('wap',array('c'=>'integral'),'member');
		$this		->	yunset('backurl',$backurl);
		$this		->	yunset('header_title',yun_at('wap_user_00170'));

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
	    
		$this	->	yunset('header_title',yun_at('wap_com_00068'));
        $backurl  =  Url('wap',array('c'=>'finance'),'member');
        $this->yunset('backurl',$backurl);
		$this	->	waptpl('paylog');
	}



    function special_action(){
        
		$backurl=Url('wap',array('c' => 'jobcolumn'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset("header_title",yun_at('wap_com_00310'));
        $this->waptpl('special');
    }
    
	function zhaopinhui_action(){
		
		$backurl=Url('wap',array('c' => 'jobcolumn'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset("header_title",yun_at('wap_com_00309'));
		$this->waptpl('zhaopinhui');
	}
	

	function set_action(){

	    $backurl  =  Url('wap', array(), 'member');
	    $this->yunset('backurl', $backurl);
	    $this->yunset('header_title', yun_at('wap_user_00214'));
	    $this->waptpl('set');
	}

	function sysnews_action(){

        $this->yunset('header_title',yun_at('wap_user_00365'));
		$this->waptpl('sysnews');
	}

	function msg_action(){
		
        $backurl = Url('wap',array('c'=>'sysnews'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',yun_at('wap_com_00408'));
        $this->waptpl('msg');
	}

	function sxnews_action(){

		$backurl = Url('wap',array('c'=>'sysnews'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',yun_at('wap_user_00363'));
		$this->waptpl('sxnews');
	}

	function attention_me_action(){
	    
	    $backurl=Url('wap',array('c'=>'sysnews'),'member');
	    $this->yunset('backurl',$backurl);

		$this->yunset('header_title',yun_at('wap_com_00407'));
	    $this->waptpl('attention_me');
	}



	function finance_action(){
        $reg_url =Url('wap',array('c'=>'register','uid'=>$this->uid));
        $this->yunset('reg_url', $reg_url);
		$backurl =	Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',yun_at('wap_user_00213'));
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
		$this->yunset('header_title',yun_at('wap_user_00171'));
		$this->waptpl('integral');
	}

	function resumecolumn_action(){

		$backurl=Url('wap',array(),'member');

		$this->yunset('backurl',$backurl);

		$this->yunset('header_title',yun_at('wap_com_00105'));

		$this->waptpl('resumecolumn');
	}

    function jobcolumn_action(){

		$backurl=Url('wap',array(),'member');
		$this->yunset('backurl',$backurl);
 		$this->yunset("header_title",yun_at('wap_user_00196'));
		$this->waptpl('jobcolumn');
	}

	function integral_reduce_action(){
		$backurl	=	Url('wap',array('c'=>'integral'),'member');
		$this		->	yunset('backurl',$backurl);
		$this		->	yunset('header_title',yun_at('wap_user_00252'));
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
		$this->yunset('header_title',yun_at('admin_user_company_00283'));
		$this->waptpl('banner');
	}

	function show_action(){

		$backurl = Url('wap',array('c'=>'set'),'member');
		$this->yunset('backurl',$backurl);
		$this->yunset('header_title',yun_at('wap_user_00157'));
		$this->waptpl('show');
	}

    /**
     * @desc Membership packages, addons, and single-purchase page
     */
    function server_action(){

        $this->yunset('header_title', yun_at('wap_com_00432'));
        $this->waptpl('server');
    }

	/**
	 * Invitation template list
	 */
	function yqmb_action(){

		$backurl	=   Url('wap',array('c'=>'set'), 'member');
		$this -> yunset('backurl', $backurl);
		$this -> yunset('header_title', yun_at('wap_com_00433'));
		$this -> waptpl('yqmb');
	}

	/**
	 * Create invitation template
	 */
	function yqmbedit_action(){

		$backurl	=   Url('wap',array('c' => 'yqmb'), 'member');
		$this -> yunset('backurl', $backurl);
		$this -> yunset('header_title', yun_at('wap_com_00434'));
		$this -> waptpl('yqmbedit');
	}

    /**
     * Scheduled refresh
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

            echo json_encode(array('error' => 0, 'msg' => yun_at('wap_com_00228')));
            die;
        }
    }

    function logout_action()
    {

        $backurl	=	Url('wap',array('c' => 'safe'),'member');
        $this->yunset('backurl',$backurl);

        $this->yunset('header_title',yun_at('member_com_00538'));
        $this->waptpl('logout');
    }

    /**
     * Work address management
     */
    function address_action()
    {

        $backurl    =   Url('wap', array('c' => 'set'), 'member');
        $this->yunset('backurl', $backurl);
        $this->yunset('header_title', yun_at('wap_com_00402'));
        $this->waptpl('address');
    }

    /**
     * Add work address
     */
    function newAddress_action()
    {

        $backurl    =   Url('wap', array('c' => 'address'), 'member');
        $this->yunset('backurl', $backurl);
        $this->yunset('header_title', yun_at('wap_00894'));
        $this->waptpl('address_new');
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
    function poi_action(){
        
        echo getPoi($_POST);
    }
}

?>