//! M2.3 环境与时间系统
//!
//! 仿真时钟推进、学期/周/天/时段层次时间。

use ai_school_core::types::SimulationTime;

/// 仿真时钟
pub struct SimulationClock {
    current: SimulationTime,
    /// 每步推进的小时数
    step_hours: u32,
}

impl SimulationClock {
    pub fn new(step_hours: u32) -> Self {
        Self {
            current: SimulationTime::new(),
            step_hours,
        }
    }

    pub fn current_time(&self) -> &SimulationTime {
        &self.current
    }

    /// 推进一步，返回触发的时间事件
    pub fn advance(&mut self) -> Vec<TimeEvent> {
        let mut events = Vec::new();
        let old_hour = self.current.hour;
        let _old_day = self.current.day_of_week;

        self.current.tick += 1;
        self.current.hour += self.step_hours;

        // 处理日期进位
        if self.current.hour >= 24 {
            self.current.hour -= 24;
            self.current.day_of_week += 1;
            events.push(TimeEvent::NewDay);

            if self.current.day_of_week > 7 {
                self.current.day_of_week = 1;
                self.current.week += 1;
                events.push(TimeEvent::NewWeek);

                if self.current.week > 20 {
                    self.current.week = 1;
                    self.current.semester += 1;
                    events.push(TimeEvent::NewSemester);
                }
            }

            if self.current.day_of_week > 5 {
                events.push(TimeEvent::Weekend);
            }
        }

        // 课程表时间事件
        if self.current.day_of_week <= 5 {
            for h in old_hour..self.current.hour + self.step_hours {
                match h {
                    8 => events.push(TimeEvent::ClassStart { period: 1 }),
                    9 => events.push(TimeEvent::ClassStart { period: 2 }),
                    10 => events.push(TimeEvent::Break),
                    11 => events.push(TimeEvent::ClassStart { period: 3 }),
                    12 => events.push(TimeEvent::LunchBreak),
                    14 => events.push(TimeEvent::ClassStart { period: 4 }),
                    15 => events.push(TimeEvent::ClassStart { period: 5 }),
                    16 => events.push(TimeEvent::FreeTime),
                    18 => events.push(TimeEvent::Dinner),
                    19 => events.push(TimeEvent::EveningStudy),
                    22 => events.push(TimeEvent::Bedtime),
                    _ => {}
                }
            }
        }

        events
    }

    /// 获取当前时间段描述
    pub fn current_period_description(&self) -> &str {
        if self.current.day_of_week > 5 {
            return "周末自由时间";
        }

        match self.current.hour {
            0..=6 => "睡觉时间",
            7 => "起床 & 早餐",
            8..=9 => "上午课程",
            10 => "课间休息",
            11 => "上午课程",
            12..=13 => "午餐 & 午休",
            14..=15 => "下午课程",
            16..=17 => "课外活动/自由时间",
            18 => "晚餐",
            19..=21 => "晚自习",
            22..=23 => "就寝",
            _ => "未知时段",
        }
    }

    /// 重置时钟
    pub fn reset(&mut self) {
        self.current = SimulationTime::new();
    }
}

/// 时间事件
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeEvent {
    ClassStart { period: u32 },
    Break,
    LunchBreak,
    FreeTime,
    Dinner,
    EveningStudy,
    Bedtime,
    NewDay,
    NewWeek,
    NewSemester,
    Weekend,
}

impl TimeEvent {
    pub fn description(&self) -> &str {
        match self {
            TimeEvent::ClassStart { period } => match period {
                1 => "第一节课开始",
                2 => "第二节课开始",
                3 => "第三节课开始",
                4 => "第四节课开始",
                5 => "第五节课开始",
                _ => "课程开始",
            },
            TimeEvent::Break => "课间休息",
            TimeEvent::LunchBreak => "午餐时间",
            TimeEvent::FreeTime => "课外活动/自由时间",
            TimeEvent::Dinner => "晚餐时间",
            TimeEvent::EveningStudy => "晚自习开始",
            TimeEvent::Bedtime => "就寝时间",
            TimeEvent::NewDay => "新的一天",
            TimeEvent::NewWeek => "新的一周",
            TimeEvent::NewSemester => "新学期",
            TimeEvent::Weekend => "周末",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_advance() {
        let mut clock = SimulationClock::new(1);
        assert_eq!(clock.current_time().hour, 8);

        let events = clock.advance();
        assert_eq!(clock.current_time().hour, 9);
        // Should trigger ClassStart for period 2
        assert!(events.iter().any(|e| matches!(e, TimeEvent::ClassStart { period: 2 })));
    }

    #[test]
    fn test_day_rollover() {
        let mut clock = SimulationClock::new(1);
        // Advance to end of day
        for _ in 0..16 {
            clock.advance();
        }
        // hour should have wrapped around
        assert!(clock.current_time().hour < 24);
    }
}
