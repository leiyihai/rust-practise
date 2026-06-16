#[derive(Debug)]
struct Item {
    name: String,
    count: u32,
}

#[derive(Debug)]
struct PlayerBag {
    items: Vec<Item>,
}

impl PlayerBag {
    fn new() -> PlayerBag {
        PlayerBag { items: Vec::new() }
    }

    fn add(&mut self, item_name: &str, num: u32) {
        if let Some(item) = self.items.iter_mut().find(|i| i.name == item_name) {
            item.count += num;
        } else {
            self.items.push(Item {
                name: item_name.to_string(),
                count: num,
            });
        }
    }

    fn get_item(&self, item_name: &str) -> Option<&Item> {
        self.items
            .iter()
            .find(|item| item.name == item_name && item.count > 0)
    }

    // 修复：拆分查找和数量判断，区分两种错误；新增数量归零自动删除
    fn use_item(&mut self, item_name: &str, num: u32) -> Result<u32, String> {
        // 第一步：只按名字查找，不提前判断数量
        match self.items.iter_mut().find(|item| item.name == item_name) {
            None => Err(format!("物品「{}」不存在", item_name)),
            Some(good) => {
                if good.count < num {
                    return Err(format!(
                        "「{}」数量不足，现有{}个，需要{}个",
                        item_name, good.count, num
                    ));
                }
                // 扣减数量
                good.count -= num;
                let remain = good.count;

                // ========== 数量归零，在这里移除整条物品 ==========
                if remain == 0 {
                    self.items.retain(|x| x.name != item_name);
                }

                Ok(remain)
            }
        }
    }

    fn list_all(&self) {
        println!("\n背包清单：");
        if self.items.is_empty() {
            println!("背包已空");
            return;
        }
        for item in &self.items {
            print!("{}:{}个；", item.name, item.count);
        }
        println!();
    }
}

fn main() {
    let mut my_bag = PlayerBag::new();
    my_bag.add("红药水", 5);
    my_bag.add("铁剑", 1);
    my_bag.list_all();

    // 第一次消耗2个红药水
    match my_bag.use_item("红药水", 2) {
        Ok(rem) => println!("消耗成功，红药水剩余: {}个", rem),
        Err(e) => println!("消耗失败：{}", e),
    }

    // 使用铁剑（数量1，用完直接自动删除条目）
    match my_bag.use_item("铁剑", 1) {
        Ok(_) => println!("成功使用铁剑！铁剑已从背包移除"),
        Err(e) => println!("消耗失败：{}", e),
    }

    // 第二次消耗红药水：剩余3个，要拿4个，触发数量不足Err
    match my_bag.use_item("红药水", 4) {
        Ok(rem) => println!("消耗成功，红药水剩余: {}个", rem),
        Err(e) => println!("消耗失败：{}", e),
    }

    my_bag.list_all();
}
