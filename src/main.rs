use std::io;

mod usuario;
mod grupo;
mod arquivo;
mod diretorio;
mod permissao;
mod menus;

use arquivo::Arquivo;
use diretorio::Diretorio;
use grupo::Grupo;
use permissao::Permissao;
use usuario::Usuario;

//input para ler a escolha do usuário 
fn ler_escolha() -> String {
    let mut escolha = String::new();
    io::stdin().read_line(&mut escolha).expect("erro ao ler mensagem");
    escolha.trim().to_string()
}

fn main() {
    let mut grupos: Vec<Grupo> = Vec::new();
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut arquivos: Vec<Arquivo> = Vec::new();
    let mut diretorios: Vec<Diretorio> = Vec::new();
    //menu principal
    loop {
        println!("\nMenu Principal");
        println!("1. Manipular Usuários");
        println!("2. Manipular Grupos");
        println!("3. Manipular Arquivos");
        println!("4. Manipular Diretórios");
        println!("5. Sair");
        //variavel para ler escolha
        let escolha = ler_escolha();
        //tipo if else para a escolha do usuário
        match escolha.as_str() {
            "1" => menus::menu_usuario::menu_usuario(&mut usuarios, &mut grupos),
            "2" => menus::menu_grupo::menu_grupo(&mut grupos, &mut usuarios),
            "3" => menus::menu_arquivo::menu_arquivo(&mut arquivos, &mut usuarios, &mut grupos),
            "4" => menus::menu_diretorio::menu_diretorio(&mut diretorios, &mut arquivos, &mut usuarios),
            "5" => break,
            _ => println!("Opção Inválida!"),
        }
    }   
}