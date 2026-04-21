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
class tiny_controller extends common{
	function index_action(){
		if($this->config['sy_tiny_web']=="2"){
			$this->ACT_msg_wap('index.php','很抱歉！该模块已关闭！', 1, 3);
		}
		$tinyM		=	$this->MODEL('tiny');
		
		$CacheM		=	$this->MODEL('cache');
		$CacheList	=	$CacheM->GetCache(array('user','city'));
        $this->yunset($CacheList);
		foreach($_GET as $k=>$v){
			if($k!=""){
				$searchurl[]	=	$k."=".$v;
			}
		}
		$searchurl=@implode("&",$searchurl);
		$this->yunset("searchurl",$searchurl);

		$ip		=	fun_ip_get();
		$this->yunset("ip",$ip);
		
		$s_time	=	strtotime(date('Y-m-d 00:00:00')); //今天开始时间
        $m_tiny_total=	$tinyM->getResumeTinyNum(array('time'=>array('>',$s_time)));
		$m_tiny	=	$tinyM->getResumeTinyNum(array('login_ip'=>$ip,'time'=>array('>',$s_time)));
        if($this->config['sy_tiny_totalnum'] == 0 || ($this->config['sy_tiny_totalnum'] > $m_tiny_total)){
            $isFb = true;
        }else{
            $isFb = false;
        }

		if($this->config['sy_tiny']>0){
			$num=$this->config['sy_tiny']-$m_tiny;
        }else{
			$num=1;
        }
        $this->yunset("isFb",$isFb);
        $this->yunset("num",$num);
		$adtime	=	array("0"=>"不限","7"=>"一周以内","15"=>"半个月","30"=>"一个月","60"=>"两个月","180"=>"半年","365"=>"一年");
		$this->yunset("adtime",$adtime);
		$this->seo("tiny");
		$this->yunset("topplaceholder","请输入关键字如：普工");
		$this->yunset("headertitle","普工专区");
		$this->yuntpl(array('wap/tiny'));
	}
	function ajax_action(){
		
		$_POST		=	$this->post_trim($_POST);
		$tinyM		=	$this->MODEL('tiny');
		$data		=	array(
			'code'		=>	$_POST['checkcode'],
			'id'		=>	(int)$_POST['id'],
			'password'	=>	$_POST['password'],
			'type'		=>	$_POST['operation_type'],
			'utype'		=>	'wap'
			
		);
		$return		=	$tinyM->setResumeTinyPassword($data);
		echo json_encode($return);die;
	}
    //短信验证
    function sendmsg_action(){
        $noticeM	=	$this->MODEL('notice');
        $result		=	$noticeM->jycheck($_POST['authcode'],'普工简历');
        if(!empty($result)){
            echo json_encode(array('msg'=>$result['msg'],'error'=>$result['error']));
            return;
        }
        $moblie = $_POST['moblie'];

        $result = $noticeM->sendCode($moblie, 'code', 2, array(), 6, 120, 'msg');
        echo json_encode($result);
        exit();
    }
	function add_action(){
		if($this->config['sy_tiny_web']=="2"){
			$this->ACT_msg_wap('index.php','很抱歉！该模块已关闭！', 1, 3);
		}
		$this->get_moblie();
        $tinyM	=	$this->MODEL('tiny');
		
        $info	=	$tinyM->getResumeTinyInfo(array('id'=>intval($_GET['id'])),array('cache'=>1));
		
		if(!empty($info)){
			//检测当前密码是否对应
			session_start();
			
			if($info['password'] == $_SESSION['tinypass']){
				$this->yunset('row',$info);
				$this->yunset($info['cache']);
			}else{
				
				header("Location:".Url('wap',array('c'=>'tiny','a'=>'show','id'=>(int)$_GET['id'])));
				exit();
			}
		}
		
		if($_POST['submit']){
			$tinyM	= 	$this ->  MODEL('tiny');
			$authcode=strtolower($_POST['authcode']);

			$post	=   array(
				'username' 		=>  $_POST['name'],
				'sex' 			=>  $_POST['sex'],
				'exp' 			=>  $_POST['exp'],
				'job' 			=>  $_POST['job'],
				'mobile' 		=>  $_POST['mobile'],
				'password'		=>	$_POST['password'],
				'provinceid' 	=>  $_POST['provinceid'],
				'cityid' 		=>  $_POST['cityid'],
				'three_cityid'	=>  $_POST['three_cityid'],
				'production' 	=>  $_POST['production'],
				'status' 		=>  $this->config['user_wjl'],
				'login_ip'		=>	fun_ip_get(),
				'time'			=>	time(),
				'lastupdate'	=>	time(),
				'did'			=>	$this->userdid,
			);
			$data	=	array(
				'id'				=>	(int)$_POST['id'],
				'post'				=>	$post,
                'authcode'			=>	$authcode,
                'moblie_code'		=>	$_POST['moblie_code'],
                'verify_token'		=>	$_POST['verify_token'],
                'fast'				=>	$_COOKIE['fast'],
                'utype'				=>	'wap'
			);
						
			$return	= 	$tinyM  ->  addResumeTinyInfo($data);
			if($return['errcode']==9){
				$return['url']	=	Url('wap',array('c'=>'tiny'));
			}
			echo json_encode($return);die;
		} 
		$this->yunset($this->MODEL('cache')->GetCache(array('user')));
		
		$this->yunset("headertitle","普工专区");
		$this->yunset("title","添加普工简历");
		$this->yuntpl(array('wap/tiny_add'));
	}
	function show_action(){
		if($this->config['sy_tiny_web']=="2"){
			$this->ACT_msg_wap('index.php','很抱歉！该模块已关闭！', 1, 3);
		}
		$id			=	(int)$_GET['id'];
		$tinyM		=	$this->MODEL('tiny');
		
		$tinyM->upResumeTiny(array('hits'=>array('+',1)),array('id'=>$id));
		
		$t_info		=	$tinyM->getResumeTinyInfo(array('id'=>$id));
			
		$this->yunset("row",$t_info);
		
		$data['tiny_username']	=	$t_info['username'];
		$data['tiny_job']		=	$t_info['job'];
		$data['tiny_desc']		=	$t_info['production'];
		$this->data				=	$data;
		$this->seo('tiny_cont');
		
		$this->get_moblie();
		$this->yunset("headertitle","普工专区");
		$this->yuntpl(array('wap/tiny_show'));
	}
}
?>