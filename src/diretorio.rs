use crate::arquivo::Arquivo;
use crate::permissao::Permissao;
use crate::usuario::Usuario;

pub struct Diretorio {
    pub nome: String,
    pub arquivos: Vec<Arquivo>, // Vetor de arquivos no diretório
    pub permissoes: (Permissao, Permissao, Permissao), // Permissões do diretório
    pub dono: Usuario, // Dono do diretório
}

impl Diretorio {
    // Método para criar um novo diretório
    pub fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), dono: Usuario) -> Diretorio {
        Diretorio {
            nome,
            arquivos: Vec::new(),
            permissoes,
            dono,
        }
    }

    // Adiciona um arquivo ao diretório
    pub fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    // Remove um arquivo do diretório pelo nome
    pub fn remove_arquivo(&mut self, nome_arquivo: &str) {
        self.arquivos.retain(|a| a.nome != nome_arquivo);
    }

    // Lista o conteúdo do diretório
    pub fn listar_conteudo(&self) {
        if self.arquivos.is_empty() {
            println!("O diretório {} está vazio", self.nome);
        } else {
            println!("Conteúdo do diretório {}:", self.nome);
            for arquivo in &self.arquivos {
                println!("Arquivo: {} (tamanho: {} bytes)", arquivo.nome, arquivo.tamanho);
            }
        }
    }
}