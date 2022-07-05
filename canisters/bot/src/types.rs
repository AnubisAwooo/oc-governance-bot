use ic_cdk::export::{
    candid::CandidType,
    serde::Deserialize
};

// 枚举 canister 的创建状态
#[derive(CandidType, Deserialize, Debug)]
pub enum CanisterCreationStatus {
    Pending, // 等待
    InProgress, // 创建中
    Created // 已创建
}

// ? 枚举确认状态，不知道什么的确认状态？
#[derive(CandidType, Deserialize, Debug)]
pub enum ConfirmationState {
    PhoneNumber(PhoneNumber), // 手机号
    RegistrationFee(CurrentUserResponseUnconfirmedStateRegistrationFee) // 注册费？
}

// ? 已确认等待中用户名
#[derive(CandidType, Deserialize, Debug)]
pub struct ConfirmedPendingUsername {
    canister_creation_status: CanisterCreationStatus, // canister 创建床头
    confirmation_state: ConfirmationState // 确认状态
}

// ========================= 创建 Canister =========================

// 创建 canister 的参数
#[derive(CandidType, Deserialize)]
pub struct CreateCanisterArgs {}

// 枚举 创建 canister 的响应
#[derive(CandidType, Deserialize, Debug)]
pub enum CreateCanisterResponse {
    Success(ic_cdk::export::Principal), // 成功，携带 Principal ID
    UserNotFound, // 用户未发现
    UserUnconfirmed, // 用户未确认
    UserAlreadyCreated, // 用户已创建
    CreationInProgress, // canister 创建中
    CyclesBalanceTooLow, // cycles 余额不足
    InternalError(String) // 内部错误
}

// ========================= 当前用户 =========================

// 当前用户参数
#[derive(CandidType, Deserialize)]
pub struct CurrentUserArgs {}

// ? 枚举 当前用户响应体
#[derive(CandidType, Deserialize, Debug)]
pub enum CurrentUserResponse {
    UserNotFound, // 用户未发现
    Unconfirmed(CurrentUserResponseUnconfirmed), // 未确认
    ConfirmedPendingUsername(ConfirmedPendingUsername), // 已确认等待中用户名
    Confirmed(CurrentUserResponseConfirmed), // 已确认
    Created(CurrentUserResponseCreated) // 已创建
}

// 当前用户已确认响应
#[derive(CandidType, Deserialize, Debug)]
pub struct CurrentUserResponseConfirmed {
    username: String
}

// ? 当前用户响应已创建
#[derive(CandidType, Deserialize, Debug)]
pub struct CurrentUserResponseCreated {
    pub user_id: ic_cdk::export::Principal
}

// 当前用户响应未确认
#[derive(CandidType, Deserialize, Debug)]
pub struct CurrentUserResponseUnconfirmed {
    state: CurrentUserResponseUnconfirmedState
}

// 枚举 当前用户响应未确认状态
#[derive(CandidType, Deserialize, Debug)]
pub enum CurrentUserResponseUnconfirmedState {
    PhoneNumber(CurrentUserResponseUnconfirmedStatePhoneNumber), // 手机号
    RegistrationFee(CurrentUserResponseUnconfirmedStateRegistrationFee) // 注册费用
}

// 当前用户响应未确认状态手机号
#[derive(CandidType, Deserialize, Debug)]
pub struct CurrentUserResponseUnconfirmedStatePhoneNumber {
    phone_number: PhoneNumber,
    valid_until: u64
}

// ? 当前用户响应未确认状态注册费用
#[derive(CandidType, Deserialize, Debug)]
pub enum CurrentUserResponseUnconfirmedStateRegistrationFee {
    ICP(ICPRegistrationFee), // ICP 费用？
    Cycles(CyclesRegistrationFee) // cycles 费用？
}

// cycles 注册费
#[derive(CandidType, Deserialize, Debug)]
pub struct CyclesRegistrationFee {
    valid_until: u64
}

// 群聊简介
#[derive(CandidType, Deserialize, Debug)]
pub struct GroupChatSummary {
    name: String
}

// 群恢复内容
#[derive(CandidType, Deserialize)]
pub struct GroupReplyContext {
    event_index: u32
}

// 注册费用 ICP
#[derive(CandidType, Deserialize, Debug)]
pub struct ICPRegistrationFee {
    valid_until: u64
}

// ========================= 加群 =========================

// 加群参数
#[derive(CandidType, Deserialize, Debug)]
pub struct JoinGroupArgs {
    pub chat_id: ic_cdk::export::Principal, // 群 id
    pub as_super_admin: bool // 是否超级管理员
}

// 加群响应
#[derive(CandidType, Deserialize, Debug)]
pub enum JoinGroupResponse {
    Success(GroupChatSummary), // 成功
    AlreadyInGroup, // 已经入群
    Blocked, // 阻塞？
    GroupNotFound, // 未发现群
    GroupNotPublic, // 群不是公开的
    ParticipantLimitReached(u32), // 加群受限
    InternalError(String), // 内部错误
    NotSuperAdmin // 非超级管理员
}

// ========================= 列出提案 =========================

// 列出提案信息
#[derive(CandidType, Deserialize)]
pub struct ListProposalInfo {
    pub include_reward_status: Vec<i32>, // 包含奖励状态 ？
    pub before_proposal: Option<NeuronId>, // 之前提案？
    pub limit: u32, // 限制？
    pub exclude_topic: Vec<i32>, // 排除主题
    pub include_status: Vec<i32> // 包含状态
}

// 列出提案响应
#[derive(CandidType, Deserialize)]
pub struct ListProposalInfoResponse {
    pub proposal_info: Vec<ProposalInfo>
}

// 枚举 消息内容
#[derive(CandidType, Deserialize)]
pub enum MessageContent {
    Text(TextContent) // 文本内容
}

// 神经元 id  是个 64 位的数字吗？
#[derive(CandidType, Deserialize, Clone)]
pub struct NeuronId {
    pub id: u64
}

// 手机号
#[derive(CandidType, Deserialize, Debug)]
pub struct PhoneNumber {
    country_code: u16, // 国别码
    number: String // 手机号
}

// ========================= 提案 =========================

// 提案结构体
#[derive(CandidType, Deserialize, Clone)]
pub struct Proposal {
    pub url: String, // url
    pub title: Option<String>, // 标题
    pub summary: String // 提案简介
}

// 提案信息
#[derive(CandidType, Deserialize, Clone)]
pub struct ProposalInfo {
    pub id: Option<NeuronId>, // id 应该是数字，不应该是 NeuronId 才是
    pub topic: i32, // 提案主题
    pub proposal: Option<Proposal> // 提案内容
}

// ========================= 注册用户 =========================

// 注册用户参数
#[derive(CandidType, Deserialize)]
pub struct RegisterUserArgs {
    pub username: String
}

// 注册用户响应
#[derive(CandidType, Deserialize, Debug)]
pub enum RegisterUserResponse {
    Success,
    AlreadyRegistered,
    UserLimitReached,
    UsernameTaken,
    UsernameInvalid,
    UsernameTooShort(u16),
    UsernameTooLong(u16),
    NotSupported
}

// ========================= 发送消息 =========================

// 发送消息参数
#[derive(CandidType, Deserialize)]
pub struct SendMessageArgs {
    pub message_id: u128, // 消息 id
    pub content: MessageContent, // 消息内容
    pub sender_name: String, // 发送名称
    pub replies_to: Option<GroupReplyContext>, // ? 回复内容
    pub mentioned: Vec<User> // 被@的用户
}

// 枚举 发送消息响应
#[derive(CandidType, Deserialize, Debug)]
pub enum SendMessageResponse {
    Success(SendMessageResponseSuccess), // 成功
    MessageEmpty, // 空消息体
    TextTooLong(u32), // 文本太长
    CallerNotInGroup // 调用者不在群组
}

// 发送消息响应成功
#[derive(CandidType, Deserialize, Debug)]
pub struct SendMessageResponseSuccess {
    message_index: u32, // 消息序号
    event_index: u32, // 时间序号
    timestamp: u64 // 时间戳
}




// 文本内容
#[derive(CandidType, Deserialize)]
pub struct TextContent {
    pub text: String // 文本
}

// 用户结构体
#[derive(CandidType, Deserialize)]
pub struct User {
    pub user_id: ic_cdk::export::Principal, // 用户 id
    pub username: String // 用户名称
}