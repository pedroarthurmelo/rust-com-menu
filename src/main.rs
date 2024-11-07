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

////////////////////////// MENU ARQUIVO ///////////////////////////

fn menu_arquivo(arquivos: &mut Vec<Arquivo>, usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    loop {
        println!("\nMenu Arquivo:");
        println!("1. Criar Arquivo");
        println!("2. Alterar Permissões do Arquivo");
        println!("3. Exibir Informações do Arquivo");
        println!("4. Voltar");

        let escolha = ler_escolha();

        match escolha.as_str() {
            "1" => criar_arquivo(arquivos, usuarios, grupos),
            "2" => alterar_permissoes_arquivo(arquivos),
            "3" => exibir_info_arquivo(arquivos),
            "4" => return, // Volta para o menu principal
            _ => println!("Opção inválida!"),
        }
    }
}

// Função para criar um novo arquivo
fn criar_arquivo(arquivos: &mut Vec<Arquivo>, usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    if usuarios.is_empty() || grupos.is_empty() {
        println!("É necessário ter pelo menos um usuário e um grupo cadastrados.");
        return;
    }

    // Solicita o nome e tamanho do arquivo
    println!("Digite o nome do arquivo: ");
    let mut nome_input = String::new();
    io::stdin().read_line(&mut nome_input).expect("Erro ao ler nome do arquivo");
    let nome = nome_input.trim().to_string();

    println!("Digite o tamanho do arquivo (em bytes): ");
    let mut tamanho_input = String::new();
    io::stdin().read_line(&mut tamanho_input).expect("Erro ao ler tamanho");
    let tamanho: u64 = tamanho_input.trim().parse().expect("Erro ao converter tamanho");

    // Seleciona o usuário dono do arquivo
    println!("Escolha o número do usuário para ser o dono do arquivo:");
    for (i, usuario) in usuarios.iter().enumerate() {
        println!("{}: Nome: {}, UID: {}", i + 1, usuario.nome, usuario.uid);
    }

    let mut escolha_usuario_input = String::new();
    io::stdin().read_line(&mut escolha_usuario_input).expect("Erro ao ler escolha");
    let escolha_usuario: usize = escolha_usuario_input.trim().parse().expect("Erro ao converter escolha");

    let usuario = &usuarios[escolha_usuario - 1];

    // Seleciona o grupo associado ao arquivo
    println!("Escolha o número do grupo associado ao arquivo:");
    for (i, grupo) in grupos.iter().enumerate() {
        println!("{}: Nome: {}, GID: {}", i + 1, grupo.nome, grupo.gid);
    }

    let mut escolha_grupo_input = String::new();
    io::stdin().read_line(&mut escolha_grupo_input).expect("Erro ao ler escolha");
    let escolha_grupo: usize = escolha_grupo_input.trim().parse().expect("Erro ao converter escolha");

    let grupo = &grupos[escolha_grupo - 1];

    // Cria o arquivo com permissões padrão
    let novo_arquivo = Arquivo::new(nome.clone(), tamanho, usuario.clone(), grupo.clone());

    arquivos.push(novo_arquivo);
    println!("Arquivo '{}' criado com sucesso!", nome);
}

// Função para alterar as permissões de um arquivo
fn alterar_permissoes_arquivo(arquivos: &mut Vec<Arquivo>) {
    if arquivos.is_empty() {
        println!("Nenhum arquivo cadastrado.");
        return;
    }

    println!("Escolha o número do arquivo para alterar permissões:");
    for (i, arquivo) in arquivos.iter().enumerate() {
        println!("{}: Arquivo: {}, Dono: {}, Grupo: {}", i + 1, arquivo.nome, arquivo.usuario.nome, arquivo.grupo.nome);
    }

    let mut escolha_input = String::new();
    io::stdin().read_line(&mut escolha_input).expect("Erro ao ler escolha");
    let escolha: usize = escolha_input.trim().parse().expect("Erro ao converter escolha");

    if escolha > 0 && escolha <= arquivos.len() {
        let arquivo = &mut arquivos[escolha - 1];

        println!("Digite as permissões do arquivo (dono, grupo, outros):");
        println!("Formato: XXX (onde cada X é um número entre 0 e 7, por exemplo, 755)");

        let mut permissoes_input = String::new();
        io::stdin().read_line(&mut permissoes_input).expect("Erro ao ler permissões");

        // Verifica se o usuário forneceu permissões no formato correto
        let permissoes: Vec<&str> = permissoes_input.trim().split_whitespace().collect();
        if permissoes.len() == 1 {
            let permissoes_num: u32 = permissoes[0].parse().unwrap_or(0);

            let dono = (permissoes_num / 100) % 10;
            let grupo = (permissoes_num / 10) % 10;
            let outros = permissoes_num % 10;

            // Converte as permissões numéricas em permissões booleanas (leitura, escrita, execução)
            let nova_permissao = (
                Permissao::new(dono >= 4, dono >= 2, dono >= 1),
                Permissao::new(grupo >= 4, grupo >= 2, grupo >= 1),
                Permissao::new(outros >= 4, outros >= 2, outros >= 1),
            );

            arquivo.alterar_permissao(nova_permissao);
            println!("Permissões do arquivo '{}' alteradas.", arquivo.nome);
        } else {
            // Caso o usuário não forneça permissões, usa as permissões padrão
            println!("Formato inválido ou nenhum valor fornecido. Usando permissões padrão.");
            arquivo.alterar_permissao((
                Permissao::new(false, true, false), // Padrão para dono
                Permissao::new(false, true, false), // Padrão para grupo
                Permissao::new(false, true, false), // Padrão para outros
            ));
        }
    } else {
        println!("Escolha inválida.");
    }
}


// Função para exibir informações de um arquivo
fn exibir_info_arquivo(arquivos: &Vec<Arquivo>) {
    if arquivos.is_empty() {
        println!("Nenhum arquivo cadastrado.");
        return;
    }

    println!("Escolha o número do arquivo para exibir informações:");
    for (i, arquivo) in arquivos.iter().enumerate() {
        println!("{}: Arquivo: {}, Dono: {}, Grupo: {}", i + 1, arquivo.nome, arquivo.usuario.nome, arquivo.grupo.nome);
    }

    let mut escolha_input = String::new();
    io::stdin().read_line(&mut escolha_input).expect("Erro ao ler escolha");
    let escolha: usize = escolha_input.trim().parse().expect("Erro ao converter escolha");

    if escolha > 0 && escolha <= arquivos.len() {
        let arquivo = &arquivos[escolha - 1];
        arquivo.stat();
    } else {
        println!("Escolha inválida.");
    }
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
    let mut arquivos: Vec<Arquivo> = Vec::new();
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
            "3" => menu_arquivo(&mut arquivos, &mut usuarios, &mut grupos),
            "4" => menu_diretorio(),
            "5" => break,
            _ => println!("Opção Inválida!"),
        }
    }   
}