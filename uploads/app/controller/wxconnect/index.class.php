<?php



class index_controller extends common
{

    function index_action()
    {
        if ($this->uid != "" && $this->username != "") {

            $this->ACT_msg($this->config['sy_weburl'] . "/member", "您已经登录了！");
        }
        $this->seo("weixin");
        $this->yun_tpl(array('index'));
    }
}

