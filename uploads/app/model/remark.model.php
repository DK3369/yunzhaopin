<?php

/**
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
 * 版权所有 2009-2022 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */
class remark_model extends model{

    private function addMemberLog($uid,$usertype,$content,$opera='',$type='',$detail='') {
        require_once('log.model.php');
        $LogM = new log_model($this->db, $this->def);
        return $LogM->addMemberLog($uid, $usertype, $content, $opera, $type, $detail);
    }

    /**查询 resume_remark 表数据，单条查询
     * @param array $where
     * @param array $data
     * @return array|bool|false|string|void
     */
    function getRemarkInfo($where = array(), $data = array()) {

        $select  =  $data['field'] ? $data['field'] : '*';

        $List   =  $this -> select_once('resume_remark',$where,$select);

        return $List;

    }

    /**查询 resume_remark 表数据，多条查询
     * @param array $where
     * @param array $data
     * @return array|bool|false|string|void
     */
    function getRemarkList($where = array(), $data = array()) {

        $select  =  $data['field'] ? $data['field'] : '*';

        $List   =  $this -> select_all('resume_remark',$where,$select);

        return $List;
    }

    /**添加简历备注
     * @param array $data
     * @return mixed
     */
    public function Remark($data = array()){
        // 去除换行符
        $data['remark'] = str_replace(PHP_EOL, '', $data['remark']);
        if($data['remark']==''){
            $return['msg']      =  '备注内容不能为空！';
            $return['errcode']  =  8;
        }
        // 设置简历备注，先查询此简历是否有备注
        $row = $this->select_once('resume_remark', array('eid'=>$data['eid'],'uid'=>$data['uid'],'comid'=>$data['comid']));
        
        if (!empty($row)){
            // 存在备注，进行更新
            $id  =   $this->update_once('resume_remark',array('remark'=>$data['remark'],'status' => $data['status'],'ctime'=>time()),array('id'=>$row['id']));
        }else{
            // 没有备注，添加备注
            $Rdata = array(
                'eid'     =>  $data['eid'],
                'comid'   =>  $data['comid'],
                'uid'     =>  $data['uid'],
                'remark'  =>  $data['remark'],
                'status'  =>  $data['status'],
                'ctime'   =>  time()
            );
            
            $id  =  $this->insert_into('resume_remark', $Rdata);
        }
        
        if ($id){
            $logContent =   '信息备注：备注备注';
            $logDetail  =   '简历（ID：'.$data['eid'].'）备注：'.$data['remark'];
            $this -> addMemberLog($data['uid'],$data['usertype'],$logContent,5,1, $logDetail);
            $return['msg']      =  '备注成功！';
            $return['errcode']  =  '9';
        }else{
            $return['msg']      =  '备注失败！';
            $return['errcode']  =  '8';
        }
        return $return;
    }

}