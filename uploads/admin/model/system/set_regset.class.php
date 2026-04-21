<?php

/*
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */
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