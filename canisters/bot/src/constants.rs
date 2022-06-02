// 机器人名称
// 发送消息到群时需要
// 注册机器人用户需要
// 发送消息
pub const GOVERNANCE_BOT_USER_NAME: &str = "GovernanceBot";

// ? 明明有用到，怎么非提示没用到？？
// 治理机器人加入群聊使用
// ? 不知道这个 id 是怎么来的
pub const GOVERNANCE_BOT_USER_CANISTER_ID: &str = "edwiy-ryaaa-aaaaf-abu7q-cai";
// 提案的 canister id，调用其方法时使用
pub const GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";
// ? 明明有用到，怎么非提示没用到？？
// 用户索引 canister id？
// 盲猜是 OpenChat 存储用户列表的 canister
// 注册机器人用户
// 创建机器人用户的 canister，这个应该会生成一个 canister id，那么和上面的 GOVERNANCE_BOT_USER_CANISTER_ID 是什么关系？
// 获取机器人用户的 canister id
pub const USER_INDEX_CANISTER_ID: &str = "4bkt6-4aaaa-aaaaf-aaaiq-cai";

// ? 不知道这些 TOPIC 从哪里来的
// 根据 id 映射对应的 canister_id
pub const TOPIC_0_GROUP_CANISTER_ID: &str = "7l7zi-kqaaa-aaaaf-abrtq-cai";
pub const TOPIC_1_GROUP_CANISTER_ID: &str = "6gr5g-fyaaa-aaaaf-abrua-cai";
pub const TOPIC_2_GROUP_CANISTER_ID: &str = "6bq3s-iaaaa-aaaaf-abruq-cai";
pub const TOPIC_3_GROUP_CANISTER_ID: &str = "kcmzb-zaaaa-aaaaf-abscq-cai";
pub const TOPIC_4_GROUP_CANISTER_ID: &str = "klps5-piaaa-aaaaf-absda-cai";
pub const TOPIC_5_GROUP_CANISTER_ID: &str = "kmouj-cqaaa-aaaaf-absdq-cai";
pub const TOPIC_6_GROUP_CANISTER_ID: &str = "nkjgq-xiaaa-aaaaf-absta-cai";
pub const TOPIC_7_GROUP_CANISTER_ID: &str = "evgax-iiaaa-aaaaf-abtba-cai";
pub const TOPIC_8_GROUP_CANISTER_ID: &str = "glmx5-dyaaa-aaaaf-abutq-cai";
pub const TOPIC_9_GROUP_CANISTER_ID: &str = "hgctt-mqaaa-aaaaf-abuua-cai";
pub const TOPIC_10_GROUP_CANISTER_ID: &str = "hbdvh-biaaa-aaaaf-abuuq-cai";

pub const ONE_SECOND: u64 = 1000000000; // 1秒 单位纳秒
pub const ONE_MINUTE: u64 = 60 * ONE_SECOND;
pub const FIVE_MINUTE: u64 = 5 * ONE_MINUTE;

pub const PROCESS_INTERVAL: u64 = FIVE_MINUTE; // 执行间隔 5 分钟

pub const INITIAL_PROPOSAL_LIMIT: u32 = 1; // ? 初始化时提案数量？