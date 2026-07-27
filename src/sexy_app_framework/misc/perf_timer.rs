// [TRANSLATION_NOTE]: PerfTimer.h + PerfTimer.cpp -> Rust
// 性能计时器。C++ 版本基于 SDL 高性能计数器，Rust 版本使用 std::time::Instant

use std::collections::BTreeSet;
use std::time::Instant;
use std::sync::Mutex;

// ============================================================
// PerfTimer
// ============================================================
pub struct PerfTimer {
    m_start: Instant,
    m_duration: f64,
    m_running: bool,
}

impl PerfTimer {
    pub fn new() -> Self {
        PerfTimer {
            m_start: Instant::now(),
            m_duration: 0.0,
            m_running: false,
        }
    }

    fn calc_duration(&mut self) {
        let an_end = Instant::now();
        self.m_duration = an_end.duration_since(self.m_start).as_secs_f64() * 1000.0;
    }

    pub fn start(&mut self) {
        self.m_running = true;
        self.m_start = Instant::now();
    }

    pub fn stop(&mut self) {
        if self.m_running {
            self.calc_duration();
            self.m_running = false;
        }
    }

    pub fn set_start_time(&mut self, the_time_milliseconds_ago: i32) {
        self.m_start = Instant::now() - std::time::Duration::from_millis(the_time_milliseconds_ago as u64);
        self.m_running = true;
    }

    pub fn get_duration(&mut self) -> f64 {
        if self.m_running {
            self.calc_duration();
        }
        self.m_duration
    }

    pub fn get_cpu_speed() -> i64 {
        1
    }

    pub fn get_cpu_speed_mhz() -> i32 {
        0
    }
}

// ============================================================
// PerfInfo
// ============================================================
#[derive(Clone)]
struct PerfInfo {
    m_perf_name: String,
    m_start_time: Instant,
    m_duration_ns: i64,
    m_longest_call_ns: i64,
    m_start_count: i32,
    m_call_count: i32,
}

impl PerfInfo {
    fn new(the_name: &str) -> Self {
        PerfInfo {
            m_perf_name: the_name.to_string(),
            m_start_time: Instant::now(),
            m_duration_ns: 0,
            m_longest_call_ns: 0,
            m_start_count: 0,
            m_call_count: 0,
        }
    }
}

impl PartialEq for PerfInfo {
    fn eq(&self, other: &Self) -> bool {
        self.m_perf_name.eq_ignore_ascii_case(&other.m_perf_name)
    }
}

impl Eq for PerfInfo {}

impl PartialOrd for PerfInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PerfInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.m_perf_name.to_ascii_lowercase().cmp(&other.m_perf_name.to_ascii_lowercase())
    }
}

// ============================================================
// PerfRecord
// ============================================================
struct PerfRecord {
    m_name: String,
    m_time: Instant,
    m_start: bool,
}

impl PerfRecord {
    fn new(the_name: &str, start: bool) -> Self {
        PerfRecord {
            m_name: the_name.to_string(),
            m_time: Instant::now(),
            m_start: start,
        }
    }
}

// ============================================================
// 全局状态
// ============================================================
static G_PERF_INFO_SET: Mutex<BTreeSet<PerfInfo>> = Mutex::new(BTreeSet::new());
static mut G_PERF_ON: bool = false;
static mut G_START_TIME: Option<Instant> = None;
static mut G_COLLATE_TIME_NS: i64 = 0;
static mut G_DURATION: f64 = 0.0;
static mut G_START_COUNT: i32 = 0;
static G_PERF_RECORD_VEC: Mutex<Vec<PerfRecord>> = Mutex::new(Vec::new());

// ============================================================
// SexyPerf
// ============================================================
pub struct SexyPerf;

impl SexyPerf {
    pub fn is_perf_on() -> bool {
        unsafe { G_PERF_ON }
    }

    pub fn begin_perf(measure_perf_overhead: bool) {
        if let Ok(mut set) = G_PERF_INFO_SET.lock() {
            set.clear();
        }
        if let Ok(mut vec) = G_PERF_RECORD_VEC.lock() {
            vec.clear();
        }
        unsafe {
            G_START_COUNT = 0;
            G_COLLATE_TIME_NS = 0;
            if !measure_perf_overhead {
                G_PERF_ON = true;
            }
            G_START_TIME = Some(Instant::now());
        }
    }

    pub fn end_perf() {
        let an_end_time = Instant::now();

        Self::collate_perf_records();

        unsafe {
            G_PERF_ON = false;
            let start = G_START_TIME.unwrap();
            let duration_ns = an_end_time.duration_since(start).as_nanos() as i64;
            let freq = 1_000_000_000i64;
            G_DURATION = (duration_ns - G_COLLATE_TIME_NS) as f64 * 1000.0 / freq as f64;
        }

        if let Ok(set) = G_PERF_INFO_SET.lock() {
            for info in set.iter() {
                let _ = info; // 保留数据供 GetResults 使用
            }
        }
    }

    pub fn start_timing(the_name: &str) {
        unsafe {
            if G_PERF_ON {
                G_START_COUNT += 1;
                if let Ok(mut vec) = G_PERF_RECORD_VEC.lock() {
                    vec.push(PerfRecord::new(the_name, true));
                }
            }
        }
    }

    pub fn stop_timing(the_name: &str) {
        unsafe {
            if G_PERF_ON {
                if let Ok(mut vec) = G_PERF_RECORD_VEC.lock() {
                    vec.push(PerfRecord::new(the_name, false));
                }
                G_START_COUNT -= 1;
                if G_START_COUNT == 0 {
                    Self::collate_perf_records();
                }
            }
        }
    }

    pub fn get_results() -> String {
        let mut a_result = String::new();
        unsafe {
            let _ = G_DURATION;
        }
        // 简化实现：返回基本信息
        a_result.push_str("PerfTimer results (stub)\n");
        a_result
    }

    fn insert_perf_record(the_record: &PerfRecord) {
        if let Ok(mut set) = G_PERF_INFO_SET.lock() {
            if the_record.m_start {
                let mut info = PerfInfo::new(&the_record.m_name);
                // 尝试插入或获取已有
                if let Some(existing) = set.replace(info.clone()) {
                    info = existing;
                }
                info.m_call_count += 1;
                info.m_start_count += 1;
                if info.m_start_count == 1 {
                    info.m_start_time = the_record.m_time;
                }
                set.replace(info);
            } else {
                // find the entry
                let key = PerfInfo::new(&the_record.m_name);
                let mut found = false;
                if let Some(info) = set.take(&key) {
                    let mut info = info;
                    info.m_start_count -= 1;
                    if info.m_start_count == 0 {
                        let a_duration = the_record.m_time.duration_since(info.m_start_time).as_nanos() as i64;
                        info.m_duration_ns += a_duration;
                        if a_duration > info.m_longest_call_ns {
                            info.m_longest_call_ns = a_duration;
                        }
                    }
                    set.insert(info);
                    found = true;
                }
                if !found {
                    // ignore stop without start
                }
            }
        }
    }

    fn collate_perf_records() {
        let time1 = Instant::now();
        let records: Vec<PerfRecord> = {
            if let Ok(mut vec) = G_PERF_RECORD_VEC.lock() {
                std::mem::take(&mut *vec)
            } else {
                Vec::new()
            }
        };

        for record in &records {
            Self::insert_perf_record(record);
        }

        let time2 = Instant::now();
        unsafe {
            G_COLLATE_TIME_NS += time2.duration_since(time1).as_nanos() as i64;
        }
    }
}

// ============================================================
// SexyAutoPerf
// ============================================================
pub struct SexyAutoPerf {
    m_name: String,
    m_is_started: bool,
}

impl SexyAutoPerf {
    pub fn new(the_name: &str) -> Self {
        SexyPerf::start_timing(the_name);
        SexyAutoPerf {
            m_name: the_name.to_string(),
            m_is_started: true,
        }
    }

    pub fn with_condition(the_name: &str, do_start: bool) -> Self {
        if do_start {
            SexyPerf::start_timing(the_name);
        }
        SexyAutoPerf {
            m_name: the_name.to_string(),
            m_is_started: do_start,
        }
    }

    pub fn start(&mut self) {
        if !self.m_is_started {
            self.m_is_started = true;
            SexyPerf::start_timing(&self.m_name);
        }
    }

    pub fn stop(&mut self) {
        if self.m_is_started {
            SexyPerf::stop_timing(&self.m_name);
            self.m_is_started = false;
        }
    }
}

impl Drop for SexyAutoPerf {
    fn drop(&mut self) {
        self.stop();
    }
}
