<?php


class vs_controller extends company
{

    function index_action()
    {
        $this->public_action();

        $this->company_satic();
 
        $this->com_tpl('vs');
    }

    function save_action()
    {
        $UserinfoM  =   $this->MODEL('userinfo');

        if ($_POST['submit']) {

            $data   =   array(
                
                'uid'           =>  $this->uid,
                'usertype'      =>  $this->usertype,
                'oldpassword'   =>  $_POST['oldpassword'],
                'password'      =>  $_POST['password'],
                'repassword'    =>  $_POST['repassword']
                
            );
            
            $info   =   $UserinfoM -> getInfo(array('uid'=> $this->uid), array('field'=>'username'));
      
            if (is_array($info)) {

                if ($this->config['sy_uc_type'] == "uc_center" && $info['name_repeat'] != "1") {

                    $this->obj->uc_open();

                    $ucresult = uc_user_edit($info['username'], $_POST['oldpassword'], $_POST['password'], "", "1");
                
                    if ($ucresult == - 1) {

                        $this->ACT_layer_msg("原始密码错误！", 8, "index.php?c=vs");
                        
                    }elseif ($ucresult == 1){
                        $err    =   $UserinfoM -> savePassword($data);
                    }elseif ($ucresult == 0||$ucresult == -7){
                        $err    =   array('msg'=>'没有做任何修改！','errcode'=>8);
                    }elseif ($ucresult == -4){
                        $err    =   array('msg'=>'Email 格式有误！','errcode'=>8);
                    }elseif ($ucresult == -5){
                        $err    =   array('msg'=>'Email 不允许注册！','errcode'=>8);
                    }elseif ($ucresult == -6){
                        $err    =   array('msg'=>'该 Email 已经被注册！','errcode'=>8);
                    }elseif ($ucresult == -8){
                        $err    =   array('msg'=>'该用户受保护无权限更改！','errcode'=>8);
                    }
                    
                }

                $err    =   $UserinfoM -> savePassword($data);
                

                if($err['errcode'] == '8'){
                     
                    $this -> ACT_layer_msg($err['msg'], $err['errcode'], "index.php?c=vs");
                    
                }else{
                    $this -> cookie -> unset_cookie();
                    $this -> ACT_layer_msg($err['msg'], $err['errcode'], Url('login'));
                    
                }
            }
        }
    }
}
?>