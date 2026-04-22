<?php

class version_controller extends version_base_controller{
    // 获取网站当前版本code
    function getinfo_action(){
//        //校验网站外呼key
//        if (trim($_GET['token']) != trim($this->config['sy_outcall_appkey'])) {
//            echo 'fail';
//        }
        $versionM = $this->MODEL('version');
        echo $versionM->getLatestCode();
    }
}
?>
