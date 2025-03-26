use std::fmt;

/// Representa um produto no catálogo, incluindo informações básicas e produtos relacionados.
#[derive(Clone)]
pub struct Produto {
    pub id: String,
    pub nome: String,
    pub categoria: String,
    pub relacionados: Vec<String>,  // IDs dos produtos relacionados
}

impl Produto {
    /// Cria um novo produto com ID, nome e categoria.
    pub fn novo(id: &str, nome: &str, categoria: &str) -> Self {
        Produto {
            id: id.to_string(),
            nome: nome.to_string(),
            categoria: categoria.to_string(),
            relacionados: Vec::new(),
        }
    }

    /// Adiciona um produto relacionado, evitando duplicatas.
    pub fn adicionar_relacionado(&mut self, produto_id: &str) {
        if !self.relacionados.contains(&produto_id.to_string()) {
            self.relacionados.push(produto_id.to_string());
        }
    }

    /// Exibe os produtos relacionados de forma legível.
    pub fn exibir_relacionados(&self) {
        if self.relacionados.is_empty() {
            println!("Nenhum produto relacionado ao '{}'.", self.nome);
        } else {
            println!(
                "Produtos relacionados ao '{}': {}",
                self.nome,
                self.relacionados.join(", ")
            );
        }
    }

    /// Substitui a lista de produtos relacionados.
    pub fn atualizar_relacionados(&mut self, novos_relacionados: Vec<String>) {
        self.relacionados = novos_relacionados;
    }
}

impl fmt::Debug for Produto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Produto")
            .field("id", &self.id)
            .field("nome", &self.nome)
            .field("categoria", &self.categoria)
            .field("relacionados", &self.relacionados)
            .finish()
    }
}
