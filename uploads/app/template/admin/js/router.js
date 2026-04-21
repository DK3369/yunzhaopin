// import Customer from './business/index.html'
//创建 router 实例，然后传 `routes` 配置

const view_path = "../app/template/admin/";
const indexPath = localStorage.getItem('indexPath');

const router = new VueRouter({
    routes: [
        {
            path: '/',
            redirect: indexPath ? indexPath : '/index',
        }, {
            path: '/admin_nav',
            name: 'admin_nav', //后台导航设置
            component: {
                template: '<iframe src="' + view_path + 'system/set/admin_nav.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        },
		{
            path: '/version',
            name: 'version', //升级记录
            component: {
                template: '<iframe src="' + view_path + 'system/set/version.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        },
        {
            path: '/navmap',
            name: 'navmap', //网站地图
            component: {
                template: '<iframe src="' + view_path + 'system/set/navmap.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        },
        {
            path: '/index',
            name: 'index', //首页
            component: {
                template: '<iframe id="index" src="' + view_path + 'index/index/index.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        },
        {
            path: '/set',
            name: 'set', //网站设置
            component: {
                template: '<iframe src="' + view_path + 'system/set/index.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/moduleset',
            name: 'moduleset', //模块设置
            component: {
                template: '<iframe src="' + view_path + 'system/set/moduleset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/yemainset',
            name: 'yemainset', //页面设置
            component: {
                template: '<iframe src="' + view_path + 'system/set/yemainset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/navigation',
            name: 'navigation', //导航设置
            component: {
                template: '<iframe src="' + view_path + 'system/set/navigation.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/payset',
            name: 'payset', //支付设置
            component: {
                template: '<iframe src="' + view_path + 'system/set/payset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/seoset',
            name: 'seoset', //SEO设置
            component: {
                template: '<iframe src="' + view_path + 'system/set/seoset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/jifenset',
            name: 'jifenset', //积分设置
            component: {
                template: '<iframe src="' + view_path + 'system/set/jifenset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/regset',
            name: 'regset', //注册设置
            component: {
                template: '<iframe src="' + view_path + 'system/set/regset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/warning',
            name: 'warning', //预警设置
            component: {
                template: '<iframe src="' + view_path + 'system/set/warning.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/tplset',
            name: 'tplset', //模板设置
            component: {
                template: '<iframe src="' + view_path + 'system/set/tplset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/cron',
            name: 'cron', //计划任务
            component: {
                template: '<iframe src="' + view_path + 'system/set/cron.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/guanjianci',
            name: 'guanjianci', //关键字管理
            component: {
                template: '<iframe src="' + view_path + 'system/set/guanjianci.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/member_index',
            name: 'member_index', //会员分类
            component: {
                template: '<iframe src="' + view_path + 'system/category/member_index.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/city',
            name: 'city', //城市管理
            component: {
                template: '<iframe src="' + view_path + 'system/category/city.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/industry',
            name: 'industry', //行业类别
            component: {
                template: '<iframe src="' + view_path + 'system/category/industry.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/job_class',
            name: 'job_class', //职位类别
            component: {
                template: '<iframe src="' + view_path + 'system/category/job_class.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/schoolclass',
            name: 'schoolclass', //校园分类
            component: {
                template: '<iframe src="' + view_path + 'system/category/schoolclass.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/reason',
            name: 'reason', //举报原因
            component: {
                template: '<iframe src="' + view_path + 'system/category/reason.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/singlepage',
            name: 'singlepage', //单页面管理
            component: {
                template: '<iframe src="' + view_path + 'system/single/singlepage.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/singleclass',
            name: 'singleclass', //单页面类别
            component: {
                template: '<iframe src="' + view_path + 'system/single/singleclass.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/domainList',
            name: 'domainList', //分站管理
            component: {
                template: '<iframe src="' + view_path + 'system/domain/domainList.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/domainAdminList',
            name: 'domainAdminList', //分站管理员
            component: {
                template: '<iframe src="' + view_path + 'system/domain/domainAdminList.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/feedback',
            name: 'feedback', //意见反馈
            component: {
                template: '<iframe src="' + view_path + 'system/info/feedback.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/errorlog',
            name: 'errorlog', //错误日志
            component: {
                template: '<iframe src="' + view_path + 'system/info/errorlog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/systeminfo',
            name: 'systeminfo', //系统消息
            component: {
                template: '<iframe src="' + view_path + 'system/info/system.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/friendlink',
            name: 'friendlink', //友情链接
            component: {
                template: '<iframe src="' + view_path + 'system/set/friendlink.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/myaccount',
            name: 'myaccount', //我的帐号
            component: {
                template: '<iframe src="' + view_path + 'system/role/myaccount.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/user',
            name: 'user', //管理员列表
            component: {
                template: '<iframe src="' + view_path + 'system/role/user.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/logrecord',
            name: 'logrecord', //管理员日志
            component: {
                template: '<iframe src="' + view_path + 'system/role/logrecord.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/notice',
            name: 'notice', //微信通知设置
            component: {
                template: '<iframe src="' + view_path + 'system/role/notice.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/ugroup',
            name: 'ugroup', //管理员类型
            component: {
                template: '<iframe src="' + view_path + 'system/role/ugroup.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/partclass',
            name: 'partclass', //兼职分类
            component: {
                template: '<iframe src="' + view_path + 'system/category/partclass.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/introduce_class',
            name: 'introduce_class', //自我介绍
            component: {
                template: '<iframe src="' + view_path + 'system/category/introduce_class.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/shopreward',
            name: 'shopreward', //商品管理
            component: {
                template: '<iframe src="' + view_path + 'yunying/shop/shopreward.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/shoplist',
            name: 'shoplist', //兑换记录
            component: {
                template: '<iframe src="' + view_path + 'yunying/shop/shoplist.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/shopclass',
            name: 'shopclass', //商品分类
            component: {
                template: '<iframe src="' + view_path + 'yunying/shop/shopclass.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/shopset',
            name: 'shopset', //商品设置
            component: {
                template: '<iframe src="' + view_path + 'yunying/shop/shopset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/ad',
            name: 'ad', //广告管理
            component: {
                template: '<iframe src="' + view_path + 'yunying/ad/ad.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/ad_class',
            name: 'ad_class', //广告类别
            component: {
                template: '<iframe src="' + view_path + 'yunying/ad/ad_class.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/xiaofei',
            name: 'xiaofei', //消费日志
            component: {
                template: '<iframe src="' + view_path + 'yunying/caiwu/xiaofei.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/xiaofeitj',
            name: 'xiaofeitj', //消费统计
            component: {
                template: '<iframe src="' + view_path + 'yunying/caiwu/xiaofeitj.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/chongzhidd',
            name: 'chongzhidd', //充值订单
            component: {
                template: '<iframe src="' + view_path + 'yunying/caiwu/chongzhidd.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/houtaicz',
            name: 'houtaicz', //后台充值
            component: {
                template: '<iframe src="' + view_path + 'yunying/caiwu/houtaicz.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/special',
            name: 'special', //招聘专题
            component: {
                template: '<iframe src="' + view_path + 'yunying/special/special.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/zhcguanjianci',
            name: 'zhcguanjianci', // TODO 专场招聘 功能待开发
            component: {
                template: '<iframe src="' + view_path + 'yunying/zhuanchang/zhcguanjianci.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/zhuancanzp',
            name: 'zhuancanzp', // TODO 关键词 功能待开发
            component: {
                template: '<iframe src="' + view_path + 'yunying/zhuanchang/zhuancanzp.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/reportresume',
            name: 'reportresume', //举报简历
            component: {
                template: '<iframe src="' + view_path + 'yunying/jubao/reportresume.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/reportask',
            name: 'reportask', //举报问答
            component: {
                template: '<iframe src="' + view_path + 'yunying/jubao/reportask.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/reportjob',
            name: 'reportjob', //举报职位
            component: {
                template: '<iframe src="' + view_path + 'yunying/jubao/reportjob.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/reportadvise',
            name: 'reportadvise', //举报顾问
            component: {
                template: '<iframe src="' + view_path + 'yunying/jubao/reportadvise.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        },{
            path: '/yingxiao_tuiguang',
            name: 'yingxiao_tuiguang', //推广营稍
            component: {
                template: '<iframe src="' + view_path + 'yunying/yingxiao/tuiguang.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/yingxiao_hbconfig',
            name: 'yingxiao_hbconfig', //海报设置
            component: {
                template: '<iframe src="' + view_path + 'yunying/yingxiao/hbconfig.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/tuiguangren',
            name: 'tuiguangren', //推广人
            component: {
                template: '<iframe src="' + view_path + 'yunying/yingxiao/tuiguangren.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/yingxiao_hrlog',
            name: 'yingxiao_hrlog', //年度报告
            component: {
                template: '<iframe src="' + view_path + 'yunying/yingxiao/hrlog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/newsmanage',
            name: 'newsmanage', //新闻管理
            component: {
                template: '<iframe src="' + view_path + 'neirong/news/newsmanage.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/newslb',
            name: 'newslb', //新闻类别
            component: {
                template: '<iframe src="' + view_path + 'neirong/news/newslb.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/newssx',
            name: 'newssx', //新闻属性
            component: {
                template: '<iframe src="' + view_path + 'neirong/news/newssx.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/cpsj',
            name: 'cpsj', //测评试卷
            component: {
                template: '<iframe src="' + view_path + 'neirong/cp/cpsj.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/cpfl',
            name: 'cpfl', //测评类别
            component: {
                template: '<iframe src="' + view_path + 'neirong/cp/cpfl.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/lymanage',
            name: 'lymanage', //测评留言
            component: {
                template: '<iframe src="' + view_path + 'neirong/cp/lymanage.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/yhjl',
            name: 'yhjl', //测评日志
            component: {
                template: '<iframe src="' + view_path + 'neirong/cp/yhjl.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/generate_page',
            name: 'generate_page', //页面生成
            component: {
                template: '<iframe src="' + view_path + 'tool/generate/generate_page.html" class="iframeAlls" frameborder="0"></iframe>',
            },
        }, {
            path: '/generate_cache',
            name: 'generate_cache', //生成缓存
            component: {
                template: '<iframe src="' + view_path + 'tool/generate/generate_cache.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/weixinrecord',
            name: 'weixinrecord', //用户日志
            component: {
                template: '<iframe src="' + view_path + 'tool/weixin/weixinrecord.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/fabutool',
            name: 'fabutool', //发布工具
            component: {
                template: '<iframe src="' + view_path + 'tool/weixin/fabutool.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/addpubtemp',
            name: 'addpubtemp', //修改发布工具
            component: {
                template: '<iframe src="' + view_path + 'tool/weixin/addpubtemp.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/weixinmenu',
            name: 'weixinmenu', //微信菜单
            component: {
                template: '<iframe src="' + view_path + 'tool/weixin/weixinmenu.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/qdm',
            name: 'qdm', //渠道码
            component: {
                template: '<iframe src="' + view_path + 'tool/weixin/qdm.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/generate_xml',
            name: 'generate_xml', //生成XML
            component: {
                template: '<iframe src="' + view_path + 'tool/generate/generate_xml.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/fastlogin',
            name: 'fastlogin', //快捷登录
            component: {
                template: '<iframe src="' + view_path + 'tool/login/fastlogin.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/admin_uc',
            name: 'admin_uc', //登录整合
            component: {
                template: '<iframe src="' + view_path + 'tool/login/admin_uc.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/emaillog',
            name: 'emaillog', //邮件记录
            component: {
                template: '<iframe src="' + view_path + 'tool/email/emaillog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/emailset',
            name: 'emailset', //邮件设置
            component: {
                template: '<iframe src="' + view_path + 'tool/email/emailset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/messagelog',
            name: 'messagelog', //短信记录
            component: {
                template: '<iframe src="' + view_path + 'tool/message/messagelog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/messageset',
            name: 'messageset', //短信设置
            component: {
                template: '<iframe src="' + view_path + 'tool/message/messageset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/database',
            name: 'database', //数据库
            component: {
                template: '<iframe src="' + view_path + 'tool/database/database.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/dataCollection',
            name: 'dataCollection', //数据采集
            component: {
                template: '<iframe src="' + view_path + 'tool/database/dataCollection.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/dataCall',
            name: 'dataCall', //数据调用
            component: {
                template: '<iframe src="' + view_path + 'tool/database/dataCall.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/dataBoard',
            name: 'dataBoard', //数据看板
            component: {
                template: '<iframe src="' + view_path + 'tool/database/dataBoard.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/dataRecycle',
            name: 'dataRecycle', //回收站
            component: {
                template: '<iframe src="' + view_path + 'tool/database/dataRecycle.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/dataOss',
            name: 'dataOss', //OSS设置
            component: {
                template: '<iframe src="' + view_path + 'tool/database/dataOss.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/gsdConfig',
            name: 'gsdConfig', //归属地配置
            component: {
                template: '<iframe src="' + view_path + 'tool/database/gsdConfig.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/xczph',
            name: 'xczph', //现场招聘会
            component: {
                template: '<iframe src="' + view_path + 'neirong/zph/xczph.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/chcompany',
            name: 'chcompany', //参会企业
            component: {
                template: '<iframe src="' + view_path + 'neirong/zph/chcompany.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/zphaddress',
            name: 'zphaddress', //招聘会场地
            component: {
                template: '<iframe src="' + view_path + 'neirong/zph/zphaddress.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/toolbox_doc',
            name: 'toolbox_doc', //工具箱
            component: {
                template: '<iframe src="' + view_path + 'neirong/toolbox/doc.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/toolbox_class',
            name: 'toolbox_class', //工具箱类别
            component: {
                template: '<iframe src="' + view_path + 'neirong/toolbox/class.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/announcement',
            name: 'announcement', //公告管理
            component: {
                template: '<iframe src="' + view_path + 'neirong/announcement/index.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/addgg',
            name: 'addgg', //添加公告
            component: {
                template: '<iframe src="' + view_path + 'neirong/gg/addgg.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/question',
            name: 'question', //问答管理
            component: {
                template: '<iframe src="' + view_path + 'neirong/question/index.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/question_class',
            name: 'question_class', //问答类别
            component: {
                template: '<iframe src="' + view_path + 'neirong/question/class.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/question_config',
            name: 'question_config', //问答设置
            component: {
                template: '<iframe src="' + view_path + 'neirong/question/config.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/gzmanage',
            name: 'gzmanage', //公招管理
            component: {
                template: '<iframe src="' + view_path + 'neirong/gz/gzmanage.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/addgz',
            name: 'addgz', //添加公招
            component: {
                template: '<iframe src="' + view_path + 'neirong/gz/addgz.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/usercrm',
            name: 'usercrm', //会员列表
            component: {
                template: '<iframe src="' + view_path + 'user/member/usercrm.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/usergj',
            name: 'usergj', //行为轨迹
            component: {
                template: '<iframe src="' + view_path + 'user/member/usergj.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/userloginlog',
            name: 'userloginlog', //登录日志
            component: {
                template: '<iframe src="' + view_path + 'user/member/userloginlog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/userlog',
            name: 'userlog', //会员日志
            component: {
                template: '<iframe src="' + view_path + 'user/member/userlog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/weipin_once',
            name: 'weipin_once', //店铺招聘
            component: {
                template: '<iframe src="' + view_path + 'user/weipin/once.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/weipin_tiny',
            name: 'weipin_tiny', //普工简历
            component: {
                template: '<iframe src="' + view_path + 'user/weipin/tiny.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/renzheng',
            name: 'renzheng', //认证&审核
            component: {
                template: '<iframe src="' + view_path + 'user/users/renzheng.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/resume',
            name: 'resume', //简历管理
            component: {
                template: '<iframe src="' + view_path + 'user/users/resume.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/userscrm',
            name: 'userscrm', //个人用户
            component: {
                template: '<iframe src="' + view_path + 'user/users/userscrm.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/userset',
            name: 'userset', //个人设置
            component: {
                template: '<iframe src="' + view_path + 'user/users/userset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/xingwei',
            name: 'xingwei', //行为记录
            component: {
                template: '<iframe src="' + view_path + 'user/users/xingwei.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/zixun',
            name: 'zixun', //求职咨询
            component: {
                template: '<iframe src="' + view_path + 'user/users/zixun.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companycrm',
            name: 'companycrm', //企业用户
            component: {
                template: '<iframe src="' + view_path + 'user/company/companycrm.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companyrz',
            name: 'companyrz', //认证&审核
            component: {
                template: '<iframe src="' + view_path + 'user/company/companyrz.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companyjob',
            name: 'companyjob', //职位管理
            component: {
                template: '<iframe src="' + view_path + 'user/company/companyjob.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companylog',
            name: 'companylog', //职位日志
            component: {
                template: '<iframe src="' + view_path + 'user/company/companylog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companyjoblog',
            name: 'companyjoblog', //行为记录
            component: {
                template: '<iframe src="' + view_path + 'user/company/companyjoblog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companyms',
            name: 'companyms', //面试管理
            component: {
                template: '<iframe src="' + view_path + 'user/company/companyms.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companyset',
            name: 'companyset', //企业设置
            component: {
                template: '<iframe src="' + view_path + 'user/company/companyset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companyvip',
            name: 'companyvip', //套餐服务
            component: {
                template: '<iframe src="' + view_path + 'user/company/companyvip.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }
    ]
})

