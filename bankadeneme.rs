use std::io;

fn main() {
    let mut balance = 0.0; // Başlangıç bakiyesi

    loop {
        println!("Yapmak istediğiniz işlemi seçin:");
        println!("3. Bakiye Görüntüle");
        println!("4. Çıkış");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Geçersiz girdi.");
        let choice: u32 = choice.trim().parse().expect("Geçersiz seçim.");

        match choice {
           
            3 => {
                println!("Bakiyeniz: {} TL", balance); // Bakiyeyi göster
            }
            4 => {
                println!("Çıkış yapılıyor.");
                break; // Döngüden çık
            }
            _ => {
                println!("Geçersiz seçim. Lütfen tekrar deneyin.");
            }
        }
    }
}
