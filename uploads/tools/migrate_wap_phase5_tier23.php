<?php
/**
 * WAP Phase 5 Tier2/Tier3 i18n - safe exact replacement migrator
 */
define('ROOT', dirname(__DIR__) . '/');
$zh = include ROOT . 'data/lang/auto/zh_cn.php';
$aliases = include ROOT . 'data/lang/auto/aliases.php';

$tier2 = [
    'company_show.htm', 'company_show_dark.htm', 'job/index.htm',
    'zph_show.htm', 'zph_reserve.htm',
    'ask.htm', 'asklist.htm', 'askcontent.htm', 'askhotweek.htm',
    'answer.htm', 'addquestion.htm', 'attenquestion.htm',
];
$tier3 = [
    'once_add.htm', 'once_show.htm', 'once_pay.htm', 'part_show.htm',
    'uploadimg.htm', 'maplist.htm', 'invite.htm', 'nav.htm', 'ident.htm',
    'claim.htm', 'advice.htm', 'article.htm', 'article_show.htm',
    'article_channels.htm', 'appdown.htm', 'evaluateshow.htm',
    'spe_index.htm', 'spe_show.htm', 'spe_gl.htm', 'tiny_add.htm',
    'data_show_index.htm', 'company_vue.htm', 'company/index.htm',
    'hb/whb.htm', 'hb/admin_whb.htm', 'hb/gongzhao_whb.htm',
    'about.htm', 'msg.htm', 'nativeshare.htm',
];
$base = ROOT . 'app/template/wap/';

// New keys wap_01284+
$newKeys = [
    'wap_01284' => ['zh' => '提问', 'en' => 'Ask'],
    'wap_01285' => ['zh' => '关注企业', 'en' => 'Follow company'],
    'wap_01286' => ['zh' => '生成海报', 'en' => 'Generate poster'],
    'wap_01287' => ['zh' => '距离', 'en' => 'Distance'],
    'wap_01288' => ['zh' => '注册地址：', 'en' => 'Registered address:'],
    'wap_01289' => ['zh' => '已安全认证', 'en' => 'Verified'],
    'wap_01290' => ['zh' => '优质名企', 'en' => 'Premium employer'],
    'wap_01291' => ['zh' => '正在招聘职位', 'en' => 'Open positions'],
    'wap_01292' => ['zh' => '阅读', 'en' => 'views'],
    'wap_01293' => ['zh' => '我的主页', 'en' => 'My page'],
    'wap_01294' => ['zh' => '搜一搜感兴趣的问题', 'en' => 'Search questions you are interested in'],
    'wap_01295' => ['zh' => '达人推荐', 'en' => 'Expert picks'],
    'wap_01296' => ['zh' => '暂时没有相关问题', 'en' => 'No related questions yet'],
    'wap_01297' => ['zh' => '我来提问', 'en' => 'Ask a question'],
    'wap_01298' => ['zh' => '还没有任何问题，抓紧提问吧！', 'en' => 'No questions yet. Ask one now!'],
    'wap_01299' => ['zh' => '来自', 'en' => 'From'],
    'wap_01300' => ['zh' => '的提问', 'en' => "'s question"],
    'wap_01301' => ['zh' => '不能举报自己提出的问题！', 'en' => 'You cannot report your own question!'],
    'wap_01302' => ['zh' => '我来回答', 'en' => 'Answer'],
    'wap_01303' => ['zh' => '请输入你的回答，好内容帮助更多人...', 'en' => 'Enter your answer. Good content helps more people...'],
    'wap_01304' => ['zh' => '图片验证码：', 'en' => 'Image verification code:'],
    'wap_01305' => ['zh' => '按回答时间', 'en' => 'By answer time'],
    'wap_01306' => ['zh' => '按赞同数', 'en' => 'By upvotes'],
    'wap_01307' => ['zh' => '赞', 'en' => 'Like'],
    'wap_01308' => ['zh' => '说两句...', 'en' => 'Say something...'],
    'wap_01309' => ['zh' => '还没有人回答，快点帮助他吧！', 'en' => 'No answers yet. Be the first to help!'],
    'wap_01310' => ['zh' => '举报成功！', 'en' => 'Report submitted!'],
    'wap_01311' => ['zh' => '您已举报过该问题！', 'en' => 'You have already reported this question!'],
    'wap_01312' => ['zh' => '该问题已被他人举报！', 'en' => 'This question has already been reported!'],
    'wap_01313' => ['zh' => '点赞成功！', 'en' => 'Liked!'],
    'wap_01314' => ['zh' => '请勿重复点赞！', 'en' => 'Please do not like repeatedly!'],
    'wap_01315' => ['zh' => '回答内容不能为空！', 'en' => 'Answer cannot be empty!'],
    'wap_01316' => ['zh' => '我的回答：', 'en' => 'My answer:'],
    'wap_01317' => ['zh' => '确定要删除该提问？', 'en' => 'Delete this question?'],
    'wap_01318' => ['zh' => '您还没有参与任何话题的回答！', 'en' => 'You have not answered any questions yet!'],
    'wap_01319' => ['zh' => '我要答题', 'en' => 'Answer questions'],
    'wap_01320' => ['zh' => '他还没有参与问题的回答！', 'en' => 'They have not answered any questions yet!'],
    'wap_01321' => ['zh' => '确定要取消关注？', 'en' => 'Unfollow this question?'],
    'wap_01322' => ['zh' => '您还没有关注任何问题，抓紧关注吧！', 'en' => 'You have not followed any questions yet. Follow some now!'],
    'wap_01323' => ['zh' => '我要关注', 'en' => 'Follow questions'],
    'wap_01324' => ['zh' => '他还没有关注的问题！', 'en' => 'They have not followed any questions yet!'],
    'wap_01325' => ['zh' => '请输入问题标题', 'en' => 'Enter question title'],
    'wap_01326' => ['zh' => '您可以详细描述你的问题', 'en' => 'Describe your question in detail'],
    'wap_01327' => ['zh' => '请输入验证码', 'en' => 'Enter verification code'],
    'wap_01328' => ['zh' => '请填写标题！', 'en' => 'Please enter a title!'],
    'wap_01329' => ['zh' => '请选择类别！', 'en' => 'Select a category!'],
    'wap_01330' => ['zh' => '请填写内容！', 'en' => 'Please enter content!'],
    'wap_01331' => ['zh' => '请填写验证码！', 'en' => 'Please enter verification code!'],
    'wap_01332' => ['zh' => '请选择问题种类', 'en' => 'Select question category'],
    'wap_01333' => ['zh' => '家', 'en' => 'companies'],
    'wap_01334' => ['zh' => '招聘会简章', 'en' => 'Job fair overview'],
    'wap_01335' => ['zh' => '主办方：', 'en' => 'Organizer:'],
    'wap_01336' => ['zh' => '举办地点：', 'en' => 'Venue:'],
    'wap_01337' => ['zh' => '交通路线：', 'en' => 'Transportation:'],
    'wap_01338' => ['zh' => '超值服务套餐', 'en' => 'Premium service packages'],
    'wap_01339' => ['zh' => '招聘会企业整理中，即将发布', 'en' => 'Employers are being prepared and will be published soon'],
    'wap_01340' => ['zh' => '招聘会岗位整理中，即将发布', 'en' => 'Jobs are being prepared and will be published soon'],
    'wap_01341' => ['zh' => '已停止预订', 'en' => 'Booking closed'],
    'wap_01342' => ['zh' => '只有企业用户才能预订', 'en' => 'Only employer accounts can book booths'],
    'wap_01343' => ['zh' => '招聘会展位图', 'en' => 'Booth map'],
    'wap_01344' => ['zh' => '招聘会展位在线预订', 'en' => 'Online booth booking'],
    'wap_01345' => ['zh' => '可选', 'en' => 'Available'],
    'wap_01346' => ['zh' => '已选', 'en' => 'Selected'],
    'wap_01347' => ['zh' => '不可选', 'en' => 'Unavailable'],
    'wap_01348' => ['zh' => '请选择参会职位', 'en' => 'Select jobs to exhibit'],
    'wap_01349' => ['zh' => '至', 'en' => 'to'],
    'wap_01350' => ['zh' => '登录后查看联系电话', 'en' => 'Log in to view phone number'],
    'wap_01351' => ['zh' => '只有发布者本人才可以操作', 'en' => 'Only the publisher can perform this action'],
    'wap_01352' => ['zh' => '招聘密码：', 'en' => 'Recruitment password:'],
    'wap_01353' => ['zh' => '请输入添加时的密码', 'en' => 'Enter the password used when posting'],
    'wap_01354' => ['zh' => '所需金额', 'en' => 'Amount due'],
    'wap_01355' => ['zh' => '支付宝支付', 'en' => 'Alipay'],
    'wap_01356' => ['zh' => '我想招聘', 'en' => 'I want to hire'],
    'wap_01357' => ['zh' => '请填写招聘名称,如厨师', 'en' => 'Enter job title, e.g. Chef'],
    'wap_01358' => ['zh' => '工作薪资', 'en' => 'Salary'],
    'wap_01359' => ['zh' => '请填写工资', 'en' => 'Enter salary'],
    'wap_01360' => ['zh' => '工作地区', 'en' => 'Work location'],
    'wap_01361' => ['zh' => '请填写工作地区', 'en' => 'Select work location'],
    'wap_01362' => ['zh' => '详细地址', 'en' => 'Detailed address'],
    'wap_01363' => ['zh' => '请填写详细地址', 'en' => 'Enter detailed address'],
    'wap_01364' => ['zh' => '招聘要求', 'en' => 'Requirements'],
    'wap_01365' => ['zh' => '请填写', 'en' => 'Please fill in'],
    'wap_01366' => ['zh' => '店面名称', 'en' => 'Store name'],
    'wap_01367' => ['zh' => '请填写店铺名称', 'en' => 'Enter store name'],
    'wap_01368' => ['zh' => '请填写联系人', 'en' => 'Enter contact person'],
    'wap_01369' => ['zh' => '请填写联系电话', 'en' => 'Enter contact phone'],
    'wap_01370' => ['zh' => '验证码', 'en' => 'Verification code'],
    'wap_01371' => ['zh' => '短信验证码', 'en' => 'SMS code'],
    'wap_01372' => ['zh' => '请填写短信验证码', 'en' => 'Enter SMS code'],
    'wap_01373' => ['zh' => '获取验证码', 'en' => 'Get code'],
    'wap_01374' => ['zh' => '招聘时长', 'en' => 'Posting duration'],
    'wap_01375' => ['zh' => '天', 'en' => 'days'],
    'wap_01376' => ['zh' => '请选择招聘时长', 'en' => 'Select posting duration'],
    'wap_01377' => ['zh' => '店面营业执照', 'en' => 'Business license'],
    'wap_01378' => ['zh' => '店面形象', 'en' => 'Store image'],
    'wap_01379' => ['zh' => '设置密码', 'en' => 'Set password'],
    'wap_01380' => ['zh' => '请输入密码', 'en' => 'Enter password'],
    'wap_01381' => ['zh' => '提示：密码可用于刷新/修改/删除此信息', 'en' => 'Tip: password is used to refresh/edit/delete this post'],
    'wap_01382' => ['zh' => '请选择地区', 'en' => 'Select region'],
    'wap_01383' => ['zh' => '请填写招聘的具体要求，如性别、学历、年龄、工作经验和工作待遇等', 'en' => 'Enter requirements such as gender, education, age, experience and benefits'],
    'wap_01384' => ['zh' => '确定', 'en' => 'Confirm'],
    'wap_01385' => ['zh' => '继续发布', 'en' => 'Continue posting'],
    'wap_01386' => ['zh' => '去付款', 'en' => 'Pay now'],
    'wap_01387' => ['zh' => '重新发送', 'en' => 'Resend'],
    'wap_01388' => ['zh' => '重新发送(', 'en' => 'Resend ('],
    'wap_01389' => ['zh' => '请输入手机号码！', 'en' => 'Please enter mobile number!'],
    'wap_01390' => ['zh' => '手机格式错误！', 'en' => 'Invalid mobile number format!'],
    'wap_01391' => ['zh' => '请勿重复发送！', 'en' => 'Please do not send repeatedly!'],
    'wap_01392' => ['zh' => '优选', 'en' => 'Featured'],
    'wap_01393' => ['zh' => '短期', 'en' => 'Short-term'],
    'wap_01394' => ['zh' => '有效期至：', 'en' => 'Valid until:'],
    'wap_01395' => ['zh' => '暂未开放联系方式', 'en' => 'Contact info not available'],
    'wap_01396' => ['zh' => '只有个人用户才能申请报名', 'en' => 'Only job seekers can apply'],
    'wap_01397' => ['zh' => '您的姓名', 'en' => 'Your name'],
    'wap_01398' => ['zh' => '证件号码', 'en' => 'ID number'],
    'wap_01399' => ['zh' => '上传图片', 'en' => 'Upload image'],
    'wap_01400' => ['zh' => '（ 文字清晰，四角齐全 )', 'en' => '(Text clear, all four corners visible)'],
    'wap_01401' => ['zh' => '格式为', 'en' => 'Format:'],
    'wap_01402' => ['zh' => '不得超过', 'en' => 'Max size'],
    'wap_01403' => ['zh' => '公司名称', 'en' => 'Company name'],
    'wap_01404' => ['zh' => '信用代码', 'en' => 'Credit code'],
    'wap_01405' => ['zh' => '上传营业执照/组织机构代码证', 'en' => 'Upload business license / organization code certificate'],
    'wap_01406' => ['zh' => '选择上传图片', 'en' => 'Select image to upload'],
    'wap_01407' => ['zh' => '执照中的文字、图片、章印等需清晰可辨别，否则不能通过认证。', 'en' => 'Text, images and seals on the license must be clearly readable, otherwise verification will fail.'],
    'wap_01408' => ['zh' => '上传经办人身份证', 'en' => 'Upload agent ID card'],
    'wap_01409' => ['zh' => '图片和文字需清晰可辨别，否则不能通过认证。', 'en' => 'Images and text must be clearly readable, otherwise verification will fail.'],
    'wap_01410' => ['zh' => '上传委托书/承诺函', 'en' => 'Upload authorization letter / commitment letter'],
    'wap_01411' => ['zh' => '上传其他材料（选填）', 'en' => 'Upload other materials (optional)'],
    'wap_01412' => ['zh' => '请上传图片', 'en' => 'Please upload an image'],
    'wap_01413' => ['zh' => '请填写您的姓名', 'en' => 'Please enter your name'],
    'wap_01414' => ['zh' => '请填写证件号码', 'en' => 'Please enter ID number'],
    'wap_01415' => ['zh' => '请填写正确证件号码！', 'en' => 'Please enter a valid ID number!'],
    'wap_01416' => ['zh' => '请填写公司名称', 'en' => 'Please enter company name'],
    'wap_01417' => ['zh' => '上传中', 'en' => 'Uploading...'],
    'wap_01418' => ['zh' => '列表', 'en' => 'List'],
    'wap_01419' => ['zh' => '地图', 'en' => 'Map'],
    'wap_01420' => ['zh' => '换一批', 'en' => 'Refresh batch'],
    'wap_01421' => ['zh' => '查看详情>>', 'en' => 'View details >>'],
    'wap_01422' => ['zh' => '您的附近没有相关职位！', 'en' => 'No nearby jobs found!'],
    'wap_01423' => ['zh' => '您确定查找该地区附近的职位吗？', 'en' => 'Search for jobs near this area?'],
    'wap_01424' => ['zh' => '经验', 'en' => 'experience'],
    'wap_01425' => ['zh' => '岁', 'en' => 'years old'],
    'wap_01426' => ['zh' => '面试时间', 'en' => 'Interview time'],
    'wap_01427' => ['zh' => '请选择面试时间', 'en' => 'Select interview time'],
    'wap_01428' => ['zh' => '选择面试时间', 'en' => 'Select interview time'],
    'wap_01429' => ['zh' => '面试职位', 'en' => 'Interview position'],
    'wap_01430' => ['zh' => '邀请模板', 'en' => 'Invitation template'],
    'wap_01431' => ['zh' => '联系人', 'en' => 'Contact person'],
    'wap_01432' => ['zh' => '联系方式', 'en' => 'Contact info'],
    'wap_01433' => ['zh' => '面试地址', 'en' => 'Interview address'],
    'wap_01434' => ['zh' => '请填写面试地址', 'en' => 'Enter interview address'],
    'wap_01435' => ['zh' => '备注信息', 'en' => 'Remarks'],
    'wap_01436' => ['zh' => '可告知求职者面试时所需材料,面试前的相关注意事项', 'en' => 'Materials needed and notes before the interview'],
    'wap_01437' => ['zh' => '更新', 'en' => 'Update'],
    'wap_01438' => ['zh' => '发送面试邀请', 'en' => 'Send interview invitation'],
    'wap_01439' => ['zh' => '请选择面试职位', 'en' => 'Select interview position'],
    'wap_01440' => ['zh' => '联系电话格式错误', 'en' => 'Invalid phone number format'],
    'wap_01441' => ['zh' => '邀请中', 'en' => 'Inviting...'],
    'wap_01442' => ['zh' => '邀请成功', 'en' => 'Invitation sent'],
    'wap_01443' => ['zh' => '我的回答', 'en' => 'My answers'],
    'wap_01444' => ['zh' => '我的关注', 'en' => 'My follows'],
    'wap_01445' => ['zh' => '他的提问', 'en' => 'Their questions'],
    'wap_01446' => ['zh' => '我要找工作', 'en' => 'Find a job'],
    'wap_01447' => ['zh' => '我是求职者，我要找工作', 'en' => 'I am a job seeker looking for work'],
    'wap_01448' => ['zh' => '我要招人', 'en' => 'Hire talent'],
    'wap_01449' => ['zh' => '我是企业，我要招人', 'en' => 'I am an employer looking to hire'],
    'wap_01450' => ['zh' => '确认新密码：', 'en' => 'Confirm new password:'],
    'wap_01451' => ['zh' => '请输入新的用户名', 'en' => 'Enter new username'],
    'wap_01452' => ['zh' => '请输入新的用户密码', 'en' => 'Enter new password'],
    'wap_01453' => ['zh' => '请输入确认新的用户密码', 'en' => 'Confirm new password'],
    'wap_01454' => ['zh' => '输入新的用户名！', 'en' => 'Enter new username!'],
    'wap_01455' => ['zh' => '留下您的意见或反馈，我们会不断改进~', 'en' => 'Leave your feedback and we will keep improving~'],
    'wap_01456' => ['zh' => '短信验证', 'en' => 'SMS verification'],
    'wap_01457' => ['zh' => '输入短信验证码', 'en' => 'Enter SMS code'],
    'wap_01458' => ['zh' => '请选择意见类型', 'en' => 'Select feedback type'],
    'wap_01459' => ['zh' => '联系人不能空!', 'en' => 'Contact name required!'],
    'wap_01460' => ['zh' => '联系手机不能为空!', 'en' => 'Mobile number required!'],
    'wap_01461' => ['zh' => '手机格式错误!', 'en' => 'Invalid mobile number format!'],
    'wap_01462' => ['zh' => '反馈内容不能为空!', 'en' => 'Feedback cannot be empty!'],
    'wap_01463' => ['zh' => '我的频道', 'en' => 'My channels'],
    'wap_01464' => ['zh' => '推荐频道', 'en' => 'Recommended channels'],
    'wap_01465' => ['zh' => '推荐', 'en' => 'Recommended'],
    'wap_01466' => ['zh' => '请输入文章关键字', 'en' => 'Enter article keywords'],
    'wap_01467' => ['zh' => '没有搜索到资讯', 'en' => 'No articles found'],
    'wap_01468' => ['zh' => '重新搜索', 'en' => 'Search again'],
    'wap_01469' => ['zh' => '暂无资讯', 'en' => 'No articles yet'],
    'wap_01470' => ['zh' => '更多', 'en' => 'More'],
    'wap_01471' => ['zh' => '收起', 'en' => 'Collapse'],
    'wap_01472' => ['zh' => '点击：', 'en' => 'Clicks:'],
    'wap_01473' => ['zh' => '微信扫一扫分享资讯', 'en' => 'Scan with WeChat to share'],
    'wap_01474' => ['zh' => '相关推荐', 'en' => 'Related recommendations'],
    'wap_01475' => ['zh' => '暂无相关推荐', 'en' => 'No related recommendations'],
    'wap_01476' => ['zh' => '转发分享', 'en' => 'Share'],
    'wap_01477' => ['zh' => '找自己喜欢的工作', 'en' => 'Find the job you love'],
    'wap_01478' => ['zh' => '高薪职位招聘，找工作求职必备', 'en' => 'High-paying jobs — essential for job hunting'],
    'wap_01479' => ['zh' => '题', 'en' => 'questions'],
    'wap_01480' => ['zh' => '开始测试', 'en' => 'Start test'],
    'wap_01481' => ['zh' => '进入专题', 'en' => 'Enter topic'],
    'wap_01482' => ['zh' => '抱歉，还没有专题招聘的相关信息', 'en' => 'Sorry, no special recruitment topics yet'],
    'wap_01483' => ['zh' => '全力助您梦想起航', 'en' => 'Help you launch your career'],
    'wap_01484' => ['zh' => '月薪：', 'en' => 'Monthly salary:'],
    'wap_01485' => ['zh' => '向左滑动', 'en' => 'Swipe left'],
    'wap_01486' => ['zh' => '无福利待遇', 'en' => 'No benefits listed'],
    'wap_01487' => ['zh' => '查看该公司其他职位', 'en' => 'View other jobs at this company'],
    'wap_01488' => ['zh' => '其他联系方式', 'en' => 'Other contact info'],
    'wap_01489' => ['zh' => '查看详细信息', 'en' => 'View details'],
    'wap_01490' => ['zh' => '查看招聘职位', 'en' => 'View open jobs'],
    'wap_01491' => ['zh' => '请点击下方按钮跳转', 'en' => 'Tap the button below to continue'],
    'wap_01492' => ['zh' => '秒后自动跳转', 'en' => 'seconds until redirect'],
    'wap_01493' => ['zh' => '立即跳转', 'en' => 'Go now'],
    'wap_01494' => ['zh' => '分享成功', 'en' => 'Shared successfully'],
    'wap_01495' => ['zh' => '分享失败', 'en' => 'Share failed'],
    'wap_01496' => ['zh' => '分享给朋友或者朋友圈', 'en' => 'Share with friends or Moments'],
    'wap_01497' => ['zh' => '长按图片保存', 'en' => 'Long press to save image'],
    'wap_01498' => ['zh' => '生成中...', 'en' => 'Generating...'],
    'wap_01499' => ['zh' => '选择海报展示职位信息', 'en' => 'Select jobs to show on poster'],
    'wap_01500' => ['zh' => '生成', 'en' => 'Generate'],
    'wap_01501' => ['zh' => '招聘', 'en' => 'Recruitment'],
    'wap_01502' => ['zh' => '大数据', 'en' => 'Big Data'],
    'wap_01503' => ['zh' => '年度分析报告', 'en' => 'Annual analysis report'],
    'wap_01504' => ['zh' => '求职者数据', 'en' => 'Job seeker data'],
    'wap_01505' => ['zh' => '求职者画像', 'en' => 'Job seeker profile'],
    'wap_01506' => ['zh' => '地区，年龄，经验，男女，学历', 'en' => 'Region, age, experience, gender, education'],
    'wap_01507' => ['zh' => '地区分布', 'en' => 'Regional distribution'],
    'wap_01508' => ['zh' => '年龄分布', 'en' => 'Age distribution'],
    'wap_01509' => ['zh' => '经验分布', 'en' => 'Experience distribution'],
    'wap_01510' => ['zh' => '男性求职者', 'en' => 'Male job seekers'],
    'wap_01511' => ['zh' => '性别占比', 'en' => 'Gender ratio'],
    'wap_01512' => ['zh' => '女性求职者', 'en' => 'Female job seekers'],
    'wap_01513' => ['zh' => '学历分布', 'en' => 'Education distribution'],
    'wap_01514' => ['zh' => '大专', 'en' => 'College'],
    'wap_01515' => ['zh' => '本科', 'en' => 'Bachelor'],
    'wap_01516' => ['zh' => '高中', 'en' => 'High school'],
    'wap_01517' => ['zh' => '求职者行为', 'en' => 'Job seeker behavior'],
    'wap_01518' => ['zh' => '活跃趋势、行为趋势', 'en' => 'Activity trends, behavior trends'],
    'wap_01519' => ['zh' => '1-12月活跃趋势', 'en' => 'Jan-Dec activity trend'],
    'wap_01520' => ['zh' => '1-12月注册趋势', 'en' => 'Jan-Dec registration trend'],
    'wap_01521' => ['zh' => '企业数据', 'en' => 'Employer data'],
    'wap_01522' => ['zh' => '企业画像', 'en' => 'Employer profile'],
    'wap_01523' => ['zh' => '公司地区，公司规模，公司性质', 'en' => 'Company region, size, type'],
    'wap_01524' => ['zh' => '公司地区分布', 'en' => 'Company regional distribution'],
    'wap_01525' => ['zh' => '公司规模分布', 'en' => 'Company size distribution'],
    'wap_01526' => ['zh' => '公司性质分布', 'en' => 'Company type distribution'],
    'wap_01527' => ['zh' => '企业行为', 'en' => 'Employer behavior'],
    'wap_01528' => ['zh' => '登录趋势，发布岗位趋势', 'en' => 'Login trends, job posting trends'],
    'wap_01529' => ['zh' => '1-12月登录趋势', 'en' => 'Jan-Dec login trend'],
    'wap_01530' => ['zh' => '1-12月发布岗位趋势', 'en' => 'Jan-Dec job posting trend'],
    'wap_01531' => ['zh' => '谢谢您的观看', 'en' => 'Thank you for watching'],
    'wap_01532' => ['zh' => '以上数据由', 'en' => 'Data provided by'],
    'wap_01533' => ['zh' => '提供，最终解释权归我司所有', 'en' => '. All rights reserved.'],
    'wap_01534' => ['zh' => '点击查看好工作', 'en' => 'View great jobs'],
    'wap_01535' => ['zh' => '年度数据', 'en' => 'Annual data'],
    'wap_01536' => ['zh' => '招聘岗位', 'en' => 'Open jobs'],
    'wap_01537' => ['zh' => '执行中', 'en' => 'Processing...'],
    'wap_01538' => ['zh' => '我的频道最少要有一个', 'en' => 'You must keep at least one channel'],
    'wap_01539' => ['zh' => '向', 'en' => 'Swipe'],
    'wap_01540' => ['zh' => '左', 'en' => 'left'],
    'wap_01541' => ['zh' => '滑', 'en' => ''],
    'wap_01542' => ['zh' => '动', 'en' => ''],
    'wap_01543' => ['zh' => '次', 'en' => 'times'],
    'wap_01544' => ['zh' => '您还有', 'en' => 'You have'],
    'wap_01545' => ['zh' => '个订单未付款，是否继续发布！', 'en' => 'unpaid orders. Continue posting?'],
    'wap_01546' => ['zh' => '主&nbsp;  办&nbsp; 方：', 'en' => 'Organizer:'],
    'wap_01547' => ['zh' => '举办时间：', 'en' => 'Event time:'],
];

// Existing keys to prefer over new ones
$existingMap = [
    '分享海报' => 'wap_00872',
    '参会企业' => 'wap_00559',
    '参会职位' => 'wap_00560',
    '招聘人数' => 'wap_com_00333',
    '投递简历' => 'wap_com_00235',
    '招聘会介绍' => 'wap_00566',
    '举办时间' => 'wap_00567',
    '媒体宣传' => 'wap_00569',
    '展位设置方案' => 'wap_00564',
    '参与办法' => 'wap_00568',
    '查看全部岗位' => 'wap_00565',
    '展位号：' => 'wap_00607',
    '申请' => 'wap_00574',
    '电话咨询' => 'wap_00570',
    '预订展位' => 'wap_00571',
    '进行中' => 'wap_00604',
    '预定中' => 'wap_00605',
    '价格' => 'wap_00563',
    '参会展位' => 'wap_com_00425',
    '全选' => 'wap_js_00074',
    '提交' => 'wap_user_00176',
    '元' => 'common_02056',
    '已预订' => 'wap_00562',
    '人访问过' => 'wap_00612',
    '开始测试 >' => 'wap_00613',
    '手机上传' => 'wap_00542',
    '提交问题' => 'wap_00104',
    '我要提问' => 'wap_00278',
    '输入图片验证码' => 'wap_00262',
    '提交操作' => 'wap_00354',
    '实地核验' => 'wap_00274',
    '个在招职位' => 'wap_com_00094',
    '保存' => 'wap_user_00101',
    '个' => 'common_02050',
    '人' => 'common_02051',
    '举办地点' => 'admin_00282',
    '交通路线' => 'admin_00284',
    '主办方' => 'admin_00287',
];

// Build zh=>key map (prefer existing)
$strToKey = [];
foreach ($newKeys as $key => $data) {
    $strToKey[$data['zh']] = $key;
}
foreach ($existingMap as $str => $key) {
    $strToKey[$str] = $key;
}
foreach ($aliases as $str => $key) {
    if (!isset($strToKey[$str]) && isset($zh[$key])) {
        $strToKey[$str] = $key;
    }
}
uksort($strToKey, function($a, $b) {
    return mb_strlen($b, 'UTF-8') - mb_strlen($a, 'UTF-8');
});

function tTag($key) {
    return "{yun:}t key='{$key}'{/yun}";
}

function isInsideCommentOrI18n($content, $pos) {
    $before = substr($content, 0, $pos);
    // inside HTML comment
    $lastOpen = strrpos($before, '<!--');
    $lastClose = strrpos($before, '-->');
    if ($lastOpen !== false && ($lastClose === false || $lastClose < $lastOpen)) return true;
    // inside {yun:}t
    $lastT = strrpos($before, "{yun:}t");
    if ($lastT !== false) {
        $afterT = substr($content, $lastT, $pos - $lastT);
        if (strpos($afterT, '{/yun}') === false) return true;
    }
    // inside strpos config check
    $ctx = substr($content, max(0, $pos - 80), 160);
    if (preg_match('/strpos\s*\([^)]*$/', substr($content, max(0, $pos - 80), 80))) return true;
    if (preg_match('/code_web/', $ctx) && preg_match('/["\']/', $ctx)) return true;
    return false;
}

function migrateContent($content, $strToKey) {
    $changes = 0;
    foreach ($strToKey as $str => $key) {
        $tag = tTag($key);
        $offset = 0;
        while (($pos = mb_strpos($content, $str, 0, 'UTF-8')) !== false) {
            if (isInsideCommentOrI18n($content, $pos)) {
                $offset = $pos + mb_strlen($str, 'UTF-8');
                $content = substr_replace($content, '', $pos, 0); // no-op trick - use mb_strpos with offset
                // re-search from next position
                $searchFrom = $pos + 1;
                $found = false;
                $rest = mb_substr($content, $searchFrom, null, 'UTF-8');
                $relPos = mb_strpos($rest, $str, 0, 'UTF-8');
                if ($relPos === false) break;
                $pos = $searchFrom + $relPos;
                if (isInsideCommentOrI18n($content, $pos)) {
                    // skip this occurrence by temporarily marking - just continue loop from pos+1
                    $tmp = $pos;
                    while (($pos = mb_strpos($content, $str, $pos + 1, 'UTF-8')) !== false) {
                        if (!isInsideCommentOrI18n($content, $pos)) { $found = true; break; }
                    }
                    if (!$found) break;
                }
            }
            // Check we're not already replaced
            $checkBefore = substr($content, max(0, $pos - 20), 20);
            if (strpos($checkBefore, "{yun:}t") !== false) {
                $pos = $pos + mb_strlen($str, 'UTF-8');
                continue;
            }
            $content = mb_substr($content, 0, $pos, 'UTF-8') . $tag . mb_substr($content, $pos + mb_strlen($str, 'UTF-8'), null, 'UTF-8');
            $changes++;
            // continue searching after replacement
        }
    }
    return [$content, $changes];
}

// Simpler: line-by-line exact replacement avoiding already-i18n lines
function migrateFileSimple($path, $strToKey) {
    $lines = file($path);
    $changes = 0;
    foreach ($lines as $i => $line) {
        if (preg_match('/\{yun:\}t\s+key=/', $line)) continue;
        if (preg_match('/^\s*<!--/', $line) || preg_match('/<!--.*-->/', $line)) {
            // still process if mixed content
        }
        if (preg_match('/strpos\s*\(\s*\$config\.code_web/', $line)) continue;
        if (preg_match('/职场提问|店铺招聘|普工简历|意见反馈/', $line) && preg_match('/strpos|code_web/', $line)) continue;

        $origLine = $line;
        foreach ($strToKey as $str => $key) {
            if (mb_strpos($line, $str) === false) continue;
            if (mb_strpos($line, tTag($key)) !== false) continue;
            $line = str_replace($str, tTag($key), $line);
        }
        if ($line !== $origLine) {
            $changes++;
            $lines[$i] = $line;
        }
    }
    return [implode('', $lines), $changes > 0, $changes];
}

$done = [];
$remaining = [];
foreach (array_merge($tier2, $tier3) as $f) {
    $path = $base . $f;
    if (!file_exists($path)) { echo "MISSING: $f\n"; continue; }
    list($newContent, $changed, $changes) = migrateFileSimple($path, $strToKey);
    if ($changed) {
        file_put_contents($path, $newContent);
        $done[] = $f;
        echo "OK: $f ($changes lines)\n";
    } else {
        echo "SKIP: $f\n";
    }
    // check remaining Chinese
    if (preg_match('/[\x{4e00}-\x{9fff}]/u', $newContent)) {
        preg_match_all('/[\x{4e00}-\x{9fff}][^\n<{]*[\x{4e00}-\x{9fff}]/u', $newContent, $m);
        $left = array_unique($m[0]);
        $left = array_filter($left, function($s) {
            return !preg_match('/^\s*\/\//', $s) && mb_strlen($s) > 1;
        });
        if (count($left) > 0) {
            $remaining[$f] = array_slice($left, 0, 8);
        }
    }
}

echo "\n=== REMAINING CHINESE ===\n";
foreach ($remaining as $f => $items) {
    echo "$f:\n";
    foreach ($items as $item) echo "  - $item\n";
}

// Write lang pack additions
$zhFile = ROOT . 'data/lang/auto/zh_cn.php';
$enFile = ROOT . 'data/lang/auto/en_us.php';
$aliasFile = ROOT . 'data/lang/auto/aliases.php';

$zhContent = file_get_contents($zhFile);
$enContent = file_get_contents($enFile);
$aliasContent = file_get_contents($aliasFile);

$zhAdd = "\n";
$enAdd = "\n";
$aliasAdd = "\n";
foreach ($newKeys as $key => $data) {
    if (strpos($zhContent, "'$key'") !== false) continue;
    $zhEsc = addcslashes($data['zh'], "'\\");
    $enEsc = addcslashes($data['en'], "'\\");
    $zhAdd .= "  '$key' => '$zhEsc',\n";
    $enAdd .= "  '$key' => '$enEsc',\n";
    $aliasAdd .= "  '$zhEsc' => '$key',\n";
}

$zhContent = preg_replace('/\);\s*$/', rtrim($zhAdd) . "\n);", $zhContent);
$enContent = preg_replace('/\);\s*$/', rtrim($enAdd) . "\n);", $enContent);
$aliasContent = preg_replace('/\);\s*$/', rtrim($aliasAdd) . "\n);", $aliasContent);

file_put_contents($zhFile, $zhContent);
file_put_contents($enFile, $enContent);
file_put_contents($aliasFile, $aliasContent);

echo "\nAdded " . count($newKeys) . " new keys (wap_01284 - wap_01547)\n";
echo "Migrated " . count($done) . " files\n";
