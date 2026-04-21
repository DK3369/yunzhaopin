<?php
/*
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */
class index_controller extends common{
	
	function index_action(){
        if($this->config['province']){
			$_GET['provinceid'] 	= 	$this->config['province'];
		}
		if($this->config['cityid']){
		    $_GET['cityid'] 		= 	$this->config['cityid'];
		}
		if($this->config['three_cityid']){
		    $_GET['three_cityid'] 	= 	$this->config['three_cityid'];
		}
		if($_GET['city']){//城市匹配
			$city					=	explode("_",$_GET['city']);
			$_GET['provinceid']		=	$city[0];
			$_GET['cityid']			=	$city[1];
			$_GET['three_cityid']	=	$city[2];
		}
		$FinderParams	=	array('city','provinceid','cityid','three_cityid','add_time');
		foreach($_GET as $k=>$v){
			if(in_array($k,$FinderParams) && $v!="" && $v!="0"){ 
				$finder[$k]	=	$v; 
			}
		}
		unset($finder['city']);
		$this->yunset('finder',$finder);
		$CacheM		=	$this->MODEL('cache');
		$CacheList	=	$CacheM->GetCache(array('city'));
		$this->yunset($CacheList);
		$onceM		=	$this->MODEL('once');
		//后台已关闭
		if($this->config['sy_once_web']=="2"){
			header("location:".Url('error'));
		}
		if($_GET['keyword']=='请输入店铺招聘的关键字'){
			$_GET['keyword']	=	'';
		}
		$ip			=	fun_ip_get();
    
		$this->yunset("ip",$ip);
		$start_time	=	strtotime(date('Y-m-d 00:00:00')); //开始时间
        $totalMessNum = $onceM->getOnceNum(array('ctime'=>array('>',$start_time)));//当天总的已发布量

		$mess		=	$onceM->getOnceNum(array('login_ip'=>$ip,'ctime'=>array('>',$start_time)));//当天单IP已发布量
        if($this->config['sy_once_totalnum'] == 0 || ($this->config['sy_once_totalnum'] > $totalMessNum)){
            $isFb = true;
       }else{
            $isFb = false;
        }
        $this->yunset('isFb',$isFb);
        if($this->config['sy_once'] > 0){
            $num	=	$this->config['sy_once']-$mess;
        }else{
            $num	=	1;
        }

		$this->yunset('num',$num);
		
		if($_COOKIE['fast']){
			//未付款订单
			$companyorderM	=	$this->MODEL('companyorder');
			$orderNum 		= 	$companyorderM->getCompanyOrderNum(array('order_state'=>1,'type'=>25,'fast'=>$_COOKIE['fast']));
			$this->yunset("ordernum",$orderNum);
			
			$paylog 		= 	$companyorderM->getList(array('order_state'=>1,'type'=>25,'fast'=>$_COOKIE['fast']));
			
			$this->yunset("paylog",$paylog);
			$this->yunset("fast",$_COOKIE['fast']);
		}
		//关键字显示
		include PLUS_PATH."keyword.cache.php";
		if(is_array($keyword)){
			foreach($keyword as $k=>$v){
				if($v['type']=='1' && $v['tuijian']=='1'){
					
					$oncekeyword[]	=	$v;
				}
			}
		}
		$this->yunset("oncekeyword",$oncekeyword);
		//关键字显示end
		
        $add_time	=	array("0"=>"不限","7"=>"一周以内","15"=>"半个月","30"=>"一个月","60"=>"两个月","180"=>"半年","365"=>"一年");
        $this->yunset("add_time",$add_time);
		
		
		$this->seo("once");
		$this->yun_tpl(array('index'));
		 
	}
	//'"刷新、修改、删除"验证密码'
	function ajax_action(){
		$onceM		=	$this->MODEL('once');
		$data		=	array(
			'code'		=>	$_POST['code'],
			'id'		=>	(int)$_POST['tid'],
			'password'	=>	$_POST['pw'],
			'type'		=>	$_POST['type'],
			'utype'		=>	'pc'
			
		);
		$return		=	$onceM -> setOncePassword($data);
		
		echo json_encode($return);die;
	}
	//店铺详情
	function show_action(){
		$id		=	(int)$_GET['id'];
				
    
		$CacheM		=	$this->MODEL('cache');
    
		$CacheList	=	$CacheM->GetCache(array('city'));
    
		$this->yunset($CacheList);
		$onceM	=	$this->MODEL('once');
		
		$onceM->upOnce(array('hits'=>array('+',1)),array('id'=>$id));
		
		$o_info	=	$onceM->getOnceInfo(array('id'=>$id));
		
		if($o_info['status']<'1' && !$_GET['pay']){
			$this->ACT_msg(Url('once'),"店铺正在审核中！");
		}
		$this->yunset('o_info',$o_info);
		
		$ip=fun_ip_get();
		$this->yunset("ip",$ip);
		
		$start_time=strtotime(date('Y-m-d 00:00:00')); //开始时间
		$mess=$onceM->getOnceNum(array('login_ip'=>$ip,'ctime'=>array('>',$start_time)));
		if($this->config['sy_once']){
			$num=$this->config['sy_once']-$mess;
		}else{
			$num=1;
		} 
		$this->yunset("num",$num);
		
		if($_COOKIE['fast']){
			//未付款订单
			$companyorderM	=	$this->MODEL('companyorder');
			$orderNum 		= 	$companyorderM->getCompanyOrderNum(array('order_state'=>1,'type'=>25,'fast'=>$_COOKIE['fast']));
			$this->yunset('ordernum',$orderNum);
			
			$paylog 		= 	$companyorderM->getList(array('order_state'=>1,'type'=>25,'fast'=>$_COOKIE['fast']));
			
			$this->yunset("paylog",$paylog);
			$this->yunset("fast",$_COOKIE['fast']);
		}
		

		
		$data['once_job']	=	$o_info['title'];//新闻名称
		$data['once_name']	=	$o_info['companyname'];//描述		
		$description		=	$o_info['require_n'];
		$data['once_desc']	=	$this->GET_content_desc($description);//描述
		$this->data			=	$data;
		$this->seo('once_show');
		$this->yun_tpl(array('show'));
	} 
	//发布招聘支付
	function pay_action(){
		$onceM	=	$this->MODEL('once');
		$data	=	array(
			'id'			=>	$_POST['onceid'],
			'did'			=>	$this->userdid,
			'pay_type'		=>	$_POST['pay_type'],
			'oncepricegear'	=>	$_POST['oncepricegear'],
			'once_price'	=>	$_POST['once_price']
		);
		
		$return	=	$onceM->payOnce($data);
		
		echo json_encode($return);
	}
		
	//取消招聘订单付款
	function delpaylog_action(){
		$orderM	=	$this->MODEL('companyorder');
		$return	=	$orderM->del((int)$_GET['id'],array('utype'=>'once'));
		if($return['errcode']==9){
			$return['msg']='取消订单成功！';
		}else{
			$return['msg']='取消订单失败！';
		}
		$this->layer_msg($return['msg'],$return['errcode'],$return['layertype'],Url("once"));
		
	}
	//添加、修改店铺招聘页面
	function add_action(){
		
		$onceM	=	$this->MODEL('once');
		$info	=	$onceM->getOnceInfo(array('id'=>(int)$_GET['id']));
		if(!empty($info)){
			//检测当前密码是否对应
			session_start();
			
			if($info['password'] == $_SESSION['oncepass']){
				$this->yunset('info',$info);
			}else{
				
				header("Location:".Url('once',array('c'=>'show','id'=>(int)$_GET['id'])));
				exit();
			}
		}
		
		$CacheM		=	$this->MODEL('cache');
		$CacheList	=	$CacheM->GetCache(array('city', 'oncepricegear'));
        $this->yunset($CacheList);
		
		$this->seo('once');
		$this->yun_tpl(array('add'));
	}
    //短信验证码发送
    function sendmsg_action()
    {
        $noticeM	=	$this->MODEL('notice');
        $result		=	$noticeM->jycheck($_POST['code'],'店铺招聘');
        if(!empty($result)){
            echo json_encode(array('msg'=>$result['msg'],'error'=>$result['error']));
            return;
        }
        $moblie = $_POST['moblie'];

        $result = $noticeM->sendCode($moblie, 'code', 1, array(), 6, 120, 'msg');
        echo json_encode($result);
        exit();
    }
    //保存店铺招聘信息
	function save_action(){
		$onceM		= 	$this ->  MODEL('once');
		$authcode	=	$_POST['authcode'];
		// if($_POST['edate']){
		// 	$edate		=	strtotime("+".(int)$_POST['edate']." days");
		// }
		$post		=   array(
			'title' 		=>  $_POST['title'],
			'companyname'	=>  $_POST['companyname'],
			'linkman'		=>	$_POST['linkman'],
			'phone' 		=>  $_POST['phone'],
			'provinceid' 	=>  $_POST['provinceid'],
			'cityid' 		=>  $_POST['cityid'],
			'three_cityid'	=>  $_POST['three_cityid'],
			'address' 		=>  $_POST['address'],
			'require' 		=>  $_POST['require'],
			'yyzz'			=>	$_FILES['yyzz'],
            'file'			=>	$_FILES['file'],
			// 'edate'			=>	$edate,
			'salary'		=>	$_POST['salary'],
			'password'		=>	$_POST['password'],
			'status' 		=>  $this->config['com_fast_status'],
			'ctime'			=>	time(),
			'did'			=>	$this->userdid ? $this->userdid : $this->config['did'],
			'login_ip'		=>	fun_ip_get()
		);
		$data		=	array(
			'id'				=>	(int)$_POST['id'],
			'oncepricegear'     => $_POST['oncepricegear'],
			'post'				=>	$post,
			'authcode'			=>	$authcode,
			'verify_token'		=>	$_POST['verify_token'],
			'moblie_code'		=>	$_POST['moblie_code'],
			'fast'				=>	$_COOKIE['fast'],
			'utype'				=>	'pc'
		);
					
		$return  	= 	$onceM  ->  addOnceInfo($data);
		
		if($return['url']){
			$this->ACT_layer_msg($return['msg'],$return['errcode'],$return['url']);
		}else{
			$this->ACT_layer_msg($return['msg'],$return['errcode']);
		}
	}
	
}
?>