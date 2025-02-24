use ricq::RQError;
use walle_core::error_type;
use walle_core::resp::RespError;

macro_rules! format_error_type {
    ($name: ident, $retcode: expr, $message: expr, $k: ident, $kt: ty) => {
        pub fn $name($k: $kt) -> RespError {
            RespError {
                code: $retcode,
                message: format!("{}:{}", $message, $k),
            }
        }
    };
}

format_error_type!(bad_param, 10003, "参数错误", param, &str);
error_type!(empty_message, 10003, "消息为空");
format_error_type!(unsupported_param, 10004, "不支持的参数", param, &str);
error_type!(prepare_file_first, 20003, "请先 Prepare 再上传后续文件");
error_type!(file_total_size_not_match, 20004, "文件大小不匹配");
error_type!(file_sha256_not_match, 20005, "文件sha256不匹配");
format_error_type!(file_open_error, 32001, "文件打开失败", e, std::io::Error);
format_error_type!(file_read_error, 32002, "文件读取失败", e, std::io::Error);
format_error_type!(file_create_error, 32003, "文件创建失败", e, std::io::Error);
format_error_type!(file_write_error, 32004, "文件写入失败", e, std::io::Error);
error_type!(file_not_found, 32005, "文件不存在");
error_type!(file_type_not_match, 32006, "文件类型不匹配");
error_type!(net_download_fail, 33001, "网络下载失败");
format_error_type!(rq_error, 34001, "ricq错误", e, RQError);
error_type!(message_not_exist, 35001, "消息不存在");
error_type!(friend_not_exist, 35002, "好友不存在");
error_type!(group_not_exist, 35003, "群不存在");
error_type!(group_member_not_exist, 35004, "群成员不存在");
error_type!(image_info_decode_error, 41001, "图片解码错误");
format_error_type!(bad_image_url, 41002, "图片URL错误", url, &str);
format_error_type!(bad_image_path, 41003, "图片路径错误", path, &str);
format_error_type!(bad_image_data, 41004, "图片内容错误", data, &str);
format_error_type!(
    audio_encode_failed,
    41005,
    "音频编码失败",
    e,
    std::io::Error
);
format_error_type!(
    silk_encode_failed,
    41005,
    "silk编码失败",
    e,
    silk_rs::SilkError
);
