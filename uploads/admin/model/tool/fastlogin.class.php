<?php

class fastlogin_controller extends adminCommon{
    /**
     * 工具 移动端 app配置
     */
    function index_action(){
        
        $data = array(
            'sy_qqlogin'    => $this->config['sy_qqlogin'],
            'sy_qqappid'    => $this->config['sy_qqappid'],
            'sy_qqappkey'   => $this->config['sy_qqappkey'],
            'sy_qqdt'       => $this->config['sy_qqdt'],

            'sy_sinalogin'  => $this->config['sy_sinalogin'],
            'sy_sinaappid'  => $this->config['sy_sinaappid'],
            'sy_sinaappkey' => $this->config['sy_sinaappkey'],
        );
        
        $this->render_json(0,'ok', $data);
    }

    function save_action(){
        
        $_POST =  $this->post_trim($_POST);
        
        $configM  =  $this->MODEL('config');
        
        $configM -> setConfig($_POST);
        
        $this->web_config();
        
        $this->render_json(0,'快捷登录参数配置修改成功！');
    }

}
?>