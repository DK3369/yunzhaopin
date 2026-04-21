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

class binding_controller extends company{
    /*
    查看认证和绑定页面
    **/
    function index_action(){
        
        $this   ->  company_satic();
        $this   ->  public_action();

        $company	=	$this -> comInfo['info'];
		$this -> yunset("company", $company);
        
        $uid        =   intval($this->uid);
        
        $UserinfoM  =   $this -> MODEL('userinfo');
        $member     =   $UserinfoM -> getInfo(array('uid'=> $uid), array('setname'=>'1'));
        $this -> yunset("member", $member);
		
        $comM		=   $this -> MODEL('company');
        $cert       =   $comM -> getCertInfo(array('uid'=>$uid, 'type'=>'3'));
        $this ->  yunset("cert",$cert);
        
        $this ->  com_tpl("binding");
    }
    
    /**
     * @desc 企业会员中心绑定手机
     */
    function save_action(){
        
        $CookieM   =   $this->MODEL('cookie');
        $CookieM   ->  SetCookie('delay', '', time() - 60);
        
        $ComapnyM  =   $this->MODEL('company');
        
        $data      =   array(
            'uid'		=>	$this->uid,
            'usertype'	=>	$this->usertype,
            'moblie'	=>	$_POST['moblie']
        );
		if(!$this -> comInfo['uid']){
			$userinfoM    =   $this->MODEL("userinfo");
			$userinfoM -> activUser($this->uid,2);
		}
        $errCode   =   $ComapnyM -> upCertInfo(array('uid'=>$this->uid, 'check2'=>$_POST['code']), array('status'=>'1'), $data);

        echo $errCode; die;
    }
    /**
    查看企业资质认证页面
    */
    function comcert_action(){


        $this   ->  company_satic();
        $this   ->  public_action();
        
        $uid        =   intval($this->uid);

        $company    =   $this -> comInfo['info'];
        $this -> yunset("company", $company);
        $comM       =   $this -> MODEL('company');
        $cert       =   $comM -> getCertInfo(array('uid'=>$uid, 'type'=>'3'));
        $this ->  yunset("cert",$cert);

        $this ->  com_tpl("comcert");
    }
    
    /**
     * @desc 企业会员中心，企业认证
     */
    function savecert_action(){
        
        $_POST = $this->post_trim($_POST);

        $ComapnyM  =   $this->MODEL('company');

        $CookieM   =   $this->MODEL('cookie');
        $CookieM   ->  SetCookie('delay', '', time() - 60);
        
        $uid       =   intval($this->uid);
        $usertype  =   intval($this->usertype);

        $row       =   $ComapnyM -> getCertInfo(array('uid'=>$uid, 'type' => '3'));
        
        if($this ->comInfo['r_status']==0){
            $status=   0;
        }else{
            $status=   $this->config['com_cert_status'] == '1' ? 0 : 1;
        }
        /* 更新企业资质参数整理 */
        $upData    =   array(
            'status'    =>  $status,
            'ctime'     =>  time()
        );
        //判断是否上传必要资质
        $errcode     =   0;
        $msg         =   '必须上传';
        $douhao      =   false;

        if($this->config['com_social_credit']==1 && !$_POST['social_credit']){
            if($douhao){
                $msg     .= ',';
            }
            $douhao  =   true;
            $msg     .=  '统一社会信用代码';
            $errcode =   8;
        }

        if($this->config['com_cert_owner']==1 && !$_FILES['owner_cert']['tmp_name'] && !$row['owner_cert']){
            if($douhao){
                $msg     .= ',';
            }
            $douhao  =   true;
            $msg     .=  '经办人身份证';
            $errcode =   8;
        }
        
        if($this->config['com_cert_wt']==1 && !$_FILES['wt_cert']['tmp_name'] && !$row['wt_cert']){
            if($douhao){
                $msg     .= ',';
            }
            $douhao  =   true;
            $msg     .=   '委托函';
            $errcode =   8;
        }
        
        if($errcode==8){
            $this->ACT_layer_msg($msg,8,$_SERVER['HTTP_REFERER']);
        }
        //判断是否上传必要资质end
        if($_POST['social_credit']){
            $upData['social_credit'] = $_POST['social_credit'];
        }
        if($_FILES['check'] && $_FILES['check']['tmp_name']){
            $upData['check'] = $_FILES['check'];
        }
        if($_FILES['owner_cert'] && $_FILES['owner_cert']['tmp_name']){
            $upData['owner_cert'] = $_FILES['owner_cert'];
        }
        if($_FILES['wt_cert'] && $_FILES['wt_cert']['tmp_name']){
            $upData['wt_cert'] = $_FILES['wt_cert'];
        }
        if($_FILES['other_cert'] && $_FILES['other_cert']['tmp_name']){
            $upData['other_cert'] = $_FILES['other_cert'];
        }
        
        if (!empty($row) && is_array($row) && $row['ctime']) {
            
            $err   =   $ComapnyM -> upCertInfo(array('id'=>intval($row['id']), 'uid' => $uid), $upData, array('yyzz' => '1', 'usertype' => $usertype, 'com_name'=>trim($_POST['company_name']),'type'=>'3'));
            
        }else{
            
            /* 新增企业资质认证参数整理，包含自定义查询参数  */
            $postData  =   array(
                
                'uid'      =>  $uid,
                'type'     =>  '3',
                'step'     =>  '1',
                'did'      =>  $this->userdid,
                'usertype' =>  $usertype,
                'com_name' =>  trim($_POST['company_name'])
            );
            
            $postData  =   array_merge($postData, $upData);
			//容错机制，在未生成企业身份时提前企业资质认证会出错
            if(!$this -> comInfo['info']['uid']){
				$userinfoM    =   $this->MODEL("userinfo");
				$userinfoM -> activUser($uid,2);
			}
            $err       =   $ComapnyM -> addCertInfo($postData);
        }
        
        
        $this->ACT_layer_msg($err['msg'],$err['errcode'],$_SERVER['HTTP_REFERER']);
        
    }
    /**
     * @desc 解除绑定   
     */
    function del_action(){
        
        $companyM	=	$this->MODEL('company');
        
        $return 	=	$companyM->delBd($this->uid,array('type'=>$_GET['type'],'usertype'=>$this->usertype));
        
        $this->layer_msg($return['msg'],$return['errcode'],0,$_SERVER['HTTP_REFERER']);
    }
}
?>