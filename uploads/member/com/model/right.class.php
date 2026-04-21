<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2023 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */
class right_controller extends company
{

    function index_action()
    {

        $this->company_satic();
        $this->public_action();

        $sy_only_price  =   @explode(',', $this->config['sy_only_price']);
        if (in_array('vip', $sy_only_price)) {
            $this->yunset('meal', 1);
        }

        $ratingM    =   $this->MODEL('rating');
        $whereData  =   array('category' => '1', 'orderby' => array('type,asc', 'sort,desc'));
        if ($this->config['com_vip_type'] == 2 || $this->config['com_vip_type'] == 0) {
            $whereData['type'] = '1';
            if (!empty($this->comInfo['package'])) {

                $whereData['id'] = array('in', $this->comInfo['package']);
                $rows = $ratingM->getList($whereData);
            } else {
                $whereData['display'] = 1;

                $rows = $ratingM->getList($whereData);
                if (!empty($this->config['com_package_open'])) {
                    $packageOpenArr = explode(',', $this->config['com_package_open']);
                    if (in_array($this->comInfo['rating'], $packageOpenArr) || ($this->comInfo['vipetime'] > 0 && $this->comInfo['vipetime'] < time() && in_array('999', $packageOpenArr))) {

                        $rows = array();
                    }
                }
            }
            $this->yunset("rows", $rows);
            $this->com_tpl('member_right');
        } elseif ($this->config['com_vip_type'] == 1) {
            $whereData['type'] = '2';
            if (!empty($this->comInfo['package'])) {

                $whereData['id'] = array('in', $this->comInfo['package']);
                $times = $ratingM->getList($whereData);
            } else {
                $whereData['display'] = 1;

                $times = $ratingM->getList($whereData);
                if (!empty($this->config['com_package_open'])) {
                    $packageOpenArr = explode(',', $this->config['com_package_open']);
                    if (in_array($this->comInfo['rating'], $packageOpenArr) || ($this->comInfo['vipetime'] > 0 && $this->comInfo['vipetime'] < time() && in_array('999', $packageOpenArr))) {

                        $times = array();
                    }
                }
            }
            $this->yunset("times", $times);
            $this->com_tpl('member_time');
        }
    }

    function time_action()
    {

        $this->company_satic();
        $this->public_action();

        $sy_only_price = @explode(',', $this->config['sy_only_price']);
        if (in_array('vip', $sy_only_price)) {
            $this->yunset('meal', 1);
        }
        $ratingM = $this->MODEL('rating');
        $whereData = array('category' => '1', 'orderby' => array('type,asc', 'sort,desc'));
        if ($this->config['com_vip_type'] == 2) {
            $whereData['type'] = '1';
            if (!empty($this->comInfo['package'])) {

                $whereData['id'] = array('in', $this->comInfo['package']);
                $rows = $ratingM->getList($whereData);
            } else {
                $whereData['display'] = 1;

                $rows = $ratingM->getList($whereData);
                if (!empty($this->config['com_package_open'])) {
                    $packageOpenArr = explode(',', $this->config['com_package_open']);
                    if (in_array($this->comInfo['rating'], $packageOpenArr) || ($this->comInfo['vipetime'] > 0 && $this->comInfo['vipetime'] < time() && in_array('999', $packageOpenArr))) {

                        $rows = array();
                    }
                }
            }
            $this->yunset("rows", $rows);
            $this->com_tpl('member_right');
        } elseif ($this->config['com_vip_type'] == 1 || $this->config['com_vip_type'] == 0) {
            $whereData['type'] = '2';
            if (!empty($this->comInfo['package'])) {

                $whereData['id'] = array('in', $this->comInfo['package']);
                $times = $ratingM->getList($whereData);
            } else {

                $whereData['display'] = 1;

                $times = $ratingM->getList($whereData);
                if (!empty($this->config['com_package_open'])) {
                    $packageOpenArr = explode(',', $this->config['com_package_open']);
                    if (in_array($this->comInfo['rating'], $packageOpenArr) || ($this->comInfo['vipetime'] > 0 && $this->comInfo['vipetime'] < time() && in_array('999', $packageOpenArr))) {

                        $times = array();
                    }
                }
            }
            $this->yunset("times", $times);
            $this->com_tpl('member_time');
        }
    }

    function added_action()
    {

        $sy_only_price = @explode(',', $this->config['sy_only_price']);
        if (in_array('pack', $sy_only_price)) {
            $this->yunset('meal', 1);
        }

        $statis = $this->company_satic();

        if ($statis['rating_type'] == 2) {
            $this->ACT_msg("index.php?c=right", "时间会员无需购买增值服务！");
        }
        $this->public_action();
        $id = intval($_GET['id']);

        $ratingM = $this->MODEL('rating');
        $rows = $ratingM->getComServiceList(array('display' => '1', 'orderby' => 'sort,desc'));

        if (empty($id)) {
            $id = $rows[0]['id'];
        }
        $info = $ratingM->getComSerDetailList(array('type' => $id, 'orderby' => 'sort,desc'));

        if ($statis) {
            $discount = $ratingM->getInfo(array('id' => $statis['rating']));
            $this->yunset("discount", $discount);
        }
        $this->yunset("info", $info);
        $this->yunset("rows", $rows);
        $this->com_tpl('added');
    }
}

?>
