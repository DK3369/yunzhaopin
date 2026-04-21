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
class company_interview_controller extends adminCommon
{
    //面试模板
    function index_action()
    {
        $pageM = $this->MODEL('page');
        $yqmbM = $this->MODEL('yqmb');
        $typeStr = intval($_POST['type']);
        $keywordStr = trim($_POST['keyword']);

        if (!empty($keywordStr)) {
            if ($typeStr == 1) {
                //公司名称
                $comM = $this->MODEL('company');
                $com = $comM->getList(array('name' => array('like', $keywordStr)), array('field' => '`uid`'));
                $cuids = array();
                if (is_array($com['list'])) {
                    foreach ($com['list'] as $v) {
                        $cuids[] = $v['uid'];
                    }
                }
                $where['uid'] = array('in', pylode(',', $cuids));
            } elseif ($typeStr == 2) {
                //用户ID
                $where['uid'] = intval($keywordStr);
            }
        }

        if (isset($_POST['status'])) {
            $where['status'] = intval($_POST['status']);
        }
        //提取分页
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('yqmb', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        if ($pages['total'] > 0) {
            $where['orderby'] = array('status,asc', 'id,desc');
            $where['limit'] = $pages['limit'];

            $list = $yqmbM->getList($where, array('utype' => 'admin'));
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }

    /**
     * @desc 删除分享职位
     */
    function delYqmb_action()
    {
        if ($_POST['del']) {

            $yqmbM  =   $this->MODEL('yqmb');

            $return =   $yqmbM->delYqmb($_POST['del']);

            if ($return['errcode'] == 9) {
                $this->admin_json(0, $return['msg']);
            } else {
                $this->render_json($return['errcode'], $return['msg']);
            }
        } else {
            $this->render_json(1, '请选择要删除的内容！');
        }
    }

    function save_action()
    {
        $yqmbM  =   $this->MODEL('yqmb');

        if ($_POST['id']) {
            $where['id']    =   $_POST['id'];
        }
        $data   =   array(
            'uid'   =>  $_POST['uid']
        );
        $setdata    =   array(
            'name'      =>  $_POST['name'],
            'linkman'   =>  $_POST['linkman'],
            'linktel'   =>  $_POST['linktel'],
            'content'   =>  $_POST['content'],
            'intertime' =>  $_POST['intertime'],
            'address'   =>  $_POST['address'],
            'status'    =>  0
        );
        $return     =   $yqmbM->addInfo($setdata, $data, $where);

        if ($return['errcode'] == '9') {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }

    /**
     * 审核
     */
    function status_action()
    {
        if (!$_POST['pid'] || !$_POST['status']) {
            $this->render_json(1, '参数有误');
        }
        $yqmbM      =   $this->MODEL('yqmb');
        $statusData =   array(
            'status'        =>  intval($_POST['status']),
            'statusbody'    =>  trim($_POST['statusbody'])
        );

        $return     =   $yqmbM->statusYqmb($_POST['pid'], $statusData);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
}