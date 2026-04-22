<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 13:00:32
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/admin/index.htm" */ ?>
<?php /*%%SmartyHeaderCode:71270829569e855f0617803-06056704%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '564bd058cb5a3d1d6e8e30344b04e14a3d4a3a9b' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/admin/index.htm',
      1 => 1708477840,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '71270829569e855f0617803-06056704',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'menu' => 0,
    'mval' => 0,
    'navigation' => 0,
    'nval' => 0,
    'one_menu' => 0,
    'oval' => 0,
    'two_menu' => 0,
    'tval' => 0,
    'power' => 0,
    'admin_lasttime' => 0,
    'pytoken' => 0,
    'baseUrl' => 0,
    'sy_weburl' => 0,
    'indexData' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e855f063bbf0_29484835',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e855f063bbf0_29484835')) {function content_69e855f063bbf0_29484835($_smarty_tpl) {?><?php if (!is_callable('smarty_modifier_date_format')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/modifier.date_format.php';
?><!DOCTYPE html>
<html lang="en">

<head>
    <title><?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
 - 管理后台</title>
    <?php echo '<script'; ?>
 src="../app/template/admin/home_header.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
    <?php echo '<script'; ?>
 src="../app/template/admin/js/vue-router.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
    <link rel="stylesheet" href="../app/template/admin/adstyle/allcss/index.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
">
</head>

<body>
    <section id="homeapp" class="subjectDome">
        <div class="subContent">
            <div class="subContNav" :class="{'subContNavCur': subWidth}">
                <div class="subHeadtop">
                    <div class="subHeadLogo">
                        <img src="../app/template/admin/images/admin_new_logo.png" alt="">
                    </div>
                    <!-- 导航开关按钮预留 -->
                    <div class="widthButn" @click="subWidth = !subWidth" :class="{'widthButnCur': subWidth}">
                        <el-tooltip class="item" effect="dark" content="收起导航" placement="right">
                            <i class="el-icon-s-fold iconone"></i>
                        </el-tooltip>
                        <i class="el-icon-s-unfold icontwo"></i>
                    </div>
                </div>
                <div class="subContNavTite">
                    <div class="subNavTite">
                        <i></i>
                        <span>功能导航</span>
                        <i></i>
                    </div>
                    <div class="subNavLogo">
                        <img src="../app/template/admin/images/navimg.png" alt="">
                    </div>
                </div>
                <div class="subContNavLink">
                    <!--请不要删除我-->
                    <ul v-if="cur_menu == 0">
                        <li :class="{'subContLinkCur': cur_menu_one == 0}">
                            <div class="subNavLinkTite" @click="MenuOpenChange(0)">
                                <div class="subNavLinkImg kjcz">
                                    <span>快捷操作</span>
                                </div>
                                <div class="subNavLinkIcon" :class="{'subNavLinkIconCur': checkMenuOpen(0)}">
                                    <i class="el-icon-arrow-up iconup"></i>
                                    <i class="el-icon-arrow-down icondwon"></i>
                                </div>
                            </div>
                            <?php if ($_smarty_tpl->tpl_vars['menu']->value) {?>
                            <el-collapse-transition>
                                <div class="subNavLinkText" v-show="checkMenuOpen(0)">
                                    <?php  $_smarty_tpl->tpl_vars['mval'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['mval']->_loop = false;
 $_smarty_tpl->tpl_vars['mkey'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['menu']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['mval']->key => $_smarty_tpl->tpl_vars['mval']->value) {
$_smarty_tpl->tpl_vars['mval']->_loop = true;
 $_smarty_tpl->tpl_vars['mkey']->value = $_smarty_tpl->tpl_vars['mval']->key;
?>
                                    <a href="javascript:void(0);" @click="checkMenuTwo(0, 0, <?php echo $_smarty_tpl->tpl_vars['mval']->value['id'];?>
, '<?php echo $_smarty_tpl->tpl_vars['mval']->value['name'];?>
', '<?php echo $_smarty_tpl->tpl_vars['mval']->value['path'];?>
')">
                                        <span :class="{'curspan': cur_menu_two == <?php echo $_smarty_tpl->tpl_vars['mval']->value['id'];?>
}"><?php echo $_smarty_tpl->tpl_vars['mval']->value['name'];?>
</span>
                                    </a>
                                    <?php } ?>
                                </div>
                            </el-collapse-transition>
                            <?php }?>
                        </li>
                    </ul>
                    <?php  $_smarty_tpl->tpl_vars['nval'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['nval']->_loop = false;
 $_smarty_tpl->tpl_vars['nkey'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['navigation']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['nval']->key => $_smarty_tpl->tpl_vars['nval']->value) {
$_smarty_tpl->tpl_vars['nval']->_loop = true;
 $_smarty_tpl->tpl_vars['nkey']->value = $_smarty_tpl->tpl_vars['nval']->key;
?>
                    <?php if ($_smarty_tpl->tpl_vars['one_menu']->value[$_smarty_tpl->tpl_vars['nval']->value['id']]) {?>
                    <ul v-if="cur_menu == <?php echo $_smarty_tpl->tpl_vars['nval']->value['id'];?>
">
                        <?php  $_smarty_tpl->tpl_vars['oval'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['oval']->_loop = false;
 $_smarty_tpl->tpl_vars['okey'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['one_menu']->value[$_smarty_tpl->tpl_vars['nval']->value['id']]; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['oval']->key => $_smarty_tpl->tpl_vars['oval']->value) {
$_smarty_tpl->tpl_vars['oval']->_loop = true;
 $_smarty_tpl->tpl_vars['okey']->value = $_smarty_tpl->tpl_vars['oval']->key;
?>
                        <li :class="{'subContLinkCur': cur_menu_one == <?php echo $_smarty_tpl->tpl_vars['oval']->value['id'];?>
}">
                            <div class="subNavLinkTite" @click="MenuOpenChange(<?php echo $_smarty_tpl->tpl_vars['oval']->value['id'];?>
)">
                                <div class="subNavLinkImg <?php echo $_smarty_tpl->tpl_vars['oval']->value['classname'];?>
">
                                    <span><?php echo $_smarty_tpl->tpl_vars['oval']->value['name'];?>
</span>
                                </div>
                                <div class="subNavLinkIcon" :class="{'subNavLinkIconCur': checkMenuOpen(<?php echo $_smarty_tpl->tpl_vars['oval']->value['id'];?>
)}">
                                    <i class="el-icon-arrow-up iconup"></i>
                                    <i class="el-icon-arrow-down icondwon"></i>
                                </div>
                            </div>
                            <?php if ($_smarty_tpl->tpl_vars['two_menu']->value[$_smarty_tpl->tpl_vars['oval']->value['id']]) {?>
                            <el-collapse-transition>
                                <div class="subNavLinkText" v-show="checkMenuOpen(<?php echo $_smarty_tpl->tpl_vars['oval']->value['id'];?>
)">
                                    <?php  $_smarty_tpl->tpl_vars['tval'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['tval']->_loop = false;
 $_smarty_tpl->tpl_vars['tkey'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['two_menu']->value[$_smarty_tpl->tpl_vars['oval']->value['id']]; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['tval']->key => $_smarty_tpl->tpl_vars['tval']->value) {
$_smarty_tpl->tpl_vars['tval']->_loop = true;
 $_smarty_tpl->tpl_vars['tkey']->value = $_smarty_tpl->tpl_vars['tval']->key;
?>
                                    <a href="javascript:void(0);" @click="checkMenuTwo(<?php echo $_smarty_tpl->tpl_vars['nval']->value['id'];?>
, <?php echo $_smarty_tpl->tpl_vars['oval']->value['id'];?>
, <?php echo $_smarty_tpl->tpl_vars['tval']->value['id'];?>
, '<?php echo $_smarty_tpl->tpl_vars['tval']->value['name'];?>
', '<?php echo $_smarty_tpl->tpl_vars['tval']->value['path'];?>
')">
                                        <span :class="{'curspan': cur_menu_two == <?php echo $_smarty_tpl->tpl_vars['tval']->value['id'];?>
}"><?php echo $_smarty_tpl->tpl_vars['tval']->value['name'];?>
</span>
                                    </a>
                                    <?php } ?>
                                </div>
                            </el-collapse-transition>
                            <?php }?>
                        </li>
                        <?php } ?>
                    </ul>
                    <?php }?>
                    <?php } ?>
                </div>
            </div>
            <div class="subContPage" :class="{'subContPageCur': subWidth}">
                <div class="subHeader">
                    <div class="subHeaderLeft">
                        <div class="subHeadNavs">
                            <ul>
                                <!--请不要删除我-->
                                <li :class="{'subHeadNavCue': cur_menu == 0}">
                                    <a href="javascript:void(0)"  @click="checkMenu(0)">
                                        首页
                                    </a>
                                </li>
                                <?php  $_smarty_tpl->tpl_vars['nval'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['nval']->_loop = false;
 $_smarty_tpl->tpl_vars['nkey'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['navigation']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['nval']->key => $_smarty_tpl->tpl_vars['nval']->value) {
$_smarty_tpl->tpl_vars['nval']->_loop = true;
 $_smarty_tpl->tpl_vars['nkey']->value = $_smarty_tpl->tpl_vars['nval']->key;
?>
                                <li :class="{'subHeadNavCue': cur_menu == <?php echo $_smarty_tpl->tpl_vars['nval']->value['id'];?>
}"><a href="javascript:void(0)" @click="checkMenu(<?php echo $_smarty_tpl->tpl_vars['nval']->value['id'];?>
)"><?php echo $_smarty_tpl->tpl_vars['nval']->value['name'];?>
</a></li>
                                <?php } ?>
                            </ul>
                        </div>
                    </div>
                    <div class="subHeadRight">
                        <div class="subHeadRigNumer">
                            <el-popover placement="bottom" width="450" trigger="hover" @show="getMsgNum">
                                <div class="subHeaNumerDomes" v-loading="msgNumLoad">
                                    <div class="subHeaNumerName">
                                        <span>待处理事项</span>
                                    </div>
                                    <!-- 有提示时显示 -->
                                    <div class="subHeaNumLibos" v-if="msgNumData.length>0">
                                        <ul>
											<template  v-for="item in msgNumData">
												<li  :key="item.name" @click="checkMenuTwo(item.menudata.nval,item.menudata.oval,item.menudata.tval,item.menudata.name,item.menudata.path,item.menudata.query)">
													<div class="subHeaNumminc">
														<a href="javascript:void(0);">{{item.name}}</a>
													</div>
													<div class="subHeaNumData">
														<a href="javascript:void(0);">( {{item.num}} )</a>
													</div>
												</li>
											</template>
                                        </ul>
                                    </div>
                                    <!-- 无提示时显示 -->
                                    <div class="subHeaNumNones" v-else>
                                        <el-empty description="暂无待处理事项"></el-empty>
                                    </div>
                                </div>
                                <el-button slot="reference">
                                    <el-badge :value="msgNum" :max="99" :hidden="msgNum===0" class="item">
                                        <el-button size="small">
                                            <div class="subHeadRigIcon">
                                                <img src="../app/template/admin/images/head2.png" alt="">
                                            </div>
                                        </el-button>
                                    </el-badge>
                                </el-button>
                            </el-popover>
                        </div>
                        <div class="subHeadRigList" @click="openPage('<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
')">
                            <div class="subHeadRigIcon">
                                <el-tooltip class="item" effect="dark" content="网站首页" placement="bottom-end">
                                    <img src="../app/template/admin/images/head3.png" alt="">
                                </el-tooltip>
                            </div>
                        </div>
                        <?php if (in_array('161',$_smarty_tpl->tpl_vars['power']->value)) {?>
                        <div class="subHeadRigList" @click="openMap">
                            <div class="subHeadRigIcon">
                                <el-tooltip class="item" effect="dark" content="后台地图" placement="bottom-end">
                                    <img src="../app/template/admin/images/head4.png" alt="">
                                </el-tooltip>
                            </div>
                        </div>
                        <?php }?>
                        <div class="subHeadRigList" @click="clearCache">
                            <div class="subHeadRigIcon">
                                <el-tooltip class="item" effect="dark" content="清除缓存" placement="bottom-end">
                                    <img src="../app/template/admin/images/head5.png" alt="">
                                </el-tooltip>
                            </div>
                        </div>
                        <div class="subHeadRigUser">
                            <el-popover placement="top-start" width="140" trigger="hover">
                                <div class="subHeadlogout">
                                    <div class="subjeHeadDenl">
                                        <h3>最近登录</h3>
                                        <span><?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['admin_lasttime']->value,"%Y-%m-%d %H:%M:%S");?>
</span>
                                    </div>
                                    <div class="subjeHeadFlex" @click="openShortcutMenu">
                                        <img src="../app/template/admin/images/index_topright4.png" alt="">
                                        <span>快捷菜单管理</span>
                                    </div>
                                    <div class="subjeHeadTuichus">
                                        <?php if (in_array('809',$_smarty_tpl->tpl_vars['power']->value)) {?>
                                        <div @click="checkMenuTwo(5, 102, 104, '我的帐号', '/myaccount', {ly: 'pass'})">
                                            <img src="../app/template/admin/images/index_topright7.png" alt="">
                                            <span>修改密码</span>
                                        </div>
                                        <?php }?>
                                        <div @click="logout">
                                            <img src="../app/template/admin/images/admin_navicon7.png" alt="">
                                            <span>退出登录</span>
                                        </div>
                                    </div>
                                </div>
                                <el-button slot="reference">
                                    <div class="subHeadRigIcon"><img src="../app/template/admin/images/head6.png" alt=""></div>
                                </el-button>
                            </el-popover>
                        </div>
                    </div>
                </div>
                <div class="subContPageTips">
                    <div class="subContPageWidth">
                        <ul>
                            <li @click="checkTab(tabIndex)" :class="{'subContTipsCur': cur_menu_two == tabItem.two_menu_id}" v-for="(tabItem, tabIndex) in tabList" :key="tabIndex">
                                <div class="spana">
                                    <span class="curspan"><template>{{tabItem.name}}</template></span>
                                    <i v-if="tabItem.isdel" class="el-icon-close" @click.stop="closeTab(tabIndex)"></i>
                                </div>
                            </li>
                        </ul>
                    </div>
                    <div class="subContPageCose">
                        <el-popover placement="bottom" width="100" trigger="hover">
                            <div class="subPageBurt">
                                <div>
                                    <el-button type="text" @click="refresh" icon="el-icon-refresh">刷新</el-button>
                                </div>
                                <div>
                                    <el-button type="text" icon="el-icon-close" @click="closeTabOther">关闭其他</el-button>
                                </div>
                                <div>
                                    <el-button type="text" icon="el-icon-close" @click="closeTabAll">关闭全部</el-button>
                                </div>
                            </div>
                            <el-button slot="reference" icon="el-icon-menu"></el-button>
                        </el-popover>
                    </div>
                </div>
                <div class="subContPageInfo">
                    <!-- 路由渲染组件 -->
                    <transition mode="out-in">
                        <router-view></router-view>
                    </transition>
                </div>
            </div>
        </div>
        <!-- 弹窗 -->
        <div class="homeelDialog">
            <el-dialog title="快捷菜单管理" width="680px" :visible.sync="dialogShortcutMenu" :with-header="true" :modal-append-to-body="false">
                <div class="homeDiaCaidan" style="height: 420px;">
                    <el-checkbox-group v-model="formShortcutMenu">
                        <div class="homeDiaCaiConts" v-for="(oneSMItem, oneSMIndex) in navigation" :key="oneSMIndex">
                            <div class="homeDiaCaiOntite">
                                <span>{{oneSMItem.name}}</span>
                            </div>
                            <div class="homeDiaCaiLis" v-for="(twoSMItem, twoSMIndex) in oneSMItem.children" :key="twoSMIndex">
                                <div class="homeCaiTwoTite">
                                    <span>{{twoSMItem.name}}</span>
                                </div>
                                <div class="homeCaiTwoNeir" v-if="twoSMItem.children">
                                    <div class="homeCaiTwocheck" v-for="(threeSMItem, threeSMIndex) in twoSMItem.children" :key="threeSMIndex">
                                        <el-checkbox :label="threeSMItem.id">{{threeSMItem.name}}</el-checkbox>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </el-checkbox-group>
                </div>
                <div slot="footer" class="dialog-footer">
                    <el-button @click="dialogShortcutMenu = false">取 消</el-button>
                    <el-button type="primary" @click="saveShortcutMenu">确 定</el-button>
                </div>
            </el-dialog>
        </div>
        <div class="homeelDialog">
            <el-dialog title="后台地图" width="680px" :visible.sync="dialogMap" :with-header="true" :modal-append-to-body="false">
                <div class="homeDiaCaiSouse">
                    <div class="homeDiaCaiFrom">
                        <el-input v-model="searchFormMap.keyword" placeholder="请输入你要搜索的关键字"></el-input>
                    </div>
                    <div class="homeDiaCaiFrom">
                        <el-button type="primary" size="small" icon="el-icon-search" @click="searchMap">搜索</el-button>
                    </div>
                </div>
                <div class="homeDiaCaidan" style="height: 420px;">
                    <div class="homeDiaCaiConts" v-for="(oneMItem, oneMIndex) in navigation" :key="oneMIndex" v-if="!oneMItem.hide">
                        <div class="homeDiaCaiOntite">
                            <span>{{oneMItem.name}}</span>
                        </div>
                        <div class="homeDiaCaiLis" v-for="(twoMItem, twoMIndex) in oneMItem.children" :key="twoMIndex" v-if="!twoMItem.hide">
                            <div class="homeCaiTwoTite">
                                <span>{{twoMItem.name}}</span>
                            </div>
                            <div class="homeCaiTwoNeir" v-if="twoMItem.children">
                                <div class="homeCaiTwocheck" v-for="(threeMItem, threeMIndex) in twoMItem.children" :key="threeMIndex" v-if="!threeMItem.hide">
                                    <a href="javascript:void(0);" @click="checkMenuTwo(oneMItem.id, twoMItem.id, threeMItem.id, threeMItem.name, threeMItem.path);dialogMap = false;">{{threeMItem.name}}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </el-dialog>
        </div>
    </section>
    <!-- script -->
    <?php echo '<script'; ?>
 src="../app/template/admin/js/router.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
    <?php echo '<script'; ?>
>
    
    localStorage.setItem("pytoken", '<?php echo $_smarty_tpl->tpl_vars['pytoken']->value;?>
');
    localStorage.setItem("baseUrl", '<?php echo $_smarty_tpl->tpl_vars['baseUrl']->value;?>
');
    localStorage.setItem("sy_weburl", '<?php echo $_smarty_tpl->tpl_vars['sy_weburl']->value;?>
');
    var pic_accept = '.<?php echo $_smarty_tpl->tpl_vars['config']->value['pic_type'];?>
';
    localStorage.setItem("pic_accept", pic_accept ? pic_accept.split(',').join(',.') : '.jpg,.png,.jpeg,.bmp,.gif');
    var file_accept = '.<?php echo $_smarty_tpl->tpl_vars['config']->value['file_type'];?>
';
    localStorage.setItem("file_accept", file_accept ? file_accept.split(',').join(',.') : '.doc,.docx,.xls,.pdf');

	var msgNumDef = {
		company_job:{name:'待审核职位',num:0,menudata:{nval:1,oval:6,tval:40,name:'职位管理',path:'/companyjob',query:{state: '4'}}},
		partjob:{name:'待审核兼职',num:0,menudata:{nval:1,oval:6,tval:40,name:'职位管理',path:'/companyjob',query:{state: '4', tabs: 'partjob'}}},
		company:{name:'待审核企业',num:0,menudata:{nval:1,oval:6,tval:16,name:'企业用户',path:'/companycrm',query:{status: '4'}}},
		company_cert:{name:'待审核企业资质',num:0,menudata:{nval:1,oval:6,tval:47,name:'认证&审核',path:'/companyrz',query:{status: '3'}}},
		comlogo:{name:'待审核企业LOGO',num:0,menudata:{nval:1,oval:6,tval:47,name:'认证&审核',path:'/companyrz',query:{status: '1', tabs: 'logo'}}},
		comshow:{name:'待审核企业环境',num:0,menudata:{nval:1,oval:6,tval:47,name:'认证&审核',path:'/companyrz',query:{status: '1', tabs: 'show'}}},
		combanner:{name:'待审核企业横幅',num:0,menudata:{nval:1,oval:6,tval:47,name:'认证&审核',path:'/companyrz',query:{status: '1', tabs: 'banner'}}},
		company_product:{name:'待审核企业产品',num:0,menudata:{nval:1,oval:6,tval:47,name:'认证&审核',path:'/companyrz',query:{status: '3', tabs: 'product'}}},
		company_news:{name:'待审核企业新闻',num:0,menudata:{nval:1,oval:6,tval:47,name:'认证&审核',path:'/companyrz',query:{status: '3', tabs: 'news'}}},
		once_job:{name:'待审核店铺招聘',num:0,menudata:{nval:1,oval:6,tval:48,name:'店铺招聘',path:'/weipin_once',query:{status: '3'}}},
		resume_expect:{name:'待审核简历',num:0,menudata:{nval:1,oval:7,tval:33,name:'简历管理',path:'/resume',query:{status: '4'}}},
		resumetrust:{name:'待审核委托简历',num:0,menudata:{nval:1,oval:7,tval:33,name:'简历管理',path:'/resume',query:{status: '3', tabs: 'trust'}}},
		appealnum:{name:'会员申诉',num:0,menudata:{nval:1,oval:401,tval:402,name:'会员列表',path:'/usercrm',query:{tabs: 'four',params:{appealstate:'1'}}}},
		logout:{name:'待处理注销账号',num:0,menudata:{nval:1,oval:401,tval:402,name:'会员列表',path:'/usercrm',query:{status: '1', tabs: 'three'}}},
		usercertNum:{name:'待审核个人认证',num:0,menudata:{nval:1,oval:7,tval:34,name:'认证&审核',path:'/renzheng',query:{status: '2'}}},
		userpic:{name:'待审核个人头像',num:0,menudata:{nval:1,oval:7,tval:34,name:'认证&审核',path:'/renzheng',query:{status: '1', tabs: 'second'}}},
		usershow:{name:'待审核作品案例',num:0,menudata:{nval:1,oval:7,tval:34,name:'认证&审核',path:'/renzheng',query:{status: '1', tabs: 'third'}}},
		linkNum:{name:'待审核友情链接',num:0,menudata:{nval:5,oval:121,tval:194,name:'友情链接',path:'/friendlink',query:{state: '2'}}},
		tiny:{name:'待审核普工简历',num:0,menudata:{nval:1,oval:156,tval:36,name:'普工简历',path:'/weipin_tiny',query:{status: '2'}}},
		zphcom:{name:'待审核参会企业',num:0,menudata:{nval:2,oval:12,tval:61,name:'现场招聘会',path:'/xczph',query:{status: '3', tabs: 'com'}}},
		ask:{name:'待审核问答',num:0,menudata:{nval:2,oval:15,tval:66,name:'问答管理',path:'/question',query:{status: '0'}}},
		answer_msg:{name:'待审核问答回复',num:0,menudata:{nval:2,oval:15,tval:66,name:'问答管理',path:'/question',query:{status: '0', drawer: 'answer'}}},
		answerreview_msg:{name:'待审核问答评论',num:0,menudata:{nval:2,oval:15,tval:66,name:'问答管理',path:'/question',query:{status: '0', drawer: 'comment'}}},
		redeem:{name:'待审核商品兑换',num:0,menudata:{nval:3,oval:74,tval:77,name:'兑换记录',path:'/shoplist',query:{status: '-1'}}},
		order:{name:'待处理充值订单',num:0,menudata:{nval:3,oval:161,tval:162,name:'充值订单',path:'/chongzhidd',query:{order_state: '1'}}},
		specialcom:{name:'待审核企业专题',num:0,menudata:{nval:3,oval:343,tval:49,name:'招聘专题',path:'/special',query:null}},
		reportjob:{name:'待处理举报职位',num:0,menudata:{nval:3,oval:176,tval:178,name:'举报职位',path:'/reportjob',query:{status: '0'}}},
		reportresume:{name:'待处理举报简历',num:0,menudata:{nval:3,oval:176,tval:179,name:'举报简历',path:'/reportresume',query:{status: '0'}}},
		reportask:{name:'待处理举报问答',num:0,menudata:{nval:3,oval:176,tval:180,name:'举报问答',path:'/reportask',query:{status: '0'}}},
		reportgw:{name:'待处理投诉顾问',num:0,menudata:{nval:3,oval:176,tval:181,name:'举报顾问',path:'/reportadvise',query:{status: '0'}}},
		handlenum:{name:'待处理意见反馈',num:0,menudata:{nval:5,oval:198,tval:65,name:'意见反馈',path:'/feedback',query:{status: '1'}}},
		errlog:{name:'错误日志',num:0,menudata:{nval:5,oval:198,tval:814,name:'错误日志',path:'/errorlog',query:null}},
		yqmb_msg:{name:'待审核面试模板',num:0,menudata:{nval:1,oval:6,tval:46,name:'面试管理',path:'/companyms',query:{status: '0', tabs: 'first'}}},
		usermsg_msg:{name:'待审核求职咨询',num:0,menudata:{nval:1,oval:7,tval:37,name:'求职咨询',path:'/zixun',query:{status: '0'}}},
		warning:{name:'待查看预警',num:0,menudata:{nval:5,oval:121,tval:175,name:'预警设置',path:'/warning',query:{status: '2'}}},
	};

    const homeapp = new Vue({
        el: '#homeapp',
        router,
        data: function() {
            return {
                subWidth: false,

                MenuOpen: [],
                // cur_menu: '<?php echo $_smarty_tpl->tpl_vars['navigation']->value[0]['id'];?>
',
                cur_menu: parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['nav_id'];?>
'),
                cur_menu_one: parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['one_menu_id'];?>
'),
                cur_menu_two: parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['two_menu_id'];?>
'),
                tabArr: [parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['two_menu_id'];?>
')],
                tabList: [{
                    nav_id: parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['nav_id'];?>
'),
                    one_menu_id: parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['one_menu_id'];?>
'),
                    two_menu_id: parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['two_menu_id'];?>
'),
                    name: '<?php echo $_smarty_tpl->tpl_vars['indexData']->value['name'];?>
',
                    path: '<?php echo $_smarty_tpl->tpl_vars['indexData']->value['path'];?>
',
                    isdel: false
                }],
                mine: null,

                msgNumData: [], // 消息数量数据
				msgNum:0,
				msgNumLoad:true,

                navigation: [], // 导航
                customizeIds: [], // 快捷导航ID

                dialogMap: false,
                searchFormMap: {
                    keyword: ''
                },

                dialogShortcutMenu: false,
                formShortcutMenu: [],

                saveLoading: false,
            }
        },
        created: function() {
            let cur_menu = localStorage.getItem("cur_menu"),
                cur_menu_one = localStorage.getItem("cur_menu_one"),
                cur_menu_two = localStorage.getItem("cur_menu_two"),
                tabArrStr = localStorage.getItem("tabArr"),
                tabListStr = localStorage.getItem("tabList");

            if (cur_menu) {
                this.cur_menu = parseInt(cur_menu);
            }
            if (cur_menu_one) {
                this.cur_menu_one = parseInt(cur_menu_one);
            }
            if (cur_menu_two) {
                this.cur_menu_two = parseInt(cur_menu_two);
            }
            if (tabArrStr) {
                this.tabArr = JSON.parse(tabArrStr);
            }
            if (tabListStr) {
                this.tabList = JSON.parse(tabListStr);
            }
            this.getMsgNum();
        },
        methods: {
            
            logout() {
                var that = this
                axios.post('index.php?m=index&c=logout', {}).then(function(response) {
                    var res = response.data;
                    if (res.error == 0) {
                        // 清空localStorage
                        localStorage.clear();
                        window.location.reload();
                    } else {
                        that.$message.error(res.msg);
                    }
                }).catch(function(error) {
                    console.log(error);
                })
            },
            refresh() {
                location.reload();
            },
            checkMenu(val) {
                this.cur_menu = val;
                localStorage.setItem("cur_menu", val);
                let tab = this.tabList[0];
                if (val == 0 && tab.path == '/index') { // 如果点击的是首页并且有首页tab时，才能联动跳转
					this.getMsgNum();
                    this.checkMenuTwo(tab.nav_id, tab.one_menu_id, tab.two_menu_id, tab.name, tab.path);
                }
            },
            MenuOpenChange(val) {
                let idx = this.MenuOpen.indexOf(val);
                if (idx > -1) {
                    this.MenuOpen.splice(idx, 1)
                } else {
                    this.MenuOpen.push(val)
                }
            },
            checkMenuOpen(val) {
                let idx = this.MenuOpen.indexOf(val);
                return idx > -1 ? false : true;
            },
            checkMenuTwo(nval, oval, tval, name, path, query = null) {
                nval = parseInt(nval);
                oval = parseInt(oval);
                tval = parseInt(tval);

                this.cur_menu_one = oval;
                this.cur_menu_two = tval;
                localStorage.setItem("cur_menu_one", oval);
                localStorage.setItem("cur_menu_two", tval);

                if (this.tabArr.indexOf(tval) == -1) {
                    this.tabArr.push(tval);
                    this.tabList.push({
                        nav_id: nval,
                        one_menu_id: oval,
                        two_menu_id: tval,
                        name: name,
                        path: path,
                        isdel: true
                    })
                    localStorage.setItem("tabArr", JSON.stringify(this.tabArr));
                    localStorage.setItem("tabList", JSON.stringify(this.tabList));
                }


                if (this.$route.path === path) { // 当前链接和点击链接相同
                    if (query) { // 如果为带参数
                        if (JSON.stringify(query) === JSON.stringify(this.$route.query)) { // 两次参数一致，刷新
                            this.$router.go(0);
                        } else { // 不一致，深度重载
                            this.$router.push({
                                path: path,
                                query: query
                            }).catch(err => {}) // 因为path没变，会报错，这里进行拦截，避免抛出异常
                            this.$router.go(0);
                        }
                        return false;
                    } else {
                        return false; // 不允许重复点击，结束后续执行
                    }
                }

                this.$router.push({
                    path: path,
                    query: query
                })
            },
            checkTab(idx) {
                let tab = this.tabList[idx];

                if (this.cur_menu_two == tab.two_menu_id) { // 不可重复点击
                    return
                }

                this.cur_menu = tab.nav_id;
                this.cur_menu_one = tab.one_menu_id;
                this.cur_menu_two = tab.two_menu_id;

                localStorage.setItem("cur_menu", this.cur_menu);
                localStorage.setItem("cur_menu_one", this.cur_menu_one);
                localStorage.setItem("cur_menu_two", this.cur_menu_two);

                this.$router.push({
                    path: tab.path
                })
            },
            closeTab(idx) {
                if (this.tabArr.length > 1) {
                    let tab,
                        delTab = this.tabList[idx];
                    if (idx == 0) {
                        tab = this.tabList[idx + 1];
                    } else {
                        tab = this.tabList[idx - 1];
                    }

                    if (delTab.two_menu_id == this.cur_menu_two) { // 删除的是当前选中的才会触发变更
                        this.cur_menu = tab.nav_id;
                        this.cur_menu_one = tab.one_menu_id;
                        this.cur_menu_two = tab.two_menu_id;

                        this.$router.push({
                            path: tab.path // TODO不能跳转成功
                        })
                    }
                } else {
                    this.cur_menu = parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['nav_id'];?>
');
                    this.cur_menu_one = parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['one_menu_id'];?>
');
                    this.cur_menu_two = parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['two_menu_id'];?>
');
                    this.$router.push({
                        path: '<?php echo $_smarty_tpl->tpl_vars['indexData']->value['path'];?>
' // 正常跳转至首页
                    })
                }
                this.tabArr.splice(idx, 1);
                this.tabList.splice(idx, 1);

                this.setLocalStorage();
            },
            setLocalStorage() {
                localStorage.setItem("cur_menu", this.cur_menu);
                localStorage.setItem("cur_menu_one", this.cur_menu_one);
                localStorage.setItem("cur_menu_two", this.cur_menu_two);
                localStorage.setItem("tabArr", JSON.stringify(this.tabArr));
                localStorage.setItem("tabList", JSON.stringify(this.tabList));
            },
            closeTabOther() {
                let tabArr = this.tabArr,
                    tabNum = tabArr.length,
                    tabList = this.tabList,
                    cur_menu_two = this.cur_menu_two;

                if (tabNum > 2) {
                    let idx = tabArr.indexOf(cur_menu_two)
                    if (idx > -1) {
                        if (tabNum - 1 == idx) { // 最后一个，删除前面的  索引是0开始故-1
                            tabArr.splice(1, tabNum - 2); // 不清除首页和自身
                            tabList.splice(1, tabNum - 2); // 不清除首页和自身
                        } else { // 删除中间的
                            tabArr.splice(idx + 1, tabNum - 1 - idx); // 先删除自身后面的  计算出后边的实际个数
                            tabList.splice(idx + 1, tabNum - 1 - idx); // 先删除自身后面的  计算出后边的实际个数
                            if (idx > 1) { // 判断前面的是不是只有首页，大于1说明前面至少有1个选项卡
                                tabArr.splice(1, idx - 1); // 删除自身前面的  计算出前边的实际个数
                                tabList.splice(1, idx - 1); // 删除自身前面的  计算出前边的实际个数
                            }
                        }
                        this.tabArr = tabArr;
                        this.tabList = tabList;

                        localStorage.setItem("tabArr", JSON.stringify(tabArr));
                        localStorage.setItem("tabList", JSON.stringify(tabList));
                    }
                }
            },
            closeTabAll() {
                let tabNum = this.tabArr.length;
                if (tabNum > 1) {
                    this.tabArr.splice(1, tabNum - 1); // 不清除首页
                    this.tabList.splice(1, tabNum - 1); // 不清除首页

                    this.cur_menu = parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['nav_id'];?>
');
                    this.cur_menu_one = parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['one_menu_id'];?>
');
                    this.cur_menu_two = parseInt('<?php echo $_smarty_tpl->tpl_vars['indexData']->value['two_menu_id'];?>
');
                    this.$router.push({
                        path: '<?php echo $_smarty_tpl->tpl_vars['indexData']->value['path'];?>
' // 正常跳转至首页
                    })

                    this.setLocalStorage();
                }
            },
            topage(param = {}) {
                this.$router.push(param)
            },

            openPage(url) {
                window.open(url);
            },

            clearCache() {
                httpPost('m=index&c=del_cache', {}, {}, localStorage.getItem("baseUrl")).then(function(response) {
                    let res = response.data;

                    if (res.error > 0) {
                        message.error('清除缓存失败');
                    } else {
                        message.success('清除缓存成功');
                    }
                })
            },

            async openMap() {
                if (this.navigation.length > 0) { // 此判断是为了减少打开请求，不需要可直接删除
                    this.searchFormMap.keyword = '';
                    this.searchMap();
                } else {
                    await this.getMenu();
                }

                this.dialogMap = true;
            },

            async searchMap() {
                let that = this,
                    query = that.searchFormMap.keyword,
                    navigation = deepClone(that.navigation),
                    twoList = [],
                    threeList = [];

                if (navigation && navigation.length > 0) {
                    navigation.forEach(function(oneItem, oneKey) {
                        navigation[oneKey].hide = true; // 隐藏一级导航
                        twoList = oneItem.children;
                        if (twoList && twoList.length > 0) {
                            twoList.forEach(function(twoItem, twoKey) {
                                navigation[oneKey]['children'][twoKey].hide = true; // 隐藏二级导航
                                threeList = twoItem.children;
                                if (threeList && threeList.length > 0) {
                                    threeList.forEach(function(threeItem, threeKey) {
                                        if (query) {
                                            if (threeItem.name.includes(query)) { // 三级须包含关键字
                                                navigation[oneKey].hide = false; // 显示一级导航
                                                navigation[oneKey]['children'][twoKey].hide = false; // 显示二级导航
                                                navigation[oneKey]['children'][twoKey]['children'][threeKey].hide = false; // 二级导航标记显示
                                            } else {
                                                if (navigation[oneKey].hide !== false) { // 如果被标记过显示，后续不会在隐藏一级导航
                                                    navigation[oneKey].hide = true; // 隐藏一级导航
                                                }
                                                if (navigation[oneKey]['children'][twoKey].hide !== false) { // 如果被标记过显示，后续不会在隐藏二级导航
                                                    navigation[oneKey]['children'][twoKey].hide = true; // 隐藏二级导航
                                                }
                                                navigation[oneKey]['children'][twoKey]['children'][threeKey].hide = true; // 隐藏三级导航
                                            }
                                        } else { // 未搜索，显示所有导航
                                            navigation[oneKey].hide = false;
                                            navigation[oneKey]['children'][twoKey].hide = false;
                                            navigation[oneKey]['children'][twoKey]['children'][threeKey].hide = false;
                                        }
                                    })
                                }
                            })
                        }
                    })

                    that.navigation = navigation;
                }
            },

            async getMenu() {
                let response = await httpPost('m=index&c=getMenu', {}, {}, localStorage.getItem("baseUrl"));

                let res = response.data,
                    data = res.data;

                this.navigation = data.navigation;
                this.customizeIds = data.customizeIds;
            },

            async openShortcutMenu() {
                if (this.navigation.length > 0) { // 此判断是为了减少打开请求，不需要可直接删除
                    this.searchMap();
                } else {
                    await this.getMenu();
                }

                this.formShortcutMenu = this.customizeIds.length > 0 ? deepClone(this.customizeIds) : [];

                this.dialogShortcutMenu = true;
            },

            saveShortcutMenu() {
                let that = this,
                    params = {
                        chk_value: that.formShortcutMenu
                    };

                if (that.formShortcutMenu.length == 0) {
                    message.error('请至少选择一个');
                    return false;
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                httpPost('m=index&c=shortcut_menu', params, {}, localStorage.getItem("baseUrl")).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        that.saveLoading = false;
                        message.error(res.msg);
                    } else {
                        that.dialogShortcutMenu = false;
                        that.customizeIds = deepClone(that.formShortcutMenu);
                        message.success(res.msg, function() {
                            that.saveLoading = false;
                            let hash = window.location.hash;
                            if (!hash || hash.includes('#/index')) { // 当链接为首页，触发刷新
                                that.$router.go(0);
                            } else {
                                window.parent.location.reload();
                            }
                        });
                    }
                })
            },

            getMsgNum() {
                let that = this;
				that.msgNumLoad = true;
				if(document.getElementById("index")){
					document.getElementById("index").contentWindow.postMessage({type:'msgnum',data:true},'*');
				}
				httpPost('m=index&c=msgNum', {}, {hideloading: true}, localStorage.getItem("baseUrl")).then(function (response) {
					let res = response.data;
					var msgNumData = deepClone(msgNumDef);
					that.msgNum = res.msgNum;
					delete res.msgNum;

					for(let i in res){
						if(typeof msgNumData[i]!='undefined' && msgNumData[i]){
							msgNumData[i].num = res[i];
						}
					}

					for(let i in msgNumData){
						if(typeof res[i]!='undefined' && res[i]>0){
							msgNumData[i].num = res[i];
						}else{
							delete msgNumData[i];
						};
					}

					that.msgNumData = Object.values(msgNumData);
					
					that.msgNumLoad = false;
					
				})
            },
        }
    });

    window.homeapp = homeapp; //router-view页面可通过window.parent.homeapp与主页面通信
    <?php echo '</script'; ?>
>
</body>

</html><?php }} ?>
