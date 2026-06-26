<?php


class index_controller extends common
{

    function index_action()
    {
        
        if ($_GET['uid']) {
            
            $uid        =   intval($_GET['uid']);

            $UserinfoM  =   $this->MODEL('userinfo');
            $member     =   $UserinfoM->getInfo(array('uid' => $uid), array('field' => '`claim`'));
            
            if ($member['claim'] == '1') {
                $this->ACT_msg($this->config['sy_weburl'], yun_at('wap_00170'));
            }
            
            $ComM       =   $this->MODEL('company');
            $cert       =   $ComM->getCertInfo(array('uid' => $uid, 'type' => 6));
            if ($cert['check2'] != $_GET['code'] || $cert['check2'] == '') {
                $this->ACT_msg($this->config['sy_weburl'], yun_at('wap_00171'));
            }
            
        }
        
        $this -> seo('claim');
        $this -> yun_tpl(array('index'));
    }

    function save_action()
    {
        if ($_POST['submit']) {
            
            $UserinfoM  =   $this->MODEL('userinfo');
            $member     =   $UserinfoM->getInfo(array('uid' => intval($_POST['uid'])), array('field' => '`claim`'));
            if ($member['claim'] == '1') {
                $this->ACT_layer_msg('wap_00170', 8);
            }
            
            $ComM       =   $this -> MODEL('company');
            $cert       =   $ComM -> getCertInfo(array('uid' => intval($_POST['uid']), 'type' => 6));
            
            if ($cert['check2'] != $_POST['code'] || $cert['check2'] == '') {
                $this->ACT_layer_msg('wap_00171', 8, $_SERVER['HTTP_REFERER']);
            }
            $row        =   $UserinfoM -> getInfo(array('username' => $_POST['username']), array('field' => '`uid`'));
            
            if ($row['uid'] > 0) {
                $this->ACT_layer_msg('wap_01779', 8);
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
             
            $return     =   $UserinfoM ->upInfo(array('uid' => intval($_POST['uid'])), $mData,'');
            
            $return ? $this->ACT_layer_msg('wap_01780', 9, Url('login')) : $this->ACT_layer_msg('wap_01781', 8, '');
        }
    }
}