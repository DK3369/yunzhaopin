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

class yingxiao_hrlog_controller extends adminCommon
{
    // HR报告列表
    function index_action()
    {
        if (trim($_POST['keyword'])) {
            $companyM = $this->MODEL('company');
            $com = $companyM->getChCompanyList(array('name' => array('like', trim($_POST['keyword']))), array('field' => 'uid'));
            if (!empty($com)) {
                $uids = array();
                foreach ($com as $v) {
                    $uids[] = $v['uid'];
                }
                $where['uid'] = array('in', pylode(',', $uids));
            } else {
                $where['uid'] = '';
            }
        }

        $pageM = $this->MODEL('page');

        $pages = $pageM->adminPageList('hr_log', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];
        $list = array();
        if ($pages['total'] > 0) {
            $where['limit'] = $pages['limit'];
            $hrM = $this->MODEL('hrlog');
            $list = $hrM->getHrLogList($where);
        }

        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
    }

    // 设置-数据获取
    function set_action()
    {
        $config = $this->config;

        $set = array(
            'sy_yearreport_isopen' => $config['sy_yearreport_isopen'] ? $config['sy_yearreport_isopen'] : '0',
            'sy_yearreport_ewmtype' => $config['sy_yearreport_ewmtype'],
            'sy_yearreport_tip_n' => checkpic($config['sy_yearreport_tip']),
            'sy_yearreport_pic_n' => checkpic($config['sy_yearreport_pic']),
        );

        $this->render_json(0, 'ok', compact('set'));
    }

    // 设置-数据保存
    function setSave_action()
    {
        if ($_FILES['sy_yearreport_pic']['tmp_name']) {
            $upArr = array(
                'file' => $_FILES['sy_yearreport_pic'],
                'dir' => 'whb'
            );

            $uploadM = $this->MODEL('upload');

            $pic = $uploadM->newUpload($upArr);

            if (!empty($pic['msg'])) {
                $this->ACT_layer_msg($pic['msg'], 8);
            } elseif (!empty($pic['picurl'])) {
                $configData['sy_yearreport_pic'] = $pic['picurl'];
            }
        }
        if ($_FILES['sy_yearreport_tip']['tmp_name']) {
            $upArr = array(
                'file' => $_FILES['sy_yearreport_tip'],
                'dir' => 'whb'
            );

            $uploadM = $this->MODEL('upload');

            $pic = $uploadM->newUpload($upArr);

            if (!empty($pic['msg'])) {
                $this->ACT_layer_msg($pic['msg'], 8);
            } elseif (!empty($pic['picurl'])) {
                $configData['sy_yearreport_tip'] = $pic['picurl'];
            }
        }

        $configM = $this->MODEL('config');

        $configData['sy_yearreport_isopen'] = $_POST['sy_yearreport_isopen'] ? 1 : 0;
        $configData['sy_yearreport_ewmtype'] = $_POST['sy_yearreport_ewmtype'];

        $configM->setConfig($configData);

        $this->web_config();

        $this->admin_json(0, '设置配置修改成功');
    }

    // 年度报告修改-保存
    function editsave_action()
    {
        if (empty($_POST['id'])) {
            $this->render_json(1, '参数错误');
        }

        $id = $_POST['id'];
        unset($_POST['id']);

        $hrM = $this->MODEL('hrlog');
        if (!empty($_POST['lastwork'])) {
            $_POST['lastwork'] = strtotime($_POST['lastwork']);
        }
        $_POST['uptime'] = time();
        $return = $hrM->uphrlog(array('id' => $id), $_POST);

        if ($return) {
            $this->admin_json(0, '年度报告数据(ID:' . $id . ')修改成功');
        } else {
            $this->render_json(1, '年度报告数据修改失败');
        }
    }
    function rehrlog_action(){
        if (empty($_POST['id'])) {
            $this->render_json(1, '参数错误');
        }

        $id = $_POST['id'];
        unset($_POST['id']);

        $hrM = $this->MODEL('hrlog');
        $hrlog = $hrM->gethrlog(array('id'=>$id));

        if(!empty($hrlog)){
            $return = $hrM->sethrlog($hrlog['uid'],array('adminup'=>true));
        }
        
        if ($return) {
            $this->admin_json(0, '年度报告数据(ID:' . $id . ')修改成功');
        } else {
            $this->render_json(1, '年度报告数据修改失败');
        }
    }
    // 年度报告-海报链接生成
    function getHb_action()
    {
        $hbUrl = Url('ajax', array(
            'c' => 'lastYearReport',
            'uid' => $_POST['uid']
        ));

        $this->render_json(0, 'ok', compact('hbUrl'));
    }

    // 区域报告
    function datashowset_action()
    {
        $previewUrl = $this->config['sy_weburl'] . '/wap/index.php?c=ajax&a=dataShowIndex';
        $previewCode = Url('ajax', array(
            'c' => 'pubqrcode',
            'toc' => 'ajax',
            'toa' => 'dataShowIndex',
            'toid' => -1
        ));

        $config = $this->config;
        $config = array(
            'sy_datashow_title' => $config['sy_datashow_title'],
            'sy_datashowhy_base' => $config['sy_datashowhy_base'],
            'sy_datashowreg_base' => $config['sy_datashowreg_base'],
            'sy_datashowlogin_base' => $config['sy_datashowlogin_base'],
            'sy_datashowjob_base' => $config['sy_datashowjob_base'],
            'sy_datashow_city_lev' => $config['sy_datashow_city_lev'],
        );

        $this->render_json(0, 'ok', compact('previewUrl', 'previewCode', 'config'));
    }

    // 区域报告-保存
    function datashowsetSave_action()
    {
        $configM = $this->MODEL('config');

        $configData['sy_datashow_title'] = trim($_POST['sy_datashow_title']);
        $configData['sy_datashowhy_base'] = intval($_POST['sy_datashowhy_base']);
        $configData['sy_datashowreg_base'] = intval($_POST['sy_datashowreg_base']);
        $configData['sy_datashowlogin_base'] = intval($_POST['sy_datashowlogin_base']);
        $configData['sy_datashowjob_base'] = intval($_POST['sy_datashowjob_base']);
        $configData['sy_datashow_city_lev'] = $_POST['sy_datashow_city_lev'] ? intval($_POST['sy_datashow_city_lev']) : 2;
        $configM->setConfig($configData);

        $this->web_config();

        $this->admin_json(0, '区域报告配置修改成功');
    }

    function setlog_action()
    {

        $limit = 1;
        $lastyear = date('Y', strtotime('-1 year'));
        $start = strtotime($lastyear . '-01-01');
        $end = strtotime($lastyear . '-12-31');
        // 查询今年登录过的企业
        $logM = $this->MODEL('log');
        $frontW = array('usertype' => 2, 'groupby' => 'uid');
        $frontW['PHPYUNBTWSTART'] = '';
        $frontW['ctime'][] = array('>', $start);
        $frontW['ctime'][] = array('<', $end);
        $frontW['PHPYUNBTWEND'] = '';

        if (isset($_POST['page'])) {//分页
            $pagenav = ($_POST['page'] - 1) * $limit;
            $where['limit'] = array($pagenav, $limit);
        } else {
            $where['limit'] = $limit;
        }

        $com = $logM->getLoginlogList(array_merge($frontW, $where), array('field' => 'uid'));

        $total = $logM->getLoginlogList($frontW, array('field' => 'uid'));
        $remaind = count($total) - intval($_POST['page']);

        $hrM = $this->MODEL('hrlog');

        foreach ($com as $v) {
            $hrM->sethrlog($v['uid']);
        }
        if ($remaind > 0) {
            echo json_encode(array('error' => 1, 'msg' => '数据生成中，还剩余' . $remaind . '条', 'page' => $_POST['page'] + 1));
        } else {
            echo json_encode(array('error' => 0, 'msg' => '数据已全部生成'));
        }
    }
}

?>