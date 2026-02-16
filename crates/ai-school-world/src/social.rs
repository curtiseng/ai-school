//! M2.2 社交与校园活动
//!
//! 社团模型、社交事件模板。

use serde::{Deserialize, Serialize};

use ai_school_core::types::{AgentId, LocationId};

/// 社团定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Club {
    pub name: String,
    pub description: String,
    pub location: LocationId,
    pub members: Vec<AgentId>,
    pub max_members: usize,
    /// 活动频率（每周次数）
    pub activity_frequency: u32,
}

/// 创建默认社团列表
pub fn create_default_clubs() -> Vec<Club> {
    vec![
        Club {
            name: "编程社".to_string(),
            description: "学习编程和计算机科学".to_string(),
            location: LocationId("classroom_science".to_string()),
            members: Vec::new(),
            max_members: 15,
            activity_frequency: 2,
        },
        Club {
            name: "文学社".to_string(),
            description: "阅读和创作文学作品".to_string(),
            location: LocationId("library".to_string()),
            members: Vec::new(),
            max_members: 20,
            activity_frequency: 2,
        },
        Club {
            name: "篮球队".to_string(),
            description: "篮球训练和比赛".to_string(),
            location: LocationId("playground".to_string()),
            members: Vec::new(),
            max_members: 12,
            activity_frequency: 3,
        },
        Club {
            name: "辩论社".to_string(),
            description: "锻炼逻辑思维和表达能力".to_string(),
            location: LocationId("club_room".to_string()),
            members: Vec::new(),
            max_members: 16,
            activity_frequency: 2,
        },
        Club {
            name: "美术社".to_string(),
            description: "绘画和艺术创作".to_string(),
            location: LocationId("club_room".to_string()),
            members: Vec::new(),
            max_members: 15,
            activity_frequency: 2,
        },
    ]
}

/// 社交事件模板
#[derive(Debug, Clone)]
pub struct SocialEventTemplate {
    pub name: String,
    pub description: String,
    pub min_participants: usize,
    pub max_participants: usize,
    pub location: LocationId,
}

/// 预定义社交事件模板
pub fn social_event_templates() -> Vec<SocialEventTemplate> {
    vec![
        SocialEventTemplate {
            name: "小组讨论".to_string(),
            description: "几个同学围坐在一起讨论某个话题".to_string(),
            min_participants: 2,
            max_participants: 5,
            location: LocationId("rest_area".to_string()),
        },
        SocialEventTemplate {
            name: "食堂聚餐".to_string(),
            description: "一群朋友在食堂一起吃饭聊天".to_string(),
            min_participants: 2,
            max_participants: 6,
            location: LocationId("cafeteria".to_string()),
        },
        SocialEventTemplate {
            name: "操场运动".to_string(),
            description: "几个同学在操场一起运动".to_string(),
            min_participants: 2,
            max_participants: 10,
            location: LocationId("playground".to_string()),
        },
    ]
}
