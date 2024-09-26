use super::index::Post;

impl Post {
    fn new(
        id: u64,
        category_id: u64,
        category_name: &'static str,
        title: &'static str,
        created_at: Option<chrono::DateTime<chrono::Local>>,
    ) -> Self {
        Self {
            id,
            category_id,
            category_name: category_name.to_string(),
            title: title.to_string(),
            description: None,
            created_at,
            updated_at: None,
        }
    }
}

pub fn make_fake_posts() -> Vec<Post> {
    vec![
        Post::new(
            1,
            1,
            "大前端",
            "“大前端已死？”没有永远的技术红利，但应具备重拾信心的勇气",
            None,
        ),
        Post::new(
            2,
            1,
            "大前端",
            "用 WASM 连接 Rust 与 Python|Rust 学习笔记（三）",
            None,
        ),
        Post::new(
            3,
            1,
            "大前端",
            "手把手教学，使用Rust + WASM 进行Web 开发",
            None,
        ),
        Post::new(
            4,
            2,
            "后端相关",
            "Axum: 简洁而强大的Rust Web框架",
            None,
        ),
        Post::new(
            5,
            2,
            "后端相关",
            "Spring 新闻汇总：Spring Boot、Cloud、Security、Session 和 Spring AI 发布里程碑版本",
            None,
        ),
        Post::new(
            6,
            2,
            "后端相关",
            "Rust 1.80 提供 Lazy Statics 稳定支持，并支持在模式匹配中使用开区间",
            None,
        ),
    ]
}
