<?php

/**
 * Class dataOss_controller
 *
 * @author shiy
 * @version 7.0
 */

class dataOss_controller extends adminCommon
{

    function index_action()
    {


        $ossConfig = array();
        require_once APP_PATH."data/api/aliyun_oss/oss_data.php";
        if (isset($oss_data)) {

            $ossConfig = $oss_data;

            $ossConfig['sy_oss']    =   $this->config['sy_oss'];
            if (empty($ossConfig['userdomain'])){

                $ossConfig['userdomain']    =   $this->config['sy_ossurl'];
            }
        }
        $this->render_json(0, '', $ossConfig);
    }

    public function setOssConfig_action()
    {

        if (isset($_POST['ossConfig']) && intval($_POST['ossConfig']) == 1) {


            $sy_oss     =   intval($_POST['sy_oss']);
            $sy_ossurl  =   $_POST['sy_oss'] == 1 ? $_POST['userdomain'] : '';

            $config     =   array(

                'sy_oss'    =>  $sy_oss,
                'sy_ossurl' =>  $sy_ossurl
            );
            $configM    =  $this->MODEL('config');

            $configM->setConfig($config);
            $this->web_config();

            $oss_data   =   array(

                'access_id'   =>  $_POST['access_id'],
                'access_key'  =>  $_POST['access_key'],
                'endpoint'    =>  $_POST['endpoint'],
                'bucket'      =>  $_POST['bucket'],
                'userdomain'  =>  $_POST['userdomain']
            );
            made_web(APP_PATH.'data/api/aliyun_oss/oss_data.php',ArrayToString($oss_data),'oss_data');

            $this->admin_json(0, 'OSS配置成功');
        }
    }
}

?>