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
class shop_reward_controller extends adminCommon{
	function index_action(){
		$redeemM	=	$this->MODEL("redeem");

		if(trim($_POST['keyword'])){
			if($_POST['ctype']=='2'){
				$where['integral']	=	intval($_POST['keyword']);
			}else{
				$where['name']		=	array('like', trim($_POST['keyword']));
			}
		}
		
		if($_POST['nid']){
			$where['nid']			=	$_POST['nid'];
		}
		
		if($_POST['status']){
			if($_POST['status']=='2'){
				$where['status']	=	0;
			}else{
				$where['status']	=	$_POST['status'];
			}
		}
		
		if($_POST['rec']){
			if($_POST['rec']=='2'){
				$where['rec']		=	0;
			}else{
				$where['rec']		=	$_POST['rec'];
			}
		}
		
		if($_POST['hot']){
			if($_POST['hot']=='2'){
				$where['hot']		=	'0';
			}else{
				$where['hot']		=	$_POST['hot'];
			}
		}
        if ($_POST['order']) {
            $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
        } else {
            $where['orderby'] = 'id';
        }

        $pageM = $this->MODEL('page');

        $pages = $pageM->adminPageList('reward', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        $list = [];
        if ($pages['total'] > 0) {
            $where['limit'] = $pages['limit'];
            $list = $redeemM->getList($where);
        }
        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
	}

    function index_base_data_action()
    {
        $redeemM = $this->MODEL("redeem");
        //提取类别
        $classdata = array();
        $acWhere['id'] = array('>', '0');
        $classAll = $redeemM->GetRewardClass($acWhere);
        foreach ($classAll['list'] as $val) {
            $classdata[$val['id']] = $val['name'];
        }

        $search_list[] = array("param" => "status", "name" => '状态', "value" => array("1" => "上架", "2" => "下架"));
        $search_list[] = array("param" => "nid", "name" => '类别', "value" => $classdata);
        $search_list[] = array("param" => "rec", "name" => '推荐', "value" => array("1" => "是", "2" => "否"));
        $search_list[] = array("param" => "hot", "name" => '热门', "value" => array("1" => "是", "2" => "否"));
        $this->render_json(0, 'ok', compact('search_list'));
    }
	
	function add_action(){

        if (isset($_POST['add'])){

            $redeemM	=	$this->MODEL("redeem");
            $info       = array();
            if($_POST['id']){
                $info	=	$redeemM->getInfo(array('id'=>(int)$_POST['id']));
            }

            $cWhere['keyid']	=	'0';
            $class			=	$redeemM->GetRewardClass($cWhere);
            $class = $class['list'];
            $integral_pricename = $this->config['integral_pricename'];
            $this->render_json(0, 'ok', compact('info', 'class','integral_pricename'));
        }else{
            $redeemM	=	$this->MODEL("redeem");

            //处理数据
            if($_FILES['file']['tmp_name']){
                $upArr    =  array(
                    'file'  	=>  $_FILES['file'],
                    'dir'   	=>  'reward',
                );
                //缩率图参数
                $uploadM  =  $this->MODEL('upload');
                $pic      =  $uploadM->newUpload($upArr);
                if (!empty($pic['msg'])){
                    $this->render_json( 1,  $pic['msg']);
                }elseif (!empty($pic['picurl'])){
                    $pictures 	=  	$pic['picurl'];
                }
            }
            if(isset($pictures)){
                $value['pic']		=	$pictures;
            }
            $value['name']			=	$_POST['name'];
            $value['nid']			=	$_POST['nid'];
            $value['tnid']			=	$_POST['tnid'];
            $value['integral']		=	$_POST['integral'];
            $value['restriction']	=	$_POST['restriction'];
            $value['stock']			=	$_POST['stock'];
            $value['sort']			=	$_POST['sort'];
            $value['content']		=	str_replace("&amp;","&",$_POST['content']);
            $value['status']		=	$_POST['status'];
            $value['sdate']			=	time();
            $value['hot']			=	'0';

            if($_POST['id']){
                $nbid	=	$redeemM->upInfo($value,array('id'=>(int)$_POST['id']));
                $this->render_json( isset($nbid)?0:1,  isset($nbid)?'商品更新成功':'更新失败');
            }else{
                $nbid	=	$redeemM->addInfo($value);
                $this->render_json( isset($nbid)?0:1,  isset($nbid)?'商品添加成功':'添加失败');
            }
        }


	}
    function getclass_action(){
        $redeemM	=	$this->MODEL("redeem");
        $nid = $_POST['nid'];
        if (!$nid){
            $this->render_json('1','参数错误,请重试');
        }
        $cWhere['keyid']	=	$nid;
        $class = $redeemM->GetRewardClass($cWhere);
        $class = $class['list'];
        $this->render_json(0,'ok',compact('class'));
    }
	function save_action(){

	}
	
	function status_action(){
		$redeemM	=	$this->MODEL("redeem");
		$id			=	$redeemM->upInfo(array('status'=>$_POST['status']),array('id'=>(int)$_POST['id']));
		$this->MODEL('log')->addAdminLog("商品(ID:".$_POST['id'].")状态设置成功！");

        $this->render_json($id?0:1, $id?'状态设置成功':'设置失败');
	}

	function rec_action(){
		$redeemM	=	$this->MODEL("redeem");
		$id			=	$redeemM->upInfo(array('rec'=>$_POST['rec']),array('id'=>(int)$_POST['id']));
		$this->MODEL('log')->addAdminLog("商品(ID:".$_POST['id'].")推荐设置成功！");
        $this->render_json($id?0:1, $id?'推荐设置成功':'设置失败');
	}

    function hot_action(){
		$redeemM	=	$this->MODEL("redeem");
		$id			=	$redeemM->upInfo(array('hot'=>$_POST['hot']),array('id'=>(int)$_POST['id']));
		$this->MODEL('log')->addAdminLog("商品(ID:".$_POST['id'].")热门设置成功！");
        $this->render_json($id?0:1, $id?'热门设置成功':'设置失败');
	}

	function del_action(){
		$redeemM	=	$this->MODEL("redeem");
		if(is_array($_POST['del'])){
			$where['id']	=	array('in',pylode(',',$_POST['del']));
			$del			=	$redeemM->delReward($where,array('type'=>'all'));
			$delid			=	pylode(',',$_POST['del']);
		}else{
			$where['id']	=	(int)$_POST['del'];
			$del			=	$redeemM->delReward($where,array('type'=>'one'));
			$delid			=	(int)$_POST['del'];
		}
		
		if(!$delid){
            $this->render_json(1, '请选择要删除的内容');
		}
        $this->render_json($del?0:1, $del?'商品删除成功':'删除失败');
	}	

}
?>