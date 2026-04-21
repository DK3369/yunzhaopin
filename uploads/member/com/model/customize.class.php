<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class customize_controller extends company
{

    function index_action()
    {

        $this->public_action();

        if ($this->comInfo['is_nav'] == 2) {

            $navList = $this->leftNav;
        } else {

            $navList = array(
                0 => array('name' => '职位管理', 'url' => 'job', 'sort' => 1, 'target' => 1, 'show' => 1),
                1 => array('name' => '简历管理', 'url' => 'hr', 'sort' => 2, 'target' => 1, 'show' => 1),
                2 => array('name' => '面试管理', 'url' => 'invite', 'sort' => 3, 'target' => 1, 'show' => 1),
                3 => array('name' => '人才库', 'url' => 'resume', 'sort' => 4, 'target' => 1, 'show' => 1),
                4 => array('name' => '招聘会', 'url' => 'zhaopinhui', 'sort' => 5, 'target' => 1, 'show' => 1),
                5 => array('name' => '会员服务', 'url' => 'right', 'sort' => 6, 'target' => 1, 'show' => 1),
                6 => array('name' => '企业资料', 'url' => 'info', 'sort' => 7, 'target' => 1, 'show' => 1),
                7 => array('name' => '账号设置', 'url' => 'binding', 'sort' => 8, 'target' => 1, 'show' => 1)
            );
        }

        $this->yunset('navList', $navList);

        $this->com_tpl('customize_nav');
    }

    public $navNameArr = array(
        'job' => array('name' => '职位管理', 'icon' => 2),
        'hr' => array('name' => '简历管理', 'icon' => 4),
        'invite' => array('name' => '面试管理', 'icon' => 10),
        'resume' => array('name' => '人才库', 'icon' => 3),
        'zhaopinhui' => array('name' => '招聘会', 'icon' => 12),
        'right' => array('name' => '会员服务', 'icon' => 7),
        'info' => array('name' => '企业资料', 'icon' => 8),
        'binding' => array('name' => '账号设置', 'icon' => 11)
    );

    function saveCustomize_action()
    {

        $value  =   array();

        $arr    =   [0, 1, 2, 3, 4, 5, 6, 7];

        foreach ($arr as $v) {
            $value[$_POST['url_'.$v]]   =   array(

                'name'  =>  $this->navNameArr[$_POST['url_'.$v]]['name'],
                'url'   =>  $_POST['url_'.$v],
                'sort'  =>  $_POST['sort_'.$v],
                'target'=>  $_POST['target_'.$v] ? 1 : 2,
                'show'  =>  $_POST['show_'.$v] ? 1 : 2,
                'icon'  =>  $this->navNameArr[$_POST['url_'.$v]]['icon']
            );
        }

        $comM   =   $this->MODEL('company');

        $valueData  =   array(
            'uid'       =>  $this->uid,
            'nav_info'  =>  serialize($value)
        );

        $return = $comM->saveLeftNav($valueData, array('nav' => $this->comInfo['is_nav'], 'uid' => $this->uid));

        $this->ACT_layer_msg($return['errmsg'], $return['errcode'], $return['url']);
    }
}

?>