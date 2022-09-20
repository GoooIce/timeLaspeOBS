use std::os::raw::c_char;

use dep_log::{
    set_boxed_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record, SetLoggerError,
};
use libobs_sys::{blog, LOG_DEBUG, LOG_ERROR, LOG_INFO, LOG_WARNING};

/// 一个嵌入OBS日志系统的 logger
///
/// OBS拥有4个日志等级，最低级别只有调试模式可用，这个logger提供将低级日志提升为"info"。
///
/// 这个日志是可选的，但是我们建议使用，因为OBS会将日志系统中的内容写入文件中，便于查看。
///
/// # 示例
///
/// 使用默认设置初始化
///
/// ```compile_fail
/// let _ = Logger::new().init();
/// ```
pub struct Logger {
    max_level: LevelFilter,
    promote_debug: bool,
}

impl Logger {
    /// 创建默认设置为的logger
    /// 日志等级：[`LevelFilter::Trace`]
    /// 是否提升调试 `false`
    #[must_use = "必须调用`init()`初始化日志记录"]
    pub fn new() -> Logger {
        Logger {
            max_level: LevelFilter::Trace,
            promote_debug: false,
        }
    }

    /// 初始化logger。*必须*这样调用，日志系统才能生效。
    pub fn init(self) -> Result<(), SetLoggerError> {
        set_max_level(self.max_level);
        set_boxed_logger(Box::new(self))
    }

    /// 设置日志记录的最大等级
    #[must_use = "必须调用`init()`初始化日志记录"]
    pub fn with_max_level(mut self, max_level: LevelFilter) -> Self {
        self.max_level = max_level;
        self
    }

    /// 设置是否升级`Trace`与`Debug`日志
    #[must_use = "必须调用`init()`初始化日志记录"]
    pub fn with_promote_debug(mut self, promote_debug: bool) -> Self {
        self.promote_debug = promote_debug;
        self
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.max_level >= metadata.level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let level = record.level();
        let target = if record.target().is_empty() {
            record.module_path().unwrap_or_default()
        } else {
            record.target()
        };

        let line = if self.promote_debug && level <= Level::Debug {
            format!("({}) ({}) {}\0", level, target, record.args())
        } else {
            format!("[{}] {}\0", target, record.args())
        };

        unsafe {
            blog(
                match level {
                    Level::Error => LOG_ERROR,
                    Level::Warn => LOG_WARNING,
                    Level::Info => LOG_INFO,
                    _ => {
                        // DEBUG级别日志仅适用于debug版OBS
                        if self.promote_debug {
                            LOG_INFO
                        } else {
                            LOG_DEBUG
                        }
                    }
                } as i32,
                "%s\0".as_ptr() as *const c_char,
                line.as_ptr() as *const c_char,
            )
        }
    }

    fn flush(&self) {
        /*
        不需要
        */
    }
}
