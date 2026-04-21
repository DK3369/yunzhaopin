<?php

class messageset_controller extends adminCommon{
    /**
     * 工具 移动端 app配置
     */
    function index_action(){
        
        $msgnum = $businessnum = $khnum = 0;

        if (!empty($this->config['sy_msg_appkey']) && !empty($this->config['sy_msg_appsecret'])) {

            //短信检测
            $url = 'https://u.phpyun.com/feature';
            $url .= '?appKey=' . $this->config['sy_msg_appkey'] . '&appSecret=' . $this->config['sy_msg_appsecret'];

            if (extension_loaded('curl')) {

                $return = CurlGet($url);
            } else if (function_exists('file_get_contents')) {

                $return = file_get_contents($url);
            }

            if ($return) {
                $msgInfo = json_decode($return, true);
                if ($msgInfo['code'] == '200') {
                    $msgnum = $msgInfo['num'];
                }
                unset($return);
            }
        }

        if (!empty($this->config['sy_kh_appkey']) && !empty($this->config['sy_kh_appsecret'])) {
            //空号检测
            $url = 'https://u.phpyun.com/feature';
            $url .= '?appKey=' . $this->config['sy_kh_appkey'] . '&appSecret=' . $this->config['sy_kh_appsecret'];
            if (extension_loaded('curl')) {

                $return = CurlGet($url);
            } else if (function_exists('file_get_contents')) {

                $return = file_get_contents($url);
            }
            if ($return) {
                $msgInfo = json_decode($return, true);
                if ($msgInfo['code'] == '200') {
                    $khnum = $msgInfo['num'];
                }
                unset($return);
            }
        }

        if (!empty($this->config['sy_tyc_appkey']) && !empty($this->config['sy_tyc_appsecret'])) {
            //天眼查检测
            $url = 'https://u.phpyun.com/feature';
            $url .= '?appKey=' . $this->config['sy_tyc_appkey'] . '&appSecret=' . $this->config['sy_tyc_appsecret'];

            if (extension_loaded('curl')) {

                $return = CurlGet($url);
            } else if (function_exists('file_get_contents')) {

                $return = file_get_contents($url);
            }
            if ($return) {
                $msgInfo = json_decode($return, true);
                if ($msgInfo['code'] == '200') {
                    $businessnum = $msgInfo['num'];
                }
            }
        }

        $data = array(
            'sy_msg_isopen'     =>  $this->config['sy_msg_isopen'],
            'sy_msg_appkey'     =>  $this->config['sy_msg_appkey'],
            'sy_msg_appsecret'  =>  $this->config['sy_msg_appsecret'],
            'ip_msgnum'         =>  $this->config['ip_msgnum'],
            'moblie_msgnum'     =>  $this->config['moblie_msgnum'],
            'cert_msgtime'      =>  $this->config['cert_msgtime'],
            'moblie_codetime'   =>  $this->config['moblie_codetime'],
            'sy_kh_isopen'      =>  $this->config['sy_kh_isopen'],
            'sy_kh_appkey'      =>  $this->config['sy_kh_appkey'],
            'sy_kh_appsecret'   =>  $this->config['sy_kh_appsecret'],
            'sy_kh_city'        =>  $this->config['sy_kh_city'],
            'sy_tyc_appkey'     =>  $this->config['sy_tyc_appkey'],
            'sy_tyc_appsecret'  =>  $this->config['sy_tyc_appsecret'],
            
            'rest_msgnum'            =>  $msgnum,
            'rest_businessnum'       =>  $businessnum,
            'rest_khnum'             =>  $khnum,
        );
        
        $this->render_json(0,'ok', $data);
    }

    function save_action(){
        
        $_POST =  $this->post_trim($_POST);
        
        $configM  =  $this->MODEL('config');
        
        $configM -> setConfig($_POST);
        
        $this->web_config();
        
        $this->render_json(0,'短信配置设置成功！');
    }

    function tplswitch_action(){
        
        include(CONFIG_PATH."db.tpl.php");

        $data = array(
            'public'=>array('name'=>'公共设置','configarr'=>array()),
            'user'=>array('name'=>'个人短信设置','configarr'=>array()),
            'com'=>array('name'=>'企业短信设置','configarr'=>array()),
        );
        
        foreach ($arr_tpl as $k => $v) {
            if($v['type']=='msg'){
                $data[$v['cate']]['configarr'][] = array('name'=>$v['name'],'tpl'=>$k,'config_name'=>$v['config'],'config_val'=>$this->config[$v['config']]);
            }
        }

        $this->render_json(0,'', $data);
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
            
            $templatesM->upInfo(array('name'=>trim($_POST['name'])),array('content'=>$content));
            
        }else{
            
            $templatesM->addInfo(array('name'=>trim($_POST['name']),'content'=>$content));
        
        }

        $this->render_json(0,'短信模板配置成功');
    }
    
}
?>