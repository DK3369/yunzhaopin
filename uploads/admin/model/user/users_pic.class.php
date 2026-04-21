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
class users_pic_controller extends adminCommon
{
    //会员-个人-认证&审核

    function set_search()
    {
        $search_list[] = array(
            "param" => "status",
            "name" => '审核状态',
            "value" => array('0' => '已审核', '1' => '未审核')
        );
        return $search_list;
    }
//region 个人头像

    /**
     * 个人头像（列表）
     */
    function index_action()
    {
        $where['photo'] = array('<>', '');
        $where['defphoto'] = 1;
        if ($_POST['keyword']) {
            $keytype = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);
            if ($keytype == 1) {
                $where['name'] = array('like', $keyword);
            } elseif ($keytype == 2) {
                $where['uid'] = array('=', $keyword);
            }
        }
        if (isset($_POST['status'])) {
            $status = intval($_POST['status']);
            $where['photo_status'] = $status;
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
                $where['orderby'] = array('photo_status,desc', 'uid,desc');
            }
            $where['limit'] = $pages['limit'];
            $list = $resumeM->getResumeList($where, array('utype' => 'admin', 'field' => '`uid`,`name`,`sex`,`photo`,`photo_status`'));
        }



        $this->render_json(0, '', array('list' => $list, 'total' => intval($pages['total']),
            'perPage' => $pageSize, 'pageSizes' => $pageSizes));
    }
    public function getStatist_action(){
        //数据统计
        $resumeM = $this->MODEL('resume');
        //数据统计
        $numAll = intval($resumeM->getResumeNum(array('photo' => array('<>', ''), 'defphoto' => 1)));
        //已审核
        $numAudited = intval($resumeM->getResumeNum(array('photo' => array('<>', ''), 'defphoto' => 1, 'status' => 1)));
        //未审核
        $numUnaudited = intval($resumeM->getResumeNum(array('photo' => array('<>', ''), 'defphoto' => 1, 'status' => 0)));
        $data = array(
            'numAll' => $numAll,
            'numAudited' => $numAudited,
            'numUnaudited' => $numUnaudited,
        );
        $this->render_json(0,'',$data);
    }
    /**
     * 个人头像（获取审核说明）
     */
    function getStatusBody_action()
    {
        $resumeM = $this->MODEL('resume');
        $return = $resumeM->getInfo(array('uid' => intval($_POST['uid']), 'rfield' => 'photo_statusbody'));
        $this->render_json(0, '', trim($return['resume']['photo_statusbody']));
    }

    /**
     * 个人头像（审核）
     */
    function status_action()
    {
        $resumeM = $this->MODEL('resume');
        $post = array(
            'photo_status' => intval($_POST['status']),
            'photo_statusbody' => $_POST['statusbody']
        );
        $return = $resumeM->statusPhoto($_POST['uid'], array('post' => $post));
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }

    /**
     * 个人头像（修改）
     */
    function savePhoto_action()
    {
        $file = $_FILES['file'];
        $resumeM = $this->MODEL('resume');
        $id = intval($_POST['id']);
        $return = $resumeM->upPhoto(array('uid' => $id), array('photo' => $file));
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }

    /**
     * 个人头像（删除）
     */
    function delPhoto_action()
    {
        if ($_POST['id']) {
            $id = intval($_POST['id']);
        } elseif ($_POST['del']) {
            $id = $_POST['del'];
        }
        $resumeM = $this->MODEL('resume');
        $return = $resumeM->delPhoto($id);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
//endregion
//region 作品案例
    /**
     * 作品案例（列表）
     */
    function show_action()
    {
        $pageM = $this->MODEL('page');
        $resumeM = $this->MODEL('resume');
        if ($_POST['keyword']) {
            $keytype = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);
            if ($keytype == 1) {
                $resume = $resumeM->getResumeList(array('name' => array('like', $keyword)), array('field' => 'uid'));
                if ($resume) {
                    foreach ($resume as $v) {
                        $uids[] = $v['uid'];
                    }
                    $where['uid'] = array('in', pylode(',', $uids));
                }
            } elseif ($keytype == 2) {
                $where['uid'] = array('=', $keyword);
            }
        }
        if (isset($_POST['status'])) {
            $status = intval($_POST['status']);
            $where['status'] = $status;
        }
        //提取分页
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('resume_show', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('status,desc', 'id,desc');
            }
            $where['limit'] = $pages['limit'];
            $list = $resumeM->getResumeShowList($where, array('utype' => 'admin'));
        }

        $this->render_json(0, '', array('list' => $list, 'total' => intval($pages['total']),
            'perPage' => $pageSize, 'pageSizes' => $pageSizes));
    }
    public function getShowStatist_action(){
        //数据统计
        $resumeM = $this->MODEL('resume');
        $numAll = intval($resumeM->getResumeShowNum());
        //已审核
        $numAudited = intval($resumeM->getResumeShowNum(array('status' => 0)));
        //未审核
        $numUnaudited = intval($resumeM->getResumeShowNum(array('status' => 1)));
        $data = array(
            'numAll' => $numAll,
            'numAudited' => $numAudited,
            'numUnaudited' => $numUnaudited,
        );
        $this->render_json(0,'',$data);
    }
    /**
     * 作品（获取审核说明）
     */
    function getShowStatusBody_action()
    {
        $resumeM = $this->MODEL('resume');
        $return = $resumeM->getResumeShowInfo(array('id' => intval($_POST['id'])), array('field' => '`statusbody`'));
        $this->render_json(0, '', trim($return['statusbody']));
    }

    /**
     * 作品（审核）
     */
    function showStatus_action()
    {
        $resumeM = $this->MODEL('resume');
        $post = array(
            'status' => intval($_POST['status']),
            'statusbody' => $_POST['statusbody']
        );
        $return = $resumeM->statusShow($_POST['id'], array('post' => $post));
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }

    /**
     * 作品案例（修改）
     */
    function saveShow_action()
    {
        $resumeM = $this->MODEL('resume');
        $post = array(
            'title' => $_POST['title'],
            'sort' => $_POST['sort'],
        );
        $data = array(
            'post' => $post,
            'id' => intval($_POST['id']),
            'file' => $_FILES['file']
        );
        $return = $resumeM->addResumeShow($data);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }

    /**
     * 作品案例（删除）
     */
    function delShow_action()
    {
        if ($_POST['id']) {
            $id = intval($_POST['id']);
        } elseif ($_POST['del']) {
            $id = $_POST['del'];
        }
        $resumeM = $this->MODEL('resume');
        $return = $resumeM->delShow($id, array('utype' => 'admin'));
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
//endregion
}