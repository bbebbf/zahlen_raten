use crate::werkzeuge;

pub fn zahlen_raten() {
    println!("Zahlen raten");
    println!("============");
    ein_spiel();
}

fn ein_spiel() {
    let min: usize = 1;
    let max: usize = werkzeuge::lese_zahl("Obergrenze:", min, 1000);
    let gesuchte_zahl = werkzeuge::erzeuge_zufallszahl(min, max);

    loop {
        let tipp: usize = werkzeuge::lese_zahl(format!("Rate eine Zahl zwischen {min} und {max}:").as_str(), min, max);

        if tipp > gesuchte_zahl {
            println!("Die gesuchte Zahl ist kleiner als {tipp}.");
        }
        else if tipp < gesuchte_zahl {
            println!("Die gesuchte Zahl ist größer als {tipp}.");
        }
        else {
            println!("Richtig. Die gesuchte Zahl ist {gesuchte_zahl}.");
            break;
        }
    }
}
