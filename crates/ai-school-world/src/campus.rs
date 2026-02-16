//! 校园空间模型
//!
//! 定义校园功能区域、Agent 位置追踪、空间邻接关系。

use ai_school_core::types::{Location, LocationId, LocationType};

/// 创建默认校园布局
pub fn create_default_campus() -> Vec<Location> {
    vec![
        Location {
            id: LocationId("classroom_math".to_string()),
            name: "数学教室".to_string(),
            location_type: LocationType::Classroom { subject: Some("数学".to_string()) },
            capacity: 40,
            position: (200.0, 100.0),
            adjacent: vec![
                LocationId("classroom_chinese".to_string()),
                LocationId("hallway".to_string()),
            ],
        },
        Location {
            id: LocationId("classroom_chinese".to_string()),
            name: "语文教室".to_string(),
            location_type: LocationType::Classroom { subject: Some("语文".to_string()) },
            capacity: 40,
            position: (350.0, 100.0),
            adjacent: vec![
                LocationId("classroom_math".to_string()),
                LocationId("classroom_english".to_string()),
                LocationId("hallway".to_string()),
            ],
        },
        Location {
            id: LocationId("classroom_english".to_string()),
            name: "英语教室".to_string(),
            location_type: LocationType::Classroom { subject: Some("英语".to_string()) },
            capacity: 40,
            position: (500.0, 100.0),
            adjacent: vec![
                LocationId("classroom_chinese".to_string()),
                LocationId("hallway".to_string()),
            ],
        },
        Location {
            id: LocationId("classroom_science".to_string()),
            name: "理科实验室".to_string(),
            location_type: LocationType::Classroom { subject: Some("科学".to_string()) },
            capacity: 30,
            position: (200.0, 200.0),
            adjacent: vec![LocationId("hallway".to_string())],
        },
        Location {
            id: LocationId("library".to_string()),
            name: "图书馆".to_string(),
            location_type: LocationType::Library,
            capacity: 50,
            position: (650.0, 150.0),
            adjacent: vec![
                LocationId("hallway".to_string()),
                LocationId("study_room".to_string()),
            ],
        },
        Location {
            id: LocationId("study_room".to_string()),
            name: "自习室".to_string(),
            location_type: LocationType::StudyRoom,
            capacity: 20,
            position: (650.0, 250.0),
            adjacent: vec![LocationId("library".to_string()), LocationId("hallway".to_string())],
        },
        Location {
            id: LocationId("playground".to_string()),
            name: "操场".to_string(),
            location_type: LocationType::Playground,
            capacity: 200,
            position: (400.0, 400.0),
            adjacent: vec![
                LocationId("hallway".to_string()),
                LocationId("cafeteria".to_string()),
            ],
        },
        Location {
            id: LocationId("cafeteria".to_string()),
            name: "食堂".to_string(),
            location_type: LocationType::Cafeteria,
            capacity: 100,
            position: (200.0, 400.0),
            adjacent: vec![
                LocationId("playground".to_string()),
                LocationId("hallway".to_string()),
            ],
        },
        Location {
            id: LocationId("dormitory".to_string()),
            name: "宿舍".to_string(),
            location_type: LocationType::Dormitory,
            capacity: 100,
            position: (100.0, 500.0),
            adjacent: vec![LocationId("hallway".to_string())],
        },
        Location {
            id: LocationId("club_room".to_string()),
            name: "社团活动室".to_string(),
            location_type: LocationType::ClubRoom { club_name: None },
            capacity: 30,
            position: (500.0, 300.0),
            adjacent: vec![LocationId("hallway".to_string()), LocationId("auditorium".to_string())],
        },
        Location {
            id: LocationId("auditorium".to_string()),
            name: "礼堂".to_string(),
            location_type: LocationType::Auditorium,
            capacity: 300,
            position: (650.0, 350.0),
            adjacent: vec![LocationId("club_room".to_string()), LocationId("hallway".to_string())],
        },
        Location {
            id: LocationId("rest_area".to_string()),
            name: "休息区".to_string(),
            location_type: LocationType::RestArea,
            capacity: 30,
            position: (350.0, 300.0),
            adjacent: vec![LocationId("hallway".to_string())],
        },
        Location {
            id: LocationId("hallway".to_string()),
            name: "走廊".to_string(),
            location_type: LocationType::RestArea,
            capacity: 500,
            position: (350.0, 200.0),
            adjacent: vec![
                LocationId("classroom_math".to_string()),
                LocationId("classroom_chinese".to_string()),
                LocationId("classroom_english".to_string()),
                LocationId("classroom_science".to_string()),
                LocationId("library".to_string()),
                LocationId("study_room".to_string()),
                LocationId("playground".to_string()),
                LocationId("cafeteria".to_string()),
                LocationId("dormitory".to_string()),
                LocationId("club_room".to_string()),
                LocationId("auditorium".to_string()),
                LocationId("rest_area".to_string()),
            ],
        },
    ]
}

/// 检查两个位置是否相邻
pub fn are_adjacent(locations: &[Location], from: &LocationId, to: &LocationId) -> bool {
    locations
        .iter()
        .find(|l| &l.id == from)
        .map(|l| l.adjacent.contains(to))
        .unwrap_or(false)
}
