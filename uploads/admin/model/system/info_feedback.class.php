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
class info_feedback_controller extends adminCommon{

	function index_action(){
	    $where = array();
		$adviceM=$this->MODEL('advice');
		if(trim($_POST['keyword'])){
            if ($_POST['type'] == "1" || $_POST['type'] == '') {
				$where['username']	=	array('like',trim($_POST['keyword']));
			}else{
				$where['content']	=	array('like',trim($_POST['keyword']));
			}
		}
		if($_POST['feedbacktime']){
			if($_POST['feedbacktime']=='1'){
				$where['ctime']		=	array('>=',strtotime(date("Y-m-d 00:00:00")));
			}else{
				$where['ctime']		=	array('>=',strtotime('-'.$_POST['feedbacktime'].'day'));
			}
		}
		if($_POST['feedbacktype']){
			$where['infotype']		=	$_POST['feedbacktype'];
		}
		if($_POST['feedbackstatus']){
			$where['status']		=	$_POST['feedbackstatus'];
		}
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('advice_question', $where, $page, array('limit' => $pageSize));

        $total  =   intval($pages['total']);
        if ($total > 0) {

            if($_POST['order']){
                $where['orderby']		=	$_POST['t'].','.$_POST['order'];
            }else{
                $where['orderby']		=	array('id,desc');
            }
            $where['limit'] =   $pages['limit'];
        }
        $List =	$adviceM -> getList($where);
        $data = array(
            'list'=> $List,
            'total'=> intval($total),
            'pageSize'=>intval($pageSize),
            'pageSizes'=>$pages['page_sizes']
        );
        $this->render_json(0,'ok',$data);
	}
	function del_action(){
        $del=$_POST['del'];
        if($del){
            $return	=	$this -> MODEL('advice') -> delInfo($del,'');
            $this->admin_json($return['errcode'] == 9 ? 0 : 1,$return['msg']);
        }else{
            $this->render_json(500, '参数错误,请重试');
        }
	}
	function status_action(){
	    $adviceM	=	$this -> MODEL('advice');

	    $post       =  array(
	        'status'     =>  intval($_POST['status']),
	        'handlecontent'  =>  trim($_POST['content'])
	    );
	    $return     =  $adviceM -> statusInfo($post,array('id'=>$_POST['id']));
        $this->admin_json($return['errcode'] == 9 ? 0 : 1,$return['msg']);
	}

}