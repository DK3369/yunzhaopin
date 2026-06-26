<?php


class black_model extends model
{
    /**
     * @desc 引用log类，添加用户日志
     *
     * @param int $uid
     * @param int $usertype
     * @param string $content
     * @param string $opera
     * @param string $type
     * @param string $detail
     * @return void
     */
    private function addMemberLog($uid, $usertype, $content, $opera = '', $type = '', $detail = '')
    {

        require_once('log.model.php');
        $LogM   =   new log_model($this->db, $this->def);

        return $LogM->addMemberLog($uid, $usertype, $content, $opera, $type, $detail);
    }

    /**
     * @desc 获取黑名单详细信息
     *
     * @param $where
     * @param array $data
     * @return array|bool|false|string|void
     */
    function getBlackInfo($where, $data = array())
    {

        $field  =   $data['field'] ? $data['field'] : '*';
        return $this->select_once('blacklist', $where, $field);
    }

    /**
     * @desc 获取屏蔽企业列表
     *
     * @param array $Where
     * @param array $data
     * @return array|bool|false|string|void
     */
    function getBlackList($Where = array(), $data = array())
    {

        $select =   $data['field'] ? $data['field'] : '*';
        $result = $this->select_all('blacklist', $Where, $select);
        if($result){
            foreach ($result as $k=>$v){
                $result[$k]['wapcom_url'] = Url('wap',array('c'=>'company','a'=>'show','id'=>$v['p_uid']));
            }
        }
        return $result;
    }

    /**
     * @desc 屏蔽企业数量
     *
     * @param array $Where
     * @return array|bool|false|string|void
     */
    function getBlackNum($Where = array())
    {
        return $this->select_num('blacklist', $Where);
    }

    /**
     * @desc 个人面试通知里的屏蔽企业
     *
     * @param array $data
     * @return array
     */
    public function addBlacklist($data = array())
    {

        if ($data['type'] == 'yqms') {  //个人面试通知里的屏蔽企业

            $id         =   $data['id'];
            $uid        =   intval($data['uid']);
            $usertype   =   intval($data['usertype']);

            include_once('job.model.php');
            $jobM       =   new job_model($this->db, $this->def);
            $info       =   $jobM->getYqmsInfo(array('id' => $id), array('field' => 'fid,fname'));

            $arr        =   array(
                'p_uid'     =>  $info['fid'],
                'inputtime' =>  time(),
                'c_uid'     =>  $uid,
                'usertype'  =>  1,
                'com_name'  =>  $info['fname']
            );
            $haves      =   $this->getBlackInfo(array('c_uid' => $uid, 'p_uid' => $info['fid'], 'usertype' => 1));
            if (is_array($haves)) {

                return array('msg' => yun_at('common_00916'), 'errcode' => 8);
            } else {

                $nid    =   $this->insert_into('blacklist', $arr);

                $jobM->delYqms('', array('where' => array('uid' => $uid, 'usertype' => $usertype, 'fid' => $info['fid'])));

                if ($nid) {

                    $logContent =   'common_01246'.$info['uid'];
                    $logDetail  =   'common_01601'.$info['fname'].'common_00964';
                    $this->addMemberLog($data['uid'], $data['usertype'], $logContent, 26, 1, $logDetail);
                    return array('msg' => yun_at('wap_js_00159'), 'errcode' => 9);
                } else {

                    return array('msg' => yun_at('model_00003'), 'errcode' => 8);
                }
            }
        } elseif ($data['cuid']) {      //个人隐私里屏蔽企业

            $cuid   =   $data['cuid'];  //获取企业uid
            if (!empty($cuid)) {
                if (is_array($cuid)) {

                    $ids    =   $cuid;
                } else {

                    $ids    =   @explode(',', $cuid);
                }

                $id         =   pylode(',', $ids);

                require_once('company.model.php');
                $companyM   =   new company_model($this->db, $this->def);
                $company    =   $companyM->getList(array('uid' => array('in', $id)), array('field' => '`uid`,`name`'));

                foreach ($company['list'] as $v) {
                    $cdata  =   array(

                        'p_uid'     =>  $v['uid'],
                        'c_uid'     =>  $data['uid'],
                        'inputtime' =>  time(),
                        'usertype'  =>  1,
                        'com_name'  =>  $v['name']
                    );
                    $this->insert_into('blacklist', $cdata);
                }

                return array('msg' => yun_at('wap_js_00159'), 'errcode' => 9, 'layertype' => 1);
            } else {

                return array('msg' => yun_at('common_01068'), 'errcode' => 8, 'layertype' => 1);
            }
        }
    }

    /**
     * @DESC 删除黑名单
     *
     * @param string $id 格式：单个，如1 ; 批量，如1,2,3
     * @param array $data
     * @return bool
     */
    function delBlackList($id = null, $data = array())
    {

        if (!empty($id) || !empty($data['where'])) {

            $where  =   array();

            if (!empty($id)) {
                if (is_array($id)) {

                    $ids    =   $id;
                    $return['layertype']    =   1;
                } else {

                    $ids    =   @explode(',', $id);
                    $return['layertype']    =   0;
                }

                $id         =   pylode(',', $ids);
                $where['id']=   array('in', $id);
            }

            if ($data['where']) {

                $where      =   array_merge($where, $data['where']);
            }
            $result         =   $this->delete_all('blacklist', $where, '');

            if ($data['uid']) {
                if ($data['type'] == 'all') {

                    $this->addMemberLog($data['uid'], $data['usertype'], 'common_00652', 26, 3);
                } else {

                    $this->addMemberLog($data['uid'], $data['usertype'], 'common_00651', 26, 3);
                }
            }
            $return['errcode']  =   $result ? 9 : 8;
            $return['msg']      =   $result ? yun_at('admin_user_00187') : yun_at('admin_user_00186');
        } elseif ($data['where']) {

            $where  =   $data['where'];

            $nid    =   $this->delete_all('blacklist', $where, '');

            return $nid;
        } else {
            $return['msg']          =   yun_at('common_00921');
            $return['errcode']      =   8;
            $return['layertype']    =   0;
        }

        return $return;
    }

    /**
     * @desc
     * @param array $data
     * @return bool
     */
    public function addBlackone($data = array())
    {

        if (!empty($data)) {

            $nid = $this->insert_into("blacklist", $data);
        }
        return $nid;
    }
}

?>