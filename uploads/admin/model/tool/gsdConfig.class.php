<?php

/**
 * Class gsdConfig_controller
 *
 * @author shiy
 * @version 7.0
 */

class gsdConfig_controller extends adminCommon
{

    function index_action()
    {

        $gsdConfig  =   array(

            'sy_ip'                 =>  $this->config['sy_ip'],
            'sy_ip_appkey'          =>  $this->config['sy_ip_appkey'],
            'sy_ip_appsecret'       =>  $this->config['sy_ip_appsecret'],

            'sy_mobile'             =>  $this->config['sy_mobile'],
            'sy_mobile_appkey'      =>  $this->config['sy_mobile_appkey'],
            'sy_mobile_appsecret'   =>  $this->config['sy_mobile_appsecret']
        );

        $this->render_json(0, '', $gsdConfig);
    }

    public function getRestNum_action()
    {

        $data   =   array('rest_num' => 0);

        $url    =   '';

        if ($_POST['type'] == 'ip'){
            if (!empty($this->config['sy_ip_appkey']) && !empty($this->config['sy_ip_appsecret'])) {

                $url    =   'https://u.phpyun.com/feature';
                $url    .=  '?appKey='.$this->config['sy_ip_appkey'].'&appSecret='.$this->config['sy_ip_appsecret'];
            }
        }else if ($_POST['type'] == 'phone'){

            if (!empty($this->config['sy_mobile_appkey']) && !empty($this->config['sy_mobile_appsecret'])) {

                $url    =   'https://u.phpyun.com/feature';
                $url    .=  '?appKey='.$this->config['sy_mobile_appkey'].'&appSecret='.$this->config['sy_mobile_appsecret'];
            }
        }

        if ($url != ''){
            if (extension_loaded('curl')) {

                $return =   CurlGet($url);
            } else if (function_exists('file_get_contents')) {

                $return =   file_get_contents($url);
            }

            $msgInfo    =   json_decode($return, true);

            if ($msgInfo['code'] == '200') {

                $data['rest_num']   =   $msgInfo['num'];
            }
        }
        $this->render_json(0, '', $data);
    }

    public function setIpAddressConfig_action()
    {

        if ($_POST['gsdConfig']) {

            $config =   array(

                'sy_ip'             =>  $_POST['sy_ip'],
                'sy_ip_appkey'      =>  $_POST['sy_ip_appkey'],
                'sy_ip_appsecret'   =>  $_POST['sy_ip_appsecret']
            );

            $configM=   $this->MODEL('config');
            $configM->setConfig($config);
            $this->web_config();

            $this->admin_json(0, 'IP归属地配置设置成功');
        }
    }

    public function setPhoneAddressConfig_action()
    {

        if($_POST['gsdConfig']){

            $config =   array(

                'sy_mobile'             =>  $_POST['sy_mobile'],
                'sy_mobile_appkey'      =>  $_POST['sy_mobile_appkey'],
                'sy_mobile_appsecret'   =>  $_POST['sy_mobile_appsecret']
            );

            $configM    =   $this->MODEL('config');
            $configM->setConfig($config);
            $this->web_config();

            $this->admin_json(0, '手机号归属地配置设置成功');
        }
    }
}

?>