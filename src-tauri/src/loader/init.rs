use flexi_logger::{
    colored_opt_format, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming, WriteMode,
};

pub fn log_init() {
    // 创建一个配置构建器
    // 初始化日志记录，配置输出到文件，设置文件大小限制和滚动日志
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("logs") // 设置日志文件目录
                .basename("app") // 设置日志文件前缀
                .suffix("log"), // 设置日志文件后缀
        )
        .rotate(
            Criterion::Size(10_000_000), // 设置日志文件大小限制为 10 MB
            Naming::Timestamps,          // 使用数字序号进行文件命名
            Cleanup::KeepLogFiles(3),    // 保留最近的 3 个日志文件
        )
        .format_for_files(colored_opt_format)
        .format_for_stdout(colored_opt_format) // 使用详细格式，包含时间戳
        .write_mode(WriteMode::BufferAndFlush) // 设置日志写入模式
        .duplicate_to_stdout(Duplicate::Info) // 将警告级别的日志复制到标准错误输出
        .start()
        .unwrap();
}
