<template>
    <div class="yejiClassDome">
        <el-tabs v-model="activeName" :tab-position="tabPosition" @tab-click="handleTabClick">
            <el-tab-pane label="{yun:}t key='admin_tool_00147'{/yun}" name="first">
                <div class="yejiTabCont">
                    <div class="yejiTabHead">
                        <div class="yejiTabHeaButn">
                            <span :class="search.days == -1 ? 'dayCur' : ''" @click="handleSearch(-1)">{yun:}t key='common_02000'{/yun}</span>
                            <span :class="search.days == 1 ? 'dayCur' : ''" @click="handleSearch(1)">{yun:}t key='common_01940'{/yun}</span>
                            <span :class="search.days == 2 ? 'dayCur' : ''" @click="handleSearch(2)">{yun:}t key='admin_tool_00142'{/yun}</span>
                            <span :class="search.days == 3 ? 'dayCur' : ''" @click="handleSearch(3)">{yun:}t key='admin_tool_00143'{/yun}</span>
                            <span :class="search.days == 4 ? 'dayCur' : ''" @click="handleSearch(4)">{yun:}t key='admin_tool_00146'{/yun}</span>
                            <span :class="search.days == 5 ? 'dayCur' : ''" @click="handleSearch(5)">{yun:}t key='admin_tool_00145'{/yun}</span>
                        </div>
                        <div class="yejiTabHeaIput">
                            <el-date-picker v-model="search.time" type="daterange" :picker-options="pickerOptions"
                                range-separator="{yun:}t key='admin_company_00019'{/yun}" start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}"
                                value-format="timestamp" @change="handleSearch(0)"></el-date-picker>
                        </div>
                        <div style="margin-left: 5px;">
                            <el-button type="primary" size="mini" icon="el-icon-search" @click="handleSearch(0)">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                        </div>
                    </div>
                    <div class="yejiTabChats">
                        <div class="yejiTabChatOne">
                            <div class="yejiTabData">
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/number1.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00105'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.allReg }}</span>
                                    </div>
                                </div>
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/number2.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_user_company_00281'{/yun}</span>
                                        <el-button type="primary" plain @click="showComBox(1)">{yun:}t key='member_com_00325'{/yun}</el-button>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.comReg }}</span>
                                    </div>
                                </div>
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/number3.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_system_00129'{/yun}</span>
                                        <el-button type="primary" plain @click="showUserBox(1)">{yun:}t key='member_com_00325'{/yun}</el-button>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.userReg }}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00168'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main11"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00154'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main11_1"></div>
                            </div>
                        </div>
                        <div class="yejiTabChatOne">
                            <div class="yejiTabData">
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/number1.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00106'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.allLogin }}</span>
                                    </div>
                                </div>
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/number2.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_user_company_00281'{/yun}</span>
                                        <el-button type="primary" plain @click="showComBox(2)">{yun:}t key='member_com_00325'{/yun}</el-button>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.comLogin }}</span>
                                    </div>
                                </div>
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/number3.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_system_00129'{/yun}</span>
                                        <el-button type="primary" plain @click="showUserBox(2)">{yun:}t key='member_com_00325'{/yun}</el-button>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.userLogin }}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00169'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main12"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </el-tab-pane>
            <el-tab-pane label="{yun:}t key='admin_tool_00148'{/yun}" name="second">
                <div class="yejiTabCont">
                    <div class="yejiTabHead">
                        <div class="yejiTabHeaButn">
                            <span :class="search.days == -1 ? 'dayCur' : ''" @click="handleSearch(-1)">{yun:}t key='common_02000'{/yun}</span>
                            <span :class="search.days == 1 ? 'dayCur' : ''" @click="handleSearch(1)">{yun:}t key='common_01940'{/yun}</span>
                            <span :class="search.days == 2 ? 'dayCur' : ''" @click="handleSearch(2)">{yun:}t key='admin_tool_00142'{/yun}</span>
                            <span :class="search.days == 3 ? 'dayCur' : ''" @click="handleSearch(3)">{yun:}t key='admin_tool_00143'{/yun}</span>
                            <span :class="search.days == 4 ? 'dayCur' : ''" @click="handleSearch(4)">{yun:}t key='admin_tool_00146'{/yun}</span>
                            <span :class="search.days == 5 ? 'dayCur' : ''" @click="handleSearch(5)">{yun:}t key='admin_tool_00145'{/yun}</span>
                        </div>
                        <div class="yejiTabHeaIput">
                            <el-date-picker v-model="search.time" size="small" type="daterange"
                                :picker-options="pickerOptions" range-separator="{yun:}t key='admin_company_00019'{/yun}" start-placeholder="{yun:}t key='admin_00343'{/yun}"
                                end-placeholder="{yun:}t key='admin_00344'{/yun}" value-format="timestamp" @change="handleSearch(0)"></el-date-picker>
                            <el-button type="primary" icon="el-icon-search" @click="handleSearch(0)">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                        </div>
                    </div>
                    <div class="yejiTabChats">
                        <div class="yejiTabChatOne">
                            <div class="yejiTabData">
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/dataim4.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00112'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.job }}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00171'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main13"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00100'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.jobList" height="300" border style="width: 100%" v-loading="loading" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="name" label="{yun:}t key='admin_tool_00165'{/yun}"></el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='wap_com_00112'{/yun}" width="80"></el-table-column>
                                        <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80">
                                            <template slot-scope="scope">
                                                <div class="cz_button">
                                                    <el-button size="small" type="primary"
                                                        @click="openPage(scope.row.id, 'job')">{yun:}t key='wap_com_00427'{/yun}
                                                    </el-button>
                                                </div>
                                            </template>
                                        </el-table-column>
                                    </el-table>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00102'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.jobComList" height="300" border style="width: 100%" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="name" label="{yun:}t key='wap_com_00157'{/yun}"></el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='wap_com_00112'{/yun}" width="80"></el-table-column>
                                        <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80">
                                            <template slot-scope="scope">
                                                <div class="cz_button">
                                                    <el-button size="small" type="primary"
                                                        @click="openPage(scope.row.uid, 'company')">{yun:}t key='wap_com_00427'{/yun}
                                                    </el-button>
                                                </div>
                                            </template>
                                        </el-table-column>
                                    </el-table>
                                </div>
                            </div>
                        </div>
                        <div class="yejiTabChatOne">
                            <div class="yejiTabData">
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/dataim5.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00110'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.resume }}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00170'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main14"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00101'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.resumeList" height="300" border style="width: 100%" v-loading="loading" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="uname" label="{yun:}t key='admin_user_00185'{/yun}" width="100"></el-table-column>
                                        <el-table-column label="{yun:}t key='admin_tool_00134'{/yun}">
                                            <template slot-scope="scope">
                                                <span>{{ lc('admin_job_intention_value', [scope.row.jobclassname]) }}</span>
                                                <span>{{ lc('admin_intended_area_value', [scope.row.cityclassname]) }}</span>
                                                <span>{{ lc('admin_education_value', [scope.row.eduname]) }}</span>
                                                <span>{{ lc('admin_experience_value', [scope.row.expname]) }}</span>
                                            </template>
                                        </el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='wap_com_00112'{/yun}" width="80"></el-table-column>
                                        <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80">
                                            <template slot-scope="scope">
                                                <div class="cz_button">
                                                    <el-button size="small" type="primary"
                                                        @click="openPage(scope.row.id, 'resume')">{yun:}t key='wap_com_00427'{/yun}
                                                    </el-button>
                                                </div>
                                            </template>
                                        </el-table-column>
                                    </el-table>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00118'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.resumeComList" height="300" border style="width: 100%" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="name" label="{yun:}t key='wap_com_00157'{/yun}"></el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='wap_com_00112'{/yun}" width="80"></el-table-column>
                                        <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80">
                                            <template slot-scope="scope">
                                                <div class="cz_button">
                                                    <el-button size="small" type="primary"
                                                        @click="openPage(scope.row.uid, 'company')">{yun:}t key='wap_com_00427'{/yun}
                                                    </el-button>
                                                </div>
                                            </template>
                                        </el-table-column>
                                    </el-table>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </el-tab-pane>
            <el-tab-pane label="{yun:}t key='admin_tool_00149'{/yun}" name="third">
                <div class="yejiTabCont">
                    <div class="yejiTabHead">
                        <div class="yejiTabHeaButn">
                            <span :class="search.days == -1 ? 'dayCur' : ''" @click="handleSearch(-1)">{yun:}t key='common_02000'{/yun}</span>
                            <span :class="search.days == 1 ? 'dayCur' : ''" @click="handleSearch(1)">{yun:}t key='common_01940'{/yun}</span>
                            <span :class="search.days == 2 ? 'dayCur' : ''" @click="handleSearch(2)">{yun:}t key='admin_tool_00142'{/yun}</span>
                            <span :class="search.days == 3 ? 'dayCur' : ''" @click="handleSearch(3)">{yun:}t key='admin_tool_00143'{/yun}</span>
                            <span :class="search.days == 4 ? 'dayCur' : ''" @click="handleSearch(4)">{yun:}t key='admin_tool_00146'{/yun}</span>
                            <span :class="search.days == 5 ? 'dayCur' : ''" @click="handleSearch(5)">{yun:}t key='admin_tool_00145'{/yun}</span>
                        </div>
                        <div class="yejiTabHeaIput">
                            <el-date-picker v-model="search.time" size="small" type="daterange"
                                :picker-options="pickerOptions" range-separator="{yun:}t key='admin_company_00019'{/yun}" start-placeholder="{yun:}t key='admin_00343'{/yun}"
                                end-placeholder="{yun:}t key='admin_00344'{/yun}" value-format="timestamp" @change="handleSearch(0)"></el-date-picker>
                            <el-button type="primary" icon="el-icon-search" @click="handleSearch(0)">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                        </div>
                    </div>
                    <div class="yejiTabChats">
                        <div class="yejiTabChatOne">
                            <div class="yejiTabData">
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/dataim7.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00113'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.invite }}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00114'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main15"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00093'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.inviteResume" height="300" border style="width: 100%" v-loading="loading" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="name" label="{yun:}t key='admin_tool_00166'{/yun}"></el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='admin_tool_00137'{/yun}" width="80"></el-table-column>
                                        <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80">
                                            <template slot-scope="scope">
                                                <div class="cz_button">
                                                    <el-button size="small" type="primary"
                                                        @click="openPage(scope.row.eid, 'resume')">{yun:}t key='wap_com_00427'{/yun}
                                                    </el-button>
                                                </div>
                                            </template>
                                        </el-table-column>
                                    </el-table>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00089'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.inviteCom" height="300" border style="width: 100%" v-loading="loading" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="name" label="{yun:}t key='wap_com_00157'{/yun}"></el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='admin_tool_00144'{/yun}" width="80"></el-table-column>
                                        <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80">
                                            <template slot-scope="scope">
                                                <div class="cz_button">
                                                    <el-button size="small" type="primary"
                                                        @click="openPage(scope.row.uid, 'company')">{yun:}t key='wap_com_00427'{/yun}
                                                    </el-button>
                                                </div>
                                            </template>
                                        </el-table-column>
                                    </el-table>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00083'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main15_1"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00090'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main15_2"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00092'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main15_3"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00091'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main15_4"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00082'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main15_5"></div>
                            </div>
                        </div>
                        <div class="yejiTabChatOne">
                            <div class="yejiTabData">
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/dataim6.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00107'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.down }}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00108'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main16"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00085'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.downResume" height="300" border style="width: 100%" v-loading="loading" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="name" label="{yun:}t key='admin_user_00185'{/yun}"></el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='admin_tool_00120'{/yun}" width="80"></el-table-column>
                                        <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80">
                                            <template slot-scope="scope">
                                                <div class="cz_button">
                                                    <el-button size="small" type="primary"
                                                        @click="openPage(scope.row.eid, 'resume')">{yun:}t key='wap_com_00427'{/yun}
                                                    </el-button>
                                                </div>
                                            </template>
                                        </el-table-column>
                                    </el-table>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00084'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.downCom" height="300" border style="width: 100%" v-loading="loading" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="name" label="{yun:}t key='wap_com_00157'{/yun}"></el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='admin_tool_00120'{/yun}" width="80"></el-table-column>
                                        <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80">
                                            <template slot-scope="scope">
                                                <div class="cz_button">
                                                    <el-button size="small" type="primary"
                                                        @click="openPage(scope.row.uid, 'company')">{yun:}t key='wap_com_00427'{/yun}
                                                    </el-button>
                                                </div>
                                            </template>
                                        </el-table-column>
                                    </el-table>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00094'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main16_1"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00111'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main16_2"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00095'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main16_3"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00097'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main16_4"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00104'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main16_5"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00133'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main16_6"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </el-tab-pane>
            <el-tab-pane label="{yun:}t key='admin_tool_00150'{/yun}" name="fourth">
                <div class="yejiTabCont">
                    <div class="yejiTabHead">
                        <div class="yejiTabHeaButn">
                            <span :class="search.days == -1 ? 'dayCur' : ''" @click="handleSearch(-1)">{yun:}t key='common_02000'{/yun}</span>
                            <span :class="search.days == 1 ? 'dayCur' : ''" @click="handleSearch(1)">{yun:}t key='common_01940'{/yun}</span>
                            <span :class="search.days == 2 ? 'dayCur' : ''" @click="handleSearch(2)">{yun:}t key='admin_tool_00142'{/yun}</span>
                            <span :class="search.days == 3 ? 'dayCur' : ''" @click="handleSearch(3)">{yun:}t key='admin_tool_00143'{/yun}</span>
                            <span :class="search.days == 4 ? 'dayCur' : ''" @click="handleSearch(4)">{yun:}t key='admin_tool_00146'{/yun}</span>
                            <span :class="search.days == 5 ? 'dayCur' : ''" @click="handleSearch(5)">{yun:}t key='admin_tool_00145'{/yun}</span>
                        </div>
                        <div class="yejiTabHeaIput">
                            <el-date-picker v-model="search.time" size="small" type="daterange"
                                :picker-options="pickerOptions" range-separator="{yun:}t key='admin_company_00019'{/yun}" start-placeholder="{yun:}t key='admin_00343'{/yun}"
                                end-placeholder="{yun:}t key='admin_00344'{/yun}" value-format="timestamp" @change="handleSearch(0)"></el-date-picker>
                            <el-button type="primary" icon="el-icon-search" @click="handleSearch(0)">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                        </div>
                    </div>
                    <div class="yejiTabChats">
                        <div class="yejiTabChatOne">
                            <div class="yejiTabData">
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/nav_reward1.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00158'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.order }}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00159'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main17"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00096'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.orderCom" height="300" border style="width: 100%" v-loading="loading" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="username" label="{yun:}t key='admin_user_00140'{/yun}"></el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='wap_user_00312'{/yun}" width="120"></el-table-column>
                                    </el-table>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='wap_user_00318'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main17_1"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='wap_user_00313'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main17_2"></div>
                            </div>
                        </div>
                        <div class="yejiTabChatOne">
                            <div class="yejiTabData">
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/nav_data1.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00116'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.ad }}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00099'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main18"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00119'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.adClick" height="300" border style="width: 100%" v-loading="loading" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="name" label="{yun:}t key='admin_yunying_00047'{/yun}"></el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='admin_tool_00130'{/yun}" width="80"></el-table-column>
                                    </el-table>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </el-tab-pane>
            <el-tab-pane label="{yun:}t key='admin_tool_00086'{/yun}" name="fifth">
                <div class="yejiTabCont">
                    <div class="yejiTabHead">
                        <div class="yejiTabHeaButn">
                            <span :class="search.days == -1 ? 'dayCur' : ''" @click="handleSearch(-1)">{yun:}t key='common_02000'{/yun}</span>
                            <span :class="search.days == 1 ? 'dayCur' : ''" @click="handleSearch(1)">{yun:}t key='common_01940'{/yun}</span>
                            <span :class="search.days == 2 ? 'dayCur' : ''" @click="handleSearch(2)">{yun:}t key='admin_tool_00142'{/yun}</span>
                            <span :class="search.days == 3 ? 'dayCur' : ''" @click="handleSearch(3)">{yun:}t key='admin_tool_00143'{/yun}</span>
                            <span :class="search.days == 4 ? 'dayCur' : ''" @click="handleSearch(4)">{yun:}t key='admin_tool_00146'{/yun}</span>
                            <span :class="search.days == 5 ? 'dayCur' : ''" @click="handleSearch(5)">{yun:}t key='admin_tool_00145'{/yun}</span>
                        </div>
                        <div class="yejiTabHeaIput">
                            <el-date-picker v-model="search.time" type="daterange" :picker-options="pickerOptions"
                                range-separator="{yun:}t key='admin_company_00019'{/yun}" start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}"
                                value-format="timestamp" @change="handleSearch(0)"></el-date-picker>
                            <el-button type="primary" icon="el-icon-search" @click="handleSearch(0)">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                        </div>
                    </div>
                    <div class="yejiTabChats">
                        <div class="yejiTabChatOne">
                            <div class="yejiTabData">
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/dataim4.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00160'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.addJob }}</span>
                                    </div>
                                </div>
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/dataim5.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00098'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.upJob }}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00161'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main19"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00103'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.addJobCom" height="300" border style="width: 100%" v-loading="loading" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="name" label="{yun:}t key='wap_com_00157'{/yun}"></el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='admin_tool_00167'{/yun}" width="80"></el-table-column>
                                        <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80">
                                            <template slot-scope="scope">
                                                <div class="cz_button">
                                                    <el-button size="small" type="primary"
                                                        @click="openPage(scope.row.uid, 'company')">{yun:}t key='wap_com_00427'{/yun}
                                                    </el-button>
                                                </div>
                                            </template>
                                        </el-table-column>
                                    </el-table>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00131'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main19_1"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00132'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main19_2"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='wap_com_00290'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main19_3"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='wap_com_00283'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main19_4"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='wap_user_00240'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main19_5"></div>
                            </div>
                        </div>
                        <div class="yejiTabChatOne">
                            <div class="yejiTabData">
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/dataim1.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00162'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.addResume }}</span>
                                    </div>
                                </div>
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/dataim5.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00163'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.upResume }}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00164'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main20"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00129'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main20_1"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00126'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main20_2"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='wap_user_00016'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main20_3"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='member_com_00011'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main20_4"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='wap_user_00240'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main20_5"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00133'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main20_6"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_yunying_00139'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main20_7"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </el-tab-pane>
            <el-tab-pane label="{yun:}t key='admin_tool_00151'{/yun}" name="sixth">
                <div class="yejiTabCont">
                    <div class="yejiTabHead">
                        <div class="yejiTabHeaButn">
                            <span :class="search.days == -1 ? 'dayCur' : ''" @click="handleSearch(-1)">{yun:}t key='common_02000'{/yun}</span>
                            <span :class="search.days == 1 ? 'dayCur' : ''" @click="handleSearch(1)">{yun:}t key='common_01940'{/yun}</span>
                            <span :class="search.days == 2 ? 'dayCur' : ''" @click="handleSearch(2)">{yun:}t key='admin_tool_00142'{/yun}</span>
                            <span :class="search.days == 3 ? 'dayCur' : ''" @click="handleSearch(3)">{yun:}t key='admin_tool_00143'{/yun}</span>
                            <span :class="search.days == 4 ? 'dayCur' : ''" @click="handleSearch(4)">{yun:}t key='admin_tool_00146'{/yun}</span>
                            <span :class="search.days == 5 ? 'dayCur' : ''" @click="handleSearch(5)">{yun:}t key='admin_tool_00145'{/yun}</span>
                        </div>
                        <div class="yejiTabHeaIput">
                            <el-date-picker v-model="search.time" type="daterange" :picker-options="pickerOptions"
                                range-separator="{yun:}t key='admin_company_00019'{/yun}" start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}"
                                value-format="timestamp" @change="handleSearch(0)"></el-date-picker>
                            <el-button type="primary" icon="el-icon-search" @click="handleSearch(0)">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                        </div>
                    </div>
                    <div class="yejiTabChats">
                        <div class="yejiTabChatOne">
                            <div class="yejiTabData">
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/number2.png" alt="">
                                        <span class="spannum">{yun:}t key='admin_tool_00122'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.comOne }}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00123'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main21"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00156'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main21_1"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00157'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main21_2"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00124'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main21_3"></div>
                            </div>
                        </div>
                        <div class="yejiTabChatOne">
                            <div class="yejiTabData">
                                <div class="yejiTabDatLis">
                                    <div class="yejiTabDatTite">
                                        <img src="../../images/number3.png" alt="">
                                        <span class="spannum">{yun:}t key='member_com_00152'{/yun}</span>
                                    </div>
                                    <div class="yejiTabDatNum">
                                        <span>{{ AllNum.apply }}</span>
                                    </div>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00109'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main22"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00087'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.applyResume" height="300" border style="width: 100%" v-loading="loading" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="uname" label="{yun:}t key='admin_tool_00166'{/yun}"></el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='admin_tool_00128'{/yun}" width="100"></el-table-column>
                                        <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80">
                                            <template slot-scope="scope">
                                                <div class="cz_button">
                                                    <el-button size="small" type="primary"
                                                        @click="openPage(scope.row.id, 'resume')">{yun:}t key='wap_com_00427'{/yun}
                                                    </el-button>
                                                </div>
                                            </template>
                                        </el-table-column>
                                    </el-table>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00088'{/yun}</span>
                                </div>
                                <div class="tabEchatDome">
                                    <el-table :data="topList.applyCom" height="300" border style="width: 100%" v-loading="loading" element-loading-text="{yun:}t key='admin_tool_00117'{/yun}" element-loading-spinner="el-icon-loading" element-loading-background="rgba(0, 0, 0, 0.8)">
                                        <el-table-column prop="name" label="{yun:}t key='admin_tool_00166'{/yun}"></el-table-column>
                                        <el-table-column prop="count" label="{yun:}t key='admin_yunying_00166'{/yun}" width="100"></el-table-column>
                                        <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80">
                                            <template slot-scope="scope">
                                                <div class="cz_button">
                                                    <el-button size="small" type="primary"
                                                        @click="openPage(scope.row.uid, 'company')">{yun:}t key='wap_com_00427'{/yun}
                                                    </el-button>
                                                </div>
                                            </template>
                                        </el-table-column>
                                    </el-table>
                                </div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00156'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main22_1"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00132'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main22_2"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='wap_user_00016'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main22_3"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='member_com_00011'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main22_4"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='wap_user_00240'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main22_5"></div>
                            </div>
                            <div class="yejiTabEchaDome">
                                <div class="yejiTabEchaName">
                                    <span>{yun:}t key='admin_tool_00133'{/yun}</span>
                                </div>
                                <div class="tabEchatDome" id="main22_6"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </el-tab-pane>
        </el-tabs>

        <!-- 企业用户信息弹出框 -->
        <el-drawer title="{yun:}t key='admin_tool_00153'{/yun}" :visible.sync="comBoxShow" append-to-body size="85%">
            <company-user ref="companyUser" :jump_params="jumpParams" v-if="comBoxShow"></company-user>
        </el-drawer>
        <!-- 个人用户信息弹出框 -->
        <el-drawer title="{yun:}t key='admin_tool_00152'{/yun}" :visible.sync="userBoxShow" append-to-body size="85%">
            <users-all ref="userBoxShow" :jump_params="jumpParams" v-if="userBoxShow"></users-all>
        </el-drawer>
    </div>
</template>

<script>
module.exports = {
    data: function () {
        return {
            loading: false,
            emptytext: window.yunAdminT("{yun:}t key='wap_js_00113'{/yun}"),
            webUrl: localStorage.getItem("sy_weburl"),
            activeName: 'first',
            tabPosition: 'left',
            type: 1,
            search: {
                days: 7,
                time: ''
            },
            pickMinData: '',
            pickerOptions: {
                onPick: ({ maxDate, minDate }) => {
                    if (minDate && this.pickMinData) {

                        this.pickMinData = null
                    } else if (minDate) {

                        this.pickMinData = minDate.getTime()
                    }
                }
            },

            AllNum: [],
            list: [],

            CountTj: [],
            topList: [],

            comBoxShow: false,
            userBoxShow: false,
            jumpParams: {}
        }
    },
    components: {
        'company-user': httpVueLoader('../../../user/company/component/companyuser.vue'),
        'users-all': httpVueLoader('../../../user/users/component/usersall.vue'),
    },
    created() {
        this.handleSearch(2);
    },
    methods: {
        handleTabClick(tab, event) {
            let that = this;
            if (tab._props.name == 'first') {

                that.type = 1;
            } else if (tab._props.name == 'second') {

                that.type = 2;
            } else if (tab._props.name == 'third') {

                that.type = 3;
            } else if (tab._props.name == 'fourth') {

                that.type = 4;
            } else if (tab._props.name == 'fifth') {

                that.type = 5;
            } else if (tab._props.name == 'sixth') {

                that.type = 6;
            }
            that.handleSearch(2);
        },
        handleSearch(day) {
            let that = this;
            if (day != 0) {
                that.search.days = day;
                that.search.time = '';
            } else {
                if(that.search.time==''){
                	that.search.days = 1;
                }else{
                	that.search.days = '';
                }
            }
            let params = JSON.parse(JSON.stringify(that.search));
            params.type = that.type;
            that.loading = true;
            httpPost('m=tool&c=dataBoard&a=class', params, { hideloading: true }).then(function (res) {
                if (res.data.error == 0) {

                    let data = res.data.data;
                    that.AllNum = data.AllNum;
                    that.list = data.List;
                    if (that.type == 1) {

                        that.CountTj = data.CountTj;

                        that.initCharts11();
                        that.initCharts11_1();
                        that.initCharts12();
                    } else if (that.type == 2) {

                        that.topList = data.topList;
                        
                        that.initCharts13();
                        that.initCharts14();
                    } else if (that.type == 3) {

                        that.topList = data.topList;
                        that.CountTj = data.CountTj;

                        that.initCharts15();
                        that.initCharts15_1();
                        that.initCharts15_2();
                        that.initCharts15_3();
                        that.initCharts15_4();
                        that.initCharts15_5();

                        that.initCharts16();
                        that.initCharts16_1();
                        that.initCharts16_2();
                        that.initCharts16_3();
                        that.initCharts16_4();
                        that.initCharts16_5();
                        that.initCharts16_6();
                    } else if (that.type == 4) {

                        that.topList = data.topList;
                        that.CountTj = data.CountTj;

                        that.initCharts17();
                        that.initCharts17_1();
                        that.initCharts17_2();

                        that.initCharts18();
                    } else if (that.type == 5) {

                        that.topList = data.topList;
                        that.CountTj = data.CountTj;

                        that.initCharts19();
                        that.initCharts19_1();
                        that.initCharts19_2();
                        that.initCharts19_3();
                        that.initCharts19_4();
                        that.initCharts19_5();

                        that.initCharts20();
                        that.initCharts20_1();
                        that.initCharts20_2();
                        that.initCharts20_3();
                        that.initCharts20_4();
                        that.initCharts20_5();
                        that.initCharts20_6();
                        that.initCharts20_7();
                    } else if (that.type == 6) {

                        that.CountTj = data.CountTj;
                        that.topList = data.topList;

                        that.initCharts21();
                        that.initCharts21_1();
                        that.initCharts21_2();
                        that.initCharts21_3();

                        that.initCharts22();
                        that.initCharts22_1();
                        that.initCharts22_2();
                        that.initCharts22_3();
                        that.initCharts22_4();
                        that.initCharts22_5();
                        that.initCharts22_6();
                    }
                    that.loading = false;
                }
            });
        },
        initCharts11() {
            let that = this;
            let allList = JSON.parse(JSON.stringify(that.list.allReg.list));
            let comList = JSON.parse(JSON.stringify(that.list.comReg.list));
            let userList = JSON.parse(JSON.stringify(that.list.userReg.list));

            let xAxisData = [];
            for (let i in allList) {
                xAxisData.push(allList[i].date);
            }
            let seriesArr = [];
            seriesArr.name = [that.list.userReg.name, that.list.comReg.name, that.list.allReg.name];
            let oneData = [];
            let twoData = [];
            let threeData = [];
            for (let i in userList) {
                oneData.push(userList[i].count);
            }
            for (let i in comList) {
                twoData.push(comList[i].count);
            }
            for (let i in allList) {
                threeData.push(allList[i].count);
            }
            seriesArr.data = [oneData, twoData, threeData]
            var myChart = echarts.init(document.getElementById('main11'));
            var option = {
                color: ['#8997FB', '#23C9C9', '#1890FF'],
                legend: {
                    left: 'right',
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '12%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    data: xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    minInterval: 1,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: [
                    {
                        name: seriesArr.name[0],
                        data: seriesArr.data[0],
                        type: 'bar',
                        stack: 'total',
                        barWidth: '25'
                    },
                    {
                        name: seriesArr.name[1],
                        data: seriesArr.data[1],
                        type: 'bar',
                        stack: 'total',
                        barWidth: '25'
                    },
                    {
                        name: seriesArr.name[2],
                        data: seriesArr.data[2],
                        type: 'line',
                        stack: 'total',
                        symbol: 'circle',//拐点样式
                        symbolSize: 10,
                    },
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts11_1() {
            let that = this;
            let data = [];
            if (that.CountTj.source != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj.source));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main11_1'));
            var option = {
                // color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '10%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00154'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '42%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },

        initCharts12() {
            let that = this;
            let allList = JSON.parse(JSON.stringify(that.list.allLogin.list));
            let comList = JSON.parse(JSON.stringify(that.list.comLogin.list));
            let userList = JSON.parse(JSON.stringify(that.list.userLogin.list));

            let xAxisData = [];
            for (let i in allList) {
                xAxisData.push(allList[i].date);
            }
            let seriesArr = [];
            seriesArr.name = [that.list.userLogin.name, that.list.comLogin.name, that.list.allLogin.name];
            let oneData = [];
            let twoData = [];
            let threeData = [];
            for (let i in userList) {
                oneData.push(userList[i].count);
            }
            for (let i in comList) {
                twoData.push(comList[i].count);
            }
            for (let i in allList) {
                threeData.push(allList[i].count);
            }
            seriesArr.data = [oneData, twoData, threeData]
            var myChart = echarts.init(document.getElementById('main12'));
            var option = {
                color: ['#8997FB', '#23C9C9', '#1890FF'],
                legend: {
                    left: 'right',
                    // icon: 'circle',
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '12%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    // boundaryGap: false,
                    data: xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    minInterval: 1,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: [
                    {
                        name: seriesArr.name[0],
                        data: seriesArr.data[0],
                        type: 'bar',
                        stack: 'total',
                        barWidth: '25'
                    },
                    {
                        name: seriesArr.name[1],
                        data: seriesArr.data[1],
                        type: 'bar',
                        stack: 'total',
                        barWidth: '25'
                    },
                    {
                        name: seriesArr.name[2],
                        data: seriesArr.data[2],
                        type: 'line',
                        stack: 'total',
                        symbol: 'circle',//拐点样式
                        symbolSize: 10,
                    },
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts13() {
            let that = this;
            let jobList = JSON.parse(JSON.stringify(that.list.job.list));
            let xAxisData = [];
            let seriesData = [];
            let seriesName = that.list.job.name;

            for (let i in jobList) {
                xAxisData.push(jobList[i].date);
                seriesData.push(jobList[i].count);
            }
            var myChart = echarts.init(document.getElementById('main13'));
            var option = {
                color: ['#8997FB', '#23C9C9', '#1890FF'],
                legend: {
                    left: 'right',
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '12%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    data: xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    minInterval: 1,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: [
                    {
                        name: seriesName,
                        data: seriesData,
                        type: 'line',
                        stack: 'total',
                        symbol: 'circle',//拐点样式
                        symbolSize: 10,
                    },
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts14() {
            let that = this;
            let resumeList = JSON.parse(JSON.stringify(that.list.resume.list));
            let xAxisData = [];
            let seriesData = [];
            let seriesName = that.list.resume.name;

            for (let i in resumeList) {
                xAxisData.push(resumeList[i].date);
                seriesData.push(resumeList[i].count);
            }
            var myChart = echarts.init(document.getElementById('main14'));
            var option = {
                color: ['#8997FB', '#23C9C9', '#1890FF'],
                legend: {
                    left: 'right',
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '12%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    data: xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    minInterval: 1,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: [
                    {
                        name: seriesName,
                        data: seriesData,
                        type: 'line',
                        stack: 'total',
                        symbol: 'circle',//拐点样式
                        symbolSize: 10,
                    },
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },

        initCharts15() {
            let that = this;
            let inviteList = JSON.parse(JSON.stringify(that.list.invite.list));
            let xAxisData = [];
            let seriesData = [];
            let seriesName = that.list.invite.name;

            for (let i in inviteList) {
                xAxisData.push(inviteList[i].date);
                seriesData.push(inviteList[i].count);
            }
            var myChart = echarts.init(document.getElementById('main15'));
            var option = {
                color: ['#8997FB', '#23C9C9', '#1890FF'],
                legend: {
                    left: 'right',
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '12%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    data: xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    minInterval: 1,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: [
                    {
                        name: seriesName,
                        data: seriesData,
                        type: 'line',
                        stack: 'total',
                        symbol: 'circle',//拐点样式
                        symbolSize: 10,
                    },
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts15_1() {
            let that = this;
            let data = [];
            if (that.CountTj[0].job1 != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[0].job1));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main15_1'));
            var option = {
                // color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00141'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '42%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts15_2() {
            let that = this;
            let data = [];
            if (that.CountTj[0].provinceid != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[0].provinceid));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main15_2'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00138'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts15_3() {
            let that = this;
            let data = [];
            if (that.CountTj[0].salary != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[0].salary));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main15_3'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00140'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts15_4() {
            let that = this;
            let data = [];
            if (that.CountTj[0].edu != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[0].edu));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main15_4'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00139'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts15_5() {
            let that = this;
            let data = [];
            if (that.CountTj[0].exp != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[0].exp));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main15_5'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00115'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },

        initCharts16() {
            let that = this;
            let downList = JSON.parse(JSON.stringify(that.list.down.list));
            let xAxisData = [];
            let seriesData = [];
            let seriesName = that.list.down.name;

            for (let i in downList) {
                xAxisData.push(downList[i].date);
                seriesData.push(downList[i].count);
            }
            var myChart = echarts.init(document.getElementById('main16'));
            var option = {
                color: ['#8997FB', '#23C9C9', '#1890FF'],
                legend: {
                    left: 'right',
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '12%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    data: xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    minInterval: 1,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: [
                    {
                        name: seriesName,
                        data: seriesData,
                        type: 'line',
                        stack: 'total',
                        symbol: 'circle',//拐点样式
                        symbolSize: 10,
                    },
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts16_1() {
            let that = this;
            let data = [];
            if (that.CountTj[1].job1 != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[1].job1));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main16_1'));
            var option = {
                // color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00141'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '42%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts16_2() {
            let that = this;
            let data = [];
            if (that.CountTj[1].provinceid != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[1].provinceid));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main16_2'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00121'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts16_3() {
            let that = this;
            let data = [];
            if (that.CountTj[1].salary != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[1].salary));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main16_3'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00135'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts16_4() {
            let that = this;
            let data = [];
            if (that.CountTj[1].edu != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[1].edu));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main16_4'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00125'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts16_5() {
            let that = this;
            let data = [];
            if (that.CountTj[1].exp != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[1].exp));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main16_5'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='wap_user_00240'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '38%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts16_6() {
            let that = this;
            let data = [];
            if (that.CountTj[1].sex != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[1].sex));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main16_6'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00133'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },

        initCharts17() {
            let that = this;
            let orderList = JSON.parse(JSON.stringify(that.list.order.list));
            let xAxisData = [];
            let seriesData = [];
            let seriesName = that.list.order.name;

            for (let i in orderList) {
                xAxisData.push(orderList[i].date);
                seriesData.push(orderList[i].count);
            }
            var myChart = echarts.init(document.getElementById('main17'));
            var option = {
                color: ['#8997FB', '#23C9C9', '#1890FF'],
                legend: {
                    left: 'right',
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '12%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    data: xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    minInterval: 1,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: [
                    {
                        name: seriesName,
                        data: seriesData,
                        type: 'line',
                        stack: 'total',
                        symbol: 'circle',//拐点样式
                        symbolSize: 10,
                    },
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts17_1() {
            let that = this;
            let data = [];
            if (that.CountTj.type != undefined) {
                let typeList = JSON.parse(JSON.stringify(that.CountTj.type));
                for (let i in typeList) {
                    data.push({ value: typeList[i].count, name: typeList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main17_1'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='wap_user_00318'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts17_2() {
            let that = this;
            let data = [];
            if (that.CountTj.ordertype != undefined) {
                let orderTypeList = JSON.parse(JSON.stringify(that.CountTj.ordertype));
                for (let i in orderTypeList) {
                    data.push({ value: orderTypeList[i].count, name: orderTypeList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main17_2'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='wap_user_00313'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },

        initCharts18() {
            let that = this;
            let adList = JSON.parse(JSON.stringify(that.list.ad.list));
            let xAxisData = [];
            let seriesData = [];
            let seriesName = that.list.ad.name;

            for (let i in adList) {
                xAxisData.push(adList[i].date);
                seriesData.push(adList[i].count);
            }
            var myChart = echarts.init(document.getElementById('main18'));
            var option = {
                color: ['#8997FB', '#23C9C9', '#1890FF'],
                legend: {
                    left: 'right',
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '12%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    data: xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    minInterval: 1,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: [
                    {
                        name: seriesName,
                        data: seriesData,
                        type: 'line',
                        stack: 'total',
                        symbol: 'circle',//拐点样式
                        symbolSize: 10,
                    },
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },

        initCharts19() {
            let that = this;
            let addJobList = JSON.parse(JSON.stringify(that.list.addJob.list));
            let upJobList = JSON.parse(JSON.stringify(that.list.upJob.list));
            let xAxisData = [];
            for (let i in addJobList) {
                xAxisData.push(addJobList[i].date);
            }
            let seriesArr = [];
            seriesArr.name = [that.list.addJob.name, that.list.upJob.name];
            let oneData = [];
            let twoData = [];
            for (let i in addJobList) {
                oneData.push(addJobList[i].count);
            }
            for (let i in upJobList) {
                twoData.push(upJobList[i].count);
            }
            seriesArr.data = [oneData, twoData]
            var myChart = echarts.init(document.getElementById('main19'));
            var option = {
                title: {
                    textStyle: {
                        color: '#333',
                        fontWeight: '500',
                        fontSize: '16'
                    }
                },
                legend: {
                    left: 'right',
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '12%',
                    left: '2%',
                    right: '3%',
                    bottom: '2%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    data: xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    minInterval: 1,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: [
                    {
                        name: seriesArr.name[0],
                        data: seriesArr.data[0],
                        type: 'bar',
                        color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [
                            {
                                offset: 0,
                                color: '#399EFC'
                            },
                            {
                                offset: 1,
                                color: '#3ED572'
                            }
                        ])
                    },
                    {
                        name: seriesArr.name[1],
                        data: seriesArr.data[1],
                        type: 'bar',
                        color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [
                            {
                                offset: 0,
                                color: '#FEC59A'
                            },
                            {
                                offset: 1,
                                color: '#FEA15C'
                            }
                        ])
                    },
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts19_1() {
            let that = this;
            let data = [];
            if (that.CountTj[0].job1 != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[0].job1));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main19_1'));
            var option = {
                // color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00131'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '42%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts19_2() {
            let that = this;
            let data = [];
            if (that.CountTj[0].provinceid != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[0].provinceid));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main19_2'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00132'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts19_3() {
            let that = this;
            let data = [];
            if (that.CountTj[0].salary != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[0].salary));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main19_3'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='wap_com_00290'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts19_4() {
            let that = this;
            let data = [];
            if (that.CountTj[0].edu != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[0].edu));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main19_4'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='wap_com_00283'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts19_5() {
            let that = this;
            let data = [];
            if (that.CountTj[0].exp != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[0].exp));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main19_5'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='wap_user_00240'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },

        initCharts20() {
            let that = this;
            let addResumeList = JSON.parse(JSON.stringify(that.list.addResume.list));
            let upResumeList = JSON.parse(JSON.stringify(that.list.upResume.list));
            let xAxisData = [];
            for (let i in addResumeList) {
                xAxisData.push(addResumeList[i].date);
            }
            let seriesArr = [];
            seriesArr.name = [that.list.addResume.name, that.list.upResume.name];
            let oneData = [];
            let twoData = [];
            for (let i in addResumeList) {
                oneData.push(addResumeList[i].count);
            }
            for (let i in upResumeList) {
                twoData.push(upResumeList[i].count);
            }
            seriesArr.data = [oneData, twoData]
            var myChart = echarts.init(document.getElementById('main20'));
            var option = {
                title: {
                    textStyle: {
                        color: '#333',
                        fontWeight: '500',
                        fontSize: '16'
                    }
                },
                legend: {
                    left: 'right',
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '12%',
                    left: '2%',
                    right: '3%',
                    bottom: '2%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    data: xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    minInterval: 1,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: [
                    {
                        name: seriesArr.name[0],
                        data: seriesArr.data[0],
                        type: 'bar',
                        color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [
                            {
                                offset: 0,
                                color: '#399EFC'
                            },
                            {
                                offset: 1,
                                color: '#3ED572'
                            }
                        ])
                    },
                    {
                        name: seriesArr.name[1],
                        data: seriesArr.data[1],
                        type: 'bar',
                        color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [
                            {
                                offset: 0,
                                color: '#FEC59A'
                            },
                            {
                                offset: 1,
                                color: '#FEA15C'
                            }
                        ])
                    },
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts20_1() {
            let that = this;
            let data = [];
            if (that.CountTj[1].job1 != undefined) {
                let jobList = JSON.parse(JSON.stringify(that.CountTj[1].job1));
                for (let i in jobList) {
                    data.push({ value: jobList[i].count, name: jobList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main20_1'));
            var option = {
                // color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00129'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '42%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts20_2() {
            let that = this;
            let data = [];
            if (that.CountTj[1].provinceid != undefined) {
                let cityList = JSON.parse(JSON.stringify(that.CountTj[1].provinceid));
                for (let i in cityList) {
                    data.push({ value: cityList[i].count, name: cityList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main20_2'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00127'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts20_3() {
            let that = this;
            let data = [];
            if (that.CountTj[1].salary != undefined) {
                let salaryList = JSON.parse(JSON.stringify(that.CountTj[1].salary));
                for (let i in salaryList) {
                    data.push({ value: salaryList[i].count, name: salaryList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main20_3'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='wap_user_00016'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts20_4() {
            let that = this;
            let data = [];
            if (that.CountTj[1].edu != undefined) {
                let eduList = JSON.parse(JSON.stringify(that.CountTj[1].edu));
                for (let i in eduList) {
                    data.push({ value: eduList[i].count, name: eduList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main20_4'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='member_com_00011'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts20_5() {
            let that = this;
            let data = [];
            if (that.CountTj[1].exp != undefined) {
                let expList = JSON.parse(JSON.stringify(that.CountTj[1].exp));
                for (let i in expList) {
                    data.push({ value: expList[i].count, name: expList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main20_5'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='wap_user_00240'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts20_6() {
            let that = this;
            let data = [];
            if (that.CountTj[1].sex != undefined) {
                let sexList = JSON.parse(JSON.stringify(that.CountTj[1].sex));
                for (let i in sexList) {
                    data.push({ value: sexList[i].count, name: sexList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main20_6'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00133'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts20_7() {
            let that = this;
            let data = [];
            if (that.CountTj[1].source != undefined) {
                let sourceList = JSON.parse(JSON.stringify(that.CountTj[1].source));
                for (let i in sourceList) {
                    data.push({ value: sourceList[i].count, name: sourceList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main20_7'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_yunying_00139'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '42%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },

        initCharts21() {
            let that = this;
            let oneList = JSON.parse(JSON.stringify(that.list.comOne.list));
            let twoList = JSON.parse(JSON.stringify(that.list.comTwo.list));
            let xAxisData = [];
            for (let i in oneList) {
                xAxisData.push(oneList[i].date);
            }
            let seriesArr = [];
            seriesArr.name = [that.list.comOne.name, that.list.comTwo.name];
            let oneData = [];
            let twoData = [];
            for (let i in oneList) {
                oneData.push(oneList[i].count);
            }
            for (let i in twoList) {
                twoData.push(twoList[i].count);
            }
            seriesArr.data = [oneData, twoData]
            var myChart = echarts.init(document.getElementById('main21'));
            var option = {
                title: {
                    textStyle: {
                        color: '#333',
                        fontWeight: '500',
                        fontSize: '16'
                    }
                },
                legend: {
                    left: 'right',
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '12%',
                    left: '2%',
                    right: '3%',
                    bottom: '2%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    data: xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    minInterval: 1,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: [
                    {
                        name: seriesArr.name[0],
                        data: seriesArr.data[0],
                        type: 'bar',
                        color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [
                            {
                                offset: 0,
                                color: '#399EFC'
                            },
                            {
                                offset: 1,
                                color: '#3ED572'
                            }
                        ])
                    },
                    {
                        name: seriesArr.name[1],
                        data: seriesArr.data[1],
                        type: 'bar',
                        color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [
                            {
                                offset: 0,
                                color: '#FEC59A'
                            },
                            {
                                offset: 1,
                                color: '#FEA15C'
                            }
                        ])
                    },
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts21_1() {
            let that = this;
            let data = [];
            if (that.CountTj[0].hy != undefined) {
                let hyList = JSON.parse(JSON.stringify(that.CountTj[0].hy));
                for (let i in hyList) {
                    data.push({ value: hyList[i].count, name: hyList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main21_1'));
            var option = {
                // color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00136'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts21_2() {
            let that = this;
            let data = [];
            if (that.CountTj[0].rating != undefined) {
                let ratingList = JSON.parse(JSON.stringify(that.CountTj[0].rating));
                for (let i in ratingList) {
                    data.push({ value: ratingList[i].count, name: ratingList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main21_2'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00157'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts21_3() {
            let that = this;
            let data = [];
            if (that.CountTj[0].is != undefined) {
                let isList = JSON.parse(JSON.stringify(that.CountTj[0].is));
                for (let i in isList) {
                    data.push({ value: isList[i].count, name: isList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main21_3'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00124'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },

        initCharts22() {
            let that = this;
            let applyList = JSON.parse(JSON.stringify(that.list.apply.list));
            let xAxisData = [];
            let seriesData = [];
            let seriesName = that.list.apply.name;

            for (let i in applyList) {
                xAxisData.push(applyList[i].date);
                seriesData.push(applyList[i].count);
            }
            var myChart = echarts.init(document.getElementById('main22'));
            var option = {
                color: ['#8997FB', '#23C9C9', '#1890FF'],
                legend: {
                    left: 'right',
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '12%',
                    left: '2%',
                    right: '2%',
                    bottom: '2%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    data: xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    minInterval: 1,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: [
                    {
                        name: seriesName,
                        data: seriesData,
                        type: 'line',
                        stack: 'total',
                        symbol: 'circle',//拐点样式
                        symbolSize: 10,
                    },
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts22_1() {
            let that = this;
            let data = [];
            if (that.CountTj[1].job1 != undefined) {
                let jobList = JSON.parse(JSON.stringify(that.CountTj[1].job1));
                for (let i in jobList) {
                    data.push({ value: jobList[i].count, name: jobList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main22_1'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00156'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts22_2() {
            let that = this;
            let data = [];
            if (that.CountTj[1].provinceid != undefined) {
                let cityList = JSON.parse(JSON.stringify(that.CountTj[1].provinceid));
                for (let i in cityList) {
                    data.push({ value: cityList[i].count, name: cityList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main22_2'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00155'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts22_3() {
            let that = this;
            let data = [];
            if (that.CountTj[1].salary != undefined) {
                let salaryList = JSON.parse(JSON.stringify(that.CountTj[1].salary));
                for (let i in salaryList) {
                    data.push({ value: salaryList[i].count, name: salaryList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main22_3'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='wap_user_00016'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts22_4() {
            let that = this;
            let data = [];
            if (that.CountTj[1].edu != undefined) {
                let eduList = JSON.parse(JSON.stringify(that.CountTj[1].edu));
                for (let i in eduList) {
                    data.push({ value: eduList[i].count, name: eduList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main22_4'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='member_com_00011'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts22_5() {
            let that = this;
            let data = [];
            if (that.CountTj[1].exp != undefined) {
                let expList = JSON.parse(JSON.stringify(that.CountTj[1].exp));
                for (let i in expList) {
                    data.push({ value: expList[i].count, name: expList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main22_5'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='wap_user_00240'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        initCharts22_6() {
            let that = this;
            let data = [];
            if (that.CountTj[1].sex != undefined) {
                let sexList = JSON.parse(JSON.stringify(that.CountTj[1].sex));
                for (let i in sexList) {
                    data.push({ value: sexList[i].count, name: sexList[i].name });
                }
            }
            var myChart = echarts.init(document.getElementById('main22_6'));
            var option = {
                color: ['#2659F0', '#73A0FB', '#B0CCFE', '#FFAD35', '#FFD454'],
                tooltip: {
                    trigger: 'item',
                    formatter: "{a} <br/>{b}: {c} ({d}%)"
                },
                legend: {
                    left: 'center',
                    top: 'bottom',
                    icon: 'circle',
                },
                grid: {
                    top: '0%',
                    left: '2%',
                    right: '2%',
                    bottom: '5%',
                    containLabel: true
                },
                series: [
                    {
                        name: window.yunAdminT("{yun:}t key='admin_tool_00133'{/yun}"),
                        type: 'pie',
                        radius: ['42%', '65%'],
                        center: ['50%', '50%'],
                        avoidLabelOverlap: false,
                        data: data
                    }
                ]
            };
            myChart.setOption(option);
            setTimeout(function () {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },

        toPage: function (type) {

            window.parent.homeapp.topage({
                name: type,
                params:{
                    day: this.search.day,
                    time: this.search.time
                }
            });
        },
        showComBox: function (type) {
            let _this = this;
            _this.jumpParams = {};
            if (type == 1){

                _this.jumpParams.reg_days = _this.search.days;
                _this.jumpParams.reg_time = _this.search.time;
            }else if (type == 2){

                _this.jumpParams.login_days = _this.search.days;
                _this.jumpParams.login_time = _this.search.time;
            }

            _this.jumpParams.search_class = 'drawer';
            _this.comBoxShow = true;
        },
        showUserBox: function (type) {
            let _this = this;
            _this.jumpParams = {};
            if (type == 1){

                _this.jumpParams.reg_days = _this.search.days;
                _this.jumpParams.reg_time = _this.search.time;
            }else if (type == 2){

                _this.jumpParams.login_days = _this.search.days;
                _this.jumpParams.login_time = _this.search.time;
            }
            _this.jumpParams.search_class = 'drawer';
            _this.userBoxShow = true;
        },
        openPage: function (id, type) {
            let url = '';
            if (type == 'job') {
                url = this.webUrl + '/index.php?m=job&c=comapply&id=' + id;
            } else if (type == 'resume') {
                url = this.webUrl + '/index.php?m=resume&c=show&id=' + id;
            } else if (type == 'company') {
                url = this.webUrl + '/index.php?m=company&c=show&id=' + id;
            }
            window.open(url, '_blank');
        }
    },
};
</script>
<style scoped>
    .yejiTabHeaButn span{
        cursor: pointer;
    }
</style>