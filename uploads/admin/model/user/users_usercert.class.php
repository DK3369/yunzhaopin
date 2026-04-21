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
class users_usercert_controller extends adminCommon
{
    //会员-个人-认证&审核：身份认证审核

    //设置高级搜索功能
    function set_search()
    {
        $search[] = array("param" => "status", "name" => '审核状态', "value" => array("1" => "已审核", "2" => "未审核", "3" => "未通过"));
        $lo_time = array('1' => '今天', '3' => '最近三天', '7' => '最近七天', '15' => '最近半月', '30' => '最近一个月');
        $search[] = array("param" => "time", "name" => '发布时间', "value" => $lo_time);
        $this->render_json(0, '', $search);
    }

    //身份认证审核
    function index_action()
    {
        $where['idcard_pic'] = array('<>', '');
        if ($_POST['status']) {
            $status = intval($_POST['status']);
            if ($status == 1) {
                $where['idcard_status'] = 1;
            } elseif ($status == 2) {
                $where['idcard_status'] = 0;
            } elseif ($status == 3) {
                $where['idcard_status'] = 2;
            }
        }
        if ($_POST['keyword']) {
            $keyword = trim($_POST['keyword']);
            $where['name'] = array('like', $keyword);
        }
        if ($_POST['time']) {
            $cert_time = intval($_POST['time']);
            if ($cert_time == 1) {
                $where['cert_time'] = array('>=', strtotime('today'));
            } else {
                $where['cert_time'] = array('>', strtotime('-' . $cert_time . ' day'));
            }
        }


        //提取分页
        $pageM = $this->MODEL('page');
        $resumeM = $this->MODEL('resume');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('resume', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('idcard_status,ASC', 'cert_time,DESC');
            }
            $where['limit'] = $pages['limit'];

            $list = $resumeM->getResumeList($where, array('utype' => 'admin'));
        }



        $this->render_json(0, '', array('list' => $list, 'total' => intval($pages['total']),
            'perPage' => $pageSize, 'pageSizes' => $pageSizes));
    }
    public function getSfStatist_action(){
        //数据统计
        $resumeM = $this->MODEL('resume');
        $numAll = intval($resumeM->getResumeNum(array('idcard_pic' => array('<>', ''))));
        //已审核
        $numAudited = intval($resumeM->getResumeNum(array('idcard_pic' => array('<>', ''), 'idcard_status' => '1')));;
        //未审核
        $numUnaudited = intval($resumeM->getResumeNum(array('idcard_pic' => array('<>', ''), 'idcard_status' => '0')));
        //未通过
        $numFailed = intval($resumeM->getResumeNum(array('idcard_pic' => array('<>', ''), 'idcard_status' => '2')));
        $data = array(
            'numAll' => $numAll,
            'numAudited' => $numAudited,
            'numUnaudited' => $numUnaudited,
            'numFailed' => $numFailed,
        );
        $this->render_json(0,'',$data);
    }
    /**
     * 审核
     */
    function status_action()
    {
        $resumeM = $this->MODEL('resume');
        $post = array(
            'idcard_status' => intval($_POST['status']),
            'statusbody' => trim($_POST['statusbody'])
        );
        $return = $resumeM->statusCert($_POST['uid'], array('post' => $post));
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
    /**
     * 查询审核原因
     */
    function sbody_action(){
        $resumeM  =  $this -> MODEL('resume');
        $resume   =  $resumeM -> getResumeInfo(array('uid'=>intval($_POST['uid'])),array('field'=>'statusbody'));
        $this->render_json(0, '', trim($resume['statusbody']));
    }
    /**
     * 删除个人认证
     */
    function del_action()
    {
        if ($_POST['id']) {
            $uid = intval($_POST['id']);
        } elseif ($_POST['del']) {
            $uid = $_POST['del'];
        }
        $resumeM = $this->MODEL('resume');
        $return = $resumeM->delResumeCert($uid);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
}