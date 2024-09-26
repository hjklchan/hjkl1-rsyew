use super::Category;

impl Category {
    fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            children: None,
        }
    }
}

pub fn make_fake_categories() -> Vec<Category> {
    vec![
        Category::new(1, "前端相关".to_string()),
        Category::new(2, "后端相关".to_string()),
        Category::new(3, "网络相关".to_string()),
        Category::new(4, "全栈".to_string()),
        Category::new(5, "软件工程".to_string()),
        Category::new(6, "云计算".to_string()),
    ]
}
