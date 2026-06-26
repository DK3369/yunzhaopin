<?php



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
            $return['msg']      =   yun_at('model_00225') . $return['id'] . yun_at('model_00130');
            $return['errcode']  =   $return['id'] ? '9' : '8';
            $return['msg']      =   $return['id'] ? $return['msg'] . 'admin_system_00138' : $return['msg'] . 'admin_system_00137';

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
                $return['msg']      =   yun_at('common_06572');
            } else {

                $return['msg']      =   yun_at('common_00818');
            }
        } else {

            $return['msg']          =   yun_at('wap_01298');
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
                $return['msg']      =   yun_at('common_06572');

            } else {

                $return['msg']      = yun_at('common_00818');
            }
        } else {

            $return['msg']          =   yun_at('wap_01298');
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

            $return['msg']  =   yun_at('model_00226') . $delId . yun_at('model_00130');

            $return['errcode']  =   $return['id'] ? '9' : '8';

            $return['msg']  =   $return['id'] ? $return['msg'] . 'admin_user_00187' : $return['msg'] . 'admin_user_00186';
        } else {

            $return['msg']  =   yun_at('model_00034');
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