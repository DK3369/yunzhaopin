<?php



class customize_controller extends company
{

    function index_action()
    {

        $this->public_action();

        if ($this->comInfo['is_nav'] == 2) {

            $navList = $this->leftNav;
        } else {

            $navList = array(
                0 => array('name' => 'wap_com_00106', 'url' => 'job', 'sort' => 1, 'target' => 1, 'show' => 1),
                1 => array('name' => 'wap_com_00105', 'url' => 'hr', 'sort' => 2, 'target' => 1, 'show' => 1),
                2 => array('name' => 'member_com_00213', 'url' => 'invite', 'sort' => 3, 'target' => 1, 'show' => 1),
                3 => array('name' => 'member_com_00597', 'url' => 'resume', 'sort' => 4, 'target' => 1, 'show' => 1),
                4 => array('name' => 'member_com_00293', 'url' => 'zhaopinhui', 'sort' => 5, 'target' => 1, 'show' => 1),
                5 => array('name' => 'wap_com_00097', 'url' => 'right', 'sort' => 6, 'target' => 1, 'show' => 1),
                6 => array('name' => 'wap_com_00096', 'url' => 'info', 'sort' => 7, 'target' => 1, 'show' => 1),
                7 => array('name' => 'member_user_00059', 'url' => 'binding', 'sort' => 8, 'target' => 1, 'show' => 1)
            );
        }

        $this->yunset('navList', $navList);

        $this->com_tpl('customize_nav');
    }

    public $navNameArr = array(
        'job' => array('name' => 'wap_com_00106', 'icon' => 2),
        'hr' => array('name' => 'wap_com_00105', 'icon' => 4),
        'invite' => array('name' => 'member_com_00213', 'icon' => 10),
        'resume' => array('name' => 'member_com_00597', 'icon' => 3),
        'zhaopinhui' => array('name' => 'member_com_00293', 'icon' => 12),
        'right' => array('name' => 'wap_com_00097', 'icon' => 7),
        'info' => array('name' => 'wap_com_00096', 'icon' => 8),
        'binding' => array('name' => 'member_user_00059', 'icon' => 11)
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