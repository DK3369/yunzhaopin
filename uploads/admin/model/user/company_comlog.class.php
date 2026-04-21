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
class company_comlog_controller extends adminCommon
{
    //会员--企业--行为记录

//region 会员-企业-行为记录：职位申请记录
    /**
     * 会员-企业-行为记录：职位申请记录
     */
    function index_action()
    {
        $JobM = $this->MODEL('job');

        $typeStr = intval($_POST['type']);
        $keywordStr = trim($_POST['keyword']);
        if (!empty($keywordStr)) {
            if ($typeStr == 1) {
                //职位名称
                $where['job_name'] = array('like', $keywordStr);
            } elseif ($typeStr == 2) {
                //公司名称
                $where['com_name'] = array('like', $keywordStr);
            } elseif ($typeStr == 3) {
                //个人姓名
                $resumeM = $this->MODEL('resume');
                $rwhere['name'] = array('like', $keywordStr);
                $rlist = $resumeM->getResumeList($rwhere, array('field' => '`uid`'));
                if (is_array($rlist)) {
                    foreach ($rlist as $v) {
                        $muids[] = $v['uid'];
                    }
                }
                $where['uid'] = array('in', pylode(',', $muids));
            }
        }
        //是否查看
        if ($_POST['browse']) {
            $where['is_browse'] = intval($_POST['browse']);
        }
        //申请时间
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            $where['PHPYUNBTWSTART_A'] = '';
            $where['datetime'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
            $where['datetime'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
            $where['PHPYUNBTWEND_A'] = '';
        }
        //  申请职位ID
        if (isset($_POST['job_id']) && !empty($_POST['job_id'])) {
            $where['job_id'] = $_POST['job_id'];
        }
        //  申请企业ID
        if (isset($_POST['com_id']) && !empty($_POST['com_id'])) {
            $where['com_id'] = $_POST['com_id'];
        }
        //  申请用户ID
        if (isset($_POST['user_id']) && !empty($_POST['user_id'])) {
            $where['uid'] = $_POST['user_id'];
        }

        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('userid_job', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('id,desc');
            }

            $where['limit'] = $pages['limit'];

            $rows = $JobM->getSqJobList($where, array('utype' => 'admin'));
            foreach ($rows as $value) {
                $list[] = array(
                    'id' => $value['id'],
                    'uid' => $value['uid'],
                    'job_name' => $value['job_name'],
                    'job_url' => $value['job_url'],
                    'com_name' => $value['com_name'],
                    'com_url' => $value['com_url'],
                    'username_n' => $value['username_n'],
                    'eid' => $value['eid'],
                    'telphone' => $value['telphone'],
                    'telphone_url' => $value['telphone_url'],
                    'is_browse' => $value['is_browse'],
                    'datetime' => $value['datetime'],
                    'datetime_n_n' => $value['datetime_n_n'],
                    'isdel_n' => $value['isdel_n'],
                );
            }
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }

    /**
     * 删除职位申请记录
     */
    function deluseridjob_action()
    {
        $JobM = $this->MODEL('job');
        if (is_array($_POST['del'])) {
            $id = $_POST['del'];
        } else {
            $id = intval($_POST['del']);
        }

        $arr = $JobM->delSqJob($id, array('utype' => 'admin'));
        if ($arr['errcode'] == 9 || $arr['errcode'] === 0) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json($arr['errcode'], $arr['msg']);
        }
    }
//endregion
//region 会员-企业-行为记录：邀请面试记录
    /**
     * 会员-企业-行为记录：邀请面试记录
     */
    function useridmsg_action()
    {
        $JobM = $this->MODEL('job');

        $typeStr = intval($_POST['type']);
        $keywordStr = trim($_POST['keyword']);
        if (!empty($keywordStr)) {
            if ($typeStr == 1) {
                //姓名
                $resumeM = $this->MODEL('resume');
                $rwhere['name'] = array('like', $keywordStr);
                $rlist = $resumeM->getResumeList($rwhere, array('field' => '`uid`'));
                if (is_array($rlist)) {
                    foreach ($rlist as $v) {
                        $muids[] = $v['uid'];
                    }
                }
                $where['uid'] = array('in', pylode(',', $muids));
            } elseif ($typeStr == 2) {
                //邀约企业
                $where['fname'] = array('like', $keywordStr);
            } elseif ($typeStr == 3) {
                //邀请标题
                $where['title'] = array('like', $keywordStr);
            } elseif ($typeStr == 4) {
                //邀请内容
                $where['content'] = array('like', $keywordStr);
            }
        }
        //是否查看
        if ($_POST['browse']) {
            $where['is_browse'] = intval($_POST['browse']);
        }
        //邀约时间
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            $where['PHPYUNBTWSTART_A'] = '';
            $where['datetime'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
            $where['datetime'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
            $where['PHPYUNBTWEND_A'] = '';
        }
        //岗位ID
        if (isset($_POST['jobid']) && !empty($_POST['jobid'])) {
            $where['jobid'] = intval($_POST['jobid']);
        }
        //企业ID
        if (isset($_POST['comid']) && !empty($_POST['comid'])) {
            $where['fid'] = intval($_POST['comid']);
        }

        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('userid_msg', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('id,desc');
            }
            $where['limit'] = $pages['limit'];
            $rows = $JobM->getYqmsList($where, array('utype' => 'admin'));
            foreach ($rows as $value) {
                $list[] = array(
                    'id' => $value['id'],
                    'realname' => $value['realname'],
                    'name' => $value['name'],
                    'jobname' => $value['jobname'],
                    'job_url' => $value['job_url'],
                    'fname' => $value['fname'],
                    'com_url' => $value['com_url'],
                    'content' => $value['content'],
                    'datetime' => $value['datetime'],
                    'datetime_n' => $value['datetime_n'],
                    'is_browse' => $value['is_browse'],
                    'isdel_n' => $value['isdel_n'],
                    'eid' => $value['eid'],
                    'uid' => $value['uid'],
                );
            }
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }

    /**
     * 删除邀请面试记录
     */
    function deluseridmsg_action()
    {
        $JobM = $this->MODEL('job');
        if (is_array($_POST['del'])) {
            $id = $_POST['del'];
        } else {
            $id = intval($_POST['del']);
        }
        $arr = $JobM->delYqms($id, array('utype' => 'admin'));
        if ($arr['errcode'] == 9 || $arr['errcode'] === 0) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json($arr['errcode'], $arr['msg']);
        }
    }
//endregion
//region 会员-企业-行为记录：职位浏览记录
    /**
     * 会员-企业-行为记录：职位浏览记录
     */
    function lookjob_action()
    {
        $JobM = $this->MODEL('job');

        $typeStr = intval($_POST['type']);
        $keywordStr = trim($_POST['keyword']);
        if (!empty($keywordStr)) {
            if ($typeStr == 1) {
                //姓名
                $resumeM = $this->MODEL('resume');
                $rwhere['name'] = array('like', $keywordStr);
                $rlist = $resumeM->getResumeList($rwhere, array('field' => '`uid`'));
                if (is_array($rlist)) {
                    foreach ($rlist as $v) {
                        $muids[] = $v['uid'];
                    }
                }
                $where['uid'] = array('in', pylode(',', $muids));

            } else {
                $JobM = $this->MODEL('job');
                if ($typeStr == 2) {
                    //职位名称
                    $job = $JobM->getList(array('name' => array('like', $keywordStr)), array('field' => '`id`'));
                    $jobids = array();
                    if (is_array($job['list'])) {
                        foreach ($job['list'] as $v) {
                            $jobids[] = $v['id'];
                        }
                    }
                    $where['jobid'] = array('in', pylode(',', $jobids));
                } elseif ($typeStr == 3) {
                    //所属企业
                    $job = $JobM->getList(array('com_name' => array('like', $keywordStr)), array('field' => '`uid`'));
                    $cuids = array();
                    if (is_array($job['list'])) {
                        foreach ($job['list'] as $v) {
                            $cuids[] = $v['uid'];
                        }
                    }
                    $where['com_id'] = array('in', pylode(',', $cuids));
                }
            }
        }
        //浏览时间
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            $where['PHPYUNBTWSTART_A'] = '';
            $where['datetime'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
            $where['datetime'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
            $where['PHPYUNBTWEND_A'] = '';
        }
        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('look_job', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();

        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {

            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('id,desc');
            }
            $where['limit'] = $pages['limit'];
            $list = $JobM->getLookJobList($where, array('utype' => 'admin'));
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }

    /**
     * 删除职位浏览记录
     */
    function dellookjob_action()
    {
        $JobM = $this->MODEL('job');
        if (is_array($_POST['del'])) {
            $id = $_POST['del'];
        } else {
            $id = intval($_POST['del']);
        }
        $arr = $JobM->delLookJob($id, array('utype' => 'admin'));
        if ($arr['errcode'] == 9 || $arr['errcode'] === 0) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json($arr['errcode'], $arr['msg']);
        }
    }
//endregion
//region 会员-企业-行为记录：兼职报名记录
    /**
     * 会员-企业-行为记录：兼职报名记录
     */
    function partapply_action()
    {
        $partM = $this->MODEL('part');

        $typeStr = intval($_POST['type']);
        $keywordStr = trim($_POST['keyword']);
        if (!empty($keywordStr)) {
            if ($typeStr == 1) {
                //姓名
                $resumeM = $this->MODEL('resume');
                $rwhere['name'] = array('like', $keywordStr);
                $rlist = $resumeM->getResumeList($rwhere, array('field' => '`uid`'));
                if (is_array($rlist)) {
                    foreach ($rlist as $v) {
                        $muids[] = $v['uid'];
                    }
                }
                $where['uid'] = array('in', pylode(',', $muids));
            } elseif ($typeStr == 2) {
                //职位名称
                $job = $partM->getList(array('name' => array('like', $keywordStr)), array('field' => '`id`'));
                $jobids = array();
                if (is_array($job)) {
                    foreach ($job as $v) {
                        $jobids[] = $v['id'];
                    }
                }
                $where['jobid'] = array('in', pylode(',', $jobids));
            } elseif ($typeStr == 3) {
                //所属企业
                $job = $partM->getList(array('com_name' => array('like', $keywordStr)), array('field' => '`uid`'));
                $cuids = array();
                if (is_array($job)) {
                    foreach ($job as $v) {
                        $cuids[] = $v['uid'];
                    }
                }
                $where['comid'] = array('in', pylode(',', $cuids));
            }
        }
        //标记状态
        if ($_POST['status']) {
            $where['status'] = intval($_POST['status']);
        }
        //报名时间
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            $where['PHPYUNBTWSTART_A'] = '';
            $where['ctime'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
            $where['ctime'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
            $where['PHPYUNBTWEND_A'] = '';
        }
        if ($_POST['job_id']) {
            $where['jobid'] = $_POST['job_id'];
        }

        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('part_apply', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('id,desc');
            }
            $where['limit'] = $pageM->pageLimit($_POST);
            $rows = $partM->getPartSqList($where);
            foreach ($rows as $key => $value) {
                $value['part_url'] = Url('part', array('c' => 'show', 'id' => $value['jobid'], 'look' => 'admin'));
                $value['com_url'] = Url('company', array('c' => 'show', 'id' => $value['comid'], 'look' => 'admin'));
                $value['ctime_n_n'] = $value['ctime'] ? date('Y-m-d H:i', $value['ctime']) : '';

                $list[] = array(
                    'id' => $value['id'],
                    'eid' => $value['eid'],
                    'uid' => $value['uid'],
                    'name_n' => $value['name_n'],
                    'r_username' => $value['r_username'],
                    'jobid' => $value['jobid'],
                    'partname' => $value['partname'],
                    'part_url' => $value['part_url'],
                    'comid' => $value['comid'],
                    'comname' => $value['comname'],
                    'com_url' => $value['com_url'],
                    'ctime' => $value['ctime'],
                    'ctime_n_n' => $value['ctime_n_n'],
                    'status' => $value['status'],
                    'status_n' => $value['status_n'],
                );
            }
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }

    /**
     * 会员-企业-行为记录：兼职报名记录（删除）
     */
    function delpartapply_action()
    {
        $partM = $this->MODEL('part');
        if (is_array($_POST['del'])) {
            $id = $_POST['del'];
        } else {
            $id = intval($_POST['del']);
        }
        $arr = $partM->delPartApply($id);
        if ($arr['errcode'] == 9 || $arr['errcode'] === 0) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json($arr['errcode'], $arr['msg']);
        }
    }
//endregion
//region 会员-企业-行为记录：职位收藏记录
    /**
     * 会员-企业-行为记录：职位收藏记录
     */
    function favjob_action()
    {
        $jobM = $this->MODEL('job');
        if ($_POST['keyword']) {
            $keytype = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);
            if ($keytype == 1) {
                //搜企业
                $where['com_name'] = array('like', $keyword);
            } elseif ($keytype == 2) {
                //职位名称
                $where['job_name'] = array('like', $keyword);
            } elseif ($keytype == 3) {
                //个人用户名
                $userinfoM = $this->MODEL('userinfo');
                $muser = $userinfoM->getList(array('username' => array('like', $keyword), 'usertype' => 1), array('field' => 'uid'));
                if ($muser) {
                    $uids = array();
                    foreach ($muser as $v) {
                        $uids[] = $v['uid'];
                    }
                    $where['uid'] = array('in', pylode(',', $uids));
                }
            } elseif ($keytype == 4) {
                //个人姓名/手机号
                $resumeM = $this->MODEL('resume');
                $res['PHPYUNBTWSTART'] = '';
                $res['telphone'] = array('like', $keyword);
                $res['name'] = array('like', $keyword, 'OR');
                $res['PHPYUNBTWEND'] = '';
                $resume = $resumeM->getResumeList($res, array('field' => 'uid'));
                if ($resume) {
                    $uids = array();
                    foreach ($resume as $v) {
                        $uids[] = $v['uid'];
                    }
                    $where['uid'] = array('in', pylode(',', $uids));
                } else {
                    $where['uid'] = '';
                }
            }
        }
        //下载时间
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            $where['PHPYUNBTWSTART_A'] = '';
            $where['datetime'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
            $where['datetime'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
            $where['PHPYUNBTWEND_A'] = '';
        }
        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('fav_job', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = 'id';
            }
            $where['limit'] = $pages['limit'];
            $rows = $jobM->getFavJobList($where, array('datatype' => 'moreinfo', 'utype' => 'admin'));
            foreach ($rows as $value) {
                $list[] = array(
                    'id' => $value['id'],
                    'uid' => $value['uid'],
                    'username_n' => $value['username_n'],
                    'telphone' => $value['telphone'],
                    'telphone_url' => $value['telphone_url'],
                    'job_id' => $value['job_id'],
                    'job_name' => $value['job_name'],
                    'job_url' => $value['job_url'],
                    'com_id' => $value['com_id'],
                    'com_name' => $value['com_name'],
                    'com_url' => $value['com_url'],
                    'datetime' => $value['datetime'],
                    'datetime_n_n' => $value['datetime_n_n'],
                );
            }
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }

    /**
     * 会员-企业-行为记录：职位收藏记录（删除）
     */
    function delfavjob_action()
    {
        $id = $_POST['del'];
        $jobM = $this->MODEL('job');
        $return = $jobM->delFavJob($id, array('utype' => 'admin'));
        if ($return['errcode'] == 9 || $return['errcode'] === 0) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
//endregion
//region 会员-企业-行为记录：拨号记录
    function jobtellog_search_list_action(){
        include(CONFIG_PATH . 'db.data.php');

        $source = array();
        foreach ($arr_data['source'] as $sk => $sv) {
            if (in_array($sk, array('2', '3', '13', '19', '22'))) {
                $source[$sk] = $sv;
            }
        }
        $search_list = array();
        $search_list[] = array('param' => 'source', 'name' => '操作平台', 'value' => $source);
        $this->render_json(0, '', $search_list);
    }
    function jobtellog_action(){
        if ($_POST['keyword']) {
            $type = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);

            if ($type == 1) {
                //拨号人
                $resumeM = $this->MODEL('resume');
                $where['uid'] = array('like', $keyword);
                $resumelist = $resumeM->getResumeList(array('name' => array('like', $keyword)), array('field' => 'uid'));
                if ($resumelist) {
                    $uids = array();
                    foreach ($resumelist as $rv) {
                        $uids[] = $rv['uid'];
                    }
                    $where['uid'] = array('in', pylode(',', $uids));
                }
            } else if ($type == 2) {
                //职位名称
                $jobM = $this->MODEL('job');
                $joblist = $jobM->getList(array('name' => array('like', $keyword)), array('field' => 'id'));
                if ($joblist['list']) {
                    $jobids = array();
                    foreach ($joblist['list'] as $jv) {
                        $jobids[] = $jv['id'];
                    }
                    $where['jobid'] = array('in', pylode(',', $jobids));
                }
            } else if ($type == 3) {
                //所属企业
                $ComM = $this->MODEL('company');
                $listC = $ComM->getList(array('name' => array('like', $keyword)), array('field' => 'uid'));
                $minfo = $listC['list'];
                if ($minfo) {
                    $uids = array();
                    foreach ($minfo as $mv) {
                        $uids[] = $mv['uid'];
                    }
                    $where['comid'] = array('in', pylode(',', $uids));
                }
            } else if ($type == 4) {
                //拨号人ip
                $where['ip'] = array('like', $keyword);
            }
        }
        //操作平台
        if ($_POST['source']) {
            $where['source'] = intval($_POST['source']);
        }
        //操作时间
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            $where['PHPYUNBTWSTART_A'] = '';
            $where['ctime'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
            $where['ctime'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
            $where['PHPYUNBTWEND_A'] = '';
        }

        //提取分页
        $pageM = $this->MODEL('page');
        $jobM = $this->MODEL('job');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('job_tellog', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('id,desc');
            }
            $where['limit'] = $pageM->pageLimit($_POST);
            $list = $jobM->getTelLogs($where, array('utype' => 'admin'));
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }
    function deljobtellog_action(){
        if (is_array($_POST['del'])) {
            $id = pylode(',', $_POST['del']);
            $where = array('id' => array('in', $id));
        } else {
            $where = array('id' => intval($_POST['del']));
        }
        $jobM = $this->MODEL('job');
        $return = $jobM->delJobTelLog($where);
        if ($return['errcode'] == 9 || $return['errcode'] === 0) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
//endregion
}