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
class users_userlog_controller extends adminCommon
{
    //会员-个人-行为记录

//region 会员-个人-行为记录：简历被下载记录
    /**
     * 会员-个人-简历记录管理：简历下载记录（列表）
     */
    function down_action()
    {
        //从企业列表更多过来的
        if ($_POST['comid']) {
            $comid = intval($_POST['comid']);
            $where['comid'] = $comid;
            $where['usertype'] = 2;
        }
        if ($_POST['keyword']) {
            $keytype = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);
            if ($keytype == 1) {
                //搜企业
                $userinfoM = $this->MODEL('userinfo');
                $cw['username'] = array('like', $keyword);
                $cw['PHPYUNBTWSTART'] = '';
                $cw['usertype'][] = array('=', 2);
                $cw['usertype'][] = array('=', 3, 'OR');
                $cw['PHPYUNBTWEND'] = '';
                $mcom = $userinfoM->getList($cw, array('field' => 'uid'));
                if ($mcom) {
                    $uids = array();
                    foreach ($mcom as $v) {
                        $uids[] = $v['uid'];
                    }
                    $where['comid'] = array('in', pylode(',', $uids));
                } else {
                    $where['comid'] = "";
                }
            } elseif ($keytype == 2) {
                //公司
                $companyM = $this->MODEL('company');
                $com = $companyM->getList(array('name' => array('like', $keyword)), array('field' => 'uid'));
                $uids = array();
                if ($com) {
                    foreach ($com['list'] as $v) {
                        $uids[] = $v['uid'];
                    }
                }
                if (!empty($uids)) {
                    $where['comid'] = array('in', pylode(',', $uids));
                } else {
                    $where['comid'] = "";
                }
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
                } else {
                    $where['uid'] = "";
                }
            } elseif ($keytype == 4) {
                //姓名/手机号
                $nametel = $this->MODEL('resume');
                $nt['PHPYUNBTWSTART'] = '';
                $nt['telphone'] = array('like', $keyword);
                $nt['name'] = array('like', $keyword, 'OR');
                $nt['PHPYUNBTWEND'] = '';
                $nametels = $nametel->getResumeList($nt, array('field' => 'uid'));
                if ($nametels) {
                    $uid = array();
                    foreach ($nametels as $v) {
                        $uid[] = $v['uid'];
                    }
                    $where['uid'] = array('in', pylode(',', $uid));
                } else {
                    $where['uid'] = "";
                }
            }
        }
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            $where['PHPYUNBTWSTART_A'] = '';
            $where['downtime'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
            $where['downtime'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
            $where['PHPYUNBTWEND_A'] = '';
        }
        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('down_resume', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];

        $downM = $this->MODEL('downresume');
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
            $List = $downM->getList($where, array('utype' => 'admin'));
            $list = $List['list'];
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }
	
	/**
     * 免费下载记录
    */
    function freedown_action()
    {
        if ($_POST['keyword']) {
            $keytype = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);
            if ($keytype == 1) {
                //搜企业
                $userinfoM = $this->MODEL('userinfo');
                $cw['username'] = array('like', $keyword);
                $cw['PHPYUNBTWSTART'] = '';
                $cw['usertype'][] = array('=', 2);
                $cw['PHPYUNBTWEND'] = '';
                $mcom = $userinfoM->getList($cw, array('field' => 'uid'));
                if ($mcom) {
                    $uids = array();
                    foreach ($mcom as $v) {
                        $uids[] = $v['uid'];
                    }
                    $where['comid'] = array('in', pylode(',', $uids));
                } else {
                    $where['comid'] = "";
                }
            } elseif ($keytype == 2) {
                //公司
                $companyM = $this->MODEL('company');
                $com = $companyM->getList(array('name' => array('like', $keyword)), array('field' => 'uid'));
                $uids = array();
                if ($com) {
                    foreach ($com['list'] as $v) {
                        $uids[] = $v['uid'];
                    }
                }
                if (!empty($uids)) {
                    $where['comid'] = array('in', pylode(',', $uids));
                } else {
                    $where['comid'] = "";
                }
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
                } else {
                    $where['uid'] = "";
                }
            } elseif ($keytype == 4) {
                //姓名/手机号
                $nametel = $this->MODEL('resume');
                $nt['PHPYUNBTWSTART'] = '';
                $nt['telphone'] = array('like', $keyword);
                $nt['name'] = array('like', $keyword, 'OR');
                $nt['PHPYUNBTWEND'] = '';
                $nametels = $nametel->getResumeList($nt, array('field' => 'uid'));
                if ($nametels) {
                    $uid = array();
                    foreach ($nametels as $v) {
                        $uid[] = $v['uid'];
                    }
                    $where['uid'] = array('in', pylode(',', $uid));
                } else {
                    $where['uid'] = "";
                }
            }
        }
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            $where['PHPYUNBTWSTART_A'] = '';
            $where['downtime'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
            $where['downtime'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
            $where['PHPYUNBTWEND_A'] = '';
        }
        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('freedown_resume', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];

        $downM = $this->MODEL('downresume');
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
            $List = $downM->getFreeList($where, array('utype' => 'admin'));
            $list = $List['list'];
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }
	
	function delfreedown_action()
    {
        if ($_POST['id']) {
            $id = intval($_POST['id']);
        } elseif ($_POST['del']) {
            $id = $_POST['del'];
        }
        $downM = $this->MODEL('downresume');
        $return = $downM->delfreeInfo($id, array('utype' => 'admin'));
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
	
    /**
     * 会员-个人-简历记录管理：简历下载记录（删除）
     */
    function deldown_action()
    {
        if ($_POST['id']) {
            $id = intval($_POST['id']);
        } elseif ($_POST['del']) {
            $id = $_POST['del'];
        }
        $downM = $this->MODEL('downresume');
        $return = $downM->delInfo($id, array('utype' => 'admin'));
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
//endregion
//region 会员-个人-行为记录：简历推送记录
    /**
     * 会员-个人-行为记录：简历推送记录（列表）
     */
    function trust_action()
    {
        //从企业列表更多过来的
        if ($_POST['comid']) {
            $comid = intval($_POST['comid']);
            $where['comid'] = $comid;
        }
        if ($_POST['keyword']) {
            $keytype = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);
            if ($keytype == 1) {
                //简历名称
                $resumeM = $this->MODEL('resume');
                $expect = $resumeM->getSimpleList(array('name' => array('like', $keyword)), array('field' => 'id'));
                if ($expect) {
                    foreach ($expect as $v) {
                        $ids[] = $v['id'];
                    }
                    $where['eid'] = array('in', pylode(',', $ids));
                }
            } elseif ($keytype == 2) {
                //企业名称
                $companyM = $this->MODEL('company');
                $com = $companyM->getList(array('name' => array('like', $keyword)), array('field' => 'uid'));
                if ($com) {
                    foreach ($com['list'] as $v) {

                        $uids[] = $v['uid'];
                    }
                    $where['comid'] = array('in', pylode(',', $uids));
                }
            } elseif ($keytype == 3) {
                $jobM = $this->MODEL('job');
                $job = $jobM->getList(array('name' => array('like', $keyword)), array('field' => 'id'));
                if ($job) {
                    foreach ($job['list'] as $v) {
                        $ids[] = $v['id'];
                    }
                    $where['jobid'] = array('in', pylode(',', $ids));
                }
            }
        }
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            $where['PHPYUNBTWSTART_A'] = '';
            $where['ctime'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
            $where['ctime'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
            $where['PHPYUNBTWEND_B'] = '';
        }

        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('user_entrust_record', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];

        $userEntrustM = $this->MODEL('userEntrust');
        $list = array();
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            $where['orderby'] = 'id';
            $where['limit'] = $pages['limit'];
            $List = $userEntrustM->getRecordList($where, array('utype' => 'admin'));
            $list = $List['list'];
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }
    /**
     * 会员-个人-行为记录：简历推送记录（删除）
     */
    function deltrust_action()
    {
        if ($_POST['id']) {
            $id = intval($_POST['id']);
        } elseif ($_POST['del']) {
            $id = $_POST['del'];
        }
        $userEntrustM = $this->MODEL('userEntrust');
        $return = $userEntrustM->delRecord($id);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
//endregion
//region 会员-个人-行为记录：简历被浏览记录
    /**
     * 会员-个人-行为记录：简历被浏览记录（列表）
     */
    function lookresume_action()
    {
        //从企业列表更多过来的
        if ($_POST['comid']) {
            $comid = intval($_POST['comid']);
            $where['comid'] = $comid;
        }
        if ($_POST['keyword']) {
            $keytype = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);
            if ($keytype == 1) {
                //姓名
                $resumeM = $this->MODEL('resume');
                $resume = $resumeM->getResumeList(array('name' => array('like', $keyword)), array('field' => 'uid'));
                if ($resume) {
                    foreach ($resume as $v) {
                        $uids[] = $v['uid'];
                    }
                    $where['uid'] = array('in', pylode(',', $uids));
                }else{
                    $where['1'] = array('<>',1);
                }
            } elseif ($keytype == 2) {
                //简历名称
                $resumeM = $this->MODEL('resume');
                $expect = $resumeM->getSimpleList(array('name' => array('like', $keyword)), array('field' => 'id'));
                if ($expect) {
                    foreach ($expect as $v) {
                        $ids[] = $v['id'];
                    }
                    $where['resume_id'] = array('in', pylode(',', $ids));
                }else{
                    $where['1'] = array('<>',1);
                }
            } elseif ($keytype == 3) {
                //公司名称
                $companyM = $this->MODEL('company');
                $com = $companyM->getList(array('name' => array('like', $keyword)), array('field' => 'uid'));
              
                if ($com) {
                    foreach ($com['list'] as $v) {
                        $uids[] = $v['uid'];
                    }
                }
                $where['com_id'] = array('in', pylode(',', $uids));
            }
        }
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            $where['PHPYUNBTWSTART_A'] = '';
            $where['datetime'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
            $where['datetime'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
            $where['PHPYUNBTWEND_B'] = '';
        }
        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('look_resume', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];

        $lookresumeM = $this->MODEL('lookresume');
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
            $List = $lookresumeM->getList($where, array('utype' => 'admin'));
            $list = $List['list'];
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }
    /**
     * 会员-个人-行为记录：简历被浏览记录（删除）
     */
    function dellook_action()
    {
        if ($_POST['id']) {
            $id = intval($_POST['id']);
        } elseif ($_POST['del']) {
            $id = $_POST['del'];
        }
        $lookresumeM = $this->MODEL('lookresume');
        $return = $lookresumeM->delInfo(array('id' => $id));
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
//endregion
//region 会员-个人-行为记录：简历被收藏记录
    /**
     * @desc 收藏人才记录
     */
    function talentpool_action()
    {
        if (intval($_POST['comid'])) {
            $where['cuid'] = intval($_POST['comid']);
        }
        $ComM = $this->MODEL('company');
        $resumeM = $this->MODEL('resume');
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            $where['PHPYUNBTWSTART_A'] = '';
            $where['ctime'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
            $where['ctime'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
            $where['PHPYUNBTWEND_A'] = '';
        }
        $typeStr = intval($_POST['type']);
        $keywordStr = trim($_POST['keyword']);
        if (!empty($typeStr)) {
            if ($typeStr == 1) {
                $rwhere['name'] = array('like', $keywordStr);
                $rlist = $resumeM->getResumeList($rwhere, array('field' => '`uid`'));
                if (is_array($rlist)) {
                    foreach ($rlist as $v) {
                        $muids[] = $v['uid'];
                    }
                }
                $where['uid'] = array('in', pylode(',', $muids));
            } else if ($typeStr == 2) {
                $coms = $ComM->getList(array('name' => array('like', $keywordStr)), array('field' => '`uid`'));
                if (is_array($coms)) {
                    foreach ($coms['list'] as $v) {
                        $cuids[] = $v['uid'];
                    }
                }
                $where['cuid'] = array('in', pylode(',', $cuids));
            }
        }
        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('talent_pool', $where, $page, array('limit' => $pageSize));
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
            $list = $resumeM->getTalentList($where, array('utype' => 'admin'));
            $this->yunset(array('list' => $list));
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }

    /**
     * @desc 删除收藏人才记录
     */
    function deltalentpool_action()
    {
        $resumeM = $this->MODEL('resume');
        if (is_array($_POST['del'])) {
            $id = $_POST['del'];
        } else {
            $id = intval($_POST['id']);
        }
        $arr = $resumeM->delTalentPool($id, array('utype' => 'admin'));
        if ($arr['errcode'] == 9) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json($arr['errcode'], $arr['msg']);
        }
    }
//endregion
//region 会员-个人-行为记录：简历刷新记录
    public function sxLog_action()
    {
        $logM = $this->MODEL('log');
        $where = array();
        $keyStr = trim($_POST['keyword']);
        $typeStr = (int)$_POST['type'];
        if (!empty($keyStr)) {
            $rWhere = array();
            if ($typeStr == 1) {
                $rWhere['uname'] = array('like', $keyStr);
            } elseif ($typeStr == 2) {
                $rWhere['name'] = array('like', $keyStr);
            }
            $resumeS = $logM->getResumeBySxLog($rWhere, array('field' => 'id'));
            $resumeIds = array();
            foreach ($resumeS as $rk => $rv) {
                if (!in_array($rv['id'], $resumeIds)) {
                    $resumeIds[] = $rv['id'];
                }
            }
            $where['resume_id'] = array('in', pylode(',', $resumeIds));
        }
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            $where['PHPYUNBTWSTART_A'] = '';
            $where['r_time'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
            $where['r_time'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
            $where['PHPYUNBTWEND_A'] = '';
        }
        $pageM = $this->MODEL('page');
        //提取分页
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('resume_refresh_log', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = 'id,desc';
            }
            $where['limit'] = $pageM->pageLimit($_POST);
            $list = $logM->getSxResumeLogList($where, array('utype' => 'admin'));
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }
    //删除
    function delSxLog_action()
    {
        $LogM = $this->MODEL('log');
        if ($_POST['del']) {
            $id = $_POST['del'];
        } else {
            $id = intval($_POST['id']);
        }
        $arr = $LogM->delSxResumeLog($id, array('utype' => 'admin'));
        if ($arr['errcode'] == 9) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json($arr['errcode'], $arr['msg']);
        }
    }
//endregion
}