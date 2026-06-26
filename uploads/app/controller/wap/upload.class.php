<?php

class upload_controller extends common{
	
	private $tokenSalt = 'phpyun';// token salt

	// （）
	private function generateToken($type, $uid){
		// ，tokenSaltpassword，token
		$userinfoM	=	$this->MODEL('userinfo');
		$row 		= 	$userinfoM->getInfo(array('uid'=> $uid),array('field'=>'`password`'));
		$password 	= isset($row['password']) ? $row['password'] : '';
		$password 	= substr($password, 0, 8);
		$time       = time();
		
		$this->tokenSalt	= $this->config['sy_safekey'];
		return yunEncrypt("{$type}|{$uid}|{$password}|{$time}", $this->tokenSalt);
	}

	// （）
	private function checkToken($token){
		$token		= 	urldecode($token);
		
		$this->tokenSalt = $this->config['sy_safekey'];
		$str 		= 	yunDecrypt($token, $this->tokenSalt);
		$arr 		= 	explode('|', $str);
		
		if(count($arr) != 4 || $arr[1] == ''){
			return false;
		}
		// token
		if(intval($arr[3] + 86400) < time()){
		    return false;
		}
		// uidpassword，password
		$uid = $arr[1];
		$userinfoM	=	$this->MODEL('userinfo');
		$row 		= 	$userinfoM->getInfo(array('uid'=> $uid),array('field'=>'`password`'));
		
		$password 	= 	isset($row['password']) ? $row['password'] : '';
		$password 	= 	substr($password, 0, 8);
		if($password != $arr[2]){
			return false;
		}
		return array('uid' => $uid, 'type' => $arr[0]);
	}

	// （）
	public function qrcode_action(){
		if(!$this->uid){
			exit(yun_auto_t('请先登录登录'));
		}
		// type , save_action
		$type 	= isset($_GET['type']) ? $_GET['type'] : '';
		if($type == ''){
			exit(yun_auto_t('扫码上传图片可选类型type：1企业营业执照上传，2个人身份证上传，3个人头像，4企业logo'));
		}
		$token 	= $this->generateToken($type, $this->uid);
		$token 	= urlencode($token);
		$url 	= Url('wap',array('c'=> 'upload', 'a' => 'p', 't' => $token) );

		include_once LIB_PATH."yunqrcode.class.php";
		YunQrcode::generatePng2($url, 4);
	}

	// wap
	public function p_action(){

		$userinfoM	=	$this->MODEL('userinfo');

		$token 	= isset($_GET['t']) ? $_GET['t'] : '';
		$arr 	= $this->checkToken($token);
		if($arr == false || !isset($arr['type']) || !isset($arr['uid']) ){
			exit(yun_auto_t('抱歉，功能维护中'));
		}
		$this->yunset('token', $token);
		$this->yunset('type', $arr['type']);
		
		if($arr['type'] == 3 || $arr['type'] == 4 || $arr['type'] == 5 || $arr['type'] == 6){// avatar upload
			$pic	=	$icon	=	'';
		    if ($arr['type']==3){
				
				$photo 		= 	$userinfoM->getUserInfo(array('uid'=>$arr['uid']),array('usertype'=>1,'field'=>'`photo`,`sex`'));

		        if(!$photo['photo']){
		            if ($photo['sex']==1){
		                $icon	=	$this->config['sy_member_icon'];
		            }else{
		                $icon	=	$this->config['sy_member_iconv'];
		            }
		        }else{
		            $pic		=	$photo['photo'];
		        }
		    }elseif ($arr['type']==4){

				$photo 		= 	$userinfoM->getUserInfo(array('uid'=>$arr['uid']),array('usertype'=>2,'field'=>'logo'));

		        if(!$photo['logo']){
		            $icon	=	$this->config['sy_unit_icon'];
		        }else{
		            $pic	=	$photo['logo'];
		        }
		    }
		    $photo['photo']	=	checkpic($pic,$icon);
			$this->yunset('photo',$photo['photo']);

			$this->seo("wap_upload");
			$this->yuntpl(array('wap/uploadimg_userlogo'));
		}else{
			$this->yuntpl(array('wap/uploadimg'));
		}
	}

	// 
	public function uploadimg_save_action(){
	    $token = isset($_POST['token']) ? $_POST['token'] : '';
		if($token == ''){
			echo yun_json_encode(array('status' => -1, 'msg' => yun_auto_t('二维码传图出错，请联系网站管理员')));
			exit;
		}
		$arr = $this->checkToken($token);
		if($arr == false || !isset($arr['type']) || !isset($arr['uid']) ){
			echo yun_json_encode(array('status' => -1, 'msg' => yun_auto_t('操作超时，请刷新pc端网页二维码重试') . $token));
			exit;
		}

		$path = $this->uploadimg_save_path($arr['type'], $arr['uid']);
		
		echo yun_json_encode($path);exit;

		if($path != ''){
			echo yun_json_encode(array('status' => 1, 'path' => $path));
			exit;
		}else{
			echo yun_json_encode(array('status' => -1, 'msg' => yun_auto_t('上传失败，请重试')));
			exit;
		}
	}

	// ，，
	private function uploadimg_save_path($type, $uid){
		
		$companyM 	= 	$this->MODEL('company');
		$resumeM 	= 	$this->MODEL('resume');
		$UserinfoM	=	$this->MODEL('userinfo');

		$uid 		= 	addslashes($uid);

		switch($type){
			case 1:// business license		
				// $pic 		= 	$this->upload();
				// $path 		= 	$pic;
				
				$cert 		=   $companyM -> getCertInfo(array('uid' => $uid, 'type' => '3'));

				$postData   =   array(
					'status'	=> 	$this -> config['com_cert_status'] == '1' ? 0 : 1,
					'ctime'		=> 	time()
				);
				if (!empty($_POST['social_credit'])) {
				    $postData['social_credit']   =  $_POST['social_credit'];
				}
				if (!empty($_POST['preview'])) {
				    $postData['check']   =  $_POST['preview'];
				}
				if (!empty($_POST['owner_cert'])) {
				    $postData['owner_cert']   =  $_POST['owner_cert'];
				}
				if (!empty($_POST['wt_cert'])) {
				    $postData['wt_cert']   =  $_POST['wt_cert'];
				}
				if (!empty($_POST['other_cert'])) {
				    $postData['other_cert']   =  $_POST['other_cert'];
				}
				
				if (!empty($cert) && is_array($cert) && $cert['ctime']) {
			        
			        $return   =   $companyM -> upCertInfo(array('id'=>intval($cert['id']), 'uid' => $uid), $postData, array('yyzz' => '1', 'usertype' => 2, 'com_name'=>trim($_POST['com_name'])));
			    }else{
					$postData['uid']		=	$uid;
					$postData['type']		=	'3';
					$postData['step']		=	'1';
					$postData['did']		=	$this ->config['did'];
					$postData['usertype']	=	2;
					$postData['com_name']	=	trim($_POST['com_name']);
					
			        $return	=	$companyM -> addCertInfo($postData);
				}
				return $return;

				break;
			case 2:// id card
			
				$pic 	= $this->upload();
				$path 	= $pic;

				$data	=	array(
				    'usertype'		=>	1,
				    'name'			=>	$_POST['name'],
					'idcard'		=>	$_POST['idcard'],
					'idcard_pic'	=>	$path,
				);
				$return	=	$UserinfoM -> upidcardInfo(array('uid'=>$uid,'wap'=>'1'),$data);
				if($return['errcode']==9){
					$_COOKIE['uid'] 		= 	$uid;
    		    	$_COOKIE['usertype']	= 	1;
				}
				return $return;
			break;
			case 3:// user avatar
				$return   =  $resumeM -> upPhoto(array('uid'=>$uid),array('utype'=>'user','base'=>$_POST['uimage']));
				return $return;
			break;
			case 4://企业上传logo

				$return   =  $companyM -> upLogo(array('uid'=>$uid),array('utype'=>'user','base'=>$_POST['uimage']));
				return $return;
		    break;
		}
		return '';
	}

	private function upload($path=''){
		
		if($_POST['preview']){
			$upArr   =  array(
                'dir'      =>  'cert',
                'base'     =>  $_POST['preview'],
            );
            
            $result  =  $this -> newupload($upArr);
            
            if (!empty($result['msg'])){
                
                $return['msg']      =  $result['msg'];
                
                echo yun_json_encode(array('msg' => $result['msg']));exit;
                
            }elseif (!empty($result['picurl'])){
                
                return   $result['picurl'];
            }
            
		}else{
			echo yun_json_encode(array('status' => -1, 'msg' => yun_auto_t('请上传图片')));exit;
		}
	}
	/**
      * @desc Upload helper: file, dir, type, base, preview
     */
    private function newupload($data = array('file'=>null,'dir'=>null,'type'=>null,'base'=>null,'preview'=>null)){
          
        $UploadM =	$this->MODEL('upload');
          
        $upArr   =  array(
            'file'     =>  $data['file'],
            'dir'      =>  $data['dir'],
            'type'     =>  $data['type'],
            'base'     =>  $data['base'],
            'preview'  =>  $data['preview']
        );
        $return  =  $UploadM -> newUpload($upArr);
        return $return;
    }
	/**
	 * Upload refactor notes (internal):
	 * 1. Audit Upload_pic usages and saved file paths
	 * 2. Move shared validation into upload model
	 * 4. Mobile camera/album upload page
	 * 5. QR scan upload API
	 * 6. PC upload areas with QR helper
	 * 7. TODO: refresh PC page after mobile upload completes
	 */
	function upCertPic_action(){
		$UploadM		=	$this	->	MODEL('upload');

		$picurl			=	'';
		$msg			=	'';
		$error			=	'';

		if(isset($_POST['preview'])){
				    // pc
		    $upArr    	=  array(
		        'base'  =>  $_POST['preview'],
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
		    $msg 	= 	yun_auto_t('请选择图片');
		}

		$return['error'] 	= $error;
		$return['msg'] 		= $msg;
		$return['picurl'] 	= $picurl;

		echo yun_json_encode($return);die;
	}
}
?>