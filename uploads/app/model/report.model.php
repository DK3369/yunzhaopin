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

class report_model extends model
{
    /**
     * @desc    引用log类，添加用户日志
     * @param   $uid
     * @param   $usertype
     * @param   $content
     * @param   string $opera
     * @param   string $type
     * @return void
     */
    private function addMemberLog($uid, $usertype, $content, $opera = '', $type = '')
    {

        require_once('log.model.php');
        $LogM   =   new log_model($this->db, $this->def);
        return $LogM->addMemberLog($uid, $usertype, $content, $opera, $type);
    }

    /**
     * 获取被举报职位列表
     * @param array $whereData
     * @param array $data
     * @return array
     */
    function getReportList($whereData = array(), $data = array())
    {

        $ListNew        =   array();
        $data['field']  =   empty($data['field']) ? '*' : $data['field'];
        $List           =   $this->select_all('report', $whereData, $data['field']);

        include PLUS_PATH . "/reason.cache.php";

        if (!empty($List)) {

            $uids   =   $eids   =   $jobids =   array();
            foreach ($List as $key => $val) {

                if (in_array($val['c_uid'], $uids) == false) {
                    $uids[] =   $val['c_uid'];
                }
                if (in_array($val['p_uid'], $uids) == false) {
                    $uids[] =   $val['p_uid'];
                }
                if ($val['type'] == 0){

                    if (in_array($val['eid'], $eids) == false && $val['usertype']==2) {

                        $eids[] =   $val['eid'];
                    }else if (in_array($val['eid'], $jobids) == false && $val['usertype']==1 ) {

                        $jobids[] =   $val['eid'];
                    }
                }else{

                    $eids[] =   $val['eid'];
                }
            }

            //获取手机号码
            $mWhere['uid']  =   array('in', pylode(',', $uids));
            $mdata['field'] =   '`uid`,`moblie`';
            $member         =   $this->getMemberList($mWhere, $mdata);

            //获取职位名称
            if ($data['type'] == 0) {

                if (isset($jobids) && !empty($jobids)){

                    $jWhere['id']   =   array('in', pylode(',', $jobids));
                    $job            =   $this->select_all('company_job', $jWhere, '`id`, `name`');
                    $u_user         =   $this->select_all('resume', array('uid' => array('in', pylode(',', $uids))), '`uid`, `name`');
                }elseif (isset($eids) && !empty($eids)){

                    //获取简历名称
                    $rWhere['id']   =   array('in', pylode(',', $eids));
                    $resume         =   $this->select_all('resume_expect', $rWhere, '`id`,`name`,`uname`');
                    $u_com          =   $this->select_all('company', array('uid' => array('in', pylode(',', $uids))), '`uid`, `name`');
                }
            }


            //获取question
            $qWhere['id']   =   array('in', pylode(',', $eids));
            $qdata['field'] =   '`id`,`title`';
            $question       =   $this->select_all('question', $qWhere, $qdata['field']);
            foreach ($List as $key => $val) {

                if ($data['utype'] == 'admin') {


                    $List[$key]['status_n']     =   $val['status'] == 0 ? '<span style="color: red;">未处理</span>' : '<span style="color: green;">已处理</span>';

                    if ($data['type'] == '0') {
                        if ($member && is_array($member)) {
                            foreach ($member as $v) {
                                if ($v['uid'] == $val['c_uid']) {

                                    $List[$key]['c_mobile'] =   $v['moblie'];
                                }
                                if ($v['uid'] == $val['p_uid']) {

                                    $List[$key]['p_mobile'] =   $v['moblie'];
                                }
                            }
                        }
                        if (isset($job) && is_array($job)) {
                            foreach ($job as $k=>$v) {
                                if ($v['id'] == $val['eid']) {

                                    $List[$key]['name']     =   $v['name'];
                                    $List[$key]['url']      =  Url('job', array('c' => 'comapply', 'look' => 'admin', 'id' => $val['eid']));
                                }
                            }
                            foreach ($u_user as $uk => $uv) {
                                if ($val['p_uid'] == $uv['uid']){
                                    $List[$key]['p_name']   =   $uv['name'];
                                }
                            }
                        }else if (isset($resume) && is_array($resume)) {
                            foreach ($resume as $k=>$v) {
                                if ($v['id'] == $val['eid']) {

                                    $List[$key]['name']     =   $v['name'] ? $v['name'] : $v['uname'];
                                    $List[$key]['url']      =   Url('resume', array('c' => 'show', 'look' => 'admin', 'id' => $val['eid']));
                                }
                            }
                            foreach ($u_com as $uk => $uv) {
                                if ($val['p_uid'] == $uv['uid']){
                                    $List[$key]['p_name']   =   $uv['name'];
                                }
                            }
                        }
                    }

                    if ($data['type'] == '1') {

                        $List[$key]['c']        =   "add";
                        $List[$key]['is_del']   =   '问题已被删除';

                        if (!empty($question)) {
                            foreach ($question as $qv) {
                                if ($qv['id'] == $val['eid']) {
                                    if ($qv['title'] != '') {

                                        $List[$key]['title']    =   $qv['title'];
                                        $List[$key]['url']      =   "index.php?m=admin_question&id=" . $val['eid'];
                                        $List[$key]['is_del']   =   '';
                                    }
                                }
                            }
                        }
                        if (isset($reason)){
                            foreach ($reason as $v) {
                                if ($val['r_reason'] == $v['id']) {

                                    $List[$key]['reason']       =   $v['name'];
                                } elseif ($val['r_reason'] == '0') {

                                    $List[$key]['reason']       =   '原因已被删除';
                                }
                            }
                        }
                    }

                    if ($data['type'] == '3') {

                        $rea                    =   @explode('@', $val['r_reason']);
                        $List[$key]['ereason']  =   $rea[0];
                        $List[$key]['rreason']  =   $rea[1];
                    }
                    $List[$key]['inputtime_n'] = date('Y-m-d H:i',$val['inputtime']);
                }
            }

            $ListNew['list'] = $List;
        }

        return $ListNew;
    }

    /**
     * @desc 添加report 举报列表数据
     * @param array $data
     * @return bool
     */
    public function addReport($data = array())
    {

        return $this->insert_into('report', $data);
    }

    /**
     * @desc 举报宣讲会
     * @param array $data
     * @return number|string
     */
    public function addSchoolReport($data = array())
    {

        $nid    =   $this->addReport($data);
        if ($nid) {

            require_once('admin.model.php');
            $adminM     =   new admin_model($this->db, $this->def);

            $thingtitle = strlen($data['r_name'])>8?mb_substr($data['r_name'],0,5,'utf-8').'...':$data['r_name'];
            $thing = '举报了《'.$thingtitle.'》的校招宣讲会';

            $adminM->sendAdminMsg(array('first' => '用户《'.$data['username'].'》举报了《'.$data['r_name'].'》的校招宣讲会。', 'type' => 27,'customer'=>$data['username'],'thing'=>$thing));
        }
        return $nid;
    }
    public function addChatReport($data = array()){
        $nid    =   $this->addReport($data);
        if ($nid) {

            $wxtempMsg = '用户《'.$data['username'].'》举报了聊天对象《'.$data['r_name'].'》';
            if($data['r_reason']){
                $wxtempMsg .= '，理由：'.$data['r_reason'];
            }
            require_once('admin.model.php');
            $adminM =   new admin_model($this->db, $this->def);
            $thingtitle = strlen($data['r_name'])>10?mb_substr($data['r_name'],0,7,'utf-8').'...':$data['r_name'];
            $thing = '举报了聊天对象《'.$thingtitle.'》';

            $adminM->sendAdminMsg(array('first' =>$wxtempMsg, 'type' => 32,'customer'=>$data['username'],'thing'=>$thing));
        }
        return $nid;
    }
    /**
     * @desc 投诉顾问
     * @param array $data
     * @return number|string
     */
    public function addCrmReport($data = array())
    {

        $nid    =   $this->addReport($data);
        if ($nid) {
            $wxtempMsg = '用户《'.$data['username'].'》举报了顾问《'.$data['r_name'].'》';
            if($data['r_reason']){
                $wxtempMsg .= '，理由：'.$data['r_reason'];
            }
            require_once('admin.model.php');
            $adminM =   new admin_model($this->db, $this->def);
            $thingtitle = strlen($data['r_name'])>12?mb_substr($data['r_name'],0,9,'utf-8').'...':$data['r_name'];
            $thing = '举报了顾问《'.$thingtitle.'》';

            $adminM->sendAdminMsg(array('first' =>$wxtempMsg, 'type' => 28,'customer'=>$data['username'],'thing'=>$thing));
        }
        return $nid;
    }

    /**
     * @desc 问答举报
     * @param array $data
     * @return number|string
     */
    public function addAskReport($data = array())
    {

        $nid    =   $this->addReport($data);
        if ($nid) {

            require_once('admin.model.php');
            $adminM =   new admin_model($this->db, $this->def);
            $ask    =   $this->select_once('question', array('id' => $data['eid']), '`title`');

            $wxtempMsg = '《'.$data['username'].'》举报了《'.$data['r_name'].'》的问答《'.$ask['title'].'》';

            if($data['r_reason']){
                $wxtempMsg .= '，理由：'.$data['r_reason'];
            }

            $thingtitle = strlen($ask['title'])>12?mb_substr($ask['title'],0,9,'utf-8').'...':$ask['title'];
            $thing = '举报了问答《'.$thingtitle.'》';

            $adminM->sendAdminMsg(array('first' =>$wxtempMsg, 'type' => 29,'customer'=>$data['username'],'thing'=>$thing));
        }
        return $nid;
    }

    /**
     * @desc 职位举报
     * @param array $data
     * @return number|string
     */
    public function addJobReport($data = array())
    {
        $nid    =   $this->addReport($data);
        if ($nid) {

            require_once('admin.model.php');
            $adminM =   new admin_model($this->db, $this->def);

            $job    =   $this->select_once('company_job', array('id' => $data['eid']), '`name`,`com_name`');

            $wxtempMsg = '《'.$data['username'].'》举报了《'.$data['r_name'].'》的职位《'.$job['name'].'》';
            
            if($data['r_reason']){
                $wxtempMsg .= '，理由：'.$data['r_reason'];
            }

            $thingtitle = strlen($job['name'])>12?mb_substr($job['name'],0,9,'utf-8').'...':$job['name'];
            $thing = '举报了职位《'.$thingtitle.'》';

            $adminM->sendAdminMsg(array('first' =>$wxtempMsg, 'type' => 30,'customer'=>$data['username'],'thing'=>$thing));
        }
        return $nid;
    }

    /**
     * @desc 简历举报
     * @param array $data
     * @return array
     */
    public function ReportResume($data = array())
    {

        $haves  =   $this->select_once('report', array('p_uid' => $data['p_uid'], 'c_uid' => $data['c_uid'], 'eid' => $data['eid'], 'usertype' => $data['usertype']));

        if (is_array($haves)) {

            $return =   array(
                'msg'       =>  '您已经举报过该用户！',
                'errcode'   =>  8
            );
        } else {
            if (is_array($data['reason'])) {

                $data['reason'] =   @implode(',', $data['reason']);
            }
            $datas  =   array(

                'c_uid'         =>  $data['c_uid'],
                'inputtime'     =>  $data['inputtime'],
                'p_uid'         =>  $data['p_uid'],
                'did'           =>  $data['did'],
                'usertype'      =>  $data['usertype'],
                'eid'           =>  $data['eid'],
                'r_name'        =>  $data['r_name'],
                'username'      =>  $data['username'],
                'r_reason'      =>  $data['reason'] // 提交的是汉字，不能用pylode
            );

            $nid    =   $this->addReport($datas);
            if ($nid) {

                $R_expect   =   $this->select_once('resume_expect', array('id' => $data['eid']), '`name`');
                require_once('admin.model.php');
                $adminM     =   new admin_model($this->db, $this->def);

                $wxtempMsg = '《'.$datas['username'].'》举报了《'.$datas['r_name'].'》的简历《'.$R_expect['name'].'》';
            
                if($datas['r_reason']){
                    $wxtempMsg .= '，理由：'.$datas['r_reason'];
                }

                $thingtitle = strlen($R_expect['name'])>12?mb_substr($R_expect['name'],0,9,'utf-8').'...':$R_expect['name'];
                $thing = '举报了简历《'.$thingtitle.'》';

                $adminM->sendAdminMsg(array('first' =>$wxtempMsg, 'type' => 3,'customer'=>$data['username'],'thing'=>$thing));
                $this->addMemberLog($data['p_uid'], $data['usertype'], "举报：简历".$data['r_name'], 23, 1);
                $return = array(
                    'msg'       =>  '举报成功！',
                    'errcode'   =>  9
                );
            } else {

                $return =   array(
                    'msg'       =>  '举报失败！',
                    'errcode'   =>  8
                );
            }
        }
        return $return;
    }

    /**
     * @param array $whereData
     * @param array $data
     * @return bool
     */
    function upReport($whereData = array(), $data = array())
    {

        return $this->update_once('report', $data, $whereData);
    }

    /**
     * @param array $whereData
     * @param array $data
     * @return array|bool|false|string|void
     */
    function getReportOne($whereData = array(), $data = array())
    {
        if ($whereData) {

            $data['field']  =   empty($data['field']) ? '*' : $data['field'];
            $List           =   $this->select_once('report', $whereData, $data['field']);
        }
        return $List;
    }

    /**
     * @desc 删除
     * @param $whereData
     * @param array $data
     * @return mixed
     */
    function delReport($whereData, $data = array())
    {

        if ($whereData['id']) {

            if (is_array($whereData['id'])) {

                $where['id']        =   array('in', pylode(',', $whereData['id']));
                $return['layertype']=   1;
                $del                =   pylode(',', $whereData['id']);
            } else {

                $return['layertype']=   0;
                $where['id']        =   $whereData['id'];
                $del                =   $whereData['id'];
            }

            if ($whereData['uid']) {
                $where['p_uid']     =   $whereData['uid'];
            }

            $return['id']           =   $this->delete_all('report', $where, '');

            $return['msg']          =   $data['title'] . '(ID:' . $del . ')';
            $return['errcode']      =   $return['id'] ? '9' : '8';
            $return['msg']          =   $return['id'] ? $return['msg'] . '删除成功！' : $return['msg'] . '删除失败！';

        } else {

            $return['msg']          =   '请选择您要删除的' . $data['title'] . '！';
            $return['layertype']    =   0;
            $return['errcode']      =   8;
        }

        return $return;
    }

    /**
     * 举报数量
     * @param array $Where
     * @return array|bool|false|string|void
     */
    public function getNum($Where = array())
    {
        return $this->select_num('report', $Where);
    }

    /**
     * @desc    引用userinfo类，查询member列表信息
     * @param   array $whereData
     * @param   array $data
     * @return  array|bool|false|string|void
     */
    private function getMemberList($whereData = array(), $data = array())
    {

        require_once('userinfo.model.php');
        $UserInfoM  =   new userinfo_model($this->db, $this->def);
        return $UserInfoM->getList($whereData, $data);
    }

    /**
     * @desc    引用job类，查询job列表信息
     * @param   array $whereData
     * @param   array $data
     * @return  array
     */
    private function getJobList($whereData = array(), $data = array())
    {

        require_once('job.model.php');
        $jobM   =   new job_model($this->db, $this->def);
        return $jobM->getList($whereData, $data);
    }

    /**
     * @desc    引用company类，查询company列表信息
     * @param   array $whereData
     * @param   array $data
     * @return  array
     */
    private function getComList($whereData = array(), $data = array())
    {

        require_once('company.model.php');
        $CompanyM   =   new company_model($this->db, $this->def);
        return $CompanyM->getList($whereData, $data);
    }
}

?>