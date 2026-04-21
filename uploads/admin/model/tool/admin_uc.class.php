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
//模板操作类
class admin_uc_controller extends adminCommon{

	function index_action()
	{
		
		require_once APP_PATH."data/api/uc/config.inc.php";
		$data['ucinfo'] = $ucinfo;

		require_once APP_PATH."data/api/pw_api/pw_config.php";
		$data['pw_ucinfo'] = $ucinfo;

		$data['config'] = array(
			'sy_uc_type'=>$this->config['sy_uc_type'],
			'sy_pw_type'=>$this->config['sy_pw_type'],
			
		);
		$this->render_json(0,'ok', $data);
	}
	function ucsave_action(){
		$config = "<?php \r\n";
		if($_POST){
			$parr					=	"";
			$sy_uc_type				=	$_POST['sy_uc_type'];//设置状态
			
			unset($_POST['sy_uc_type']);
			$_POST['UC_CONNECT']	=	"mysql";
			$_POST['UC_DBCONNECT']	=	"0";
			$_POST['UC_DBCHARSET']	=	'utf8';
			$_POST['UC_IP']			=	"";
			$_POST['UC_DBTABLEPRE']	=	"`".$_POST['UC_DBNAME']."`.".$_POST['UC_DBTABLEPRE_NEW'];
			
			foreach($_POST as $key=>$value)
			{
				$config.= "define(\"".$key."\",\"".$value."\"); \r\n";
				$parr  .= "\"".$key."\"=>\"".$value."\",";
			}
			$parr		= rtrim($parr,",");
			$config	   .= "\$ucinfo=array(".$parr."); \r\n";
		}
		$path	=	APP_PATH."data/api/uc/config.inc.php";
		$fp		=	@fopen($path,"w");
		fwrite($fp,$config);
		fclose($fp);
		
		$this -> MODEL('config') -> setConfig(array('sy_uc_type'=>$sy_uc_type));
		
		$this->web_config();		
		$this->render_json(0,'保存成功');
	}


	function pwsave_action() { 
		
		$configM	=	$this -> MODEL('config');
		
		$userinfoM	=	$this -> MODEL('userinfo');
		$config = "<?php \r\n";
		if($_POST){
			$parr					=	"";
			$sy_pw_type				=	$_POST['sy_pw_type'];//设置状态
			
			unset($_POST['sy_pw_type']);
			$_POST['UC_DBCONNECT']	=	"0";
			$_POST['UC_IP']			=	"";
			$_POST['UC_DBCHARSET']	=	$_POST['UC_CHARSET'];
			foreach($_POST as $key=>$value){
				$config.= "define(\"".$key."\",\"".$value."\"); \r\n";
				$parr  .= "\"".$key."\"=>\"".$value."\",";
			}
			$parr       = rtrim($parr,",");
			$config    .= "\$ucinfo=array(".$parr."); \r\n";
		}
		$path	=	APP_PATH."data/api/pw_api/pw_config.php";
		$fp 	=	@fopen($path,"w");
		fwrite($fp,$config);
		fclose($fp);
		
		$configM -> setConfig(array('sy_pw_type'=>$sy_pw_type));
		
		$configM -> upInfo(array('name'=>'sy_uc_type'),array('config'=>''));
		$this->web_config();

		if($sy_pw_type=='pw_center'){
			$user_arr = $userinfoM -> getList("1",array('field'=>'`uid`,`username`,`password`,`salt`'));

			require_once APP_PATH.'/api/pw_api/pw_client/class_db.php';

			$db_uc = new UcDB;
			
			include(APP_PATH."data/api/pw_api/pw_config.php");
			$db_uc->connect(UC_DBHOST, UC_DBUSER, UC_DBPW, UC_DBNAME, UC_DBCONNECT, UC_DBCHARSET);
			$pw_query=$db_uc->query("SELECT `uid`,`username`,`password` FROM ".UC_DBTABLEPRE."members");
			
			while($pw_rs = $db_uc->fetch_array($pw_query)){
				if(is_array($user_arr)){
					foreach($user_arr as $key=>$value){
						//判断是否有重名用户
						if($value['username']==$pw_rs['username']&&$value['pw_repeat']!="1"){
							if($value['pwuid']<1){
								//判断密码是否一致
								if($value['password']==md5($pw_rs['password'].$value['salt'])){
									//用户名密码一致，绑定用户PWid
									$userinfoM -> upInfo(array('pwuid'=>$pw_rs['uid']),array('uid'=>$value['uid']));
								}else{
									//用户名密码不一致设定为重名用户，不予同步
									$userinfoM -> upInfo(array('pw_repeat'=>'1'),array('uid'=>$value['uid']));
								}
							}
						}
					}
				}
			} 
		}
		$this->render_json(0,'保存成功');
	}
}
?>