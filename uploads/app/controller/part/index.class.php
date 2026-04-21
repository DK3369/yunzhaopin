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

class index_controller extends part_controller
{
    //兼职列表
    function index_action()
    {
        $this->toLoginPage();

        $cacheM =   $this->MODEL('cache');
        $cache  =   $cacheM->GetCache(array('city', 'part'));
        $this->yunset($cache);

        if ($_GET['city']) {

            $city                   =   explode("_", $_GET['city']);
            $_GET['provinceid']     =   $city[0];
            $_GET['cityid']         =   $city[1];
            $_GET['three_cityid']   =   $city[2];
        }
        if ($this->config['province']) {
            $_GET['provinceid']     =   $this->config['province'];
        }
        if ($this->config['cityid']) {
            $_GET['cityid']         =   $this->config['cityid'];
        }
        if ($this->config['three_cityid']) {
            $_GET['three_cityid']   =   $this->config['three_cityid'];
        }
        if ($_GET['part_type']) {
            $search[]               =   $cache['partclass_name'][$_GET['part_type']];
        }
        if ($_GET['cycle']) {
            $search[]               =   $cache['partclass_name'][$_GET['cycle']];
        }
        if ($_GET['provinceid']) {
            $search[]               =   $cache['city_name'][$_GET['provinceid']];
        }
        if ($_GET['cityid']) {
            $search[]               =   $cache['city_name'][$_GET['cityid']];
        }
        if ($_GET['three_cityid']) {
            $search[]               =   $cache['city_name'][$_GET['three_cityid']];
        }
        if ($_GET['keyword']) {
            $search[]               =   $_GET['keyword'];
        }
        if (!empty($search)) {
            $data['seacrh_class']   =   @implode("-", $search);
            $this->data = $data;
        }
        $partKeyword                =   array();
        include PLUS_PATH."keyword.cache.php";
        if (isset($keyword) && is_array($keyword)) {
            foreach ($keyword as $k => $v) {
                if ($v['type'] == '2' && $v['tuijian'] == '1') {

                    $partKeyword[]  =   $v;
                }
            }
        }
        $this->yunset("partkeyword", $partKeyword);
        $this->seo('part_index');
        $this->yun_tpl(array('index'));
    }

    //兼职详情
    function show_action()
    {

        $partM      =   $this->MODEL("part");
        if ($_GET['id']) {

            $id     =   (int)$_GET['id'];
            $info   =   $partM->getInfo(array('id' => $id), array('cache' => 1, 'com' => 1, 'uid' => $this->uid, 'usertype' => $this->usertype));
            $job    =   $info['info'];
            $this->yunset($info['cache']);

            if ($job['uid'] != $this->uid && ($job['id'] == '' || $job['state'] == 0 || $job['state'] == 3)) {
                $this->ACT_msg($this->config['sy_weburl'], '该兼职暂无法展示！');
            }

            $partM->upInfo(array('hits' => array('+', 1)), array('id' => $id));

            if ($this->usertype == 1) {

                $apply      =   $partM->getPartSqInfo(array("uid" => $this->uid, "jobid" => $id));
                $this->yunset('apply', $apply);

                $collect    =   $partM->getPartCollectInfo(array("uid" => $this->uid, "jobid" => $id));
                $this->yunset('collect', $collect);
            }
            $this->yunset('job', $job);
            $this->yunset('ComInfo', $info['com']);

        }
        $data['part_name']  =   $job['name'];
        $this->data         =   $data;
        $this->seo('part_show');
        $this->yun_tpl(array('show'));
    }

    //兼职报名
    function partapply_action()
    {

        $partM  =   $this->MODEL('part');

        $data   =   array(
            'uid'       =>  $this->uid,
            'usertype'  =>  $this->usertype,
            'jobid'     =>  (int)$_POST['jobid'],
            'port'      =>  '1'
        );
        $return =   $partM->addPartSq($data);
        echo json_encode($return);
        die;
    }

    //兼职收藏
    function partcollect_action()
    {

        $partM  =   $this->MODEL("part");

        $data   =   array(
            'uid'       =>  $this->uid,
            'usertype'  =>  $this->usertype,
            'jobid'     =>  (int)$_POST['jobid'],
            'comid'     =>  (int)$_POST['comid'],

        );
        $return =   $partM->addPartCollect($data);
        echo json_encode($return);
        die;
    }

    //微信扫码查看联系方式
    function telQrcode_action()
    {

        $WxM    = $this->MODEL('weixin');
        $qrcode = $WxM->pubWxQrcode('parttel', $_GET['id']);

        if (isset($qrcode)) {

            $imgStr = CurlGet($qrcode);
            header("Content-Type:image/png");
            echo $imgStr;
        }
    }
}

?>