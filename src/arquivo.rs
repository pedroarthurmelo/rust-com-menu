use crate::usuario::Usuario;
use crate::grupo::Grupo;
use crate::permissao::Permissao;

#[derive(Clone)]
pub struct Arquivo {
    pub nome: String,
    pub tamanho: u64,
    pub permissoes: (Permissao, Permissao, Permissao), // Permissões para dono, grupo, e outros
    pub usuario: Usuario, // Dono do arquivo
    pub grupo: Grupo, // Grupo associado ao arquivo
}

impl Arquivo {
    // Método para criar um novo arquivo com permissões padrão
    pub fn new(nome: String, tamanho: u64, usuario: Usuario, grupo: Grupo) -> Arquivo {
        // Permissões padrão: leitura = false, escrita = true, execução = falsef
        let permissoes = (
            Permissao::new(false, true, false), // Permissão para dono
            Permissao::new(false, true, false), // Permissão para grupo
            Permissao::new(false, true, false), // Permissão para outros
        );

        Arquivo {
            nome,
            tamanho,
            permissoes,
            usuario,
            grupo,
        }
    }

    // Método para alterar a permissão do arquivo
    pub fn alterar_permissao(&mut self, nova_permissao: (Permissao, Permissao, Permissao)) {
        self.permissoes = nova_permissao;
    }

    // Método para exibir informações do arquivo
    pub fn stat(&self) {
        let (octal, rwx) = Permissao::octal_e_rwx_total(&self.permissoes.0, &self.permissoes.1, &self.permissoes.2);
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {}", self.tamanho);
        println!("Permissões: ({}/{})", octal, rwx);
        println!("Uid: {}", self.usuario.uid);
        println!("Gid: {}", self.grupo.gid);
    }
}