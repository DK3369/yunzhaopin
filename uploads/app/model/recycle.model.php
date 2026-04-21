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

class recycle_model extends model
{

    /**
     * 获取回收站列表
     * @param $whereData    查询条件
     * @return array|bool|false|string|void
     */
    function getList($whereData)
    {

        $List   =    $this->select_all('recycle', $whereData);
        if (!empty($List)){
            foreach ($List as $k => $v){

                $List[$k]['time_n'] =   date('Y-m-d H:i:s', $v['ctime']);
                $List[$k]['body_n'] = unserialize($v['body']);
            }
        }

        return $List;
    }

    /**
     * 获取回收站详情
     * @param $whereData    查询条件
     * @param array $data   自定义处理数组
     * @return array|bool|false|string|void
     */
    function getInfo($whereData, $data = array())
    {

        if (!empty($whereData)) {

            $data['field']  =   empty($data['field']) ? '*' : $data['field'];

            $Info           =   $this->select_once('recycle', $whereData, $data['field']);
        }

        return $Info;

    }

    /**
     * 创建回收站
     * @param $data
     * @return mixed
     */
    function addInfo($data)
    {

        if (isset($data) && !empty($data)) {

            $return['id']       =   $this->insert_into('outside', $data);
            $return['msg']      =   '回收站(ID:' . $return['id'] . ')';
            $return['errcode']  =   $return['id'] ? '9' : '8';
            $return['msg']      =   $return['id'] ? $return['msg'] . '添加成功！' : $return['msg'] . '添加失败！';

            return $return;
        }
    }


    /**
     * @desc    恢复数据,单表恢复
     * @param array $where
     * @return array
     */
    function recoverTb($where = array())
    {

        $return =   array(

            'errcode'   =>  8,
            'msg'       =>  ''
        );

        if (!empty($where)) {

            $id         =   $where['id'];
            if (is_array($id)) {
                $ids    =   $id;
            } else {
                $ids    =   @explode(',', $id);
            }

            $recycleList    =   $this->getList(array('id' => array('in', pylode(',', $ids))));

            if (!empty($recycleList)) {

                foreach ($recycleList as $v) {

                    $body   =   unserialize($v['body']);
                    $this->insert_into($v['tablename'], $body);
                    $this->delRecycle(array('id' => $v['id']));
                }

                $return['errcode']  =                                                                                            9;
                $return['msg']      =   '数据恢复成功！';
            } else {

                $return['msg']      =   '请选择有效数据进行恢复！';
            }
        } else {

            $return['msg']          =   '参数错误！';
        }

        return $return;
    }

    /**
     * @desc    恢复数据，一键恢复，通过删除插入的总记录进行查询恢复
     * @param   array $where
     * @return  array
     */
    function recoverByIdent($where = array())
    {

        $return =   array(

            'errcode'   =>  8,
            'msg'       =>  ''
        );

        if (!empty($where)) {

            $rLists     =   $this->getList($where);

            if (isset($rLists) && !empty($rLists)) {

                foreach ($rLists as $v) {

                    $body   =   unserialize($v['body']);
                    $this->insert_into($v['tablename'], $body);

                    $this->delRecycle(array('id' => $v['id']));
                }

                $return['errcode']  =   9;
                $return['msg']      =   '数据恢复成功！';

            } else {

                $return['msg']      = '请选择有效数据进行恢复！';
            }
        } else {

            $return['msg']          =   '参数错误！';
        }

        return $return;
    }

    /**
     * 删除数据调用
     * @param $whereData 查询条件 istime    是否根据时间删除
     * @return mixed
     */
    function delRecycle($whereData, $data = array())
    {
        if ($whereData) {
            if (is_array($whereData['id'])) {

                $delId  =   $whereData['id'][1];
            } else {

                $delId  =   $whereData['id'];
            }

            $return['id']   =   $this->delete_all('recycle', $whereData, $data['limit'], '', '1');

            $return['msg']  =   '回收站数据(ID:' . $delId . ')';

            $return['errcode']  =   $return['id'] ? '9' : '8';

            $return['msg']  =   $return['id'] ? $return['msg'] . '删除成功！' : $return['msg'] . '删除失败！';
        } else {

            $return['msg']  =   '请选择您要删除的信息！';
            $return['errcode'] = 8;
        }

        return $return;
    }

    /*
    * 查询数量
    * $whereData 	查询条件
    *
    */
    function getNum($whereData){

        if(!empty($whereData)){

            $num	=	$this -> select_num('recycle',$whereData);

        }

        return $num;

    }
}