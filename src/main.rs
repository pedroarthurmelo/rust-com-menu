use std::io;

mod usuario;
mod grupo;
mod arquivo;
mod diretorio;
mod permissao;

use usuario::Usuario;
use grupo::Grupo;
use arquivo::Arquivo;
use diretorio::Diretorio;
use permissao::Permissao;

fn main() {
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();
    let mut arquivos: Vec<Arquivo> = Vec::new();
    let mut diretorios: Vec<Diretorio> = Vec::new();

    loop {
        println!("\n===== Menu Principal =====");
        println!("1. Gerenciar Usuários");
        println!("2. Gerenciar Grupos");
        println!("3. Gerenciar Arquivos");
        println!("4. Gerenciar Diretórios");
        println!("5. Sair");
        println!("Escolha uma opção:");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Erro ao ler entrada");
        match escolha.trim() {
            "1" => menu_usuario(&mut usuarios),
            "2" => menu_grupo(&mut grupos, &mut usuarios),
            "3" => menu_arquivo(&mut arquivos, &usuarios, &grupos),
            "4" => menu_diretorio(&mut diretorios, &mut arquivos, &usuarios),
            "5" => break,
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}

// Submenu para gerenciar usuários
fn menu_usuario(usuarios: &mut Vec<Usuario>) {
    loop {
        println!("\n===== Gerenciar Usuários =====");
        println!("1. Criar Usuário");
        println!("2. Listar Usuários");
        println!("3. Remover Usuário");
        println!("4. Voltar");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Erro ao ler entrada");
        match escolha.trim() {
            "1" => {
                println!("Nome do usuário:");
                let mut nome = String::new();
                io::stdin().read_line(&mut nome).expect("Erro ao ler nome");

                println!("UID do usuário:");
                let mut uid = String::new();
                io::stdin().read_line(&mut uid).expect("Erro ao ler UID");
                let uid: u16 = uid.trim().parse().expect("UID inválido");

                let usuario = Usuario::new(nome.trim().to_string(), uid);
                usuarios.push(usuario);
                println!("Usuário criado com sucesso!");
            },
            "2" => {
                println!("===== Lista de Usuários =====");
                for usuario in usuarios.iter() {
                    println!("UID: {}, Nome: {}", usuario.uid, usuario.nome);
                }
            },
            "3" => {
                println!("UID do usuário a remover:");
                let mut uid = String::new();
                io::stdin().read_line(&mut uid).expect("Erro ao ler UID");
                let uid: u16 = uid.trim().parse().expect("UID inválido");

                // Removendo usuário manualmente
                let mut found = false;
                for i in 0..usuarios.len() {
                    if usuarios[i].uid == uid {
                        usuarios.remove(i);
                        found = true;
                        println!("Usuário removido com sucesso!");
                        break;
                    }
                }
                if !found {
                    println!("Usuário não encontrado.");
                }
            },
            "4" => break,
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}

// Submenu para gerenciar grupos
fn menu_grupo(grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
    loop {
        println!("\n===== Gerenciar Grupos =====");
        println!("1. Criar Grupo");
        println!("2. Listar Grupos");
        println!("3. Adicionar Usuário ao Grupo");
        println!("4. Remover Usuário do Grupo");
        println!("5. Voltar");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Erro ao ler entrada");
        match escolha.trim() {
            "1" => {
                println!("Nome do grupo:");
                let mut nome = String::new();
                io::stdin().read_line(&mut nome).expect("Erro ao ler nome");

                println!("GID do grupo:");
                let mut gid = String::new();
                io::stdin().read_line(&mut gid).expect("Erro ao ler GID");
                let gid: u16 = gid.trim().parse().expect("GID inválido");

                let grupo = Grupo::new(nome.trim().to_string(), gid);
                grupos.push(grupo);
                println!("Grupo criado com sucesso!");
            },
            "2" => {
                println!("===== Lista de Grupos =====");
                for grupo in grupos.iter() {
                    println!("GID: {}, Nome: {}", grupo.gid, grupo.nome);
                }
            },
            "3" => {
                println!("GID do grupo:");
                let mut gid = String::new();
                io::stdin().read_line(&mut gid).expect("Erro ao ler GID");
                let gid: u16 = gid.trim().parse().expect("GID inválido");

                let mut grupo_found = false;
                for grupo in grupos.iter_mut() {
                    if grupo.gid == gid {
                        println!("UID do usuário a adicionar:");
                        let mut uid = String::new();
                        io::stdin().read_line(&mut uid).expect("Erro ao ler UID");
                        let uid: u16 = uid.trim().parse().expect("UID inválido");

                        let mut usuario_found = false;
                        for usuario in usuarios.iter() {
                            if usuario.uid == uid {
                                grupo.adiciona_membro(usuario.clone());
                                usuario_found = true;
                                println!("Usuário adicionado ao grupo!");
                                break;
                            }
                        }
                        if !usuario_found {
                            println!("Usuário não encontrado.");
                        }
                        grupo_found = true;
                        break;
                    }
                }
                if !grupo_found {
                    println!("Grupo não encontrado.");
                }
            },
            "4" => {
                println!("GID do grupo:");
                let mut gid = String::new();
                io::stdin().read_line(&mut gid).expect("Erro ao ler GID");
                let gid: u16 = gid.trim().parse().expect("GID inválido");

                let mut grupo_found = false;
                for grupo in grupos.iter_mut() {
                    if grupo.gid == gid {
                        println!("UID do usuário a remover:");
                        let mut uid = String::new();
                        io::stdin().read_line(&mut uid).expect("Erro ao ler UID");
                        let uid: u16 = uid.trim().parse().expect("UID inválido");

                        grupo.remover_membro(uid);
                        println!("Usuário removido do grupo!");
                        grupo_found = true;
                        break;
                    }
                }
                if !grupo_found {
                    println!("Grupo não encontrado.");
                }
            },
            "5" => break,
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}

// Submenu para gerenciar arquivos
fn menu_arquivo(arquivos: &mut Vec<Arquivo>, usuarios: &Vec<Usuario>, grupos: &Vec<Grupo>) {
    // Implementar funcionalidades para gerenciar arquivos, conforme necessário
}

// Submenu para gerenciar diretórios
fn menu_diretorio(diretorios: &mut Vec<Diretorio>, arquivos: &Vec<Arquivo>, usuarios: &Vec<Usuario>) {
    // Implementar funcionalidades para gerenciar diretórios, conforme necessário
}
