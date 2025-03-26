use std::collections::HashMap;
use crate::produto::Produto;
use crate::busca::CatalogoProdutos;

/// Representa um grafo para gerenciar recomendações de produtos.
#[derive(Debug)]
pub struct GrafoProdutos {
    conexoes: HashMap<String, Vec<String>>,  // Mapeia um produto para seus IDs relacionados
    produtos: HashMap<String, Produto>,      // Mapeia IDs para os produtos correspondentes
}

impl GrafoProdutos {
    /// Cria um grafo vazio.
    pub fn novo() -> Self {
        GrafoProdutos {
            conexoes: HashMap::new(),
            produtos: HashMap::new(),
        }
    }

    /// Adiciona um produto ao grafo.
    pub fn adicionar_produto(&mut self, produto: Produto) {
        self.produtos.insert(produto.id.clone(), produto);
    }

    /// Cria uma relação de recomendação entre dois produtos.
    pub fn adicionar_relacao(&mut self, produto_id: &str, relacionado_id: &str) {
        // Atualiza as conexões no grafo
        self.conexoes
            .entry(produto_id.to_string())
            .or_insert_with(Vec::new)
            .push(relacionado_id.to_string());

        self.conexoes
            .entry(relacionado_id.to_string())
            .or_insert_with(Vec::new)
            .push(produto_id.to_string());

        // Atualiza os produtos diretamente
        if let Some(produto) = self.produtos.get_mut(produto_id) {
            produto.adicionar_relacionado(relacionado_id);
        }

        if let Some(produto) = self.produtos.get_mut(relacionado_id) {
            produto.adicionar_relacionado(produto_id);
        }
    }

    /// Retorna os IDs dos produtos relacionados a um produto específico.
    pub fn recomendar(&self, produto_id: &str) -> Option<&Vec<String>> {
        self.conexoes.get(produto_id)
    }

    /// Atualiza os produtos relacionados no catálogo com base no grafo.
    pub fn sincronizar_relacionados(&self, catalogo: &mut CatalogoProdutos) {
        for (produto_id, relacionados) in &self.conexoes {
            if let Some(produto) = catalogo.iterar_produtos_mut().find(|p| p.id == *produto_id) {
                produto.relacionados = relacionados.clone(); // Atualiza corretamente o campo relacionados
            }
        }
    }

    /// Retorna uma referência imutável a um produto pelo ID.
    pub fn obter_produto(&self, produto_id: &str) -> Option<&Produto> {
        self.produtos.get(produto_id)
    }
}
