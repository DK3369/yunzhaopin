<?php



class yqmb_model extends model
{

    function getInfo($whereData = array(), $data = array())
    {

        if (!empty($whereData)) {

            $field  =   $data['field'] ? $data['field'] : '*';

            $info   =   $this->select_once('yqmb', $whereData, $field);

            if ($info['intertime']) {
                $info['time']       =   $info['intertime'];
                $info['intertime']  =   date('Y-m-d H:i:s', $info['intertime']);
            }

            return $info;
        }
    }

    function getList($whereData = array(), $data = array())
    {

        if (!empty($whereData)) {

            $field  =   $data['field'] ? $data['field'] : '*';

            $List   =   $this->select_all('yqmb', $whereData, $field);
            foreach ($List as $k => $v) {
                if ($v['intertime']) {
                    $List[$k]['intertime']  =   date('Y-m-d H:i', $v['intertime']);
                    $List[$k]['addtime_n']  =   date('Y-m-d H:i', $v['addtime']);
                }
            }
            if ($data['utype'] == 'admin') {
                $List   =   $this->moreListData($List);
            }
            return $List;
        }
    }

    function moreListData($list = array())
    {
        if (!empty($list)) {

            $uids   =   array();
            foreach ($list as $k => $v) {
                if ($v['uid'] && !in_array($v['uid'], $uids)) {

                    $uids[] =   $v['uid'];
                }
            }

            if (!empty($uids)) {

                $comList    =   $this->select_all('company', array('uid' => array('in', pylode(',', $uids))), '`uid`,`name`');

                $comNameArr =   array();

                foreach ($comList as $ck => $cv) {

                    $comNameArr[$cv['uid']]     =   $cv['name'];
                }

                if (!empty($comNameArr)) {

                    foreach ($list as $lk => $lv) {

                        $list[$lk]['comname']   =   $comNameArr[$lv['uid']];
                    }
                }
            }

        }
        return $list;
    }

    function getNum($whereData = array(), $data = array())
    {

        // if (!empty($whereData)) {

            $num = $this->select_num('yqmb', $whereData);

            return $num;
        // }
    }

    /**
     * 添加邀请模板
     *
     * @param array $setData
     * @param array $data
     * @param array $whereData
     * @return array
     */

    function addInfo($setData = array(), $data = array(), $whereData = array())
    {

        $return =   array();

        if (!empty($setData)) {

            if ($data['uid']) {

                $com    =   $this->select_once('company', array('uid' => $data['uid']), '`uid`');


                if (!empty($com)) {

                    $mbNum  =   $this->select_num('yqmb', array('uid' => $com['uid']));


                    if (empty($whereData) && $mbNum >= $this->config['com_yqmb_num']) {

                        $return['error']    =   4;
                        $return['msg']      =   yun_at('common_06678').$this->config['com_yqmb_num'].'member_com_00683';

                    } else {

                        $intertime  =   strtotime($setData['intertime']);

                        if (empty($setData['linkman'])) {

                            $return['msg']  =   yun_at('member_com_00677');
                        } elseif (empty($intertime)) {

                            $return['msg']  =   yun_at('member_com_00681');
                        } elseif ($intertime < time()) {

                            $return['msg']  =   yun_at('common_00752');
                        } elseif (empty($setData['linktel'])) {

                            $return['msg']  =   yun_at('common_06291');
                        } elseif (!CheckMobile($setData['linktel']) && !CheckTell($setData['linktel'])) {

                            $return['msg']  =   yun_at('common_06679');
                        } elseif (empty($setData['address'])) {

                            $return['msg']  =   yun_at('member_com_00680');
                        } else {

                            $setData['name']=   $setData['name'] ? $setData['name'] : $setData['linkman'] . 'admin_00709';

                            $dataV  =   array(
                                'uid'       =>  $com['uid'],
                                'name'      =>  $setData['name'],
                                'content'   =>  $setData['content'],
                                'address'   =>  $setData['address'],
                                'linkman'   =>  $setData['linkman'],
                                'linktel'   =>  $setData['linktel'],
                                'intertime' =>  $intertime,
                                'did'       =>  $setData['did'],
                                'addtime'   =>  time(),
                                'status'    =>  isset($data['status']) ? $data['status'] : $this->config['com_yqmb_status']
                            );


                            if (!empty($whereData)) {

                                $nid            =   $this->update_once('yqmb', $dataV, $whereData);
                                $return['msg']  =   yun_at('common_06680');
                                $id             =   $whereData['id'];
                            } else {
                                $nid            =   $this->insert_into('yqmb', $dataV);
                                $return['msg']  =   yun_at('common_06681');
                                $id             =   $nid;
                            }

                            if ($nid) {
                                $return['error']=   1;
                                $return['msg']  .=  'common_01499' . $id . ')';
                            } else {
                                $return['error']=   2;
                                $return['msg']  .=  'admin_tool_00501';
                            }
                        }
                    }
                } else {
                    $return['error']            =   2;
                    $return['msg']              =   yun_at('common_06682');
                }
            } else {
                //缺少参数uid
                $return['error']    =   2;
                $return['msg']      =   yun_at('common_06682');
            }
        } else {

            $return['error']    =   2;
            $return['msg']      =   yun_at('common_06682');
        }
        $return['errcode']      =   $nid ? '9' : '8';

        return $return;

    }

    public function delYqmb($delId, $data = array())
    {

        if (!empty($delId)) {

            $return['layertype']        =   0;

            if (is_array($delId)) {

                $delId                  =   pylode(',', $delId);
                $return['layertype']    =   1;
            }
        }
        if ($data['uid']) {

            $delWhere       =   array('id' => array('in', $delId), 'uid' => $data['uid']);
        } else {

            $delWhere       =   array('id' => array('in', $delId));
        }

        $return['id']       =   $this->delete_all('yqmb', $delWhere, '');
        $return['errcode']  =   $return['id'] ? '9' : '8';
        $return['msg']      =   $return['id'] ? '邀请面试模板(ID:' . $delId . ')删除成功！' : '邀请面试模板(ID:' . $delId . ')删除失败！';

        return $return;
    }

    public function statusYqmb($id, $upData = array())
    {

        $ids    =   @explode(',', trim($id));

        $return =   array('msg' => yun_at('model_00001'), 'errcode' => 8);

        if (!empty($id)) {

            $idstr  =   pylode(',', $ids);

            $upData =   array(

                'status'        =>  intval($upData['status']),
                'statusbody'    =>  trim($upData['statusbody']),
            );

            $result =   $this->update_once('yqmb', $upData, array('id' => array('in', $idstr)));

            if ($result) {

                if ($upData['status'] == 1 || $upData['status'] == 2) {

                    $msg    =   array();
                    $uids   =   array();

                    $mbs    =   $this->getList(array('id' => array('in', $idstr)), array('field' => '`id`,`uid`,`name`'));

                    foreach ($mbs as $v) {

                        $uids[] =   $v['uid'];
                    }

                    foreach ($mbs as $k => $v) {

                        if ($upData['status'] == 2) {

                            $statusInfo         =   '您的邀请面试模板《' . $v['name'] . '》审核未通过';

                            if ($upData['statusbody']) {

                                $statusInfo     .= 'wap_00800' . $upData['statusbody'];
                            }

                            $msg[$v['uid']][]   =   $statusInfo;
                        } elseif ($upData['status'] == 1) {

                            $msg[$v['uid']][]   =   '您的邀请面试模板《' . $v['name'] . '》审核通过';
                        }
                    }


                    //发送系统通知
                    require_once 'sysmsg.model.php';
                    $sysmsgM    =   new sysmsg_model($this->db, $this->def);
                    $sysmsgM->addInfo(array('uid' => $uids, 'usertype' => 2, 'content' => $msg));
                }

                $return['msg']      =   yun_auto_t('邀请面试模板(ID:') . $idstr . ')审核成功!';

                $return['errcode']  =   9;

            } else {

                $return['msg']      =   yun_auto_t('审核模板(ID:') . $idstr . ')设置失败';
                $return['errcode']  =   8;
            }

        } else {

            $return['msg']      =   yun_at('common_06683');
            $return['errcode']  =   8;
        }

        return $return;
    }


    private function getUserList($whereData, $data = array())
    {

        require_once('userinfo.model.php');
        $UserInfoM  =   new userinfo_model($this->db, $this->def);
        return $UserInfoM->getList($whereData, $data);
    }
}

?>