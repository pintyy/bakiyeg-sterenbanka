use std::io;

fn main() {
    let mut database: Vec<User> = Vec::new();

    
    register(10191165532_i128, 135799_i128, &mut database);
    

    if login(10191165532_i128, 135799_i128, &database) {
    
        add_balance_prompt(10191165532_i128, &mut database);
        
        check_balance(10191165532_i128, &database);
    } else {
        println!("Giriş başarısız.");
    }
}

#[derive(Clone, Debug)]
struct User {
    tc_no: i128,
    password: i128,
    balance: f64,
}

fn login(tc_no: i128, password: i128, database: &Vec<User>) -> bool {
    for user in database {
        if user.tc_no == tc_no && user.password == password {
            println!("Giriş başarılı!");
            return true;
        }
    }
    false
}

fn register(tc_no: i128, password: i128, database: &mut Vec<User>) {
    if tc_no.to_string().len() == 11 {
        let user = User {
            tc_no,
            password,
            balance: 0.0,
        };
        database.push(user);
        println!("Kayıt başarılı!");
    } else {
        println!("Yanlış TC numarası.");
    }
}

fn add_balance_prompt(tc_no: i128, database: &mut Vec<User>) {
    let mut input = String::new();
    println!("Bakiyenizi eklemek için bir miktar girin:");

    io::stdin()
        .read_line(&mut input)
        .expect("Giriş hatası");

    if let Ok(amount) = input.trim().parse::<f64>() {
        for user in database {
            if user.tc_no == tc_no {
                user.balance += amount;
                println!("Bakiyeniz {} TL artırıldı.", amount);
                return;
            }
        }
    } else {
        println!("Geçersiz bir sayı girdiniz.");
    }
}

fn check_balance(tc_no: i128, database: &Vec<User>) {
    for user in database {
        if user.tc_no == tc_no {
            println!("Bakiyeniz: {} TL", user.balance);
            return;
        }
    }
    println!("Kullanıcı bulunamadı.");
}
