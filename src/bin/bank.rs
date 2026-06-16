#[derive(Debug)]
struct Account {
    acc_id: u32,
    owner: String,
    balance: f64,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Bank {
        Bank {
            accounts: Vec::new(),
        }
    }

    fn create_acc(&mut self, acc_id: u32, name: &str) {
        match self.accounts.iter_mut().find(|a| a.acc_id == acc_id) {
            Some(_acc) => {
                println!("id:{} 此账户已存在，请重新创建。", acc_id);
            }
            None => {
                self.accounts.push(Account {
                    acc_id,
                    owner: name.to_string(),
                    balance: 0.0,
                });
                println!("成功开户! 持有者：{}, 账户id：{}", name, acc_id);
            }
        }
    }

    fn query_acc(&self, acc_id: u32) -> Option<&Account> {
        self.accounts.iter().find(|a| a.acc_id == acc_id)
    }

    fn deposit(&mut self, acc_id: u32, money: f64) -> Result<(f64, f64), String> {
        let deposit_money = money;
        match self.accounts.iter_mut().find(|a| a.acc_id == acc_id) {
            Some(acc) => {
                acc.balance += money;
                Ok((deposit_money, acc.balance))
            }
            None => Err(format!("由于未知原因，金额存入失败")),
        }
    }

    fn withdraw(&mut self, acc_id: u32, money: f64) -> Result<(f64, f64), String> {
        let withdraw_money = money;
        match self.accounts.iter_mut().find(|a| a.acc_id == acc_id) {
            Some(acc) => {
                if money > acc.balance {
                    return Err(format!("余额不足，无法取出"));
                } else {
                    acc.balance -= money;
                }
                println!("成功取出：￥{},剩余：￥{}", money, acc.balance);
                Ok((withdraw_money, acc.balance))
            }
            None => Err(format!("找不到该账户")),
        }
    }
}

fn main() {
    println!("====创建银行====");
    let mut bank = Bank::new();
    println!("已成功创建银行!");
    println!();

    println!("====银行开户====");
    bank.create_acc(1, "张三");
    bank.create_acc(2, "李四");
    bank.create_acc(3, "王五");
    bank.create_acc(4, "赵六");
    println!("----创建重复账户----");
    bank.create_acc(1, "赵六");
    println!();

    println!("====账户查询====");
    println!("----查询存在账户----");
    match bank.query_acc(1) {
        Some(acc) => {
            println!("查询到账户：{}", acc.acc_id)
        }
        None => println!("此账户不存在"),
    }
    println!("----查询不存在账户----");
    let search_id: u32 = 999;
    match bank.query_acc(search_id) {
        Some(acc) => {
            println!("查询到账户：{}{}", acc.owner, acc.acc_id)
        }
        None => println!("id:{} 此账户不存在", search_id),
    }
    println!();

    println!("====账户存款====");
    match bank.deposit(1, 1_0000_0000.0) {
        Ok(data) => {
            println!("已存入：￥{}，当前余额：￥{}", data.0, data.1)
        }
        Err(e) => println!("{}", e),
    }
    println!();

    println!("====账户取款====");
    match bank.withdraw(1, 3000_0000.0) {
        Ok(b) => {
            println!("已取出：￥{}，剩余余额：￥{}", b.0, b.1)
        }
        Err(e) => println!("{}", e),
    }
    println!("----超额取款----");
    match bank.withdraw(1, 7000_0001.0) {
        Ok(b) => {
            println!("已取出：￥{}，剩余余额：￥{}", b.0, b.1)
        }
        Err(e) => println!("{}", e),
    }
}
