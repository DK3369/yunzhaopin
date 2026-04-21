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

class emaillog_controller extends adminCommon
{

    function index_action()
    {
        
        $msgM   =   $this->MODEL('email');

        $where['del']       =   array('<>',1);
        
        if(trim($_POST['keyword'])){
            
            $_POST['keyword']    =   trim($_POST['keyword']);

            if ($_POST['type']=='1'){
                
                $where['email']    =   array('like',$_POST['keyword']);
                
            }else if($_POST['type']=='2'){
                
                if($_POST['keyword'] == '系统'){

                    $where['cuid']  =   0;

                }else{

                    $mwhere=array(
                        '1'=>array(
                            'name'      =>  array('like',$_POST['keyword']),
                            'limit'     =>  '50'
                        ),
                        '2'=>array(
                            'name'      =>  array('like',$_POST['keyword']),
                            'limit'     =>  '50'
                        ),
                        '3'=>array(
                            'realname'  =>  array('like',$_POST['keyword']),
                            'limit'     =>  '50'
                        ),
                        '4'=>array(
                            'name'      =>  array('like',$_POST['keyword']),
                            'limit'     =>  '50'
                        )
                    );
                    $userinfoM      =   $this       ->  MODEL('userinfo');
                    $muids          =   $userinfoM  ->  getUidsByWhere($mwhere);
                    $where['PHPYUNBTWSTART_A']  =   '';
                    $where['cuid'][] =   array('in',  pylode(',', $muids));
                    $where['cuid'][] =   array('<>', '0');
                    $where['PHPYUNBTWEND_A']    =   '';
                }
                 
            }else if($_POST['type']=='3'){
                
                if($_POST['keyword']=='系统'){

                    $where['uid']   =   0;

                }else{
                    $mwhere=array(
                        '1'=>array(
                            'name'      =>  array('like',$_POST['keyword']),
                            'limit'     =>  '50'
                        ),
                        '2'=>array(
                            'name'      =>  array('like',$_POST['keyword']),
                            'limit'     =>  '50'
                        ),
                        '3'=>array(
                            'realname'  =>  array('like',$_POST['keyword']),
                            'limit'     =>  '50'
                        ),
                        '4'=>array(
                            'name'      =>  array('like',$_POST['keyword']),
                            'limit'     =>  '50'
                        )
                    );
                    $userinfoM      =   $this       ->  MODEL('userinfo');
                    $muids          =   $userinfoM  ->  getUidsByWhere($mwhere);
                    $where['PHPYUNBTWSTART_A']  =   '';
                    $where['uid'][] =   array('in',  pylode(',', $muids));
                    $where['uid'][] =   array('<>', '0');
                    $where['PHPYUNBTWEND_A']    =   '';
                }
                 
            }else if($_POST['type']=='4'){
                
                $where['smtpserver']   =   array('like',$_POST['keyword']);
                 
            }
        }

        if ($_POST['date1'] && $_POST['date2']) {

            $where['ctime'][]   =   array('>=', $_POST['date1']);
            $where['ctime'][]   =   array('<=', $_POST['date2']+86399);
        }

        $pageM          =   $this->MODEL('page');
        $pages          =   $pageM->adminPageList('email_msg', $where,$_POST['page'],array('limit'=>$_POST['limit']));
        if ($pages['total'] > 0) {
            if($_POST['order']){
                if($_POST['order']=="desc"){
                    $where['orderby']   =   $_POST['t'].',desc';
                }else{
                    $where['orderby']   =   $_POST['t'].',asc';
                }
            }else{
                $where['orderby']       =   'id,desc';
            }
            $where['limit']     =   $pages['limit'];
            $list               =   $msgM -> getEmsgList($where);
        }
        $data['list']           =   !empty($list['list'])?$list['list']:array();
        $data['total']          =   $pages['total'];
        $data['page_sizes']     =   $pages['page_sizes'];
        $data['page_size']     =   $pages['limit'][1];
        $this->render_json(0, '', $data);
    }
    function del_action(){

        $msgM   =   $this->MODEL('email');
        
        if(is_array($_POST['del'])){
            
            $where['id']    =   array('in',pylode(',',$_POST['del']));
            
            $del            =   $msgM->delEmailMsg($where,array('type'=>'all'));
            
            $layer_type     =   1;
            
            $delid          =   pylode(',',$_POST['del']);
            
        }else{
            
            $where['id']    =   (int)$_POST['del'];
            
            $del            =   $msgM->delEmailMsg($where,array('type'=>'one'));
            
            $layer_type     =   0;
            
            $delid          =   (int)$_POST['del'];
        }
    
        if(!$delid){
            
            $this->render_json(1, '请选择要删除的内容！');
        
        }
        
        if($del){
            $msg = '邮件记录(ID:'.$delid.')删除成功！';
            $error = 0;
        }else{
            $msg = '删除失败！';
            $error = 1;
        }

        $this->admin_json($error,$msg);
    
    }   
    //失败邮件重发
    function repeat_action()
    {

        if ($_POST['id']) {

            $emailM =   $this->MODEL('email');
            $msg    =   $emailM->repeat($_POST['id']);
            $error      =   0;
        } else {
            $error      =   1;
            $msg = "请选择需要重发的短信！";
        }
        $this->render_json($error,$msg);
    }
}

?>