<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

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
                        $return['msg']      =   '最多只能设置'.$this->config['com_yqmb_num'].'个面试模板';

                    } else {

                        $intertime  =   strtotime($setData['intertime']);

                        if (empty($setData['linkman'])) {

                            $return['msg']  =   '联系人不能为空！';
                        } elseif (empty($intertime)) {

                            $return['msg']  =   '面试时间不能为空！';
                        } elseif ($intertime < time()) {

                            $return['msg']  =   '面试时间不能小于当前时间！';
                        } elseif (empty($setData['linktel'])) {

                            $return['msg']  =   '联系方式不能为空！';
                        } elseif (!CheckMobile($setData['linktel']) && !CheckTell($setData['linktel'])) {

                            $return['msg']  =   '手机格式错误';
                        } elseif (empty($setData['address'])) {

                            $return['msg']  =   '面试地址不能为空！';
                        } else {

                            $setData['name']=   $setData['name'] ? $setData['name'] : $setData['linkman'] . '邀请面试模板';

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
                                $return['msg']  =   '面试模板更新';
                                $id             =   $whereData['id'];
                            } else {
                                $nid            =   $this->insert_into('yqmb', $dataV);
                                $return['msg']  =   '面试模板添加';
                                $id             =   $nid;
                            }

                            if ($nid) {
                                $return['error']=   1;
                                $return['msg']  .=  '成功(ID:' . $id . ')';
                            } else {
                                $return['error']=   2;
                                $return['msg']  .=  '失败';
                            }
                        }
                    }
                } else {
                    $return['error']            =   2;
                    $return['msg']              =   '数据异常，请重试';
                }
            } else {
                //缺少参数uid
                $return['error']    =   2;
                $return['msg']      =   '数据异常，请重试';
            }
        } else {

            $return['error']    =   2;
            $return['msg']      =   '数据异常，请重试';
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

        $return =   array('msg' => '非法操作！', 'errcode' => 8);

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

                                $statusInfo     .= '，原因：' . $upData['statusbody'];
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

                $return['msg']      =   '邀请面试模板(ID:' . $idstr . ')审核成功!';

                $return['errcode']  =   9;

            } else {

                $return['msg']      =   '审核模板(ID:' . $idstr . ')设置失败';
                $return['errcode']  =   8;
            }

        } else {

            $return['msg']      =   '请选择需要审核的模板！';
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