<?php


class hotkey_model extends model
{

    /**
     * 引用log类，添加用户日志
     */
    private function addAdminLog($content, $opera = '', $type = '', $opera_id = '')
    {
        require_once ('log.model.php');

        $LogM = new log_model($this->db, $this->def);

        return $LogM->addAdminLog($content, $opera = '', $type = '', $opera_id = '');
    }

    function getHotkeyOne($Where = array(), $data = array())
    {
        $field = $data['field'] ? $data['field'] : '*';

        $info = $this->select_once('hot_key', $Where, $field);
        return $info;
    }

    function upHotkey($Where = array(), $data = array())
    {
        $nid = $this->update_once('hot_key', $data, $Where);

        return $nid;
    }

    function getList($whereData, $data = array())
    {
        $field  =  !empty($data['field']) ? $data['field'] : '*';
        $List = $this->select_all('hot_key', $whereData, $field);

        return $List;
    }

    function addInfo($setData)
    {
        $nid = $this->insert_into('hot_key', $setData);
        
        return $nid;
    }

    public function delHotkey($whereData = array())
    {
        
        $return =   array('layertype' => 0);
        
        if (! empty($whereData)) {
        
            if (! empty($whereData['id']) && $whereData['id'][0] == 'in') {
            
                $return['layertype']    =   1;
            }
            $return['id']       =   $this->delete_all('hot_key', $whereData, '');

            $return['msg']      =   yun_at('admin_tool_00574');
            $return['errcode']  =   $return['id'] ? '9' : '8';
            $return['msg']      =   $return['id'] ? $return['msg'] . 'admin_user_00187' : $return['msg'] . 'admin_user_00186';
        } else {
            
            $return['msg']      =   yun_at('common_01066');
            $return['errcode']  =   8;
        }
        
        return $return;
    }

    public function recupHotkey($setData = array())
    {
        if (! empty($setData)) {
            
            $type   =   $setData['type'];
            
            $nid    =   $this -> upHotkey(array('id' => $setData['id']), array($type => $setData['rec']));
            
            $row    =   $this -> getHotkeyOne(array('id' => $setData['id']));
            
            if ($type == "bold") {
            
                $this->addAdminLog('admin_system_00045' . $row['name'] . 'common_01270');
            } elseif ($type == "tuijian") {
                
                $this->addAdminLog('admin_system_00045' . $row['name'] . 'common_01272');
            } elseif ($type == "check") {
                
                $this->addAdminLog('admin_system_00045' . $row['name'] . 'common_01271');
            }

            return $nid;
        }
    }
}
?>