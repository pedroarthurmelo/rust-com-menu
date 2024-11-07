use crate::usuario::Usuario;

#[derive(Clone)]
pub struct Grupo {
    pub nome: String,
    pub gid: u16,
    pub membros: Vec<Usuario>,
}

impl Grupo {
    // Construtor para criar um novo grupo com nome e gid
    pub fn new(nome: String, gid: u16) -> Grupo {
        Grupo {
            nome,
            gid,
            membros: Vec::new(),
        }
    }

    // Adiciona um usuário ao grupo
    pub fn adiciona_membro(&mut self, usuario: Usuario) {
        self.membros.push(usuario);
    }

    // Remove um membro do grupo pelo nome do usuário
    pub fn remover_membro(&mut self, uid: u16) {
        self.membros.retain(|u| u.uid != uid);
    }

    // Lista todos os membros do grupo
    pub fn listar_grupos_membros(&self) {
        for membro in &self.membros {
            println!("Membro: {} (uid: {})", membro.nome, membro.uid);
        }
    }
}