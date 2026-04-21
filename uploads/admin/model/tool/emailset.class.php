<?php

class emailset_controller extends adminCommon{
    /**
     * 工具 移动端 app配置
     */
    function index_action(){
        
        $emailM         =   $this->MODEL('email');

        $where['id']    =   array('>','0');
        
        $List           =   $emailM->getList($where);
        
        $SMTPlist       =   !empty($List['list'])?$List['list']:array();

        $data = array(
            'SMTPlist'          =>  $SMTPlist,
            'sy_email_online'   =>  $this->config['sy_email_online']==1?1:2,
            'accesskey'         =>  $this->config['accesskey'],
            'accesssecret'      =>  $this->config['accesssecret'],
            'ali_email'         =>  $this->config['ali_email'],
            'ali_tag'           =>  $this->config['ali_tag'],
            'ali_name'          =>  $this->config['ali_name'],
        );
        
        $this->render_json(0,'ok', $data);
    }

    function save_action(){
        
        $emailM     =   $this->MODEL('email');
        
        $configM    =   $this->MODEL('config');
        
        if($_POST['sy_email_online']==1){
        
            $configM->setConfig(
                array(
                    'sy_email_online'   =>  $_POST['sy_email_online'],
                    'sy_email_set'      =>  '1'
                )
            );

            $_POST['content']  =  stripslashes($_POST['content']);
            $smtp_post = json_decode($_POST['content'],true);

            foreach ($smtp_post as $k => $v) {
                $datav = array();
                $datav['smtpserver'] = $v['smtpserver'];
                $datav['smtpuser'] = $v['smtpuser'];
                $datav['smtppass'] = $v['smtppass'];
                $datav['smtpport'] = $v['smtpport'];
                $datav['smtpnick'] = $v['smtpnick'];
                $datav['default'] = $v['default'];

                if($datav['smtpserver'] && $datav['smtpuser'] && $datav['smtppass']){
                    if($v['isadd']==1){
                        $emailM->addInfo($datav);
                    }else if($v['id']){
                        $emailM->upInfo(array('id'=>$v['id']),$datav);
                    }
                }
            }
        }else if($_POST['sy_email_online']==2){
            
            $aliData['sy_email_online'] =   $_POST['sy_email_online'];

            $aliData['sy_email_set']    =   1;

            $aliData['accesskey']       =   $_POST['accesskey'];
            
            $aliData['accesssecret']    =   $_POST['accesssecret'];
            
            $aliData['ali_email']       =   $_POST['ali_email'];
            
            $aliData['ali_tag']         =   $_POST['ali_tag'];
            
            $aliData['ali_name']        =   $_POST['ali_name'];
            
            $configM->setConfig($aliData);
            
        }

        $this->get_cache();
        $this->web_config();
        
        $this->render_json(0,'邮件服务器设置成功！');
    }
    function get_cache(){
        
        include(LIB_PATH."cache.class.php");
        
        $cacheclass =   new cache(PLUS_PATH,$this->obj);
        
        $makecache  =   $cacheclass->emailconfig_cache("emailconfig.cache.php");
    }
    function ceshi_action(){
        
        $notice = $this->MODEL('notice');
        
        if($_POST["ceshi_email"]){
            
            //发送邮件并记录入库
            $emailData['smtpServerId']  =   $_POST["id"];
            $emailData['email']         =   $_POST["ceshi_email"];
            
            $emailData['subject']       =   $this->config['sy_webname']." - 测试邮件";
            
            $emailData['content']       =   "恭喜你，该邮件帐户可以正常使用<br> ".$this->config['sy_webname']."- Powered by PHPYun.";
            
            $sendid = $notice->sendEmail($emailData);
            
            if($sendid['status'] != -1){
                
                $error = 0;
                $msg = '测试发送成功！';
            
            }else{
                $error = 1;
                $msg = '测试发送失败！' . $sendid['msg'];
            }
            
            $this->render_json($error,$msg);
         
        }
    }

    function delconfig_action(){
        
        $emailM =   $this->MODEL('email');
        
        if($_POST["id"]){
            
            $emailConfig    =   $emailM->getInfo(array('id'=>(int)$_POST["id"]));
            
            //查询邮件服务器数量
            $num            =   $emailM->getNum(array('default'=>'1'));
            
            if($emailConfig['default']=='1' && $num<2){
                
                $msg        =   '请至少保留一组可用邮箱！';
                
                $error   =   1;
                
            }else{
                
                $emailM->delEmail(array('id'=>(int)$_POST["id"]),array('type'=>'one'));
                
                $msg    =   '邮箱(ID:'.$_POST["id"].')删除成功！';
                
                $error   =   0;
                
                $this->get_cache();
            }
            
            $this->render_json($error,$msg);
        }
    }

    function tplswitch_action(){
        
        include(CONFIG_PATH."db.tpl.php");

        $data = array(
            'public'=>array('name'=>'公共设置','configarr'=>array()),
            'user'=>array('name'=>'个人邮件设置','configarr'=>array()),
            'com'=>array('name'=>'企业邮件设置','configarr'=>array()),
        );
        
        foreach ($arr_tpl as $k => $v) {
            if($v['type']=='email'){
                $data[$v['cate']]['configarr'][] = array('name'=>$v['name'],'tpl'=>$k,'config_name'=>$v['config'],'config_val'=>$this->config[$v['config']]);
            }
        }

        $this->render_json(0,'', $data);
    }
    function savetplconfig_action(){
        
        $_POST =  $this->post_trim($_POST);
        
        $configM  =  $this->MODEL('config');
        
        $configM -> setConfig($_POST);
        
        $this->web_config();
        
        $this->render_json(0,'短信配置设置成功！');
    }
    function gettpl_action(){

        include(CONFIG_PATH."db.tpl.php");

        $templatesM =   $this->MODEL("templates");

        $row    =   $templatesM->getInfo(array('name'=>$_POST['name']));

        $tpl_temp = !empty($arr_tpl[$_POST['name']])?$arr_tpl[$_POST['name']]:array();

        $tpl_n = $tpl_temp['name'];

        unset($tpl_temp['name']);
        unset($tpl_temp['type']);
        unset($tpl_temp['config']);
        unset($tpl_temp['cate']);

        $data = array(
            'info'=>$row,
            'tpl_temp'=>$tpl_temp,
            'tpl_n'=>$tpl_n
        );

        $this->render_json(0,'', $data);
    }
    function savetpl_action(){

        $templatesM =   $this->MODEL("templates");

        $configNum  =   $templatesM->getNum(array('name'=>trim($_POST['name'])));
                
        $content    =   str_replace("amp;nbsp;","nbsp;",$_POST['content']);
        
        if($configNum>0){
                
            $templatesM->upInfo(array('name'=>trim($_POST['name'])),array('content'=>$content,'title'=>trim($_POST['title'])));
            
        }else{
            
            $templatesM->addInfo(array('name'=>trim($_POST['name']),'content'=>$content,'title'=>trim($_POST['title'])));
        
        }

        $this->render_json(0,'邮件模板配置成功');
    }
    
}
?>