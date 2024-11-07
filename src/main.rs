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

/////////////////////////FUNÇÕES MENU////////////////////////////////////////
//input para ler a escolha do usuário 
fn ler_escolha() -> String {
    let mut escolha = String::new();
    io::stdin().read_line(&mut escolha).expect("erro ao ler mensagem");
    escolha.trim().to_string()
}

////////////////////////////////MENU USUÁRIO/////////////////////////////////////////
fn menu_usuario(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
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
//////////////////////////MENU GRUPO/////////////////////////////////////


fn menu_grupo(grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
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
            println!("Membros do grupo {}:", grupo.nome);
            for membro in &grupo.membros {
                println!("ID: {}, Nome: {}", membro.uid, membro.nome);
            }
            grupo_encontrado = true;
            break;
        }
    }

    if !grupo_encontrado {
        println!("Grupo não encontrado.");
    }
}

//////////////////////////MENU ARQUIVO///////////////////////////////////
fn menu_arquivo() {
    loop {
        println!("\nMenu Arquivo:");
        println!("1. Criar Arquivo");
        println!("2. Alterar Permissões do Arquivo");
        println!("3. Exibir Informações do Arquivo");
        println!("4. Voltar");

        let escolha = ler_escolha();

        match escolha.as_str() {
            "1" => criar_arquivo(),
            "2" => alterar_permissoes_arquivo(),
            "3" => exibir_info_arquivo(),
            "4" => return, // volta para o menu principal
            _ => println!("Opção inválida!"),
        }
    }
}

fn criar_arquivo() {
    println!("Criando arquivo...");
    // Implementação da criação de arquivo
}

fn alterar_permissoes_arquivo() {
    println!("Alterando permissões do arquivo...");
    // Implementação da alteração de permissões do arquivo
}

fn exibir_info_arquivo() {
    println!("Exibindo informações do arquivo...");
    // Implementação da exibição de informações do arquivo
}
//////////////////////////MENU DIRETÓRIO//////////////////////////////
fn menu_diretorio() {
    loop {
        println!("\nMenu Diretório:");
        println!("1. Criar Diretório");
        println!("2. Adicionar Arquivo ao Diretório");
        println!("3. Remover Arquivo do Diretório");
        println!("4. Listar Conteúdo do Diretório");
        println!("5. Voltar");

        let escolha = ler_escolha();

        match escolha.as_str() {
            "1" => criar_diretorio(),
            "2" => adicionar_arquivo_diretorio(),
            "3" => remover_arquivo_diretorio(),
            "4" => listar_conteudo_diretorio(),
            "5" => return, // volta para o menu principal
            _ => println!("Opção inválida!"),
        }
    }
}

fn criar_diretorio() {
    println!("Criando diretório...");
    // Implementação da criação de diretório
}

fn adicionar_arquivo_diretorio() {
    println!("Adicionando arquivo ao diretório...");
    // Implementação da adição de arquivo ao diretório
}

fn remover_arquivo_diretorio() {
    println!("Removendo arquivo do diretório...");
    // Implementação da remoção de arquivo do diretório
}

fn listar_conteudo_diretorio() {
    println!("Listando conteúdo do diretório...");
    // Implementação da listagem de conteúdo do diretório
}
//////////////////////MENU PRINCIPAL///////////////////////
fn main() {
    let mut grupos: Vec<Grupo> = Vec::new();
    let mut usuarios: Vec<Usuario> = Vec::new();
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
            "1" => menu_usuario(&mut usuarios, &mut grupos),
            "2" => menu_grupo(&mut grupos, &mut usuarios),
            "3" => menu_arquivo(),
            "4" => menu_diretorio(),
            "5" => break,
            _ => println!("Opção Inválida!"),
        }
    }   
}