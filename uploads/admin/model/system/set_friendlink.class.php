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
class set_friendlink_controller extends adminCommon{
	function index_action(){
	    $where = array();
        $linkM	=	$this  -> MODEL('link');
		if($_POST['searchOption']['state']=='1'){
			$where['link_state']	=	1;
		}elseif($_POST['searchOption']['state']=='2'){
		    $where['PHPYUNBTWSTART_A']    =   '';
            $where['link_state'][]  =   array('=',  0);
            $where['link_state'][]  =   array('=', 2,'OR');
            $where['PHPYUNBTWEND_A']      =   '';
		}
		if($_POST['searchOption']['type']){
			$where['link_type']		=	$_POST['searchOption']['type'];
		}
		if($_POST['searchOption']['did']){
			$where['did']			=	$_POST['searchOption']['did'];
		}
		if($_POST['searchOption']['ctime']){
			if($_POST['searchOption']['ctime']=='1'){
				$where['link_time']		=	array('>=',strtotime(date("Y-m-d 00:00:00")));
			}else{
				$where['link_time']		=	array('>',strtotime('-'.intval($_POST['searchOption']['ctime']).' day'));
			}
		}
        if (trim($_POST['searchOption']['keyword'])) {
			if ($_POST['searchOption']['type']=='1'){
				$where['link_name']		=	array('like',trim($_POST['searchOption']['keyword']));
				$where['link_type']		=	1;
			}elseif ($_POST['searchOption']['type']=='2'){
				$where['link_name']		=	array('like',trim($_POST['searchOption']['keyword']));
				$where['link_type']		=	2;
			}else{
				$where['link_name']		=	array('like',trim($_POST['searchOption']['keyword']));
			}
		}
        $total = $linkM->getLinkNum($where);
        if ($_POST['order']) {
            $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
        } else {
            $where['orderby']		=	array('id,desc');
        }
        $page = !empty($_POST['pagination']['page']) ? intval($_POST['pagination']['page']) : 1;
        $pageSize = !empty($_POST['pagination']['pageSize']) ? intval($_POST['pagination']['pageSize']) : intval($this->config['sy_listnum']);
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('admin_link', $where, $page, array('limit' => $pageSize));

        $where['limit'] = array(($page - 1) * $pageSize, $pageSize);
        $List =	$linkM -> getList($where);

        $domainData = $this->getDomain();

        $data = array(
            'list'=> $List,
            'total'=> intval($total),
            'domain'=> $domainData,
            'pageSize'=>$pageSize,
            'pageSizes'=>$pages['page_sizes']
        );
        $this->render_json(0,'ok',$data);
	}

	function getInfo_action(){
		if($_POST['id']){
			$linkM	=	$this  -> MODEL('link');
			$info	=	$linkM -> getInfo(array('id'=>$_POST['id']));
            $this->render_json(0,'ok',$info);
		}
	}
	function getDomain(){
        $cacheM  =	$this -> MODEL('cache');
        $domain  =	$cacheM -> GetCache('domain',$Options=array('needreturn'=>true,'needassign'=>true,'needall'=>true));
        $domainData = array();
        foreach ($domain['Dname'] as $k=>$v){
            $domainData[] = array(
                'label'=>$v,
                'value'=>$k
            );
        }
        return $domainData;
    }
    function getCache_action(){
        $domainData = $this->getDomain();
        $data = array(
            'domain'=> $domainData
        );
        $this->render_json(0,'ok',$data);
    }
	//删除链接
	function del_action(){
        if ($_POST['del']){
            $id	=	$_POST['del'];
            $linkM	=	$this  -> MODEL('link');
            $return	=	$linkM -> delInfo($id);
            $this->admin_json($return['errcode'] == 9 ? 0 : 1,$return['msg']);
        }else{
            $this->render_json(1, '参数错误,请重试');
        }
	}
	//审核链接
	function status_action(){
			
		$id		=	$_POST['formdata']['id'];
		$linkM	=	$this  -> MODEL('link');
		
		$return	=	$linkM -> setLinkStatus($id,array('status'=>$_POST['formdata']['status'],'statusbody'=>$_POST['formdata']['content']));
        $this->admin_json($return['errcode'] == 9 ? 0 : 1,$return['msg']);
	}
	//保存信息
	function save_action(){
		$linkM			=	$this  -> MODEL('link');
		if($_POST['phototype']==1){
			if($_FILES['file']['tmp_name']){
		 		$upArr    =  array(
					'file'  =>  $_FILES['file'],
					'dir'   =>  'link'
				);
				$uploadM  =  $this->MODEL('upload');
				$pic      =  $uploadM->newUpload($upArr);
				if (!empty($pic['msg'])){
                    $this->render_json(1, $pic['msg']);
				}elseif (!empty($pic['picurl'])){
					$pictures 	=  	$pic['picurl'];
				}
		 	}
		}else{
			$pictures		=	$_POST['uplocadpic'];
		}
		$post	=	array(
			'did'			=>	$_POST['did'],
			'link_name'		=>	trim($_POST['title']),
			'link_url'		=>	$_POST['url'],
			'link_type'		=>	$_POST['type'],
			'tem_type'		=>	$_POST['tem_type'],
			'img_type'		=>	$_POST['phototype'],
			'link_sorting'	=>	$_POST['sorting'],
			'link_state'	=>	1,
		);
		if(isset($pictures)){
			$post['pic']	=	$pictures;
		}
		$data	=	array(
			'post'	=>	$post,
			'id'	=>	$_POST['id'],
			'utype'	=>	'admin'
		);
		$return	=	$linkM -> addInfo($data);
        $this->admin_json($return['errcode']==9? 0 : 1, $return['msg']);
	}
	function sitedid_action(){
		$linkM	=	$this  -> MODEL('link');
		$data	=	array(
			'id'=>$_POST['id'],
			'did'=>$_POST['did']
		);
		$return	=	$linkM -> setLinkSite($data);
        $this->admin_json($return['errcode']==9? 0 : 1, $return['msg']);
	}

}

?>