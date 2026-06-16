struct CarGoods {
    item_name: String,
    price: f32,
    num: u32,
}

struct ShopCart {
    items: Vec<CarGoods>,
}

impl ShopCart {
    fn new() -> ShopCart {
        ShopCart { items: Vec::new() }
    }

    fn add_item(&mut self, name: &str, price: f32, num: u32) {
        match self.items.iter_mut().find(|i| i.item_name == name) {
            Some(item) => {
                item.price = price;
                item.num = num;
            }
            None => self.items.push(CarGoods {
                item_name: name.to_string(),
                price,
                num,
            }),
        }
        println!("已添加{}：{}个", name, num);
    }

    fn sub_item(&mut self, name: &str, num: u32) -> Result<u32, String> {
        match self.items.iter_mut().find(|i| i.item_name == name) {
            Some(item) => {
                if item.num < num {
                    return Err(format!("{}数量不足{}个,无法移除", name, num));
                }

                item.num -= num;
                println!("已减少{}{}个", name, num);

                let remain = item.num;
                if remain == 0 {
                    self.items.retain(|i| i.item_name != name);
                }

                Ok(remain)
            }
            None => Err(format!("商品{}不存在", name)),
        }
    }

    fn get_item(&self, name: &str) -> Option<&CarGoods> {
        self.items.iter().find(|i| i.item_name == name)
    }

    fn calc_total(&self) -> f32 {
        let mut total_price: f32 = 0.0;
        for item in &self.items {
            total_price += item.price * item.num as f32;
        }
        total_price
    }

    fn show_cart(&self) {
        println!("====购物车商品清单====");
        for item in &self.items {
            println!(
                "{}[单价：{}, 数量：{}]",
                item.item_name, item.price, item.num
            );
        }
    }
}

fn main() {
    let mut shop_cart = ShopCart::new();

    println!("====增加购物清单====");
    shop_cart.add_item("洗衣液", 32.0, 2);
    shop_cart.add_item("酸菜牛肉面", 5.5, 1);
    shop_cart.add_item("螺蛳粉", 8.8, 4);
    shop_cart.add_item("凉茶", 11.0, 1);
    println!();

    println!("====减少购买数量====");
    match shop_cart.sub_item("螺蛳粉", 2) {
        Ok(_) => {}
        Err(e) => print!("{}", e),
    }
    match shop_cart.sub_item("洗衣液", 2) {
        Ok(_) => {}
        Err(e) => print!("{}", e),
    }
    match shop_cart.sub_item("凉茶", 2) {
        Ok(_) => {}
        Err(e) => print!("{}", e),
    }
    println!();

    println!("====查询购物车商品====");
    match shop_cart.get_item("乳酸菌素片") {
        Some(item) => println!("找到商品：{}，数量：{}", item.item_name, item.num),
        None => println!("没有找到改物品"),
    }
    println!();

    println!("====结算购物车====");
    shop_cart.show_cart();
    let total_price = shop_cart.calc_total();
    println!("共计：{}", total_price);
}
