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
class ajax_controller extends common{
	// wapadmin使用
	function wap_job_action(){
		include(PLUS_PATH."job.cache.php");
		
		$data	=	"<option value=''>--请选择--</option>";
		
		if(is_array($job_type[$_POST['id']])){
			foreach($job_type[$_POST['id']] as $v){
				$data	.=	"<option value='$v'>".$job_name[$v]."</option>";
			}
		}
		echo $data;
	}
	// wapadmin使用
	function wap_city_action(){
	    include(PLUS_PATH."city.cache.php");
	    
	    if(is_array($city_type[$_POST['id']])){
	        $data	=	"<option value=''>--请选择--</option>";
	        foreach($city_type[$_POST['id']] as $v){
	            $data	.=	"<option value='$v'>".$city_name[$v]."</option>";
	        }
	    }
	    echo $data;
	}
	/**
	 * 简历详情
	 * 面试邀请
	 * 2019-06-22
	 */
	function sava_ajaxresume_action(){
		
		$jobM		=	$this -> MODEL('job');

		$_POST		=	$this -> post_trim($_POST);
		$_POST['port']	=	'2';
		$fidArr		=	array(
			'fuid'		=>	$this -> uid,
			'fusername'	=>	$this -> username,
			'fusertype'	=>	$this -> usertype
		);

		$res		=	$jobM -> addYqmsInfo(array_merge($fidArr, $_POST));
		echo json_encode($res);
		die;
	}
	/**
	 * 下载简历（查看联系方式）
	 * 2019-06-24
	 */
	function forlink_action(){
		$downReM		=	$this -> MODEL('downresume');
		$data			=	array(
			'eid'		=>	$_POST['eid'],
		    'uid'		=>	$this -> uid,
		    'usertype'	=>	$this -> usertype,
			'utype'		=>	'wap',
			'did'		=>	$this -> userdid,
		);
		$downRes		=	$downReM -> downResume($data);
		
		$downRes['usertype']	 =	$this->usertype;
		if(!empty($downRes['msgList'])){
		    $downRes['msgList']  =  $downRes['msgList']['wxapp'];
		}
		echo json_encode($downRes);die;

	}

    //加入人才库（收藏简历）
	function talentpool_action(){
		
		$data		=	array(
			'eid'		=>	$_POST['eid'],
			'cuid'		=>	$this->uid,
			'usertype'	=>	$this->usertype,
			'uid'		=>	(int)$_POST['uid'],
		);
		
		$ResumeM	=	$this->MODEL('resume');
	    $return		=	$ResumeM -> addTalent($data);

		echo json_encode($return);die;
	}
    // 邀请面试
    function indexajaxresume_action()
    {
        
        if ($_POST) {
            
            $M                  =   $this->MODEL('comtc');
            
            $_POST['uid']       =   $this->uid;
            $_POST['username']  =   $this->username;
            $_POST['usertype']  =   $this->usertype;
            
            $return             =   $M->invite_resume($_POST);
            if ($return['status']) {
                $return['msgList']  =   $return['msgList']['wxapp'];
                echo json_encode($return);
                die();
            }
        }
    }
    
	//签到，TODO:会员中心
	function sign_action(){
		
	    $arr = array('type' => -2);
	    // 防止重复签到判断
	    session_start();
	    if (!isset($_SESSION['qiandao'])){
	        $_SESSION['qiandao'] = 1;
	    }else{
	        echo json_encode($arr);
	        die;
	    }
		if($_POST['rand']){
			$IntegralM	=	$this -> MODEL('integral');
		
			$userinfoM	=	$this -> MODEL('userinfo');
			
			$date		=	date("Ymd");
			
			$member		=	$userinfoM -> getInfo(array('uid'=>$this->uid,'usertype'=>$_COOKIE['usertype']),array('field'=>"`signday`,`signdays`"));
			
			$lastreg	=	$userinfoM -> getMemberregInfo(array('uid'=>$this->uid,'usertype'=>$_COOKIE['usertype'],'orderby'=>'id,desc'));
			
			$lastregdate=	date("Ymd",$lastreg['ctime']);
			
			if($lastregdate!=$date){
				
				$yesterday	=	date("Ymd",strtotime("-1 day"));
				
				if($lastregdate==$yesterday&&intval(date("d"))>1){
					
					if($member['signday']>=5){
						
						$integral	=	$this->config['integral_signin']*2;
						
					}else{
						
						$integral	=	$this->config['integral_signin'];
						
					}
					$signday	=	$member['signday']+1;
					
					$msg		=	'连续签到'.$signday."天";
					
				}else{
					$signday	=	'1';
					
					$integral	=	$this->config['integral_signin'];
					
					$msg		=	'第一次签到';
					
				}
				$arr	=	array();
				
				$nid	=	$userinfoM -> addMemberreg(array("uid"=>$this->uid,"usertype"=>$this->usertype,'date'=>$date,"ctime"=>time(),'ip'=>fun_ip_get()));
				
				if($nid){
					
				    $IntegralM->company_invtal($this->uid,$this->usertype,$integral,true,$msg,true,2,'integral');
					
					$userinfoM -> upInfo(array('uid'=>$this->uid),array('signday'=>$signday,'signdays'=>array('+','1')));
					
					$arr['type']=date("j");
					// 签到成功，请除session
					unset($_SESSION['qiandao']);
				}else{
					
					$arr['type']=-2;
					
				} 
				$arr['integral']	=	$integral.$this->config['integral_pricename'];
				
				$arr['signday']		=	$signday;
				
				$arr['signdays']	=	$member['signdays']+1;
				
			}
			echo json_encode($arr);die;
		}
	}
	//邮箱认证,发送邮件，TODO:会员中心
	function emailcert_action()
    {
        $ComapnyM   =   $this->MODEL('company');

        session_start();

        $code       =   $_POST['authcode'];

       if (md5(strtolower($code)) != $_SESSION['authcode'] || empty($_SESSION['authcode'])) {

            echo json_encode(array('error' => 2, 'msg' => '图片验证码不正确'));
            die();
        }

        $data   =   array(

            'usertype'  =>  $this->usertype,

            'did'       =>  $this->userid,

            'email'     =>  $_POST['email']
        );

        $errCode    =   $ComapnyM->sendCertEmail(array( 'uid' => $this->uid, 'type' => '1'), $data);

        echo json_encode(array('error' => $errCode ? 0 : 2, 'msg' => $errCode ? '' : '发送失败'));
        die();
    }
	
	
	//手机认证,发送短信，TODO:会员中心
	function mobliecert_action(){
	    
		$noticeM 		= 	$this->MODEL('notice');
		
		$result			=		$noticeM->jycheck($_POST['code'],'');
		if(!empty($result)){
			$this->layer_msg($result['msg'], 9, 0, '', 2, $result['error']);
		}
	  
	    if(!$this->uid || !$this->username){
	        $this->layer_msg('请先登录', 9, 0, '', 2, 110);
	    }else{
	        $shell=$this->GET_user_shell($this->uid,$_COOKIE['shell']);
	        if(!is_array($shell)){
	            $this->layer_msg('登录有误', 9, 0, '', 2, 111);
	        }
	        $moblie = $_POST['str'];
	        $user 	= array(
	            'uid'      => $this->uid,
	            'name'     => $this->username,
	            'usertype' => $this->usertype
	        );
	        
	        $result  =  $noticeM->sendCode($moblie, 'cert', 2, $user);

            $logM	    =   $this->MODEL('log');
            $logContent =   '账号认证：发送手机认证验证码';
            $logDetail  =   '手机认证，发送短信验证码；认证手机号码：'.$moblie;
            $logM->addMemberLog($user['uid'], $user['usertype'], $logContent, 12, 1, $logDetail);
			
	        echo json_encode($result);exit();
	    }
	}
	//注册会员，发送短信验证码，TODO:wap前台
    function regcode_action(){
        $this->regcode(2); // PC发送短信
    }

    //发送短信验证码，WAP快速投递
    function regcodeks_action(){
        $this->regcode(8); // WAP快速投递
    }

	function regcode($port = 2){
		$noticeM 	= 	$this->MODEL('notice');
		$result		=	$noticeM->jycheck($_POST['code'],'注册会员');
		if(!empty($result)){
			$this->layer_msg($result['msg'], 9, 0, '', 2, $result['error']);
		}
	    $moblie		= 	trim($_POST['moblie']);
	    // 两种情况验证手机号是否被使用，改为在发送验证码时验证
	    // 1-用户名注册且实名认证，需要发送短信验证码
	    // 2-手机号注册，有极验/顶象验证码
	    if ($_POST['noblur']){
	        $registerM	=	$this->MODEL('register');
	        $return 	= 	$registerM->regMoblie(array('moblie'=>$moblie));
	        
	        if($return['errcode']==0){
	            
	            $result 	= 	$noticeM->sendCode($moblie, 'regcode', $port);
	            
	            echo json_encode($result);exit();
	        }else{
	            echo json_encode($return);exit();
	        }
	    }else{
	        $result 	= 	$noticeM->sendCode($moblie, 'regcode', $port);
	        
	        echo json_encode($result);exit();
	    }
	}
	//快速申请职位入口
	function temporaryresume_action(){
		$userinfoM	=	$this->MODEL("userinfo");
		$_POST 		= 	$this->post_trim($_POST);
        $tdRes = $userinfoM->fastToudi($_POST, 2);
        echo json_encode($tdRes);die;
	}

	function pl_action(){
		$msgM	=	$this -> MODEL('msg');
		
		$blackM	=	$this -> MODEL('black');
		
		if($this->uid==''||$this->username==''){
			echo 3;die;
		}
		if($this->usertype!= '1'){
			echo 0;die;
		}
		$black	=	$blackM -> getBlackInfo(array('p_uid'=>$this->uid,'c_uid'=>(int)$_POST['job_uid']));
		if(!empty($black)){
			echo 7;die;
		}
		if(trim($_POST['content'])==""){
			echo 2;die;
		}
		if(trim($_POST['authcode'])==""){
			echo 4;die;
		}
		session_start();
		if(md5(strtolower($_POST['authcode']))!=$_SESSION['authcode'] || empty($_SESSION['authcode'])){
			echo 5;die;
		}
		$id	=	$msgM -> addInfo(array('uid'=>$this->uid,'username'=>$this->username,'jobid'=>trim($_POST['jobid']),'job_uid'=>trim($_POST['job_uid']),'content'=>trim($_POST['content']),'com_name'=>trim($_POST['com_name']),'job_name'=>trim($_POST['job_name']),'type'=>'1','datetime'=>time()));
		if($id){
			echo 1;die;
		}else{
			echo 6;die;
		}
	}
	function atn_action(){
		$data	=	array(
			'id'		=>	(int)$_POST['id'],
			'uid'		=>	$this->uid,
			'usertype'	=>	$this->usertype,
			'username'	=>	$this->username,
			'sc_usertype'=>	(int)$_POST['type']
		);
		$atnM	=	$this->MODEL('atn');
		$return	=	$atnM->addAtnLt($data);
		echo json_encode($return);die;
	}
    //关注企业
	function atncompany_action(){
		$data	=	array(
			'id'			=>	(int)$_POST['id'],
			'uid'			=>	$this->uid,
			'usertype'		=>	$this->usertype,
			'username'		=>	$this->username,
			'utype'			=>	'teacher',
			'sc_usertype'	=>	2
		);
		 
		$atnM	=	$this->MODEL('atn');
		$return	=	$atnM->addAtnLt($data);
		echo json_encode($return);die;
	}
 
	//职位类别
	function getjob_action(){
		include(PLUS_PATH."job.cache.php");
		$data   =   '';
		if(is_array($job_type[$_POST['id']])){	
		    if($_POST['type']=="jobone_son"){				
				$data	.=	'<li onclick="check_job_li(\''.$_POST['id'].'\',\'job1\');"><a href="javascript:;">全部</a></li>';
			}
			if($_POST['type']=="job_post"){				
				$data	.=	'<li onclick="check_job_li(\''.$_POST['id'].'\',\'job1_son\');"><a href="javascript:;">全部</a></li>';
			}			
			foreach($job_type[$_POST['id']] as $v){				
				if($_POST['type']=="jobone_son"){
					if(!empty($job_type[$v])){

						$data	.=	'<li class="qCategoryt'.$v.'" onclick="Categoryt(\''.$v.'\',\''.$job_name[$v].'\',\'job_post\');"><a href="javascript:;">'.$job_name[$v].'</a></li>';
					}else{
						
						$data	.=	'<li onclick="check_job_li(\''.$v.'\',\'job1_son\');"><a href="javascript:;">'.$job_name[$v].'</a></li>';
					}
						
					}else{
						$data	.=	'<li onclick="check_job_li(\''.$v.'\',\'job_post\');"><a href="javascript:;">'.$job_name[$v].'</a></li>';
				    }				
			}			
		}else{
			if($_POST['type']=="jobone_son"){				
				$data	.=	'<li onclick="check_job_li(\''.$_POST['id'].'\',\'job1\');"><a href="javascript:;">全部</a></li>';
			}
			if($_POST['type']=="job_post"){				
				$data	.=	'<li onclick="check_job_li(\''.$_POST['id'].'\',\'job1_son\');"><a href="javascript:;">全部</a></li>';
			}
		}
		echo $data;
	}
	
	
	 
	//城市类别
	function getcity_action(){
		include(PLUS_PATH."city.cache.php");
		$data   =   '';
		if(is_array($city_type[$_POST['id']])){
			if($_POST['type']=="cityid"){				
				// 后台-页面设置-列表页区域默认设置。选择了二级城市,并且是职位、简历、企业列表
				if (!empty($this->config['sy_web_city_two']) && in_array($_POST['kzq'], array('job','resume','company'))){
				    $city_type[$_POST['id']] = array($this->config['sy_web_city_two']);
				}else{
				    $data .=  '<li onclick="check_city_li(\''.$_POST['id'].'\',\'provinceid\');"><a href="javascript:;">全部</a></li>';
				}
			}
			if($_POST['type']=="three_cityid"){				
				$data	.=	'<li onclick="check_city_li(\''.$_POST['id'].'\',\'cityid\');"><a href="javascript:;">全部</a></li>';
			}		
			foreach($city_type[$_POST['id']] as $v){				
				if($_POST['type']=="cityid"){
					if(!empty($city_type[$v])){
						$data	.=	'<li class="qgradet'.$v.'" onclick="gradet(\''.$v.'\',\''.$city_name[$v].'\',\'three_cityid\');"><a href="javascript:;">'.$city_name[$v].'</a></li>';
					}else{
						$data	.=	'<li onclick="check_city_li(\''.$v.'\',\'cityid\');"><a href="javascript:;">'.$city_name[$v].'</a></li>';
					}
					
				}else{
					$data	.=	'<li onclick="check_city_li(\''.$v.'\',\'three_cityid\');"><a href="javascript:;">'.$city_name[$v].'</a></li>';	
				}
			}		
		}else{
			if($_POST['type']=="cityid"){				
				$data	.=	'<li onclick="check_city_li(\''.$_POST['id'].'\',\'provinceid\');"><a href="javascript:;">全部</a></li>';
			}
			if($_POST['type']=="three_cityid"){				
				$data	.=	'<li onclick="check_city_li(\''.$_POST['id'].'\',\'cityid\');"><a href="javascript:;">全部</a></li>';
			}
		}
		echo $data;
	}	
	 
	/**
	 * 企业报名招聘会
	 */
	function ajaxzphjob_action(){
		$data	=	array(
			'usertype'	=>	$this->usertype,
			'uid'		=>	$this->uid,
			'jobid'		=>	$_POST['jobid'],
			'id'		=>	intval($_POST['id']),
			'zid'		=>	intval($_POST['zid']),
		);
		$zphM	=	$this->MODEL('zph');
		$arr	=	$zphM->ajaxZph($data);
		
		if ($arr['status'] == 2 && !empty($_POST['jobid'])){
		    // 套餐不足，记录jobid的cookie来备用
		    $this->cookie->setcookie('zphjobid', $_POST['jobid'], time()+86400);
		}
		
	    echo json_encode($arr);die;
	}
 
	
	//天眼查工商数据获取
	function getbusiness_action(){
		if($_POST['name']){
			$noticeM	=	$this -> MODEL('notice');
			$reurn		=	$noticeM -> getBusinessInfo($_POST['name']);
			
			if(!empty($reurn['content'])){
				$comGsInfo	=	$reurn['content'];

				echo json_encode($comGsInfo);
			}
		}
	}
	
	
	
	//修改密码，TODO:会员中心
	function setpwd_action(){
		if($_POST['password']){
			$UserinfoM  =   $this->MODEL('userinfo');
			$data   	=   array(
                
                'uid'           =>  $this->uid,
                'usertype'      =>  $this->usertype,
                'oldpassword'   =>  $_POST['password'],
                'password'      =>  $_POST['passwordnew'],
                'repassword'    =>  $_POST['passwordconfirm']
                
            );
			$err    =   $UserinfoM -> savePassword($data);
			
			$arr['type']=$err['errcode'];
			$arr['msg']=$err['msg'];
			if($err['errcode'] == '9'){
                $this -> cookie -> unset_cookie();     
            }
            echo json_encode($arr);die;
		}
	}
	
	 
    //消息数
    function msgNum_action()
    {
		$M    =  $this->MODEL('msgNum');
		$arr  =  $M->getmsgNum($this->uid, $this->usertype);
		echo json_encode($arr);
    }
    // AJAX URL参数生成
    function ajax_url_action(){
        if($_POST){
            if($_POST['url']!=""){
                $urls=@explode("&",$_POST['url']);
                foreach($urls as $v){
                    if($_POST['type']=="provinceid"||$_POST['type']=="cityid"||$_POST['type']=="three_cityid"){
                        if(strpos($v,"provinceid")===false && strpos($v,"cityid")===false&& strpos($v,"three_cityid")===false){
                            $gourl[]=$v;
                        }
                    }elseif($_POST['type']=="job1"||$_POST['type']=="job1_son"||$_POST['type']=="job_post"){
                        if(strpos($v,"job1")===false && strpos($v,"job1_son")===false&& strpos($v,"job_post")===false){
                            $gourl[]=$v;
                        }
                    }elseif($_POST['type']=="nid"||$_POST['type']=="tnid"){
                        if(strpos($v,"tnid")===false&&strpos($v,"nid")===false){
                            $gourl[]=$v;
                        }
                    }else{
                        if(strpos($v,$_POST['type'])===false){
                            $gourl[]=$v;
                        }
                    }
                }
                if($_POST['id']>0){
                    $gourl=@implode("&",$gourl)."&".$_POST['type']."=".$_POST['id'];
                }else{
                    $gourl=@implode("&",$gourl);
                }
            }else{
                $gourl=$_POST['type']."=".$_POST['id'];
            }
            echo "?".$gourl;die;
        }
    }
	//wap前台商城商品类别
	function getredeem_action(){
		include(PLUS_PATH."redeem.cache.php");
		$data   =   '<li onclick="check_redeem_li(\''.$_POST['id'].'\',\'nid\');"><a href="javascript:;">全部</a></li>';
		if(is_array($redeem_type[$_POST['id']])){
			foreach($redeem_type[$_POST['id']] as $v){				
				if($_POST['type']=="tnid"){
						$data.='<li onclick="check_redeem_li(\''.$v.'\',\'tnid\');"><a href="javascript:;">'.$redeem_name[$v].'</a></li>';
					}			
			}			
		}
		echo $data;
	}

    // 企业每日最大操作次数检查
    public function ajax_day_action_check_action()
    {
        $type   =   isset($_POST['type']) ? $_POST['type'] : '';
        
        $comM   =   $this -> MODEL('company');
        $com_id =   $this->uid;
        $result =   $comM -> comVipDayActionCheck($type, $com_id);
        
        echo json_encode($result);
        die();
    }
    // 切换账号
    function notuserout_action(){
        $jobid  =   intval($_POST['jobid']);
        
        $this->cookie->unset_cookie();
        

        if ($jobid) {
        
            $url    =   Url('wap', array('c' => 'login', 'job' => $jobid));
        } else {
        
            $url    =   Url('wap', array('c' => 'login'));
        }
        
        echo $url;
        die();
    }
    //公共二维码跳转
    function pubqrcode_action(){
        
        $wapUrl = Url('wap');
		ob_clean();
		
		$toc         = isset($_GET['toc']) ? $_GET['toc'] : '';
		$toa         = isset($_GET['toa']) ? $_GET['toa'] : '';
		$totype      = isset($_GET['totype']) ? $_GET['totype'] : '';
		$twarr       = array('job','resume','company','article','announcement','jobtel','parttel','comtel','part','register','gongzhao');
		$sy_ewm_type = isset($this -> config['sy_ewm_type']) ? $this -> config['sy_ewm_type'] : '';
		
		if(in_array($sy_ewm_type, array('weixin','xcx')) && in_array($toc, $twarr) && $totype != 'wxpubtool'){
		    // 使用场景码的功能
            $WxM	=	$this -> MODEL('weixin');

            $qrcode =	$WxM->pubWxQrcode($toc,$_GET['toid'],$sy_ewm_type);
			if($qrcode){

			    $imgStr	=	CurlGet($qrcode);

			    header("Content-Type:image/png");

			    echo $imgStr;

			}

		}else{
			if( isset($_GET['toid']) && $_GET['toid'] != ''){
			    $wapUrl = Url('wap',array('c'=>$toc,'a'=>$toa,'id'=>(int)$_GET['toid']));
			}
			if($toc == 'register'){
			    $wapUrl = Url('wap',array('c'=>$toc,'uid'=>(int)$_GET['touid']));
            }
			include_once LIB_PATH."yunqrcode.class.php";
			YunQrcode::generatePng2($wapUrl,4);
		}
    }
    // 强制关注公众号场景码
    function gzhqrcode_action(){
        
        $token		= 	urldecode($_GET['token']);
        
        $tokenSalt  =  !empty($this->config['sy_safekey']) ? $this->config['sy_safekey'] : 'phpyun';
        $str 		= 	yunDecrypt($token, $tokenSalt);
        $arr 		= 	explode('|', $str);
        if(count($arr) != 3 || $arr[1] == ''){
            echo '二维码验证失败';
        }
        //根据uid查询password，对比password
        $uid = $arr[1];
        $userinfoM	=	$this->MODEL('userinfo');
        $row 		= 	$userinfoM->getInfo(array('uid'=> $uid),array('field'=>'`password`'));
        
        $password 	= 	isset($row['password']) ? $row['password'] : '';
        $password 	= 	substr($password, 0, 8);
        if($password != $arr[2]){
            echo '二维码验证失败';
        }
        ob_clean();
        
        $WxM	= $this -> MODEL('weixin');
        $qrcode = $WxM->pubWxQrcode('gzh', $uid);
        
        if($qrcode){
            
            $imgStr	=	CurlGet($qrcode);
            
            header("Content-Type:image/png");
            
            echo $imgStr;
        }
    }

	//报名招聘会条件判断
	function ajaxComjob_action(){
	    
	    if ($_POST['zph']){
	        
	        $zphM  =  $this->MODEL('zph');
	        $comrow   =  $zphM->getZphComInfo(array('uid'=>$this->uid,'zid'=>$_POST['id']));
	        
	    } 
	    if (!empty($comrow)){
	        
	        $data['status']	= 2;
	        
	        if($comrow['status']==0){
	            
	            $data['msg']	= "您已报名,请等待审核！";
	            
	        }else if($comrow['status']==1){
	            
	            $data['msg']	= "您已报名了，请不要重复报名！";
	            
	        }else if($comrow['status']==2){
	            
	            $data['msg']	= "您已报名,且审核未通过！";
	        }
	        
	    }else{
	        
	        $where	=	array(
	            'uid'		=>	$this->uid,
	            'state'		=>	1,
	            'status'	=>	0,
	            'r_status'	=>	array('<>',2),
	        );
	        if($this->config['did']){
	            $where['did']=$this->config['did'];
	        }
	        $jobM	=	$this->MODEL('job');
	        $arr	=	$jobM->getList($where, array('field'=>'`id`,`name`'));
	        $list	=	$arr['list'];
	        
	        if(!empty($list)){
	            
	            $data['list']	= $list;
	            
	            $data['status']	= 1;
	            
	        }else{
	            
	            $data['status']	= 2;
	            
	            $data['msg']	= "您还没有发布职位，请先发布职位！";
	        }
	    }
		
	    echo json_encode($data);die;
	}
    // 申请身份切换
	function applytype_action(){
	    
		$memberM		=	$this -> MODEL('userinfo');
		
		$res			=	$memberM->checkChangeApply($this->uid,$_POST['applyusertype'],$_POST['applybody']);
		echo json_encode(array('msg'=>$res['msg'],'url'=>$res['url'],'errcode'=>$res['errcode']));die;
	}

	// 获取职位海报
    function getJobHb_action()
    {

        $whbM   =   $this->MODEL('whb');
        if (!$_GET['hb']) {

            $whb        =   $whbM->getWhb(array('type' => 1, 'isopen' => 1, 'orderby' => 'sort,desc'));
            $_GET['hb'] =   $whb['id'];
        }

        $data   =   array(
            'hb'    =>  $_GET['hb'] ? $_GET['hb'] : 1,
            'id'    =>  $_GET['id']
        );

        echo $whbM->getJobHb($data);
    }

    // 获取企业海报
    function getComHb_action()
    {

        $whbM       =   $this->MODEL('whb');

        if (!$_GET['hb'] && !$_GET['style']) {

            $whb    =   $whbM->getWhb(array('type' => 2, 'isopen' => 1, 'orderby' => 'sort,desc'));
            $_GET['hb'] = $whb['id'];
        }

        $data       =   array(
            'uid'   =>  $_GET['uid']
        );

        if ($_GET['hb']) {

            $data['hb'] = $_GET['hb'];
        }

        if ($_GET['style']) {

            $data['style']  =   $_GET['style'];
        }

        if ($_GET['jobids']) {

            $data['jobids'] =   $_GET['jobids'];
        }

        echo $whbM->getComHb($data);
    }

    // 获取邀请注册海报模板列表
    function getInviteRegHbList_action()
    {
        $whbM   =   $this->MODEL('whb');
        $list   =   $whbM->getWhbList(array('type' => 3, 'isopen' => 1, 'orderby' => 'sort,desc'), array('field' => 'id'));

        echo json_encode(['list' => $list]);die;
    }

    // 获取邀请注册海报
    function getInviteRegHb_action()
    {
        $whbM   =   $this->MODEL('whb');
        if (!$_GET['hb']) {
            $whb=   $whbM->getWhb(array('type' => 3, 'isopen' => 1, 'orderby' => 'sort,desc'));
            $_GET['hb'] = $whb['id'];
        }
        if (!$this->uid && !empty($_GET['uid'])) {
            $uid = $_GET['uid'];
        } else {
            $uid = $this->uid;
        }
        $data   =   array(
            'uid'   =>  $uid,
            'hb'    =>  $_GET['hb'] ? $_GET['hb'] : 1
        );
        echo $whbM->getInviteRegHb($data);
    }

    function addJobTelLog_action(){

    	if($_POST['jobid'] || $_POST['comid']){
    		$jobM = $this->MODEL('job');

			$jobM->addTelLog(
				array(
					'uid'		=>	$this->uid,
					'usertype'	=>	$this->usertype,
					'source'	=>	2,
					'jobid'		=>	intval($_POST['jobid']),
					'comid'		=>	intval($_POST['comid'])
				)
			);
    	}
    }
	/*
	 * 数据展示
	 */
    function dataShowIndex_action(){
		$year = date("Y")-1;
        $this->yunset('year', $year);
    	$this->yunset('webname', $this->config['sy_webname']);
    	$this->yunset('datashowtitle', $this->config['sy_datashow_title']);
        $this->yunset("headertitle","数据展示");

        $title = $this->config['sy_datashow_title'] ? $this->config['sy_datashow_title'] . '招聘大数据' : '年度数据 - '.$this->config['sy_webname'];
        $desc = '年度数据 - '.$this->config['sy_webname'];
        $shareLogo = checkpic($this->config['sy_wx_sharelogo']);
        if ($this->config['sy_seo_rewrite'] == 1) {
            $url = $this->config['sy_weburl'].'/u/'.$_GET['str'];
        } else {
            $url = $this->config['sy_weburl'].'/index.php?m=upush&str='.$_GET['str'];
        }
        $this->yunset(array('title' => $title, 'description' => $desc, 'share_url' => $url, 'share_logo' => $shareLogo, 'Info' => array('photo' => $shareLogo)));
        $this->yuntpl(array('wap/data_show_index'));
    }

    /*
     * 地区分布
     */
    function cityData_action(){
		$tjM = $this->MODEL('tongji');
		$data = $tjM->cityDataShow();
		echo json_encode(array('data' => $data));
	}

    /*
     * 年龄分布
     */
    function ageData_action(){
        $tjM = $this->MODEL('tongji');
        $data = $tjM->ageDataShow();
        echo json_encode(array('data' => $data));
    }

    /*
     * 经验分布
     */
    function expData_action(){
        $tjM = $this->MODEL('tongji');
        $data = $tjM->expDataShow();
        echo json_encode(array('data' => $data));
    }

    /*
     * 性别分布
     */
    function sexData_action(){
        $tjM = $this->MODEL('tongji');
        $data = $tjM->sexDataShow();
        echo json_encode(array('data' => $data));
    }

    /*
     * 学历分布
     */
    function eduData_action(){
        $tjM = $this->MODEL('tongji');
        $data = $tjM->eduDataShow();
        echo json_encode(array('data' => $data));
    }

    /*
     * 用户活跃趋势
     */
    function userHyChart_action(){
        $tjM = $this->MODEL('tongji');
        $data = $tjM->userHyChart();
        echo json_encode(array('data' => $data));
    }

    /*
     * 用户注册趋势
     */
    function userRegChart_action(){
        $tjM = $this->MODEL('tongji');
        $data = $tjM->userRegChart();
        echo json_encode(array('data' => $data));
    }

    /*
     * 公司地区分布
     */
    function comcityData_action(){
        $tjM = $this->MODEL('tongji');
        $data = $tjM->comcityDataShow();
        echo json_encode(array('data' => $data));
    }

    /*
     * 公司规模分布
     */
    function comgmData_action(){
        $tjM = $this->MODEL('tongji');
        $data = $tjM->comgmDataShow();
        echo json_encode(array('data' => $data));
    }
    /*
     * 公司性质分布
     */
    function comxzData_action(){
        $tjM = $this->MODEL('tongji');
        $data = $tjM->comxzDataShow();
        echo json_encode(array('data' => $data));
    }

    /*
     * 企业登陆趋势
     */
    function comLogChart_action(){
        $tjM = $this->MODEL('tongji');
        $data = $tjM->comLogChart();
        echo json_encode(array('data' => $data));
    }

    /*
     * 企业岗位趋势
     */
    function comJobChart_action(){
        $tjM = $this->MODEL('tongji');
        $data = $tjM->comJobChart();
        echo json_encode(array('data' => $data));
    }
    // 生成HR年度报告海报
    function lastYearReport_action(){
        if($_GET['uid']){
            $hrlog = $this->MODEL('hrlog');
            echo $hrlog->getLastYearReportHb(array('comid'=>$_GET['uid']));
        }
    }
}	
?>