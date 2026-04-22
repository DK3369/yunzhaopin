<?php

/**
 * Class database_controller
 *
 * @author shiy
 * @version 7.0
 */

class database_controller extends adminCommon
{

    private function get_table()
    {

        include(LIB_PATH."dbbak.class.php");
        return new DBManagement("phpyun", CONFIG_PATH . "backup/", $this->obj, $this->db);
    }

    public function getDbTable_action()
    {

        $dbTable=   $this->get_table();
        $table  =   $dbTable->GetTablesName();

        $tableList  =   array_chunk($table, 4, false);

        $data   =   array(

            'dbLength'  =>  count($table),
            'dbTable'   =>  $tableList
        );

        $this->render_json(0, '', $data);
    }

    function backUp_action()
    {

        include(LIB_PATH.'/dbbackup/class/functions.php');
        global $db_config;

        $maxFileSize	=	$_POST['maxfilesize'];
        $maxFileSize	=	is_numeric($maxFileSize) ? $maxFileSize : 300;
        $basetype = $_POST['backType'];
        $tables = array();
        if($basetype == 1){
            $dbTable=   $this->get_table();
            $tablesName  =   $dbTable->GetTablesName();
            foreach ($tablesName as $table){
                $tables[] =$table['name'];
            }
            $_POST['table'] = $tables;
        }else{
            $tables = $_POST['table'];
        }
        if (!$tables){
            $this->admin_json(1, '请选择要备份的表！');
        }

        $DBParameter	=	array(

            'phome'			=>	'',
            'mydbname'		=>	$db_config['dbname'],
            'servername'	=>	$db_config['dbhost'],
            'oldtablepre'	=>	$db_config['def'],
            'newtablepre'	=>	$db_config['def'],
            'bktype'		=>	0,
            'bakline'		=>	500,
            'autoauf'		=>	'',
            'bakstru'		=>	'',
            'bakdatetype'	=>	'',
            'mypath'		=>	'',
            'waitbaktime'	=>	'',
            'tablename'		=>	$tables,
            'dbchar'		=>	$db_config['charset'],
            'backuppath'	=>	PLUS_PATH.'/bdata/',
            'filesize'		=>	$maxFileSize
        );

        BackupDatabaseInit($DBParameter);die;

        global $db_config;

        extract($_POST);

        $dbTable=   $this->get_table();
        $fw		=   $dbTable->backup_action($table, 10000000000, $db_config);
        if ($fw){

            $this->admin_json(0, '备份成功！');
        }else{

            $this->admin_json(1, '备份失败！');
        }
    }

    function BackupDatabaseFileSize_action()
    {

        include(LIB_PATH.'/dbbackup/class/functions.php');

        $t  =   $_POST['t'];
        $s  =   $_POST['s'];
        $p  =   $_POST['p'];

        $mypath     =   $_POST['mypath'];
        $alltotal   =   $_POST['alltotal'];
        $thenof     =   $_POST['thenof'];
        $fnum       =   $_POST['fnum'];
        $stime      =   $_POST['stime'];

        BackupDatabaseFileSize($t, $s, $p, $mypath, $alltotal, $thenof, $fnum, $stime);
    }

    function BackupDatabaseRecordNum_action()
    {
        include(LIB_PATH.'/dbbackup/class/functions.php');

        $t  =   $_POST['t'];
        $s  =   $_POST['s'];
        $p  =   $_POST['p'];

        $mypath     =   $_POST['mypath'];
        $alltotal   =   $_POST['alltotal'];
        $thenof     =   $_POST['thenof'];
        $fnum       =   $_POST['fnum'];
        $auf        =   $_POST['auf'];
        $aufval     =   $_POST['aufval'];
        $stime      =   $_POST['stime'];
        BackupDatabaseRecordNum($t, $s, $p, $mypath, $alltotal, $thenof, $fnum, $auf, $aufval, $stime);
    }

    private function BackupList()
    {
        global $db_config;
        $filedb =   array();
        $handle =   opendir(PLUS_PATH.'/bdata/');
        while ($file = readdir($handle)) {
            if (preg_match("/^".$db_config['dbname']."/", $file) && is_dir(PLUS_PATH.'/bdata/'.$file)) {
                include(PLUS_PATH.'/bdata/'.$file.'/config.php');
                $time       =   date("Y-m-d H:i:s", filemtime(PLUS_PATH.'/bdata/'.$file));
                $filedb[]   =   array(
                    'name'      =>  $file,
                    'version'   =>  $b_version,
                    'time'      =>  $time,
                    'dbname'    =>  $b_dbname,
                    'charset'   =>  $b_dbchar
                );
            }
        }
        return $filedb;
    }

    function getBackFile_action()
    {

        $sqlArr		=	$this->BackupList();
        $this->render_json(0, '', $sqlArr);
    }

    function backIn_action()
    {

        include(LIB_PATH.'/dbbackup/class/functions.php');
        global $db_config;
        $add	=	array('mydbname'=>$db_config['dbname'],'waitbaktime'=>0);
        $mypath	=	$_GET['mypath'];
        RecoverData($add,$mypath);die;

        global $db_config;
        extract($_GET);
        $dbbak	=	$this->get_table();
        $dbbak	=	$dbbak->bakindata($sql);
        $dbbak?$this->layer_msg("数据库恢复成功！",9,0,$_SERVER['HTTP_REFERER']):$this->ACT_msg("恢复成功！",8,0,$_SERVER['HTTP_REFERER']);


    }

    function delBack_action()
    {

        if (isset($_POST['sql'])){
            if (is_array($_POST['sql'])){
                $success = $fail =  0;
                foreach ($_POST['sql'] as $k => $v) {
                    if (preg_match('/^[_0-9a-z]+$/i', $v)){

                        $handle =   opendir(PLUS_PATH.'/bdata/'.$v);
                        while ($file = readdir($handle)) {

                            $filedb[]   =   $file;
                            @unlink(PLUS_PATH.'/bdata/'.$v.'/'.$file);
                        }

                        $res    =   rmdir(PLUS_PATH.'/bdata/'.$v);

                        $res ? $success++ : $fail++;
                    }else{

                        $fail++;
                    }
                }
                $this->admin_json(0,'数据备份删除操作完成：成功'.$success.'，失败'.$fail.'！');
            }else{
                if (preg_match('/^[_0-9a-z]+$/i', $_POST['sql'])){

                    $handle =   opendir(PLUS_PATH.'/bdata/'.$_POST['sql']);
                    while ($file = readdir($handle)) {

                        $filedb[]   =   $file;
                        @unlink(PLUS_PATH.'/bdata/'.$_POST['sql'].'/'.$file);
                    }

                    $res    =   rmdir(PLUS_PATH.'/bdata/'.$_POST['sql']);
                    if ($res){

                        $this->admin_json(0,'数据备份删除成功！');
                    }else{

                        $this->admin_json(1,'数据备份删除失败！');
                    }
                }else{

                    $this->render_json(0, '非法操作！');
                }
            }
        }else{

            $this->render_json(0, '非法操作！');
        }
    }

    function getOptTable_action()
    {

        $dbTable=	$this->get_table();
        $table	=	$dbTable->GetTablesName();
        $num	=	0;
        $list	=	array();
        foreach ($table as $v){
            $ret = $this->db->query('SHOW TABLE STATUS LIKE \'' . $v['name'] . '%\'');
            while ($row = $this->obj->fetch_assoc()) {
                $num   += $row['Data_free'];
                $list[] = array(
                    'name'		=>	$row['Name'],
                    'type'		=>	$row['Engine'],
                    'num'		=>	$row['Rows'],
                    'size'		=>	sprintf(' %.2f KB', $row['Data_length'] / 1024),
                    'rec_index'	=>	$row['Index_length'],
                    'chip'		=>	$row['Data_free'],
                    'status'	=>	'',
                    'charset'	=>	$row['Collation']
                );
            }
        }

        $this->render_json(0,'', $list);
    }

    function optimizeTable_action()
    {

        $result     =   0;
        $errorMsg   =   '';
        if ($_POST['type'] == 2) {

            $result =   $this->db->query("REPAIR TABLE `".$_POST['name']."`");
            $errorMsg   =   "修复数据表：".$_POST['name'];
        } elseif ($_POST['type'] == 3) {

            $result =   $this->db->query("OPTIMIZE TABLE `".$_POST['name']."`");
            $errorMsg   =   "优化数据库：".$_POST['name'];
        }

        $error      =   $result ? 0 : 1;
        $errorMsg   =   $result ? $errorMsg.' 成功' : $errorMsg.' 失败';
        $this->admin_json($error, $errorMsg);
    }

    function clearData_action()
    {

        $clearTable =   $_POST['clearTable'];
        $clearTime  =   intval($_POST['clearTime']);

        $logM       =   $this->MODEL('log');
        $jobM       =   $this->MODEL('job');

        $clearTime  =   time() - $clearTime * 86400;
        $limit      =   1000;

        $data['norecycle']  =   1;

        if ($clearTable == "userid_job") {//职位申请记录

            $where['datetime']  =   array('<', $clearTime);
            $data['where']      =   $where;
            $num    =   $jobM->getSqJobNum($where);
            if ($num) {

                $data['limit']  =   'limit '.($num > $limit ? $limit : $num);
                $return =   $jobM->delSqJob('', $data);
                $nid    =   $return['id'];
            }
        }
        if ($clearTable == "userid_msg") {//面试邀请记录

            $where['datetime']  =   array('<', $clearTime);
            $data['where']      =   $where;

            $num                =   $jobM->getYqmsNum($where);
            if ($num) {

                $data['limit']  =   'limit '.($num > $limit ? $limit : $num);
                $return =   $jobM->delYqms("", $data);
                $nid    =   $return['id'];
            }
        }
        if ($clearTable == "down_resume") {//简历下载记录

            $where['downtime']  =   array('<', $clearTime);
            $data['where']      =   $where;

            $downResumeM        =   $this->MODEL('downresume');
            $num                =   $downResumeM->getDownNum($where);
            if ($num) {

                $data['limit']  =   'limit '.($num > $limit ? $limit : $num);
                $nid            =   $downResumeM->delInfo("", $data);
            }
        }
        if ($clearTable == "talent_pool") {//简历收藏记录

            $where['ctime']     =   array('<', $clearTime);
            $data['where']      =   $where;

            $resumeM            =   $this->MODEL('resume');
            $num                =   $resumeM->getTalentNum($where);
            if ($num) {

                $data['limit']  =   'limit '.($num > $limit ? $limit : $num);
                $nid            =   $resumeM->delTalentPool("", $data);
            }
        }
        if ($clearTable == "look_resume") {//浏览简历记录

            $where['datetime']  =   array('<', $clearTime);
            $data['where']      =   $where;

            $lookResumeM        =   $this->MODEL('lookresume');
            $num                =   $lookResumeM->getLookNum($where);
            if ($num) {

                $data['limit']  =   'limit '.($num > $limit ? $limit : $num);
                $nid            =   $lookResumeM->delInfo($data);
            }
        }
        if ($clearTable == "look_job") {//浏览职位记录

            $where['datetime']  =   array('<', $clearTime);
            $data['where']      =   $where;

            $num                =   $jobM->getLookJobNum($where);
            if ($num) {

                $data['limit']  =   'limit '.($num > $limit ? $limit : $num);
                $return         =   $jobM->delLookJob("", $data);
                $nid            =   $return['id'];
            }
        }
        if ($clearTable == "email_msg") {//邮件发送记录

            $where['ctime']     =   array('<', $clearTime);

            $emailM             =   $this->MODEL('email');
            $num                =   $emailM->getEmsgNum($where);
            if ($num) {

                $data['limit']  =   'limit '.($num > $limit ? $limit : $num);
                $nid            =   $emailM->delEmailMsg($where, $data);
            }
        }
        if ($clearTable == "moblie_msg") {//短信发送记录

            $where['ctime']     =   array('<', $clearTime);

            $mobileMsgM         =   $this->MODEL('mobliemsg');
            $num                =   $mobileMsgM->getNum($where);
            if ($num) {

                $data['limit']  =   'limit '.($num > $limit ? $limit : $num);
                $nid            =   $mobileMsgM->delMoblieMsg($where, $data);
            }
        }
        if ($clearTable == "member_log") {//会员日志

            $where['ctime']     =   array('<', $clearTime);

            $num                =   $logM->getMemberLogNum($where);
            if ($num) {

                $data['limit']  =   'limit '.($num > $limit ? $limit : $num);
                $return         =   $logM->delMemlog($where, $data);
                $nid            =   $return['id'];
            }
        }

        if ($clearTable == "recycle") {//回收站

            $where['ctime']     =   array('<', $clearTime);

            $recycleM           =   $this->MODEL('recycle');
            $num                =   $recycleM->getNum($where);
            if ($num) {

                $data['limit']  =   'limit '.($num > $limit ? $limit : $num);
                $return         =   $recycleM->delRecycle($where, $data);
                $nid            =   $return['id'];
            }
        }
        
        if ($clearTable == 'sysmsg') {//  系统消息

            $where['ctime']     =   array('<', $clearTime);
            $data['where']      =   $where;

            $sysMsgM            =   $this->MODEL('sysmsg');
            $num                =   $sysMsgM->getSysmsgNum($where);
            if ($num) {

                $data['limit']  =   'limit '.($num > $limit ? $limit : $num);
                $nid            =   $sysMsgM->delInfo('', $data);
            }
        }

        if ($nid) {
            if ($num - $limit > 0) {

                $this->admin_json(2, '数据表：'.$clearTable.' 已清理');
            } else {

                $this->admin_json(0, '数据表：'.$clearTable.' 已清理');
            }
        } else {

            $this->admin_json(1, '数据表：'.$clearTable.' 清理异常，请稍候重试！');
        }
    }

}

?>