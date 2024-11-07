use std::io;

use crate::grupo::Grupo;
use crate::usuario::Usuario;

//input para ler a escolha do usuário 
fn ler_escolha() -> String {
    let mut escolha = String::new();
    io::stdin().read_line(&mut escolha).expect("erro ao ler mensagem");
    escolha.trim().to_string()
}

////////////////////////////////MENU USUÁRIO/////////////////////////////////////////
pub fn menu_usuario(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    loop {
        println!("\nMenu Usuário");
        println!("1. Criar um novo usuário");
        println!("2. Adicionar um grupo para o usuário");
        println!("3. Remover um grupo para o usuário");
        println!("4. Listar grupos do usuário");
        println!("5. Voltar");

        let escolha = ler_escolha();

        match escolha.as_str() {
            "1" => {
                let novo_usuario = criar_usuario();
                usuarios.push(novo_usuario);
            },
            "2" => adicionar_grupo_usuario(usuarios, grupos),
            "3" => remover_grupo_usuario(usuarios, grupos),
            "4" => listar_grupos_usuario(usuarios),
            "5" => return, // volta para o menu principal
            _ => println!("Operação inválida"),
        }
    }
}

fn criar_usuario() -> Usuario {
    println!("Digite o nome do usuário: ");
    let mut nome_input = String::new();
    io::stdin().read_line(&mut nome_input).expect("erro ao ler linha");
    let nome = nome_input.trim().to_string();

    println!("Digite o UID do usuário: ");
    let mut uid_input = String::new();
    io::stdin().read_line(&mut uid_input).expect("erro ao ler linha");
    let uid = uid_input.trim().parse().expect("erro ao converter número");

    let usuario = Usuario::new(nome, uid);
    println!("Usuário {} criado com sucesso! (UID: {}) ", usuario.nome, usuario.uid);

    return usuario
}

fn adicionar_grupo_usuario(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    if usuarios.is_empty() {
        println!("Nenhum usuário cadastrado ainda.");
        return;
    }

    println!("Digite o nome do grupo a ser adicionado");
    let mut nome_grupo_input = String::new();
    io::stdin().read_line(&mut nome_grupo_input).expect("erro ao ler linha");
    let nome_grupo = nome_grupo_input.trim().to_string();

    let mut grupo_encontrado = false;
    for grupo in grupos.iter_mut() {
        if grupo.nome == nome_grupo {
            // Adiciona o usuário ao grupo e atualiza o grupo
            let usuario = usuarios.last_mut().unwrap();
            usuario.adiciona_membro(grupo.clone());
            grupo.membros.push(usuario.clone());
            println!("Grupo '{}' associado ao usuário {}", nome_grupo, usuario.nome);
            grupo_encontrado = true;
            break;
        }
    }

    if !grupo_encontrado {
        println!("Grupo '{}' não encontrado.", nome_grupo);
    }
}

fn remover_grupo_usuario(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    if usuarios.is_empty() {
        println!("Nenhum usuário cadastrado ainda.");
        return;
    }

    println!("Digite o nome do grupo a ser removido: ");
    let mut nome_grupo_input = String::new();
    io::stdin().read_line(&mut nome_grupo_input).expect("erro ao ler linha");
    let nome_grupo = nome_grupo_input.trim().to_string();

    let mut grupo_encontrado = false;
    for grupo in grupos.iter_mut() {
        if grupo.nome == nome_grupo {
            // Remove o usuário do grupo e atualiza o grupo
            let usuario = usuarios.last_mut().unwrap();
            usuario.remove_grupo(grupo.gid);
            let index = grupo.membros.iter().position(|u| u.uid == usuario.uid);
            if let Some(index) = index {
                grupo.membros.remove(index);
            }
            println!("Grupo '{}' removido do usuário {}", nome_grupo, usuario.nome);
            grupo_encontrado = true;
            break;
        }
    }

    if !grupo_encontrado {
        println!("Grupo '{}' não encontrado.", nome_grupo);
    }
}

fn listar_grupos_usuario(usuarios: &Vec<Usuario>) {
    if usuarios.is_empty() {
        println!("Nenhum usuário cadastrado ainda.");
        return;
    }

    println!("Listando grupos do usuário {}", usuarios.last().unwrap().nome);
    usuarios.last().unwrap().listar_grupos();
}