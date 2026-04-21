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

class admin_member_controller extends adminCommon
{

    public function getCache_action(){
        include(CONFIG_PATH.'db.data.php');
        $source         =  $arr_data['source'];
        $cacheM =   $this->MODEL('cache');
        $domain =   $cacheM->GetCache('domain');
        $this->render_json(0,'',['source'=>$source,'dname'=>(object)$domain['Dname']]);
    }
	function index_action()
    {

		$userinfoM	=	$this->MODEL('userinfo');

		$where['pid']   =   0;

        if ($_POST['utype']) {
            if ($_POST['utype'] == '5') {
                $where['usertype']  =   0;
            } else {
                $where['usertype']  =   $_POST['utype'];
            }
        }

        if ($_POST['time_type'] == 'adtime') {
            if ($_POST['times']) {
                $time = $_POST['times'];
                $where['PHPYUNBTWSTART_C'] = '';
                $where['reg_date'][] = array('>=', strtotime($time[0] . ' 00:00:00'));
                $where['reg_date'][] = array('<=', strtotime($time[1] . ' 23:59:59'));
                $where['PHPYUNBTWEND_C'] = '';
            }
        }
        if($_POST['time_type'] == 'lotime'){
            if ($_POST['times']) {
                $time = $_POST['times'];
                $where['PHPYUNBTWSTART_D'] = '';
                $where['login_date'][] = array('>=', strtotime($time[0].' 00:00:00'));
                $where['login_date'][] = array('<=', strtotime($time[1].' 23:59:59'));
                $where['PHPYUNBTWEND_D'] = '';
            }
        }
		if($_POST['status']){
			$status				    =	intval($_POST['status']);
			$where['status']	    =	$status;
		}
		if($_POST['source']){
			$where['source']	    =	intval($_POST['source']);
		}
        if (trim($_POST['keyword'])) {
            if ($_POST['type'] == 1) {
                $where['username']  =   array('like', trim($_POST['keyword']));
            } elseif ($_POST['type'] == 2) {
                $where['moblie']    =   array('like', trim($_POST['keyword']));
            } elseif ($_POST['type'] == 3) {
                $where['uid']       =   trim($_POST['keyword']);
            } elseif ($_POST['type'] == 4) {
                $where['PHPYUNBTWSTART_B']  =   '';
                $where['reg_ip']     =   array('like', trim($_POST['keyword']));
                $where['login_ip']     =   array('like', trim($_POST['keyword']),'or');
                $where['PHPYUNBTWEND_B']    =   '';
            }
        }
        if ($_POST['order']) {
            $where['orderby']   =   $_POST['t'].','.$_POST['order'];
        } else {

            $where['orderby']   =   array('uid,desc');
        }
        $page  = $_POST['page'];
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        $pageM	=	$this  -> MODEL('page');
        $pages	=	$pageM -> adminPageList('member',$where,$page,array('limit' => $pageSize));
		if(!$pages['total']){
            $this->render_json(0,'暂无数据',['data'=>[],'total'=>0,'pageSizes'=>$pages['page_sizes']]);
        }

        $where['limit']	=  	$pages['limit'];
		$field = 'uid,username,email,moblie,moblie_status,reg_ip,reg_date,login_ip,login_date,usertype,status,lock_info,source,did,login_address,moblie_address';
        $List			=	$userinfoM->getList($where,array('field'=> $field),'admin');
        foreach ($List as &$v){
            $v['login_date_n'] = $v['login_date']? date('Y-m-d H:i:s',$v['login_date']):'';
            $v['reg_date_n'] = $v['reg_date']? date('Y-m-d H:i:s',$v['reg_date']):'';
        }

        $data = array(
            'data'=>$List,'total'=>(int)$pages['total'],
            'pageSizes'=>$pages['page_sizes']
        );
        $this->render_json(0,'暂无数据',$data);

	}


	function editSave_action()
    {

        $_POST      =   $this->post_trim($_POST);
        if(!$_POST['uid']){
            $this->render_json(1,'参数错误');
        }
        $memberM    =   $this->MODEL('userinfo');
        $uData	=  array(
            'uid'  		=>  intval($_POST['uid']),
            'usertype'  =>  intval($_POST['usertype']),
            'lasturl'   =>  $_POST['lasturl']
        );

        $mData  =  array(
            'username'  =>  $_POST['username'],
            'password'  =>  $_POST['password'],
            'moblie'    =>  $_POST['moblie'],
            'email'     =>  $_POST['email'],
            'reg_ip'	=>	$_POST['reg_ip'],
            'did'		=>	$_POST['did'],
            'status'    =>  $_POST['status'],
            'utype'		=>	'admin'
        );
        $return =   $memberM->upMemberInfo($uData, $mData);
        if ($return['errcode']==9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->render_json(1,$return['msg']);
        }
	}

	/**
	 * 会员用户列表:会员锁定
	 */
    function lock_action()
    {

        $userinfoM  =   $this->MODEL('userinfo');
        if (!$_POST['uid']){
            $this->render_json(1,'参数错误');
        }

        $post       =   array(

            'status'    =>  intval($_POST['status']),
            'lock_info' =>  trim($_POST['lock_info'])
        );

        $return     =   $userinfoM->lock(array('uid' => intval($_POST['uid'])), array('post' => $post));
        if ($return['errcode']==9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->render_json(1,$return['msg']);
        }
    }
	/**
	 * 会员列表（页面统计数量）
	 */
    function memNum_action()
    {

        $MsgNum =   $this->MODEL('msgNum');
        $arr =  $MsgNum->memNumV1();
        $this->render_json('0','',$arr);
    }

	/**
	 * 会员列表（重置密码）
	 */
	function reset_pw_action(){

		$userinfoM  =  $this->MODEL('userinfo');
        if(!$_POST['uid']){
            $this->render_json(1,'参数错误');
        }

		$userinfoM -> upInfo(array('uid'=>intval($_POST['uid'])),array('password'=>'123456'));

		$this -> MODEL('log') -> addAdminLog('会员(ID:'.$_POST['uid'].')重置密码成功');
        $this->render_json('0','修改成功');

    }
	function del_action(){

		$userinfoM	=	$this->MODEL('userinfo');

		if(!$_POST['del']){
            $this->render_json(1,'参数错误');
        }
        $del = $_POST['del'];
		$return	=	$userinfoM->delMember($del);
        if ($return['errcode']==9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->render_json(1,$return['msg']);
        }
	}
	/**
	 * 会员列表（分配分站）
	 */
	function checksitedid_action(){

        if(!$_POST['uid']){
            $this->render_json(1,'参数错误');
        }
        $siteM	=	$this -> MODEL('site');

		$return	=	$siteM -> memberSiteDid($_POST);

        if ($return['errcode']==9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->render_json(1,$return['msg']);
        }
	}
	//发送邮件
	function send_action(){

		$UserInfoM = $this->MODEL('userinfo');


		if($_POST['email_title']==''||$_POST['content']==''){
            $this->render_json(1,"邮件标题均不能为空！");
 		}

		$emailarr=$user=$com=$lt=$px=$userinfo=array();

		if($_POST['utype']!='5'){
			$userrows	=	$UserInfoM->getList(array('usertype'=>$_POST['utype']),array('field'=>'email,`uid`,`usertype`'));
		}else if($_POST['utype']=='5'){
			$email_user	=	@explode(',',$_POST['email_user']);
			$email_user	=	array_filter($email_user);
			foreach($email_user as $v){
			    if(CheckRegEmail($v)){
			        $earr[]=$v;
			    }
			}
			if(!empty($earr)){
				$where['email']=array('in', "\"".@implode('","',$earr)."\"");
				$userrows	=	$UserInfoM->getList($where,array('field'=>'`email`,`uid`,`usertype`'));
			}
		}
		if(is_array($userrows)&&$userrows){
			foreach($userrows as $v){

				if($v['usertype']=='1'){	$user[]	=	$v['uid'];}
				if($v['usertype']=='2'){	$com[]	=	$v['uid'];}
				if($v['usertype']=='3'){	$lt[]	=	$v['uid'];}
				if($v['usertype']=='4'){	$px[]	=	$v['uid'];}

				$emailarr[$v['uid']]=$v["email"];
			}

			if($user&&is_array($user)){
				$where['uid']	=	array('in',pylode(',',$user));
			}
			if($com&&is_array($com)){
				$where['uid']	=	array('in',pylode(',',$com));
			}
			if($lt&&is_array($lt)){
				$where['uid']	=	array('in',pylode(',',$lt));
			}
			if($px&&is_array($px)){
				$where['uid']	=	array('in',pylode(',',$px));
			}
			$List	=	$UserInfoM->getUserList($where);

			foreach($List as $v){
				$userinfo[$v['uid']]=$v['name'];
			}
		}

		if(!count($emailarr)){
            $this->render_json(1,"没有符合条件的邮箱，请先检查！");
		}else{
			set_time_limit(10000);

			$pagesize	=	intval($_POST['pagelimit']);
			$sendok		=	intval($_POST['sendok']);
			$sendno		=	intval($_POST['sendno']?$_POST['sendno']:0);
			$value		=	intval($_POST['value']);

			//分批次发送
			$result		=	$this->send_email($emailarr,$_POST['email_title'],$_POST['content'],$userinfo,$pagesize,$sendok,$sendno,$value);

			if($result){
				$toSize = $pagesize * $result['page'];

				if(count($emailarr) > $toSize){
					$npage	=	$result['page'];
					$spage	=	$npage*$pagesize+1;
					$topage	=	($npage+1)*$pagesize;
					$name	=	$spage."-".$topage;

					$this->get_return("3",$result['page'],"正在发送".$name."封邮件",$result['sendok'],$result['sendno']);
				}else{
                    $this->render_json(0,'发送成功');
//					$this->get_return("1",$result['page'],"发送成功:".$result['sendok'].",失败:".$result['sendno']);
 				}
			}
		}
 	}

	function send_email($email=array(),$emailtitle="",$emailcoment="",$userinfo=array(),$pagesize,$sendok,$sendno,$value){

		$notice = $this->MODEL('notice');
		$sendok = intval($sendok);
		$sendno = intval($sendno);

		if($value=='0'){
			$i = 1;
			foreach($email as $key => $v){

				if($i <=$pagesize){
					$emailData['email']		=	$v;
					$emailData['subject']	=	$emailtitle;
					$emailData['content']	=	stripslashes($emailcoment);
					$emailData['uid']		=	$key;
					$emailData['name']		=	$userinfo[$key];
					$emailData['cname']		=	"系统";
					if($v){
						$sendid = $notice->sendEmail($emailData);
					}
					if($sendid['status'] != -1){
						$state=1;
						$sendok++;
					}else{
						$state=0;
						$sendno++;
					}
				}
				$i++;
			}
			$result['sendok']	=	$sendok;
			$result['sendno']	=	$sendno;
			$result['page']		=	1;

		}else{
			$page	=	$value;
			$start	=	$page*$pagesize;
			$end	=	($page+1)*$pagesize;

			$i=1;

			foreach($email as $key=>$v){

				if($i > $start && $i <= $end){

					$emailData['email']		=	$v;
					$emailData['subject']	=	$emailtitle;
					$emailData['content']	=	stripslashes($emailcoment);
					$emailData['uid']		=	$key;
					$emailData['name']		=	$userinfo[$key]['name'];
					$emailData['cname']		=	"系统";

					if($v){
						$sendid = $notice->sendEmail($emailData);
					}
					if($sendid['status'] != -1){
						$state=1;
						$sendok++;
					}else{
						$state=0;
						$sendno++;
					}
				}
				$i++;
			}

			$page	=	$page + 1;
			$result['sendok']	=	$sendok;
			$result['sendno']	=	$sendno;
			$result['page']		=	$page;

		}
		return $result;
	}
	//发送短信
	function msgsave_action(){
		$userinfoM	=	$this->MODEL('userinfo');
		if(!checkMsgOpen($this -> config)){
            $this->render_json(1,'还没有配置短信！');
		}
		if(trim($_POST['content'])==''){
            $this->render_json(1,'请输入短信内容！');
        }
		if($_POST['userarr']=='' && $_POST['utype']=='5'){
            $this->render_json(1,'手机号码不能为空！');
        }
		$uidarr=array();
		if($_POST['utype']==5){
			$mobliesarr	=	@explode(',',$_POST['userarr']);
			$userrows	=	$userinfoM->getList(array('moblie'=>array('in',$_POST['userarr'])),array('field'=>'`moblie`,`uid`,`usertype`'));
			$moblies	=	array();
			foreach($userrows as $v){
				$moblies[]=$v['moblie'];
			}
		}else{
			$userrows	=	$userinfoM->getList(array('usertype'=>$_POST['utype']),array('field'=>'`moblie`,`uid`,`usertype`'));
		}

		if(is_array($userrows)&&$userrows){
			$user=$com=$lt=$px=$userinfo=array();
			foreach($userrows as $v){

				if($v['usertype']=='1'){	$user[]	=	$v['uid'];}
				if($v['usertype']=='2'){	$com[]	=	$v['uid'];}
				if($v['usertype']=='3'){	$lt[]	=	$v['uid'];}
				if($v['usertype']=='4'){	$px[]	=	$v['uid'];}

				$uidarr[$v['uid']]	=	$v["moblie"];
			}
			if($user&&is_array($user)){
				$where['uid']	=	array('in',pylode(',',$user));
			}
			if($com&&is_array($com)){
				$where['uid']	=	array('in',pylode(',',$com));
			}
			if($lt&&is_array($lt)){
				$where['uid']	=	array('in',pylode(',',$lt));
			}
			if($px&&is_array($px)){
				$where['uid']	=	array('in',pylode(',',$px));
			}
			$List	=	$userinfoM->getUserList($where);

			foreach($List as $v){
				$userinfo[$v['uid']]=$v['name'];
			}
		}
		if($_POST['utype']==5){
			foreach($mobliesarr as $v){
				if(in_array($v,$moblies)==false&&CheckMobile($v)){
					$uidarr[]=$v;
				}
			}
		}

		if(!count($uidarr)){
            $this->render_json(1,'没有符合条件号码，请先检查！');
		}else{
			set_time_limit(10000);

			$pagesize	=	intval($_POST['pagelimit']);
			$sendok		=	intval($_POST['sendok']);
			$sendno		=	intval($_POST['sendno']?$_POST['sendno']:0);
			$value		=	intval($_POST['value']);

			//分批次发送
			$result		=	$this->sendDivMsg($uidarr,$_POST['content'],$userinfo,$pagesize,$sendok,$sendno,$value);

			if($result){
				$toSize = $pagesize * $result['page'];
				if(count($uidarr) > $toSize){
					$npage	=	$result['page'];
					$spage	=	$npage*$pagesize+1;
					$topage	=	($npage+1)*$pagesize;
					$name	=	$spage."-".$topage;
					$this->get_return("3",$result['page'],"正在发送".$name."条短信",$result['sendok'],$result['sendno']);
				}else{
                    $this->render_json(0,'发送成功');
				}
			}
		}
	}

	function sendDivMsg($uidarr=array(),$content="",$userinfo=array(),$pagesize,$sendok,$sendno,$value){

		$notice = $this->MODEL('notice');
		$sendok = intval($sendok);
		$sendno = intval($sendno);

		if($value=='0'){
			$i = 1;
			foreach($uidarr as $key => $v){

				if($i <= $pagesize){
					$msgData['mobile']	=	$v;
 					$msgData['content']	=	$content;
					$msgData['uid']		=	$key;
					$msgData['name']	=	$userinfo[$key];
					$msgData['cname']	=	"系统";
					$msgData['port']	=	'5';

					if($v){
						$sendid = $notice->sendSMS($msgData);
 					}
					if($sendid['status'] != -1){
						$sendok++;
					}else{
						$sendno++;
					}
				}
				$i++;
			}
			$result['sendok']	=	$sendok;
			$result['sendno']	=	$sendno;
			$result['page']		=	1;

		}else{
			$page	=	$value;
			$start	=	$page*$pagesize;
			$end	=	($page+1)*$pagesize;

			$i=1;

			foreach($uidarr as $key=>$v){

				if($i > $start && $i <= $end){

					$msgData['moblie']	=	$v;
 					$msgData['content'] =	$content;
					$msgData['uid']		=	$key;
 					$msgData['name']	=	$userinfo[$key]['name'];
					$msgData['cname']	=	"系统";
					$msgData['port']	=	'5';

					if($v){
						$sendid = $notice->sendSMS($msgData);
					}
					if($sendid['status'] != -1){
 						$sendok++;
					}else{
 						$sendno++;
					}
				}
				$i++;
			}

			$page	=	$page + 1;
			$result['sendok']	=	$sendok;
			$result['sendno']	=	$sendno;
			$result['page']		=	$page;

		}
		return $result;
	}

	//Other
	function get_return($status,$value,$msg,$sendok = null,$sendno = null){
		$data['status']	=	$status;
		$data['value']	=	$value;
		$data['msg']	=	$msg;
		$data['sendok']	=	$sendok;
		$data['sendno']	=	$sendno;
		echo json_encode($data);die;
	}

    function getIpAddress_action(){
        if($this->config['sy_ip'] == 2){
            $this->render_json(1,'请先开启归属地查询');
        }
        if (!$_POST['uid']) {
            $this->render_json(1,'参数错误');
        }
        if (!$_POST['ip']) {
            $this->render_json(1,'参数错误');
        }

        $ipaddress = getIpAddress($_POST['ip']);
        $userinfoM = $this->MODEL('userinfo');
        $userinfoM->upInfo(array('uid'=>$_POST['uid']),array('login_address'=>$ipaddress));
        $status =1;
        if ($ipaddress){
            $msg = '查询成功';
            $status = 0;
        }else{
            $msg = '查询失败';
        }
        $this->render_json($status,$msg);
    }

    function getMobileAddress_action(){

        if ($this->config['sy_mobile'] == 2) {
            $this->render_json(1, '请先开启归属地查询');
        }
        if (!$_POST['uid']) {
            $this->render_json(1, '参数错误');
        }
        if (!$_POST['moblie']) {
            $this->render_json(1, '电话号码为空');
        }

        $moblieaddress = getMoblieAddress($_POST['moblie']);
        $userinfoM = $this->MODEL('userinfo');
        $userinfoM->upInfo(array('uid' => $_POST['uid']), array('moblie_address' => $moblieaddress));
        $status = 1;

        if ($moblieaddress) {
            $msg = '查询成功';
            $status = 0;
        } else {
            $msg = '查询失败';
        }

        $this->render_json($status, $msg);

    }
    function Imitate_action()
    {

        $userinfoM = $this->MODEL('userinfo');

        $member = $userinfoM->getInfo(array('uid' => intval($_POST['uid'])), array('field' => '`uid`,`username`,`salt`,`email`,`password`,`usertype`,`did`'));

        $this->cookie->unset_cookie($_SESSION['auid']);

        $this->cookie->add_cookie($member['uid'], $member['username'], $member['salt'], $member['email'], $member['password'], intval($_POST['utype']), $this->config['sy_logintime'], $member['did'], '1');

        $logM = $this->MODEL('log');
        if ($_POST['utype'] == 1){
            $msg = '个人';
        }else if($_POST['utype'] == 2){
            $msg = '企业';
        }
        $content = '管理员' . $_SESSION['ausername'] . '登录个人'.$msg.'(ID:' . $member['uid'] . ')';

        $adminLo = $logM->addAdminLog($content);

        $this->render_json(0, 'ok', array('url' => $this->config['sy_weburl'] . '/member'));
    }
}
?>