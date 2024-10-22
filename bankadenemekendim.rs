use std::io;

fn main() {
    let balance = 1500.00; 

    loop {
        println!("Yapmak istediğiniz işlemi seçin:");
        println!("1. Bakiye Görüntüle");
        println!("2. Çıkış");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Geçersiz girdi.");
        let choice: i32 = choice.trim().parse().expect("Geçersiz seçim.");

        match choice {
           
             1=> {
                println!("Bakiyeniz: {} TL", balance); 
            }
            2 => {
                println!("Çıkış yapılıyor.");
                break;
            }
            _ => {
                println!("Geçersiz seçim. Lütfen tekrar deneyin.");
            }
        }
    }
}
