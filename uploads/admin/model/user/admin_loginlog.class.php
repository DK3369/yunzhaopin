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

class admin_loginlog_controller extends adminCommon
{
	function index_action(){
		$logM = $this->MODEL('log');
		$memberM = $this->MODEL('userinfo');

		if ($_POST['utype']) {
			$where['usertype'] = trim($_POST['utype']);
		} else {
			$where['usertype'] = 1;
		}
		if (intval($_POST['uid'])) {
			$where['uid'] = intval($_POST['uid']);
		}
		if (trim($_POST['keyword'])) {
			if ($_POST['type'] == 1) {
				$member = $memberM->getList(array('username' => array('like', trim($_POST['keyword']))), array('field' => '`uid`,`username`'));
				foreach ($member as $v) {
					$uid[] = $v['uid'];
				}
				$where['uid'] = array('in', pylode(",", $uid));
			} elseif ($_POST['type'] == 2) {
				$where['content'] = array('like', trim($_POST['keyword']));
			} elseif ($_POST['type'] == 3) {
				$where['uid'] = trim($_POST['keyword']);
			}
		}
		if ($_POST['times']) {
			$time = $_POST['times'];
			$time_begin = $time[0]? date('Y-m-d',strtotime(str_replace(array('T','Z'),' ',$time[0]))): date('Y-m-d', strtotime('-30 days'));
			$time_end  = $time[1] ? date('Y-m-d',strtotime(str_replace(array('T','Z'),' ',$time[1]))):date('Y-m-d');
			$where['ctime'][] = array('>=', strtotime($time_begin . "00:00:00"));
			$where['ctime'][] = array('<=', strtotime($time_end . "23:59:59"));
		}
		//内容搜索
		if (trim($_POST['content'])) {
			$where['content'] = array('like', trim($_POST['content']));
		}
		$page  = $_POST['page'];
		$pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
		$pageM	=	$this  -> MODEL('page');
		$pages	=	$pageM -> adminPageList('login_log',$where,$page,array('limit' => $pageSize));
		if (!$pages['total']) {
			$this->render_json(0,'暂无数据',['data'=>[],'total'=>0,'pageSizes'=>$pages['page_sizes']]);
		}
		if ($_POST['order']) {
			$where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
		} else {
			$where['orderby'] = array('id,desc');
		}
		$where['limit'] = $pages['limit'];
		$List = $logM->getLoginlogList($where, array('utype' => 'admin'));
		foreach ($List  as  &$value){
			$value['ctime_ymd'] = $value['ctime']?date('Y-m-d H:i:s',$value['ctime']):'';
		}
		$this->render_json(0,'',['data'=>$List,'total'=>(int)$pages['total'],'pageSizes'=>$pages['page_sizes']]);
	}

	/**
	 * @desc 会员-登录日志-列表:（统计数量）
	 */
	function loginlogNum_action()
	{
		$logM = $this->MODEL('log');
		$num = $logM->getLoginlogNum(array('usertype' => $_POST['usertype']));
		echo $num;
	}

	function dellog_action()
	{
		$logM = $this->MODEL('log');

		if ($_POST['del'] == 'allcom') {
			$where['usertype'] = '2';

			$logM->delLoginlog('', $where);
			$this->render_json('0','已清空企业日志！');
		} elseif ($_POST['del'] == 'alluser') {
			$where['usertype'] = '1';

			$logM->delLoginlog('', $where);
			$this->render_json(0,'已清空个人日志！');
		} elseif ($_POST['del'] == 'alltrain') {
			$where['usertype'] = '4';

			$logM->delLoginlog('', $where);
			$this->render_json(0,'已清空培训日志！');

		} elseif ($_POST['del']) {
			$del = $_POST['del'];
			if (!$del) {
				$this->render_json(0,'请选择您要删除的信息！');
			}
			if (is_array($del)) {
				$where['id'] = array('in', pylode(',', $del));
				$logM->delLoginlog('', $where);
			} else {
				$logM->delLoginlog('', array('id' => $del));
			}
			$this->render_json(0,'登录日志(ID:' . pylode(',', $del) . ')删除成功！');
		}
	}
}
?>

