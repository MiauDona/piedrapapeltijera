fn main() {
    
    let jugadas: [&'static str; 5] = ["piedra", "papel", "tijera", "lagarto", "spock"];
    let mut jugada1 = String::new();
    let mut jugada2 = String::new();

    // Preguntar al mejor de cuanto quiere que sea la partida
    println!("¿Al mejor de cuántas partidas quieres jugar?");
    let mut npartidas = String::new();
    std::io::stdin().read_line(&mut npartidas).unwrap();
    let mut historial2 = Vec::new();

    for mut i in 0..npartidas.trim().parse::<i32>().unwrap() {

        let mut historial = Vec::new();
        loop {
            println!("Jugador 1, ¿Piedra, papel, tijera, lagarto, spock?");
            std::io::stdin().read_line(&mut jugada1).unwrap();
            let jugada1_trimmed = jugada1.trim().to_string();
            if jugadas.contains(&jugada1.trim()) {
                // Guardar jugada en historial
                historial.push(jugada1_trimmed);
                break;
            } else {
                // Mostrar error
                println!("{} no es una jugada válida", &jugada1.trim());
            }
        }
        loop {
            println!("Jugador 2, ¿Piedra, papel, tijera, lagarto, spock?");
            std::io::stdin().read_line(&mut jugada2).unwrap();
    
            if jugadas.contains(&jugada2.trim()) {
                let jugada2_trimmed = jugada2.trim().to_string();
                // Guardar jugada en historial
                historial.push(jugada2_trimmed);
                break;
            } else {
                // Mostrar error
                println!("{} no es una jugada válida", &jugada2.trim());
            }
        }
        println!("-");
        println!("-");

        if jugada1.trim() == jugada2.trim() {
            println!("Empate");
        } else if jugada1.trim() == "piedra" && jugada2.trim() == "tijera" {
            println!("Gana jugador 1");
        } else if jugada1.trim() == "piedra" && jugada2.trim() == "lagarto" {
            println!("Gana jugador 1");
        } else if jugada1.trim() == "papel" && jugada2.trim() == "piedra" {
            println!("Gana jugador 1");
        } else if jugada1.trim() == "papel" && jugada2.trim() == "spock" {
            println!("Gana jugador 1");
        } else if jugada1.trim() == "tijera" && jugada2.trim() == "papel" {
            println!("Gana jugador 1");
        } else if jugada1.trim() == "tijera" && jugada2.trim() == "lagarto" {
            println!("Gana jugador 1");
        } else if jugada1.trim() == "lagarto" && jugada2.trim() == "papel" {
            println!("Gana jugador 1");
        } else if jugada1.trim() == "lagarto" && jugada2.trim() == "spock" {
            println!("Gana jugador 1");
        } else if jugada1.trim() == "spock" && jugada2.trim() == "piedra" {
            println!("Gana jugador 1");
        } else if jugada1.trim() == "spock" && jugada2.trim() == "tijera" {
            println!("Gana jugador 1");
        } else {
            println!("Gana jugador 2");
        }

        historial2.push(historial.clone());
        jugada1.clear();
        jugada2.clear();
        historial.clear();
        i += 1;
    }
    println!("Partidas: {:?}", historial2);
}
