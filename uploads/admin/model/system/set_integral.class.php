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
class set_integral_controller extends adminCommon{
	function index_action()
	{
	    $data = array(
	        'integral_pricename'=> $this->config['integral_pricename'],
	        'integral_priceunit'=> $this->config['integral_priceunit'],
	        'integral_min_recharge'=> $this->config['integral_min_recharge'],
	        'money_min_recharge'=> $this->config['money_min_recharge'],
	        'paypack_max_recharge'=> $this->config['paypack_max_recharge'],
	        'packprice_min_recharge'=> $this->config['packprice_min_recharge'],
	        'integral_proportion'=> $this->config['integral_proportion'],
	        'integral_signin'=> $this->config['integral_signin'],
	        'integral_reg'=> $this->config['integral_reg'],
	        'integral_login'=> $this->config['integral_login'],
	        'integral_userinfo'=> $this->config['integral_userinfo'],
	        'integral_emailcert'=> $this->config['integral_emailcert'],
	        'integral_mobliecert'=> $this->config['integral_mobliecert'],
	        'integral_avatar'=> $this->config['integral_avatar'],
	        'integral_question'=> $this->config['integral_question'],
	        'integral_answer'=> $this->config['integral_answer'],
	        'integral_answerpl'=> $this->config['integral_answerpl'],
	        'integral_invite_reg'=> $this->config['integral_invite_reg'],
	        'integral_bind_wx'=> $this->config['integral_bind_wx'],
	        'integral_add_resume'=> $this->config['integral_add_resume'],
	        'integral_identity'=> $this->config['integral_identity'],
	        'integral_comcert'=> $this->config['integral_comcert'],
	        'integral_banner'=> $this->config['integral_banner'],
	        'integral_map'=> $this->config['integral_map'],
	        'integral_ltcert'=> $this->config['integral_ltcert'],
	        'integral_px_banner'=> $this->config['integral_px_banner'],
        );
        $this->render_json(0,'',$data);
    }

	function save_action(){

		$configM    =   $this->   MODEL("config");
        foreach($_POST as $key=>$v){
            $where['name']      =  $key;
            $config = $configM   ->getNum($where);
            if($config==false){
                $data      =  array(
                    'name'    =>  $key,
                    'config'  =>  $v,
                );
                $configM  ->  addInfo($data);

            }else{
                $where['name']  =   $key;
                $data           =   array(
                    'config'		=>  $v
                );
                $configM  ->  upInfo($where,$data);
            }
        }
        $this->cache_action();
        $this->web_config();
        $this->admin_json(0,$this->config['integral_pricename']."配置修改成功！");
	}
	function class_action(){
		$integralM		=	$this->MODEL('integral');
		$list			=	$integralM->getIntClass(array('id'=>array('>',0) , 'orderby'=>'integral,asc' ));
        foreach ($list as &$value){
            $value['status']  = $value['state'] == 1?true:false;
            $value['isEditjifen'] = false;
            $value['isEditdiscount'] = false;
        }
        $this->render_json(0,'',$list);

    }


	//删除
	function del_action()
	{
		$integralM			=	$this->MODEL('integral');
		if((int)$_POST['delid'])
		{
			$ids			=	$_POST['delid'];
		}
        //批量删除
		if($_POST['del']) {
			$ids			=	$_POST['del'];
		}

		$id					=	$integralM->delIntClass($ids);
        $this->cache_action();

        if($id) {
			$this->admin_json(0,'积分优惠(ID:'.$ids.')删除成功！');
		}else{
            $this->render_json(1,"删除失败！");
		}

	}
	
	function cache_action()
	{
		include(LIB_PATH."cache.class.php");
		$cacheclass		= 	new cache(PLUS_PATH,$this->obj);
		$makecache		=	$cacheclass->integralclass_cache("integralclass.cache.php");
	}
	
	function ajax_action(){
		$integralM		=	$this->MODEL('integral');
		
		if((int)$_POST['id']&&$_POST['type']){
			
			$state		=	(int)$_POST['rec']==1 ? 1:2;
			
			$nid		=	$integralM->upIntClass(array('id' =>(int)$_POST['id']) , array($_POST['type']=>$state));
			
			$this->MODEL('log')->addAdminLog($this->config['integral_pricename']."类型(ID:".$_POST['id'].")修改状态！");
		}
		if($_POST['integral']){
			
			$integralclass		=	$integralM->getIntClass(array('integral'=>(int)$_POST['integral'] , 'id'=>array('<>',$_POST['id']) ));
			if($integralclass){
                $this->render_json('1','操作失败');
			}
			
			$nid				=	$integralM->upIntClass(array('id' =>(int)$_POST['id']) , array('integral'=>$_POST['integral']));
			
			$this->MODEL('log')->addAdminLog($this->config['integral_pricename']."充值类型(ID:".$_POST['id'].")修改数量！");
		}
		
		if(isset($_POST['discount'])&& $_POST['discount']>=0){
			$nid				=	$integralM->upIntClass(array('id' =>(int)$_POST['id']) , array('discount'=>$_POST['discount']));
			
			$this->MODEL('log')->addAdminLog($this->config['integral_pricename']."充值类型(ID:".$_POST['id'].")修改折扣！");
		}
		
		$this->cache_action();
        if($nid){
            $this->render_json('0','操作成功');
        }else{
            $this->render_json('1','操作失败');
        }
	}

    /**
     * 会员-个人-个人设置: 积分设置
     */
    function saveSet_action(){

        $post  =  array(
            'integral_add_resume'       =>  $_POST['integral_add_resume'],
            'integral_identity'         =>  $_POST['integral_identity']
        );
        $configM  =  $this -> MODEL('config');
        $configM -> setConfig($post);
        $this -> web_config();
        $this->admin_json(0,'个人积分配置修改成功！');
    }


    /**
     * 设置 积分设置 企业积分
     */
    function comjifen_action(){
        $configM    =   $this->   MODEL("config");

        if($_FILES['exa_cert_wt_files']){
            $upArr  =  array(
                'file'  =>  $_FILES['exa_cert_wt_files'],
                'dir'   =>  'comdoc'
            );
            $uploadM  =  $this->MODEL('upload');

            $result   =  $uploadM -> uploadDoc($upArr);

            if ($result['msg']){
                $this->render_json(1,$result['msg']);
            }else{
                $_POST['exa_cert_wt']   =   $result['docurl'];
            }
        }
        if (isset($_POST['sy_only_price'])) {
            $_POST['sy_only_price']     =   $_POST['sy_only_price'] ? @implode(',', $_POST['sy_only_price']) : '';
        }
        if(isset($_POST['com_single_can'])){
            $_POST['com_single_can']    =   $_POST['com_single_can'] ? @implode(',', $_POST['com_single_can']) : '';
        }
        $configM -> setConfig($_POST);
        $this->web_config();
        $this->admin_json(0,"企业设置配置修改成功！");
    }

}

?>