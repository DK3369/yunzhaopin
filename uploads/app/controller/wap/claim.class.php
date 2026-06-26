<?php


class claim_controller extends common
{

    function index_action()
    {
        
		
        if ($_GET['uid']) {
            
            $uid        =   intval($_GET['uid']);

            $UserinfoM  =   $this->MODEL('userinfo');
            $member     =   $UserinfoM->getInfo(array('uid' => $uid), array('field' => '`claim`'));
            
            if ($member['claim'] == '1') {
                $this->ACT_msg_wap($_SERVER['HTTP_REFERER'], yun_at('wap_00170'));
            }
            
			$ComM       =   $this->MODEL('company');
            $cert       =   $ComM->getCertInfo(array('uid' => $uid, 'type' => 6));
            if ($cert['check2'] != $_GET['code'] || $cert['check2'] == '') {
                $this->ACT_msg_wap($_SERVER['HTTP_REFERER'], yun_at('wap_00171'));
            } 
        }
	
		$this->yunset("headertitle",yun_at('wap_00174'));
		
        $this -> seo('claim');
        $this -> yuntpl(array('wap/claim'));
    }

    function save_action()
    {
        if ($_POST) {
            
            $UserinfoM  =   $this->MODEL('userinfo');
            $member     =   $UserinfoM->getInfo(array('uid' => intval($_POST['uid'])), array('field' => '`claim`'));
            if ($member['claim'] == '1') {
                $this->ACT_msg_wap($_SERVER['HTTP_REFERER'], yun_at('wap_00170'));
            }
            
            $ComM       =   $this -> MODEL('company');
            $cert       =   $ComM -> getCertInfo(array('uid' => intval($_POST['uid']), 'type' => 6));
            
            if ($cert['check2'] != $_POST['code'] || $cert['check2'] == '') {
                $this->ACT_msg_wap($_SERVER['HTTP_REFERER'], yun_at('wap_00171'));
            }
            $row        =   $UserinfoM -> getInfo(array('username' => $_POST['username']), array('field' => '`uid`'));
            
            if ($row['uid'] > 0) {
                $this->ACT_msg_wap($_SERVER['HTTP_REFERER'], yun_at('wap_01779'));
            }
            $salt       =   substr(uniqid(rand()), - 6);
            $pass       =   passCheck($_POST['password'], $salt);
            
            $mData      =   array(
                'username'  =>  $_POST['username'],
                'salt'      =>  $salt,
                'password'  =>  $pass,
                'claim'     =>  1,
                'source'    =>  1
            );
             
            $result	=   $UserinfoM ->upInfo(array('uid' => intval($_POST['uid'])), $mData,'');
			
			$return	=	array(
				
				'errcode'	=>	$result ? 9 : 8,
				'msg'		=>	$result	? yun_at('wap_01780') : yun_at('wap_01781'),
			);
			
			echo yun_json_encode($return);die;
        }
    }
}