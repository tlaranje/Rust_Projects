use std::io::{self, Write};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

// Função que fica rodando em segundo plano esperando o input
fn abrir_leitor_teclado() -> Receiver<String> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                // Envia o texto digitado para a thread principal
                let texto_limpo = input.trim().to_string();
                if tx.send(texto_limpo).is_err() {
                    break; // Se a thread principal morreu, encerra esta também
                }
            }
        }
    });

    rx
}

fn main() {
    let canal_input = abrir_leitor_teclado();
    println!("Loop iniciado! Digite 'q' e dê Enter para sair.\n");

    let mut contador = 0;

    loop {
        // --- SUA LÓGICA DO LOOP RODA AQUI ---
        contador += 1;
        print!("Rodando ciclo {}...\r", contador);
        io::stdout().flush().unwrap();

        // --- VERIFICA SE CHEGOU INPUT (NÃO-BLOQUEANTE) ---
        // `try_recv` não para o programa. Ele apenas espia se tem algo no canal.
        if let Ok(comando) = canal_input.try_recv() {
            println!("\n[Comando recebido: '{}']", comando);

            if comando == "q" {
                println!("Saindo do programa de forma segura!");
                break;
            }
        }

        // Controla a velocidade do loop principal
        thread::sleep(Duration::from_millis(100));
    }
}