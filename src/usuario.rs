use crate::grupo::Grupo;
#[derive(Clone)]
pub struct Usuario {
    pub nome: String,
    pub uid: u16,
    pub grupo: Vec<Grupo>, // vetor para armazenar os grupos do usuário
}

impl Usuario {
    // implementação para criar um novo usuário com nome e uid
    pub fn new(nome: String, uid: u16) -> Usuario {
        Usuario {
            nome,
            uid,
            grupo: Vec::new(),
        }
    }

    // Adiciona um grupo ao vetor de grupos do usuário
    pub fn adiciona_membro(&mut self, grupo: Grupo) {
        self.grupo.push(grupo); //push adiciona no final do vetor o usuario novo
    }

    // Remove um grupo do vetor de grupos do usuário, filtrando pelo nome do grupo
    pub fn remove_grupo(&mut self, gid: u16) {
        self.grupo.retain(|g| g.gid != gid);
    }  //retain filtra os
    

    // Lista todos os grupos aos quais o usuário pertence
    pub fn listar_grupos(&self) {
        for grupo in &self.grupo {
            println!("Grupo: {} (GID: {})", grupo.nome, grupo.gid);
        }
    
    } 
}