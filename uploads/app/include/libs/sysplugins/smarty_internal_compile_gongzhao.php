<?php
class Smarty_Internal_Compile_gongzhao extends Smarty_Internal_CompileBase{
    public $required_attributes = array('item');
    public $optional_attributes = array('name', 'key', 't_len', 'limit','ispage','endtime','order','sort');
    public $shorttag_order = array('from', 'item', 'key', 'name');
    public function compile($args, $compiler, $parameter){
        $_attr = $this->getAttributes($compiler, $args);

        $from = $_attr['from'];
        $item = $_attr['item'];
        $name = $_attr['item'];
        $name=str_replace('\'','',$name);
        $name=$name?$name:'list';$name='$'.$name;
        if (!strncmp("\$_smarty_tpl->tpl_vars[$item]", $from, strlen($item) + 24)) {
            $compiler->trigger_template_error("item variable {$item} may not be the same variable as at 'from'", $compiler->lex->taglineno);
        }

        //自定义标签 START
        $OutputStr=''.$name.'=array();$time=time();$paramer='.ArrayToString($_attr,true).';
		global $db,$db_config,$config;
		$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
		$paramer = $ParamerArr[arr];
		$Purl =  $ParamerArr[purl];
        global $ModuleName;
        if(!$Purl["m"]){
            $Purl["m"]=$ModuleName;
        }
		$where = 1;
		//分站
		if($config[\'did\']){
			$where.=" and (`did`=\'".$config[\'did\']."\' or `did`=-1)";
		}else{
			$where.=" and (`did`=-1 OR `did`=0 OR did=\'\')";
		}
        $where.=" and (`startime`<=".time()." or `startime`=0 or `startime` is null)";
        $where.=" and (`endtime`>".time()." or `endtime`=0 or `endtime` is null)";
		if($paramer[limit]){
			$limit=" LIMIT ".$paramer[limit];
		}else{
			$limit=" LIMIT 20";
		}
		if($paramer[ispage]){
			$limit = PageNav($paramer,$_GET,"gongzhao",$where,$Purl,"",0,$_smarty_tpl);
		}
		//排序字段 默认按照xuanshang排序
		if($paramer[order]){
			$where.="  ORDER BY `".$paramer[order]."`";
            if($paramer[sort]){
    			$where.=" ".$paramer[sort];
    		}
		}else{
            //排序方式默认按倒序倒序、开始时间倒序
			$where.="  ORDER BY `rec` DESC,`startime` DESC, `datetime` DESC";
		}

		'.$name.'=$db->select_all("gongzhao",$where.$limit);
		if(is_array('.$name.')){
			foreach('.$name.' as $key=>$value){
				//截取标题
				if($paramer[t_len]){
					'.$name.'[$key][title_n] = mb_substr($value[\'title\'],0,$paramer[t_len],"utf-8");
				}
				'.$name.'[$key][time]=date("Y-m-d",$value[startime]);
                '.$name.'[$key][pic_n]=checkpic($value[pic], $config[sy_gongzhaologo]);
				'.$name.'[$key][url] = Url("gongzhao",array("id"=>$value[id]),"1");
			}
		}';
        //自定义标签 START
        //global $DiyTagOutputStr;
       // $DiyTagOutputStr[]=$OutputStr;
        return SmartyOutputStr($this,$compiler,$_attr,'gongzhao',$name,$OutputStr,$name);
    }
}
class Smarty_Internal_Compile_gongzhaoelse extends Smarty_Internal_CompileBase{
    public function compile($args, $compiler, $parameter){
        $_attr = $this->getAttributes($compiler, $args);

        list($openTag, $nocache, $item, $key) = $this->closeTag($compiler, array('gongzhao'));
        $this->openTag($compiler, 'gongzhaoelse', array('gongzhaoelse', $nocache, $item, $key));

        return "<?php }\nif (!\$_smarty_tpl->tpl_vars[$item]->_loop) {\n?>";
    }
}
class Smarty_Internal_Compile_gongzhaoclose extends Smarty_Internal_CompileBase{
    public function compile($args, $compiler, $parameter){
        $_attr = $this->getAttributes($compiler, $args);
        if ($compiler->nocache) {
            $compiler->tag_nocache = true;
        }

        list($openTag, $compiler->nocache, $item, $key) = $this->closeTag($compiler, array('gongzhao', 'gongzhaoelse'));

        return "<?php } ?>";
    }
}
