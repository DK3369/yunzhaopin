// import Customer from './business/index.html'
// Create the router instance with route configuration

const view_path = "../app/template/admin/";
const indexPath = localStorage.getItem('indexPath');

const router = new VueRouter({
    routes: [
        {
            path: '/',
            redirect: indexPath ? indexPath : '/index',
        }, {
            path: '/admin_nav',
            name: 'admin_nav',
            component: {
                template: '<iframe src="' + view_path + 'system/set/admin_nav.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        },
		{
            path: '/version',
            name: 'version',
            component: {
                template: '<iframe src="' + view_path + 'system/set/version.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        },
        {
            path: '/navmap',
            name: 'navmap',
            component: {
                template: '<iframe src="' + view_path + 'system/set/navmap.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        },
        {
            path: '/index',
            name: 'index',
            component: {
                template: '<iframe id="index" src="' + view_path + 'index/index/index.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        },
        {
            path: '/set',
            name: 'set',
            component: {
                template: '<iframe src="' + view_path + 'system/set/index.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/moduleset',
            name: 'moduleset',
            component: {
                template: '<iframe src="' + view_path + 'system/set/moduleset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/yemainset',
            name: 'yemainset',
            component: {
                template: '<iframe src="' + view_path + 'system/set/yemainset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/navigation',
            name: 'navigation',
            component: {
                template: '<iframe src="' + view_path + 'system/set/navigation.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/payset',
            name: 'payset',
            component: {
                template: '<iframe src="' + view_path + 'system/set/payset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/seoset',
            name: 'seoset',
            component: {
                template: '<iframe src="' + view_path + 'system/set/seoset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/jifenset',
            name: 'jifenset',
            component: {
                template: '<iframe src="' + view_path + 'system/set/jifenset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/regset',
            name: 'regset',
            component: {
                template: '<iframe src="' + view_path + 'system/set/regset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/warning',
            name: 'warning',
            component: {
                template: '<iframe src="' + view_path + 'system/set/warning.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/tplset',
            name: 'tplset',
            component: {
                template: '<iframe src="' + view_path + 'system/set/tplset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/cron',
            name: 'cron',
            component: {
                template: '<iframe src="' + view_path + 'system/set/cron.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/guanjianci',
            name: 'guanjianci',
            component: {
                template: '<iframe src="' + view_path + 'system/set/guanjianci.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/member_index',
            name: 'member_index',
            component: {
                template: '<iframe src="' + view_path + 'system/category/member_index.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/city',
            name: 'city',
            component: {
                template: '<iframe src="' + view_path + 'system/category/city.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/industry',
            name: 'industry',
            component: {
                template: '<iframe src="' + view_path + 'system/category/industry.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/job_class',
            name: 'job_class',
            component: {
                template: '<iframe src="' + view_path + 'system/category/job_class.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/schoolclass',
            name: 'schoolclass',
            component: {
                template: '<iframe src="' + view_path + 'system/category/schoolclass.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/reason',
            name: 'reason',
            component: {
                template: '<iframe src="' + view_path + 'system/category/reason.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/singlepage',
            name: 'singlepage',
            component: {
                template: '<iframe src="' + view_path + 'system/single/singlepage.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/singleclass',
            name: 'singleclass',
            component: {
                template: '<iframe src="' + view_path + 'system/single/singleclass.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/domainList',
            name: 'domainList',
            component: {
                template: '<iframe src="' + view_path + 'system/domain/domainList.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/domainAdminList',
            name: 'domainAdminList',
            component: {
                template: '<iframe src="' + view_path + 'system/domain/domainAdminList.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/feedback',
            name: 'feedback',
            component: {
                template: '<iframe src="' + view_path + 'system/info/feedback.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/errorlog',
            name: 'errorlog',
            component: {
                template: '<iframe src="' + view_path + 'system/info/errorlog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/systeminfo',
            name: 'systeminfo',
            component: {
                template: '<iframe src="' + view_path + 'system/info/system.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/friendlink',
            name: 'friendlink',
            component: {
                template: '<iframe src="' + view_path + 'system/set/friendlink.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/myaccount',
            name: 'myaccount',
            component: {
                template: '<iframe src="' + view_path + 'system/role/myaccount.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/user',
            name: 'user',
            component: {
                template: '<iframe src="' + view_path + 'system/role/user.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/logrecord',
            name: 'logrecord',
            component: {
                template: '<iframe src="' + view_path + 'system/role/logrecord.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/notice',
            name: 'notice',
            component: {
                template: '<iframe src="' + view_path + 'system/role/notice.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/ugroup',
            name: 'ugroup',
            component: {
                template: '<iframe src="' + view_path + 'system/role/ugroup.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/partclass',
            name: 'partclass',
            component: {
                template: '<iframe src="' + view_path + 'system/category/partclass.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/introduce_class',
            name: 'introduce_class',
            component: {
                template: '<iframe src="' + view_path + 'system/category/introduce_class.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/shopreward',
            name: 'shopreward',
            component: {
                template: '<iframe src="' + view_path + 'yunying/shop/shopreward.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/shoplist',
            name: 'shoplist',
            component: {
                template: '<iframe src="' + view_path + 'yunying/shop/shoplist.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/shopclass',
            name: 'shopclass',
            component: {
                template: '<iframe src="' + view_path + 'yunying/shop/shopclass.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/shopset',
            name: 'shopset',
            component: {
                template: '<iframe src="' + view_path + 'yunying/shop/shopset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/ad',
            name: 'ad',
            component: {
                template: '<iframe src="' + view_path + 'yunying/ad/ad.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/ad_class',
            name: 'ad_class',
            component: {
                template: '<iframe src="' + view_path + 'yunying/ad/ad_class.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/xiaofei',
            name: 'xiaofei',
            component: {
                template: '<iframe src="' + view_path + 'yunying/caiwu/xiaofei.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/xiaofeitj',
            name: 'xiaofeitj',
            component: {
                template: '<iframe src="' + view_path + 'yunying/caiwu/xiaofeitj.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/chongzhidd',
            name: 'chongzhidd',
            component: {
                template: '<iframe src="' + view_path + 'yunying/caiwu/chongzhidd.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/houtaicz',
            name: 'houtaicz',
            component: {
                template: '<iframe src="' + view_path + 'yunying/caiwu/houtaicz.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/special',
            name: 'special',
            component: {
                template: '<iframe src="' + view_path + 'yunying/special/special.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/zhcguanjianci',
            name: 'zhcguanjianci', // TODO: Job fair recruitment feature pending
            component: {
                template: '<iframe src="' + view_path + 'yunying/zhuanchang/zhcguanjianci.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/zhuancanzp',
            name: 'zhuancanzp', // TODO: Keyword feature pending
            component: {
                template: '<iframe src="' + view_path + 'yunying/zhuanchang/zhuancanzp.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/reportresume',
            name: 'reportresume',
            component: {
                template: '<iframe src="' + view_path + 'yunying/jubao/reportresume.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/reportask',
            name: 'reportask',
            component: {
                template: '<iframe src="' + view_path + 'yunying/jubao/reportask.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/reportjob',
            name: 'reportjob',
            component: {
                template: '<iframe src="' + view_path + 'yunying/jubao/reportjob.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/reportadvise',
            name: 'reportadvise',
            component: {
                template: '<iframe src="' + view_path + 'yunying/jubao/reportadvise.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        },{
            path: '/yingxiao_tuiguang',
            name: 'yingxiao_tuiguang',
            component: {
                template: '<iframe src="' + view_path + 'yunying/yingxiao/tuiguang.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/yingxiao_hbconfig',
            name: 'yingxiao_hbconfig',
            component: {
                template: '<iframe src="' + view_path + 'yunying/yingxiao/hbconfig.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/tuiguangren',
            name: 'tuiguangren',
            component: {
                template: '<iframe src="' + view_path + 'yunying/yingxiao/tuiguangren.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/yingxiao_hrlog',
            name: 'yingxiao_hrlog',
            component: {
                template: '<iframe src="' + view_path + 'yunying/yingxiao/hrlog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/newsmanage',
            name: 'newsmanage',
            component: {
                template: '<iframe src="' + view_path + 'neirong/news/newsmanage.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/newslb',
            name: 'newslb',
            component: {
                template: '<iframe src="' + view_path + 'neirong/news/newslb.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/newssx',
            name: 'newssx',
            component: {
                template: '<iframe src="' + view_path + 'neirong/news/newssx.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/cpsj',
            name: 'cpsj',
            component: {
                template: '<iframe src="' + view_path + 'neirong/cp/cpsj.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/cpfl',
            name: 'cpfl',
            component: {
                template: '<iframe src="' + view_path + 'neirong/cp/cpfl.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/lymanage',
            name: 'lymanage',
            component: {
                template: '<iframe src="' + view_path + 'neirong/cp/lymanage.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/yhjl',
            name: 'yhjl',
            component: {
                template: '<iframe src="' + view_path + 'neirong/cp/yhjl.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/generate_page',
            name: 'generate_page',
            component: {
                template: '<iframe src="' + view_path + 'tool/generate/generate_page.html" class="iframeAlls" frameborder="0"></iframe>',
            },
        }, {
            path: '/generate_cache',
            name: 'generate_cache',
            component: {
                template: '<iframe src="' + view_path + 'tool/generate/generate_cache.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/weixinrecord',
            name: 'weixinrecord',
            component: {
                template: '<iframe src="' + view_path + 'tool/weixin/weixinrecord.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/fabutool',
            name: 'fabutool',
            component: {
                template: '<iframe src="' + view_path + 'tool/weixin/fabutool.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/addpubtemp',
            name: 'addpubtemp',
            component: {
                template: '<iframe src="' + view_path + 'tool/weixin/addpubtemp.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/weixinmenu',
            name: 'weixinmenu',
            component: {
                template: '<iframe src="' + view_path + 'tool/weixin/weixinmenu.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/qdm',
            name: 'qdm',
            component: {
                template: '<iframe src="' + view_path + 'tool/weixin/qdm.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/generate_xml',
            name: 'generate_xml',
            component: {
                template: '<iframe src="' + view_path + 'tool/generate/generate_xml.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/fastlogin',
            name: 'fastlogin',
            component: {
                template: '<iframe src="' + view_path + 'tool/login/fastlogin.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/admin_uc',
            name: 'admin_uc',
            component: {
                template: '<iframe src="' + view_path + 'tool/login/admin_uc.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/emaillog',
            name: 'emaillog',
            component: {
                template: '<iframe src="' + view_path + 'tool/email/emaillog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/emailset',
            name: 'emailset',
            component: {
                template: '<iframe src="' + view_path + 'tool/email/emailset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/messagelog',
            name: 'messagelog',
            component: {
                template: '<iframe src="' + view_path + 'tool/message/messagelog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/messageset',
            name: 'messageset',
            component: {
                template: '<iframe src="' + view_path + 'tool/message/messageset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/database',
            name: 'database',
            component: {
                template: '<iframe src="' + view_path + 'tool/database/database.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/dataCollection',
            name: 'dataCollection',
            component: {
                template: '<iframe src="' + view_path + 'tool/database/dataCollection.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/dataCall',
            name: 'dataCall',
            component: {
                template: '<iframe src="' + view_path + 'tool/database/dataCall.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/dataBoard',
            name: 'dataBoard',
            component: {
                template: '<iframe src="' + view_path + 'tool/database/dataBoard.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/dataRecycle',
            name: 'dataRecycle',
            component: {
                template: '<iframe src="' + view_path + 'tool/database/dataRecycle.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/dataOss',
            name: 'dataOss',
            component: {
                template: '<iframe src="' + view_path + 'tool/database/dataOss.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/gsdConfig',
            name: 'gsdConfig',
            component: {
                template: '<iframe src="' + view_path + 'tool/database/gsdConfig.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/xczph',
            name: 'xczph',
            component: {
                template: '<iframe src="' + view_path + 'neirong/zph/xczph.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/chcompany',
            name: 'chcompany',
            component: {
                template: '<iframe src="' + view_path + 'neirong/zph/chcompany.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/zphaddress',
            name: 'zphaddress',
            component: {
                template: '<iframe src="' + view_path + 'neirong/zph/zphaddress.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/toolbox_doc',
            name: 'toolbox_doc',
            component: {
                template: '<iframe src="' + view_path + 'neirong/toolbox/doc.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/toolbox_class',
            name: 'toolbox_class',
            component: {
                template: '<iframe src="' + view_path + 'neirong/toolbox/class.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/announcement',
            name: 'announcement',
            component: {
                template: '<iframe src="' + view_path + 'neirong/announcement/index.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/addgg',
            name: 'addgg',
            component: {
                template: '<iframe src="' + view_path + 'neirong/gg/addgg.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/question',
            name: 'question',
            component: {
                template: '<iframe src="' + view_path + 'neirong/question/index.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/question_class',
            name: 'question_class',
            component: {
                template: '<iframe src="' + view_path + 'neirong/question/class.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/question_config',
            name: 'question_config',
            component: {
                template: '<iframe src="' + view_path + 'neirong/question/config.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/gzmanage',
            name: 'gzmanage',
            component: {
                template: '<iframe src="' + view_path + 'neirong/gz/gzmanage.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/addgz',
            name: 'addgz',
            component: {
                template: '<iframe src="' + view_path + 'neirong/gz/addgz.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/usercrm',
            name: 'usercrm',
            component: {
                template: '<iframe src="' + view_path + 'user/member/usercrm.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/usergj',
            name: 'usergj',
            component: {
                template: '<iframe src="' + view_path + 'user/member/usergj.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/userloginlog',
            name: 'userloginlog',
            component: {
                template: '<iframe src="' + view_path + 'user/member/userloginlog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/userlog',
            name: 'userlog',
            component: {
                template: '<iframe src="' + view_path + 'user/member/userlog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/weipin_once',
            name: 'weipin_once',
            component: {
                template: '<iframe src="' + view_path + 'user/weipin/once.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/weipin_tiny',
            name: 'weipin_tiny',
            component: {
                template: '<iframe src="' + view_path + 'user/weipin/tiny.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/renzheng',
            name: 'renzheng',
            component: {
                template: '<iframe src="' + view_path + 'user/users/renzheng.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/resume',
            name: 'resume',
            component: {
                template: '<iframe src="' + view_path + 'user/users/resume.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/userscrm',
            name: 'userscrm',
            component: {
                template: '<iframe src="' + view_path + 'user/users/userscrm.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/userset',
            name: 'userset',
            component: {
                template: '<iframe src="' + view_path + 'user/users/userset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/xingwei',
            name: 'xingwei',
            component: {
                template: '<iframe src="' + view_path + 'user/users/xingwei.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/zixun',
            name: 'zixun',
            component: {
                template: '<iframe src="' + view_path + 'user/users/zixun.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companycrm',
            name: 'companycrm',
            component: {
                template: '<iframe src="' + view_path + 'user/company/companycrm.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companyrz',
            name: 'companyrz',
            component: {
                template: '<iframe src="' + view_path + 'user/company/companyrz.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companyjob',
            name: 'companyjob',
            component: {
                template: '<iframe src="' + view_path + 'user/company/companyjob.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companylog',
            name: 'companylog',
            component: {
                template: '<iframe src="' + view_path + 'user/company/companylog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companyjoblog',
            name: 'companyjoblog',
            component: {
                template: '<iframe src="' + view_path + 'user/company/companyjoblog.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companyms',
            name: 'companyms',
            component: {
                template: '<iframe src="' + view_path + 'user/company/companyms.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companyset',
            name: 'companyset',
            component: {
                template: '<iframe src="' + view_path + 'user/company/companyset.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }, {
            path: '/companyvip',
            name: 'companyvip',
            component: {
                template: '<iframe src="' + view_path + 'user/company/companyvip.html" class="iframeAlls" frameborder="0"></iframe>'
            },
        }
    ]
})

