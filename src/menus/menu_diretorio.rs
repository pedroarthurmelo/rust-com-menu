use std::io;

use crate::arquivo::Arquivo;
use crate::diretorio::Diretorio;
use crate::usuario::Usuario;
use crate::permissao::Permissao;

//input para ler a escolha do usuário 
fn ler_escolha() -> String {
    let mut escolha = String::new();
    io::stdin().read_line(&mut escolha).expect("erro ao ler mensagem");
    escolha.trim().to_string()
}

//////////////////////////MENU DIRETÓRIO//////////////////////////////
pub fn menu_diretorio(diretorios: &mut Vec<Diretorio>, arquivos: &mut Vec<Arquivo>, usuarios: &mut Vec<Usuario>) {
    loop {
        println!("\nMenu Diretório:");
        println!("1. Criar Diretório");
        println!("2. Adicionar Arquivo ao Diretório");
        println!("3. Remover Arquivo do Diretório");
        println!("4. Listar Conteúdo do Diretório");
        println!("5. Voltar");

        let escolha = ler_escolha();

        match escolha.as_str() {
            "1" => criar_diretorio(diretorios, usuarios),
            "2" => adicionar_arquivo_diretorio(diretorios, arquivos),
            "3" => remover_arquivo_diretorio(diretorios),
            "4" => listar_conteudo_diretorio(diretorios),
            "5" => return, // Volta para o menu principal
            _ => println!("Opção inválida!"),
        }
    }
}

fn criar_diretorio(diretorios: &mut Vec<Diretorio>, usuarios: &mut Vec<Usuario>) {
    if usuarios.is_empty() {
        println!("Nenhum usuário cadastrado ainda. Cadastre um usuário primeiro.");
        return;
    }

    // Solicita o nome do diretório
    println!("Digite o nome do diretório: ");
    let mut nome_input = String::new();
    io::stdin().read_line(&mut nome_input).expect("Erro ao ler nome do diretório");
    let nome = nome_input.trim().to_string();

    // Solicita as permissões do diretório
    println!("Digite as permissões para o diretório (ex: 755): ");
    let mut permissoes_input = String::new();
    io::stdin().read_line(&mut permissoes_input).expect("Erro ao ler permissões");
    let permissoes: u32 = permissoes_input.trim().parse().expect("Erro ao converter permissões");
    
    // Converte o número de permissões para o formato (dono, grupo, outros)
    let dono = (permissoes / 100) % 10;
    let grupo = (permissoes / 10) % 10;
    let outros = permissoes % 10;

    let permissoes_diretorio = (
        Permissao::new(dono >= 4, dono >= 2, dono >= 1),
        Permissao::new(grupo >= 4, grupo >= 2, grupo >= 1),
        Permissao::new(outros >= 4, outros >= 2, outros >= 1),
    );

    // Solicita o dono do diretório
    println!("Escolha o número do usuário que será o dono do diretório:");
    for (i, usuario) in usuarios.iter().enumerate() {
        println!("{}: Nome: {}, UID: {}", i + 1, usuario.nome, usuario.uid);
    }
    let mut escolha_usuario_input = String::new();
    io::stdin().read_line(&mut escolha_usuario_input).expect("Erro ao ler escolha");
    let escolha_usuario: usize = escolha_usuario_input.trim().parse().expect("Erro ao converter escolha");

    let dono = &usuarios[escolha_usuario - 1];

    // Cria o diretório
    let novo_diretorio = Diretorio::new(nome.clone(), permissoes_diretorio, dono.clone());

    diretorios.push(novo_diretorio);
    println!("Diretório '{}' criado com sucesso!", nome);
}

fn adicionar_arquivo_diretorio(diretorios: &mut Vec<Diretorio>, arquivos: &mut Vec<Arquivo>) {
    if arquivos.is_empty() {
        println!("Nenhum arquivo disponível para adicionar.");
        return;
    }

    println!("Escolha o diretório para adicionar um arquivo: ");
    for (i, diretorio) in diretorios.iter().enumerate() {
        println!("{}: Diretório: {}", i + 1, diretorio.nome);
    }

    let mut escolha_diretorio_input = String::new();
    io::stdin().read_line(&mut escolha_diretorio_input).expect("Erro ao ler escolha");
    let escolha_diretorio: usize = escolha_diretorio_input.trim().parse().expect("Erro ao converter escolha");

    if escolha_diretorio > 0 && escolha_diretorio <= diretorios.len() {
        let diretorio = &mut diretorios[escolha_diretorio - 1];

        println!("Escolha o arquivo para adicionar ao diretório: ");
        for (i, arquivo) in arquivos.iter().enumerate() {
            println!("{}: Arquivo: {}", i + 1, arquivo.nome);
        }

        let mut escolha_arquivo_input = String::new();
        io::stdin().read_line(&mut escolha_arquivo_input).expect("Erro ao ler escolha");
        let escolha_arquivo: usize = escolha_arquivo_input.trim().parse().expect("Erro ao converter escolha");

        if escolha_arquivo > 0 && escolha_arquivo <= arquivos.len() {
            let arquivo = &arquivos[escolha_arquivo - 1];
            diretorio.adiciona_arquivo(arquivo.clone());
            println!("Arquivo '{}' adicionado ao diretório '{}'", arquivo.nome, diretorio.nome);
        } else {
            println!("Escolha de arquivo inválida.");
        }
    } else {
        println!("Escolha de diretório inválida.");
    }
}

fn remover_arquivo_diretorio(diretorios: &mut Vec<Diretorio>) {
    if diretorios.is_empty() {
        println!("Nenhum diretório disponível.");
        return;
    }

    println!("Escolha o diretório para remover um arquivo: ");
    for (i, diretorio) in diretorios.iter().enumerate() {
        println!("{}: Diretório: {}", i + 1, diretorio.nome);
    }

    let mut escolha_diretorio_input = String::new();
    io::stdin().read_line(&mut escolha_diretorio_input).expect("Erro ao ler escolha");
    let escolha_diretorio: usize = escolha_diretorio_input.trim().parse().expect("Erro ao converter escolha");

    if escolha_diretorio > 0 && escolha_diretorio <= diretorios.len() {
        let diretorio = &mut diretorios[escolha_diretorio - 1];

        println!("Escolha o arquivo para remover do diretório: ");
        for (i, arquivo) in diretorio.arquivos.iter().enumerate() {
            println!("{}: Arquivo: {}", i + 1, arquivo.nome);
        }

        let mut escolha_arquivo_input = String::new();
        io::stdin().read_line(&mut escolha_arquivo_input).expect("Erro ao ler escolha");
        let escolha_arquivo: usize = escolha_arquivo_input.trim().parse().expect("Erro ao converter escolha");

        if escolha_arquivo > 0 && escolha_arquivo <= diretorio.arquivos.len() {
            let arquivo = &diretorio.arquivos[escolha_arquivo - 1]; // Empréstimo imutável
            let nome_arquivo = arquivo.nome.clone(); // Armazenando o nome do arquivo

        // Agora você pode fazer um empréstimo mutável, pois o empréstimo imutável de `diretorio` foi liberado
        diretorio.remove_arquivo(&nome_arquivo);

        println!("Arquivo '{}' removido do diretório '{}'", nome_arquivo, diretorio.nome); // Usando nome_arquivo

        } else {
            println!("Escolha de arquivo inválida.");
        }
    } else {
        println!("Escolha de diretório inválida.");
    }
}

fn listar_conteudo_diretorio(diretorios: &Vec<Diretorio>) {
    if diretorios.is_empty() {
        println!("Nenhum diretório disponível.");
        return;
    }

    println!("Escolha o diretório para listar o conteúdo: ");
    for (i, diretorio) in diretorios.iter().enumerate() {
        println!("{}: Diretório: {}", i + 1, diretorio.nome);
    }

    let mut escolha_diretorio_input = String::new();
    io::stdin().read_line(&mut escolha_diretorio_input).expect("Erro ao ler escolha");
    let escolha_diretorio: usize = escolha_diretorio_input.trim().parse().expect("Erro ao converter escolha");

    if escolha_diretorio > 0 && escolha_diretorio <= diretorios.len() {
        let diretorio = &diretorios[escolha_diretorio - 1];
        if diretorio.arquivos.is_empty() {
            println!("O diretório '{}' está vazio.", diretorio.nome);
        } else {
            println!("Conteúdo do diretório '{}':", diretorio.nome);
            for arquivo in diretorio.arquivos.iter() {
                println!("Arquivo: {}", arquivo.nome);
            }
        }
    } else {
        println!("Escolha de diretório inválida.");
    }
}