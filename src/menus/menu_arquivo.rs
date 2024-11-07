use std::io;

use crate::arquivo::Arquivo;
use crate::grupo::Grupo;
use crate::usuario::Usuario;
use crate::permissao::Permissao;

//input para ler a escolha do usuário 
fn ler_escolha() -> String {
    let mut escolha = String::new();
    io::stdin().read_line(&mut escolha).expect("erro ao ler mensagem");
    escolha.trim().to_string()
}

////////////////////////// MENU ARQUIVO ///////////////////////////

pub fn menu_arquivo(arquivos: &mut Vec<Arquivo>, usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
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