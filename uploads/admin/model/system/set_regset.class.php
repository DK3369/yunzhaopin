<?php


class set_regset_controller extends adminCommon
{

    function index_action()
    {
        $cache = $this->MODEL('cache')->GetCache('regconfig');
        $data = array(
            'config'=>$this->config,
            'regconfig'=>$cache['regConfig']
        );
        $this->render_json(0,'ok',$data);
    }

    // 保存
    function save_action()
    {
        if ($_POST['config']) {
            
            unset($_POST['config']);
            $configM = $this->MODEL('config');

            // 注册配置特殊处理
            $regData = array(
                'regname' => jsJsonDecode($_POST['regname']),
                'mobile_number' => jsJsonDecode($_POST['mobile_number']),
                'mobile_white' => jsJsonDecode($_POST['mobile_white']),
                'mobile_black' => jsJsonDecode($_POST['mobile_black'])
            );

            unset($_POST['regname']);
            unset($_POST['mobile_number']);
            unset($_POST['mobile_white']);
            unset($_POST['mobile_black']);

            $configM->setConfig($_POST);
            
            $this->web_config();

            $configM->setRegConfig($regData);

            include(LIB_PATH."cache.class.php");
            $cacheclass	= new cache(PLUS_PATH,$this->obj);
            $cacheclass->regconfig_cache("reg.cache.php"); // 生成注册配置缓存
            
            $this->admin_json(0,'注册设置成功');
        }
    }
}
?>