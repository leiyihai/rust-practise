// 食材管理模拟器
// ================
// 任务：模拟厨房食材管理——食材可以"借用"但不能重复使用，练习所有权和借用。
//
// 提示：
// - 定义一个 Ingredient 结构体，包含 name 和 used 字段
// - 实现一个 Kitchen 结构体管理食材 Vec
// - 实现 borrow_ingredient(&self, name) -> Option<&Ingredient> —— 不可变借用
// - 实现 use_ingredient(&mut self, name) -> Result —— 可变借用，标记已使用
// - 返回引用时注意生命周期标注：fn get(&self, name: &str) -> Option<&Ingredient>
// - 尝试在持有不可变引用时调用可变方法，看编译器报什么错——理解借用规则
// - 思考：为什么 Rust 不允许同时存在可变引用和不可变引用？
//
// 运行方式：cargo run --bin 04_ingredient_manager
