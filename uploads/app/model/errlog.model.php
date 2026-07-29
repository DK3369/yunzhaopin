<?php


/**
 * 系统错误日志操作类
 */
class errlog_model extends model{

    /**
     * 添加错误日志
     * @param string $uid
     * @param string $type
     * 1-注册，2-添加简历，3-申请职位，4-发布职位，5:-刷新职位,6:-下载简历，7-邀请面试,8-发送邮件，9-发送短信
     * @param string $content
     */
    public function addErrorLog($uid, $type, $content)
    {

        if ($type && $content) {

            $data   =   array(

                'uid'       =>  $uid,
                'type'      =>  $type,
                'content'   =>  $content,
                'ctime'     =>  time()
            );

            $nid    =   $this->insert_into('error_log', $data);

            if ($nid) {

                $member     =   $this->select_once('member', array('uid' => $uid), 'usertype');

                include_once 'userinfo.model.php';
                $userInfoM  =   new userinfo_model($this->db, $this->def);
                $userInfo   =   $userInfoM->getUserInfo(array('uid' => $uid), array('usertype' => $member['usertype']));

                $wxTempMsg  =   '';

                switch ($member['usertype']) {
                    case '1':
                        $wxTempMsg .= '个人用户《' . $userInfo['name'] . '》' . $content;
                        break;
                    case '2':
                        $wxTempMsg .= '，企业用户《' . $userInfo['name'] . '》' . $content;
                        break;
                }
            }
        }
    }

    public function getList($whereData = array(), $data = array())
    {

        $field  =   empty($data['field']) ? '*' : $data['field'];
        $List   =   $this->select_all('error_log', $whereData, $field);

        if (!empty($List)) {

            if ($data['utype'] == 'admin') {

                $errIdArr   =   array();
                foreach ($List as $k => $v) {

                    $errIdArr[] =   $v['id'];
                }
                $this->update_once('error_log', array('isread' => 1), array('id' => array('in', pylode(',', $errIdArr))));
            }

            foreach ($List as $key => $value) {

                $List[$key]['ctime_n'] = date('Y-m-d H:i:s', $value['ctime']);

                if ($value['type'] == 1) {

                    $List[$key]['type_n'] = yun_at('wap_00242');
                } elseif ($value['type'] == 2) {

                    $List[$key]['type_n'] = yun_at('admin_user_00296');
                } elseif ($value['type'] == 3) {

                    $List[$key]['type_n'] = yun_at('wap_00794');
                } elseif ($value['type'] == 4) {

                    $List[$key]['type_n'] = yun_at('wap_00322');
                } elseif ($value['type'] == 5) {

                    $List[$key]['type_n'] = yun_at('common_06479');
                } elseif ($value['type'] == 6) {

                    $List[$key]['type_n'] = yun_at('wap_00451');
                } elseif ($value['type'] == 7) {

                    $List[$key]['type_n'] = yun_at('resume_00029');
                } elseif ($value['type'] == 8) {

                    $List[$key]['type_n'] = yun_at('admin_user_00166');
                } elseif ($value['type'] == 9) {

                    $List[$key]['type_n'] = yun_at('admin_user_00167');
                }  
            }
            return $List;
        }
    }

    public function delErrlog($whereData = array(), $data = array())
    {

        $return['layertype']    =   0;

        if (!empty($whereData)) {
            if (!empty($whereData['id']) && $whereData['id'][0] == 'in') {

                $return['layertype']    =   1;
            }
            if ($data['norecycle'] == '1') {

                $return['id']   =   $this->delete_all('error_log', $whereData, '', '', '1');
            } else {

                $return['id']   =   $this->delete_all('error_log', $whereData, '');
            }
            $return['msg']      =   yun_at('admin_index_00076');
            $return['errcode']  =   $return['id'] ? '9' : '8';
            $return['msg']      =   $return['id'] ? $return['msg'] . 'admin_user_00187' : $return['msg'] . 'admin_user_00186';
        } else {

            $return['msg']      =   yun_at('common_00745');
            $return['errcode']  =   8;
        }
        return $return;
    }

}

?>