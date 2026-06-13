#[derive(Debug)]
struct BoxItem {
    id: u32,
    name: String,
}

#[derive(Debug)]
struct Storage {
    items: Vec<BoxItem>,
}

impl Storage {
    fn new() -> Storage {
        Storage { items: Vec::new() }
    }

    fn store_item(&mut self, id: u32, name: &str) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.name = name.to_string();
        } else {
            self.items.push(BoxItem {
                id,
                name: name.to_string(),
            });
        }
    }

    fn take_item(&mut self, id: u32) -> Result<String, String> {
        match self.items.iter_mut().find(|i| i.id == id) {
            None => Err(format!("编号{}的储物柜没有存放物品", id)),
            Some(found) => {
                let item_name = found.name.clone();
                self.items.retain(|b| b.id != id);
                Ok(item_name)
            }
        }
    }

    fn search_by_id(&self, id: u32) -> Option<&BoxItem> {
        self.items.iter().find(|b| b.id == id)
    }

    fn show_all(&self) {
        println!("====储物柜列表====");
        if self.items.is_empty() {
            println!("暂无存放物品");
            return;
        }
        for b in &self.items {
            println!("柜子{}：{}", b.id, b.name);
        }
    }
}

fn main() {
    let mut locker = Storage::new();

    // 存入物品
    locker.store_item(1, "雨伞");
    locker.store_item(2, "充电宝");
    locker.store_item(3, "背包");
    locker.show_all();

    // 查找物品
    locker.search_by_id(3);

    // 覆盖物品
    locker.store_item(1, "折叠伞");
    locker.show_all();

    // 取物品
    match locker.take_item(2) {
        Ok(name) => println!("取出物品：{}", name),
        Err(e) => println!("取出失败：{}", e),
    }

    // 重复取已拿走的柜子
    match locker.take_item(2) {
        Ok(name) => println!("取出物品：{}", name),
        Err(e) => println!("取出失败：{}", e),
    }

    locker.show_all();
}
