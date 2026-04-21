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
class mysql {
	private $db_host; //数据库主机
	private $db_user; //数据库用户名
	private $db_pwd; //数据库用户名密码
	private $db_database; //数据库名
	private $conn; //数据库连接标识;
	private $result; //执行query命令的结果资源标识
	private $sql; //sql执行语句
	private $row; //返回的条目数
	private $coding; //数据库编码，GBK,UTF8,gb2312
	private $bulletin = true; //是否开启错误记录
	private $show_error = true; //测试阶段，显示所有错误,具有安全隐患,默认关闭
	private $is_error = false; //发现错误是否立即终止,默认true,建议不启用，因为当有问题时用户什么也看不到是很苦恼的
	private $def="";
    public $SQLList = array();
	/*构造函数*/
	public function __construct($db_host, $db_user, $db_pwd, $db_database, $conn, $coding,$def="") {
		$this->db_host = $db_host;
		$this->db_user = $db_user;
		$this->db_pwd = $db_pwd;
		$this->db_database = $db_database;
		$this->coding = $coding;
		$this->def=$def;
        $this->SQLList = array();
	}
	/*数据库连接*/
	public function connect() {
		
		if(!$this->conn){
			$this->conn = @mysql_connect($this->db_host, $this->db_user, $this->db_pwd) or die('数据库连接出错!');
			if (!@mysql_select_db($this->db_database, $this->conn)) {
				if ($this->show_error) {
					$this->show_error("数据库不可用：", $this->db_database);
				}
			}
			@mysql_query("SET NAMES $this->coding");
			@mysql_query("SET character_set_connection=utf8,character_set_results=utf8,character_set_client=binary", $this->conn);
		}
		
		
	}
	/*数据库执行语句，可执行查询添加修改删除等任何sql语句*/
	public function query($sql) {


		$this->connect();

		if ($sql == "") {
			$this->show_error("SQL语句错误：", "SQL查询语句为空");
		}
		$this->sql = $sql;
		
		
		$result = mysql_query($this->sql,$this->conn);
		$this->result = $result;
		return $this->result;
	}
    
    /**
     * 通用SQL 执行 便于多表查询语句
     */

	function DB_query_all($sql,$type='one'){
		$query = $this->query($sql);
		if($type=='all'){
			 while($row=$this->fetch_array($query)){$return[]=$row;}
		}else{
			$return=$this->fetch_array($query);
		}
		return $return;
	}
    /**
     * 通用单条查询$tablename,$where = 1, $select="*"
     */
	function DB_select_once($tablename, $where = 1, $select = "*") {
		$cachename=$tablename.$where;
		if(!$return=$this->Memcache_set($cachename)){//获取是否存在memcache
			$SQL = "SELECT $select FROM " . $this->def . $tablename . " WHERE $where limit 1";
			$query = $this->query($SQL);
			$return=$this->fetch_array($query);
			$this->Memcache_set($cachename,$return);//设置memcache
		}
		return $return;
	}
	function update_all($tablename, $value, $where = 1) {
		$cachename=$tablename.$where;
		if(!$return=$this->Memcache_set($cachename)){//获取是否存在memcache
			$SQL = "UPDATE `" . $this->def . $tablename . "` SET $value WHERE $where";
			$query = $this->query($SQL);
			$return=$this->fetch_array($query);
			$this->Memcache_set($cachename,$return);//设置memcache
		}
		return $return;
	}
	function insert_once($tablename, $value) {
		
		$cachename=$tablename.$value;
		if(!$return=$this->Memcache_set($cachename)){//获取是否存在memcache
			$SQL = "INSERT INTO `" . $this->def . $tablename . "` SET ".$value;
			$this->query("set sql_mode=''");
			$query = $this->query($SQL);
			$nid= $this->insert_id();
		
		}
		return $nid;
	}
	function Memcache_set($name,$value=""){

		$this->connect();

		/**
		global $config;
		if($config[ismemcache]==2)return "";

		$memcachehost=$config[memcachehost];//所在服务器
		$memcacheport=$config[memcacheport];//所在服务器端口
		$memcachezip=0;//是否支持解压缩
		$memcachetime=$config[memcachetime];//缓存时间
		$name=str_replace(array(" ","`","'",".","="),"",$name);

		$memcache = new memcache;
		$memcache->connect($memcachehost,$memcacheport) or die ("Memcache连接失败或您的服务器不支持Memcache,请在后台关闭！");
		$val=$memcache->get($name);
		if(!is_array($val) && !$val){
        $memcache->set($name,$value,$memcachezip,$memcachetime);
		}
		//$memcache->delete($name);//删除当前名称的cache
		//print_r($val);

		$memcache->close();
		return $val;
         **/
	}
    /**
     * 通用数量查询$tablename,$where = 1, $select="*"
     */
	function select_num($tablename, $where = 1, $select = "*") {
		$cachename=$tablename.$where;
		if(!$return=$this->Memcache_set($cachename)){//获取是否存在memcache
			$SQL = "SELECT count($select) FROM " . $this->def . $tablename . " WHERE $where";
			$query = $this->query($SQL);
			while($rs = mysql_fetch_array($query)){
				$row = $rs;
			}
			$return=$row[0];
			$this->Memcache_set($cachename,$return);//设置memcache
		}
		return $return;
	}
	/**
     * 通用all查询 $tablename,$where = 1, $select="*"
     */
	function select_all($tablename, $where = 1, $select = "*") {
		$row_return=array();
		$SQL = "SELECT $select FROM `" . $this->def . $tablename . "` WHERE $where";
		$query=$this->query($SQL);
        while($row=$this->fetch_array($query)){$row_return[]=$row;}
        return $row_return;
	}
    /**
     * 通用$tablename为(select * from `test` order by `date` desc) `temp`
     *  select * from (select * from `test` order by `date` desc) `temp`  group by category_id order by `date` desc
     * 查询 $tablename, $where = 1, $select = "*"
     */
	function select_only($tablename, $where = 1, $select = "*") {
        $row_return=array();
        $SQL = "SELECT $select FROM " .$tablename . " WHERE $where";
        $query=$this->query($SQL);
        while($row=$this->fetch_array_old($query)){$row_return[]=$row;}
        return $row_return;
	}
    /**
     * 通用all查询双表 $tablename1,$tablename2, $where = 1, $select = "*"
     */
	function select_alls($tablename1,$tablename2, $where = 1, $select = "*") {
        $SQL = "SELECT $select FROM " . $this->def . $tablename1. " as a," . $this->def . $tablename2 . " as b WHERE $where";
        $query=$this->query($SQL);
        while($row=$this->fetch_array($query)){$row_return[]=$row;}
        return $row_return;
	}
	/*创建添加新的数据库*/
	public function create_database($database_name) {
		$database = $database_name;
		$sqlDatabase = 'create database ' . $database;
		$this->query($sqlDatabase);
	}
	function cacheget(){
		include PLUS_PATH."/city.cache.php";
		include PLUS_PATH."/com.cache.php";
		include PLUS_PATH."/job.cache.php";
		include PLUS_PATH."/user.cache.php";
		include PLUS_PATH."/industry.cache.php";

		$array["comclass_name"] = $comclass_name;
		$array["comdata"] = $comdata;
		$array["city_name"] = $city_name;
		$array["city_index"] = $city_index;
		$array["user_classname"] = $userclass_name;
		$array["userdata"] = $userdata;
		$array["job_name"] = $job_name;
		$array["job_index"] = $job_index;
		$array["job_type"] = $job_type;
		$array["industry_name"] = $industry_name;
		return $array;
	}
	function fscacheget(){
	    include PLUS_PATH."/cityparent.cache.php";
	    include PLUS_PATH."/jobparent.cache.php";
	    $array["job_parent"] = $job_parent;
	    $array["city_parent"] = $city_parent;
	    return $array;
	}
	//将信息替换为缓存数组中内容
	function array_action($job_info,$array=array()){
		if(!empty($array)){
			$comclass_name = $array["comclass_name"];
			$city_name = $array["city_name"];
			$industry_name = $array["industry_name"];
			$job_name = $array["job_name"];

		}else{
			include PLUS_PATH."/city.cache.php";
			include PLUS_PATH."/com.cache.php";
			include PLUS_PATH."/job.cache.php";
			include PLUS_PATH."/industry.cache.php";

		}
		$job_info[job_class_one] = $job_name[$job_info["job1"]];
		$job_info[job_class_two] = $job_name[$job_info[job1_son]];
		$job_info[job_class_three] = $job_name[$job_info[job_post]];
		$job_info[job_exp] = $comclass_name[$job_info["exp"]];
		$job_info[job_edu] = $comclass_name[$job_info[edu]];
		$job_info[job_salary] = $comclass_name[$job_info[salary]];
		$job_info[job_mun] = $comclass_name[$job_info[mun]];
		$job_info[job_type] = $comclass_name[$job_info[type]];
		$job_info[job_marriage] = $comclass_name[$job_info[marriage]];
		$job_info[job_report] = $comclass_name[$job_info[report]];
		$job_info[job_city_one] = $city_name[$job_info[provinceid]];
		$job_info[com_city] = $city_name[$job_info[com_provinceid]];
		$job_info[job_pr] = $comclass_name[$job_info[pr]];
		$job_info[job_city_two] = $city_name[$job_info[cityid]];
		$job_info[job_city_three] = $city_name[$job_info[three_cityid]];
		$job_info[job_hy] = $industry_name[$job_info[hy]];
        
        
		if($job_info[lang]!=""){
			$lang = @explode(",",$job_info[lang]);
			foreach($lang as $key=>$value){
				$langinfo[]=$comclass_name[$value];
			}
			$job_info[lang_info] = @implode(",",$langinfo);
			$job_info[lang] =$lang;
		}else{
			$job_info[lang_info] ="";
		}
		if($job_info[welfare]!=""){
			$welfare = @explode(",",$job_info[welfare]);
			foreach($welfare as $key=>$value){
				$welfareinfo[]=$value;
			}
			$job_info[welfare_info] = @implode(",",$welfareinfo);
			$job_info[welfare] =$welfare;
		}else{
			$job_info[welfare_info] ="";
		}
		return $job_info;
	}
	//将信息替换为缓存数组中内容(兼职)
	function part_array_action($job_info,$array=array()){
		if(!empty($array)){
			$partclass_name = $array["partclass_name"];
			$city_name = $array["city_name"];
		}else{
			include PLUS_PATH."/city.cache.php";
			include PLUS_PATH."/part.cache.php";
		}
		$job_info['job_salary_type'] = $partclass_name[$job_info['salary_type']];
		$job_info['job_type'] = $partclass_name[$job_info['type']];
		$job_info['job_billing_cycle'] = $partclass_name[$job_info['billing_cycle']];
		$job_info['job_city_one'] = $city_name[$job_info['provinceid']];
		$job_info['job_city_two'] = $city_name[$job_info['cityid']];
		$job_info['job_city_three'] = $city_name[$job_info['three_cityid']];
		return $job_info;
	}
	/*查询服务器所有数据库*/
	//将系统数据库与用户数据库分开，更直观的显示？
	public function show_databases() {
		$this->query("show databases");
		echo "现有数据库：" . $amount = $this->db_num_rows($rs);
		echo "<br />";
		$i = 1;
		while ($row = $this->fetch_array($rs)) {
			echo "$i $row[Database]";
			echo "<br />";
			$i++;
		}
	}
	//以数组形式返回主机中所有数据库名
	public function databases() {
		$this->connect();
		$rsPtr = mysql_list_dbs($this->conn);
		$i = 0;
		$cnt = mysql_num_rows($rsPtr);
		while ($i < $cnt) {
			$rs[] = mysql_db_name($rsPtr, $i);
			$i++;
		}
		return $rs;
	}
	/*查询数据库下所有的表*/
	public function show_tables($database_name) {
		$this->connect();
		$this->query("show tables");
		echo "现有数据库：" . $amount = $this->db_num_rows($rs);
		echo "<br />";
		$i = 1;
		while ($row = $this->fetch_array($rs)) {
			$columnName = "Tables_in_" . $database_name;
			echo "$i $row[$columnName]";
			echo "<br />";
			$i++;
		}
	}
	
	/*取得结果数据*/
	public function mysql_result_li() {
		$this->connect();
		return mysql_result($str);
	}
	/*取得记录集,获取数组-索引和关联,使用$row['content'] */
	public function fetch_array($query="") {
		$this->connect();
		if(!$query){
			return @mysql_fetch_array($this->result, MYSQL_ASSOC);
		}else{
			return @mysql_fetch_array($query, MYSQL_ASSOC);
		}
	}
	public function fetch_array_old($query="") {
		$this->connect();
		if(!$query){
			return @mysql_fetch_array($this->result);
		}else{
			return @mysql_fetch_array($query);
		}
	}
	//获取关联数组,使用$row['字段名']
	public function fetch_assoc() {
		$this->connect();
		return mysql_fetch_assoc($this->result);
	}
	//获取数字索引数组,使用$row[0],$row[1],$row[2]
	public function fetch_row() {
		$this->connect();
		return mysql_fetch_row($this->result);
	}
	//获取对象数组,使用$row->content
	public function fetch_Object() {
		$this->connect();
		return mysql_fetch_object($this->result);
	}
	/*取得上一步 INSERT 操作产生的 ID*/
	public function insert_id() {
		$this->connect();
		return mysql_insert_id();
	}
	//指向确定的一条数据记录
	public function db_data_seek($id) {
		$this->connect();
		if ($id > 0) {
			$id = $id -1;
		}
		if (!@ mysql_data_seek($this->result, $id)) {
			$this->show_error("SQL语句有误：", "指定的数据为空");
		}
		return $this->result;
	}
	// 根据select查询结果计算结果集条数
	public function db_num_rows() {
		$this->connect();
		if ($this->result == null) {
			if ($this->show_error) {
				$this->show_error("SQL语句错误", "暂时为空，没有任何内容！");
			}
		} else {
			return mysql_num_rows($this->result);
		}
	}
	// 根据insert,update,delete执行结果取得影响行数
	public function db_affected_rows() {
		$this->connect();
		return mysql_affected_rows();
	}
	public function show_error($message = "", $sql = "") {
        exit('FAILED: Sql Error!');
	}
	//释放结果集
	public function free() {
		@ mysql_free_result($this->result);
	}
	//数据库选择
	public function select_db($db_database) {
		$this->connect();
		return mysql_select_db($db_database);
	}
	//查询字段数量
	public function num_fields($table_name) {
		
		$this->connect();
		return mysql_num_fields($this->result);
		
		
	}
	//取得 MySQL 服务器信息
	public function mysql_server($num = '') {
		$this->connect();
		switch ($num) {
			case 1 :
				return mysql_get_server_info(); //MySQL 服务器信息
				break;
			case 2 :
				return mysql_get_host_info(); //取得 MySQL 主机信息
				break;
			case 3 :
				return mysql_get_client_info(); //取得 MySQL 客户端信息
				break;
			case 4 :
				return mysql_get_proto_info(); //取得 MySQL 协议信息
				break;
			default :
				return mysql_get_client_info(); //默认取得mysql版本信息
		}
	}
	
	//转义字符串
	function escape_string($string){
		
		$this->connect();
		
		return mysql_real_escape_string($string);
	}
	//关闭数据库,垃圾回收机制
	public function close() {

		if($this->conn){

			@mysql_close($this->conn);
		}
		
	}
	//析构函数，自动关闭数据库,垃圾回收机制
	public function __destruct() {

		if($this->conn){
			if (!empty ($this->result)) {
				$this->free();
			}

			@mysql_close($this->conn);
			
		}
		
	} //function __destruct();
	function getmicrotime(){
		list($usec, $sec) = @explode(" ", microtime());
		return ((float)$usec + (float)$sec);
	}
}
?>