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
class job_model extends model{

    /**
     * @desc   添加用户日志
     */
    private function addMemberLog($uid, $usertype, $content, $opera = '', $type = '', $detail = '')
    {

        require_once('log.model.php');
        $LogM = new log_model($this->db, $this->def);
        return $LogM->addMemberLog($uid, $usertype, $content, $opera, $type, $detail);
    }

    /**
     * @desc   添加错误日志
     */
    private function addErrorLog($uid, $type = '', $content)
    {

        require_once('errlog.model.php');
        $ErrlogM    =   new errlog_model($this->db, $this->def);
        return $ErrlogM->addErrorLog($uid, $type, $content);
    }

    /**
     * @desc   添加系统消息
     */
    private function addSystem($data)
    {
        include_once('sysmsg.model.php');
        $sysmsgM    =   new sysmsg_model($this->db, $this->def);
        $sysmsgM->addInfo($data);
    }

    /**
     * @desc   职位详情，单条查询
     * @param array $where :职位查询条件
     * @param string[] $data （参数处理条件：add处理发布时相关信息；com ='yes'查询企业信息；hidecontac='yes' 处理联系方式；utype请求来源;）
     * @return array|bool|false|string|void
     */
    public function getInfo($where = array(), $data = array('add' => '', 'com' => '', 'hidecontac' => '', 'utype' => ''))
    {

        if (!empty($where)) {

            $select =   isset($data['field']) ? $data['field'] : '*';

            if (isset($where['com_id'])) {

                $where['uid']   =   $where['com_id'];
                unset($where['com_id']);
            }

            $Info   =   $this->select_once('company_job', $where, $select);

            if ($Info && is_array($Info)) {
                if(!empty($Info['x']) && !empty($Info['y'])){

                    $latlng = latlngCheck(array('lat'=>$Info['y'],'lng'=>$Info['x']));

                    $Info['x'] = $latlng['lng'];
                    $Info['y'] = $latlng['lat'];
                }
                if (!empty($Info['welfare'])) {

                    $Info['welfare']        =   str_replace(' ', '', $Info['welfare']);
                    $Info['arraywelfare']   =   array_filter(explode(',', trim($Info['welfare'])));
                }

                $Info['job_lastupdate']     =   lastupdateStyle($Info['lastupdate']);

                // 修改职位，语言要求判断使用
                if (!empty($data['add'])) {

                    $CacheList      =   $this->getClass(array('com'));
                    $Info['lang']   =   @explode(',', $Info['lang']);
                    if (!empty($Info['lang'])) {
                        foreach ($Info['lang'] as $k => $v) {
                            if (empty($v) || $v == 'undefined') {

                                unset($Info['lang'][$k]);
                            } else {

                                $langname[] =   $CacheList['comclass_name'][$v];
                            }
                        }
                    }
                    $Info['langname']   =   !empty($langname) ? $langname : array();
                }

                // 微信小程序处理描述内容
                if ($data['utype'] == 'wxapp') {
                    if (!empty($Info['description'])) {

                        $description = str_replace(array('&quot;', '&nbsp;', '<>'), array('', '', ''), $Info['description']);
                        $description = htmlspecialchars_decode($description);

                        preg_match_all('<img(.*?)src=\"(.+?)\".*?>', $description, $res);

                        if (!empty($res[2])) {
                            foreach ($res[2] as $v) {
                                if (strpos($v, 'https:') === false && strpos($v, 'http:') === false) {
                                    $imgurl = checkpic($v);
                                    $description = str_replace($v, $imgurl, $description);
                                }
                            }
                        }
                        $Info['description'] = $description;
                    }
                }

                // 查询企业信息
                /**
                 * @desc 查询企业信息，职位名称字段返回是 jobname
                 */
                if ($data['com'] == 'yes') {
                    // 查询企业申请信息
                    if (isset($data['reply'])){
                        $userjobwhere   =   array('com_id' => $Info['uid'], 'job_id' => $Info['id'], 'endtime' => array('<>', ''), 'isdel' => 9);
                        $userjoblist    =   $this->getSqJobList($userjobwhere, array('field' => '`uid`,`endtime`,`datetime`'));
                        
                        $totaltime      =   0; // 总时间
                        $i              =   0;
                        
                        if (is_array($userjoblist) && $userjoblist) {
                            foreach ($userjoblist as $val) {
                                
                                $surplustime=   $val['endtime'] - $val['datetime'];
                                $totaltime  =   $totaltime + $surplustime;
                                $i          =   $i + 1;
                            }
                            $Info['totaltime']  =   $totaltime;
                            $Info['totalnum']   =   $i;
                        }
                    }
                    $ComInfo        =   $this->getComInfo($Info['uid'], array('logo' => 1));

                    // 职位信息和企业信息整合
                    $Info           =   $this->getMixInfo($Info, $ComInfo);
					if ($data['link']) {
                        if ($Info['link_id']) {
                            $linkInfo = $this->select_once('company_job_link',array('id' =>$Info['link_id']), 'link_man,link_moblie,link_phone');
                            $Info['linkInfo'] = [
                                'linkman' => $linkInfo['link_man'],
                                'linktel' => $linkInfo['link_phone'],
                                'linkphone' => $linkInfo['link_moblie']
                            ];
                        } else {
                            $Info['linkInfo'] = [
                                'linkman' => $ComInfo['linkman'],
                                'linktel' => $ComInfo['linktel'],
                                'linkphone' => $ComInfo['linkphone']
                            ];
                        }
                    }
                }
                if (is_array($Info)) {
                    $Info           =   $this->getInfoArray($Info);
                }
                if ($Info['zp_num'] == 0) {

                    $Info['job_number'] =   "";
                } else {

                    $Info['job_number'] =   $Info['zp_num'] . " 人";
                }
                if ($Info['zp_minage'] && $Info['zp_maxage']) {
                    if ($Info['zp_minage'] == $Info['zp_maxage']) {

                        $Info['job_age']=   $Info['zp_minage'] . "周岁以上";
                    } else {

                        $Info['job_age']=   $Info['zp_minage'] . '-' . $Info['zp_maxage'] . "周岁";
                    }
                } elseif ($Info['zp_minage']) {

                    $Info['job_age']    =   $Info['zp_minage'] . "周岁以上";
                } elseif ($Info['zp_maxage']) {

                    $Info['job_age']    =   $Info['zp_maxage'] . "周岁以下";
                } else {

                    $Info['job_age']    =   "";
                }

                $Info['comqcode']       =   checkpic($Info['comqcode'], $this->config['sy_member_ewm']);
                $Info['com_logo_n']     =   checkpic($Info['com_logo'], $this->config['sy_unit_icon']);
                $Info['lastupdate']     =   lastupdateStyle($Info['lastupdate']);

                if (isset($Info['rec_time'])) {
                    $Info['job_rec']    =   $Info['rec_time'] > time() ? 1 : 0;
                }
                if (isset($Info['urgent_time'])) {
                    $Info['job_urgent'] =   $Info['urgent_time'] > time() ? 1 : 0;
                }

                return $Info;
            }
        }
    }
    function getJobMappic($data=array()){

        $jobid = $data['jobid'];

        $staticimg = '';

        if(!empty($jobid)){
            $Info   =   $this->select_once('company_job',array('id'=>$jobid),'`id`,`uid`,`x`,`y`,`link_id`');
            if(!empty($Info)){
                $x = $y = '';
                if(intval($Info['link_id'])>0){
                    $linkInfo       =   $this->select_once('company_job_link',array('id' =>$Info['link_id']), 'x,y,mappic');
                    $x = $linkInfo['x'];
                    $y = $linkInfo['y'];
                    if($linkInfo['mappic']){
                        $staticimg = checkpic($linkInfo['mappic']);
                    }else if($linkInfo['x'] && $linkInfo['y']){
                        $mdata['lng']      =   $linkInfo['x'];
                        $mdata['lat']      =   $linkInfo['y'];
                        $mdata['copyright'] =   1;
                        $mdata['width']     =   320;
                        $mdata['height']    =   140;
                        $mdata['zoom']      =   14;
                        $mdata['refer']    =   $this->config['sy_weburl'];
                        $mreturn = curlMappic($mdata);

                        if($mreturn['url']){

                            $staticimg = checkpic($mreturn['url']);
                            $this->update_once('company_job_link',array('mappic'=>$mreturn['url']),array('id' =>$Info['link_id']));
                        }
                    }
                }else{
                    $comInfo       =   $this->select_once('company',array('uid' =>$Info['uid']), 'uid,x,y,mappic');
                    $x = $comInfo['x'];
                    $y = $comInfo['y'];
                    if($comInfo['mappic']){
                        $staticimg = checkpic($comInfo['mappic']);
                    }else if($comInfo['x'] && $comInfo['y']){
                        $mdata['lng']      =   $comInfo['x'];
                        $mdata['lat']      =   $comInfo['y'];
                        $mdata['copyright'] =   1;
                        $mdata['width']     =   320;
                        $mdata['height']    =   140;
                        $mdata['zoom']      =   14;
                        $mdata['refer']    =   $this->config['sy_weburl'];
                        $mreturn = curlMappic($mdata);

                        if($mreturn['url']){

                            $staticimg = checkpic($mreturn['url']);
                            $this->update_once('company',array('mappic'=>$mreturn['url']),array('uid' =>$comInfo['uid']));
                        }
                    }
                }
                if($Info['x']!=$x || $Info['y']!=$y){
                    $this->update_once('company_job',array('x'=>$x,'y'=>$y),array('id' =>$jobid));
                }
            }
        }

        return $staticimg;
    }
    /**
     * @desc    获取职位联系方式
     * @param array $data | int $id 为职位的id ; int $uid 为登录的用户uid
     * @return array
     */
    public function getCompanyJobTel($data = array())
    {
        $res    =   array('errorcode' => 8, 'msg' => '');

        //判断参数
        $uid        =   intval($data['uid']);
        $usertype   =   intval($data['usertype']);
        $id         =   intval($data['id']);

        if (empty($id)) {
            $res['msg'] =   '参数错误';
            return $res;
        }

        $Info       =   $this->select_once('company_job',array('id' => $id), 'id,uid,is_link,rating,link_id');

        $comInfo    =   $this->select_once('company', array('uid' => $Info['uid']), 'uid, linkman, linktel, linkphone, linkmail, address, rating, infostatus, not_disturb,cityid,x,y');

        $Info['linkman']    =   $comInfo['linkman'];
        $Info['linktel']    =   $comInfo['linktel'];
        $Info['linkphone']  =   $comInfo['linkphone'];
        $Info['linkmail']   =   $comInfo['linkmail'];
        $Info['address']    =   $comInfo['address'];
        $Info['rating']     =   $comInfo['rating'];
        $Info['cityid']     =   $comInfo['cityid'];

        if (!empty($comInfo['x']) && !empty($comInfo['y'])){
            $latlng = latlngCheck(array('lat'=>$comInfo['y'],'lng'=>$comInfo['x']));
            $Info['x'] = $latlng['lng'];
            $Info['y'] = $latlng['lat'];
        }

        if (empty($Info)) {
            $res['msg'] =   '数据错误';
            return $res;
        }

        if(!empty($Info['link_id'])){
            $resData   =   $this->getContactNew($Info);
        }else{
            $resData   =   $Info;
        }

        if ($data['link_type'] == 'admin') {
            return $resData;
        }

        $resData['linkphone_n'] =   !empty($resData['linkphone']) ? substr_replace($resData['linkphone'], '****', 4, 4) : '';
        if (!CheckMobile($resData['linktel'])) {

            $resData['linktel_n'] = !empty($resData['linktel']) ? substr_replace($resData['linktel'], '****', 4, 4) : '';
        } else {

            $resData['linktel_n'] = $this->setContactHide($resData['linktel']);
        }

        // 根据此参数判断是否需要获取隐私号
        $isgetPrv               =   isset($data['isgetprv']) ? $data['isgetprv'] : 0;


        $data['id']             =   $id;
        $data['uid']            =   $uid;
        $data['usertype']       =   $usertype;

        $data['resData']        =   $resData;
        $data['com_id']         =   $Info['uid'];
        $data['infostatus']     =   $comInfo['infostatus'];
        $data['is_link']        =   $Info['is_link'];
        $data['rating']         =   $Info['rating'];

        // 根据后台设置、企业设置来处理联系方式
        $res                    =   $this->setCompanyLink($data);

        // 判断 企业是否开启免打扰模式
        if ($res['linkCode'] == 1 && $comInfo['infostatus'] == 1) {
            if($comInfo['not_disturb']){
                $notDisturb = explode('-', $comInfo['not_disturb']);
                if(is_array($notDisturb)) {
                    $startDate = strtotime($notDisturb[0]);
                    $endDate = strtotime($notDisturb[1]);

                    $now = time();
                    if($startDate > $endDate){
                        // 判断免打扰开始时间大于结束时间，例如：22:00-06:00 非06:00 - 22:00 可访问
                        if(!($startDate > $now && $now > $endDate)){
                            $res['linkMsg']             =   '企业已开启免打扰模式';
                            $res['linkCode']            =      2;
                        }
                    } else {
                        if($startDate < $now && $endDate > $now){
                            $res['linkMsg']             =   '企业已开启免打扰模式';
                            $res['linkCode']            =      2;
                        }
                    }
                }
            }
        }

        //开启隐私号，并且符合展示条件以及需要直接获取隐私号
        if ($res['linkCode'] == 10 && $isgetPrv == 1) {

            //查询求职者电话
            $resume     =   $this->select_once('resume', array('uid' => $uid), 'uid,telphone');

            // 绑定隐私号
            include_once('privacy.model.php');
            $privacyM   =   new privacy_model($this->db, $this->def);

            $priData['NumberA'] =   $resData['linktel'];
            $priData['NumberB'] =   $resume['telphone'];
            $priData['uid']     =   $uid;
            $priData['comid']   =   $Info['uid'];
            $priData['jobid']   =   $id;
            $priData['type']    =   2;  // 2-企业隐私号 1-为简历隐私号

            $prvReturn  =   $privacyM->setPrivacy($priData);
            $telphone   =   $prvReturn['middleNumber'];
            $prvtime    =   $prvReturn['prvtime'];

            if ($telphone) {

                $res['linkData']['prvlinktel']  =   $telphone;
                $res['linkData']['prvusertel']  =   $this->setContactHide($resume['telphone']);
                $res['linkData']['prvtime']     =   $prvtime;
                $res['linkCode']                =   10;
            } else {

                //隐私号获取失败时，给用户一个提示
                $res['linkMsg']             =   '企业未开放联系电话，请等待企业邀您面试！';
                $res['linkCode']            =      11;
            }
        }
        return $res;
    }

    /**
     * @param $data
     * @return array
     */
    function setCompanyLink($data)
    {

        $uid    =   $data['uid'];
        $id     =   $data['id'];

        if ($uid == $data['com_id']) {

            $res['linkData']    =   $data['resData'];
            $res['linkCode']    =   1;
            return $res;
        }

        if ($data['infostatus'] == 2){      //  企业不公开

            $res['linkMsg']     =   isset($this->config['sy_link_tips']) && !empty($this->config['sy_link_tips']) ? $this->config['sy_link_tips'] : '联系方式未公开';
            $res['linkCode']    =   2;
        }elseif ($data['is_link'] == 3){    //  职位隐藏联系方式

            $res['linkMsg']     =   isset($this->config['sy_link_tips']) && !empty($this->config['sy_link_tips']) ? $this->config['sy_link_tips'] : '联系方式未公开';
            $res['linkCode']    =   3;
        }else if(in_array($data['rating'], explode(',', $this->config['com_link_no'])) && $this->config['com_link_no']!=''){    //  后台设置特定会员类型屏蔽联系方式

            $res['linkMsg']     =   isset($this->config['sy_link_tips']) && !empty($this->config['sy_link_tips']) ? $this->config['sy_link_tips'] : '联系方式未公开';
            $res['linkCode']    =   4;
        }else if ($this->config['com_link_look'] == 1 || in_array($data['provider'], array('app','weixin','baidu','toutiao'))){

            if ($this->config['com_login_link'] == 2){  //  网站未公开企业联系方式

                $res['linkMsg']     =   isset($this->config['sy_link_tips']) && !empty($this->config['sy_link_tips']) ? $this->config['sy_link_tips'] : '联系方式未公开';
                $res['linkCode']    =   5;
            }elseif ($this->config['com_login_link'] == 3){ //  登录查看联系方式

                if ($data['usertype'] != 1){

                    $res['linkMsg'] =   '请登录个人账号';
                    $res['linkCode']=   6;
                }
            }elseif ($this->config['com_login_link'] == 4){ //  拥有简历查看联系方式
                if ($data['usertype'] != 1){

                    $res['linkMsg'] =   '请登录个人账号';
                    $res['linkCode']=   6;
                }else{

                    $resumeNum      =   $this->select_num('resume_expect', array('uid' => $data['uid']));
                    if ($resumeNum == 0){

                        $res['linkMsg'] =   '请先添加简历';
                        $res['linkCode']=   7;
                    }else{
                        $defResume      =   $this->select_once('resume_expect', array('uid' => $data['uid'], 'defaults' => 1), '`state`,`status`');
                        if ($defResume['state'] != 1) {

                            $res['linkMsg']     =   '简历未审核，无法查看联系方式';
                            $res['linkCode']    =   7;
                            $res['linkSub']     =   1;
                        }
                    }
                }
            } else if ($this->config['com_login_link'] == 5) {  //  投递简历查看联系方式

                if ($data['usertype'] != 1){

                    $res['linkMsg'] =   '请登录个人账号';
                    $res['linkCode']=   6;
                }else{
                    $sqNum = $this->select_num('userid_job', array('uid' => $uid, 'job_id' => $id,'is_browse'=>array('<>',6),'isdel'=>9));
                    $msNum = $this->select_num('userid_msg', array('uid' => $uid, 'jobid' => $id, 'isdel' => 9));

                    if ($msNum > 0 || $sqNum > 0) {
                        if ($this->config['sy_comprivacy_open'] == 1 && !in_array($data['rating'], explode(',', $this->config['sy_privacy_rating']))) {
                            $res['privacy']     = 1;
                            $res['linkMsg']     = '隐私号';
                            $res['linkCode']    = 10;
                        }
                    }else{
                        $res['linkMsg']     =   '请先投递简历';
                        $res['linkCode']    =   8;
                    }
                }
            }
        }else{  //  扫码查看

            $res['linkMsg']     =   '微信扫码查看';
            $res['linkCode']    =   9;
        }

        if (!empty($res['linkMsg'])){

            unset($data['resData']['linktel']);
            unset($data['resData']['linkphone']);
        }else{
            $res['linkCode']    =   1;
        }

        $res['linkData']        =   $data['resData'];

        /*if (in_array($data['rating'], explode(',', $this->config['com_link_no'])) && !empty($this->config['com_link_no'])) {    // 判断后台有没有设置特定会员等级屏蔽联系方式

            if ($data['utype'] == 'com') {

                $res['msg']     =   '企业暂未显示联系方式，详情请咨询网站客服：' . $this->config['sy_freewebtel'];
            } else {

                $res['msg']     =   '企业暂未显示联系方式，请直接投递简历，详情请咨询网站客服：' . $this->config['sy_freewebtel'];
            }
            $res['errorcode']   =   2;
            return $res;
        } else if ($this->config['com_login_link'] == 1) {  // 后台设置开放

            if ($data['utype'] == 'com') {

                if ($data['infostatus'] == "1") {

                    $res['data']    =   $data['resData'];
                } else {

                    $res['msg']     =   '企业暂未开启查看联系方式';
                    $res['errorcode']   = 1;
                    return $res;
                }
                $res['data']        =   $data['resData'];
            } else {

                if ($data['is_link'] == "1" || $data['is_link'] == "2") {

                    $res['data']    =   $data['resData'];
                } else {

                    $res['msg']     =   '企业暂未开启查看联系方式，请直接投递简历';
                    $res['errorcode']   =   1;
                    return $res;
                }
            }
        } elseif ($this->config['com_login_link'] == 2) {   // 后台设置不开放

            if ($data['utype'] == 'com') {

                $res['msg']     =   '企业暂未显示联系方式，详情请咨询网站客服：' . $this->config['sy_freewebtel'];
            } else {

                $res['msg']     =   '企业暂未显示联系方式，请直接投递简历，详情请咨询网站客服：' . $this->config['sy_freewebtel'];
            }
            $res['errorcode']   =   2;
            return $res;
        } elseif ($this->config['com_login_link'] == 3) {   // 后台设置登录后显示（登录针对个人）

            if (empty($uid)) {

                $res['msg']         =   '登录个人账号查看联系方式';
                $res['errorcode']   =   3;
                return $res;
            } else {

                if (!empty($data['usertype']) && $data['usertype'] != 1) {

                    $res['msg']         =   '只有个人用户才能查看';
                    $res['errorcode']   =   6;
                    return $res;
                } else {

                    $res['data']        =   $data['resData'];
                }
            }
        } elseif ($this->config['com_login_link'] == 4) {   // 后台设置拥有简历

            $resumenum  =   $this->select_num('resume_expect', array('uid' => $uid, 'job_classid' => array('<>', '')));

            if ($resumenum < 1 || $data['usertype'] != '1') {

                $res['msg']         =   '添加简历后查看联系方式';
                $res['errorcode']   =   4;
                return $res;
            } else {
                $tgresume   =   $this->select_num('resume_expect', array('uid' => $uid, 'state' => 1));

                if ($tgresume > 0) {

                    $openresume =   $this->select_num('resume_expect', array('uid' => $uid, 'status' => 1));

                    if ($openresume > 0) {

                        $res['data']    =   $data['resData'];
                    } else {

                        $res['msg']     =   '简历设置为公开才能查看联系方式';
                        $res['errorcode']   = 8;
                        return $res;
                    }
                } else {

                    $res['msg']         =   '简历通过审核才能查看联系方式';
                    $res['errorcode']   =   7;
                    return $res;
                }
            }
        } elseif ($this->config['com_login_link'] == 5) {   // 后台设置投递简历

            $uWhere['uid']      =   $uid;
            $uWhere['isdel']    =   9;
            if ($data['utype'] == 'com') {

                $uWhere['com_id']   =   $data['com_id'];
            } else {

                $uWhere['job_id']   =   $id;
            }

            $msgresume  =   $this->select_num('userid_msg', array('uid' => $uid, 'jobid' => $id, 'isdel' => 9));
            $sendresume =   $this->select_num('userid_job', $uWhere);

            if (($msgresume > 0 || $sendresume > 0) && $data['usertype'] == 1) {

                //如果开启隐私号，并且未设置会员隐私号关闭；就隐藏所有联系方式，只能通过隐私号函数调用
                if ($this->config['sy_comprivacy_open'] == 1 && !in_array($data['rating'], explode(',', $this->config['sy_privacy_rating']))) {

                    $data['resData']['linktel']     =   '';
                    $data['resData']['linkphone']   =   '';
                    $data['resData']['linkqq']      =   '';
                    $data['resData']['linktel_n']   =   '';
                    $data['resData']['linkphone_n'] =   '';
                    $data['resData']['privacy']     =   1;

                    $res['data']    =   $data['resData'];
                } else {

                    $res['data']    =   $data['resData'];
                }
            } else {

                $res['msg']         =   '投递简历才能查看联系方式';
                $res['errorcode']   =   5;
                return $res;
            }
        }*/
        return $res;
    }

    /**
     * 获取联系方式(新)
     * @param array $data
     * @return array
     */
    private function getContactNew($data = array())
    {

        $link   =   $this->select_once('company_job_link', array('id' => $data['link_id']), '`link_man`,`link_moblie`,`link_phone`,`link_address`,`cityid`,`x`,`y`');

        if (!empty($link['x']) && !empty($link['y'])){

            $latlng     =   latlngCheck(array('lat'=>$link['y'],'lng'=>$link['x']));
            $link['x']  =   $latlng['lng'];
            $link['y']  =   $latlng['lat'];
        }

        $return =   array();
        if ($data['is_link'] == 1) {    //默认联系方式

            $return['linkman']  =   $data['linkman'];
            $return['linkphone']=   $data['linkphone'];
            $return['linktel']  =   $data['linktel'];
            $return['address']  =   $data['address'];
            $return['cityid']   =   $data['cityid'];
            $return['x']        =   $data['x'];
            $return['y']        =   $data['y'];
        } elseif ($data['is_link'] == 2) {

            if (!empty($link)) {//新的联系方式

                $return['linkman']  =   $link['link_man'];
                $return['linktel']  =   $link['link_moblie'];
                $return['linkphone']=   $link['link_phone'];
                $return['address']  =   $link['link_address'];
                $return['cityid']   =   $link['cityid'];
                $return['x']        =   $link['x'];
                $return['y']        =   $link['y'];
            } else {//如果新的联系方式不存在，就显示默认联系方式

                $return['linkman']  =   $data['linkman'];
                $return['linkphone']=   $data['linkphone'];
                $return['linktel']  =   $data['linktel'];
                $return['address']  =   $data['address'];
                $return['cityid']   =   $data['cityid'];
                $return['x']        =   $data['x'];
                $return['y']        =   $data['y'];
            }
        } elseif ($data['is_link'] == 3 && !empty($link)) {

            $return['linkman']  =   $link['link_man'];
            $return['linktel']  =   $link['link_moblie'];
            $return['linkphone']=   $link['link_phone'];
            $return['address']  =   $link['link_address'];
            $return['cityid']   =   $link['cityid'];
            $return['x']        =   $link['x'];
            $return['y']        =   $link['y'];
        }
        return $return;
    }

    /**
     * @desc   获取职位列表
     * @param array $whereData :查询条件
     * @param array $data :自定义处理数组  (例：后台数据：utype->admin)
     * @return array
     */
    public function getList($whereData,$data=array('cache'=>'','utype'=>''))
    {

        $ListJob  =  array();

        $select   =  isset($data['field']) ? $data['field'] : '*';

        if (isset($whereData['com_id'])) {
            $whereData['uid']   =   $whereData['com_id'];
            unset($whereData['com_id']);
        }

        if ($data['top']) {
            $topWhere = $whereData;
            $topWhere['xsdate'] = array('>', time());
            //置顶设置为随机20条时，随机查询
            if($this->config['joblist_top'] != 1){
                $topWhere['orderby'] = 'rand()';
                $topWhere['limit'] = 20;
            } else {
                if(isset($topWhere['limit'])){
                    unset($topWhere['limit']);
                }
            }
            $topList = $this -> select_all('company_job', $topWhere, $select);
            if (!empty($topList)) {
                $topJobIdArr = array_column($topList, 'id');
                $whereData['id'] = array('notin', pylode(',', $topJobIdArr));
            }
        }

        $List     =  $this -> select_all('company_job',$whereData, $select);

        !empty($topList) && $data['top'] == 1 && $List = array_merge($topList, $List); // 合并置顶数据

        if (!empty($List)) {

            $time    =  time();
            $jobids  =  array();
            $comids = array();
            $linkIdArr = [];
            foreach ($List as $v) {
                if (!empty($v['id'])){
                    $jobids[]   =  $v['id'];
                    $comids[] = $v['uid'];
                }

                if (isset($data['link']) && $data['link'] == 'yes' && !empty($v['link_id'])) {
                    $linkIdArr[] = $v['link_id'];
                }
            }

            $members = $this->select_all('member', array('uid' => array('in', pylode(',', array_unique($comids)))), '`uid`,`lock_info`');
            $memberArr = array();
            foreach ($members as $mv) {
                $memberArr[$mv['uid']] = $mv;
            }
            $options=   array('job', 'hy', 'city', 'com');
            $cache  =   $this->getClass($options);
            if (isset($data['cache']) && $data['cache'] == '1') {

                $ListJob['cache'] = $cache;
            }

            if (isset($data['sqjobnum']) && $data['sqjobnum'] == 'yes' && !empty($jobids)) {

                $sqList     =   $this -> getSqJobList(array('job_id'=>array('in', pylode(',', $jobids)),'com_id'=>$whereData['uid'],'isdel'=>9), array('field'=>'`job_id`, `type`'));
            }
            //职位联系信息
            if (isset($data['link']) && $data['link'] == 'yes') {

                $company    =   $this -> getComInfo($whereData['uid'], array('field'=>"`linktel`, `linkphone`, `linkman`, `address`"));

                if (!empty($jobids)){
                    $joblink    =   $this -> getComJobLinkList(array(
                        'id' => array('in', pylode(',', $linkIdArr)),
                        'jobid'=>array('in', pylode(',', $jobids), 'OR') // 老业务逻辑会存在job_id，固此条件保留
                    ), array('field'=>'`id`,`uid`,`jobid`,`link_type`,`link_man`,`link_moblie`,`link_address`'));
                }
            }
            //获取企业简称
            if (!empty($comids)){
                $shortnameArr    =  $this->select_all('company' , array('uid' => array('in', pylode(',', $comids))),'uid,shortname,crm_uid');
                if (!empty($data['crm_user'])) {
                    $crmUidArr = array_unique(array_column($shortnameArr, 'crm_uid'));
                    if (count($crmUidArr) > 1 || (count($crmUidArr) == 1 && $crmUidArr[0] > 0)) {
                        $crmuserList = $this->select_all('admin_user', array('uid' => array('in', pylode(',', $crmUidArr))), '`uid`,`name`');
                        $crmuserList && $crmuserList = array_column($crmuserList, null, 'uid');
                    }
                }
            }

            /* 常用参数整理 */
            $todaystart  =  strtotime(date('Y-m-d',time()));
            $beforeYesterday    =   $todaystart - 86400 * 2;
            if (isset($data['simple']) && intval($data['simple']) == 1) {
                require_once('xcx.model.php');
                $xcxM = new xcx_model($this->db, $this->def);
            }
            foreach ($List  as  $k  =>  $v) {
                if ($data['utype']=='admin') {
                    $List[$k]['iszp'] = $v['status'] == 0;
                }
                $List[$k]['lock_info'] = $memberArr[$v['uid']]['lock_info'] ? $memberArr[$v['uid']]['lock_info'] : '';
                $List[$k]['wapjob_url'] = Url('wap',array('c'=>'job','a'=>'comapply','id'=>$v['id']));
                $List[$k]['wapcom_url'] = Url('wap',array('c'=>'company','a'=>'show','id'=>$v['uid']));
                if(isset($v['sdate'])){
                    if($v['sdate']>$beforeYesterday){
                        $List[$k]['newtime']  =  1;
                    }
                    $List[$k]['sdate_n']  =  date('Y-m-d H:i',$v['sdate']);
                }
                if(isset($v['com_logo'])){
                    $List[$k]['com_logo_n']  =  checkpic($v['com_logo'],$this->config['sy_unit_icon']);
                }
                if(isset($v['is_link']) && $v['is_link']=='1' && isset($company)){ // 默认联系方式

                    $List[$k]['link_man']	  =  $company['linkman'];
                    $List[$k]['link_moblie']  =  $company['linktel']?$company['linktel']:$company['linkphone'];
                    $List[$k]['address']	  =  $company['address'];

                }else if(isset($v['is_link']) && $v['is_link']=='2' && isset($joblink)){   // 新增联系方式

                    foreach($joblink as $jlkey => $jlval){
                        // 新逻辑 联系方式ID 保存在职位表  兼容老逻辑 职位ID保存在联系方式表 为了向下兼容，先通过link_id，没匹配到，再用jobid
                        if($v['link_id']==$jlval['id'] || $v['id']==$jlval['jobid']){
                            $List[$k]['link_man']	  =  $jlval['link_man'];
                            $List[$k]['link_moblie']  =  $jlval['link_moblie'];
                            $List[$k]['address']	  =  $jlval['link_address'];
                            if ($v['link_id']==$jlval['id']) { // 优先使用新逻辑数据 （匹配到旧逻辑数据允许继续循环）
                                break; // 匹配到终止循环
                            }
                        }
                    }

                }else if(isset($v['is_link']) && $v['is_link']=='3'){   //  不展示联系方式

                    $List[$k]['link_man']	  =  '';
                    $List[$k]['link_moblie']  =  '';
                    $List[$k]['address']	  =  '';
                }

                /* 手机站地图，显示职位地址信息，其他联系方式清空 */
                if(isset($data['from']) && $data['from'] == 'wap_map'){
                    $List[$k]['link_man']	  =  '';
                    $List[$k]['link_moblie']  =  '';
                }

                if(isset($v['autotime']) && $v['autotime'] > $time){
                    $List[$k]['auto_n']       =  1;
                    $List[$k]['autodate']     =  date('Y-m-d',$v['autotime']);
                }
                if(isset($v['xsdate']) && $v['xsdate'] > $time){
                    $List[$k]['xs']			  =  1;
                    $endDay                   =  ceil(($v['xsdate'] - strtotime(date('Y-m-d')) - 86400) / 86400);
                    $List[$k]['top_day']      =  $endDay - 1 > 0 ? $endDay : 0;
                    $List[$k]['top_time_n']   =  date('Y-m-d', $v['xsdate']);
                    $List[$k]['istop']   =  true;
                }else{
                    $List[$k]['istop']   =  false;
                }
                if(isset($v['rec']) && isset($v['rec_time']) && $v['rec'] == 1 && $v['rec_time'] > $time){
                    $List[$k]['rec_n']		  =  1;
                    $endDay                   =  ceil(($v['rec_time'] - strtotime(date('Y-m-d')) - 86400) / 86400);
                    $List[$k]['rec_day']      =  $endDay - 1 > 0 ? $endDay : 0;
                    $List[$k]['rec_time_n']   =  date('Y-m-d', $v['rec_time']);
                    $List[$k]['isrec']   =  true;
                }else{
                    $List[$k]['isrec']   =  false;
                }
                if(isset($v['urgent']) && isset($v['urgent_time']) && $v['urgent'] == 1 && $v['urgent_time'] > $time){
                    $List[$k]['urgent_n']	  =  1;
                    $endDay                   =  ceil(($v['urgent_time'] - strtotime(date('Y-m-d')) - 86400) / 86400);
                    $List[$k]['urgent_day']   =  $endDay - 1 > 0 ? $endDay : 0;
                    $List[$k]['urgent_time_n']=  date('Y-m-d', $v['urgent_time']);
                    $List[$k]['isurgent']   =  true;
                }else{
                    $List[$k]['isurgent']   =  false;
                }
                if (isset($v['autotime']) && $v['autotime'] > $time){

                    $endDay                 =   ceil(($v['autotime'] - strtotime(date('Y-m-d')) - 86400) / 86400);
                    $List[$k]['auto_day']   =   $endDay - 1 > 0 ? $endDay : 0;
                    $List[$k]['autotime_n'] =   date('Y-m-d', $v['autotime']);
                }
                if(isset($v['rewardpack']) && $v['rewardpack']==1){
                    $List[$k]['xstype']	      =	 1;
                }

                if(!empty($sqList)){

                    $List[$k]['jobnum']       =  0;

                    foreach($sqList as $val){

                        if($v['id'] == $val['job_id']){

                            $List[$k]['jobnum']  +=  1;
                            $List[$k]['type']    =  $val['type'];

                        }
                    }
                }else{
                    $List[$k]['jobnum']          =  0;
                    $List[$k]['type']            =  1;
                }
                // 点击量
                if(!empty($v['jobhits'])){
                    $List[$k]['job_hits']       =   $v['jobhits'];
                    if($v['jobhits']>10000){
                        $List[$k]['jobhits']    =   floatval(round(($v['jobhits']/10000),1))."w";
                    }
                }
                // 曝光量
                if(!empty($v['jobexpoure'])){
                    $List[$k]['job_expoure']    =   $v['jobexpoure'];
                    if($v['jobexpoure']>10000){
                        $List[$k]['jobexpoure'] =    floatval(round(($v['jobexpoure']/10000),1))."w";
                    }
                }
                if(!empty($v['hy'])){
                    $List[$k]['job_hy']          =	$cache['industry_name'][$v['hy']];
                }
                if(!empty($v['job1'])){
                    $List[$k]['job_one_n']       =	$cache['job_name'][$v['job1']];
                }
                if(!empty($v['job1_son'])){
                    $List[$k]['job_two_n']       =	$cache['job_name'][$v['job1_son']];
                }
                if(!empty($v['job_post'])){
                    $List[$k]['job_three_n']     =	$cache['job_name'][$v['job_post']];
                }
                if(!empty($v['provinceid'])){
                    $List[$k]['job_city_one']    =	$cache['city_name'][$v['provinceid']];
                    $List[$k]['citystr']         =  $cache['city_name'][$v['provinceid']];
                }
                if(!empty($v['cityid'])){
                    $List[$k]['job_city_two']    =	$cache['city_name'][$v['cityid']];
                    $List[$k]['citystr']         =  $cache['city_name'][$v['cityid']];
                }
                if(!empty($v['three_cityid'])){
                    $List[$k]['job_city_three']  =	$cache['city_name'][$v['three_cityid']];
                    $List[$k]['citystr']         =  $cache['city_name'][$v['three_cityid']];
                }

                if (!empty($v['job_post'])) {
                    $List[$k]['jobClassId'] = $v['job_post'];
                } elseif (!empty($v['job1_son'])) {
                    $List[$k]['jobClassId'] = $v['job1_son'];
                } elseif (!empty($v['job1'])) {
                    $List[$k]['jobClassId'] = $v['job1'];
                }
                $List[$k]['jobClassId_n'] = $cache['job_name'][$List[$k]['jobClassId']];
                if (!empty($v['three_cityid'])) {
                    $List[$k]['cityClassId'] = $v['three_cityid'];
                } elseif (!empty($v['cityid'])) {
                    $List[$k]['cityClassId'] = $v['cityid'];
                } elseif (!empty($v['provinceid'])) {
                    $List[$k]['cityClassId'] = $v['provinceid'];
                }
                $List[$k]['cityClassId_n'] = $cache['city_name'][$List[$k]['cityClassId']];


                if(!empty($v['zp_num'])){
                    $List[$k]['job_number']      =	$v['zp_num'];
                }
                if(!empty($v['exp'])){
                    $List[$k]['job_exp']         =	$cache['comclass_name'][$v['exp']];
                }else{
                    $List[$k]['job_exp']         =	'不限';
                }
                if(!empty($v['report'])){
                    $List[$k]['job_report']      =	$cache['comclass_name'][$v['report']];
                }
                if(!empty($v['sex'])){
                    $List[$k]['job_sex']         =	$cache['com_sex'][$v['sex']];
                }
                if(!empty($v['edu'])){
                    $List[$k]['job_edu']         =	$cache['comclass_name'][$v['edu']];
                }else{
                    $List[$k]['job_edu']         =	'不限';
                }
                if(!empty($v['marriage'])){
                    $List[$k]['job_marriage']    =	$cache['comclass_name'][$v['marriage']];
                }
                if(!empty($v['age'])){
                    $List[$k]['job_age']         =	$cache['comclass_name'][$v['age']];
                }else{
                    $List[$k]['job_age']         =	'不限';
                }
                if(!empty($v['pr'])){
                    $List[$k]['job_pr']          =	$cache['comclass_name'][$v['pr']];
                }
                if(!empty($v['mun'])){
                    $List[$k]['job_mun']         =	$cache['comclass_name'][$v['mun']];
                }
                if(!empty($v['lang'])){

                    $lang                        =  @explode(',',$v['lang']);

                    foreach($lang  as $key => $value){

                        $langinfo[]              =  $cache['comclass_name'][$value];

                    }

                    $List[$k]['lang_n']          =  @implode(',', $langinfo);
                }
                $List[$k]['job_salary']          =  salaryUnit($v['minsalary'], $v['maxsalary'], !empty($data['salary_unit']) ? intval($data['salary_unit']) : 0);

                if (isset($data['isurl']) && $data['isurl']=='yes') {
                    if (isset($data['utype']) && $data['utype']=='admin') {

                        $List[$k]['joburl']  =	Url('job', array('c' => 'comapply', 'id' => $v['id'], 'look' => 'admin'));
                    }else{
                        $List[$k]['joburl']  =	Url('job', array('c' => 'comapply', 'id' => $v['id']));
                        $List[$k]['wapurl']  =	Url('wap',array("c"=>"job",'a'=>'comapply',"id"=>$v['id']));
                    }
                    $comurl_arr = array('c' => 'show', 'id' => $v['uid']);
                    if ($data['utype'] == 'admin') {
                        $comurl_arr['look'] = 'admin';
                    }
                    $List[$k]['comurl']      =	Url('company', $comurl_arr);
                }
                if(!empty($v['lastupdate'])){
                    $beginToday     =   strtotime('today');//今天开始时间戳
                    $beginYesterday =   strtotime('yesterday');//昨天开始时间戳
                    if ($v['lastupdate'] > $beginYesterday && $v['lastupdate'] < $beginToday) {
                        $List[$k]['lastupdate_date'] = "昨天";
                        $List[$k]['lastupdate_n'] = "昨天";
                    } elseif ($v['lastupdate'] > $beginToday) {
                        $List[$k]['lastupdate_date'] = lastupdateStyle($v['lastupdate']);
                        $List[$k]['lastupdate_n'] = lastupdateStyle($v['lastupdate']);
                    } else {
                        $List[$k]['lastupdate_date'] = date("Y-m-d", $v['lastupdate']);
                        $List[$k]['lastupdate_n'] = date("Y-m-d", $v['lastupdate']);
                    }
                    $List[$k]['lastupdate_n_n'] = date("Y-m-d H:i", $v['lastupdate']);
                }
                if(!empty($v['welfare'])){
                    $v['welfare'] = str_replace(' ', '',$v['welfare']);
                    $List[$k]['welfare_n']  =   array_filter(@explode(',', trim($v['welfare'])));
                }
                if (isset($v['state'])){
                    if ($v['state'] == 0){
                        $List[$k]['state_n']  =  '审核中';
                    } elseif ($v['state'] == 1){
                        $List[$k]['state_n']  =  '已审核';
                    } elseif ($v['state'] == 3){
                        $List[$k]['state_n']  =  '未通过';
                    }
                }
                //招聘状态
                if ($v['state'] == 0) {
                    $status_n = '职位未审核';
                } else if ($v['state'] == 2) {
                    $status_n = '职位已过期';
                } else if ($v['state'] == 3) {
                    $status_n = '职位审核未通过';
                } else if ($v['r_status'] == 0) {
                    $status_n = '企业未审核';
                } else if ($v['r_status'] == 2) {
                    $status_n = '企业已锁定';
                } else if ($v['r_status'] == 3) {
                    $status_n = '企业审核未通过';
                } else if ($v['r_status'] == 4) {
                    $status_n = '企业已暂停';
                } else if ($v['status'] == 4) {
                    $status_n = '已下架';
                } else {
                    $status_n = '招聘中';
                }
                $List[$k]['status_n'] = $status_n;

                if (!empty($shortnameArr)){
                    foreach ($shortnameArr as $ke=>$va){
                        if($v['uid']==$va['uid']){
                            $List[$k]['shortname'] = $va['shortname'];
                            !empty($data['crm_user']) && $List[$k]['crm_salesman'] = !empty($crmuserList[$va['crm_uid']]) ? $crmuserList[$va['crm_uid']]['name'] : '';
                        }
                    }
                }
            }

            /* 小程序请求 更新曝光量 */
            if ((isset($data['utype']) && $data['utype'] == 'wxapp' && $data['member'] == 'user')){
                $this->upJobExpoure(array('jobexpoure' => array('+', 1)), array('id' => array('in', pylode(',', $jobids))));
            }
            if (isset($data['utype']) && ($data['utype']=='admin' || $data['utype'] == 'wxapp')) {

                //  后台处理企业状态、职位申请未查看简历、职位面试简历、会员等级名称等；
                $List   =   $this -> subJobList($List,$data);
            }

            if (isset($data['video']) && $data['video'] == 1 && $this->config['sy_video_open'] == 1){

                $List   =   $this->getVideo($List);
            }

            if (isset($data['reserve']) && $data['reserve'] == 1){
                $List   =   $this->subReserveJob($List);
            }

            //  职位管理列表是否展示代招推广服务
            //  ① 开启推广开关
            //  ② 推广职位类别不为空
            //  ③ 职位类别在推广类别内
            if (isset($data['recruit']) && $data['recruit'] == 1){
                if ($this->config['sy_recruit_tg'] == 1 && !empty($this->config['sy_recruit_job_classid'])){

                    $applyList      =   $this->select_all('recruit_apply', array('uid' => $whereData['uid']));
                    $applyJobIdArr  =   array();
                    foreach ($applyList as $ak => $av){
                        $applyJobIdArr[]    =   $av['jobid'];
                    }

                    $rctJobClass    =   explode(',', $this->config['sy_recruit_job_classid']);
                    foreach ($List as $k => $v){

                        $List[$k]['recruit']        =   0;  //  默认不可申请
                        if (in_array($v['job1'], $rctJobClass) || in_array($v['job1_son'], $rctJobClass) || in_array($v['job_post'], $rctJobClass)){
                            $List[$k]['recruit']    =   1;  //  可申请代招服务
                        }
                        if (in_array($v['id'], $applyJobIdArr)){
                            $List[$k]['recruit']    =   2;  //  已经申请代招服务
                        }
                    }
                }
            }

            $ListJob['list']    =   $List;
        }

        return $ListJob;

    }

	/**
     * @desc   邀请面试职位列表
     * @param array $whereData :查询条件
     * @param array $data :自定义处理数组  (例：后台数据：utype->admin)
     * @return array
     */
    public function getMsyqJobList($whereData,$data=array('cache'=>'','utype'=>''))
    {
        $ListJob = array();
        $select = isset($data['field']) ? $data['field'] : '*';
        if (isset($whereData['com_id'])) {
            $whereData['uid'] = $whereData['com_id'];
            unset($whereData['com_id']);
        }
        $List = $this->select_all('company_job',$whereData, $select);
        if (!empty($List)) {
            $comids = array();
            $linkids = array();
            foreach ($List as $v) {
                if (!empty($v['id'])){
                    $comids[] = $v['uid'];
                    $v['link_id'] > 0 && !in_array($v['link_id'], $linkids) && array_push($linkids, $v['link_id']);
                }
            }
            //职位联系信息
            $company = $this->getComInfo($whereData['uid'], array('field'=>"`linktel`, `linkphone`, `linkman`, `address`"));
            if (!empty($linkids)){
                $joblink = $this->getComJobLinkList(array('id'=>array('in', pylode(',', $linkids))), array('field'=>'`id`,`uid`,`link_man`,`link_moblie`,`link_address`'));
            }
            foreach ($List as $k => $v) {
                if(isset($v['link_id']) && $v['link_id'] > 0){
                    foreach($joblink as $val){
                        if($v['link_id'] == $val['id']){
                            $List[$k]['link_man'] = $val['link_man'];
                            $List[$k]['link_moblie'] = $val['link_moblie'];
                            $List[$k]['address'] = $val['link_address'];
                        }
                    }
                } else {// 默认联系方式
                    $List[$k]['link_man'] = $company['linkman'];
                    $List[$k]['link_moblie'] = $company['linktel'] ? $company['linktel'] : $company['linkphone'];
                    $List[$k]['address'] = $company['address'];
                }
            }
            $ListJob['list'] = $List;
        }
        return $ListJob;
    }

    /**
     * 获取职位原始数据,某些搜索只需id
    */
    public function getListId($where,$data=array()){
        $select   =  isset($data['field']) ? $data['field'] : '*';
        $List     =  $this -> select_all('company_job',$where, $select);
        return $List;
    }

    /**
     * @desc 职位列表视频
     * @param array $List
     * @return array
     */
    private function getVideo($List = array())
    {

        $comIdArr       =   array();
        foreach ($List as $k => $v) {
            if (!in_array($v['uid'], $comIdArr)) {
                $comIdArr[] = $v['uid'];
            }
        }
        $videoList  =   $this->select_all('video', array('uid' => array('in', pylode(",", $comIdArr)), 'display' => 1), '`uid`, `pic`, `vid`, `slogan`');


        if (!empty($videoList)){
            $vidArr =   array();
            foreach ($videoList as $vk => $vv) {
                $vidArr[]   =   $vv['vid'];
            }

            $videoInfo  =   $this->select_all('video_info', array('id' => array('in', pylode(',', $vidArr))), '`id`, `file_id`, `img`, `urls`');

            require_once(LIB_PATH.'videocloud.class.php');
            $videoCloud =   new video();

            foreach($videoList as $vk => $vv){
                foreach($videoInfo as $vlk => $vlv){
                    if($vv['vid'] == $vlv['id']){

                        if (empty($vlv['urls'])) {
                            $playauth = $videoCloud->getUrls($vlv['file_id']);
                        } else {
                            $playauth = unserialize($vlv['urls']);
                        }

                        foreach ($playauth as $pk => $pv) {
                            if (strpos($pv, '.mp4')){
                                $videoList[$vk]['playurl']  =   $pv;
                            }
                        }
                        $videoList[$vk]['file_id']   =   $vlv['file_id'];
                        $videoList[$vk]['img']       =   $vlv['img'];
                    }
                }
            }

            foreach ($List as $k => $v) {
                foreach ($videoList as $vk => $vv) {
                    if ($v['uid'] == $vv['uid']){

                        $List[$k]['playurl']    =   $vv['playurl'];
                        $List[$k]['vid']        =   $vv['vid'];
                        $List[$k]['file_id']    =   $vv['file_id'];
                        $List[$k]['slogan']     =   $vv['slogan'];
                        $List[$k]['video_pic']  =   !empty($vv['pic']) ? checkpic($vv['pic']) : $vv['img'];
                        unset($videoList[$vk]);
                    }
                }
            }
        }

        return $List;
    }
	function reserveInfo($where=array()){

		if(!empty($where)){
			$info  =   $this->select_once('reserve_refresh',$where);
		}
		return $info;
	}
    /**
     * 预约刷新记录查询
     */
    private function subReserveJob($List = array())
    {

        $jobIds =   array();
        foreach ($List as $v) {

            if ($v['is_reserve'] == 1){

                $jobIds[]   =   $v['id'];
            }
        }

        $reserveList        =   $this->select_all('reserve_refresh' , array('job_id' => array('in', pylode(',', $jobIds))));
        if (!empty($reserveList)){
            foreach ($List as $k => $v) {
                foreach ($reserveList as $rv) {

                    if ($v['id'] == $rv['job_id']){

                        $List[$k]['reserve_status']     =   $rv['status'];
                        $List[$k]['reserve_interval']   =   $rv['interval'];
                        $List[$k]['reserve_start']      =   date('Y-m-d H:i:s', $rv['start_time']);
                        $List[$k]['reserve_end']        =   $rv['end_time'] > 0 ? date('Y-m-d', $rv['end_time']) : '不限';

                        $List[$k]['s_time']             =   $rv['s_time'];
                        $List[$k]['e_time']             =   $rv['e_time'];

                        if ($rv['s_time'] && $rv['e_time']){
                            $List[$k]['sx_time_n']      =   $rv['s_time'].' - '.$rv['e_time'];
                        }else if ($rv['s_time'] && empty($rv['e_time'])){
                            $List[$k]['sx_time_n']      =   $rv['s_time']. ' - 24:00';
                        }else if (empty($rv['s_time']) && $rv['e_time']){
                            $List[$k]['sx_time_n']      =   '00:00 - '.$rv['e_time'];
                        }else{
                            $List[$k]['sx_time_n']      =   '不限';
                        }
                    }
                }
            }
        }

        return $List;
    }

    /**
     *  @desc   处理列表自定义查询数据
     */
    private function subJobList($List,$data = array()) {

        foreach ($List as $v) {

            $userids[]     =   $v['uid'];
            $jobids[]      =   $v['id'];
        }

        // 查询会员套餐缓存，提取会员等级名称
        include PLUS_PATH.'comrating.cache.php';

        //  查询会员审核状态
        $comWhere['uid']  =  array('in', pylode(',', $userids));
        $comList          =  $this->select_all('company',$comWhere, '`uid`,`r_status`,`hotstart`,`hottime`,`logo`,`fact_status`');

        if ($data['utype'] == 'wxapp'){
            if (isset($data['look_where']) && !empty($data['look_where'])){
                $lookJobList    =   $this->select_all('look_job', $data['look_where'], '`jobid`');
                $lookJobIdArr   =   array();
                if (!empty($lookJobList)){
                    foreach ($lookJobList as $lv){
                        $lookJobIdArr[] =   $lv['jobid'];
                    }
                }
                foreach ($List as $k => $v) {
                    $List[$k]['isLookEd'] = in_array($v['id'], $lookJobIdArr) ? 1 : 0;
                }
            }
            //  查询数据进行匹配提取
            foreach ($List as $k => $v){
                foreach ($comrat as $val){

                    if ($v['rating']    ==  $val['id']) {

                        $List[$k]['rating_logo'] = checkpic($val['com_pic']);
                    }
                }
                foreach ($comList as $val){
                    if($v['uid'] == $val['uid'] && $val['hotstart']<=time() && $val['hottime']>=time()){
                        $List[$k]['hotlogo']    = 1;
                    }
                    if ($v['uid'] == $val['uid']){
                        $List[$k]['comlogo']    =  checkpic($val['logo'],$this->config['sy_unit_icon']);
                        $List[$k]['fact_status']=  $val['fact_status'];
                    }
                }
            }
        }else {

            //  查询申请列表，提取看简历数据、未查看简历数据
            $sqJobWhere['job_id']       =   array('in',pylode(',', $jobids));
            $sqJobData['field']         =   '`job_id`,`is_browse`';
            $sqJobList                  =   $this->getSqJobList($sqJobWhere,$sqJobData);

            //  查询邀请面试，提取职位面试数据数量
            $yqmsWhere['jobid']         =   array('in',pylode(',', $jobids));
            $yqmsWhere['groupby']       =   'jobid';
            $yqmsData['field']          =   'count(id) as num,`jobid`';
            $yqmsList                   =   $this   ->  getYqmsList($yqmsWhere,$yqmsData);

            $taskList   =   $this->select_all('wxpub_twtask', array('cuid' => array('in', pylode(',', $userids)), 'type' => 1, 'status' => 0), '`jobid`');

            //  查询数据进行匹配提取
            foreach ($List  as  $k  =>  $v){
                foreach ($comrat as $val){

                    if ($v['rating']    ==  $val['id']) {

                        $List[$k]['rating_name']    =   $val['name'];
                        $List[$k]['comlogo']    =  checkpic($val['logo'],$this->config['sy_unit_icon']);
                    }
                }
                foreach ($comList as $val){

                    if ($v['uid']   ==  $val['uid']) {

                        $List[$k]['c_status']       =   $val['r_status'];
                        $List[$k]['fact_status']=  $val['fact_status'];
                    }
                }
                if (!empty($sqJobList)) {

                    $List[$k]['browseNum']  =   0;
					$List[$k]['snum']  =   0;
                    foreach ($sqJobList as $val){
                        if ($v['id']       ==  $val['job_id']) {
                            if ($val['is_browse'] == 1){
                                $List[$k]['browseNum']  +=   1;
                            }
							$List[$k]['snum']  += 1;
                        }
                    }
                }else{

                    $List[$k]['browseNum']          =   0;
					$List[$k]['snum']  =   0;
                }


                if (!empty($yqmsList)) {

                    $List[$k]['inviteNum']  =   0;

                    foreach ($yqmsList as $val){

                        if ($v['id']       ==  $val['job_id']) {

                            $List[$k]['inviteNum']  =   $val['num'];
                        }
                    }
                }else{

                    $List[$k]['inviteNum']  =   0;
                }
                if($v['xsdate']     >   0){

                    $List[$k]['xstime']     =   date('Y-m-d',$v['xsdate']);
                }
                if($v['rec_time']   >   0){

                    $List[$k]['recdate']    =   date('Y-m-d',$v['rec_time']);
                }
                if($v['urgent_time']    >   0){

                    $List[$k]['eurgent']    =   date('Y-m-d',$v['urgent_time']);
                }

                $List[$k]['tw_num'] =   0;
                foreach ($taskList as $tk => $tv){
                    if ($v['id'] == $tv['jobid']){

                        $List[$k]['tw_num']++;
                    }
                }
            }
        }

        return $List;

    }

    /**
     * @param string[] $data
     * @return array
     */
    public function addJobInfo($data = array('utype' => ''))
    {

        require_once('warning.model.php');
        $warningM       =   new warning_model($this->db, $this->def);

        $post   =   $data['post'];

        $id     =   $data['id'];
        $uid    =   intval($data['uid']);

        unset($post['com_id']);
        $spid   =   !empty($data['spid']) ? intval($data['spid']) : '';

        if ($post['name']) {
            // 新家需求
            $isExistWhere = array('uid' => $uid, 'name' => $post['name'],'status'=>0);
            $job    =   $this->select_once('company_job',$isExistWhere, '`id`');
            if ($job){
                if(empty($id)){
                    $return['msg']      =   '您已经发布了一个相同的岗位，请勿重复发布。';
                    $return['errcode']  =   8;
                    $return['url']  =   '';
                    return  $return;
                }
                if ($job['id'] != $id) {
                    $return['msg']      =   '您已经发布了一个相同的岗位，请勿重复发布。';
                    $return['errcode']  =   8;
                    $return['url']  =   '';
                    return  $return;
                }
            }
        } else {

            $oldJob         =   $this->select_once('company_job', array('uid' => $uid, 'id' => $id), '`name`');
            $post['name']   =   $oldJob['name'];
        }

        $com                =   $this->select_once('company', array('uid' => $uid), '`uid`,`name`, `r_status`,`logo`,`provinceid`,`pr`,`mun`,`x`,`y`,`did`,`yyzz_status`,`rating`,`crm_uid`');

        if ($data['utype'] == 'admin') {    //后台修改添加不需要审核
            if ($post['r_status'] == 1) {

                $post['state']  =   1;
            } else {

                $post['state']  =   0;
            }
        } else {

            //  查询企业认证是否认证成功
            $companycert    =   $this->select_once('company_cert', array('uid' => $uid, 'type' => 3), '`uid`,`type`,`status`');

            //  在企业用户设置里企业发布职位审核未开启的情况下，未审核和未通过的企业，发布职位默认是未审核的。
            if ($com['r_status'] != 1) {

                $post['state']      =   0;
            } else {
                $job_ms_rating = !empty($this->config['job_ms_rating']) ? explode(',', $this->config['job_ms_rating']) : array();
                if ($this->config['com_free_status'] == 1 && $companycert['status'] == 1) {

                    $post['state']  =   1;
                } elseif($job_ms_rating && in_array($com['rating'], $job_ms_rating)) {
                    $post['state']  =   1;
                } else {

                    $post['state']  =   $this->config['com_job_status'];
                }
            }

            if ($post['is_link'] > 1 && (int)$post['link_id'] == 0) {

                $return['msg']      =   '请选择工作地址！';
                $return['errcode']  =   8;
            }
        }

        if ((int)$post['link_id'] > 0 && $post['is_link'] == 1){
            $post['is_link']    =   2;
        }

		if (empty($com['name'])) {
            $return['msg']      =   '企业基本信息未完善';
            $return['errcode']  =   8;
            return $return;
        }
        if ($com) {

            $post['com_name']       =   $com['name'];
            $post['com_logo']       =   $com['logo'];
            $post['com_provinceid'] =   $com['provinceid'];
            $post['pr']             =   $com['pr'];
            $post['mun']            =   $com['mun'];
            $post['did']            =   $com['did'];
			$post['yyzz_status']    =   $com['yyzz_status'];
        }
        require_once('statis.model.php');
        $statisM    =   new statis_model($this->db, $this->def);

        $suid       =   $spid ? $spid : $uid;
        $statis     =   $statisM->vipOver($uid, 2);
        if ($data['utype'] != 'admin' && !$id && $statis['job_num'] == 0) {
            return array('msg'=>'套餐已用完，立即升级VIP','errcode' => 8);
        }
        if ($statis) {
            $post['rating']     =   $statis['rating'];
        }
        if (!$id) {

            $post['sdate']      =   time();
            $post['lastupdate'] =   time();
            $post['uid']        =   $uid;
            $post['did']        =   $post['did'] ? $post['did'] : $data['did'];
            $data['add_crm'] && $post['add_crm'] = $data['add_crm']; // 记录添加职位的业务员ID
            $data['add_crm'] && $post['source'] = 16;//业务员添加的来源都是crm录入
			$ip = fun_ip_get();
            $post['add_ip']   =   $ip;
            $post['ip_address'] =   getIpAddress($ip);
            if ($statis['addjobnum'] == '0'){
                return array('msg'=>'您的会员已到期','errcode' => 8);
            }else{

                // 套餐会员。发布职位数限制的是企业上架的职位数量，不是企业发布的职位数量。发布数量不限，超出的发布后是未上架状态 20220326
                $zpjobnum  = $this->select_num('company_job', array('uid'=>$uid, 'status'=>0));
                $zppartnum = $this->select_num('partjob', array('uid'=>$uid, 'status'=>0));
                $zpltnum   = $this->select_num('lt_job', array('uid'=>$uid, 'zp_status'=>0));
                $zpnum     = $zpjobnum + $zppartnum + $zpltnum;
                if ($zpnum >= $statis['job_num']){
                    $post['status']  =  1;
                }else{

                    $payDetail  =   '发布职位，消耗上架套餐数量：1';
                    $this->addStatisDetail(array('uid' => $uid, 'type' => 1, 'num' => 1, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                }


                $nid            =   $this->insert_into('company_job', $post);
                $return['id']   =   $nid;
                $msg            =   '职位发布';
                $type           =   '1';
            }
            if ($nid) {

                $warningM->warning(1, $uid);//预警提醒
            }
        } else {

            //  lastupdate是职位刷新时间，修改职位，不再改变职位刷新时间
            unset($post['lastupdate']);

            $where['id']    =   $id;
            $where['uid']   =   $uid;
            $nid            =   $this->update_once('company_job', $post, $where);

            $return['id']   =   $id;
            $msg            =   '职位更新';;
            $type           =   2;
        }

        require_once('log.model.php');
        $LogM   =   new log_model($this->db, $this->def);

        if ($nid) {

            $job_data  =  $this->select_once('company_job', array('id' =>$return['id']), '`name`,`com_name`');

            if (!empty($id)){
                // 修改其他表职位发布时间
                $this -> update_once('wxpub_twtask',array('jobname'=>$job_data['name'],'comname'=>$job_data['com_name']),array('jobid'=>$id));
                $this -> update_once('fav_job',array('job_name'=>$job_data['name']),array('job_id'=>$id));
                // 修改名企更新时间
                $this -> update_once('hotjob',array('lastupdate'=>time()),array('uid'=>$uid));
            }

            if ($data['utype'] != 'admin') {
                //内容检测
                require_once('concheck.model.php');
                $concheckM  =  new concheck_model($this->db,$this->def);

                $check_con =array();

                if(isset($post['name']) && $post['name']!=''){
                    $check_con['name'] = strip_tags($post['name']);
                }
                if(isset($post['description']) && $post['description']!=''){
                    $check_con['description'] = strip_tags($post['description']);
                }
                if(isset($post['welfare']) && $post['welfare']!=''){
                    $check_con['welfare'] = strip_tags($post['welfare']);
                }
                $check_data = array(
                    'type'      =>  'text',
                    'uid'       =>  $data['uid'],
                    'usertype'  =>  $data['usertype'],
                    'ctype'     =>  2,
                    'cid'       =>  $return['id']
                );
                if(isset($post['source'])){
                    $check_data['source'] = $post['source'];
                }

                $cresult = $concheckM->checkContent($check_con,$check_data);
                //内容检测end
                if($cresult['code']!=1){
                    $this->update_once('company_job',array('state'=>0),array('id' => $return['id']));
                }
            }

            $this->update_once('company', array('jobtime' => time(),'lastupdate'=>time()), array('uid' => $uid));

            if ($data['utype'] != 'admin') {

                if (isset($data['is_tblink']) && (int)$data['is_tblink'] == 1){

                    $this->update_once('company_job', array('link_id' => $post['link_id'], 'is_link' => $post['is_link'], 'provinceid' => $post['provinceid'], 'cityid' => $post['cityid'], 'three_cityid' => $post['three_cityid'], 'x' => $post['x'], 'y' => $post['y']), array('uid' => $uid));
                }

                $LogM->addMemberLog($data['uid'], $data['usertype'], $msg.'（ID：'.$return['id'].'，名称：'.$post['name'].'）', 1, $type);//会员日志
            }

            if ($post['state'] == 0) {

                require_once('admin.model.php');
                $adminM         =   new admin_model($this->db, $this->def);

                $thing_type = '更新';
                if (!$id) {
                    $thing_type = '发布';
                }
                $thingtitle = strlen($post['name'])>8?mb_substr($post['name'],0,5,'utf-8').'...':$post['name'];
                $thing = '职位《'.$thingtitle.'》'.$thing_type.'，等待审核';

                $adminM->sendAdminMsg(array('first' => '企业《'.$job_data['com_name'].'》'.$msg . '(ID:' . $return['id'] . ')' . "《" . $post['name'] . "》成功，等待审核。", 'type' => 7,'customer'=>$job_data['com_name'],'thing'=>$thing,'crm_uid'=>$com['crm_uid']));
                $return['msg']  =   $msg.'成功,等待审核';
            } else {

                $return['msg']  =   $msg.'成功';
            }

            $return['errcode']  =   9;

            $jobstate = $this->select_once('company_job', array('id' => $return['id']), 'state');
            $tplid = '';
            if ($jobstate['state'] == 0) {// 待审核
                $wxopenid = $this->select_once('member', array('uid' => $uid), 'wxopenid');
                if ($wxopenid['wxopenid']) {
                    $tplinfo = $this->select_once('xcx_sub_tpl', array('name' => 'job_audit_tplid'), 'tpl_id');
                    $tplid = $tplinfo['tpl_id'] ? $tplinfo['tpl_id'] : '';
                }
            }
            $return['tpl_id'] = $tplid;

            if ($data['utype'] != 'admin') {
                //违禁词预警
                if(!empty($job_data['name'])){
                    $bandata = array(
                        'bantype'   =>  'addjobname',
                        'uid'       =>  $uid,
                        'jobid'     =>  $return['id'],
                        'content'   =>  $job_data['name']
                    );
                    $warningM->checkBanWord($bandata);
                }
                if(isset($post['description']) && strip_tags($post['description'])!=''){
                    $bandata = array(
                        'bantype'   =>  'addjob',
                        'uid'       =>  $uid,
                        'jobid'     =>  $return['id'],
                        'jname'     =>  $job_data['name'],
                        'content'   =>  strip_tags($post['description'])
                    );
                    $warningM->checkBanWord($bandata);
                }
            }
        } else {

            $return['msg']      =   $msg.'失败';
            $this->addErrorLog($uid, 4, $return['msg']);
            $return['errcode']  =   8;
            $return['url']      =   $_SERVER['HTTP_REFERER'];
        }
        return $return;
    }

    /**
     * @desc 添加职位数据
     * @param $Data
     * @return bool $return 返回信息
     */
    public function addInfo($Data)
    {

        return $this->insert_into('company_job', $Data);
    }

    /**
     * @desc    更新数据
     *
     * @param array $where ： 查询条件
     * @param array $data ： 更新数据
     * @return bool
     */
    public function upInfo($data = array(), $where = array()){

        if (!empty($where)) {

            $ListA  =   $this -> getList($where, array('field' => 'uid'));
            $list   =   $ListA['list'];

            if ($list && is_array($list)) {

                $cuids  =   array();

                foreach ($list as $v) {

                    $cuids[] = $v['uid'];
                }
            }
            $nid    =   $this -> update_once('company_job', $data, $where);

            if ($nid && $data['lastupdate']) {

                $this->upComInfo($cuids, $where = array(), array('jobtime' => time(), 'lastupdate' => time()));
                $this->update_once('hot_job', array('lastupdate' => time()), array('uid' => array('in', pylode(',', $cuids))));
            }

            return $nid;
        }
    }

    /**
     * @desc  后台审核职位
     * @param string $id (1 | 1,2,3)
     * @param array $upData
     * @return array
     */
    public function statusJob($id, $upData = array())
    {
        $text   =   $upData['single'] ? '审核职位' : '职位批量审核';
        $ids    =   @explode(',', trim($id));
        $return =   array('msg' => '非法操作！', 'errcode' =>  8);
        if (!empty($id)) {

            $idstr  =   pylode(',', $ids);

            if (count($idstr) == 1 && isset($upData['lock_status']) && $upData['lock_status'] == 1) {

                $companyJob = $this->select_once('company_job', array('id' => $id), 'r_status,uid');

                // 单个职位审核时 已锁定的企业 可以直接审核 锁定企业账号变成 正常状态
                if ($companyJob && $companyJob['r_status'] == 2) {

                    $this->update_once('member', array('status' => $upData['lock_status']), array('uid' => $companyJob['uid']));
                    $this->update_once('member', array('status' => $upData['lock_status']), array('pid' => $companyJob['uid']));
                    $this->update_once('company', array('r_status' => $upData['lock_status']), array('uid' => $companyJob['uid']));
                    $this->update_once('company_job', array('r_status' => $upData['lock_status']), array('uid' => $companyJob['uid']));
                    $this->update_once('partjob', array('r_status' => $upData['lock_status']), array('uid' => $companyJob['uid']));
                    $this->update_once('school_xjh', array('r_status' => $upData['lock_status']), array('uid' => $companyJob['uid']));
                }
            }
            $upData     =   array(
                'state'     =>  intval($upData['state']),
                'statusbody'=>  trim($upData['statusbody'])
            );

            if ($upData['state'] == 1){

                $result     =   $this -> update_once('company_job', $upData, array('id' => array('in', $idstr),'r_status'=>1));
            }else{

                $result     =   $this -> update_once('company_job', $upData, array('id' => array('in', $idstr)));
            }

            if ($result) {

                if($upData['state'] == 1){

                    $msg = $uids = $notUids = array();

                    $state_n=   '已通过';
                    $body   =   '';

                    $jobList=   $this->select_all('company_job', array('id' => array('in', $idstr)), '`id`,`uid`,`name`,`r_status`,`is_subscribe`');
                    $jobs['list']   =   $jobList;

                    foreach ($jobs['list'] as $v) {
                        if ($v['r_status'] != 1) {

                            $notUids[]  =   $v['uid'];
                        } else {

                            $uids[]     =   $v['uid'];
                        }
                    }

                    require_once 'notice.model.php';
                    $noticeM=   new notice_model($this->db, $this->def);

                    require_once 'push.model.php';
                    $pushM  =   new push_model($this->db, $this->def);

                    require_once 'weixin.model.php';
                    $wxM    =   new weixin_model($this->db, $this->def);

                    $member =   $this->getUserList(array('uid' => array('in', pylode(',', $uids))), array('field' => '`uid`,`email`,`moblie`,`wxopenid`,`usertype`'));

                    foreach ($jobs['list'] as $k => $v){
                        if($v['r_status'] != 1){
                            break;
                        }

                        $msg[$v['uid']][]   =  '您的职位<a href="comjobtpl,'.$v['id'].'">《'.$v['name'].'》</a>审核通过';
                        foreach ($member as $mv){

                            $sendData   =   $pushData   =   array();
                            if ($v['uid'] == $mv['uid']) {
                                $pushData['fuid']           =   '';
                                $pushData['puser']          =   $v['uid'];
                                $pushData['tid']            =   $v['id'];
                                $pushData['jobname']        =   $v['name'];
                                $pushData['state']          =   $upData['state'];

                                //APP 推送
                                $push   =   $pushM->pushMsg('jobState', $pushData, $push);
                                $needSend = 1;// 默认需要发送通知
                                if ($v['is_subscribe'] == 1 && $mv['wxopenid']) {// 订阅消息
                                    /*
                                     * 职位名称 {{thing1.DATA}}
                                     * 审核时间 {{time3.DATA}}
                                     * 审核状态 {{phrase4.DATA}}
                                     */
                                    $subscribeTpl = array(
                                        "thing1" => array("value" => $v['name']),// 职位名称
                                        "time3" => array("value" => date('Y-m-d H:i')),// 审核时间
                                        "phrase4" => array("value"  => '审核通过'),// 审核状态
                                    );
                                    $sendSubR = $wxM->sendWxSubscribe(array('uid' => $v['uid'],'wxid' => $mv['wxopenid'], 'url' => 'pson/pages/commember/job/index', 'tpl_data' => $subscribeTpl, 'tpl_type' => 4, 'utype' => $mv['usertype']));
                                    if ($sendSubR['errcode'] == 0) {// 订阅消息发送成功，无需发送其他通知
                                        $needSend = 0;
                                    }
                                    $this->update_once('company_job', array('is_subscribe' => 0), array('id' => $v['id']));
                                }
                                if ($needSend == 1) {
                                    $wxData['jobid']            =   $v['id'];
                                    $wxData['statusbody']       =   $upData['statusbody'];
                                    $wxData['state']            =   $upData['state'];
                                    $res = $wxM->sendWxJobStatus($wxData);
                                    if (!$res || $this->config['wx_xxtz']!='1' || !$this->config['wx_mbzwsh'] || $this->config['sy_msg_tz'] == 1){
                                        $sendData['type']           =   'zzshtg';
                                        $sendData['uid']            =   $v['uid'];
                                        $sendData['email']          =   $mv['email'];
                                        $sendData['moblie']         =   $mv['moblie'];
                                        $sendData['jobname']        =   $v['name'];
                                        $sendData['date']           =   date('Y-m-d H:i:s');
                                        $sendData['status_info']    =   $upData['statusbody'];
                                        //邮箱短信通知
                                        $noticeM->sendEmailType($sendData);
                                        $sendData['port']           =   '5';
                                        $noticeM->sendSMSType($sendData);
                                    }
                                }
                            }
                        }
                    }

                    //发送系统通知
                    require_once 'sysmsg.model.php';
                    $sysmsgM = new sysmsg_model($this->db, $this->def);
                    $sysmsgM->addInfo(array('uid' => $uids, 'usertype' => 2, 'content' => $msg));
                }else if($upData['state'] == 3){

                    $msg = $uids = array();

                    $state_n        =   '未通过';
                    $body           =   !empty($upData['statusbody']) ? '。 原因：'.$upData['statusbody'] : '';

                    $jobList        =   $this->select_all('company_job', array('id' => array('in', $idstr)), '`id`,`uid`,`name`,`is_subscribe`');
                    $jobs['list']   =   $jobList;

                    foreach ($jobs['list'] as $v){
                        $uids[] =   $v['uid'];
                    }

                    require_once 'notice.model.php';
                    $noticeM    =   new notice_model($this->db, $this->def);

                    require_once 'push.model.php';
                    $pushM      =   new push_model($this->db, $this->def);

                    require_once 'weixin.model.php';
                    $wxM      =   new weixin_model($this->db, $this->def);

                    $member     =   $this -> getUserList(array('uid' => array('in', pylode(',', $uids))), array('field' => '`uid`,`email`,`moblie`,`wxopenid`,`usertype`'));

                    foreach ($jobs['list'] as $k => $v){

                        $msg[$v['uid']][]   =   '您的职位<a href="comjobtpl,'.$v['id'].'">《'.$v['name'].'》</a>审核未通过'.$body;

                        foreach ($member as $mv){

                            $sendData   =   $pushData   =   array();
                            if ($v['uid'] == $mv['uid']) {
                                $pushData['fuid']       =   '';
                                $pushData['puser']      =   $v['uid'];
                                $pushData['tid']        =   $v['id'];
                                $pushData['jobname']    =   $v['name'];
                                $pushData['state']      =   $upData['state'];

                                //APP 推送
                                $push = $pushM->pushMsg('jobState', $pushData, $push);
                                $needSend = 1;// 默认需要发送通知
                                if ($v['is_subscribe'] == 1 && $mv['wxopenid']) {// 订阅消息
                                    /*
                                     * 职位名称 {{thing1.DATA}}
                                     * 审核时间 {{time3.DATA}}
                                     * 审核状态 {{phrase4.DATA}}
                                     */
                                    $subscribeTpl = array(
                                        "thing1" => array("value" => $v['name']),// 职位名称
                                        "time3" => array("value" => date('Y-m-d H:i')),// 审核时间
                                        "phrase4" => array("value"  => '审核未通过'),// 审核状态
                                    );
                                    $sendSubR = $wxM->sendWxSubscribe(array('uid' => $v['uid'],'wxid' => $mv['wxopenid'], 'url' => 'pson/pages/commember/job/index', 'tpl_data' => $subscribeTpl, 'tpl_type' => 4, 'utype' => $mv['usertype']));
                                    if ($sendSubR['errcode'] == 0) {// 订阅消息发送成功，无需发送其他通知
                                        $needSend = 0;
                                    }
                                    $this->update_once('company_job', array('is_subscribe' => 0), array('id' => $v['id']));
                                }
                                if ($needSend == 1) {
                                    $wxData['jobid']            =   $v['id'];
                                    $wxData['statusbody']       =   $upData['statusbody'];
                                    $wxData['state']            =   $upData['state'];
                                    $res = $wxM->sendWxJobStatus($wxData);
                                    if (!$res || $this->config['wx_xxtz']!='1' || !$this->config['wx_mbzwsh'] || $this->config['sy_msg_tz'] == 1){
                                        $sendData['type']       =   'zzshwtg';
                                        $sendData['uid']        =   $v['uid'];
                                        $sendData['email']      =   $mv['email'];
                                        $sendData['moblie']     =   $mv['moblie'];
                                        $sendData['jobname']    =   $v['name'];
                                        $sendData['date']       =   date('Y-m-d H:i:s');
                                        $sendData['status_info']=   $upData['statusbody'];
                                        //邮箱短信通知
                                        $noticeM->sendEmailType($sendData);
                                        $sendData['port']       =   '5';
                                        $noticeM->sendSMSType($sendData);
                                    }
                                }
                            }
                        }
                    }

                    //发送系统通知
                    require_once 'sysmsg.model.php';
                    $sysmsgM = new sysmsg_model($this->db, $this->def);
                    $sysmsgM->addInfo(array('uid' => $uids, 'usertype' => 2, 'content' => $msg));
                }

                if (count($notUids) > 0) {
                    $return['msg']  =   $text.$state_n.'成功'.count($uids).'条，失败'.count($notUids).'条。原因:企业账户未审核';
                } else {
                    $return['msg']  =   $text.$state_n.'成功(ID:'.$idstr.$body.')';
                }
                $return['errcode']  =   9;
            }else{

                $return['msg']      =   '审核职位设置失败(ID:'.$idstr.')';
                $return['errcode']  =   8;
            }
        }else {
            $return['msg']          =   '请选择需要审核的职位操作！';
            $return['errcode']      =   8;
        }

        return $return;
    }

    /**
     * @desc 职位审核，企业不是已审核状态，弹出同步操作状态审核
     * @param int $id
     * @param array $data|state statusbody
     */
    public function status($id, $data = array()){

        if (!$id){

            $return     =   array(
                'errcode' => 8,
                'msg'     => '参数错误！'
            );
            return $return;
        }else{

            $job        =   $this->getInfo(array('id' => $id), array('field' => '`id`,`uid`,`name`,`is_subscribe`'));

            $upData     =   array(

                'state'     =>  intval($data['state']),
                'statusbody'=>  trim($data['statusbody'])
            );

            $uid        =   $data['uid'];

            $result     =   $this -> update_once('company_job', $upData, array('id' => $id, 'uid' => $uid));

            if ($result) {

                if ($data['state'] == '1') {
                    $state_n = '已通过';
                    $body    = '';
                    $msg     = '您的职位<a href="comjobtpl,'.$id.'">《'.$job['name'].'》</a>审核通过';

                    require_once 'userinfo.model.php';
                    $userinfoM  =   new userinfo_model($this->db, $this->def);

                    $post   =   array(
                        'id'        =>  $id,
                        'status'    =>  1
                    );
                    $userinfoM -> status(array('uid' => $uid, 'usertype' => 2), array('post' => $post));
                }else{
                    $state_n = '未通过';
                    $body    = '。原因：'.$data['statusbody'];
                    $msg     = '您的职位<a href="comjobtpl,'.$id.'">《'.$job['name'].'》</a>审核未通过；原因：'.$data['statusbody'];
                }

                //发送系统通知
                require_once 'sysmsg.model.php';
                $sysmsgM    =   new sysmsg_model($this->db, $this->def);
                $sysmsgM -> addInfo(array('uid' => $uid,'usertype'=>2,'content'=>$msg));

                require_once 'notice.model.php';
                $noticeM    =   new notice_model($this->db, $this->def);

                require_once 'push.model.php';
                $pushM      =   new push_model($this->db, $this->def);

                require_once 'weixin.model.php';
                $wxM      =   new weixin_model($this->db, $this->def);

                $member     =   $this->select_once('member', array('uid' => $uid), '`uid`,`email`,`moblie`,`wxopenid`,`usertype`');
                $sendData   =   $pushData   =   array();

                if (!empty($member)) {
                    $pushData['fuid']           =   '';
                    $pushData['puser']          =   $uid;
                    $pushData['tid']            =   $id;
                    $pushData['jobname']        =   $job['name'];
                    $pushData['state']          =   $data['state'];
                    //APP 推送
                    $pushM -> pushMsg('jobState', $pushData);
                    $needSend = 1;// 默认需要发送通知
                    if ($job['is_subscribe'] == 1 && $member['wxopenid']) {// 订阅消息
                        /*
                         * 职位名称 {{thing1.DATA}}
                         * 审核时间 {{time3.DATA}}
                         * 审核状态 {{phrase4.DATA}}
                         */
                        $subscribeTpl = array(
                            "thing1" => array("value" => $job['name']),// 职位名称
                            "time3" => array("value" => date('Y-m-d H:i')),// 审核时间
                            "phrase4" => array("value"  => $data['state'] == '1' ? '审核通过' : '审核未通过'),// 审核状态
                        );
                        $sendSubR = $wxM->sendWxSubscribe(array('uid' => $member['uid'],'wxid' => $member['wxopenid'], 'url' => 'pson/pages/commember/job/index', 'tpl_data' => $subscribeTpl, 'tpl_type' => 4, 'utype' => $member['usertype']));
                        if ($sendSubR['errcode'] == 0) {// 订阅消息发送成功，无需发送其他通知
                            $needSend = 0;
                        }
                        $this->update_once('company_job', array('is_subscribe' => 0), array('id' => $job['id']));
                    }
                    if ($needSend == 1) {
                        $wxData['jobid']            =   $job['id'];
                        $wxData['statusbody']       =   $upData['statusbody'];
                        $wxData['state']            =   $upData['state'];
                        $res = $wxM->sendWxJobStatus($wxData);
                        if (!$res || $this->config['wx_xxtz']!='1' || !$this->config['wx_mbzwsh'] || $this->config['sy_msg_tz'] == 1){
                            $sendData['type']           =    $data['state'] == 3 ? 'zzshwtg' : 'zzshtg';
                            $sendData['uid']            =    $uid;
                            $sendData['email']          =    $member['email'];
                            $sendData['moblie']         =    $member['moblie'];
                            $sendData['jobname']        =    $job['name'];
                            $sendData['date']           =    date('Y-m-d H:i:s');
                            $sendData['status_info']    =    $data['statusbody'];
                            //邮箱短信通知
                            $noticeM -> sendEmailType($sendData);
                            $sendData['port']			=	'5';
                            $noticeM -> sendSMSType($sendData);
                        }
                    }
                }

                $return = array(
                    'errcode' => 9,
                    'msg'     => '职位审核'.$state_n.'设置成功！(ID:'.$id.$body.')'
                );

            }else{
                $return = array(
                    'errcode' => 8,
                    'msg'     => '职位审核设置失败！(ID:'.$id.')'
                );
            }

            return $return;
        }
    }

    /**
     * @desc    删除数据（单项、批量）
     * @param   int/array   $id
     * @param   array       $data : utype  delAccount
     */
    public function delJob($id,$data=array('utype'=>'')){

        if(!empty($id)){

            $return 		= array(
                'errcode'   => 8,
                'layertype' => 0,
                'msg'       => ''
            );

            if(is_array($id)){

                $ids    =	$id;

                $return['layertype']	=	1;

            }else{

                $ids    =   @explode(',', $id);

            }

            $id             =   pylode(',', $ids);

            $listA          =   $this -> getList(array('id'=>array('in',$id)),array('field'=>'id,uid,name'));

            $jobList        =   $listA['list'];
            if ($data['utype'] == 'admin'){

                if($data['delAccount'] == '1'){

                    $jUids	=	array();

                    foreach ($jobList as $jk => $jv){
                        $jUids[$jv['uid']]	=	$jv['uid'];
                    }

                    require_once ('userinfo.model.php');
                    $userinfoM	=	new	userinfo_model($this->db, $this->def);
                    return  $userinfoM -> delMember($jUids);
                }else{

                    $delWhere	=	array('id' => array('in',$id));
                }
            }else{
                $delWhere	=	array('id' => array('in',$id),'uid'=>$data['uid']);
            }

            $return['id']	=	$this -> delete_all('company_job', $delWhere, '');

            if($return['id']){

                $msg      =  array();
                $uids     =  array();
                $checkids =  array();

                //  提取职位 uid 和职位名称
                foreach ($jobList   as  $k => $v){

                    $uids[]  =  $v['uid'];

                    if ($data['utype'] == 'admin'){

                        $msg[$v['uid']][]	=  '您的职位《'.$v['name'].'》已被管理员删除';
                        $checkids[]			=	$v['id'];
                    }elseif($data['uid'] == $v['uid']){

                        $checkids[]			=	$v['id'];
                    }
                }
                if(!empty($checkids)){

                    $id	=	pylode(',',$checkids);
                }else{

                    $id	=	0;
                }

                if ($data['uid']){

                    $this->addMemberLog($v['uid'], 2, '职位管理：删除招聘职位（ID：'.pylode(',', $checkids).'）', 1, 3);
                }

                if(!empty($uids) && !empty($msg)){

                    $this->addSystem(array('uid'=>$uids,'usertype'=>2,'content'=>$msg));
                }

                $this -> delete_all('company_job_link', array('jobid' => array('in',$id)), '');
                $this -> delete_all('company_job_reward',array('jobid'=>array('in',$id)),'');
                $this -> delete_all('company_job_rewardlist',array('jobid'=>array('in',$id)),'');
                $this -> delete_all('company_job_rewardlog',array('jobid'=>array('in',$id)),'');
                $this -> delete_all('company_job_share',array('jobid'=>array('in',$id)), '');
                $this -> delete_all('company_job_sharelog',array('jobid'=>array('in',$id)), '');

                $this -> delete_all('fav_job', array('job_id' => array('in',$id)), '');
                $this -> delete_all('job_tellog', array('jobid' => array('in',$id)), '');
                $this -> delete_all('job_refresh_log', array('jobid' => array('in',$id)), '');
                $this -> delete_all('look_job', array('jobid' => array('in',$id)), '');
                $this -> delete_all('privacy_log', array('jobid' => array('in',$id)), '');
                $this -> delete_all('special_job_list', array('jobid' => array('in',$id)), '');
                $this -> delete_all('spview_log', array('jobid' => array('in',$id)), '');
                $this -> delete_all('spview_subscribe', array('jobid' => array('in',$id)), '');
                $this -> delete_all('spview_subscribe_msg', array('jobid' => array('in',$id)), '');
                $this -> delete_all('report', array('eid' => array('in',$id),'usertype'=>'1','type'=>'0'), '');
                $this -> delete_all('reserve_refresh', array('job_id' => array('in',$id)), '');
                $this -> delete_all('boom_groupmsg', array('job_id' => array('in',$id)), '');

                $this -> delete_all('user_entrust_record', array('jobid' => array('in',$id)), '');
                if ($data['utype'] == 'admin'){
                    $this -> delete_all('userid_msg', array('jobid' => array('in',$id)), '');
                    $this -> delete_all('userid_job', array('job_id' => array('in',$id)), '');
                }else{
                    $this -> update_once('userid_msg',array('isdel'=>2),array('jobid' => array('in',$id)));
                    $this -> update_once('userid_job',array('isdel'=>2),array('job_id' => array('in',$id)));
                }

            }

            $return['msg']		=	'职位(ID:'.$id.')';
            $return['errcode']	=	$return['id'] ? '9' :'8';
            $return['msg']		=	$return['id'] ? $return['msg'].'删除成功！' : $return['msg'].'删除失败！';
        }else{

            $return['msg']		=	'请选择您要删除的职位！';
            $return['errcode']	=	8;
        }

        return	$return;
    }

    // 查询职位数目
    function getJobNum($Where=array()){

        if (isset($Where['com_id'])) {
            $Where['uid']   =   $Where['com_id'];
            unset($Where['com_id']);
        }
        return $this->select_num('company_job',$Where);
    }
    function getRefJobNum($Where=array()){
        return $this->select_num('job_refresh_log',$Where);
    }

    //获取企业信息
    private function getComInfo($uid, $data = array())
    {

        require_once('company.model.php');
        $CompanyM = new company_model($this->db, $this->def);
        return $CompanyM->getInfo($uid, $data);
    }


    //获取企业信息列表
    private function getComList($whereData , $data = array()){

        require_once ('company.model.php');

        $CompanyM   =   new company_model($this->db, $this->def);

        return  $CompanyM   ->  getList($whereData , $data);
    }


    // 更新企业信息
    private function upComInfo($id = null, $where=array(), $data=array()){

        require_once ('company.model.php');

        $CompanyM   =   new company_model($this->db, $this->def);

        return  $CompanyM   ->  upInfo($id, $where, $data);
    }

    // 获取账户套餐信息
    private function getStatisInfo($uid, $data = array()){

        require_once ('statis.model.php');

        $StatisM    =   new statis_model($this->db, $this->def);

        return  $StatisM    ->  getInfo($uid , $data);
    }

    //获取账号信息列表
    private function getUserList($whereData, $data = array()){

        require_once ('userinfo.model.php');

        $UserinfoM = new userinfo_model($this->db, $this->def);

        return  $UserinfoM   ->  getList($whereData , $data);
    }

    //获取简历信息列表resume_expect
    private function getResumeExpectList($whereData, $data = array()){

        require_once ('resume.model.php');

        $resumeM    =   new resume_model($this->db, $this->def);

        return  $resumeM   ->  getList($whereData , $data);
    }
    //获取简历信息列表resume
    private function getResumeList($whereData, $data = array()){

        require_once ('resume.model.php');

        $resumeM    =   new resume_model($this->db, $this->def);

        return  $resumeM   ->  getResumeList($whereData , $data);
    }

    //职位信息和企业信息重组合并
    private function getMixInfo($JobInfo, $ComInfo){

        $JobInfo['lang']    =   @explode(',',$JobInfo['lang']);

        $JobInfo['jobname'] =   $JobInfo['name'];
        unset($JobInfo['name']);

        $JobInfo['jobrec']  =   $JobInfo['rec'];
        unset($JobInfo['rec']);
        unset($JobInfo['did']);
        unset($ComInfo['id']);

        unset($ComInfo['linkman']);
        unset($ComInfo['linktel']);
        unset($ComInfo['linkphone']);
        unset($ComInfo['linkmail']);

        $Info                   =   array_merge($JobInfo, $ComInfo);
        $Info['r_status']       =   $JobInfo['r_status'];
        $Info['uid']            =   $JobInfo['uid'];
        $Info['welfare']        =   $JobInfo['welfare'];
        $Info['com_provinceid'] =   $ComInfo['provinceid'];
        $Info['provinceid']     =   $JobInfo['provinceid'];
        $Info['cityid']         =   $JobInfo['cityid'];
        $Info['three_cityid']   =   $JobInfo['three_cityid'];
        $Info['sdate']          =   $JobInfo['sdate'];
        $Info['lastupdate']     =   $JobInfo['lastupdate'];
        $Info['rating']         =   $JobInfo['rating'];
        $Info['hy']             =   $JobInfo['hy'];
        $Info['pr']             =   $ComInfo['pr'];
        $Info['mun']            =   $ComInfo['mun'];
        $Info['x']              =   $JobInfo['x'];
        $Info['y']              =   $JobInfo['y'];
        $Info['statusbody']     =   $JobInfo['statusbody'];
        $Info['totaltime']      =   $JobInfo['totaltime'];
        $Info['totalnum']       =   $JobInfo['totalnum'];
        $Info['rating_name']    =   $ComInfo['rating_name'];
        $Info['reg_date_n']     =   $ComInfo['reg_date_n'];
        $Info['login_date_n']   =   $ComInfo['login_date_n'];

        if($ComInfo['hotstart']<=time() && $ComInfo['hottime']>=time()){

            $Info['hotlogo']	=   1;
        }
        //待处理：简历投递时间，回复率等问题
        if (isset($Info['totalnum'])){
            $this->getOperInfo($Info);
        }

        $Info['cert_n'] = @explode(',', $Info['cert']);

        $Info['com_url'] = Url('company', array( 'c' => 'show', 'id' => $JobInfo['uid'] ));

        $Info['description'] = str_replace(array('ti<x>tle', '“', '”','<img'), array('title', ' ', ' ','<img style="max-width:100%;height:auto;"'), $Info['description']);

        return $Info;
    }

    /**
     * 简历处理时间，效率等信息整理
     * @param $Info
     * @return
     */
    private function getOperInfo($Info)
    {

        if ($Info['totalnum'] != 0 && $Info['totaltime'] != 0) {

            $operatime = ceil($Info['totaltime'] / $Info['totalnum']);
            if ($operatime < 3600) {

                $Info['operatime'] = '一小时以内';
            } else if ($operatime >= 3600 && $operatime < 86400) {

                $Info['operatime'] = floor($operatime / 3600) . '小时';
            } else if ($operatime >= 86400) {

                $Info['operatime'] = floor($operatime / 86400) . '天';
            }
        } else {

            $Info['operatime'] = 0;
        }

        $sqWhere = array(

            'com_id' => $Info['uid'],
            'job_id' => $Info['id'],
            'isdel' => 9
        );

        $snum = $this->getSqJobNum($sqWhere);

        $Info['snum']   =   $snum;
        if ($snum == 0) {

            $Info['pre'] = '0';
        } else {

            $reWhere = $sqWhere;
            $reWhere['is_browse'] = array('>', 1);
            $replynum = $this->getSqJobNum($reWhere);
            $Info['pre'] = round(($replynum / $snum) * 100);
        }

        return $Info;
    }

    /**
     * 信息替换缓存数组内容
     * 2019-06-12 键名改为job_开头(原来键名_n结尾)
     */
    private function getInfoArray($jobInfo, $hb=''){

        $options                        =   array('hy','job','com','city');
        $cache                          =   $this -> getClass($options);

        $jobInfo['job_hy']              =   $cache['industry_name'][$jobInfo['hy']];
        $jobInfo['job_one']             =   $cache['job_name'][$jobInfo['job1']];
        $jobInfo['job_two']             =   $cache['job_name'][$jobInfo['job1_son']];
        $jobInfo['job_three']           =   $cache['job_name'][$jobInfo['job_post']];
        $jobInfo['job_city_one']        =   $cache['city_name'][$jobInfo['provinceid']];
        $jobInfo['job_city_two']        =   $cache['city_name'][$jobInfo['cityid']];
        $jobInfo['job_city_three']      =   $cache['city_name'][$jobInfo['three_cityid']];
        if (!empty($jobInfo['provinceid'])){

            $jobInfo['job_address']     =   $jobInfo['citystr']       =   $cache['city_name'][$jobInfo['provinceid']];
        }
        if (!empty($jobInfo['cityid'])){

            $jobInfo['job_address']     =   $jobInfo['citystr']       =   !empty($jobInfo['citystr']) ? $jobInfo['citystr'].'-'.$cache['city_name'][$jobInfo['cityid']] : $cache['city_name'][$jobInfo['cityid']];
        }

        if (!empty($jobInfo['three_cityid'])){

            $jobInfo['job_address']     =   !empty($jobInfo['job_address']) ? $jobInfo['job_address'].'-'.$cache['city_name'][$jobInfo['three_cityid']] : $cache['city_name'][$jobInfo['three_cityid']];
        }

        $jobInfo['com_city_one']        =   $cache['city_name'][$jobInfo['com_provinceid']];
        if(!empty($jobInfo['zp_num'])){
            $jobInfo['job_number']          =   $jobInfo['zp_num']."人";
        }else{
            $jobInfo['job_number']          =   "若干人";
        }


        $jobInfo['job_exp']             =   $cache['comclass_name'][$jobInfo['exp']];
        $jobInfo['job_report']          =   $cache['comclass_name'][$jobInfo['report']];
        if($jobInfo['sex'] == 152){
            $jobInfo['sex']			    =	2;
        }elseif ($jobInfo['sex'] == 153){
            $jobInfo['sex']			    =	1;
        }
        $jobInfo['job_sex']             =   $cache['com_sex'][$jobInfo['sex']] == '不限' ? '不限性别' : $cache['com_sex'][$jobInfo['sex']];
        $jobInfo['job_edu']             =   $cache['comclass_name'][$jobInfo['edu']];
        $jobInfo['job_marriage']        =   $cache['comclass_name'][$jobInfo['marriage']]  == '不限' ? '不限婚况' : $cache['comclass_name'][$jobInfo['marriage']];
        if(!empty($jobInfo['zp_minage']) && !empty($jobInfo['zp_maxage'])){
            if($jobInfo['zp_minage']==$jobInfo['zp_maxage']){
                $jobInfo['job_age'] = $jobInfo['zp_minage']."周岁以上";
            }else{
                $jobInfo['job_age'] = $jobInfo['zp_minage']."-".$jobInfo['zp_maxage']."周岁";
            }
        }elseif(!empty($jobInfo['zp_minage'])){
            $jobInfo['job_age'] = $jobInfo['zp_minage']."周岁以上";
        }elseif(empty($jobInfo['zp_minage']) && !empty($jobInfo['zp_maxage'])){
            $jobInfo['job_age'] = $jobInfo['zp_maxage']."周岁以下";
        }else{
            $jobInfo['job_age'] = "不限";
        }
        $jobInfo['job_pr']              =   $cache['comclass_name'][$jobInfo['pr']];
        $jobInfo['job_mun']             =   $cache['comclass_name'][$jobInfo['mun']];
        $jobInfo['job_salary']          =   salaryUnit($jobInfo['minsalary'], $jobInfo['maxsalary'], $hb);

        if(isset($jobInfo['lang']) && is_array($jobInfo['lang'])){

            $lang                       =   $jobInfo['lang'];
            foreach($lang as $key => $value){
                if($value){
                    $langinfo[]             =   $cache['comclass_name'][$value];
                }

            }
            $jobInfo['job_lang']        =   $langinfo;
            $jobInfo['lang_info']       =   @implode(',', $langinfo);

        }

        $jobInfo['welfare_info']        =   $jobInfo['welfare'];
        $jobInfo['job_welfare']         =   empty($jobInfo['welfare']) ? array() : @explode(',', $jobInfo['welfare']);

        //平均回复时长
        //$operatime                      =   time() - $jobInfo['operatime'];
        if(isset($jobInfo['totalnum']) && $jobInfo['totalnum']!=0 && $jobInfo['totaltime']!=0){
            $operatime                    =   ceil($jobInfo['totaltime']/$jobInfo['totalnum']);
            if($operatime < 3600){
                $jobInfo['operatime']       =   '1小时以内';
            }else if($operatime >= 3600 && $operatime < 86400){
                $jobInfo['operatime']       =   floor($operatime/3600).'小时';
            }else if($operatime >= 86400){
                $jobInfo['operatime']       =   floor($operatime/86400).'天';
            }
        }else{
            $jobInfo['operatime']       =   0;
        }
        if(isset($jobInfo['description'])){
            $jobInfo['job_description'] = strip_tags($jobInfo['description']);
            $jobInfo['job_description'] = str_replace(array('&quot;', '&nbsp;', '<>'), array('', '', ''), $jobInfo['job_description']);
        }
        return $jobInfo;
    }



    /**
     * 设置联系方式为保密格式
     */
    private function setContactHide($tel){

        $tmpTel	=   '';
        $tmpTel	=   $this -> getNumbers($tel);
        $tel	=   sub_string($tmpTel);

        return $tel;

    }

    /**
     * 获取数字
     */
    private function getNumbers($phoneStr){

        $resNum                     =   '';

        preg_match_all('/\d+/', $phoneStr, $pregArr);

        if(!empty($pregArr) && !empty($pregArr[0])){
            foreach($pregArr[0] as $pv){
                $resNum             .=  $pv.' ';
            }
        }
        return mb_substr(trim($resNum), 0, 13);
    }

    // 查询职位联系方式
    public function getComJobLinkInfo($Where = array(), $data = array())
    {

        $select =   $data['field'] ? $data['field'] : '*';
        $Info   =   $this->select_once('company_job_link', $Where, $select);

        if (empty($Info) && !empty($Where['uid'])) {

            // 处理因新联系方式同步到所有职位，非当前修改职位无记录，导致查不到数据情况

            $Info   =   $this->select_once('company_job_link', array('uid' => $Where['uid']), $select);
        }
        return $Info;
    }

	public function getJobLinkInfo($Where = array(), $data = array())
    {
        $select = $data['field'] ? $data['field'] : '*';
        $Info = $this->select_once('company_job_link', $Where, $select);
        return $Info;
    }

    /**
     * @desc    查询 company_job_link 表数据，多条查询
     * @param array $Where
     * @param array $data
     * @return boolean|void|string
     */
    public function getComJobLinkList($Where = array(), $data=array()){

        return $this->select_all('company_job_link', $Where,$data['field']);

    }

    // 添加职位联系方式
    public function addComJobLinkInfo($data=array()){

        return $this->insert_into('company_job_link',$data);

    }
    // 跟新职位联系方式
    public function upComJobLinkInfo($data=array(),$Where = array()){

        return $this->update_once('company_job_link',$data,$Where);

    }


    /**
     * @desc    职位置顶
     * @param   int     $id
     * @param   array   $data
     */
    public function addTopJob($id, $data = array()){

        if(!empty($id) && !empty($data)) {

            $ids    =   @explode(',', $id);

            $return =   array();

            if(is_array($ids)){

                // 查询职位信息，提取职位置顶时间 xsdate，uid，name
                $ListA          =   $this -> getList(array('id' => array('in', pylode(',', $ids))), array('field'=>'id,uid,name,xsdate'));

                $jobList        =   $ListA['list'];

                if (!empty($jobList)) {

                    if (intval($data['top']) == 1) {

                        $jobData['xsdate']      =   '0';

                        $return['id']           =   $this -> upInfo($jobData, array('id' => array('in', pylode(',', $ids))));

                        $return['msg']		    =	'取消职位置顶(ID:'.pylode(',', $ids).')';

                        $return['msg']		    =	$return['id'] ? $return['msg'].'成功！' : $return['msg'].'失败！';

                    }else if (intval($data['days']) > 0) {

                        foreach($jobList as $v){

                            if($v['xsdate']     <   time()){

                                $gid[]          =   $v['id'];   //置顶日期已过期

                            }else{

                                $mid[]          =   $v['id'];   //置顶日期尚未过期

                            }

                        }

                        $time                   =   intval($data['days']) * 86400;

                        if(!empty($gid)){

                            $jobData['xsdate']  =   time()  +   $time;

                            $return['id']       =   $this -> upInfo($jobData, array('id' => array('in', pylode(',', $gid))));
                        }

                        if(!empty($mid)){

                            $jobData['xsdate']  =   array('+', $time);

                            $return['id']       =   $this -> upInfo($jobData, array('id' => array('in', pylode(',', $mid))));

                        }

                        $return['msg']		    =	'职位置顶(ID:'.pylode(',', $id).')';
                        $return['msg']		    =	$return['id'] ? $return['msg'].'设置成功！' : $return['msg'].'设置失败！';

                    }else {

                        $return['msg']          =   '置顶天数不能为空，请重试！';

                    }

                    if($return['id']){

                        $msg      =  array();
                        $uids     =  array();

                        //  提取职位uid 和职位名称
                        foreach ($jobList   as  $k => $v){

                            $uids[]  =  $v['uid'];

                            if (intval($data['top']) == 0){

                                $msg[$v['uid']][]  =  '您的职位<a href="comjobtpl,'.$v['id'].'">《'.$v['name'].'》</a>管理员已置顶';

                            }elseif (intval($data['top']) == 1){

                                $msg[$v['uid']][]  =  '您的职位<a href="comjobtpl,'.$v['id'].'">《'.$v['name'].'》</a>被管理员取消置顶';
                            }

                        }
                        //发送系统通知
                        $this->addSystem(array('uid'=>$uids,'usertype'=>2,'content'=>$msg));

                    }

                }  else {

                    $return['msg']      =  '系统繁忙';

                }

            }

        }
        //操作状态 9：成功 8:失败 配合原有提示函数
        $return['errcode']	=	$return['id'] ? '9' :'8';

        return	$return;

    }

    //  职位推荐
    public function addRecJob($id, $data = array()){

        if(!empty($id) && !empty($data)) {

            $ids    =   @explode(',', $id);

            $return =   array();

            if(is_array($ids)){

                // 查询职位信息，提取职位推荐时间 rec_time，uid，name
                $ListA          =   $this -> getList(array('id' => array('in', pylode(',', $ids))), array('field'=>'id,uid,name,rec_time'));

                $jobList        =   $ListA['list'];

                if (!empty($jobList)) {

                    if (intval($data['rec']) == 1) {

                        $jobData['rec']         =   '0';

                        $jobData['rec_time']    =   '0';

                        $return['id']           =   $this -> upInfo($jobData, array('id' => array('in', pylode(',', $ids))));

                        $return['msg']		    =	'取消职位推荐(ID:'.pylode(',', $ids).')';

                        $return['msg']		    =	$return['id'] ? $return['msg'].'成功！' : $return['msg'].'失败！';

                    }else if (intval($data['days']) > 0) {

                        foreach($jobList as $v){

                            if($v['rec_time']   <   time()){

                                $gid[]          =   $v['id'];   //推荐日期已过期

                            }else{

                                $mid[]          =   $v['id'];   //推荐日期尚未过期

                            }

                        }

                        $time                   =   intval($data['days']) * 86400;

                        $jobData['rec']         =   '1';

                        if(!empty($gid)){

                            $jobData['rec_time']=   time()  +   $time;

                            $return['id']       =   $this -> upInfo($jobData, array('id' => array('in', pylode(',', $gid))));
                        }

                        if(!empty($mid)){

                            $jobData['rec_time']=   array('+', $time);

                            $return['id']       =   $this -> upInfo($jobData, array('id' => array('in', pylode(',', $mid))));

                        }

                        $return['msg']		    =	'职位推荐(ID:'.pylode(',', $id).')';
                        $return['msg']		    =	$return['id'] ? $return['msg'].'设置成功！' : $return['msg'].'设置失败！';

                    }else {

                        $return['msg']          =   '推荐天数不能为空，请重试！';

                    }

                    if($return['id']){

                        $msg      =  array();
                        $uids     =  array();

                        //  提取职位uid 和职位名称
                        foreach ($jobList   as  $k => $v){

                            $uids[]  =  $v['uid'];

                            if (intval($data['rec']) == 0){

                                $msg[$v['uid']][]  =  '您的职位<a href="comjobtpl,'.$v['id'].'">《'.$v['name'].'》</a>管理员已推荐';

                            }elseif (intval($data['rec']) == 1){

                                $msg[$v['uid']][]  =  '您的职位<a href="comjobtpl,'.$v['id'].'">《'.$v['name'].'》</a>被管理员取消推荐';
                            }

                        }
                        //发送系统通知
                        $this->addSystem(array('uid'=>$uids,'usertype'=>2,'content'=>$msg));

                    }

                }  else {

                    $return['msg']      =  '系统繁忙';

                }

            }

        }
        //操作状态 9：成功 8:失败 配合原有提示函数
        $return['errcode']	=	$return['id'] ? '9' :'8';

        return	$return;

    }

    //  职位紧急招聘
    public function addUrgentJob($id, $data = array()){

        if(!empty($id) && !empty($data)) {

            $ids    =   @explode(',', $id);

            if(is_array($ids)){

                // 查询职位信息，提取职位紧急招聘时间 urgent_time，uid，name
                $ListA          =   $this -> getList(array('id' => array('in', pylode(',', $ids))), array('field'=>'id,uid,name,urgent_time'));

                $jobList        =   $ListA['list'];

                if (!empty($jobList)) {

                    if (intval($data['urgent']) == 1) {

                        $jobData['urgent']      =   '0';

                        $jobData['urgent_time'] =   '0';

                        $return['id']           =   $this -> upInfo($jobData, array('id' => array('in', pylode(',', $ids))));

                        $return['msg']		    =	'取消职位紧急招聘(ID:'.pylode(',', $ids).')';

                        $return['msg']		    =	$return['id'] ? $return['msg'].'成功！' : $return['msg'].'失败！';

                    }else if (intval($data['days']) > 0) {

                        foreach($jobList as $v){

                            if($v['urgent_time']<   time()){

                                $gid[]          =   $v['id'];   //紧急招聘日期已过期

                            }else{

                                $mid[]          =   $v['id'];   //紧急招聘日期尚未过期

                            }

                        }

                        $time                   =   intval($data['days']) * 86400;

                        $jobData['urgent']      =   '1';

                        if(!empty($gid)){

                            $jobData['urgent_time']     =   time()  +   $time;

                            $return['id']       =   $this -> upInfo($jobData, array('id' => array('in', pylode(',', $gid))));
                        }

                        if(!empty($mid)){

                            $jobData['urgent_time']     =   array('+', $time);

                            $return['id']       =   $this -> upInfo($jobData, array('id' => array('in', pylode(',', $mid))));

                        }

                        $return['msg']		    =	'职位紧急招聘(ID:'.pylode(',', $id).')';
                        $return['msg']		    =	$return['id'] ? $return['msg'].'设置成功！' : $return['msg'].'设置失败！';

                    }else {

                        $return['msg']          =   '紧急招聘天数不能为空，请重试！';

                    }

                    if($return['id']){

                        $msg      =  array();
                        $uids     =  array();

                        //  提取职位uid 和职位名称
                        foreach ($jobList   as  $k => $v){

                            $uids[]  =  $v['uid'];

                            if (intval($data['urgent']) == 0){

                                $msg[$v['uid']][]  =  '您的职位<a href="comjobtpl,'.$v['id'].'">《'.$v['name'].'》</a>管理员已设置紧急招聘';

                            }elseif (intval($data['urgent']) == 1){

                                $msg[$v['uid']][]  =  '您的职位<a href="comjobtpl,'.$v['id'].'">《'.$v['name'].'》</a>被管理员取消紧急招聘';
                            }

                        }
                        //发送系统通知
                        $this->addSystem(array('uid'=>$uids,'usertype'=>2,'content'=>$msg));

                    }

                }  else {

                    $return['msg']      =  '系统繁忙';

                }

            }

        }
        //操作状态 9：成功 8:失败 配合原有提示函数
        $return['errcode']	=	$return['id'] ? '9' :'8';

        return	$return;

    }

    // 职位申请，单条查询
    function getSqJobInfo($where = array(), $data=array()) {

        if (!empty($where)) {

            $select = $data['field'] ? $data['field'] : '*';

            $info           =   $this->select_once('userid_job',$where,$select);

            if ($info && is_array($info)) {

                return $info;

            }

        };
    }

    //  申请职位列表 ，多条查询
    function getSqJobList($whereData,$data=array()) {

        $select =   isset($data['field']) ? $data['field'] : '*';

        $List   =   $this   ->  select_all('userid_job',$whereData,$select);

        $utype  =   isset($data['utype']) ? $data['utype'] : '';

        if (!empty($List) && $utype != 'simple') {

            $List   =   $this -> subSqListInfo($List, $data);

        }

        return $List;

    }

    // 申请职位列表信息补充
    private function subSqListInfo($List,$data=array()) {
        $uids           = array();
        $eids           = array();
        $jobids         = array();
        $userid_msg     = array();
        $remark         = array();
        $company_job    = array();
        $company        = array();
        $lietou         = array();
        $downList       = array();
        $freedownList   = array();
        $chatLogList    = array();
        foreach ($List as $lk => $v) {
            if ($v['uid']) {
                $uids[] = $v['uid'];
            }
            if ($v['eid']) {
                $eids[] = $v['eid'];
            }
            if ($v['job_id']) {
                $jobids[] = $v['job_id'];
                $List[$lk]['wapjob_url'] = Url('wap', array('c' => 'job', 'a' => 'comapply', 'id' => $v['job_id']));
            }
            if ($v['com_id']) {
                $comuids[] = $v['com_id'];
                $List[$lk]['wapcom_url'] = Url('wap', array('c' => 'company', 'a' => 'show', 'id' => $v['com_id']));
            }
            if (isset($v['remark'])) {
                // 过滤换行符
                $List[$lk]['remark'] = str_replace(PHP_EOL, '', $v['remark']);
            }

        }
        $cache                          =   $this -> getClass(array('job','hy','city','com'));

        //  查询个人简历名称
        $reWhere['id']					=   array('in', pylode(',', $eids));

        if($data['utype']=='lietou'){//猎头应聘简历，查询优质简历
            $reWhere['height_status']	=   2;
        }
        $reData['field']                =   '`id`,`name`,`job_classid`,`minsalary`,`maxsalary`,`height_status`,`edu`,`exp`,`hy`,`lastupdate`,`city_classid`,`sex`,`birthday`,`state`,`status`,`r_status`';

        $resumeexpectList               =   $this -> getResumeExpectList($reWhere, $reData);

        //  查询个人姓名
        $rWhere['uid']                  =   array('in', pylode(',', $uids));
        $rData['field']                 =   '`uid`,`name`,`nametype`,`sex`,`telphone`,`def_job`,`photo`,`defphoto`,`phototype`,`photo_status`';
        $rData['downresume_where']      =   array('comid'=>$data['uid'],'usertype'=>$data['usertype']);

        $resumeList                     =   $this -> getResumeList($rWhere, $rData);

        if($data['usertype']==2){
            $userid_msg		=	$this -> select_all('userid_msg',array('fid'=>$data['uid'],'isdel'=>9,'uid'=>array('in',pylode(",",$uids))),'`uid`');
            $remark         =   $this -> select_all("resume_remark", array('eid' => array('in', pylode(',', $eids)),'comid'=>$data['uid']), '`eid`,`remark`,`comid`,`status`');
        }

        if($data['usertype']==1){

            $company_job	=	$this -> getList(array('id' => array('in',pylode(',',$jobids))),array('field'=>'id,status,minsalary,maxsalary,exp,edu'));

            $company		=	$this -> getComList(array('uid' => array('in',pylode(',',$comuids))),array('field'=>'`cityid`,`uid`,`name`,`logo`'));

            require_once ('lietou.model.php');
            $ltM			=	new lietou_model($this->db, $this->def);
            $lietou			=	$ltM -> getList(array('uid'=>array('in',pylode(',',$comuids))),array('field'=>'`cityid`,`uid`,`com_name`'));
        }

        if ((isset($data['is_link']) && $data['is_link'] == 'yes') || (isset($data['hr']) && $data['hr'] == 1)) {

            $downList       =   $this->select_all('down_resume', array('comid' => array('in', pylode(',', $comuids)), 'uid'=>array('in',pylode(",",$uids))), '`comid`,`eid`');
			$freedownList = $this->select_all('freedown_resume', array('comid' => array('in', pylode(',', $comuids)), 'uid'=>array('in',pylode(",",$uids))), '`comid`,`eid`');
        }

        if (isset($data['hr']) && $data['hr'] == 1){

            $chatLogList    =   $this->select_all('chat_log', array('from' => $data['uid'], 'to' => array('in', pylode(',', $uids)), 'fusertype' => 2, 'tusertype' => 1, 'zdid' => array('>', 0)), '`to`');
            $chatUidS       =   array();
            foreach ($chatLogList as $chk => $chv){
                if (!in_array($chv['to'], $chatUidS)){

                    $chatUidS[] =   $chv['to'];
                }
            }
        }

        require_once('resume.model.php');
        $resumeM            =   new resume_model($this->db, $this->def);
        $resume_state_arr   =   $resumeM->resume_state_arr;

        foreach ($List  as  $k  =>  $v){
            $List[$k]['remark']     =   '';
            $List[$k]['isChatEd']   =   0;
            foreach ($remark as $val) {
                if ($v['eid'] == $val['eid'] && $v['com_id'] == $val['comid']) {
                    $List[$k]['remark'] = $val['remark'];
                    if(isset($val['status'])){
                        $List[$k]['status_n']        =    $cache['comclass_name'][$val['status']];
                    }
                }

            }
            if($v['is_browse']){

                $List[$k]['is_browse']      =   (int)$v['is_browse'];
            }

            if ($v['is_browse'] == 3){

                $List[$k]['zt_n']   =   '等通知';
            }else if ($v['is_browse'] == 4){

                $List[$k]['zt_n']   =   '不符合';
            }else if ($v['is_browse'] == 5){

                $List[$k]['zt_n']   =   '未接通';
            }

            if($v['datetime'] > strtotime(date('Y-m-d'))){

                $List[$k]['datetime_n']     =   '今天 '.date('H:i',$v['datetime']);

            }else if($v['datetime'] > mktime(0,0,0,1,1,date('Y'))){

                $List[$k]['datetime_n']     =   date('m月d日',$v['datetime']);
            }else{

                $List[$k]['datetime_n']     =   date('Y-m-d',$v['datetime']);
            }
            if ($v['isdel'] == 1) {
                $List[$k]['isdel_n']        =   '简历用户删除';
            } else if ($v['isdel'] == 2) {
                $List[$k]['isdel_n']        =   '企业用户删除';
            } else if ($v['isdel'] == 3) {
                $List[$k]['isdel_n']        =   '猎头用户删除';
            } else {
                $List[$k]['isdel_n']        =   '正常';
            }
            if ($data['is_link']  ==  'yes' ||  $data['hr'] == 1) {
                foreach ($downList as $dv) {
                    if ($dv['comid'] == $v['com_id'] && $v['eid'] == $dv['eid']) {

                        $List[$k]['islink'] =   '1';
                    }
                }
				foreach ($freedownList as $dv){
                    if ($dv['comid'] == $v['com_id'] && $v['eid'] == $dv['eid']) {
                        if (isVip($data['statis']['vip_etime'])) {
                            $List[$k]['islink'] = '1';
                        }else{
                            $List[$k]['islink'] = '0';
                        }
                    }
                }
            }

            if (is_array($resumeexpectList['list'])) {
	            foreach ($resumeexpectList['list'] as $rv){

	                if ($v['eid']   ==  $rv['id']) {

	                    $List[$k]['eid']            =   $rv['id'];
	                    $List[$k]['waprurl']        =   Url('wap',array('c'=>'resume','a'=>'show','id'=>$rv['id']));
	                    $List[$k]['state_n']        =   '';
	                    $List[$k]['state']          =   $rv['state'];
	                    $List[$k]['rname']			=   $rv['name'];
	                    $List[$k]['jobname']		=	$rv['job_classname'];
	                    $List[$k]['salary']			=	$rv['salary'];
	                    $List[$k]['height_status']	=	$rv['height_status'];
	                    $List[$k]['edu']			=	$rv['edu_n'];
	                    $List[$k]['exp']			=	$rv['exp_n'];
	                    $List[$k]['sex']			=	$rv['sex_n'];
	                    $List[$k]['age']			=	$rv['age_n'];
	                    $List[$k]['lastupdate_n']	=	date('Y-m-d',$rv['lastupdate']);
	                    $List[$k]['hyname']			=	$cache['industry_name'][$rv['hy']];
	                    $List[$k]['resume_status']	=	$rv['status'];

	                    if($rv['job_classid']!=""){

	                        $job  =   @explode(',' , $rv['job_classid']);

	                        $joblist=array();

	                        foreach($job as $val){

	                            $joblist[]    =   $cache['job_name'][$val];

	                        }

	                        $List[$k]['jobclassname'] =   $joblist['0'];
	                    }

	                    if($rv['city_classid']!=""){

	                        $city =   @explode(',' , $rv['city_classid']);

	                        $citylist =   array();

	                        foreach($city as $val){
	                            $citylist[]=$cache['city_name'][$val];
	                        }

	                        $List[$k]['cityclassname']=$citylist['0'];
	                    }

	                    if($rv['state']!=1 && $rv['state']!=2){
	                        $List[$k]['state_n'] = $resume_state_arr[$rv['state']];
	                    }

	                }
	            }
            }
            if($data['utype']=='lietou'){
                if (is_array($resumeexpectList['list'])) {
	                foreach ($resumeexpectList['list'] as $rv){

	                    if ($v['eid']   ==  $rv['id']) {
	                        $List[$k]['hy']				=	$rv['hy_n'];
	                        $List[$k]['cityname']		=	$rv['city_classname'];
	                        $List[$k]['lastupdate']		=	$rv['lastupdate'];
	                    }
	                }
                }
            }
            foreach ($resumeList as $val){
                $icon  =  $val['sex'] == 1 ? $this->config['sy_member_icon'] : $this->config['sy_member_iconv'];
                if ($v['uid']   ==  $val['uid']) {
                    $List[$k]['name']		=    $val['name_n'];
                    $List[$k]['username_n'] =    $val['username_n'];
					//开启隐私号模式 一律不显示联系方式,后台不受是否开启隐私号的影响
					if($data['utype'] == 'admin' || $this -> config['sy_privacy_open'] != '1'){
						$List[$k]['telphone']   =    $val['telphone'];
					}

                    $List[$k]['photo']      =    checkpic($val['photo'],$icon);
                }
            }
            foreach($userid_msg as $val){
                if($v['uid']==$val['uid']){
                    $List[$k]['userid_msg']	=	1;
                }
            }
            if (is_array($company_job['list'])) {
	            foreach($company_job['list'] as $val){
	                if($v['job_id']==$val['id']){
	                    $List[$k]['status']		=	$val['status'];
	                    $List[$k]['job_salary'] =	salaryUnit($val['minsalary'], $val['maxsalary']);
	                    $List[$k]['edu_n']=$cache['comclass_name'][$val['edu']];
	                    $List[$k]['exp_n']=$cache['comclass_name'][$val['exp']];
	                }
	            }
            }
            if (is_array($company['list'])) {
	            foreach($company['list'] as $val){
	                $icon  =  $this->config['sy_unit_icon'];
	                if($v['com_id']==$val['uid']){
	                    $List[$k]['city']		=	$val['job_city_two'];
	                    $List[$k]['logo']      =    checkpic($val['logo'],$icon);
	                }
	            }
            }


            foreach($lietou as $val){
                if($v['com_id']==$val['uid']){
                    $List[$k]['city']		=	$val['job_city_two'];
                }
            }

            if (!empty($chatUidS) && in_array($v['uid'], $chatUidS)){
                $List[$k]['isChatEd']   =   1;
            }

            if ($data['utype'] == 'admin') {
                if ($v['type'] == 3) {
                    $job_url = Url('lietou', array('c' => 'jobshow', 'id' => $v['job_id']));
                    $com_url = Url('lietou', array('c' => 'headhunter', 'uid' => $v['com_id']));
                } else {
                    $job_url = Url('job', array('c' => 'comapply', 'id' => $v['job_id'], 'look' => 'admin'));
                    $com_url = Url('company', array('c' => 'show', 'id' => $v['com_id'], 'look' => 'admin'));
                }
                $List[$k]['job_url'] = $job_url;
                $List[$k]['com_url'] = $com_url;
                $List[$k]['telphone_url'] = Url('user', array('c' => 'users_member', 'a' => 'Imitate', 'uid' => $v['uid']), 'admin');
                $List[$k]['datetime_n_n'] = $v['datetime'] ? date('Y-m-d H:i', $v['datetime']) : '';
            }
        }

        return $List;
    }

    /**
     * @desc     删除申请职位记录
     * @param    $id
     * @param    array $data
     * @return   $return
     */
    function delSqJob($id = null , $data = array()) {

        $return     =   array();

        if(!empty($id) || !empty($data['where'])){

            $where      =   array();

            if (!empty($id)) {

                if(is_array($id)){

                    $ids        =	$id;

                    $return['layertype']	=	1;

                }else{

                    $ids        =   @explode(',', $id);

                    $return['layertype']	=	0;

                }

                $ids            =   pylode(',', $ids);

                $where['id']    =   array('in', $ids);

            }

            if ($data['where']) {

                $where          =   array_merge($where, $data['where']);

            }elseif($data['utype']!='admin'){

                if($data['utype'] == 'user'){
                    $where['uid']		=	$data['uid'];
                }else{
                    $where['com_id']	=	$data['uid'];
                }


            }
            //个人会员中心删除申请记录
            if($data['utype'] == 'user'){
                if(intval($id)){
                    $userid		=   $this -> getSqJobInfo(array('id'=>intval($id),'uid'=>$data['uid']),array('field'=>'`com_id`'));
                }
            }
            //企业会员中心删除申请记录
            if($data['utype'] == 'com'){

                $sqList			=   $this -> getSqJobList(array('id'=>array('in',$ids)),array('field'=>'`uid`,`job_id`,`type`'));

                if(is_array($sqList)){

                    $jobid		=	array();
                    $uid		=	array();
                    $ltjobid    =	array();

                    foreach($sqList as $v){

                        if($v['type']==1){

                            $jobid[]	=	$v['job_id'];

                        }elseif($v['type']==2){

                            $ltjobid[]	=	$v['job_id'];

                        }

                        $uid[]			=	$v['uid'];

                    }

                    $this -> update_once('company_job',array('operatime' => time()),array('id'=>array('in',pylode(",",$jobid)),'uid'=>$data['uid']));
                    $this -> update_once('lt_job',array('operatime'=>time()),array('id'=>array('in',pylode(",",$ltjobid)),'uid'=>$data['uid']));
                    $this -> update_once('member_statis',array('sq_jobnum'=>array('-',1)),array('uid'=>array('in',pylode(",",$uid))));
                }

                $num=count($sqList);
                $this -> update_once('company_statis',array('sq_job'=>array('-',$num)),array('uid'=>$data['uid']));

                $num=count($sqList);
                $this -> update_once('lt_statis',array('sq_job'=>array('-',$num)),array('uid'=>$data['uid']));

            }


            if($data['utype']=='lietou'){
                $return['id']   =   $this -> update_once('userid_job',array('isdel'=>3),array('com_id'=>$data['uid'],'id'=>pylode(',', $ids)));
            }else{

                if($data['norecycle'] == '1'){	// 数据库清理操作，不插入回收表

                    $return['id']	=	$this -> delete_all('userid_job', $where, $data['limit'],'','1');
                }else{
                    if($data['utype'] == 'admin'){
                        // 后台操作，删除记录
                        $return['id']	=	$this -> delete_all('userid_job', $where, '');
                    }else{
                        // 用户操作，修改状态
                        $return['id']   =   $this -> update_once('userid_job',array('isdel'=>$data['usertype']),$where);
                    }
                }
            }

            if($return['id']){

                if ($data['utype'] == 'user') {
                    $this->update_once('company_statis', array('sq_job' => array('-', 1)), array('uid' => $userid['com_id']));
                    $this->update_once('member_statis', array('sq_jobnum' => array('-', 1)), array('uid' => $data['uid']));
                    $this->addMemberLog($data['uid'], $data['usertype'], '职位申请：删除申请记录(ID:' . pylode(',', $ids) . ')', 6, 3);
                }

                if ($data['utype'] == 'com') {
                    $this->addMemberLog($data['uid'], $data['usertype'], '职位申请：删除申请记录(ID:' . pylode(',', $ids) . ')', 6, 3);
                }
                if ($data['utype'] == 'lietou') {
                    $this->addMemberLog($data['uid'], $data['usertype'], '职位申请：删除申请记录(ID:' . pylode(',', $ids) . ')', 6, 3);
                }
                if ($data['utype'] != 'lietou') {
                    $return['msg'] = '职位申请记录(ID:' . pylode(',', $id) . ')';
                }
                $return['errcode']	=	9;
                $return['msg']		=	$return['msg'].'删除成功！';

            }else{
                $return['errcode']	=	'8';
                $return['msg']		=	$return['msg'].'删除失败！';
            }
        }else{
            $return['msg']		=	'请选择您要删除的数据！';
            $return['errcode']	=	8;
        }

        return	$return;
    }
    /**
     * @desc     申请职位：批量阅读
     * @param    $id
     * @param    array $data
     * @return   $return
     */
    function ReadSqJob($id = null,$data = array()) {

        if(!empty($id)){

            $rows       =   $this -> getSqJobList(array('id'=>array('in',pylode(",",$id)),'com_id'=>$data['uid']),array('field'=>"`job_id`,`type`"));

            if($rows && is_array($rows)){

                foreach($rows as $val){

                    if($val['type']==1){
                        $jobid[]	=	$val['job_id'];
                    }elseif($val['type']==2){
                        $ltjobid[]	=	$val['job_id'];
                    }

                }

                $this -> update_once('company_job', array('operatime' => time()), array('id' => array('in', pylode(',', $jobid)), 'uid' => $data['uid']));

                $this -> update_once('lt_job', array('operatime' => time()), array('id' => array('in', pylode(',' , $ltjobid)), 'uid' => $data['uid']));

            }

            $userid       =   $this -> getSqJobList(array('com_id' => $data['uid'], 'is_browse' => array('<>' , 1)),array('field' => "`id`"));

            if($userid && is_array($userid)){

                foreach($userid as $v){
                    $userids[]	=	$v['id'];
                }

            }


            $where['com_id']                 =	$data['uid'];

            if (!empty($userids)) {

                $where['PHPYUNBTWSTART_A']   =	'';
                $where['id'][]			     =	array('in',pylode(",",$id),'AND');
                $where['id'][]				 =	array('notin',pylode(",",$userids), 'AND');
                $where['PHPYUNBTWEND_A']	 =	'';

            }else{

                $where['id']                =	array('in', pylode(",",$id));

            }

            $return['id']	=	$this -> update_once('userid_job', array('is_browse' => 2,'endtime' => time()), $where);

            $this->addMemberLog($data['uid'], $data['usertype'], "职位申请：批量阅读申请记录(ID:" . pylode(',', $id) . ")", 6, 2);

            $return['layertype']=	1;

            $return['errcode']	=	$return['id'] ? 9 : 8;
            $return['msg']		=	$return['id'] ? '操作成功！' : '操作失败！';
        }else{
            $return['msg']		=	'请选择您要操作的数据！';
            $return['errcode']	=	8;
        }

        return	$return;
    }
    /**
     * @desc     申请职位：设置简历状态
     * @param    $id
     * @param    array $data
     * @return   $return
     */
    function BrowseSqJob($id = null,$data = array()) {
        if(!empty($id)){

            $browse	=	$data['browse'];
            $port	=	$data['port'];
            $row	=	$this -> getSqJobInfo(array('id'=>$id,'com_id'=>$data['uid']),array('field'=>'`uid`,`eid`,`job_id`,`type`,`endtime`'));
            if($row['type']==1){

                $this -> update_once('company_job',array('operatime'=>time()),array('id'=>$row['job_id'],'uid'=>$data['uid']));
            }elseif($row['type']==2){

                $this -> update_once('lt_job',array('operatime'=>time()),array('id'=>$row['job_id'],'uid'=>$data['uid']));
            }
            //判断当前是否为标记其他状态（除了已查看  待处理）
            if($browse>2 && $row['endtime']==""){
                $userjobdata    =   array(
                    'is_browse' =>  $browse,
                    'endtime'   =>  time()
                );
            }else{
                $userjobdata    =   array(
                    'is_browse' =>  $browse
                );
            }

            $this -> update_once('userid_job',$userjobdata,array('id'=>$id,'com_id'=>$data['uid']));

            if($browse==4){

                $resume =   $this -> select_once('resume',array('uid'=>$row['uid']),array('field'=>'uid,name,telphone,email'));

                if($row['type']==2){

                    $comjob	=	$this -> select_once('lt_job',array('id'=>$row['job_id'],'uid'=>$data['uid']),array('field'=>"`job_name` as `name`,`com_name`"));
                }elseif($row['type']==1){

                    $comjob	=	$this -> select_once("company_job",array('id'=>$row['job_id'],'uid'=>$data['uid']),array('field'=>"`name`,`com_name`"));
                }

                $ndata['uid']		=	$resume['uid'];
                $ndata['cname']		=	$data['username'];
                $ndata['name']		=	$resume['name'];
                $ndata['type']		=	"sqzwhf";
                $ndata['cuid']		=	$data['uid'];
                $ndata['company']	=	$comjob['com_name'];
                $ndata['jobname']	=	$comjob['name'];

                if(checkMsgOpen($this -> config)){
                    $ndata["moblie"]=	$resume["telphone"];
                }
                if($this -> config['sy_email_sqzwhf']=='1' && $resume["email"] && $this -> config['sy_email_set']=="1"){
                    $ndata["email"]	=	$resume["email"];
                }
                if($ndata["email"]||$ndata['moblie']){
                    include_once('notice.model.php');
                    $noticeM		=	new notice_model($this->db, $this->def);
                    $noticeM -> sendEmailType($ndata);
                    $ndata['port']	=	$port;
                    $noticeM -> sendSMSType($ndata);
                }
            }
            $return	=	1;
        }

        return	$return;
    }

    /**
     * @desc 申请职位数目
     */
    function getSqJobNum($Where = array()){
        return $this->select_num('userid_job', $Where);
    }

    /**
     * 增加申请职位记录
     * @param array $data
     * @param array $extData
     * @return bool|null
     * @throws SmartyException
     */
    function addSqJob($data = array(), $extData = array())
    {

        $nid            =   $this->insert_into('userid_job', $data);

        if (isset($nid)) {

            // 申请职位，处理向企业发送短信、邮件提醒
            $uid        =   $data['uid'];
            $eid        =   $data['eid'];
            $jobid      =   $data['job_id'];
            $comid      =   $data['com_id'];
            $is_link    =   !empty($extData['comjob']['is_link']) ? $extData['comjob']['is_link'] : 1;

            // 投递短信通知
            $is_message =   !empty($extData['comjob']['is_message']) ? $extData['comjob']['is_message'] : 1;

            // 投递邮件通知
            $is_email   =   !empty($extData['comjob']['is_email']) ? $extData['comjob']['is_email'] : 1;

            // 视频面试申请，不需要发送邮件、短信。有另外的发送渠道
            $sqtype     =   isset($extData['sqtype']) ? $extData['sqtype'] : '';

            // 处理向企业发送短信、邮件
            if ($data['is_browse'] != 4 && $data['resume_state'] == 1 && ($this->config['sy_email_set'] == 1 || $this->config['sy_msg_isopen'] == 1) && $sqtype == '') {
                if ($is_link == 1) {

                    $job_link   =   $this->select_once('company', array('uid' => $comid), '`linkmail` as email,`linktel` as link_moblie');
                } elseif ($is_link == 2) {

                    $linkIdInfo =   $this->select_once('company_job', array('id' => $jobid), 'link_id');

                    $job_link   =   $this->select_once('company_job_link', array('id' => $linkIdInfo['link_id']), '`email`,`link_moblie`');
                }else if ($is_link == 3){

                    $linkIdInfo =   $this->select_once('company_job', array('id' => $jobid), 'link_id');

                    if (!empty($linkIdInfo) && (int)$linkIdInfo['link_id'] > 0){

                        $job_link   =   $this->select_once('company_job_link', array('id' => $linkIdInfo['link_id']), '`email`,`link_moblie`');
                    }else{

                        $job_link   =   $this->select_once('company', array('uid' => $comid), '`linkmail` as email,`linktel` as link_moblie');
                    }
                }
                if ($this->config['sy_email_set'] == 1 && $this->config['sy_email_sqzw'] == 1 && !empty($job_link['email']) && $is_email == 1) {
                    include_once('resume.model.php');
                    $resumeM    =   new resume_model($this->db, $this->def);
                    $Info       =   $resumeM->getInfoByEid(array('eid' => $eid));
                    // 简历模糊化
                    $resumeCheck=   $this->config['resume_open_check'] == 1 ? 1 : 2;

                    global $phpyun;
                    $phpyun->assign('Info', $Info);
                    $phpyun->assign('resumeCheck', $resumeCheck);

                }
            }

            //5.0推送
            if ($data['resume_state'] == 1 && $data['is_browse'] != 4) {

                include_once('push.model.php');
                $pushM  =   new push_model($this->db, $this->def);
                $pushM->pushMsg('jobNewResume', array('fuid' => $uid, 'puser' => $comid, 'tid' => $nid, 'jobname' => $data['job_name']));

                // 记录会员日志
                $logContent =   '职位申请：申请企业「'.$data['com_name'].'」 → 职位《'.$data['job_name'].'》';
                $this->addMemberLog($uid, 1, $logContent, 6, 1);

                //微信
                include_once('weixin.model.php');
                $Weixin =   new weixin_model($this->db, $this->def);
                $res = $Weixin->sendWxJob($uid, $jobid);
                if (!$res || $this->config['wx_xxtz']!='1' || !$this->config['wx_mbsqzw'] || $this->config['sy_msg_tz'] == 1){
                    include_once('notice.model.php');
                    $noticeM        =   new notice_model($this->db, $this->def);
                    if ($this->config['sy_msg_isopen'] == 1 && $this->config['sy_msg_sqzw'] == 1 && !empty($job_link['link_moblie']) && $is_message == 1) {

                        $msgdata    =   array(
                            'uid'       =>  $comid,
                            'name'      =>  $data['com_name'],
                            'cuid'      =>  '',
                            'cname'     =>  '',
                            'type'      =>  'sqzw',
                            'jobname'   =>  $data['job_name'],
                            'date'      =>  date('Y-m-d'),
                            'moblie'    =>  $job_link['link_moblie'],
                            'port'      =>  '2',
                            'xcx_id'    =>  $data['eid'],
                            'xcx_type'  =>  'resume'
                        );
                        $noticeM->sendSMSType($msgdata);
                    }
                    if ($this->config['sy_email_set'] == 1 && $this->config['sy_email_sqzw'] == 1 && !empty($job_link['email']) && $is_email == 1) {
                        $contents   =   $phpyun->fetch(TPL_PATH . 'resume/sendresume.htm', time());

                        //发送email记录到数据表email_msg
                        $emaildata  =   array(

                            'email'     =>  $job_link['email'],
                            'subject'   =>  "您收到一份新的求职简历！——" . $this->config['sy_webname'],
                            'content'   =>  $contents,
                            'uid'       =>  $comid,
                            'name'      =>  $data['com_name'],
                            'cuid'      =>  '',
                            'cname'     =>  '',
                            'tbContent' =>  '简历详情eid:' . $eid
                        );
                        $noticeM->sendEmail($emaildata);
                    }
                }
            }

            // 处理申请统计
            include_once('statis.model.php');
            $statisM    =   new statis_model($this->db, $this->def);
            $statisM->upInfo(array('sq_job' => array('+', 1)), array('uid' => $comid, 'usertype' => 2));
            $statisM->upInfo(array('sq_jobnum' => array('+', 1)), array('uid' => $uid, 'usertype' => 1));

            // 申请职位预警提示
            if ($this->config['warning_sendresume']>0){

                include_once('warning.model.php');
                $warningM   =   new warning_model($this->db, $this->def);
                $warningM->warning(6, $data['uid']);
            }
            if ($this->config['warning_sqjob']>0){

                include_once('warning.model.php');
                $warningM   =   new warning_model($this->db, $this->def);
                $warningM->warning(6, $data['uid']);
            }

        }
        return $nid;
    }

    /**
     * @desc    修改申请职位记录
     * @param   array $Where
     * @param   array $data
     * @return  $return
     */
    function updSqJob($Where = array(), $data = array()){
        return $this->update_once('userid_job', $data, $Where);
    }

    /**
     * @desc 申请职位
     *
     * @param array $data 【uid usertype 申请人】 【job_id 职位id】 【eid 简历id】
     * @param string $sqtype spview:视频面试预约
     * @return array|int[] $return
     * @throws SmartyException
     */
    function applyJob($data = array(), $sqtype = '')
    {
        $res                =   array();
        $res['errorcode']   =   8;
        $res['msg']         =   '';
        $res['url']         =   '';

        if(empty($data['uid']) || empty($data['usertype'])){
            $res['msg']         =   '请先登录！';
            $res['url']         =   'index.php?c=login';
            $res['errorcode']   =   1;
            $res['showlogin']   =   1;
            return $res;
        }

        if($data['usertype'] != 1){
            $res['msg']         =   '您不是个人用户！';
            $res['errorcode']   =   2;
            return $res;
        }

        //投递数量
        $row    =   $this->select_once('userid_job', array('uid' => $data['uid'], 'job_id' => $data['job_id'], 'orderby' => 'id,desc'));

        $uid    =   $data['uid'];

        if($row){

            if($sqtype == 'spview'){    //视频面试预约已投递过简历的直接预约成功

                return array('errorcode'=>9);
            }else{

                if($row['isdel'] == 9 && $row['is_browse']!='6'){

                    $res['errorcode']   =   3;
                    $res['msg']	        =	'您已经投递过该职位，请不要重复投递！';
                    return $res;
                }else{

                    $sq_resume_interval = $this->config['sq_resume_interval'];
                    if ($sq_resume_interval && $sq_resume_interval > 0) {
                        if(strtotime("+{$sq_resume_interval} day", $row['datetime']) >= time()){
                            $res['errorcode']   =   31;
                            $res['msg']	        =	'您最近申请过该职位，请勿重复投递！';
                            return $res;
                        }
                    }
                }
            }
        }

        $msNum  =   $this->getYqmsNum(array('uid' => $data['uid'], 'jobid' => $data['job_id'], 'isdel' => 9));
        if(intval($msNum) > 0){

            $res['errorcode']   =   4;
            $res['msg']	        =	'您已经收到该公司的面试邀请，请不要重复投递！';

            if($sqtype == 'spview'){    //视频面试预约已被邀请面试的直接预约成功

                return array('errorcode'=>9);
            }else{

                return $res;
            }
        }

        //简历详情
        $resume = $resumess	=	array();
        $efield = '`id`,`uid`,`status`, `uname`, `integrity`, `state`,`exp`,`edu`,`sex`,`birthday`,`city_classid`';
        if (!empty($data['eid'])) {

            $resume =   $this->select_once('resume_expect', array('id' => $data['eid'], 'uid' => $data['uid']), $efield);
        } else {

            $resume =   $this->select_once('resume_expect', array('uid' => $data['uid'], 'defaults' => 1), $efield);

            if (empty($resume['id'])) {

                $userinfo   =   $this->select_once('resume', array('uid' => $data['uid']), 'def_job');
                $resume     =   $this->select_once('resume_expect', array('uid' => $data['uid'], 'id' => $userinfo['def_job']), $efield);
            }
        }

        //判断简历
        if(empty($resume['id'])){

            $res['msg']         =   '您还没有合适的简历，请先添加简历！';
            $res['url']         =   'member/index.php?c=addresume';
            $res['errorcode']   =   12;
            return $res;
        }

        $info               =   $this -> getInfo(array('id' => $data['job_id']));


        if($sqtype == ''){
            // 判断简历基本信息必填项是否完善
            $base = array();
            if (empty($resume['uname'])){
                $base[] = '姓名';
            }
            if (empty($resume['edu'])){
                $base[] = '学历';
            }
            if (empty($resume['exp'])){
                $base[] = '工作经验';
            }
            if (!empty($base)){
                $res['eid']     =   $resume['id'];
                $res['msg']	    =	'请完善您的基本信息：'.implode('、', $base);
                $res['url']	    =	'member/index.php?c=info';
                $res['wapurl']  =   'member/index.php?c=info';
                $res['errorcode']=  101;
                return $res;
            }
            // 判断求职意向必填项是否完善
            if (empty($resume['city_classid'])){
                $res['eid']     =   $resume['id'];
                $res['msg']	    =	'请完善您的求职意向：工作城市';
                $res['url']	    =	'member/index.php?c=expect&e='.$resume['id'];
                $res['wapurl']  =   'member/index.php?c=addexpect&eid='.$resume['id'];
                $res['errorcode']=  102;
                return $res;
            }
            if($this->config['user_sqintegrity'] && $resume['integrity'] < $this->config['user_sqintegrity']){

                // 验证跨行投递简历完整度
                $jobType[] = $info['job1'];
                $jobType[] = $info['job1_son'];
                $jobType[] = $info['job_post'];
                $jobType =  array_filter($jobType);

                $checkUserSqintegrity = false; // 是否校验完整度
                $jobclassList = $this->select_all('resume_jobclass', array('eid' => $resume['id']));
                $jobClassIds = array();
                foreach ($jobclassList as $key=>$item){
                    $jobClassIds[] = $item['job1'];
                    $jobClassIds[] = $item['job1_son'];
                    $jobClassIds[] = $item['job_post'];
                }
                $jobClassIds = array_filter(array_unique($jobClassIds));
                $sy_resume_job_classid = @explode(',', $this->config['sy_resume_job_classid']);
                // 判断当前简历职位类别是否存在后台配置中
                if(!array_intersect($sy_resume_job_classid, $jobClassIds)) {
                    if ($this->config['resume_create_exp'] == 1 && $this->config['expcreate']) { // 工作经验
                        $expcreate = @explode(',', $this->config['expcreate']);
                        $checkUserSqintegrity = in_array($resume['exp'], $expcreate); // 存在不校验
                    }
                    if ($checkUserSqintegrity && $this->config['resume_create_edu'] == 1 && $this->config['educreate']) { // 学历
                        $educreate = @explode(',', $this->config['educreate']);
                        $checkUserSqintegrity = in_array($resume['edu'], $educreate); // 存在不校验
                    }
                    if ($checkUserSqintegrity && $this->config['resume_create_project'] == 1) { // 项目
                        $project  =  $this->select_num('resume_project',array('eid'=>$resume['id'],'uid'=>$data['uid']));
                        $checkUserSqintegrity = ($project < 1) ? false : true;
                    }
                    if ($checkUserSqintegrity && $sy_resume_job_classid) { // 后台非必填项职位类别配置
                        $checkUserSqintegrity = array_intersect($sy_resume_job_classid, $jobClassIds) ? true : false;
                    }
                } else {
                    $checkUserSqintegrity = true;
                    // 存在校验 当前投递行业是否与当前简历职位类别是否有交集
                    if($this->config['sy_resume_kh_td'] == 1){
                        $checkUserSqintegrity = array_intersect($jobType, $jobClassIds) ? true : false;
                    }
                }
                 if (!$checkUserSqintegrity) {
                    $res['msg']     =   '该简历完整度未达到'.$this->config['user_sqintegrity'].'%,请先完善简历！';
                    $res['url']     =   'member/index.php?c=resume';
                    $res['errorcode']=  7;
                    return $res;
                }
            }elseif($resume['state'] == 0 && $this->config['sy_shresume_applyjob']!='1'){
                $res['errorcode']   =   11;
                $res['msg']	    =	'简历正在审核中，请联系管理员';
                $res['url']	    =	'member/index.php?c=resume';
                return $res;
            }elseif($resume['state'] == 2){
                $res['errorcode']   =   11;
                $res['msg']	    =	'简历被举报，请联系管理员';
                $res['url']	    =	'member/index.php?c=resume';
                return $res;
            }elseif($resume['state'] == 3){
                $res['errorcode']   =   11;
                $res['msg']	    =	'简历未通过审核，请联系管理员';
                $res['url']	    =	'member/index.php?c=resume';
                return $res;
            }elseif($resume['status']=='2'){
                $res['msg']     =   '请先公开您的简历！';
                $res['url']     =   'member/index.php?c=privacy';
                $res['errorcode']=  10;
                return $res;
            }
        }

        if(empty($info)){
            $res['msg']	    =	'该职位不存在';
            $res['url']	    =	'index.php?c=resume';
            $res['errorcode']=  6;
            return $res;
        }else if ($info['status'] == 1){

            $res['msg']	    =	'该职位已下架';
            $res['url']	    =	'index.php?c=resume';
            $res['errorcode']=  6;
            return $res;
        }

        if ($sqtype == ''){

            //投递要求检测
            $exp_reqs   =   $info['exp_req'] > 0 ? $info['exp_req'] : '';
            $edu_reqs   =   $info['edu_req'] > 0 ? $info['edu_req'] : '';
            $sex_reqs   =   $info['sex_req'] > 0 ? $info['sex_req'] : '';
            $minage_req =   $info['minage_req'] > 0 ? $info['minage_req'] : '';
            $maxage_req =   $info['maxage_req'] > 0 ? $info['maxage_req'] : '';

            //  是否满足工作经历需求
            if($exp_reqs){

                $sexp = $this->select_once('userclass', array('id' => $exp_reqs), '`sort`');
                $rexp = $this->select_once('userclass', array('id' => $resume['exp']), '`sort`');

                if($this->config['sqjob_req']==0){
                    if(!empty($rexp)){
                        if($rexp['sort']<$sexp['sort']){
                            $value['is_browse'] = 4;//标记为不合适
                            $remark = "您的工作经验不符合投递要求";
                        }
                    }else{
                        $value['is_browse'] = 4;//标记为不合适
                        $remark = "您的工作经验不符合投递要求";
                    }
                }elseif($this->config['sqjob_req']==1){
                    if(!empty($rexp)){
                        if($rexp['sort']<$sexp['sort']){
                            $return['errorcode']  = 13;
                            $return['msg']      = '您的工作经验不符合投递要求';
                            return $return;
                        }
                    }else{
                        $return['errorcode']  = 13;
                        $return['msg']      = '您的工作经验不符合投递要求';
                        return $return;
                    }
                }
            }

            //  是否满足教育经历需求
            if($edu_reqs){

                $sedu = $this->select_once('userclass', array('id' => $edu_reqs), '`sort`');
                $redu = $this->select_once('userclass', array('id' => $resume['edu']), '`sort`');

                if($this->config['sqjob_req']==0){
                    if (!empty($redu)) {
                        if ($redu['sort'] < $sedu['sort']) {
                            $value['is_browse'] = 4;//标记为不合适
                            $remark .= " 您的学历不符合投递要求";
                        }
                    } else {
                        $value['is_browse'] = 4;//标记为不合适
                        $remark .= " 您的学历不符合投递要求";
                    }
                }elseif($this->config['sqjob_req']==1) {
                    if (!empty($redu)) {
                        if ($redu['sort'] < $sedu['sort']) {
                            $return['errorcode'] = 13;
                            $return['msg'] = '您的学历不符合投递要求';
                            return $return;
                        }
                    } else {
                        $return['errorcode'] = 13;
                        $return['msg'] = '您的学历不符合投递要求';
                        return $return;
                    }
                }
            }

            //  是否满足性别要求
            if ($sex_reqs) {
                if ($this->config['sqjob_req'] == 0) {
                    if ($resume['sex'] != $sex_reqs && $resume['sex'] != 3 && $sex_reqs != 3) {

                        $value['is_browse'] = 4;//标记为不合适
                        $remark .= " 您的性别不符合投递要求";
                    }
                } elseif($this->config['sqjob_req']==1) {
                    if ($resume['sex'] != $sex_reqs && $resume['sex'] != 3 && $sex_reqs != 3) {

                        $return['errorcode'] = 13;
                        $return['msg'] = '您的性别不符合投递要求';
                        return $return;
                    }
                }
            }

            //  是否满足年龄要求
            if ($minage_req || $maxage_req) {
                if ($resume['birthday']) {

                    $a = date('Y', strtotime($resume['birthday']));
                    $age = date("Y") - $a;
                }
                if ($this->config['sqjob_req'] == 0) {
                    if (($minage_req && $age < $minage_req) || ($maxage_req && $age > $maxage_req)) {

                        $value['is_browse'] = 4;//标记为不合适
                        $remark .= " 您的年龄不符合投递要求";
                    }
                } elseif($this->config['sqjob_req']==1) {
                    if (($minage_req && $age < $minage_req) || ($maxage_req && $age > $maxage_req)) {

                        $return['errorcode'] = 13;
                        $return['msg'] = '您的年龄不符合投递要求';
                        return $return;
                    }
                }
            }

            //  预警设置：禁止投递
            if ($this->config['warning_sendresume_type'] == 2 && $this->config['warning_sendresume'] > 0){

                $sqNum  =   $this->select_num('userid_job', array('uid' => $data['uid'], 'datetime' => array('>=', strtotime('today'))));
                if (intval($sqNum) >= intval($this->config['warning_sendresume'])){

                    $return['errorcode'] = 13;
                    $return['msg'] = !empty($this->config['warning_sendresume_tips']) ? $this->config['warning_sendresume_tips'] : '您今日投递次数过多，请明日再试！';
                    return $return;
                }
            }

            //  预警设计：禁止投递
            if ($this->config['warning_sqjob_type'] == 2 && $this->config['warning_sqjob'] > 0){

                $sqList =   $this->select_all('userid_job', array('uid' => $data['uid'], 'datetime' => array('>=', strtotime('today')), 'limit' => 500), 'job_id');
                if (!empty($sqList)){

                    $sqJobIds   =   array();
                    foreach ($sqList as $sqk => $sqv){
                        $sqJobIds[] =   $sqv['job_id'];
                    }

                    $sqJobList  =   $this->select_all('company_job', array('id' => array('in', pylode(',', $sqJobIds))), 'job1');

                    $jobClassIdArr  =   array();
                    foreach ($sqJobList as $jk => $jv) {
                        if (!in_array($jv['job1'], $jobClassIdArr)){
                            $jobClassIdArr[]    =   $jv['job1'];
                        }
                    }

                    if (count($jobClassIdArr) > $this->config['warning_sqjob']){

                        $return['errorcode'] = 13;
                        $return['msg'] = !empty($this->config['warning_sqjob_tips']) ? $this->config['warning_sqjob_tips'] : '您今日跨行投递次数过多，请明日再试！';
                        return $return;
                    }
                }
            }
        }

        $value['remark']        =   $remark;
        $value['job_id']        =	$data['job_id'];
        $value['com_name']	    =	$info['com_name'];
        $value['job_name']	    =	$info['name'];
        $value['com_id']	    =	$info['uid'];
        $value['uid']		    =	$data['uid'];
        $value['did']		    =	$data['did'];
        $value['eid']		    =	$resume['id'];
        $value['resume_state']  =   $resume['state'];
        $value['datetime']	    =	time();

        $nid				    =   $this -> addSqJob($value, array('comjob'=>$info, 'sqtype'=>$sqtype));

        if(!empty($nid)){

            $res['errorcode']   =   9;
            $res['msg']         =   '投递成功！';
            $res['nid'] = $nid;
            return $res;
        }else{

            $res['msg']         =   '投递失败！';
            $this->addErrorLog($uid,3,$res['msg']);
            $res['errorcode']   =   2;
            return $res;
        }
    }

    //申请猎头职位
    function applyLtJob($data=array()){
        $arr    =   array('errorcode' => 8, 'msg' => '');

        if($data['usertype']!=1){

            $arr['msg']  =   '您不是个人用户！';
        }else{
            $user   =   $this -> select_once('resume_expect',array('uid'=>$data['uid'],'height_status'=>2),'`id`');

            if(!is_array($user)){
                $arr['msg'] =   '您没有优质简历！';
            }else{

                $jobid  =   (int)$data['job_id'];
                $type   =   (int)$data['type'];

                $row    =   $this -> select_once('userid_job',array('uid'=>$data['uid'],'isdel'=>9,'job_id'=>$jobid,'type'=>$type));
                if(is_array($row)){
                    $arr['msg'] =   '您已经申请过该职位！';
                }else{
                    $job        =   $this -> select_once('lt_job',array('id'=>$jobid,'status'=>1),"`job_name`,`com_name`,`id`,`uid`");
                    if($job['id']){
                        $udata  =   array(
                            'uid'       =>  $data['uid'],
                            'did'       =>  $this->config['did'],
                            'job_id'    =>  $jobid,
                            'job_name'  =>  $job['job_name'],
                            'com_name'  =>  $job['com_name'],
                            'com_id'    =>  $job['uid'],
                            'type'      =>  $type,
                            'eid'       =>  $user['id'],
                            'datetime'  =>  time(),
                        );
                        $this->insert_into("userid_job", $udata);
                        $this->update_once('member_statis', array('sq_jobnum' => array('+', 1)), array('uid' => $data['uid']));
                        $this->addMemberLog($data['uid'], $data['usertype'], '职位申请：申请猎头职位'.$job['job_name'], 6, 1);

                        $arr['msg']     	=   '申请成功！';
                        $arr['errorcode']	=   9;
                    }else{
                        $arr['msg']			=   '该职位待处理中！';
                    }
                }
            }
        }
        return $arr;
    }

    // 添加邀请面试数据
    public function addYqmsInfo($yqdata = array())
    {
        $arr	=	array(
            'status' => 0,
            'msg' => ''
        );

        if (empty($yqdata['fuid']) || empty($yqdata['fusername'])) {

            $arr['msg']		=	'请先登录企业账号！';
            $arr['login']	=	2;
            return $arr;
        }

        if($yqdata['fusertype'] != 2){
            $arr['login']	=	2;
            $arr['msg']		=	'很抱歉，只有企业账号才能够邀请面试！';
            return $arr;
        }

        // 判断邀请时间
        $intertime	=	strtotime($yqdata['intertime']);
        if (empty($intertime)) {
            $arr['msg']		=	'面试时间不能为空！';
            return $arr;
        }
        if ($intertime < time()) {
            $arr['msg']		=	'面试时间不能小于当前时间！';
            return $arr;
        }
        if (empty($yqdata['linktel'])) {
            $arr['msg']		=	'联系方式不能为空！';
            return $arr;
        }

        // if (empty($yqdata['longitude']) || empty($yqdata['latitude'])) { // 多个地方调用不能有此判断
        //     $arr['msg']		=	'面试地址坐标不能为空！';
        //     return $arr;
        // }
        if (empty($yqdata['address'])) {
            $arr['msg']		=	'面试地址不能为空！';
            return $arr;
        }

        $jobtype	=	intval($yqdata['jobtype']);

        if ($jobtype == '' || $jobtype < 2) {

            $jobtype = 0;
        }

        $uid	=	$yqdata['fuid'];
        $spid	=	$yqdata['spid'];

        $data	=	array(

            'uid'		=>	$yqdata['uid'],
            'title'		=>	'面试邀请',
            'content'	=>	$yqdata['content'],
            'fid'		=>	$uid,
            'datetime'	=>	time(),
            'address'	=>	$yqdata['address'],
            'intertime'	=>	$yqdata['intertime'],
            'linkman'	=>	$yqdata['linkman'],
            'linktel'	=>	$yqdata['linktel'],
            'jobid'		=>	intval($yqdata['jobid']),
            'jobname'	=>	$yqdata['jobname'],
            'x'     	=>	$yqdata['longitude'],
            'y'     	=>	$yqdata['latitude'],
            'mappic'    =>  !empty($yqdata['mappic'])?$yqdata['mappic']:''
        );

        $info	=	array(
            'linkman'   =>  $yqdata['linkman'],
            'linktel'   =>  $yqdata['linktel'],
            'intertime'	=>	$yqdata['intertime'],
            'address'	=>	$yqdata['address'],
            'jobname'	=>	$yqdata['jobname'],
            'username'	=>	$yqdata['username'],
            'content'	=>	$yqdata['content']
        );




        $p_uid	=	$yqdata['uid'];

        $lt_num =	$this -> select_num('lt_job', array('uid' => $uid, 'status' => 1, 'zp_status' => 0, 'id' => $data['jobid'] ));
        $num	=	$this -> getJobNum(array('uid' => $uid, 'state' => 1, 'status' => 0, 'r_status' => 1, 'id' => $data['jobid']));

        // 判断职位数量
        if ($num < 1 && $lt_num < 1) {
            $arr['status']	=	4;
            $arr['msg']		=	'职位信息错误，请重新选择！';
            return $arr;
        }

        // 是否在黑名单
        $black	=	$this -> select_num('blacklist', array('c_uid' => $p_uid, 'p_uid' => $uid));

        if (!empty($black)) {
            $arr['msg']	 =	'该用户暂不接受面试邀请！';
            return $arr;
        }

        // 查看是否邀请过
        $umessage	=	$this -> getYqmsInfo(array('uid' => $p_uid, 'fid' => $uid, 'type' => $jobtype,'isdel'=>9));
        if (! empty($umessage)) {
            $arr['msg']	=	'已经邀请过该人才，请不要重复邀请！';
            return $arr;
        }

        $com	=	$this->select_once('company', array('uid' => $uid), '`name`, `did`');

        $resume =	$this->select_once('resume', array('uid' => $p_uid), '`name`, `def_job`,`uid`');

        $data['did']	=	$com['did'];
        $data['fname']	=	$com['name'];

        //保存更新邀请模板
        if($yqdata['save_yqmb']=='1'){

            include_once('yqmb.model.php');

            $yqmbM  =  new yqmb_model($this->db, $this->def);

            $ymwhere = array();

            if($yqdata['ymid']){

                $ymwhere['id'] = $yqdata['ymid'];

            }

            $ydata               =   array(
                'uid'           =>  $uid,
            );

//            $job    =   $this -> select_once('company_job',array('id'=>$setData['jobid']),'`name`');

            $ymdata              =   array(
                'content'       =>  $yqdata['content'],
                'address'       =>  $yqdata['address'],
                'linkman'       =>  $yqdata['linkman'],
                'linktel'       =>  $yqdata['linktel'],
                'intertime'     =>  $yqdata['intertime'],
                'did'           =>  $com['did'],
                'name'          =>  $data['jobname'].'邀请面试模板',
            );
            $yqmbM -> addInfo($ymdata,$ydata,$ymwhere);
        }
        //保存邀请模板end

        $auto	=	false;

        include_once ('integral.model.php');
        $inteM		=	new integral_model($this->db, $this->def);

        include_once ('statis.model.php');
        $statisM	=	new statis_model($this->db, $this->def);

        $statisField	=	array('field' => '`rating`,`vip_etime`,`invite_resume`,`rating_type`,`integral`', 'usertype' => 2);

        $suid	=	$spid ? $spid : $uid;

        $row	=	$statisM->getInfo($suid, $statisField);

        // 判断会员是否可用
        if (isVip($row['vip_etime'])) {

            if ($row['rating_type'] == 1) { // 套餐模式

                if ($row['invite_resume'] == 0) { // 收费会员邀请简历已用完

                    if ($this->config['integral_interview'] == 0 && in_array('invite', @explode(',', $this->config['com_single_can']))){

                        $vid    =   $this->addYqms($data);
                        if (!$vid) {
                            $this->addErrorLog($uid, 7, '邀请面试失败！');
                        }

                        $arr['status'] = 3;
                    }else{
                        if (empty($spid)) {

                            if ($this->config['com_integral_online'] == 3) { // 积分模式

                                if ($row['integral'] >= $this->config['integral_interview']) {

                                    $vid	=	$this->addYqms($data);
                                    if(!$vid){
                                        $this->addErrorLog($uid,7,'邀请面试失败！');
                                    }
                                    // 积分操作记录
                                    $inteM -> company_invtal($yqdata['fuid'], 2, $this->config['integral_interview'], $auto, $this->config['integral_pricename'].'抵扣，邀请会员面试', true, 2, 'integral', 14);

                                    $arr['status'] = 3;
                                }

                            } else {

                                $arr['status']	=	2;
                            }

                        } else {

                            $arr['msg'] = '当前账户套餐余量不足，请联系主账户增配！';
                        }
                    }
                } else {

                    // 收费会员简历没有用完的状态,直接邀请
                    $vid	=	$this->addYqms($data);
                    if(!$vid){
                        $this->addErrorLog($uid,7,'邀请面试失败！');
                    }
                    // 计算消费数量
                    $statisM -> upInfo(array('invite_resume' => array('-', 1)), array('uid' => $suid, 'usertype' => 2));

                    $payDetail = '邀请面试，消耗邀请套餐数量：1';
                    $this->addStatisDetail(array('uid' => $uid, 'type' => 4, 'num' => 1, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));

                    $arr['status'] = 3;
                }
            } else { // 时间模式

                $vid	=	$this->addYqms($data);

                if(!$vid){
                    $this->addErrorLog($uid,7,'邀请面试失败！');
                }

                $payDetail = '邀请面试，消耗今日邀请套餐数量：1';
                $this->addStatisDetail(array('uid' => $uid, 'type' => 4, 'num' => 1, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));

                $arr['status'] = 3;
            }
        }

        if ($arr['status'] == 3) {

            $arr['vid']	=	$vid;

            // 记录会员日志
            $this->addMemberLog($yqdata['fuid'], $yqdata['fusertype'], '面试邀请：邀请人才：' . $resume['name'], 4, 1);

            // 5.0推送
            include_once('push.model.php');
            $pushM  =   new push_model($this->db, $this->def);
            $pushM->pushMsg('invite', array('fuid' => $yqdata['fuid'], 'puser' => $resume['uid'], 'tid' => $vid, 'comname' => $com['name']));
            // 获取用户订阅企业的投递记录
            $userSubscribe = $this->select_once('userid_job', array('uid' => $yqdata['uid'], 'com_id' => $yqdata['fuid'], 'is_subscribe' => 1), '`id`');
            include_once ('weixin.model.php');
            $wxM = new weixin_model($this->db, $this->def);
            $needSend = 1;// 默认需要发送通知
            if (isset($userSubscribe['id']) && $userSubscribe['id']) {// 已订阅面试通知
                if (empty($yqdata['content'])){
                    $yqdata['content'] = '无';
                }
                $subscribeTpl = array(
                    "thing1" => array("value" => $yqdata['jobname']),// 职位名称
                    "thing6" => array("value" => $data['fname']),// 邀约人
                    "time3" => array("value"  => $yqdata['intertime']),// 面试时间
                    "thing4" => array("value" => $yqdata['address']),// 面试地点
                    "thing8" => array("value" => $yqdata['content']),// 温馨提示
                );
                $userInfo = $this->select_once('member', array('uid' => $data['uid']), '`uid`,`username`,`wxopenid`,`usertype`');
                $sendSubR = $wxM->sendWxSubscribe(array('uid' => $data['uid'],'wxid' => $userInfo['wxopenid'], 'url' => 'pson/pages/usermember/invite/index', 'tpl_data' => $subscribeTpl, 'tpl_type' => 2, 'utype' => $userInfo['usertype']));
                if ($sendSubR['errcode'] == 0) {// 订阅消息发送成功，无需发送其他通知
                    $needSend = 0;
                }
                $this->update_once('userid_job', array('is_subscribe' => 0), array('id' => $userSubscribe['id']));// 订阅消息发送后将投递记录改为未订阅
            }
            if ($needSend == 1) {
                // 微信模板消息通知
                $res = $wxM->sendWxresume($data);
                if (!$res || $this->config['wx_xxtz']!='1' || !$this->config['wx_mbyqms'] || $this->config['sy_msg_tz'] == 1){
                    // 发送邮件 短信通知
                    $info['xcx_id']  = $vid;
                    $info['address'] = $yqdata['address'];
                    $info['date']    = $yqdata['intertime'];
                    $this->msgPost($yqdata['uid'], $yqdata['fuid'], $info, $yqdata['port']);
                }
            }
            // 查询当前信息 修改职位申请状态为“看过” userid_job.is_browse = 2
            $row    =   $this->getSqJobInfo(array('job_id' => $yqdata['jobid'], 'com_id' => $yqdata['fuid'], 'eid' => $yqdata['eid'], 'isdel' => 9), array('field' => '`is_browse`,`is_subscribe`'));
            if ($row['is_browse'] < 2) {
                $jobuserdata = array('is_browse' => 2);
                $this->update_once('userid_job', $jobuserdata, array('id' => $row['id']));
            }
        }

        return $arr;
    }

    /**
     * 邀请面试发送邮件 短信
     */
    private function msgPost($uid, $comid, $row = array(), $port=null){
        $com                =   $this -> select_once('company', array('uid' => $comid), '`uid`,`name`,`linkman`,`linktel`,`linkmail`');
        $info               =   $this -> select_once('member', array('uid' => $uid), '`email`, `moblie`');
        $resume             =   $this -> select_once('resume', array('uid' => $uid), '`name`');

        $data['uid']        =   $uid;
        $data['name']       =   $resume['name'];
        $data['cuid']       =   $com['uid'];
        $data['cname']      =   $com['name'];
        $data['type']       =   "yqms";
        $data['company']    =   $com['name'];
        $data['linkman']    =   $row['linkman']?$row['linkman']:$com['linkman'];
        $data['comtel']     =   $row['linktel']?$row['linktel']:$com['linktel'];
        $data['comemail']   =   $com['linkmail'];
        $data['content']    =   @str_replace("\n","<br/>",$row['content']);
        $data['jobname']    =   $row['jobname'];
        $data['username']   =   $row['username']?$row['username']:$resume['name'];
        $data['email']      =   $info['email'];
        $data['moblie']     =   $info['moblie'];

        require_once ('notice.model.php');
        $noticeM			=   new notice_model($this->db, $this->def);
        $noticeM -> sendEmailType($data);
        $data['port']	=	$port;
        $data['xcx_type'] = 'invite';// 生成小程序地址页面类型
        $data['xcx_id']   = $row['xcx_id'];// 生成小程序目标页面数据id
        $data['address']  = $row['address'];// 面试地址
        $data['date']     = $row['date'];// 面试时间
        $noticeM -> sendSMSType($data);
    }

    /**
     * 通用的增加邀请面试
     */
    public function addYqms($data = array())
    {

        $return =   $this->insert_into('userid_msg', $data);
        if (!empty($data['uid'])) {

            //  新增：邀请面试，同步申请记录已邀请字段；更新标记为已查看
            $this->update_once('userid_job', array('invited' => 1, 'invite_time' => time(), 'is_browse' => 2), array('uid' => $data['uid'], 'com_id' => $data['fid']));
        }
        return $return;
    }

    //  面试邀请，单条查询
    function getYqmsInfo($where = array() , $data=array()) {

        if (!empty($where)) {

            $select  =  $data['field'] ? $data['field'] : '*';

            $info    =  $this->select_once('userid_msg',$where,$select);

            if ($info && is_array($info)) {
                if($data['yqh']){//查看邀请函
                    if($data['usertype']==1){

                        $this -> update_once("userid_msg",array('is_browse'=>2),array('id'=>$where['id'],'is_browse'=>1,'uid'=>$data['uid']));
                    }
                    $info['comname']		=	$info['fname'];
                    $info['datetime']		=	date('Y-m-d',$info['datetime']);
                }
                // 企业logo
                $ComInfo = $this->select_once('company', array('uid'=>$info['fid']), '`logo`,`logo_status`');
                if (!empty($ComInfo['logo']) && $ComInfo['logo_status']==0){
                    $info['com_logo_n']    	=	checkpic($ComInfo['logo']);
                }else{
                    $info['com_logo_n']    	=	checkpic($this->config['sy_unit_icon']);
                }

                $info['com_url'] = Url('wap',array('c'=>"company","a"=>"show","id"=>$info['fid']));
                //百度静态图
                if(!empty($info['mappic'])){

                    $info['staticimg']  =  checkpic($info['mappic']);
                }
            }
            return $info;
        };

    }
    function upYqms($where=array(),$updata=array()){

        if(!empty($where) && !empty($updata)){

            $this -> update_once("userid_msg",$updata,$where);

        }

    }
    //  面试邀请列表 ，多条查询
    public function getYqmsList($whereData,$data=array()) {

        $select = $data['field'] ? $data['field'] : '*';

        $List  =   $this   ->  select_all('userid_msg',$whereData,$select);
        $utype  =   $data['utype'] ? $data['utype'] : '';

        if (!empty($List) && $utype != 'simple') {

            foreach ($List as $k => $v){
                $jobids[]	=  	$v['jobid'];
            }

            $jobs	=	$this->select_all('company_job',array('id'=>array('in',pylode(',',$jobids))) , '`status`,`id`,`minsalary`,`maxsalary`');


            foreach ($List as $lk => $v){
                if($v['datetime']){
                    $List[$lk]['datetime_n']        =   date('Y-m-d',$v['datetime']);
                }
                $List[$lk]['intertime_n']           =   strtotime($v['intertime']);
                $List[$lk]['ms_time']               =   date('Y.m.d H:i', strtotime($v['intertime']));
                $List[$lk]['over']                  =   time() > strtotime($v['intertime']) ? 1 : 0;

                foreach($jobs as $jk=>$jv){
                    if($jv['id']==$v['jobid']){
                        $List[$lk]['jobstatus']     =	$jv['status'];
                        $List[$lk]['salary']        =   salaryUnit($jv['minsalary'], $jv['maxsalary']);
                    }
                }
            }
            $List   =   $this -> subYqmsListInfo($List, $data);
        }
        return $List;
    }

    // 邀请面试列表信息补充
    private function subYqmsListInfo($List, $data = array())
    {
        $uids  =  $comids  =  array();

        foreach ($List as $v){
            if($v['uid'] && !in_array($v['uid'],$uids)){
                $uids[]    =  $v['uid'];
            }

            if($v['fid'] && !in_array($v['fid'],$comids)){
                $comids[]  =  $v['fid'];
            }
        }
        //  查询个人姓名
        $rWhere['uid']            	=   array('in', pylode(',', $uids));
        $rData['field']             =   '`uid`,`name`,`nametype`,`sex`,`telphone`,`def_job`,`photo`';

        $resume                		=   $this -> getResumeList($rWhere, $rData);
        //查询面试评价
        $cmsgWhere['uid'] = array('in', pylode(',', $uids));
        $cmsgWhere['cuid'] = $data['uid'];
        $cmsg = $this->select_all('company_msg',$cmsgWhere);
        //  查询个人简历
        $reWhere['uid']             =   array('in', pylode(',', $uids));
        $reWhere['defaults']        =   '1';
        $reData['field']            =   '`id`,`uid`,`name`,`job_classid`,`minsalary`,`maxsalary`,`height_status`,`exp`,`edu`,`sex`,`birthday`';

        $expectList                 =   $this -> getResumeExpectList($reWhere, $reData);

        $dWhere['uid']              =   array('in', pylode(',', $uids));
        $dWhere['comid']          	=   $data['uid'];

        $downList                 	=   $this -> select_all('down_resume',$dWhere, '`uid`');

        $cWhere['uid']            	=   array('in', pylode(',', $comids));
        $cData['field']             =   '`uid`,`logo`,`logo_status`';
        $cData['logo']				=   1;
        $company					=	$this -> getComList($cWhere,$cData);

        foreach ($List  as  $k  =>  $v){

            if($v['isdel']==1){
                $List[$k]['isdel_n']  =   '个人用户删除';
            }else if($v['isdel']==2){
                $List[$k]['isdel_n']  =   '企业用户删除';
            }else{
                $List[$k]['isdel_n']  =   '正常';
            }
            if(!empty($cmsg)){
                foreach ($cmsg as $ke=>$va){
                    if($v['uid'] == $va['uid']){
                        $List[$k]['is_pl'] = $va['id'];
                    }
                }
            }else{
                $List[$k]['is_pl'] = 0;
            }
            $List[$k]['datetime_n']   =   date('Y-m-d H:i',$v['datetime']);
            foreach($resume as $val){

                if($v['uid'] == $val['uid']){
                    $List[$k]['name']       =   $val['name_n'];
                    $List[$k]['realname']   =   $val['username_n'];
                    $List[$k]['telphone']   =   $val['telphone'];
                    $List[$k]['photo']      =   $val['photo'];

                }
            }
            foreach ($expectList['list'] as $rv){

                if ($v['uid']   ==  $rv['uid']) {
                    $List[$k]['waprurl']    =   Url('wap',array('c'=>'resume','a'=>'show','id'=>$rv['id']));
                    $List[$k]['exp']        =   $rv['exp_n'];
                    $List[$k]['age']        =   $rv['age_n'];
                    $List[$k]['edu']        =   $rv['edu_n'];
                    $List[$k]['sex']        =   $rv['sex_n'];
                    if ($rv['job_classid'] != "") {
                        $List[$k]['jobclassname'] = $rv['job_classname'];
                    }
                    $List[$k]['eid']        =   $rv['id'];
                }
            }
            foreach($downList as $va){
                if ($v['uid']   ==  $va['uid']) {
                    $List[$k]['down']="1";
                }
            }
            foreach($company['list'] as $val){
                if($v['fid'] == $val['uid']){
                    $List[$k]['logo']      =  $val['logo'];
                }
            }
            if ($data['utype'] == 'admin') {
                $List[$k]['job_url'] = Url('job', array('c' => 'comapply', 'id' => $v['jobid'], 'look' => 'admin'));
                $List[$k]['com_url'] = Url('company', array('c' => 'show', 'id' => $v['fid'], 'look' => 'admin'));
            }
        }

        return $List;

    }

    /**
     * @desc     删除邀请面试记录
     * @param    $id
     * @param    array $data
     * @return   $return
     */
    function delYqms($id = null , $data = array()) {

        $return         =       array();

        if(!empty($id) || !empty($data['where'])){

            $where      =       array();

            if (!empty($id)) {

                if(is_array($id)){

                    $ids    =	$id;

                    $return['layertype']	=	1;

                }else{

                    $ids        =   @explode(',', $id);
                    $return['layertype']	=	0;

                }

                $ids            =   pylode(',', $ids);

                $where['id']    =   array('in', $ids);

            }

            if (!empty($data['where'])) {

                $where          =   array_merge($where, $data['where']);

            }elseif($data['utype']!='admin'){

                if($data['usertype'] == '1'){
                    $where['uid']		=	$data['uid'];
                }elseif($data['usertype'] == '2'){
                    $where['fid']	=	$data['uid'];
                }
            }
            if($data['norecycle'] == '1'){		//	数据库清理，不插入回收站

                $return['id']	=	$this -> delete_all('userid_msg', $where, $data['limit'],'','1');
            }else{

                if($data['utype']!='admin'){
                    $return['id']   =   $this -> update_once("userid_msg",array('isdel'=>$data['usertype']),$where);
                }else{
                    $return['id']   =   $this -> delete_all('userid_msg', $where, '');
                }

            }

            $this -> addMemberLog($data['uid'],$data['usertype'],"面试邀请：删除邀请信息",4,3);

            $return['msg']		=	'邀请面试记录(ID:'.pylode(',', $id).')';

            $return['errcode']	=	$return['id'] ? '9' :'8';
            $return['msg']		=	$return['id'] ? $return['msg'].'删除成功！' : $return['msg'].'删除失败！';

        }else{
            $return['msg']		=	'请选择您要删除的数据！';
            $return['errcode']	=	8;
        }

        return	$return;
    }
    //  面试邀请数目
    function getYqmsNum($Where = array()){
        return $this->select_num('userid_msg', $Where);
    }

    /**
     * @desc     修改邀请面试状态
     * @param array $arr
     * @return array
     */
    function setYqms($arr = array()) {
        $id				=	intval($arr['id']);
        $browse			=	intval($arr['browse']);
        $uid			=	intval($arr['uid']);
        if($id){
            $dataV = array('is_browse'=>$browse);
            if($arr['remark']){
                $dataV['remark'] = $arr['remark'];
            }

            $nid		=	$this -> update_once("userid_msg",$dataV,array("id"=>$id,"uid"=>$uid));

            $comuid		=	$this -> getYqmsInfo(array("id"=>$id),array("field"=>'`fid`,`jobid`,`linktel`,`linkman`'));

            $company	=	$this -> getComInfo($comuid['fid'],array('field'=>'linkmail,linkman,linktel'));

            $resume		=	$this -> select_once('resume',array("uid"=>$uid),'name');

            $data['uid']		=	$comuid['fid'];
            $data['cname']		=	$arr['username'];
            $data['type']		=	"yqmshf";
            $data['cuid']		=	$uid;
            $data['cusername']	=	$resume['name'];

            if($browse==3){

                $data['typemsg']	=	'同意';
                $msg_content 		= 	'用户 <a href="usertpl,'.$uid.'">'.sub_string($arr['username']).' </a>同意了您的邀请面试！';

                include_once('sysmsg.model.php');
                $sysmsgM  			=  new sysmsg_model($this->db, $this->def);
                $sysmsgM -> addInfo(array('uid'=>$comuid['fid'],'usertype'=>2,'content'=>$msg_content));
            }elseif($browse==4){

                $data['typemsg']	=	'拒绝';
            }
            if($this->config['sy_msg_yqmshf']=='1' && $company["linktel"] && checkMsgOpen($this -> config)){

                $data["moblie"] 	=	$company["linktel"];
            }
            if($this->config['sy_email_yqmshf']=='1' && $company["linkmail"] && $this->config['sy_email_set']=="1"){

                $data["email"]		=	$company["linkmail"];
            }
            if($data["email"] || $data['moblie']){

                $data['name']		=	$comuid['linkman'];
                require_once ('notice.model.php');
                $noticeM			=   new notice_model($this->db, $this->def);
                $noticeM -> sendEmailType($data);
                $noticeM -> sendSMSType($data);
            }
            if($nid){

                return array('msg'=>'操作成功！','errcode'=>9);
            }else{

                return array('msg'=>'操作失败！','errcode'=>8);
            }
        }
    }
    /**
     * @desc    取消申请职位
     * @param   array $Where
     * @param   array $data
     * @return  $return
     */
    function qxSqJob($arr = array()){

        $id			=	intval($arr['id']);
        $uid		=	intval($arr['uid']);
        $usertype	=	intval($arr['usertype']);

        $nid=$this -> updSqJob(array('id'=>$id,'uid'=>$uid),array('body'=>$arr['body']));
        if($nid){

            $this->addMemberLog($uid,$usertype,"职位申请：取消职位申请（ID：".$id."）",6,3);
            return array('msg'=>'取消成功！','errcode'=>9);
        }else{
            return array('msg'=>'取消失败！','errcode'=>9);
        }
    }
    //  浏览职位，单条查询
    function getLookJobInfo($where, $data=array()) {

        $info  =  $this->select_once('look_job',$where);

        if (!empty($info)) {
            if (isset($data['utype'])){
                require_once ('resume.model.php');
                $resumeM  =  new resume_model($this->db, $this->def);

                $info['name']     =  $resumeM->getUnameByUid($info['uid'], array('comid'=>$info['com_id'],'usertype'=>2));
                $info['datetime'] =  date('Y-m-d H:i',$info['datetime']);;
            }
        }

        return $info;
    }

    //  浏览职位列表 ，多条查询
    public function getLookJobList($whereData,$data=array()) {

        $select =	$data['field'] ? $data['field'] : '*';
        $List	=   $this   ->  select_all('look_job',$whereData,$select);

        if (!empty($List)) {

            $List   =   $this -> subLookJobListInfo($List,$data);

        }

        return $List;
    }

    // 浏览职位列表信息补充
    private function subLookJobListInfo($List,$data) {

        foreach ($List as $v){

            $jobids[]       =   $v['jobid'];
            $uids[]         =   $v['uid'];
        }

        if(!empty($data)){
            $uid			=	intval($data['uid']);
            $usertype		=	intval($data['usertype']);
        }

        /* 提前职位名称，公司名称 */
        $jWhere['id']       =   array('in', pylode(',', $jobids));
        $jData['field']     =   '`id`,`name`,`com_name`,`provinceid`,`cityid`,`status`,`minsalary`,`maxsalary`,`edu`,`exp`,`com_logo`';
        $jobList            =   $this -> getList($jWhere, $jData);

        //  查询个人姓名
        $rWhere['uid']		=   array('in', pylode(',', $uids));
        $rData['field']		=   '`uid`,`name`,`nametype`,`sex`,`telphone`,`def_job`,`photo`';
        $resumeList			=   $this -> getResumeList($rWhere, $rData);


        //  查询个人简历
        $reWhere['uid']		=   array('in', pylode(',', $uids));
        $reWhere['defaults']=   '1';
        $reData['field']	=   '`id`,`uid`,`name`,`job_classid`,`minsalary`,`maxsalary`,`height_status`,`exp`,`edu`,`sex`,`birthday`,`status`';
        $expectList			=   $this -> getResumeExpectList($reWhere, $reData);

        if(!empty($expectList['list']) && $data['utype'] != 'admin'){
            $euids			=	array();
            foreach ($expectList['list'] as $val){
                $euids[]	=	$val['uid'];
            }
        }

        $userid_msg			=	$this -> select_all('userid_msg',array('fid'=>$uid,'uid'=>array('in', pylode(',', $uids)),'isdel'=>9),"uid");
        $userid_job			=   $this -> select_all('userid_job',array('com_id'=>$uid,'uid'=>array('in',pylode(',',$uids)),'isdel'=>9),'`uid`,`is_browse`');

        foreach ($List  as  $k  =>  $v){

            $List[$k]['wapjob_url'] =   Url('wap',array('c'=>'job','a'=>'comapply','id'=>$v['jobid']));
            $List[$k]['wapcom_url'] =   Url('wap',array('c'=>'company','a'=>'show','id'=>$v['com_id']));
            $List[$k]['datetime_n'] =   formatTime($v['datetime']);
            foreach ($jobList['list'] as $val){

                if ($v['jobid']   ==  $val['id']) {

                    $List[$k]['job_name']       =   $val['name'];
                    $List[$k]['com_name']       =   $val['com_name'];
                    $List[$k]['cityname']       =   $val['job_city_one'];
                    if($val['job_city_two']){
                        $List[$k]['cityname']   .=  '-'.$val['job_city_two'];
                    }
                    $List[$k]['salary']     =   $val['job_salary'];
                    $List[$k]['exp_n']      =   $val['job_exp'] ? '经验不限' : $val['job_exp'];
                    $List[$k]['edu_n']      =   $val['job_edu'] ? '学历不限' : $val['job_edu'];
                    if($val['status']=="1"){
                        $List[$k]['status'] =   "已下架招聘";
                    }else{
                        $List[$k]['status'] =   "正在招聘";
                    }
                    $List[$k]['com_logo_n'] =   $val['com_logo_n'];
                }
            }


            foreach($resumeList as $val){

                if($v['uid'] == $val['uid']){
                    $List[$k]['name']       =  $val['name_n'];
                    $List[$k]['username']   =  $val['username_n'];
                    $List[$k]['photo']      =  $val['photo'];
                }
            }

            foreach ($expectList['list'] as $val){

                if ($v['uid']   ==  $val['uid']) {
                    $List[$k]['waprurl']    =   Url('wap',array('c'=>'resume','a'=>'show','id'=>$val['id']));
                    $List[$k]['eid']        =   $val['id'];
                    $List[$k]['exp']        =   $val['exp_n'];
                    $List[$k]['edu']        =   $val['edu_n'];
                    $List[$k]['sex']        =   $val['sex_n'];
                    $List[$k]['age']        =   $val['age_n'];
                    $List[$k]['salary_n']        =   $val['salary'];
                    $List[$k]['name_n']        =   $val['name'];
                    $List[$k]['resume_status'] = $val['status'];
                    if ($val['job_classid'] != "") {
                        $List[$k]['jobclassidname'] = $val['job_classname'];
                    }
                }
            }

            foreach($userid_msg as $val){
                if($val['uid']==$v['uid'])
                {
                    $List[$k]['userid_msg']=1;
                }
            }

            foreach($userid_job as $val){

                if($v['uid']==$val['uid']){
                    $List[$k]['is_browse']		=	$val['is_browse'];
                }
            }

            if($data['utype']!='admin' && !in_array($v['uid'], $euids)){
                unset($List[$k]);
            }
            if ($data['utype'] == 'admin') {
                $List[$k]['job_url'] = Url('job', array('c'=>'comapply','id'=>$v['jobid'], 'look' => 'admin'));
                $List[$k]['com_url'] = Url('company', array('c'=>'show','id'=>$v['com_id'], 'look' => 'admin'));
                $List[$k]['datetime_n_n'] = $v['datetime'] ? date('Y-m-d H:i', $v['datetime']) : '';
            }
        }

        return $List;
    }

    /**
     * @desc     删除浏览职位记录
     * @param    $id
     * @param    array $data
     * @return   $return
     */
    function delLookJob($id = null , $data = array()) {

        $return                 =   array();

        if(!empty($id) || !empty($data['where'])){

            $where              =   array();

            if (!empty($id)) {

                if(is_array($id)){

                    $ids        =	$id;

                    $return['layertype']	=	1;

                }else{

                    $ids        =   @explode(',', $id);

                    $return['layertype']	=	0;

                }

                $ids            =   pylode(',', $ids);

                $where['id']    =   array('in', $ids);

            }

            if (!empty($data['where'])) {

                $where          =   array_merge($where, $data['where']);

            }

            if($data['usertype'] == '2'){

                $where['com_id']    =  intval($data['uid']);

                $return['id']		=	$this -> update_once('look_job',array('com_status' => 1), $where);

                $this -> addMemberLog($data['uid'],$data['usertype'],"浏览记录：删除已浏览简历记录（ID:".pylode(',', $id)."）",26,3);
                $return['msg']		=	'浏览的职位记录(ID:'.pylode(',', $id).')';
            }elseif($data['usertype'] == '1'){

                $where['uid']    =  intval($data['uid']);
                $return['id']		=	$this -> update_once('look_job',array('status' => 1), $where);
                $this -> addMemberLog($data['uid'],$data['usertype'],"浏览记录：删除职位浏览记录（ID:".pylode(',', $id)."）",26,3);
                $return['msg']		=	'职位浏览记录(ID:'.pylode(',', $id).')';
            }else{
                if($data['norecycle'] == '1'){	//	数据库清理，不插入回收站

                    $return['id']		=	$this -> delete_all('look_job', $where, $data['limit'], '', '1');
                }else{

                    $return['id']	=	$this -> delete_all('look_job', $where, '');
                }
                $return['msg']		=	'职位浏览记录(ID:'.pylode(',', $id).')';
            }

            $return['errcode']		=	$return['id'] ? '9' :'8';
            $return['msg']			=	$return['id'] ? $return['msg'].'删除成功！' : $return['msg'].'删除失败！';
        }else{

            $return['msg']			=	'请选择您要删除的数据！';
            $return['errcode']		=	8;
        }
        return	$return;
    }

    /**
     * @desc     新增浏览职位记录
     * @param    array $data
     * @return   $return
     */
    function addLookJob($data = array(), $utype = ''){

        if (!empty($data['uid']) && !empty($data['jobid'])){

            $num  =  $this->select_num('look_job', array('uid' => $data['uid'], 'jobid' => $data['jobid']));

            if ($num > 0){
                // 已浏览过的，修改浏览时间
                $this->update_once('look_job', array('datetime' => $data['datetime'], 'status' => 0),array('uid' => $data['uid'], 'jobid' => $data['jobid']));

                include_once('warning.model.php');
                $warningM   =   new warning_model($this->db, $this->def);
                $warningM->warning(8, $data['uid']);
            }else{

                $sql  =  array(
                    'uid'       =>  $data['uid'],
                    'jobid'     =>  $data['jobid'],
                    'datetime'  =>  $data['datetime'],
                    'did'       =>  !empty($data['did']) ? $data['did'] : 0,
                    'ip'        =>  fun_ip_get()
                );

                if (!empty($data['com_id']) && !empty($data['jobname'])){

                    $job['uid']     =  $data['com_id'];
                    $job['name']    =  $data['jobname'];

                    $sql['com_id']  =  $data['com_id'];
                }else{

                    $job  =  $this->select_once('company_job',array('id'=>$data['jobid']),'`uid`,`name`');

                    $sql['com_id']  =  $job['uid'];
                }

                $expect  =  $this->select_once('resume_expect', array('uid' => $data['uid'], 'defaults' => 1, 'r_status' => 1), '`id`');

                if(!empty($expect)){
                    $result  =  $this->insert_into('look_job', $sql);

                    include_once('warning.model.php');
                    $warningM   =   new warning_model($this->db, $this->def);
                    $warningM->warning(8, $data['uid']);

                    return $result;
                }
            }
        }
    }

    /**
     * 浏览职位数目
     * @param array $Where
     * @param array $data
     * @return array|bool|false|int|string|void
     */
    function getLookJobNum($Where = array(), $data = array())
    {
        return $this->select_num('look_job', $Where);
    }

    /**
     * 收藏职位
     * @param array $data
     *  uid usertype 申请人
     *  job_id       职位id
     * @return array|mixed $return
     */
    function collectJob($data = array('jobtype' => ''))
    {

        $res    =   array(
            'errorcode' =>  8,
            'msg'       =>  '',
            'url'       =>  ''
        );

        //判断是否登录
        if (empty($data['uid']) || empty($data['usertype'])) {

            $res['msg']         =   '请先登录！';
            $res['url']         =   'index.php?c=login';
            $res['errorcode']   =   8;
            $res['state']       =   0;
            return $res;
        }

        //判断是否为个人
        if ($data['usertype'] != 1) {
            $res['msg']         =   '您不是个人用户！';
            $res['errorcode']   =   8;
            $res['state']       =   4;
            return $res;
        }

        if ($data['jobtype'] == 'lt') {

            $res    =   $this->collectLtJob($data);
        } else {

            $res    =   $this->collectComJob($data);
        }
        return $res;
    }

    /**
     * @desc 收藏猎头职位
     * @param array $data
     * @return mixed
     */
    private function collectLtJob($data=array())
    {

        $lt_job =   $this->select_once('lt_job', array('id' => (int)$data['job_id']), '`id`,`com_name`,`job_name`,`uid`,`usertype`');
        $is_set =   $this->getFavJob(array('uid' => $data['uid'], 'job_id' => $data['job_id'], 'type' => $lt_job['usertype']));

        if(!empty($is_set)){

            $res['msg']       =  '您已经收藏过该职位，请不要重复收藏！';
            $res['errorcode'] =  8;
            $res['state']     =  3;
        }else{

            $value  =   array(
                'job_id'    =>  (int)$data['job_id'],
                'job_name'  =>  $lt_job['job_name'],
                'com_id'    =>  $lt_job['uid'],
                'uid'       =>  $data['uid'],
                'datetime'  =>  time(),
                'com_name'  =>  $lt_job['com_name'],
                'type'      =>  $lt_job['usertype']
            );
            $nid    =   $this -> addFavJob($value);
            if($nid){

                //修改统计数量
                include_once ('statis.model.php');
                $statisM  =  new statis_model($this->db, $this->def);
                $statisM->upInfo(array('fav_jobnum'    =>  array('+', 1)), array('uid' => $data['uid']));

                $this->addMemberLog($data['uid'],$data['usertype'],"职位收藏：收藏了猎头职位:".$lt_job['job_name'],5,1);//会员日志
                $res['msg']        =  '收藏成功！';
                $res['errorcode']  =  9;
                $res['state']	   =  1;
            }else{
                $res['msg']        =  '收藏失败！';
                $res['errorcode']  =  8;
                $res['state']	   =  2;
            }
        }
        return $res;
    }
    private function collectComJob($data=array())
    {

        $is_set =   $this->getFavJob(array('uid' => $data['uid'], 'job_id' => $data['job_id']));

        if(!empty($is_set)){

            $res['msg']       =  '您已经收藏过该职位，请不要重复收藏！';
            $res['errorcode'] =  8;
            $res['state']     =  3;
        }else{

            $job                =   $this -> getInfo(array('id' => $data['job_id']));
            $value['job_id']    =   $job['id'];
            $value['com_name']  =   $job['com_name'];
            $value['job_name']  =   $job['name'];
            $value['com_id']    =   $job['uid'];
            $value['uid']       =   $data['uid'];
            $value['datetime']  =   time();
            $nid                =   $this -> addFavJob($value);

            if(!empty($nid)){
				$expect  =  $this->select_once('resume_expect', array('uid' => $data['uid'], 'defaults' => 1, 'r_status' => 1), '`id`');
                //修改统计数量
                include_once ('statis.model.php');
                $statisM  =  new statis_model($this->db, $this->def);
                $statisM->upInfo(array('fav_jobnum'=>array('+', 1)), array('uid'=>$data['uid'], 'usertype' => 1));
                $member   =  $this->select_once("member",array("uid"=>$data['uid']),"`username`");
                //记录系统日志
                $this -> addSystem(array('uid' => $job['uid'],'usertype'=>2, 'content' => '用户<a href="resumetpl,'. $expect['id'] . '">' . sub_string($member['username']).' </a>收藏了您的职位：'.$job['name']));

                $res['msg']        =  '收藏成功！';
                $res['errorcode']  =  9;
                $res['state']	   =  1;
            }else{

                $res['msg']        =  '收藏失败！';
                $res['errorcode']  =  8;
                $res['state']	   =  2;
            }
        }
        return $res;
    }

    /**
     * @desc  新增收藏记录
     * @param array $data
     * @return bool $return
     */
    function addFavJob($data = array()){
        return $this->insert_into('fav_job', $data);
    }

    /**
     * 收藏职位数目
     * @param array $Where
     * @return array|bool|false|string|void
     */
    function getFavJobNum($Where = array()) {
        return $this->select_num('fav_job', $Where);
    }
    /**
     * 面试邀请数
    */
    function getInviteNum($Where=array()){
        return $this->select_num('userid_msg', $Where);
    }
    // 收藏职位
    function getFavJob($Where = array()) {
        return $this->select_once('fav_job', $Where);
    }
    //  申请职位列表 ，多条查询
    function getFavJobList($whereData,$data=array()) {
        $select = $data['field'] ? $data['field'] : '*';

        $List  =   $this   ->  select_all('fav_job',$whereData,$select);
        if (!empty($List)) {
            if($data['datatype']=='moreinfo'){//更多详细信息

                $resume_uid = array();

                foreach ($List as $v){
                    if($v['type']==1){

                        $com_jobid[]=   $v['job_id'];
                    }else{
                        $lt_jobid[] =   $v['job_id'];
                    }
                    if($v['uid']){
                        $resume_uid[] = $v['uid'];
                    }
                }
                //  职位lt_job
                require_once ('lietoujob.model.php');
                $ltjobM 					=	new lietoujob_model($this->db, $this->def);

                $ltjobWhere['id']           =   array('in', pylode(',', $lt_jobid));
                $ltjobData['field']			=   '`id`,`minsalary`,`maxsalary`,`provinceid`,`cityid`,`status`,`exp`,`edu`';
                $ltjobList					=	$ltjobM -> getList($ltjobWhere,$ltjobData);

                //  职位company_job
                $jobWhere['id']				=   array('in', pylode(',', $com_jobid));
                $jobData['field']			=   '`id`,`minsalary`,`maxsalary`,`provinceid`,`cityid`,`state`,`status`,`exp`,`edu`,`com_logo`';

                $jobList					=   $this -> getList($jobWhere, $jobData);

                $StateNameList=array('0'=>'等待审核','1'=>'招聘中','2'=>'已结束','3'=>'未通过');

                require_once ('resume.model.php');
                $resumeM                     =   new resume_model($this->db, $this->def);
                $rWhere['uid']               =   array('in', pylode(',', $resume_uid));
                $rData['field']              =   '`uid`,`name`,`nametype`,`sex`,`telphone`,`def_job`,`photo`';
                $resumeList                  =   $resumeM -> getResumeList($rWhere, $rData);

                foreach ($List  as  $k  =>  $v){

                    $List[$k]['datetime_n']             =   formatTime($v['datetime']);
                    $List[$k]['statename']				=	'已关闭';
                    foreach($jobList['list'] as $val){
                        if($v['job_id']==$val['id']){
                            $List[$k]['job_edu']		=	$val['job_edu'] == '不限' ? '不限学历' : $val['job_edu'];
                            $List[$k]['job_exp']		=	$val['job_exp'] == '不限' ? '不限经验' : $val['job_exp'];
                            $List[$k]['salary']			=	$val['job_salary'];
                            $List[$k]['cityname']		=	$val['job_city_one'];
                            if($val['job_city_two']){
                                $List[$k]['cityname']	.=	'-'.$val['job_city_two'];
                            }
                            $List[$k]['statename']		=	$StateNameList[$val['state']];
                            if($val['status'] == 1){
                                $List[$k]['statename']	= 	'已下架';
                            }
                            $List[$k]['com_logo_n']     =   $val['com_logo_n'];
                            $List[$k]['wapjob_url'] = Url('wap',array('c'=>'job','a'=>'comapply','id'=>$v['job_id']));
                            $List[$k]['wapcom_url'] = Url('wap',array('c'=>'company','a'=>'show','id'=>$v['com_id']));
                        }
                    }

                    foreach($ltjobList as $val){
                        if($v['job_id']==$val['id']){
                            $List[$k]['salary']			=	$val['salary'];
                            $List[$k]['cityname']		=	$val['city_one_n'];
                            if($val['city_two_n']){
                                $List[$k]['cityname']	.=	'-'.$val['city_two_n'];
                            }

                            $List[$k]['statename']		=	$StateNameList[$val['status']];
                        }
                    }
                    foreach ($resumeList as $rval){
                        if ($v['uid']   ==  $rval['uid']) {
                            $List[$k]['username_n'] =    $rval['username_n'];
                            //开启隐私号模式 一律不显示联系方式
                            if($this -> config['sy_privacy_open'] != '1'){
                                $List[$k]['telphone']   =    $rval['telphone'];
                            }
                        }
                    }
                    if ($data['utype'] == 'admin') {
                        $job_url = '';
                        if ($v['type'] == 1) {
                            $job_url = Url('job', array('c' => 'comapply', 'id' => $v['job_id'], 'look' => 'admin'));
                        } elseif ($v['type'] == 2) {
                            $job_url = Url('lietou', array('c' => 'jobcomshow', 'id' => $v['job_id'], 'look' => 'admin'));
                        } elseif ($v['type'] == 3) {
                            $job_url = Url('lietou', array('c' => 'jobshow', 'id' => $v['job_id'], 'look' => 'admin'));
                        }
                        $com_url = '';
                        if ($v['type'] == 3) {
                            $com_url = Url('lietou', array('c' => 'headhunter', 'uid' => $v['com_id'], 'look' => 'admin'));
                        } else {
                            $com_url = Url('company', array('c' => 'show', 'id' => $v['com_id'], 'look' => 'admin'));
                        }
                        $List[$k]['job_url'] = $job_url;
                        $List[$k]['com_url'] = $com_url;
                        $List[$k]['telphone_url'] = Url('user', array('c' => 'users_member', 'a' => 'Imitate', 'uid' => $v['uid']), 'admin');
                        $List[$k]['datetime_n_n'] = $v['datetime'] ? date('Y-m-d H:i', $v['datetime']) : '';
                    }
                }
            }
        }
        return $List;
    }

    /**
     * @desc 职位详情页面，个人取消收藏操作
     * @param array $data
     * @return array
     */
    public function cancelFavJob($data = array())
    {

        if (!empty($data)){

            $result =   $this->delete_all('fav_job', array('job_id' => $data['job_id'], 'uid' => $data['uid']), '');

            if ($result){

                $this->update_once('member_statis', array('fav_jobnum' => array('-', 1)), array('uid' => intval($data['uid'])));
                $this->addMemberLog(intval($data['uid']), intval($data['usertype']), '收藏管理：取消收藏职位：'.$data['job_id'], 5, 3);

                $return =   array('errcode' => 9, 'msg' => '取消收藏成功');
            }else{

                $return =   array('errcode' => 9, 'msg' => '取消收藏失败');
            }
        }else{

            $return =   array('errcode' => 9, 'msg' => '参数错误，请重试');
        }

        return $return;
    }

    /**
     * @desc 删除个人收藏职位记录
     * @param $id
     * @param array $data
     * @return array $return
     */
    function delFavJob($id = null, $data = array('utype' => null))
    {

        $return = array();

        if (!empty($id)) {

            $where = array();

            if (!empty($id)) {

                if (is_array($id)) {

                    $ids = $id;
                    $return['layertype'] = 1;
                } else {

                    $ids = @explode(',', $id);
                    $return['layertype'] = 0;
                }

                $ids = pylode(',', $ids);
                $where['id'] = array('in', $ids);
                if ($data['utype'] == 'user') {
                    $where['uid'] = $data['uid'];
                }
            }

            if ($data['utype'] == 'admin') {

                $where['groupby'] = 'zid';

                $favjobs = $this->select_all('fav_job', $where, 'uid,count(*) as num');

                unset($where['groupby']);
            }

            $return['id']   =   $this->delete_all('fav_job', $where, '');

            if ($return['id']) {
                if ($data['utype'] == 'user') {

                    $this->update_once('member_statis', array('fav_jobnum' => array('-', 1)), array('uid' => intval($data['uid'])));
                    $this->addMemberLog(intval($data['uid']), intval($data['usertype']), '收藏管理：删除收藏职位记录(ID:' . $ids . ')', 5, 3);
                } else if ($data['utype'] == 'admin') {
                    if (!empty($favjobs)) {
                        foreach ($favjobs as $fk => $fv) {

                            $this->update_once('member_statis', array('fav_jobnum' => array('-', $fv['num'])), array('uid' => intval($fv['uid'])));
                        }
                    }
                }

                $return['errcode']  =   9;
                $return['msg']      =   '删除成功！';
            } else {

                $return['errcode']  =   8;
                $return['msg']      =   '删除失败！';
            }
        } else {

            $return['msg']          =   '请选择您要删除的数据！';
            $return['errcode']      =   8;
        }

        return $return;
    }
    /**
     *  @desc   获取缓存数据
     */
    private function getClass($options){

        if (!empty($options)){

            include_once ('cache.model.php');

            $cacheM     =   new cache_model($this->db, $this->def);

            $cache      =   $cacheM -> GetCache($options);

            return $cache;
        }
    }

    //更新职位点击率
    function addJobHits($id){

        if($this -> config['sy_job_hits'] > 100 || (isset($this -> config['sy_job_hits']) && $this -> config['sy_job_hits']<=1)){
            $hits       =   1;
        }else{
            $hits       =   mt_rand(1, $this->config['sy_job_hits']);
        }
        $this -> update_once('company_job', array('jobhits' => array('+', $hits), 'jobexpoure' => array('+', $hits)), array('id' => $id));
    }

    /**
     * 后台修改职位点击量、曝光量
     *
     * @param $id
     * @param $hits
     * @param $expoure
     * @return mixed
     */
    function upJobHits($id, $hits, $expoure)
    {
        if (!empty($id)) {

            $result =   $this->update_once('company_job', array('jobhits' => $hits, 'jobexpoure' => $expoure), array('id' => $id));

            if ($result) {

                $return['msg']      =   '修改成功！';
                $return['errcode']  =   9;
            } else {

                $return['msg']      =   '修改失败！';
                $return['errcode']  =   8;
            }
        } else {
            $return['msg']          =   '参数错误！';
            $return['errcode']      =   8;
        }
        return $return;
    }
    //职位推广：紧急 推荐 置顶 兼职推荐
    function jobPromote($uid, $data = array())
    {
        require_once ('statis.model.php');

        $StatisM    =   new statis_model($this->db, $this->def);

        $type       =   intval($data['type']);

        $suid       =   !empty($data['spid']) ? intval($data['spid']) : $uid;

        $statis     =   $StatisM -> getInfo($suid, array('usertype' => '2'));

        $online     =   (int)$this->config['com_integral_online'];   //  消费模式
        $integralPro=   (int)$this->config['integral_proportion'];   //  积分比例

        $topPirce   =   $this->config['integral_job_top'];      //  职位置顶金额
        $recPirce   =   $this->config['com_recjob'];            //  职位推荐金额
        $urgentPirce=   $this->config['com_urgent'];            //  职位紧急招聘金额

        $return     =   array();

        $comSingle	=	@explode(',', $this->config['com_single_can']);

        if ($type == 1) { // 置顶

                $return['single']	=	in_array('jobtop', $comSingle)? '1' : '2';

                if ($statis['top_num'] > 0 || $topPirce == 0) {

                    $return['status']   =   1;
                    $return['num']      =   $statis['top_num'];
                    $return['price']	=	$topPirce;
                } else {

                    if(empty($data['spid'])){

                        if ($online!=4) {

                            if ($online == 3 && !in_array('jobtop', explode(',', $this->config['sy_only_price']))) {

                                $return['jifen']    =   $topPirce * $integralPro;
                                $return['integral'] =   intval($statis['integral']);
                                $return['propor']   =   $integralPro;
                            }else{

                                $return['price']    =   $topPirce;
                                $return['integral'] =   intval($statis['integral']);
                            }

                        }else{
                            $return['price']        =   $topPirce;
                        }

                        $this->sendRatingNotice(array('uid' => $uid, 'type' => 5));

                        $return['msg']      =   "您的套餐已用完，您可以<a href='".$this->config['sy_weburl']."/wap/member/index.php?c=rating' style='color:red;cursor:pointer;'>购买会员</a>！";
                        $return['online']   =   $online;
                        $return['meal']     =   in_array('jobtop', explode(',', $this->config['sy_only_price'])) ? 1 : 0;
                        $return['status']   =   2;
                    }else{

                        $return['msg']      =   '当前账户套餐余量不足，请联系主账户增配！';
                    }

                }
            } else if ($type == 2) { // 推荐

                $return['single']	=	in_array('jobrec', $comSingle)? '1' : '2';

                if ($statis['rec_num'] > 0 || $recPirce == 0) {

                    $return['status']   =   1;
                    $return['num']      =   $statis['rec_num'];
                    $return['price']	=	$recPirce;
                } else {

                    if(empty($data['spid'])){

                        if ($online!=4) {

                            if ($online == 3 && !in_array('jobrec', explode(',', $this->config['sy_only_price']))) {

                                $return['jifen']    =   $recPirce * $integralPro;
                                $return['integral'] =   intval($statis['integral']);
                                $return['propor']   =   $integralPro;
                            }else{

                                $return['price']    =   $recPirce;
                                $return['integral'] =   intval($statis['integral']);
                            }

                        }else{
                            $return['price']    =   $recPirce;
                        }

                        $this->sendRatingNotice(array('uid' => $uid, 'type' => 6));

                        $return['msg']		=   "您的套餐已用完，您可以<a href='".$this->config['sy_weburl']."/wap/member/index.php?c=rating' style='color:red;cursor:pointer;'>购买会员</a>！";
                        $return['online']   =   $online;
                        $return['meal']     =   in_array('jobrec', explode(',', $this->config['sy_only_price'])) ? 1 : 0;
                        $return['status']   =   2;
                    }else{
                        $return['msg']      =   '当前账户套餐余量不足，请联系主账户增配！';
                    };
                }
            } else if ($type == 3) { // 紧急

                $return['single']	=	in_array('joburgent', $comSingle)? '1' : '2';

                if ($statis['urgent_num'] > 0 || $urgentPirce == 0) {

                    $return['status']   =   1;
                    $return['num']      =   $statis['urgent_num'];
                    $return['price']	=	$urgentPirce;
                } else {

                    if(empty($data['spid'])){

                        if ($online!=4) {

                            if ($online == 3 && !in_array('joburgent', explode(',', $this->config['sy_only_price']))) {

                                $return['jifen']    =   $urgentPirce * $integralPro;
                                $return['integral'] =   intval($statis['integral']);
                                $return['propor']   =   $integralPro;
                            }else{

                                $return['price']    =   $urgentPirce;
                                $return['integral'] =   intval($statis['integral']);
                            }

                        }else{
                            $return['price']        =   $urgentPirce;
                        }

                        $this->sendRatingNotice(array('uid' => $uid, 'type' => 7));

                        $return['msg']		=   "您的套餐已用完，您可以<a href='".$this->config['sy_weburl']."/wap/member/index.php?c=rating' style='color:red;cursor:pointer;'>购买会员</a>！";
                        $return['online']   =   $online;
                        $return['meal']     =   in_array('jobrec', explode(',', $this->config['sy_only_price'])) ? 1: 0;
                        $return['status']   =   2;
                    }else{
                        $return['msg']      =   '当前账户套餐余量不足，请联系主账户增配！';
                    };
                }
            } else if ($type == 4) { // 兼职推荐

                $return['single']	=	in_array('jobrec', $comSingle)? '1' : '2';

                if ($statis['rec_num'] > 0 || $recPirce == 0) {

                    $return['status']   =   1;
                    $return['num']      =   $statis['rec_num'];
                    $return['price']	=	$recPirce;
                } else {

                    if(empty($data['spid'])){

                        if ($online!=4) {

                            if ($online == 3 && !in_array('jobrec', explode(',', $this->config['sy_only_price']))) {

                                $return['jifen']    =   $recPirce * $integralPro;
                                $return['integral'] =   intval($statis['integral']);
                                $return['propor']   =   $integralPro;
                            }else{

                                $return['price']    =   $recPirce;
                                $return['integral'] =   intval($statis['integral']);
                            }
                        }else{

                            $return['price']        =   $recPirce;
                        }

                        $this->sendRatingNotice(array('uid' => $uid, 'type' => 6));

                        $return['msg']		=   "您的套餐已用完，您可以<a href='".$this->config['sy_weburl']."/wap/member/index.php?c=rating' style='color:red;cursor:pointer;'>购买会员</a>！";
                        $return['online']   =   $online;
                        $return['meal']     =   in_array('jobrec', explode(',', $this->config['sy_only_price'])) ? 1 : 0;
                        $return['status']   =   2;
                    }else{

                        $return['msg']      =   '当前账户套餐余量不足，请联系主账户增配！';
                    };
                }
            }

        $return['pricename']=   $this -> config['integral_pricename'];
        return $return;
    }

    /**
     * @desc 职位推广设置：置顶、推荐（含兼职）、紧急招聘、自动刷新
     * @param $id
     * @param array $data
     * @return array
     */
    function setJobPromote($id, $data = array()) {

        $return =   array();

        if (!empty($id) && !empty($data)) {

            $uid        =   intval($data['uid']);
            $spid       =   intval($data['spid']);

            $usertype   =   intval($data['usertype']);
            $type       =   trim($data['type']);
            $days       =   intval($data['days']);

            if($type == 'autojob'){

                $job    =   $this->select_all('company_job', array('id' => array('in', $id)), '`id`,`autotime`');
            }else if ($type == 'recpart'){

                $job    =   $this->select_once('partjob', array('id' => intval($id)), '`id`,`rec_time`');
            }else{

                $job    =   $this->getInfo(array('id' => intval($id)), array('field' => '`id`,`rec`,`rec_time`,`urgent`,`urgent_time`,`xsdate`'));
            }

            $suid   =   !empty($spid) ? $spid : $uid;

            $statis =   $this -> getStatisInfo($suid, array('usertype' => $usertype, 'field' => '`top_num`,`urgent_num`,`rec_num`'));

            $pData  =   array(

                'uid'   =>  $uid,
                'spid'  =>  $spid,
                'usertype'  =>  $usertype,
                'day'   =>  $days,
                'job'   =>  $job,
                'statis'=>  $statis
            );

            if ($type == 'top') {

                $return =   $this -> setTopPromote($pData);
            }else if($type == 'rec'){

                $return =   $this -> setRecPromote($pData);
            }else if($type == 'urgent'){

                $return =   $this -> setUrgentPromote($pData);
            }else if($type == 'autojob'){

                $return =   $this -> setAutoPromote($pData);
            }else if($type == 'recpart'){

                $return =   $this -> setRecPartPromote($pData);
            }

        } else {

            $return = array('errcode' => 8, 'msg' => '参数错误，请重试！');

        }

        return $return;
    }

    /**
     * @desc    职位置顶
     * @param array $data
     * @return array
     */
    private function setTopPromote($data = array())
    {

        $return =   array('errcode' => 8, 'msg' => '参数错误，请重试！');

        if (!empty($data)) {

            $uid    =   intval($data['uid']);
            $spid   =   intval($data['spid']);
            $suid   =   !empty($spid) ? $spid : $uid;
            $usertype   =   intval($data['usertype']);
            $day    =   intval($data['day']);
            $job    =   $data['job'];
            $statis =   $data['statis'];

            if ($statis['top_num'] >= $day || $this->config['integral_job_top'] == 0) {

                $xsDate =   $job['xsdate'] > time() ? array('+', $day * 86400) : time() + $day * 86400;

                $return['id']   =   $this->upInfo(array('xsdate' => $xsDate), array('id' => intval($job['id'])));

                $logContent =   '职位更新：设置职位置顶';
                $logDetail  =   '职位置顶天数 + '.$day;
                $this->addMemberLog($uid, $usertype, $logContent, 1, 4, $logDetail);

                if ($statis['top_num'] >= $day) {

                    $this->update_once('company_statis', array('top_num' => array('-', $day)), array('uid' => $suid));

                    $payDetail = '职位置顶，消耗置顶套餐数量：' . $day;
                    $this->addStatisDetail(array('uid' => $uid, 'type' => 7, 'num' => $day, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                } else if ($statis['top_num'] > 0) {

                    $this->update_once('company_statis', array('top_num' => 0), array('uid' => $suid));
                    $payDetail = '职位置顶，消耗置顶套餐数量：' . $statis['top_num'];
                    $this->addStatisDetail(array('uid' => $uid, 'type' => 7, 'num' => $statis['top_num'], 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                }

                $return['msg'] = '职位置顶设置成功！';
                $return['errcode'] = 9;
            } else {

                $return['msg'] = '您的套餐数据不足当前设置的置顶天数，请重新输入！';
                $return['errcode'] = 7;
            }
        }

        return $return;
    }

    /**
     * @desc    职位自动刷新
     * @param array $data
     * @return array
     */
    private function setAutoPromote($data = array())
    {

        $return =   array('errcode' => 8, 'msg' => '参数错误，请重试！');

        if (!empty($data)) {

            $uid    =   intval($data['uid']);
            $usertype   =   intval($data['usertype']);
            $day    =   intval($data['day']);
            $job    =   $data['job'];

            if ($this->config['job_auto'] == 0) {
                foreach ($job as $k => $v) {

                    $autotime   =   $v['autotime'] > time() ? array('+', $day * 86400) : time() + $day * 86400;
                    $this->upInfo(array('autotime' => $autotime), array('id' => intval($v['id'])));
                }

                $logContent =   '职位更新：设置职位自动刷新';
                $logDetail  =   '自动刷新天数 + '.$day;
                $this->addMemberLog($uid, $usertype, $logContent, 1, 4, $logDetail);

                $return['msg']      =   '职位自动刷新设置成功！';
                $return['errcode']  =   9;
            } else {

                $return['msg']      =   '系统参数错误！';
                $return['errcode']  =   7;
            }
        }

        return $return;
    }

    /**
     * @desc    职位推荐
     * @param array $data
     * @return array
     */
    private function setRecPromote($data = array())
    {

        $return =   array('errcode' => 8, 'msg' => '参数错误，请重试！');

        if (!empty($data)) {

            $uid    =   intval($data['uid']);
            $spid   =   intval($data['spid']);
            $suid   =   !empty($spid) ? $spid : $uid;
            $usertype   =   intval($data['usertype']);
            $day    =   intval($data['day']);
            $job    =   $data['job'];
            $statis =   $data['statis'];

            if ($statis['rec_num'] >= $day || $this->config['com_recjob'] == 0) {

                $recDate    =   $job['rec_time'] > time() ? $job['rec_time'] + $day * 86400 : time() + $day * 86400;
                $this->upInfo(array('rec_time' => $recDate, 'rec' => 1), array('id' => intval($job['id'])));

                $logContent =   '职位更新：设置职位推荐';
                $logDetail  =   '职位推荐天数 + '.$day;
                $this->addMemberLog($uid, $usertype, $logContent, 1, 4, $logDetail);

                if ($statis['rec_num'] >= $day) {

                    $this->update_once('company_statis', array('rec_num' => array('-', $day)), array('uid' => $suid));

                    $payDetail = '职位推荐，消耗推荐套餐数量：' . $day;
                    $this->addStatisDetail(array('uid' => $uid, 'type' => 5, 'num' => $day, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                } else if ($statis['rec_num'] > 0) {

                    $this->update_once('company_statis', array('rec_num' => 0), array('uid' => $suid));

                    $payDetail = '职位推荐，消耗推荐套餐数量：' . $statis['rec_num'];
                    $this->addStatisDetail(array('uid' => $uid, 'type' => 5, 'num' => $statis['rec_num'], 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                }

                $return['msg'] = '职位推荐设置成功！';
                $return['errcode'] = 9;
            } else {

                $return['msg'] = '您的套餐数据不足当前设置的推荐天数，请重新输入！';
                $return['errcode'] = 7;
            }
        }

        return $return;
    }

    /**
     * @desc 职位紧急招聘
     * @param array $data
     * @return array
     */
    private function setUrgentPromote($data = array())
    {

        $return =   array('errcode' => 8, 'msg' => '参数错误，请重试！');

        if (!empty($data)) {

            $uid    =   intval($data['uid']);
            $spid   =   intval($data['spid']);
            $suid   =   !empty($spid) ? $spid : $uid;
            $usertype   =   intval($data['usertype']);
            $day    =   intval($data['day']);
            $job    =   $data['job'];
            $statis =   $data['statis'];

            if ($statis['urgent_num'] >= $day || $this->config['com_urgent'] == 0) {

                $urgentDate =   $job['urgent_time'] > time() ? $job['urgent_time'] + $day * 86400 : time() + $day * 86400;
                $this->upInfo(array('urgent_time' => $urgentDate, 'urgent' => 1), array('id' => intval($job['id'])));

                $logContent =   '职位更新：设置职位紧急招聘';
                $logDetail  =   '职位紧急招聘天数 + '.$day;
                $this->addMemberLog($uid, $usertype, $logContent, 1, 4, $logDetail);

                if ($statis['urgent_num'] >= $day) {

                    $this->update_once('company_statis', array('urgent_num' => array('-', $day)), array('uid' => $suid));

                    $payDetail = '紧急招聘，消耗紧急招聘套餐数量：' . $day;
                    $this->addStatisDetail(array('uid' => $uid, 'type' => 6, 'num' => $day, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                } else if ($statis['urgent_num'] > 0) {

                    $this->update_once('company_statis', array('urgent_num' => 0), array('uid' => $suid));

                    $payDetail = '紧急招聘，消耗紧急招聘套餐数量：' . $statis['urgent_num'];
                    $this->addStatisDetail(array('uid' => $uid, 'type' => 6, 'num' => $statis['urgent_num'], 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                }

                $return['msg'] = '职位紧急招聘设置成功！';
                $return['errcode'] = 9;
            } else {

                $return['msg'] = '您的套餐数据不足当前设置的紧急招聘天数，请重新输入！';
                $return['errcode'] = 7;
            }
        }

        return $return;
    }

    /**
     * @desc    兼职推荐
     * @param array $data
     * @return array
     */
    private function setRecPartPromote($data = array())
    {

        $return =   array('errcode' => 8, 'msg' => '参数错误，请重试！');

        if (!empty($data)) {

            $uid    =   intval($data['uid']);
            $spid   =   intval($data['spid']);
            $suid   =   !empty($spid) ? $spid : $uid;
            $usertype   =   intval($data['usertype']);
            $day    =   intval($data['day']);
            $part   =   $data['job'];
            $statis =   $data['statis'];

            if ($statis['rec_num'] >= $day || $this->config['com_recjob'] == 0) {

                $recDate    =   $part['rec_time'] > time() ? $part['rec_time'] + $day * 86400 : time() + $day * 86400;
                $this->update_once('partjob', array('rec_time' => $recDate), array('id' => intval($part['id'])));

                $logContent =   '兼职更新：设置职位推荐';
                $logDetail  =   '兼职推荐天数 + '.$day;
                $this->addMemberLog($uid, $usertype, $logContent, 9, 4, $logDetail);

                if ($statis['rec_num'] >= $day) {

                    $this->update_once('company_statis', array('rec_num' => array('-', $day)), array('uid' => $suid));

                    $payDetail  =   '兼职推荐，消耗推荐套餐数量：'.$day;
                    $this->addStatisDetail(array('uid' => $uid, 'type' => 5, 'num' => $day, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                } else if ($statis['rec_num'] > 0) {

                    $this->update_once('company_statis', array('rec_num' => 0), array('uid' => $suid));

                    $payDetail  =   '兼职推荐，消耗推荐套餐数量：'.$statis['rec_num'];
                    $this->addStatisDetail(array('uid' => $uid, 'type' => 5, 'num' => $statis['rec_num'], 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                }

                $return['msg'] = '兼职推荐设置成功！';
                $return['errcode'] = 9;
            } else {

                $return['msg'] = '您的套餐数据不足当前设置的推荐天数，请重新输入！';
                $return['errcode'] = 7;
            }
        }

        return $return;
    }

    /**
     * @desc 关闭职位推广设置：置顶、推荐（含兼职）、紧急招聘
     * @param $id
     * @param array $data
     * @return array
     */
    function closeJobPromote($id, $data = array()) {

        $return =   array();

        if ($this->config['tg_back'] == 1) {

            if (!empty($id) && !empty($data)) {

                $uid        =   intval($data['uid']);
                $usertype   =   intval($data['usertype']);
                $type       =   trim($data['type']);

                if ($type == 'recpart') {

                    $job    =   $this->select_once('partjob', array('id' => intval($id)), '`id`, `uid`,`rec_time`');
                    if (isset($job['rec_time']) && $job['rec_time'] > time()) {

                        $endDay             =   ceil(($job['rec_time'] - strtotime(date('Y-m-d')) - 86400) / 86400);
                        $job['rec_day']     =   $endDay - 1;
                    }
                } else {

                    $job = $this->getInfo(array('id' => intval($id)), array('field' => '`id`,`uid`,`rec`,`rec_time`,`urgent`,`urgent_time`,`xsdate`'));

                    if (isset($job['xsdate']) && $job['xsdate'] > time()) {

                        $endDay             =   ceil(($job['xsdate'] - strtotime(date('Y-m-d')) - 86400) / 86400);
                        $job['top_day']     =   $endDay - 1;
                    }
                    if (isset($job['rec']) && isset($job['rec_time']) && $job['rec'] == 1 && $job['rec_time'] > time()) {

                        $endDay             =   ceil(($job['rec_time'] - strtotime(date('Y-m-d')) - 86400) / 86400);
                        $job['rec_day']     =   $endDay - 1;
                    }
                    if (isset($job['urgent']) && isset($job['urgent_time']) && $job['urgent'] == 1 && $job['urgent_time'] > time()) {

                        $endDay             =   ceil(($job['urgent_time'] - strtotime(date('Y-m-d')) - 86400) / 86400);
                        $job['urgent_day']  =   $endDay - 1;
                    }
                }


                if ($type == 'top') {
                    $this->update_once('company_job', array('xsdate' => ''), array('id' => $id));
                    if ($job['top_day'] > 0) {
                        $this->update_once('company_statis', array('top_num' => array('+', intval($job['top_day']))), array('uid' => $job['uid']));
                        $logContent =   '取消职位置顶（ID：'.$id.'）';
                        $logDetail  =   '返还置顶套餐：'.$job['top_day'].'天';
                    }else{
                        $logContent =   '取消职位置顶（ID：'.$id.'）';
                    }
                    $opera  =   1;
                } else if ($type == 'rec') {
                    $this->update_once('company_job', array('rec_time' => '', 'rec' => 0), array('id' => $id));
                    if ($job['rec_day'] > 0) {
                        $this->update_once('company_statis', array('rec_num' => array('+', intval($job['rec_day']))), array('uid' => $job['uid']));
                        $logContent =   '取消职位推荐（ID：'.$id.'）';
                        $logDetail  =   '返还推荐套餐：'.$job['rec_day'].'天';
                    }else{
                        $logContent =   '取消职位推荐（ID：'.$id.'）';
                    }
                    $opera  =   1;
                } else if ($type == 'urgent') {
                    $this->update_once('company_job', array('urgent_time' => '', 'urgent' => 0), array('id' => $id));
                    if ($job['urgent_day'] > 0) {
                        $this->update_once('company_statis', array('urgent_num' => array('+', intval($job['urgent_day']))), array('uid' => $job['uid']));
                        $logContent =   '取消职位紧急招聘（ID：'.$id.'）';
                        $logDetail  =   '返还紧急招聘套餐：'.$job['urgent_day'].'天';
                    }else{
                        $logContent =   '取消职位紧急招聘（ID：'.$id.'）';
                    }
                    $opera  =   1;
                } else if ($type == 'recpart') {
                    $this->update_once('partjob', array('rec_time' => ''), array('id' => $id));
                    if ($job['rec_day'] > 0) {
                        $this->update_once('company_statis', array('rec_num' => array('+', intval($job['rec_day']))), array('uid' => $job['uid']));
                        $logContent =   '取消兼职推荐（ID：'.$id.'）';
                        $logDetail  =   '返还推荐套餐：'.$job['rec_day'].'天';
                    }else{
                        $logContent =   '取消兼职推荐（ID：'.$id.'）';
                    }
                    $opera  =   9;
                }

                if (isset($logContent)){

                    $this->addMemberLog($uid, $usertype, $logContent, $opera, 2, $logDetail);
                    $return =   array('errcode' => 9, 'msg' => '职位推广取消成功');
                }else{

                    $return =   array('errcode' => 8, 'msg' => '职位推广取消失败');
                }
            } else {

                $return = array('errcode' => 8, 'msg' => '参数错误，请重试！');
            }
        }else{

            $return = array('errcode' => 8, 'msg' => '系统错误，尚未开启职位推广取消功能！');
        }

        return $return;
    }

    /**
     * @desc     屏蔽企业
     */
    function pbComs($pbData = array()) {

        $return	=	array();

        $info	=	$this->getYqmsInfo(array('id'=>$pbData['id'], 'uid'=>$pbData['uid']) );
        $data['p_uid']		=	$info['fid'];
        $data['inputtime']	=	mktime();
        $data['c_uid']		=	$pbData['uid'];
        $data['usertype']	=	1;
        $data['com_name']	=	$info['fname'];

        $haves	=	$this->select_once('blacklist',array('c_uid'=>$data['c_uid'],'p_uid'=>$data['p_uid'],'usertype'=>$data['usertype']) );

        if(is_array($haves)){

            $return['msg']		=	"该用户已在您黑名单中！";
            $return['url']		=	$_SERVER['HTTP_REFERER'];
            $return['errcode']	=	8;
        }else{

            $nid	=	$this->insert_into('blacklist',$data);

            $this->update_once('userid_msg',array('isdel'=>$data['usertype']),array('uid'=>$data['c_uid'],'fid'=>$data['p_uid']));
            if($nid){

                $logContent =   '屏蔽企业：UID'.$data['p_uid'];
                $logDetail  =   '屏蔽企业《'.$data['fname'].'》，并且删除邀请信息';
                $this->addMemberLog($data['c_uid'], $data['usertype'], $logContent, 26, 3, $logDetail);

                $return['msg']		=	'操作成功！';
                $return['url']		=	'index.php?c=invite';
                $return['errcode']	=	9;
            }else{

                $return['msg']		=	'操作失败！';
                $return['url']		=	'index.php?c=invite';
                $return['errcode']	=	8;
            }
        }
        return	$return;
    }


    /**
     * @desc 发布职位条件查询
     *
     * @param $uid
     * @param null $job
     * @param string $spid
     * @param string $wxapp
     * @param array $data
     * @return array
     */
    public function getAddJobNeedInfo($uid, $job = null, $spid = '', $wxapp = '', $data = array())
    {

        $provider   =   isset($data['provider']) ? $data['provider'] : '';
        require_once'company.model.php';
        $comM       =   new company_model($this->db, $this->def);

        $info       =   $comM->getInfo($uid);

        $msgList    =   array();

        if ($this->config['com_enforce_info'] == 1){
			if (!$info['name'] || !$info['provinceid'] || !$info['linktel']) {
				if (empty($spid)) {

					$msgList['pc'][]            =   '<div class="yun_prompt_release_ws">基本信息未完善 <a href="' . $this->config['sy_weburl'] . "/member/index.php?c=info" . '" class="yun_prompt_release_ws_a" target="_blank">立即完善&gt;</a></div>';
					$msgList['wxapp']['name']   =   1;
				} else {

					$msgList['pc'][]            =   '<div class="yun_prompt_release_ws">基本信息未完善 <a class="yun_prompt_release_ws_a" target="_blank">待完善&gt;</a></div>';
					$msgList['wxapp']['name']   =   1;
				}
			}
        }

        if ($this->config['com_enforce_mobilecert'] == 1) {
            if ($info['moblie_status'] != "1") {
                if (empty($spid)) {

                    $msgList['pc'][]            =   '<div class="yun_prompt_release_ws">手机未认证 <a href="' . $this->config['sy_weburl'] . "/member/index.php?c=binding" . '" class="yun_prompt_release_ws_a" target="_blank">立即认证&gt;</a></div>';
                    $msgList['wxapp']['tel']    =   1;
                } else {

                    $msgList['pc'][]            =   '<div class="yun_prompt_release_ws">手机未认证 <a class="yun_prompt_release_ws_a" target="_blank">待认证&gt;</a></div>';
                    $msgList['wxapp']['tel']    =   1;
                }
            }
        }

        if ($this->config['com_enforce_emailcert'] == 1) {
            if ($info['email_status'] != "1") {
                if (empty($spid)) {

                    $msgList['pc'][]            =   '<div class="yun_prompt_release_ws">邮箱未认证 <a href="' . $this->config['sy_weburl'] . "/member/index.php?c=binding" . '" class="yun_prompt_release_ws_a" target="_blank">立即认证&gt;</a></div>';
                    $msgList['wxapp']['email']  =   1;
                } else {

                    $msgList['pc'][]            =   '<div class="yun_prompt_release_ws">邮箱未认证 <a  class="yun_prompt_release_ws_a" target="_blank">待认证&gt;</a></div>';
                    $msgList['wxapp']['email']  =   1;
                }
            }
        }

        if ($this->config['com_enforce_licensecert'] == 1) {

             $cert   =   $comM->getCertInfo(array('uid' => $uid, 'type' => 3), array('field' => '`uid`,`status`'));

            if ($info['yyzz_status'] != "1" && (empty($cert) || $cert['status'] == 2)) {
                if (empty($spid)) {

                    $msgList['pc'][]            =   '<div class="yun_prompt_release_ws">企业资质未认证 <a href="' . $this->config['sy_weburl'] . "/member/index.php?c=binding" . '" class="yun_prompt_release_ws_a" target="_blank">立即认证&gt;</a></div>';
                    $msgList['wxapp']['yyzz']   =   1;
                } else {

                    $msgList['pc'][]            =   '<div class="yun_prompt_release_ws">企业资质未认证 <a class="yun_prompt_release_ws_a" target="_blank">待认证&gt;</a></div>';
                    $msgList['wxapp']['yyzz']   =   1;
                }
            }
        }

        if ($this->config['com_enforce_setposition'] == 1) {
            if (empty($info['x']) || empty($info['y'])) {
                if (empty($spid)) {

                    $msgList['pc'][]        =   '<div class="yun_prompt_release_ws">地图未设置 <a href="' . $this->config['sy_weburl'] . "/member/index.php?c=map" . '" class="yun_prompt_release_ws_a" target="_blank">立即设置&gt;</a></div>';
                    $msgList['wxapp']['xy'] =   1;
                } else {

                    $msgList['pc'][]        =   '<div class="yun_prompt_release_ws">地图未设置 <a class="yun_prompt_release_ws_a" target="_blank">待设置&gt;</a></div>';
                    $msgList['wxapp']['xy'] =   1;
                }
            }
        }

        if ($this->config['com_gzgzh'] == '1') {

            $uInfo      =   $this->select_once('member', array('uid' => $uid), '`wxid`,`subscribe`,`wxopenid`,`app_wxid`,`unionid`');
            $isSubscribe=   0;
            if ($uInfo['subscribe'] != 1) {
                if (!empty($uInfo['wxid'])) {

                    require_once 'weixin.model.php';
                    $wxM        =   new weixin_model($this->db, $this->def);
                    $wxUserInfo =   $wxM->getWxUser($uInfo['wxid']);

                    $this->update_once('member', array('subscribe' => $wxUserInfo['subscribe']), array('uid' => $uid));

                    if ($wxUserInfo['subscribe'] == 1) {
                        $isSubscribe    =   1;
                    }
                }
            } else if ($uInfo['subscribe'] == 1) {

                $isSubscribe    =   1;
            }

            if ($wxapp == '') {
                if ($isSubscribe == 0) {

                    $msgList['pc'][] = '<div class="yun_prompt_release_ws">微信公众号未关注 <a href="javascript:;" onclick="gzhShow();" class="yun_prompt_release_ws_a">立即关注&gt;</a></div>';
                }
            } else {
                if ($provider != 'toutiao' && $provider != 'baidu') {
                    if($provider == 'h5'){
                        // wap处理
                        if ($isSubscribe == 0) {
                            $msgList['wxapp']['gzh']    =   1;
                        }
                    }else{
                        // app处理
                        if($provider == 'app'){
                            if ($isSubscribe == 0) {
                                if (empty($uInfo['app_wxid'])){
                                    if (!empty($uInfo['wxid']) || !empty($uInfo['wxopenid'])) {

                                        $msgList['wxapp']['gzh']    =   2;
                                    } else {

                                        $msgList['wxapp']['gzh']    =   1;
                                    }
                                }
                            }
                        }else{
                            // 小程序处理
                            if ($isSubscribe == 0) {
                                if(empty($uInfo['wxopenid'])) {
                                    if (!empty($uInfo['wxid']) || !empty($uInfo['app_wxid'])) {

                                        $msgList['wxapp']['gzh']    =   2;
                                    } else {
                                        $msgList['wxapp']['gzh']    =   1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if (empty($job)) {
			require_once 'statis.model.php';
			$statisM	=	new statis_model($this->db, $this->def);
			$sInfo		=	$statisM->vipOver($uid, 2, '');
			$msgList['pc'][]	=	'<div class="yun_prompt_release_ws">发布职位 <a href="javascript:;" onclick="jobadd_url('.$sInfo['addjobnum'].');return false;" class="yun_prompt_release_ws_a">立即发布&gt;</a></div><script src="'.$this->config['sy_weburl'].'/js/member_public.js"></script>';
            $msgList['wxapp']['job']    =   1;
        }

        return $msgList;
    }

    /**
     * @desc 猎头会员发布职位条件查询
     * @param $uid
     * @param null $job
     * @return array
     */
    public function getAddJobNeedLtInfo($uid, $job = null)
    {

        require_once 'lietou.model.php';
        $ltM    =   new lietou_model($this->db, $this->def);

        $info   =   $ltM->getInfo(array('uid' => $uid));

        $msgList=   array();

        if (!$info['realname'] || !$info['com_name'] || !$info['provinceid'] || !$info['moblie']) {

            $msgList['pc'][]        =   '<div class="yun_prompt_release_ws">基本信息不完善 <a href="' . $this->config['sy_weburl'] . "/member/index.php?c=info" . '" class="yun_prompt_release_ws_a" target="_blank">立即完善&gt;</a></div>';
            $msgList['wap'][]       =   '<div class="yun_prompt_release_ws">基本信息不完善 <a href="' . $this->config['sy_weburl'] . "/wap/member/index.php?c=info" . '" class="yun_prompt_release_ws_a">立即完善&gt;</a></div>';
        }

        if ($this->config['lt_enforce_mobilecert'] == "1") {
            if ($info['moblie_status'] == '0') {

                $msgList['pc'][]    =   '<div class="yun_prompt_release_ws">手机未认证 <a href="' . $this->config['sy_weburl'] . "/member/index.php?c=binding" . '" class="yun_prompt_release_ws_a" target="_blank">立即认证&gt;</a></div>';
                $msgList['wap'][]   =   '<div class="yun_prompt_release_ws">手机未认证 <a href="' . $this->config['sy_weburl'] . "/wap/member/index.php?c=bindingbox&type=moblie" . '" class="yun_prompt_release_ws_a">立即认证&gt;</a></div>';
            }
        }

        if ($this->config['lt_enforce_emailcert'] == 1) {
            if ($info['email_status'] == '0') {

                $msgList['pc'][]    =   '<div class="yun_prompt_release_ws">邮箱未认证 <a href="' . $this->config['sy_weburl'] . "/member/index.php?c=binding" . '" class="yun_prompt_release_ws_a" target="_blank">立即认证&gt;</a></div>';
                $msgList['wap'][]   =   '<div class="yun_prompt_release_ws">邮箱未认证 <a href="' . $this->config['sy_weburl'] . "/wap/member/index.php?c=bindingbox&type=email" . '" class="yun_prompt_release_ws_a">立即认证&gt;</a></div>';
            }
        }

        if ($this->config['lt_enforce_licensecert'] == "1") {

            require_once 'company.model.php';
            $comM   =   new company_model($this->db, $this->def);
            $cert   =   $comM->getCertInfo(array('uid' => $uid, 'type' => 4), array('field' => 'uid'));

            if ($info['yyzz_status'] == '0' && (empty($cert) || $cert['status'] == 2)) {

                $msgList['pc'][]    =   '<div class="yun_prompt_release_ws">猎头资质未认证 <a href="' . $this->config['sy_weburl'] . "/member/index.php?c=binding" . '" class="yun_prompt_release_ws_a" target="_blank">立即认证&gt;</a></div>';
                $msgList['wap'][]   =   '<div class="yun_prompt_release_ws">猎头资质未认证 <a href="' . $this->config['sy_weburl'] . "/wap/member/index.php?c=ltcert" . '" class="yun_prompt_release_ws_a">立即认证&gt;</a></div>';
            }
        }

        if (empty($job)) {
            $msgList['pc'][]    =   '<div class="yun_prompt_release_ws">发布职位 <a href="' . $this->config['sy_weburl'] . "/member/index.php?c=jobadd" . '" class="yun_prompt_release_ws_a" target="_blank">立即发布&gt;</a></div>';
            $msgList['wap'][]   =   '<div class="yun_prompt_release_ws">发布职位 <a href="' . $this->config['sy_weburl'] . "/wap/member/index.php?c=jobadd" . '" class="yun_prompt_release_ws_a">立即发布&gt;</a></div>';
        }
        return $msgList;
    }

    /**
     * 发布工具搜索
     * @param array $where
     * @param array $data
     * @return array
     */
    public function Getpubtool($where = array(),$data = array())
    {

        $select =   $data['field'] ? $data['field'] : '*';
        $lists  =   $this->select_all('company_job',$where,$select);


        //是否限制职位
        if(isset($data['rule'])){
            $lists      =   $this->makelists($lists,$where,1,$data['rule']);
        }

        $comid = array();
        $linkid = array();
        foreach ($lists as $key => $value) {
            if($value['uid'] && !in_array($value['uid'],$comid)){
                $comid[] = $value['uid'];
            }
            if ($value['link_id'] > 0 && !in_array($value['link_id'], $linkid)){
                $linkid[]   =   $value['link_id'];
            }
        }
        $comData = array();
        if(!empty($comid)){
            $comlists  =   $this->select_all('company',array('uid'=>array('in',pylode(',',$comid))));
            foreach ($comlists as $ck => $cv) {
                $comData[$cv['uid']]['desc'] = str_replace(array('&quot;', '&nbsp;', '<>'), array('', '', ''), strip_tags($cv['content']));
                $comData[$cv['uid']]['phone'] = !empty($cv['linktel']) ? $cv['linktel'] : $cv['linkphone'];
                $comData[$cv['uid']]['address'] =  $cv['address'];
            }
        }
        $linkData = array();
        if (!empty($linkid)){

            $linkList   =   $this->select_all('company_job_link',array('id'=>array('in',pylode(',',$linkid))));
            foreach ($linkList as $lk => $lv){

                $linkData[$lv['id']]['provinceid']  =   $lv['provinceid'];
                $linkData[$lv['id']]['cityid']      =   $lv['cityid'];
                $linkData[$lv['id']]['three_cityid']=   $lv['three_cityid'];
                $linkData[$lv['id']]['address']     =   $lv['link_address'];
                $linkData[$lv['id']]['phone']       =   !empty($lv['link_moblie']) ? $lv['link_moblie'] : $lv['link_phone'];
            }
        }

        $newlist        =   array();

        foreach ($lists as $k => $v) {
            if ($v['is_link'] == 2 && !empty($v['link_id'])){
                $v['provinceid']    =   $linkData[$v['link_id']]['provinceid'];
                $v['cityid']        =   $linkData[$v['link_id']]['cityid'];
                $v['three_cityid']  =   $linkData[$v['link_id']]['three_cityid'];
            }

            $list       =   $this->getInfoArray($v);

            if(!empty($comData[$v['uid']])){
                $list['com_desc'] = $comData[$v['uid']]['desc'];
            }

            if ($v['is_link'] == 2 && !empty($v['link_id'])){

                $list['address'] = $linkData[$v['link_id']]['address'];
                $list['phone']  = $linkData[$v['link_id']]['phone'];
            }else{

                $list['address'] = $comData[$v['uid']]['address'];
                $list['phone']  = $comData[$v['uid']]['phone'];
            }

            $newlist[]  =   $list;
        }

        return $newlist;
    }

    /**
     * 发布工具限制企业职位数
     * @param $lists
     * @param $where
     * @param int $page
     * @param $rpt      限制重复企业数
     * @return array
     */
    protected function makelists($lists, $where, $page = 1, $rpt)
    {

        $limit      =   $where['limit'][1];

        //去重之后列表
        $newlist    =   $this->arrayuniq($lists, $rpt, $limit);

        $count      =   count($newlist);

        //职位条数不够继续取
        if ($count < $limit) {

            $pages          =   $page * $limit;
            $where['limit'] =   array("$pages", "$limit");
            $lists          =   $this->select_all('company_job', $where, '*');
            if (empty($lists)) {

                return $newlist;
            } else {

                $lists      =   array_merge($newlist, $lists);
                return $this->makelists($lists, $where, $page + 1, $rpt);
            }
        } else {

            return $newlist;
        }
    }

    /**
     * 除去多余企业职位
     *
     * @param $lists
     * @param $rpt
     * @param $limit
     * @return array
     */
    protected function arrayuniq($lists,$rpt,$limit)
    {

        $i          =   1;
        $newlist    =   array();
        foreach ($lists as $k => $v) {

            $arr    =   array_column($newlist, 'uid');
            $arr    =   array_count_values($arr);
            $uid    =   $arr[$v['uid']];
            if ($uid < $rpt) {
                $i++;
                if ($i > $limit) {
                    break;
                }
                $newlist[] = $v;
            }
        }
        return $newlist;
    }

    /**
     * 添加拨号记录
     *
     * @param array $data
     */
    function addTelLog($data = array())
    {

        $jobid  =   isset($data['jobid']) ? $data['jobid'] : 0;
        $comid  =   isset($data['comid']) ? $data['comid'] : 0;

        if ($jobid || $comid) {

            $dataV              =   array();
            if ($jobid) {
                $job            =   $this->getInfo(array('id' => intval($jobid)));
                $dataV['jobid'] =   $job['id'];
                $comid          =   $job['uid'];
            }

            if ($comid && $data['uid']!=$comid) {

                $dataV['comid'] =   $comid;
                $dataV['ip']    =   fun_ip_get();
                $dataV['ctime'] =   time();

                if (isset($data['uid'])) {
                    $dataV['uid']   =   $data['uid'];
                }

                if (isset($data['source'])) {
                    $dataV['source']=   $data['source'];
                }

                $this->insert_into('job_tellog', $dataV);

                include_once('warning.model.php');
                $warningM   =   new warning_model($this->db, $this->def);
                $warningM->warning(9, $data['uid']);
            }
        }
    }

    function getTelLogNum($whereData = array())
    {
        return $this->select_num('job_tellog', $whereData);
    }

    /**
     * 拨号记录
     *
     * @param array $where
     * @param array $data
     * @return array|bool|false|string|void
     */
    function getTelLogs($where = array(), $data = array())
    {

        $logs   =   array();

        if (!empty($where)) {

            $field  =   $data['field'] ? $data['field'] : '*';

            unset($data['field']);

            $logs   =   $this->select_all('job_tellog', $where, $field);

            if (isset($data['utype']) && $data['utype'] == 'admin' && !empty($logs)) {

                $uids = $comids = $alluids = $jobids = array();

                foreach ($logs as $key => $value) {

                    if (isset($value['uid']) && $value['uid'] && !in_array($value['uid'], $uids)) {

                        $uids[]         =   $value['uid'];
                        if (!in_array($value['uid'], $alluids)) {
                            $alluids[]  =   $value['uid'];
                        }
                    }

                    if (isset($value['comid']) && $value['comid'] && !in_array($value['uid'], $comids)) {

                        $comids[]       =   $value['comid'];
                        if (!in_array($value['comid'], $alluids)) {
                            $alluids[]  =   $value['comid'];
                        }
                    }

                    if (isset($value['jobid']) && $value['jobid'] && !in_array($value['jobid'], $jobids)) {

                        $jobids[]       =   $value['jobid'];
                    }
                }

                $members = $users = $jobs = array();

                include(CONFIG_PATH.'db.data.php');

                include_once('userinfo.model.php');
                $UserinfoM  =   new userinfo_model($this->db, $this->def);

                if (!empty($uids)) {

                    $users  =   $UserinfoM->getUserInfoList(array('uid' => array('in', pylode(',', $uids))), array('usertype' => 1, 'field' => '`uid`,`name`'));
                }
                if (!empty($jobids)) {

                    $jobs   =   $this->getList(array('id' => array('in', pylode(',', $jobids))), array('field' => '`id`,`uid`,`name`,`com_name`'));
                }

                if (!empty($comids)) {

                    $companys   =   $this->select_all('company', array('uid' => array('in', pylode(',', $comids))), '`uid`,`name`');
                }

                if (!empty($alluids)) {

                    $memberlist =   $UserinfoM->getList(array('uid' => array('in', pylode(',', $alluids))), array('field' => '`uid`,`username`'));

                    foreach ($memberlist as $mk => $mv) {
                        $members[$mv['uid']]    =   $mv['username'];
                    }
                }

                foreach ($logs as $k => $v) {

                    $logs[$k]['source'] =   $arr_data['source'][$v['source']];

                    if (!empty($users) && $v['uid']) {

                        foreach ($users as $uk => $uv) {

                            if ($v['uid'] == $uv['uid']) {

                                $logs[$k]['username']   =   $uv['name'] ? $uv['name'] : $members[$v['uid']];
                            }
                        }
                    } else {

                        $logs[$k]['username']           =   '游客';
                    }
                    if (!empty($companys) && $v['comid']) {

                        foreach ($companys as $ck => $cv) {

                            if ($v['comid'] == $cv['uid']) {

                                $logs[$k]['com_name']   =   $cv['name'];
                                $com_url = Url('company', array('c' => 'show', 'id' => $v['comid'], 'look' => 'admin'));
                                $logs[$k]['com_url'] = $com_url;
                            }
                        }
                    }
                    if (!empty($jobs['list']) && $v['jobid']) {

                        foreach ($jobs['list'] as $jk => $jv) {

                            if ($v['jobid'] == $jv['id']) {

                                $logs[$k]['job_name']   =   $jv['name'];
                                $logs[$k]['job_url'] = Url('job', array('c'=>'comapply','id'=>$v['jobid'], 'look' => 'admin'));
                            }
                        }
                    }
                    $logs[$k]['ctime_n'] = date('Y-m-d H:i', $v['ctime']);
                }
            }
        }
        return $logs;
    }

    /**
     * 删除拨号记录
     *
     * @param array $whereData
     * @param array $data
     * @return mixed
     */
    function delJobTelLog($whereData = array(), $data = array())
    {

        $return['layertype']    =   0;

        if (!empty($whereData)) {

            if (!empty($whereData['id']) && $whereData['id'][0] == 'in') {

                $return['layertype']    =   1;
            }

            if ($data['norecycle'] == '1') {  //  数据库清理，不插入回收站

                $return['id']   =   $this->delete_all('job_tellog', $whereData, '', '', '1');
            } else {

                $return['id']   =   $this->delete_all('job_tellog', $whereData, '');
            }

            $return['msg']      =   '拨号记录';
            $return['errcode']  =   $return['id'] ? '9' : '8';
            $return['msg']      =   $return['id'] ? $return['msg'] . '删除成功！' : $return['msg'] . '删除失败！';
        } else {

            $return['msg']      =   '请选择您要删除的拨号记录！';
            $return['errcode']  =   8;
        }

        return $return;
    }

    /**
     * 小程序请求职位列表，更新职位曝光量
     *
     * @param array $upData
     * @param array $whereData
     */
    public function upJobExpoure($upData = array(), $whereData = array())
    {

        if (!empty($upData) && !empty($whereData)) {
            $this->update_once('company_job', $upData, $whereData);
        }
    }

    /**
     * 预约职位刷新
     * @param array $post
     * @param string[] $data
     * @return array
     */
    public function reserveUpJob($post = array(), $data = array('uid' => ''))
    {

        $return =   array('error' => 0, 'msg' => '');

        if (!empty($post) && $data['uid']) {

            $jobId  =   $post['job_id'];
            $uid    =   $data['uid'];

            $statis =   $this->select_once('company_statis', array('uid' => $uid), '`breakjob_num`,`rating`');
            $ratingInfo =   $this->select_once('company_rating', array('id' => $statis['rating']),'`freerefresh_num`');
            if ((int)$ratingInfo['freerefresh_num'] > 0){

                $freeArr        =   $this->select_all('job_refresh_log', array('uid' => $uid, 'free' => 1, 'r_time' => array('>=', strtotime('today'))), 'sum(free_num) as `num`');   //  今日免费刷新数目
                $freeNum        =   $freeArr[0]['num'];

                $statis['free_num']     =   intval($ratingInfo['freerefresh_num']) - $freeNum;
            }else{

                $statis['free_num']     =   0;
            }

            $num    =   ($statis['breakjob_num'] + $statis['free_num']) / $this->config['sy_reserve_refresh_price'];

            if (intval($num) == 0 && $post['status'] != 2){
                $return =   array(

                    'error' =>  -1,
                    'msg'   =>  '剩余刷新套餐不足预约'
                );
            }else if ($this->config['com_job_reserve'] != 1){

                $return =   array(

                    'error' =>  0,
                    'msg'   =>  '预约刷新功能未开启'
                );
            }else {

                $job    =   $this->select_all('company_job', array('id' => array('in', $jobId), 'uid' => $uid, 'state' => 1, 'r_status' => 1, 'status' => 0));

                if (isset($job) && !empty($job)) {

                    $is_reserve =   $_POST['status'] == 1 ? 1 : 0;

                    if ($post['end_time'] > 0 && $post['end_time'] < strtotime(date('Y-m-d',strtotime('+1 day'))) && $is_reserve == 1) {

                        $return =   array(
                            'error' =>  0,
                            'msg'   =>  '截止日期不得设置今天'
                        );
                    } else if ($post['interval'] < $this->config['sy_reserve_refresh_interval'] && $is_reserve == 1){

                        $return =   array(
                            'error' =>  0,
                            'msg'   =>  '预约职位刷新，时间间隔不得低于'.$this->config['sy_reserve_refresh_interval'].'分钟'
                        );
                    }else {

                        if (!empty($post['s_time']) && !empty($post['e_time'])){

                            $stime  =   explode(':', $post['s_time']);
                            $etime  =   explode(':', $post['e_time']);
                            if (intval($stime[0]) > intval($etime[0]) || (intval($stime[0]) == intval($etime[0]) && intval($stime[1]) >= intval($etime[1]))){

                                $return =   array(
                                    'error' =>  0,
                                    'msg'   =>  '刷新时间段设置不合理：开始时间不能超过结束时间'
                                );
                            }
                        }

                        $reserveList    =   $this->select_all('reserve_refresh', array('uid' => $uid), '`job_id`');
                        $reserveJobIdA  =   array();
                        if (!empty($reserveList)){
                            foreach ($reserveList as $rk => $rv){

                                $reserveJobIdA[]    =   $rv['job_id'];
                            }
                        }

                        $newJobIdA  =   array();
                        $upJobIdA   =   array();
                        foreach ($job as $jk => $jv) {
                            if (!in_array($jv['id'], $reserveJobIdA)){

                                $newJobIdA[]    =   $jv['id'];
                            }else{

                                $upJobIdA[]     =   $jv['id'];
                            }
                        }

                        $value          =   array(

                            'status'        =>  $post['status'],
                            'interval'      =>  $post['interval'],
                            'start_time'    =>  time(),
                            'end_time'      =>  $post['end_time'] ? $post['end_time'] : 0,
                            'last_time'     =>  '0',
                            'next_time'     =>  strtotime('+ '.$post['interval'].' minutes'),
                            's_time'        =>  isset($post['s_time']) && !empty($post['s_time']) ? $post['s_time'] : '',
                            'e_time'        =>  isset($post['e_time']) && !empty($post['e_time']) ? $post['e_time'] : ''
                        );


                        if (!empty($newJobIdA)) { //  插入职位预约刷新

                            $newData    =   array();
                            foreach ($newJobIdA as $nk => $nv){

                                $newData[$nk]['status'] =   $value['status'];
                                $newData[$nk]['interval'] =   $value['interval'];
                                $newData[$nk]['start_time'] =   $value['start_time'];
                                $newData[$nk]['end_time'] =   $value['end_time'];
                                $newData[$nk]['last_time'] =   $value['last_time'];
                                $newData[$nk]['next_time'] =   $value['next_time'];
                                $newData[$nk]['s_time'] =   $value['s_time'];
                                $newData[$nk]['e_time'] =   $value['e_time'];
                                $newData[$nk]['job_id'] =   $nv;
                                $newData[$nk]['uid'] =   $uid;
                            }

                            $nid            =   $this->DB_insert_multi('reserve_refresh', $newData);

                            if ($is_reserve == 1){

                                $this->update_once('company_job', array('is_reserve' => $is_reserve), array('id' => array('in', implode(',', $newJobIdA)), 'uid' => $uid));
                            }

                            $logContent = '职位更新：设置职位（ID：' . implode(',', $newJobIdA) . '）预约刷新';
                            $this->addMemberLog($uid, 2, $logContent, 1, 4);
                        }

                        if (!empty($upJobIdA)){ //  更新职位预约刷新

                            $upData         =   $value;
                            $nid            =   $this->update_once('reserve_refresh', $upData, array('job_id' => array('in', implode(',', $upJobIdA)), 'uid' => $uid));

                            if ($is_reserve == 1){

                                $this->update_once('company_job', array('is_reserve' => 1), array('id' => array('in', implode(',', $upJobIdA)), 'uid' => $uid));
                                if (isset($data['utype']) && $data['utype'] == 'admin'){

                                    require_once ('log.model.php');
                                    $LogM   =   new log_model($this->db, $this->def);
                                    $LogM->addAdminLog('预约刷新调整，ID：'.implode(',', $upJobIdA));
                                }else {

                                    $logContent = '职位更新：更新职位（ID：' . implode(',', $upJobIdA) . '）预约刷新设置';
                                    $this->addMemberLog($uid, 2, $logContent, 1, 4);
                                }
                            }else{

                                $this->update_once('company_job', array('is_reserve' => 0), array('id' => array('in', implode(',', $upJobIdA)), 'uid' => $uid));
                                if (isset($data['utype']) && $data['utype'] == 'admin'){
                                    require_once ('log.model.php');
                                    $LogM   =   new log_model($this->db, $this->def);
                                    $LogM->addAdminLog('关闭预约刷新，ID：'.implode(',', $upJobIdA) );
                                }else {
                                    $logContent = '职位更新：关闭职位（ID：' . implode(',', $upJobIdA)  . '）预约刷新';
                                    $this->addMemberLog($uid, 2, $logContent, 1, 4);
                                }
                            }
                        }

                        $return['error']    =   $nid ? 1 : 0;
                        $return['msg']      =   $nid ? '职位预约刷新设置成功' : '职位预约刷新设置失败';
                    }

                } else {

                    $return =   array(
                        'error' =>  0,
                        'msg'   =>  '职位信息查询失败'
                    );
                }
            }
        } else {
            $return =   array(
                'error' =>  0,
                'msg'   =>  '参数错误'
            );
        }

        return $return;
    }

    /**
     * 计划任务：预约刷新职位
     */
    function upReserveJob()
    {

        //  剔除非招聘状态职位信息
        $nzpWhere   =   array(
            'is_reserve'        =>  1,
            'PHPYUNBTWSTART_A'  =>  '',
            'state'     =>  array('<>', 1, ''),
            'status'    =>  array('=', 1, 'OR'),
            'r_status'  =>  array('<>', 1, 'OR'),
            'PHPYUNBTWEND_A'    =>  ''
        );
        $jobList    =   $this->select_all('company_job', $nzpWhere, '`id`');
        if (!empty($jobList)){

            $jobIdS =   array();
            foreach ($jobList as $k => $v){

                $jobIdS[]   =   $v['id'];
            }
            $this->update_once('company_job', array('is_reserve' => 0), array('id' => array('in', pylode(',', $jobIdS))));
            $this->update_once('reserve_refresh', array('status' => 2), array('job_id' => array('in', pylode(',', $jobIdS))));
        }

        //  剔除已到达预约刷新截止日期职位
        $endWhere       =   array(
            'status'            =>  1,
            'PHPYUNBTWSTART_A'	=>	'',
            'end_time'  =>  array(
                '0'	=>	array('<=', strtotime(date('Y-m-d')), ''),
                '1'	=>	array('>', 0, '')
            ),
            'PHPYUNBTWEND_A'    =>  ''
        );
        $endReserveList =   $this->select_all('reserve_refresh', $endWhere);
        if (!empty($endReserveList)) {

            $endIdArr       =   array();
            $endJobIdArr    =   array();
            foreach ($endReserveList as $ek => $ev) {

                $endIdArr[]     =   $ev['id'];
                $endJobIdArr[]  =   $ev['job_id'];
            }

            $this->update_once('reserve_refresh', array('status' => 2), array('id' => array('in', pylode(',', $endIdArr))));
            $this->update_once('company_job', array('is_reserve' => 0), array('id' => array('in', pylode(',', $endJobIdArr))));
        }

        // 可刷新预约职位
        $where          =   array(
            'status'            =>  1,
            'PHPYUNBTWSTART_A'  =>  '',
            'end_time'          =>  array(
                '0'     =>  array('>', strtotime(date('Y-m-d')), 'OR'),
                '1'     =>  array('=', 0, 'OR')
            ),
            'PHPYUNBTWEND_A'    =>  '',
            'next_time'         =>  array('<', time()),
            'PHPYUNBTWSTART_B'  =>  '',
            'last_time'         =>  array(
                '0' =>  array('=', 0, 'OR'),
                '1' =>  array('<', strtotime('- '.$this->config['sy_reserve_refresh_interval'].' minutes'), 'OR')
            ),
            'PHPYUNBTWEND_B'    =>  ''
        );
        $reserveList    =   $this->select_all('reserve_refresh', $where);
        if (!empty($reserveList)) {

            $hour       =   date('H');
            $minutes    =   date('i');

            foreach ($reserveList as $k => $v) {

                if(!empty($v['s_time'])){
                    $stime  =   explode(':', $v['s_time']);

                    if (intval($stime[0]) > $hour || (intval($stime[0]) == $hour && intval([$stime[1]]) > $minutes)){
                        unset($reserveList[$k]);
                    }
                }
                if(!empty($v['e_time'])){
                    $etime  =   explode(':', $v['e_time']);
                    if (intval($etime[0]) < $hour || (intval($etime[0]) == $hour && intval([$etime[1]]) < $minutes)){
                        unset($reserveList[$k]);
                    }
                }
            }

            $jobIds         =   array();
            foreach ($reserveList as $k => $v) {

                $jobIds[]   =   $v['job_id'];
            }
            $zpJobList      =   $this->select_all('company_job', array('status' => 0, 'state' => 1, 'r_status' => 1, 'id' => array('in', pylode(',', $jobIds))), 'id');

            if (!empty($zpJobList)) {

                $jobIds     =   array();

                foreach ($zpJobList as $zk => $zv) {
                    $jobIds[]   =   $zv['id'];
                }
                foreach ($reserveList as $k => $v) {
                    if (!in_array($v['job_id'], $jobIds)) {

                        unset($reserveList[$k]);
                    }
                }
            } else {

                $reserveList    =   array();
            }

            if (!empty($reserveList)){
                foreach ($reserveList as $k => $v) {

                    $LastTime   =   $v['next_time'] > strtotime('today') ? $v['next_time'] : time();

                    $statis     =   $this->select_once('company_statis', array('uid' => $v['uid']), '`uid`,`rating`,`rating_type`,`breakjob_num`,`vip_etime`');

                    $ratingInfo =   $this->select_once('company_rating', array('id' => $statis['rating']), '`freerefresh_num`');

                    /**
                     * 存在每日免费刷新套餐数量
                     */
                    $freeNum    =   0;
                    if (intval($ratingInfo['freerefresh_num']) > 0){

                        //  今日已经免费刷新职位数量
                        $freedArr   =   $this->select_once('job_refresh_log', array('uid' => $v['uid'], 'free' => 1, 'r_time' => array('>=', strtotime('today'))), 'sum(`free_num`) as `num`');
                        $freedNum   =   $freedArr['num'];
                        //  今日剩余免费刷新数量
                        $freeNum    =   intval($ratingInfo['freerefresh_num']) - $freedNum;
                    }
                    if (isVip($statis['vip_etime'])) {

                        if ($statis['rating_type'] == 2) {

                            $currentNum =   $this->select_num('job_refresh_log', array('uid' => $v['uid'], 'free' => 2, 'r_time' => array('>=', strtotime('today'))));   //  今日刷新数目（套餐+消费）

                            $needNum    =   $currentNum > $this->config['sy_reserve_refresh_price'] ? $currentNum : $this->config['sy_reserve_refresh_price'];

                            if (($statis['breakjob_num'] + $freeNum)  > $needNum){

                                $dValue =   array('last_time' => $LastTime, 'next_time' => strtotime('+ '.$v['interval'].' minutes', $LastTime));

                                $jValue =   array('lastupdate' => $LastTime);

                                if (strtotime('+ '.$v['interval'].' minutes', $LastTime) >= ($v['end_time'] - 1) && $v['end_time'] > 0) {

                                    $dValue['status']       =   2;
                                    $jValue['is_reserve']   =   0;
                                }

                                $result    =   $this->update_once('company_job', $jValue, array('id' => $v['job_id'], 'uid' => $v['uid'], 'state' => 1, 'r_status' => 1, 'status'=> 0));

                                if ($result) {

                                    $this->update_once('reserve_refresh', $dValue, array('id' => $v['id']));
                                    $this->update_once('company', array('lastupdate'=> $LastTime), array('uid' => $v['uid']));
                                    $this->update_once('hot_job', array('lastupdate' => $LastTime), array('uid' => $v['uid']));

                                    if ($freeNum > 0){

                                        $free_num   =   $freeNum > $this->config['sy_reserve_refresh_price'] ?    $this->config['sy_reserve_refresh_price'] : $freeNum;
                                        $this->addJobSxLog(array('uid' => $v['uid'], 'usertype' => 2, 'jobid' => $v['job_id'], 'r_time' => $LastTime, 'type' => 1, 'remark' => '计划任务：职位预约刷新'), 1, $free_num);
                                    }else{

                                        $this->addJobSxLog(array('uid' => $v['uid'], 'usertype' => 2, 'jobid' => $v['job_id'], 'r_time' => $LastTime, 'type' => 1, 'remark' => '计划任务：职位预约刷新'));
                                    }
                                }
                            }else{

                                $dValue =   array('next_time' => strtotime('+1 days 6 hours'));

                                if ($dValue >= ($v['end_time'] - 1) && $v['end_time'] > 0) {

                                    $dValue['status']       =   2;
                                    $jValue['is_reserve']   =   0;

                                    $this->update_once('company_job', $jValue, array('id' => $v['job_id']));
                                }

                                $this->update_once('reserve_refresh', $dValue, array('id' => $v['id']));
                            }
                        }else if ($statis['rating_type'] == 1){

                            if (($statis['breakjob_num'] + $freeNum) >= $this->config['sy_reserve_refresh_price']){

                                $dValue =   array('last_time' => $LastTime, 'next_time' => strtotime('+ '.$v['interval'].' minutes', $LastTime));
                                $jValue =   array('lastupdate' => $LastTime);

                                if (strtotime('+ '.$v['interval'].' minutes', $LastTime) >= ($v['end_time'] - 1) && $v['end_time'] > 0){

                                    $jValue['is_reserve']   =   0;
                                    $dValue['status']       =   2;
                                }

                                $result    =   $this->update_once('company_job', $jValue, array('id' => $v['job_id'], 'uid' => $v['uid'], 'state' => 1, 'r_status' => 1, 'status'=> 0));

                                if ($result) {

                                    $this->update_once('reserve_refresh', $dValue, array('id' => $v['id']));
                                    $this->update_once('company', array('lastupdate' => $LastTime), array('uid' => $v['uid']));

                                    $subNum =   $this->config['sy_reserve_refresh_price'] - $freeNum > 0 ? $this->config['sy_reserve_refresh_price'] - $freeNum : 0;

                                    $this->update_once('company_statis', array('breakjob_num' => array('-', $subNum)), array('uid' => $v['uid']));

                                    $this->update_once('hot_job', array('lastupdate' => $LastTime), array('uid' => $v['uid']));

                                    if ($freeNum > 0){

                                        $free_num   =   $subNum == 0 ? $this->config['sy_reserve_refresh_price'] : 0;
                                        $this->addJobSxLog(array('uid' => $v['uid'], 'usertype' => 2, 'jobid' => $v['job_id'], 'r_time' => $LastTime, 'type' => 1, 'remark' => '计划任务：职位预约刷新'), 1, $free_num);
                                    }else{

                                        $this->addJobSxLog(array('uid' => $v['uid'], 'usertype' => 2, 'jobid' => $v['job_id'], 'r_time' => $LastTime, 'type' => 1, 'remark' => '计划任务：职位预约刷新'), 2, 0);
                                    }

                                    if ($subNum > 0) {

                                        $payDetail  =   $freeNum > 0 ? '刷新操作，消耗刷新套餐数量：'.$subNum.'；免费刷新套餐数量：'.$freeNum : '刷新操作，消耗刷新套餐数量：'.$subNum;
                                    }else if ($freeNum > 0){

                                        $payDetail  =   '刷新操作，消耗免费刷新套餐数量：'.$freeNum;
                                    }
                                    $this->addStatisDetail(array('uid' => $v['uid'], 'type' => 2, 'num' => $this->config['sy_reserve_refresh_price'], 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                                }
                            }
                        }
                    }else{

                        $this->update_once('company_job', array('is_reserve' => 0), array('id' => $v['job_id'], 'uid' => $v['uid']));
                        $this->update_once('reserve_refresh', array('status' => 2), array('id' => $v['id']));
                    }
                }
            }
        }
    }

    /**
     * @desc    管理员操作，关闭预约刷新
     * @param array $data
     * @return array
     */
    function closeReserve($data = array())
    {

        if (isset($data['jobids']) && !empty($data['jobids'])) {

            $this->update_once('company_job', array('is_reserve' => 0), array('id' => array('in', $data['jobids'])));
            $this->update_once('reserve_refresh', array('status' => 2), array('job_id' => array('in', $data['jobids'])));

            return array('errcode' => 9, 'msg' => '预约刷新关闭成功');
        } else if (isset($data['auto']) && $data['auto'] == 1) {

            $where  =   array(
                'is_reserve'        =>  1,
                'PHPYUNBTWSTART_A'  =>  '',
                'state'     =>  array('<>', 1, ''),
                'status'    =>  array('=', 1, 'OR'),
                'r_status'  =>  array('<>', 1, 'OR'),
                'PHPYUNBTWEND_A'    =>  ''
            );
            $jobList = $this->select_all('company_job', $where, '`id`');
            if (!empty($jobList)){

                $jobIdS =   array();
                foreach ($jobList as $k => $v){

                    $jobIdS[]   =   $v['id'];
                }
                $this->update_once('company_job', array('is_reserve' => 0), array('id' => array('in', pylode(',', $jobIdS))));
                $this->update_once('reserve_refresh', array('status' => 2), array('job_id' => array('in', pylode(',', $jobIdS))));

                return array('errcode' => 9, 'msg' => '预约刷新关闭成功');
            }
        } else {

            return array('errcode' => 8, 'msg' => '参数错误');
        }
    }

    /**
     * @desc 管理员/业务员 批量投递
     * @param array $where
     * @param array $extData
     * @return array|int[]
     * @throws SmartyException
     */
    function applyJobByAdmin($where = array(), $extData = array())
    {

        if(!empty($where['eid']) && !empty($where['job_id'])){

            $where['isdel'] = 9;
            $sqInfo     =   $this->select_once('userid_job', $where);
            $jobInfo    =   $this->select_once('company_job', array('id' => $where['job_id'], 'uid' =>$where['com_id']),'`id`, `name`, `uid`, `com_name`,`is_link`,`is_message`,`is_email`');

            if(empty($sqInfo) && !empty($jobInfo)){

                $value  =   array(
                    'uid'       =>  $where['uid'],
                    'job_id'    =>  $jobInfo['id'],
                    'job_name'  =>  $jobInfo['name'],
                    'com_id'    =>  $jobInfo['uid'],
                    'com_name'  =>  $jobInfo['com_name'],
                    'eid'       =>  $where['eid'],
                    'datetime'  =>  time(),
                    'type'      =>  1,
                    'is_browse' =>  1,
                );

                $nid    =   $this->insert_into('userid_job', $value);

                if (isset($nid)) {

                    $uid        =   $where['uid'];
                    $eid        =   $where['eid'];
                    $jobid      =   $jobInfo['id'];
                    $comid      =   $jobInfo['uid'];

                    $is_link    =   $jobInfo['is_link'];
                    $is_message =   $jobInfo['is_message'];
                    $is_email   =   $jobInfo['is_email'];

                    // 处理向企业发送短信、邮件
                    if (($this->config['sy_email_set'] == 1 || $this->config['sy_msg_isopen'] == 1)) {

                        if ($is_link == 1) {

                            $job_link   =   $this->select_once('company', array('uid' => $comid), '`linkmail` as email,`linktel` as link_moblie');
                        } elseif ($is_link == 2) {

                            $linkIdInfo =   $this->select_once('company_job', array('id' => $jobid), 'link_id');

                            $job_link   =   $this->select_once('company_job_link', array('id' => $linkIdInfo['link_id']), '`email`,`link_moblie`');
                        }

                        if ($this->config['sy_email_set'] == 1 && $this->config['sy_email_sqzw'] == 1 && !empty($job_link['email']) && $is_email == 1) {

                            include_once('resume.model.php');
                            $resumeM        =   new resume_model($this->db, $this->def);
                            $Info           =   $resumeM->getInfoByEid(array('eid' => $eid));
                            // 简历模糊化
                            $resumeCheck    =   $this->config['resume_open_check'] == 1 ? 1 : 2;
                            global $phpyun;
                            $phpyun->assign('Info', $Info);
                            $phpyun->assign('resumeCheck', $resumeCheck);
                        }
                    }

                    //5.0推送
                    include_once('push.model.php');
                    $pushM = new push_model($this->db, $this->def);
                    $pushM->pushMsg('jobNewResume', array('fuid' => $uid, 'puser' => $comid, 'tid' => $nid, 'jobname' => $jobInfo['name']));
                    // 记录会员日志
                    $logContent =   '职位申请：申请企业「'.$jobInfo['com_name'].'」 → 职位《'.$jobInfo['job_name'].'》';
                    $this->addMemberLog($uid, 1, $logContent, 6, 1);
                    //微信
                    include_once('weixin.model.php');
                    $Weixin = new weixin_model($this->db, $this->def);
                    $res = $Weixin->sendWxJob($uid, $jobid);
                    if (!$res || $this->config['wx_xxtz']!='1' || !$this->config['wx_mbsqzw'] || $this->config['sy_msg_tz'] == 1){
                        include_once('notice.model.php');
                        $noticeM    =   new notice_model($this->db, $this->def);
                        if ($this->config['sy_email_set'] == 1 && $this->config['sy_email_sqzw'] == 1 && !empty($job_link['email']) && $is_email == 1) {
                            $contents       =   $phpyun->fetch(TPL_PATH . 'resume/sendresume.htm', time());
                            $emaildata      =   array(
                                'email'     =>  $job_link['email'],
                                'subject'   =>  "您收到一份新的求职简历！——" . $this->config['sy_webname'],
                                'content'   =>  $contents,
                                //发送email记录到数据表email_msg
                                'uid'       =>  $comid,
                                'name'      =>  $jobInfo['com_name'],
                                'cuid'      =>  '',
                                'cname'     =>  '',
                                'tbContent' =>  '简历详情eid:' . $eid
                            );
                            $noticeM->sendEmail($emaildata);
                        }
                        if ($this->config['sy_msg_isopen'] == 1 && $this->config['sy_msg_sqzw'] == 1 && !empty($job_link['link_moblie']) && $is_message == 1) {

                            $msgdata    =   array(
                                'uid'       =>  $comid,
                                'name'      =>  $jobInfo['com_name'],
                                'cuid'      =>  '',
                                'cname'     =>  '',
                                'type'      =>  'sqzw',
                                'jobname'   =>  $jobInfo['name'],
                                'date'      =>  date('Y-m-d'),
                                'moblie'    =>  $job_link['link_moblie'],
                                'port'      =>  '2'
                            );
                            $noticeM->sendSMSType($msgdata);
                        }
                    }
                    // 处理申请统计
                    include_once('statis.model.php');
                    $statisM = new statis_model($this->db, $this->def);
                    $statisM->upInfo(array('sq_job' => array('+', 1)), array('uid' => $comid, 'usertype' => 2));
                    $statisM->upInfo(array('sq_jobnum' => array('+', 1)), array('uid' => $uid, 'usertype' => 1));

                    $return =   array('msg'=>'投递成功','errcode' => 9);
                }else{

                    $return =    array('msg'=>'网络错误请重试','errcode' => 8);
                }
            }else{

                $return     =   array('msg'=>'该简历已投递','errcode' => 8);
            }
        }else{

            $return         =   array('errcode' => 8, 'msg' => '请选择要投递的简历');
        }

        return $return;
    }

    /**
     * @desc    海报职位
     * @param   array $where
     * @param   array $data
     * @return  array|bool|false|string|void
     */
    // public function getHbJobList($where = array(), $data = array())
    // {

    //     $select   =  isset($data['field']) ? $data['field'] : '*';

    //     $List     =  $this -> select_all('company_job',$where, $select);

    //     if (!empty($List)){

    //         foreach ($List as $k => $v) {

    //             $List[$k]['job_salary']  =  salaryUnit($v['minsalary'], $v['maxsalary'], 1);
    //         }
    //     }

    //     return $List;
    // }

    /**
     * @desc 职位刷新日志
     * @param array $data
     * @return void
     */
    private function addJobSxLog($data = array(), $free = 2, $free_num = 0)
    {

        require_once('log.model.php');
        $logM   =   new log_model($this->db, $this->def);
        return $logM->addJobSxLog($data, $free, $free_num);
    }

	/**
     * @desc 权益预警通知
     * @param array $data
     */
    private function sendRatingNotice($data = array())
    {
        require_once('statis.model.php');
        $statisM    =   new statis_model($this->db, $this->def);
        return $statisM->sendRatingNotice($data);
    }

    /**
     * @desc 套餐消耗明细
     * @param array $data
     * @return mixed
     */
    private function addStatisDetail($data = array())
    {
        require_once('statis.model.php');
        $statisM = new statis_model($this->db, $this->def);
        return $statisM->addStatisDetail($data);
    }
	/**
     *users:王旭
     *Data:2023/2/4
     *Time:9:46
     * @param array $upData
     * @param array $whereData
     * @return array
     */
    public function setlinkopen($upData = array(), $whereData = array())
    {
        if (!empty($upData) && !empty($whereData)) {
            $nid = $this->update_once('company_job', $upData, $whereData);
        }
        if($nid){
            return array('msg'=>'设置成功！','errcode'=>9);
        }else{
            return array('msg'=>'设置失败！','errcode'=>8);
        }
    }
    /**
     *users:王旭
     *Data:2023/2/13
     *Time:14:27
     * @param array $data
     * @return array
     */
    public function getJobLink($data = array())
    {

        $id =   intval($data['id']);
        if (empty($id)) {

            $resData['msg'] =   '参数错误';
            return $resData;
        }

        $jobInfo = $this->select_once('company_job', array('id' => $id), 'id,uid,is_link,rating,link_id');
        if (empty($jobInfo)) {

            $resData['msg'] =   '数据错误';
            return $resData;
        }
        $comInfo            =   $this->select_once('company', array('uid' => $jobInfo['uid']), 'uid, linkman, linktel, linkphone, linkmail, address, rating, infostatus, not_disturb, cityid, x, y');
        $Info['link_id']    =   $jobInfo['link_id'];
        $Info['is_link']    =   $jobInfo['is_link'];
        $Info['linkman']    =   $comInfo['linkman'];
        $Info['linktel']    =   $comInfo['linktel'];
        $Info['linkphone']  =   $comInfo['linkphone'];
        $Info['linkmail']   =   $comInfo['linkmail'];
        $Info['address']    =   $comInfo['address'];
        $Info['cityid']     =   $comInfo['cityid'];
        $Info['rating']     =   $comInfo['rating'];

        if (!empty($comInfo['x']) && !empty($comInfo['y'])) {

            $latlng         =   latlngCheck(array('lat' => $comInfo['y'], 'lng' => $comInfo['x']));
            $Info['x']      =   $latlng['lng'];
            $Info['y']      =   $latlng['lat'];
        }
        if (!empty($Info['link_id'])) {
            $resData        =   $this->getContactNew($Info);
        } else {
            $resData        =   $Info;
        }
        return $resData;
    }

}
?>