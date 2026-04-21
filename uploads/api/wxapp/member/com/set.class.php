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

class set_controller extends com_controller
{
    
	function getCert_action()
	{

		$comM		=   $this -> MODEL('company');
		$comCert	=   $comM -> getCertInfo(array('uid' => $this->member['uid'], 'type' => '3'));
		
        $data['comname']    =   $this->comInfo['name'];
        $data['statusbody'] =   $comCert['statusbody'];
        
        // 首先根据企业表的认证状态来判断
        if ($this->comInfo['yyzz_status'] == 1){
            $data['status'] = 1;
        }else{
            $data['status'] = isset($comCert['status']) ? $comCert['status'] : -1;
        }

        $data['com_social_credit']	=   $this->config['com_social_credit'];
        $data['com_cert_owner']	    =   $this->config['com_cert_owner'];
        $data['com_cert_wt']	    =   $this->config['com_cert_wt'];
        $data['com_cert_other']	    =   $this->config['com_cert_other'];
        // 小程序里面因安全域名限制，在开启oss的情况下，委托书/承诺函范本上传时服务器也传了一份，这里直接用服务器上的
        $data['exa_cert_wt']        =   $this->config['exa_cert_wt'] ? $this->config['sy_weburl'].$this->config['exa_cert_wt']:'';
        $data['pic_type']           =   $this->config['pic_type'];
        $data['file_maxsize']       =   $this->config['file_maxsize'];
        $data['cert_status']        =   $this->config['com_cert_status'];
        
        if($comCert['check'] || $comCert['owner_cert'] || $comCert['wt_cert'] || $comCert['other_cert']){
            
            $data['social_credit']  =   $comCert['social_credit'];
            $data['url']            =   $comCert['old_check'];
            $data['ocurl']          =   $comCert['old_owner_cert'];
            $data['wturl']          =   $comCert['old_wt_cert'];
            $data['otherurl']       =   $comCert['old_other_cert'];
	        $this->render_json(1,'ok',$data);
	    }else{
	        
	        $this->render_json(2,'',$data);
	    }

	}

	//上传企业认证图片处理返回url
	function upCertPic_action(){

		$UploadM		=	$this	->	MODEL('upload');

		$picurl			=	'';
		$msg			=	'';
		$error			=	'';

		if(isset($_FILES['file'])){
				    // pc端上传
		    $upArr    	=  array(
		        'file'  =>  $_FILES['file'],
		        'dir'   =>  'cert'
		    );
		    $uploadM  	=	$this->MODEL('upload');
		    $pic      	=	$uploadM->newUpload($upArr);

		    if (!empty($pic['msg'])){

		    	$error	=	2;
		        $msg 	= 	$pic['msg'];

		    }elseif (!empty($pic['picurl'])){
		        $error	=	1;
		        $picurl =  $pic['picurl'];
		    }
		}else{
			$error	=	2;
		    $msg 	= 	'请选择图片';
		}

		$this->render_json($error,$msg,$picurl);
	}

	//上传企业资质
	function saveCert_action()
	{
		
		$comM		=   $this -> MODEL('company');

		if($this->comInfo['r_status']==0){
		    
		    $status	=	$this->comInfo['r_status'];
		}else{

		    $status	=	$this -> config['com_cert_status'] == '1' ? 0 : 1;
		}

        $upData     =   array(
            'ctime'     =>  time(),
            'status'    =>  $status
        );
		if($_POST['social_credit']){
			$upData['social_credit']    =   $_POST['social_credit'];
		}
		if($_POST['check']){

			$upData['check']            =   $_POST['check'];
		}else if($_POST['base']){

            $upData['base']             =   $_POST['base'];
        }
		if($_POST['owner_cert']){

			$upData['owner_cert']       =   $_POST['owner_cert'];
		}else if($_POST['base_owner_cert']){

            $upData['base_owner_cert']  =   $_POST['base_owner_cert'];
        }
		if($_POST['wt_cert']){

			$upData['wt_cert']          =   $_POST['wt_cert'];
		}else if($_POST['base_wt_cert']){

            $upData['base_wt_cert']     =   $_POST['base_wt_cert'];
        }
		if($_POST['other_cert']){

			$upData['other_cert']       =   $_POST['other_cert'];
		}else if($_POST['base_other_cert']){

            $upData['base_other_cert']  =   $_POST['base_other_cert'];
        }

		$cert       =   $comM -> getCertInfo(array('uid' =>$this->member['uid'], 'type' => '3'));
		
		//判断是否上传必要资质
        $errcode    =   0;
        $msg        =   '必须上传';
        $douhao     =   false;


        if($this->config['com_cert_owner']==1 && !$_POST['owner_cert'] && !$cert['owner_cert'] && !$_POST['base_owner_cert']){
            if($douhao){
                $msg    .=  ',';
            }
            $douhao     =   true;
            $msg        .=  '经办人身份证';
            $errcode    =   8;
        }
        if($this->config['com_cert_wt']==1 && !$_POST['wt_cert'] && !$_POST['base_wt_cert'] && !$cert['wt_cert']){
            if($douhao){
                $msg    .=  ',';
            }
            $douhao     =   true;
            $msg        .=  '委托函';
            $errcode    =   8;
        }
        
        if($errcode==8){
        	$this->render_json(2,$msg);
		}
        //判断是否上传必要资质end

		if (!empty($cert) && $cert['ctime']) {
		     
		    $err        =   $comM -> upCertInfo(array('id'=>$cert['id'], 'uid' => $this->member['uid']), $upData, array('yyzz' => '1', 'usertype' => 2, 'com_name'=>trim($_POST['name'])));
			
		}else{
			$postData   =   array(
				'uid'       =>  $this->member['uid'],
				'type'      =>  '3',
				'step'      =>  '1',
				'did'       =>  $this ->config['did'],
				'usertype'  =>  2,
			    'com_name'  =>  trim($_POST['name'])
			);
			$postData	=   array_merge($postData, $upData);
			$err		=   $comM -> addCertInfo($postData);
		}

		if($err){
			$error		=	$err['errcode']==9 ? 1 : 2;
			if($error==1){
                $msg    =   '更新成功！';
			}else{
			    $msg    =   !empty($err['msg']) ? $err['msg'] : '更新失败！';
			}
		}
		$this->render_json($error,$msg);
	}


	//手机认证,发送短信；
	function mobliecert_action()
	{
		
		$com 	=	array(
		    'uid'      => $this->member['uid'],
		    'usertype' => '2'
		);
		$moblie		=	trim($_POST['moblie']);
		$noticeM	=	$this->MODEL('notice');

		$result		=	$noticeM->sendCode($moblie, 'cert', $this->port, $com);

		if ($result['error'] == 1){

            $logM       =   $this->MODEL('log');
            $logContent =   '账号认证：发送手机认证验证码';
            $logDetail  =   '手机认证，发送短信验证码；认证手机号码：'.$moblie;
            $logM->addMemberLog($com['uid'], $com['usertype'], $logContent, 12, 1, $logDetail);

		    $this->render_json(0,'ok');
		}else{
		    $this->render_json($result['error'],$result['msg']);
		}
	}

	function bindingbox_action()
	{
		$comM				=   	$this -> MODEL('company');

		$UserinfoM   		=   	$this->MODEL('userinfo');

		$uid          		=   	$this->member['uid'];
	    if($_POST['id']=='tel'){

			$moblie      		=    	$_POST['moblie'];
		
		  	$where['uid']      	=   	array('<>',$uid);
			
			$where['moblie']   	=   	$moblie;
		
			$Info              	=  		$UserinfoM->getInfo($where);
			if($Info){
				$this	->	render_json(2,'手机号码已存在，请重新填写新号码','');
			}else{
				$data     =   array(
					'uid'         =>	$this ->member['uid'],
					'usertype'    =>	$this ->member['usertype'],
					'moblie'      =>	$_POST['moblie'],
				);
				$return  =  array();
				$user    =  $UserinfoM->getInfo(array('uid'=>$uid),array('field'=>'username,moblie,password,salt,usertype'));
				if (isset($_POST['provider']) && $user['username'] == $user['moblie']){
				    // 用户名和手机号重复，修改手机号会修改用户名，需要重新生成token;
				    $token  =  md5($data['moblie'].$user['password'].$user['salt'].$user['usertype']);
				    $return['user']  =  array('uid'=>$uid,'usertype'=>$user['usertype'],'token'=>$token);
				}
				$result  =   $comM -> upCertInfo(array('uid'=>$this ->member['uid'], 'check2'=>$_POST['code']), array('status'=>'0'), $data);
				
				if($result==1){
				    $this	->	render_json(0,'手机绑定成功',$return);	
				}if($result==4){
					
				   $this	->	render_json(4,'短信验证码已过期，请重新发送！');
					
				}else  if($result==3){
					$this	->	render_json(3,'短信验证码不正确！');
				}else if($result==2){
					$this	->	render_json(2,'请先获取短信验证码！');
				}else{
					$this	->	render_json(2,'手机绑定失败！');
				}

			}

	    }elseif ($_POST['id']=='email'){
            if(!empty($_POST['source']) && $_POST['source'] == 'wap'){
                session_start();
                $code       =   $_POST['authcode'];
                if (md5(strtolower($code)) != $_SESSION['authcode'] || empty($_SESSION['authcode'])) {
                    $error	=	4;
                    $data['errmsg']	=	'验证码不正确';
                    $this	->	render_json($error,$data['errmsg']);
                }
            }

			$email      		=    	$_POST['email'];

			$where['uid']      	=   	array('<>',$uid);
			  
          	$where['email']   	=   	$email;
          
			$Info              	=  		$UserinfoM->getInfo($where);

			if($Info){
				$error	=	2;
				$data['errmsg']	=	'邮箱已存在，请重新填写邮箱';
				$this	->	render_json($error,$data['errmsg']);
			}else{
				$data      =   array(
					'usertype' =>  $this ->member['usertype'],
					'email'    =>  $_POST['email']
				);
				
				$return   =   $comM -> sendCertEmail(array('uid'=>$this->member['uid'], 'type'=>'1'), $data);
				if($return=='1'){
					$error	=	0;
					$data['errmsg']	=	'邮箱绑定成功';
					$this	->	render_json($error,$data['errmsg']);
				}elseif($return  == 3){
					$data['errmsg']    		=    	'邮件没有配置，请联系管理员！';
					$this	->	render_json($return,$data['errmsg']);
				}elseif($return ==2){
					$data['errmsg']			=		'邮件通知已关闭，请联系管理员';
					$this	->	render_json($return,$data['errmsg']);
				}else{
					$data['errmsg']	=	'操作错误';
					$this	->	render_json($return,$data['errmsg']);
				}
				
			}

	    }

	}

	function pwd_action()
	{
		if($_POST['newpwd']){
			$UserinfoM  =   $this->MODEL('userinfo');
			$data   	=   array(
			    'uid'           =>  $this->member['uid'],
			    'usertype'      =>  $this->member['usertype'],
                'oldpassword'   =>  $_POST['oldpwd'],
                'password'      =>  $_POST['newpwd'],
                'repassword'    =>  $_POST['confirmpwd']
            );
			$return    =   $UserinfoM -> savePassword($data);
			
		}
		$data['error']	=	$return['errcode']==9 ? 0 : 2;
		
		$this -> render_json($data['error'], $return['msg']);

	}
	
	function setname_action(){
		$UserinfoM	=	$this->MODEL('userinfo');
		$data	=	array(
			'username'	=>  trim($_POST['username']),
			'password'	=>  trim($_POST['password']),
			'uid'		=>  $this->member['uid'],
			'usertype'	=>  2,
			'restname'	=>  '1'
		);
		if (!empty($data['username'])) {
			$return =	$UserinfoM->saveUserName($data);
			if($return['errcode'] == '1'){
				$error	=	1;
			}else{
				$error	=	2;
				$msg	=	$return['msg'];
			}
		}else{
			$error	=	2;
			$msg	=	'修改失败';
		}
		$this -> render_json($error, $msg,$return);
	}
	function upMap_action(){
	    $companyM	=	$this->MODEL('company');
	    if($_POST){
	        $data	=	array(
	            'xvalue'	=>	$_POST['x'],
	            'yvalue'	=>	$_POST['y']
	        );
	        $return	=	$companyM->setMap($this->member['uid'],$data);
	        if($return['cod']  == '9'){
	            $error	=	1;
	        }else{
	            $error	=	2;
	        }
	        $this->render_json($error, $return['msg']);
	    }
	}
	function getBind_action(){
	    
	    $userInfoM  =  $this->MODEL('userinfo');
	    $member     =  $userInfoM->getInfo(array('uid'=>$this->member['uid']),array('field'=>'`qqid`,`qqunionid`,`wxid`,`unionid`,`sinaid`'));
	    
	    $return  =  array(
	        'qqbind'    =>  0,
	        'wxbind'    =>  0,
	        'sinabind'  =>  0
	    );
	    if (isset($this->config['sy_qqdt']) && $this->config['sy_qqdt'] == 1 && !empty($member['qqunionid'])){
	        $return['qqbind']  =  1;
	    }elseif (!empty($member['qqid'])){
	        $return['qqbind']  =  1;
	    }
	    if ($_POST['provider'] == 'h5'){
	        if (!empty($member['wxid']) || !empty($member['unionid'])){
	            $return['wxbind']  =  1;
	        }
	    }
	    if (!empty($member['sinaid'])){
	        $return['sinabind']  =  1;
	    }
	    $this->render_json(0, 'ok', $return);
	}
	
	function binding_action()
	{
	    $userInfoM  =  $this->MODEL('userinfo');
	    
	    if ($_POST['isbind'] == 1){
	        
	        $uni = 'wxapp';
	        if ($_POST['type'] == 'weixin'){
	            
	            $up  =  array('wxid'=>'','wxopenid'=>'',,'unionid'=>'', 'subscribe' => 2);
	            
	        }elseif ($_POST['type'] == 'qq'){
	            
	            $up  =  array('qqid'=>'','qqunionid'=>'');
	            $uni =  'APP/QQ';
	            
	        }elseif ($_POST['type'] == 'sinaweibo'){
	            
	            $up  =  array('sinaid'=>'');
	            $uni =  'APP/weibo';
	        }
	        
	        $userInfoM->upInfo(array('uid'=>$this->member['uid']), $up);

            $logM       =   $this->Model('log');
            $logContent =   '账号认证：解除绑定';
            $logDetail  =   $uni.'解除绑定';
            $logM->addMemberLog($this->member['uid'], $this->member['usertype'], $logContent, 12, 3, $logDetail);
	        
	        $this->render_json(0, 'ok');
	    }
	}
	// 查询申请记录
	function getLogout_action()
	{
	    $logoutM  =  $this->MODEL('logout');
	    $row      =  $logoutM->getInfo(array('uid'=>$this->member['uid']));
	    
	    if (!empty($row)){
	        
	        $this->render_json(1,'您已申请了注销账号');
	    }else{
	        
	        $this->render_json(0,'ok');
	    }
	}
	//注销账号申请
	public function logoutApply_action()
	{
        $_POST  =   $this->post_trim($_POST);
        $p      =   array(
            'password' => $_POST['password']
        );

        $logoutM=   $this->MODEL('logout');
        $return =   $logoutM->apply(array('uid' => $this->member['uid']), $p);

        if ($return['errcode'] == 9) {

            $this->render_json(0, 'ok');
        } else {

            $this->render_json($return['errcode'], $return['msg']);
        }
	}
    //手机注销账号申请
    public function logoutmsg_action()
    {
        $_POST      =  $this->post_trim($_POST);
        $p          =  array(
            'mobile'  =>  $_POST['mobile'],
            'uid'     =>  $_POST['uid'],
            'code'    =>  $_POST['code'],
        );

        $logoutM    =  $this->MODEL('logout');
        $return     =  $logoutM->applymsg(array('uid'=>$this->member['uid']), $p);


        if ($return['errcode'] == 9) {

            $this->render_json(0, 'ok');
        } else {

            $this->render_json($return['errcode'], $return['msg']);
        }
    }
	function gettel_action()
    {
        $userM = $this -> MODEL('userinfo');
        $data  = $userM -> getInfo(array('uid' => $this->member['uid']));

        $this->render_json(0,'',$data);
    }
	/**
	 * 邀请模板列表
	 */
	function yqmb_action(){
	    
	    
		$yqmbM              =   $this -> MODEL('yqmb');

        $page				=	$_POST['page'];

        $where['uid'] 	    =   $this ->member['uid'];
        		
		$where['orderby']	=	'id,desc';		

		$limit				=	$_POST['limit'] ? $_POST['limit'] : 10;

		if($page){//分页
			$pagenav		=	($page-1)*$limit;
			$where['limit']	=	array($pagenav,$limit);
		}else{
			$where['limit']	=	$limit;
		}

        $rows	=	$yqmbM -> getList($where);

        $mbnum	=	$yqmbM -> getNum(array('uid' => $this->member['uid']));
		
        $list	=	count($rows) ? $rows : array();

        $data['list'] 	= $list;

        $data['mbnum'] 	= $mbnum ? $mbnum : 0;

        $data['maxnum'] = $this->config['com_yqmb_num'] ? $this->config['com_yqmb_num'] : 0;

		$this->render_json(0,'',$data);
	}
	
	/**
     * 删除模板
     */
    public function delYqmb_action()
    {
        $_POST  =   $this -> post_trim($_POST);

        $yqmbM  =   $this -> MODEL('yqmb');

        $res    =   $yqmbM->delYqmb($_POST['id'],array('uid'=>$this->member['uid']));
        $error	=	$res['errcode']==9 ? 1 : 2;
        $msg	=	$res['msg'];
        $this	->	render_json($error,$msg);
    }
    
    function yqmbedit_action(){

		$yid		=	intval($_POST['yid']);
		$yqmbM      =   $this -> MODEL('yqmb');
		$error		=	1;
		
		if(empty($yid)){
			$mbnum	=	$yqmbM->getNum(array('uid'=>$this->member['uid']));
			
			$info   =  array(
			    'name'     =>  '',
			    'linkman'  =>  '',
			    'linktel'  =>  '',
			    'address'  =>  '',
			    'content'  =>  ''
			);
			if($mbnum>=$this->config['com_yqmb_num']){
				$msg	=	'最多可以创建'.$this->config['com_yqmb_num'].'个模板';	
				$error	=	3;
			}
		}else{
			$info		=	$yqmbM -> getInfo(array('id' => $yid));
			if(empty($info)){
				$msg	=	'模板不存在';
				$error	=	2;
			}
		}

		$this->render_json($error,$msg,$info);
		
		
	}
	function yqmbeditsave_action(){
		$_POST      =   $this -> post_trim($_POST);
        
		$yqmbM 		= 	$this->MODEL('yqmb');
        $yid       	=   intval($_POST['yid']);

        $where  	=   array();
		if($yid){
            
            $where['id']=   $yid;
            
        }
        
        $data = array(
            'uid'       =>  $this->member['uid']
        );
        $setdata = array(
            'name'      => $_POST['name'],
            'linkman'   => $_POST['linkman'],
            'linktel'   => $_POST['linktel'],
            'content'   => $_POST['content'],
            'intertime' => $_POST['intertime'],
            'address'   => $_POST['address'],
        );
        $return         =   $yqmbM->addInfo($setdata,$data,$where);

        $error		=	$return['errcode'] == 9 ? 1:2;
        $msg		=	$return['msg'];

        $this->render_json($error,$msg);
	}
    /**
     * 工作地址列表
     */
    function address_action()
    {

        $addressM       =   $this->MODEL('address');

        $page           =   $_POST['page'];

        $where['uid']   =   $this->member['uid'];
        $total          =   $addressM->getAddressNum($where);

        $where['orderby']   =   'id';

        $limit              =   $_POST['limit'] ? $_POST['limit'] : 10;

        if ($page) {

            $pagenav        =   ($page - 1) * $limit;
            $where['limit'] =   array($pagenav, $limit);
        } else {

            $where['limit'] =   $limit;
        }

        $rows   =   $addressM->getAddressList($where);

        $list   =   count($rows) ? $rows : array();

        $data   =   array();

        $data['list']       =   $list;

        if (!empty($this->comInfo['name']) && !empty($this->comInfo['address'])){

            $cityStr            =   $this->comInfo['job_city_one'].$this->comInfo['job_city_two'].$this->comInfo['job_city_three'];
            $data['comLink']    =   array(

                'link_man'      =>  $this->comInfo['linkman'],
                'phone_n'       =>  $this->comInfo['linktel'],
                'tel_n'         =>  $this->comInfo['linkphone'],
                'cityStr'       =>  $cityStr,
                'link_address'  =>  $this->comInfo['address'],
                'x'             =>  $this->comInfo['x'],
                'y'             =>  $this->comInfo['y']
            );

            if ($this->comInfo['linktel']) {

                $data['comLink']['linkmsg'] =   $this->comInfo['linkman'] . ' - ' . $this->comInfo['linktel'] . ' - ' . $cityStr . ' - ' . $this->comInfo['address'];
            } else if ($this->comInfo['linkphone']) {

                $data['comLink']['linkmsg'] =   $this->comInfo['linkman'] . ' - ' . $this->comInfo['linkphone'] . ' - ' . $cityStr . ' - ' . $this->comInfo['address'];
            }
        }

        $this->render_json(0, '', $data, $total);
    }

    function getAddressInfo_action()
    {

        $addressM           =   $this->MODEL('address');

        $where['uid']       =   $this->member['uid'];

        if ($_POST['id']) {

            $where['id']    =   $_POST['id'];

            $Info           =   $addressM->getAddressInfo($where);
        }else{

            $Info['provinceid']     =   $this->comInfo['provinceid'];
            $Info['cityid']         =   $this->comInfo['cityid'];
            $Info['three_cityid']   =   $this->comInfo['three_cityid'];

            $Info['city_one']       =   $this->comInfo['job_city_one'];
            $Info['city_two']       =   $this->comInfo['job_city_two'];
            $Info['city_three']     =   $this->comInfo['job_city_three'];

            $Info['link_address']   =   $this->comInfo['address'];
            if ($this->comInfo['x'] && $this->comInfo['y']) {
                $Info['x']          =   $this->comInfo['x'];
                $Info['y']          =   $this->comInfo['y'];
            }
        }

        $this->render_json(0, '', $Info);
    }

    function saveAddress_action()
    {

        $addressM   =   $this->MODEL('address');

        $linkData   =   array(

            'link_man'      =>  $_POST['link_man'],
            'link_moblie'   =>  $_POST['link_moblie'],
            'link_phone'    =>  $_POST['link_phone'],
            'email'         =>  $_POST['email'],
            'link_address'  =>  $_POST['link_address'],
            'provinceid'    =>  $_POST['provinceid'],
            'cityid'        =>  $_POST['cityid'],
            'three_cityid'  =>  $_POST['three_cityid'],
            'x'             =>  $_POST['x'],
            'y'             =>  $_POST['y']
        );

        if ((int)$_POST['id'] > 0){

            $result         =   $addressM->upAddress($linkData, array('id' => $_POST['id'], 'uid' => $this->member['uid']));

            $linkData['id'] =   $_POST['id'];
            $linkData['uid']=   $this->member['uid'];
            $msg            =   $result ? '工作地址更新成功' : '工作地址更新失败';
        }else{

            $linkData['uid']=   $this->member['uid'];
            $result         =   $addressM->saveAddress($linkData, array('uid' => $this->member['uid']));

            $linkData['id'] =   $result;
            $msg            =   $result ? '工作地址添加成功' : '工作地址添加失败';
        }

        if ($result){

            $linkData['phone_n']    =   $linkData['link_moblie'];
            $linkData['tel_n']      =   $linkData['link_phone'];

            $cityData               =   $addressM->getCityStr(array('provinceid' => $linkData['provinceid'], 'cityid' => $linkData['cityid'], 'three_cityid' => $linkData['three_cityid']));
            $linkData['city']       =   $cityData['cityStr'];

            if ($linkData['link_moblie']) {

                $linkData['linkmsg']=   $linkData['link_man'] . ' - ' . $linkData['link_moblie'] . ' - ' . $linkData['city'] . ' - ' . $linkData['link_address'];
            } else if ($linkData['link_phone']) {

                $linkData['linkmsg']=   $linkData['link_man'] . ' - ' . $linkData['link_phone'] . ' - ' . $linkData['city'] . ' - ' . $linkData['link_address'];
            } else {

                $linkData['linkmsg']=   $linkData['link_man'] . ' - ' . $linkData['city'] . ' - ' . $linkData['link_address'];
            }

            $linkData['x']          =   $_POST['x'];
            $linkData['y']          =   $_POST['y'];
        }

        $error              =   $result ? 1 : 2;
        $this->render_json($error, $msg, $linkData);
    }

    function delAddress_action()
    {

        $_POST      =   $this->post_trim($_POST);

        $addressM   =   $this->MODEL('address');

        $res        =   $addressM->delAddress(array('uid' => $this->member['uid'], 'id' => $_POST['id']));

        $error      =   $res['errcode'] == 9 ? 1 : 2;
        $msg        =   $res['msg'];

        $this->render_json($error, $msg);
    }

}
?>