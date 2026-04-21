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
class users_msg_controller extends adminCommon
{
    //会员/个人/求职咨询

    /**
     * 高级搜索参数选项
     */
    function set_search()
    {
        $search[] = array('param' => 'status', 'name' => '审核状态', 'value' => array('0' => '未审核', '1' => '已审核', '2' => '未通过'));
        $search[] = array('param' => 'job', 'name' => '职位类型', 'value' => array('1' => '普通', '2' => '高级'));
        $lo_time = array('1' => '今天', '3' => '最近三天', '7' => '最近七天', '15' => '最近半月', '30' => '最近一个月');
        $search[] = array('param' => 'zx', 'name' => '咨询时间', 'value' => $lo_time);
        $f_time = array('1' => '今天', '3' => '最近三天', '7' => '最近七天', '15' => '最近半月', '30' => '最近一个月');
        $search[] = array('param' => 'hf', 'name' => '回复时间', 'value' => $f_time);
        return $search;
    }

    /**
     * 会员-个人-求职咨询: 列表
     */
    function index_action()
    {
        if ($_POST['keyword']) {
            $keytype = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);
            if ($keytype == 1) {
                //咨询人
                $resumeM = $this->MODEL('resume');
                $rwhere['name'] = array('like', $keyword);
                $rlist = $resumeM->getResumeList($rwhere, array('field' => '`uid`'));
                if (is_array($rlist)) {
                    foreach ($rlist as $v) {
                        $muids[] = $v['uid'];
                    }
                }
                $userinfoM = $this->MODEL('userinfo');
                $mwhere['username'] = array('like', $keyword);
                $mlist = $userinfoM->getList($mwhere, array('field' => '`uid`'));
                if (is_array($mlist)) {
                    foreach ($mlist as $mv) {
                        !in_array($mv['uid'], $muids) && array_push($muids, $mv['uid']);
                    }
                }
                $where['uid'] = array('in', pylode(',', $muids));
//                $where['username'] = array('like', $keyword);
            } elseif ($keytype == 2) {
                //咨询职位
                $where['job_name'] = array('like', $keyword);
            } elseif ($keytype == 3) {
                //咨询公司
                $where['com_name'] = array('like', $keyword);
            } elseif ($keytype == 4) {
                //咨询内容
                $where['content'] = array('like', $keyword);
            } elseif ($keytype == 5) {
                //回复内容
                $where['reply'] = array('like', $keyword);
            }
        }
        //咨询时间
        if ($_POST['zx']) {
            $zx = intval($_POST['zx']);
            if ($zx == 1) {
                $where['datetime'] = array('>=', strtotime('today'));
            } else {
                $where['datetime'] = array('>=', strtotime('-' . $zx . ' day'));
            }
        }
        //回复时间
        if ($_POST['hf']) {
            $hf = intval($_POST['hf']);
            if ($hf == 1) {
                $where['reply_time'] = array('>=', strtotime('today'));
            } else {
                $where['reply_time'] = array('>=', strtotime('-' . $hf . ' day'));
            }
        }
        //职位类型
        if ($_POST['job']) {
            $where['type'] = intval($_POST['job']);
        }
        //审核状态
        if (isset($_POST['status'])) {
            $where['status'] = intval($_POST['status']);
        }
        //提取分页
        $pageM = $this->MODEL('page');
        $msgM = $this->MODEL('msg');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('msg', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = 'id,' . $_POST['order'];
            } else {
                $where['orderby'] = 'id,desc';
            }
            $where['limit'] = $pages['limit'];
            $List = $msgM->getList($where, array('utype' => 'admin'));
            $list = $List['list'];
        }



        $this->render_json(0, '', array('list' => $list, 'total' => intval($pages['total']),
            'perPage' => $pageSize, 'pageSizes' => $pageSizes));
    }
    public function getStatist_action(){
        //数据统计
        $msgM = $this->MODEL('msg');
        $numAll = intval($msgM->getNum());
        //未审核
        $numUnaudited = intval($msgM->getNum(array('status' => 0)));
        //已审核
        $numAudited = intval($msgM->getNum(array('status' => 1)));
        //未通过
        $numFailed = intval($msgM->getNum(array('status' => 2)));
        $data = array(
            'numAll' => $numAll,
            'numAudited' => $numAudited,
            'numUnaudited' => $numUnaudited,
            'numFailed' => $numFailed,
        );
        $this->render_json(0,'',$data);
    }
    /**
     * 会员-个人-求职咨询: 删除求职咨询
     */
    function del_action()
    {
        if ($_POST['id']) {
            $id = intval($_POST['id']);
        } elseif ($_POST['del']) {
            $id = $_POST['del'];
        }
        $msgM = $this->MODEL('msg');
        $return = $msgM->delInfo($id);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }

    /**
     * 会员-个人-求职咨询: 求职咨询预览
     */
    function msgshow_action()
    {
        if ($_POST['id']) {
            $id = intval($_POST['id']);
            $msgM = $this->MODEL('msg');
            $return = $msgM->getInfo(array('id' => $id));
            if (is_array($return) && count($return)) {
                $this->render_json(0, '', $return);
            } else {
                $this->render_json(8, '');
            }
        }
    }

    /**
     * 返回审核原因
     */
    function lockinfo_action()
    {
        $msgM = $this->MODEL('msg');
        $info = $msgM->getInfo(array('id' => intval($_POST['id'])), array('field' => '`statusbody`'));
        $this->render_json(0, '', trim($info['statusbody']));
    }

    /**
     * 修改
     */
    function msgedit_action()
    {
        $msgM = $this->MODEL('msg');
        $data = array(
            'content' => trim($_POST['content']),
            'reply' => trim($_POST['reply']),
            'reply_time' => time()
        );
        $return = $msgM->eidtMsg($_POST['id'], $data);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }

    /**
     * 审核操作
     */
    function status_action()
    {
        $msgM = $this->MODEL('msg');
        $statusData = array(
            'status' => intval($_POST['status']),
            'statusbody' => trim($_POST['statusbody'])
        );
        $return = $msgM->statusMsg($_POST['pid'], $statusData);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
}