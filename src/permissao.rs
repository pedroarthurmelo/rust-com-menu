pub struct Permissao{
    leitura: bool,
    escrita: bool,
    execucao: bool,
}

impl Permissao {
    pub fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao{
        Permissao {
            leitura,
            escrita,
            execucao,
        }
    }
    pub fn octal(&self) -> u8{
        let a: u8 = if self.leitura {1} else {0};
        let b: u8 = if self.escrita {1} else {0};
        let c: u8 = if self.execucao {1} else {0};

        match (a, b, c){
            (0,0,0) => 0,
            (0,0,1) => 1,
            (0,1,0) => 2,
            (0,1,1) => 3,
            (1,0,0) => 4,
            (1,0,1) => 5,
            (1,1,0) => 6,
            (1,1,1) => 7,
            _ => 20,

        }
    }
    pub fn rwx(&self) -> String {
        let r = if self.leitura {'r'} else {'-'};
        let w = if self.escrita {'w'} else {'-'};
        let x = if self.execucao {'x'} else {'-'};
        format!("{}{}{}", r, w, x)
    }
    pub fn octal_e_rwx_total(dono: &Permissao, grupo: &Permissao, outros: &Permissao) -> (String, String) {
        let octal = format!("{}{}{}", dono.octal(), grupo.octal(), outros.octal());
        let rwx = format!("{}{}{}", dono.rwx(), grupo.rwx(), outros.rwx());
        (octal, rwx)
    }
}
