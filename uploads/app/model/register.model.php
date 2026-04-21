<?php

/**
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
 * 版权所有 2009-2022 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class register_model extends model
{
    /**
     * @desc   引用log类，添加用户日志
     * @param $uid
     * @param $usertype
     * @param $content
     * @param string $opera
     * @param string $type
     * @param string $detail
     * @return void
     */
    private function addMemberLog($uid, $usertype, $content, $opera = '', $type = '', $detail = '')
    {

        require_once('log.model.php');
        $LogM = new log_model($this->db, $this->def);
        return $LogM->addMemberLog($uid, $usertype, $content, $opera, $type, $detail);
    }

    /**
     * 验证注册输入内容
     * @param array $data 自定义处理数组 | username、email、realname
     * @return mixed
     */
    public function ajaxReg($data = array())
    {
		
		$post		=	$data['post'];
		if($post['username']){
			$key_name 	=	'username';
		}elseif($post['email']){
			$key_name 	=	'email';
		}elseif($post['password']){
			$key_name 	=	'password';
		}
		if($key_name=="username"){//验证注册用户名
			
			$username		= 	$post['username'];

			$username 	=	str_replace('！', '!', $username);
			
			$msg		=	regUserNameComplex($username);//检测用户名复杂度

			if($this->config['sy_uc_type']=="uc_center"){
				include APP_PATH.'data/api/uc/config.inc.php';
				include APP_PATH.'/api/uc/include/db_mysql.class.php';
				include APP_PATH.'/api/uc/uc_client/client.php';
				$user 		= 	uc_get_user($username);
			}else{
				$user 		= 	$this->select_num('member',array("username"=>$username));
			}

			if($msg!=''){
				$return['msg']		=	$msg;
				$return['errcode']	=	4;
			}elseif($user){
			
				$return['msg']		=	'用户名已存在！';
				$return['errcode']	=	1;
			}elseif(CheckRegUser($username)==false && CheckRegEmail($username)==false){

				$return['msg']		=	'用户名不得包含特殊字符！';
				$return['errcode']	=	2;
            } elseif (!checkRegnameLegal($username)) {//限制注册用户名

                $return['msg'] = '该用户名已被禁止注册！';
                $return['errcode'] = 3;
            } else {
				$return['msg']		=	'填写正确！';
				$return['errcode']	=	0;
			}
		}elseif($key_name=="email"){//验证注册邮箱
			
			$user 		= 	$this->select_num('member',array("email" => $post['email'], "username" => array('=', $post['email'], 'OR')));
			
			if(CheckRegEmail($post['email'])==false){
				$return['msg']		=	'邮箱格式错误！';
				$return['errcode']	=	2;
			}elseif($user){
				$return['msg']		=	'邮箱已被使用！';
				$return['errcode']	=	1;
			}else{
				$return['msg']		=	'填写正确！';
				$return['errcode']	=	0;
			}
		}elseif($key_name == 'realname'){//验证注册真实姓名
			
			$realname=$post['realname'];

            if (!checkRegnameLegal($realname)) {//限制注册用户名
                $return['msg'] = '该姓名已被禁止使用！';
                $return['errcode'] = 3;
            }
			if($this->config['sy_fkeyword']!=""){//过滤关键词
				
				$fkeyword =@explode(",",$this->config['sy_fkeyword']);
				
				if(in_array($realname,$fkeyword)){
					$return['msg']		=	'该真实姓名包含敏感词！';
					$return['errcode']	=	3;
				}
			}
		}else if($key_name=="password"){//验证密码复杂度
			
			$password		= 	$post['password'];

			$msg	=	regPassWordComplex($password);//检测密码复杂度

			if($msg!=''){
				$return['msg']		=	$msg;
				$return['errcode']	=	4;
			}else{
				$return['msg']		=	'填写正确！';
				$return['errcode']	=	0;
			}
		}
		
		return	$return;
	}

    function regMoblie($post = array())
    {
	    if(isset($post['moblie']) || isset($post['email'])){
            if (isset($post['moblie'])) {//限制会员手机号
                if (!checkMobileLegal($post['moblie'])) {
                    $return['errcode'] = 2;//该手机号已被禁止使用！

                    return $return;
                }
            }
			require_once ('userinfo.model.php');
			$userinfoM = new userinfo_model($this->db, $this->def);
			
			if(isset($post['moblie'])){//验证moblie
				
				$m_info 		= 	$userinfoM->getInfo(array('moblie'=>$post['moblie'],'username'=>array('=',$post['moblie'],'OR')),array('field'=>"`uid`,`usertype`,`username`,`moblie`"));
				
			}elseif(isset($post['email'])){//验证email
				
				$m_info 		= 	$userinfoM->getInfo(array('email'=>$post['email'],'username'=>array('=',$post['email'],'OR')),array('field'=>"`uid`,`usertype`,`username`,`email`"));
			}
			// 首先只验证member表，让提示的身份是当前用户身份，减少提示其他身份造成误解

			if(isset($m_info)&&!empty($m_info)){
			    
			    
		        $info 	= 	$userinfoM->getUserInfo(array('uid'=>$m_info['uid']),array('usertype'=>$m_info['usertype'],'field'=>"name"));
		        
		        $data['name']	=	$info['name'];
			        
			    
			    $data['uid']		=	$m_info['uid'];
			    $data['usertype']	=	$m_info['usertype'];
			    
			}else{
			    // member表没查到，再分别验证
			    if(isset($post['moblie'])){//验证moblie
			        
			        $resume_info 	= 	$userinfoM->getUserInfo(array('telphone'=>$post['moblie'],'moblie_status'=>1),array('usertype'=>'1','field'=>"uid,name"));
			        
			        $company_info 	= 	$userinfoM->getUserInfo(array('linktel'=>$post['moblie'],'moblie_status'=>1),array('usertype'=>'2','field'=>"uid,name"));
			        
			        
			    }elseif(isset($post['email'])){//验证email
			        
			        $resume_info 	= 	$userinfoM->getUserInfo(array('email'=>$post['email'],'email_status'=>1),array('usertype'=>'1','field'=>"uid,name"));
			        
			        $company_info 	= 	$userinfoM->getUserInfo(array('linkmail'=>$post['email'],'email_status'=>1),array('usertype'=>'2','field'=>"uid,name"));
			        
			    }
			    $data  =  array();
			    if(isset($resume_info) && !empty($resume_info)){
			        
			        $data['name']		=	$resume_info['name'];
			        $data['uid']		=	$resume_info['uid'];
			        $data['usertype']	=	'1';
			        
			    }else if(isset($company_info) && !empty($company_info)){
			        
			        $data['name']		=	$company_info['name'];
			        $data['uid']		=	$company_info['uid'];
			        $data['usertype']	=	'2';
			        
			    }

			}

			if(!empty($data)){
				$return['errcode']	=	9;
				$return['data']		=	$data;
			}else{
				$return['errcode']	=	0;//填写正确！
			}
			
			return $return;
		}
	}

	/**
	 * 注册时，填写的手机号、邮箱已存在，操作解除绑定
	 * @param array $data
	 * @return string
	 */
    function writtenOff($data = array())
    {
	    if(isset($data['code'])){
			session_start();
		
			if(md5(strtolower($data['code']))!=$_SESSION['authcode'] || empty($_SESSION['authcode'])){
				unset($_SESSION['authcode']);
				$return['errcode']	=	3;
				$return['msg']		=	'验证码错误！';
			}
		}
		$user 	= 	$this->select_once('member',array("uid" => $data['zyuid']),'username,moblie,status,email,uid,password,salt,usertype');
		
		if(!passCheck($data['pw'],$user['salt'],$user['password'])){
			
			$return['errcode']	=	2;
			$return['msg']		=	'密码错误！';
		}else{
			
			if($data['mobile']!=""){
				if($user['status'] == 2){
                    $return['errcode']	=	4;
                    $return['msg']		=	'账号已锁定无法解绑！';
                    return $return;
                }else {
					if($user['username'] == $data['mobile']){
						$str 	= 	md5(uniqid(mt_rand(), true));   
						$new 	= 	"yun".substr($str,0,8);
						$newid 	= 	$this->update_once("member",array('username'=>$new,'restname'=>0),array('uid'=>$data['zyuid']));
					}
					
					if($user['moblie'] == $data['mobile']){
						
						$nid	=	$this->update_once('member', array('moblie'=>'', 'moblie_status'=>0), array('uid'=>$data['zyuid']));
						$this->update_once('resume',array('telphone'=>'', 'moblie_status'=>0), array('uid'=>$data['zyuid']));
						
						include_once('resume.model.php');
						$resumeM   =   new resume_model($this->db, $this->def);
						$resumeM->setExpectState(array('state'=>'0'),array('uid'=>$data['zyuid']));
						
						$this->update_once('company',array('linktel'=>'', 'moblie_status'=>0), array('uid'=>$data['zyuid']));
					}

                    if ($newid) {

                        $logContent =   '修改账号：账号手机解绑';
                        $logDetail  =   '账号手机解绑，用户名和手机号一致，同步修改用户名：《'.$user['username'].'》 → 《'.$new.'》';
                        $this->addMemberLog($user['uid'], $user['usertype'], $logContent, 11, 1, $logDetail);
                    }
                    if ($nid) {
                        $logContent =   '账号认证：解除手机绑定';
                        $logDetail  =   '注册处解除绑定手机：'.$data['mobile'];
                        $this->addMemberLog($user['uid'], $user['usertype'], $logContent, 12, 2, $logDetail);
                    }
				}
				
			}else if($data['email']!=""){
				if($user['status'] == 2){
                    $return['errcode']	=	4;
                    $return['msg']		=	'账号已锁定无法解绑！';
                    return $return;
                }else {
					if($user['username'] == $data['email']){
						$str 	= 	md5(uniqid(mt_rand(), true));   
						$new 	= 	"yun".substr($str,0,8);
						$newid 	= 	$this->update_once("member",array('username'=>$new,'restname'=>0),array('uid'=>$data['zyuid']));
					}
					
					if($user['email'] == $data['email']){
						
						$nid    =   $this->update_once("member",array('email'=>'','email_status' => 0),array('uid'=>$data['zyuid']));
						$this->update_once("resume",array('email'=>'','email_status'=>0),array('uid'=>$data['zyuid']));
						$this->update_once("company",array('linkmail'=>'','email_status'=>0),array('uid'=>$data['zyuid']));
					}
                    if ($newid) {

                        $logContent =   '修改账号：账号邮箱解绑';
                        $logDetail  =   '账号邮箱解绑，用户名和邮箱一致，同步修改用户名：《'.$user['username'].'》 → 《'.$new.'》';
                        $this->addMemberLog($user['uid'], $user['usertype'], $logContent, 11, 1, $logDetail);
                    }
                    if ($nid) {
                        $logContent =   '账号认证：解除邮箱绑定';
                        $logDetail  =   '注册处解除绑定邮箱：'.$data['email'];
                        $this->addMemberLog($user['uid'], $user['usertype'], $logContent, 12, 2, $logDetail);
                    }
				}
			}
			$return['errcode']	=	1;
			$return['msg']		=	'解绑成功！';
		}
		return $return;
	}

    public function checkRegFirst($data = array())
    {

        if (!empty($data)){

            if (isset($data['c_name'])){

                $isExist    =   $this->select_once('company', array('name' => $data['c_name']));

            }else if (isset($data['p_name'])){

                $isExist    =   $this->select_once('px_train', array('name' => $data['c_name']));
            }

            $return['errcode']  =   $isExist ? 1 : 0;
        }

        return $return;
    }
}

?>