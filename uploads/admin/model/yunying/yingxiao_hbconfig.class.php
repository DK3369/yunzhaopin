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
class yingxiao_hbconfig_controller extends adminCommon
{
    // 海报设置
    function index_action()
    {
        $config = $this->config;
        $config = array(
            'sy_haibao_isopen' => isset($config['sy_haibao_isopen']) ? strval($config['sy_haibao_isopen']) : '1',
            'sy_haibao_web_type' => isset($config['sy_haibao_web_type']) ? strval($config['sy_haibao_web_type']) : '3',
            'sy_haibao_web_name' => isset($config['sy_haibao_web_name']) ? $config['sy_haibao_web_name'] : '',
            'sy_haibao_web_logo_n' => isset($config['sy_haibao_web_logo']) ? checkpic($config['sy_haibao_web_logo']) : ''
        );

        $this->render_json(0, 'ok', compact('config'));
    }

    // 海报设置保存
    function saveSet_action()
    {
        if (empty($_POST)) {
            $this->render_json(1, '参数错误');
        }

        if ($_FILES['sy_haibao_web_logo']['tmp_name']) {
            $uploadM = $this->MODEL('upload');

            $upData['file'] = $_FILES['sy_haibao_web_logo'];
            $upData['dir'] = 'logo';

            $upRes = $uploadM->newUpload($upData);
            if ($upRes && !empty($upRes['msg'])) {
                $this->render_json(1, $upRes['msg']);
            } else {
                $configData['sy_haibao_web_logo'] = $upRes['picurl'];
            }
        }

        $configM = $this->MODEL('config');

        $configData['sy_haibao_isopen'] = $_POST['sy_haibao_isopen'];
        $configData['sy_haibao_web_type'] = $_POST['sy_haibao_web_type'];
        $configData['sy_haibao_web_name'] = trim($_POST['sy_haibao_web_name']);

        $configM->setConfig($configData);

        $this->web_config();

        $this->admin_json(0, "海报设置保存成功");
    }

    // 职位海报
    function job_action()
    {
        $whbM = $this->MODEL('whb');

        $list = $whbM->getWhbList(array('type' => 1, 'orderby' => 'sort,desc'));

        $this->render_json(0, 'ok', compact('list'));
    }

    // 企业海报
    function com_action()
    {
        $whbM = $this->MODEL('whb');

        $list = $whbM->getWhbList(array('type' => 2, 'orderby' => array('style,desc', 'sort,desc')));

        $this->render_json(0, 'ok', compact('list'));
    }

    // 邀请注册海报
    function inviteReg_action()
    {
        $whbM = $this->MODEL('whb');

        $list = $whbM->getWhbList(array('type' => 3, 'orderby' => array('style,desc', 'sort,desc')));

        $this->render_json(0, 'ok', compact('list'));
    }

    // 公招海报
    function gongzhao_action()
    {
        $whbM = $this->MODEL('whb');

        $list = $whbM->getWhbList(array('type' => 4, 'orderby' => array('style,desc', 'sort,desc')));

        $this->render_json(0, 'ok', compact('list'));
    }

    // 海报配置保存
    function saveWhbConfig_action()
    {
        if (!isset($_POST["sy_job_hb"]) && !isset($_POST["sy_com_hb"]) && !isset($_POST["sy_invite_reg_hb"]) && !isset($_POST["sy_gongzhao_hb"])) {
            $this->render_json(1, '参数错误');
        }

        unset($_POST['pytoken']);

        $whbM = $this->MODEL('whb');

        $hbIds = array();

        $openHbIds = array();
        $closeHbIds = array();

        // 职位海报
        if (isset($_POST['sy_job_hb'])) {
            $type = 1;
            $hbIds = explode(',', $_POST['sy_job_hb']);
        }
        // 企业海报
        if (isset($_POST['sy_com_hb'])) {
            $type = 2;
            $hbIds = explode(',', $_POST['sy_com_hb']);
        }
        // 邀请注册海报
        if (isset($_POST['sy_invite_reg_hb'])) {
            $type = 3;
            $hbIds = explode(',', $_POST['sy_invite_reg_hb']);
        }
        // 公招海报
        if (isset($_POST['sy_gongzhao_hb'])) {
            $type = 4;
            $hbIds = explode(',', $_POST['sy_gongzhao_hb']);
        }
        $imgList = $whbM->getWhbList(array('orderby' => 'sort,desc', 'type' => $type));

        foreach ($imgList as $k => $v) {
            if (in_array($v['id'], $hbIds)) {
                $openHbIds[] = $v['id'];
            } else {
                $closeHbIds[] = $v['id'];
            }
        }

        if (!empty($openHbIds)) {
            $whbM->updateWhb(array('isopen' => 1), array('id' => array('in', pylode(',', $openHbIds))));
        }
        if (!empty($closeHbIds)) {
            $whbM->updateWhb(array('isopen' => 0), array('id' => array('in', pylode(',', $closeHbIds))));
        }

        $this->web_config();

        $this->admin_json(0, '海报设置配置修改成功');
    }

    // 删除海报
    function delWhb_action()
    {
        if (empty($_POST['id'])) {
            $this->render_json(1, '参数错误');
        }

        $WhbM = $this->MODEL('whb');

        $whb = $WhbM->getWhb(array('id' => $_POST['id']));

        if (!empty($whb)) {
            $WhbM->delWhb(array('id' => $whb['id']));
            $this->admin_json(0, '海报（ID：' . $whb['id'] . '）删除成功');
        }
    }

    // 海报保存-职位海报
    function saveWhb_action()
    {
        $whbM = $this->MODEL('whb');

        $_POST = $this->post_trim($_POST);

        $id = $_POST['id'];

        $data = array(
            'type' => intval($_POST['type']),
            'name' => $_POST['name'],
            'sort' => intval($_POST['sort']),
            'num' => intval($_POST['num']),
            'style' => intval($_POST['style']),
            'isopen' => intval($_POST['isopen'])
        );

        if ($_FILES['pic']['tmp_name']) {
            $upArr = array(
                'file' => $_FILES['pic'],
                'dir' => 'whb'
            );

            $uploadM = $this->MODEL('upload');

            $pic = $uploadM->newUpload($upArr);

            if (!empty($pic['msg'])) {
                $this->render_json(1, $pic['msg']);
            } elseif (!empty($pic['picurl'])) {
                $data['pic'] = $pic['picurl'];
            }
        }

        $where = array();

        if ($id) {
            $where['id'] = $id;
        }

        $return = $whbM->setWhb($data, $where);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }
}

?>