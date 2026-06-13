struct Mail {
    id: u32,
    title: String,
    content: String,
    is_read: bool,
}

struct Inbox {
    mails: Vec<Mail>,
}

impl Inbox {
    fn new() -> Inbox {
        Inbox{ mails: Vec::new(), }
    }

    fn add_mail(&mut self, id: u32, title: &str, content: &str) {
        match self.mails.iter_mut().find(|m| m.id == id) {
            Some(mail) => {
                mail.title = title.to_string();
                mail.content = content.to_string();
            },
            None => {
                self.mails.push(Mail { 
                    id, 
                    title: title.to_string(), 
                    content: content.to_string(), 
                    is_read: false, 
                });
            }
        }
    }

    fn find_mail(&self, id: u32) -> Option<&Mail> {
        self.mails.iter().find(|m|m.id == id)
    }

    fn read_mail(&mut self, id: u32) -> Result<(), String> {
        if let Some(mail) = self.mails.iter_mut().find(|m|m.id == id) {
            mail.is_read = true;
            Ok(())
        }else {
            Err(format!("未找到id{}的邮件", id))
        }
    }

    fn delete_mail(&mut self, id: u32) -> Result<(),String> {
        match self.mails.iter_mut().find(|m|m.id == id) {
            None => Err(format!("id{}对应的邮件不存在，无法删除", id)),
            Some(_) => {
                self.mails.retain(|m|m.id != id);
                Ok(())
            }
        }
    }

    fn list_unread(&self) {
        for mail in &self.mails {
            if !mail.is_read {
                println!("{}.{}-{}", mail.id, mail.title, mail.content);
            }
        }
    }

}

fn main(){
    // 1. 创建空收件箱
    let mut inbox = Inbox::new();

    // 2. 新增3封邮件
    inbox.add_mail(1, "会议通知", "下午3点技术例会");
    inbox.add_mail(2, "快递提醒", "你的包裹已送达驿站");
    inbox.add_mail(3, "周报提交", "本周五下班前上交周报");

    println!("=== 初始全部未读邮件 ===");
    inbox.list_unread();

    // 3. 查询id=2的邮件（Option匹配）
    match inbox.find_mail(2) {
        Some(mail) => println!("\n查询到邮件id{}：{}", mail.id, mail.title),
        None => println!("\n未找到该邮件"),
    }

    // 4. 标记id=2邮件为已读（Result处理）
    match inbox.read_mail(2) {
        Ok(_) => println!("\nid=2 邮件标记已读成功"),
        Err(e) => println!("\n标记失败：{}", e),
    }

    println!("\n=== 标记已读后，剩余未读邮件 ===");
    inbox.list_unread();

    // 5. 重复标记已读（不会报错，只是重复赋值）
    let _ = inbox.read_mail(2);

    // 6. 覆盖id=1旧邮件，重新写入新内容
    inbox.add_mail(1, "紧急会议", "临时调整到下午2点开会");
    println!("\n=== 覆盖id=1邮件后的未读列表 ===");
    inbox.list_unread();

    // 7. 删除id=3邮件
    match inbox.delete_mail(3) {
        Ok(_) => println!("\nid=3 邮件删除成功"),
        Err(e) => println!("\n删除失败：{}", e),
    }

    println!("\n=== 删除后未读邮件 ===");
    inbox.list_unread();

    // 8. 删除不存在的id=99，触发Err错误
    match inbox.delete_mail(99) {
        Ok(_) => println!("删除成功"),
        Err(e) => println!("\n异常测试：{}", e),
    }
}