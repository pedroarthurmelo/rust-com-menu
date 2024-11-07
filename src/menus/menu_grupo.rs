use std::io;

use crate::grupo::Grupo;
use crate::usuario::Usuario;

//input para ler a escolha do usuário 
fn ler_escolha() -> String {
    let mut escolha = String::new();
    io::stdin().read_line(&mut escolha).expect("erro ao ler mensagem");
    escolha.trim().to_string()
}

//////////////////////////MENU GRUPO/////////////////////////////////////


pub fn menu_grupo(grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
    loop {
        println!("\nMenu Grupo:");
        println!("1. Criar Grupo");
        println!("2. Adicionar Membro ao Grupo");
        println!("3. Remover Membro do Grupo");
        println!("4. Listar Membros do Grupo");
        println!("5. Voltar");

        let escolha = ler_escolha();

        match escolha.as_str() {
            "1" => criar_grupo(grupos),
            "2" => adicionar_membro_grupo(grupos, usuarios),
            "3" => remover_membro_grupo(grupos, usuarios),
            "4" => listar_membros_grupo(grupos),
            "5" => return, // volta para o menu principal
            _ => println!("Opção inválida!"),
        }
    }
}

fn criar_grupo(grupos: &mut Vec<Grupo>) {
    println!("Digite o nome do grupo: ");
    let mut nome_input = String::new();
    io::stdin().read_line(&mut nome_input).expect("erro ao ler linha");
    let nome = nome_input.trim().to_string();

    println!("Digite o GID do grupo: ");
    let mut gid_input = String::new();
    io::stdin().read_line(&mut gid_input).expect("erro ao ler linha");
    let gid: u16 = gid_input.trim().parse().expect("erro ao converter");

    let novo_grupo = Grupo::new(nome, gid);
    grupos.push(novo_grupo);
    println!("Grupo criado com sucesso")
}

fn adicionar_membro_grupo(grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
    println!("Digite o nome do grupo para adicionar um membro: ");
    let mut nome_grupo_input = String::new();
    io::stdin().read_line(&mut nome_grupo_input).expect("Erro ao ler nome do grupo");
    let nome_grupo = nome_grupo_input.trim().to_string();

    let mut grupo_encontrado = false;
    for grupo in grupos.iter_mut() {
        if grupo.nome == nome_grupo {
            println!("Usuários cadastrados:");
            for (i, usuario) in usuarios.iter().enumerate() {
                println!("{}: ID: {}, Nome: {}", i + 1, usuario.uid, usuario.nome);
            }

            println!("Escolha o número do usuário a ser adicionado: ");
            let mut escolha_input = String::new();
            io::stdin().read_line(&mut escolha_input).expect("Erro ao ler linha");
            let escolha: usize = escolha_input.trim().parse().expect("Erro ao converter escolha");

            if escolha > 0 && escolha <= usuarios.len() {
                let usuario = &usuarios[escolha - 1];
                println!("Você escolheu o usuário: {} (ID: {})", usuario.nome, usuario.uid);
                grupo.adiciona_membro(usuario.clone());
                // Atualiza o usuário
                usuarios[escolha - 1].adiciona_membro(grupo.clone());
                println!("Usuário adicionado ao grupo!");
            } else {
                println!("Escolha inválida.");
            }

            grupo_encontrado = true;
            break;
        }
    }

    if !grupo_encontrado {
        println!("Grupo não encontrado.");
    }
}

fn remover_membro_grupo(grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
    println!("Digite o nome do grupo para remover um membro: ");
    let mut nome_grupo_input = String::new();
    io::stdin().read_line(&mut nome_grupo_input).expect("Erro ao ler nome do grupo");
    let nome_grupo = nome_grupo_input.trim().to_string();

    println!("Digite o UID do usuário a ser removido: ");
    let mut uid_input = String::new();
    io::stdin().read_line(&mut uid_input).expect("Erro ao ler UID");
    let uid: u16 = uid_input.trim().parse().expect("Erro ao converter UID");

    let mut grupo_encontrado = false;
    for grupo in grupos.iter_mut() {
        if grupo.nome == nome_grupo {
            grupo.remover_membro(uid);
            // Atualiza o usuário
            for usuario in usuarios.iter_mut() {
                if usuario.uid == uid {
                    usuario.remove_grupo(grupo.gid);
                    break;
                }
            }
            println!("Membro removido do grupo!");
            grupo_encontrado = true;
            break;
        }
    }

    if !grupo_encontrado {
        println!("Grupo não encontrado.");
    }
}

fn listar_membros_grupo(grupos: &Vec<Grupo>) {
    println!("Digite o nome do grupo para listar seus membros: ");
    let mut nome_grupo_input = String::new();
    io::stdin().read_line(&mut nome_grupo_input).expect("Erro ao ler nome do grupo");
    let nome_grupo = nome_grupo_input.trim().to_string();

    let mut grupo_encontrado = false;
    for grupo in grupos.iter() {
        if grupo.nome == nome_grupo {
            // Aqui, você chama o método listar_grupos_membros
            grupo.listar_grupos_membros();
            grupo_encontrado = true;
            break;
        }
    }

    if !grupo_encontrado {
        println!("Grupo não encontrado.");
    }
}