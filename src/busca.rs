use std::collections::HashMap;
use crate::produto::Produto;

/// Gerencia um catálogo de produtos com índices para busca eficiente.
#[derive(Debug)]
pub struct CatalogoProdutos {
    indice: HashMap<String, Vec<Produto>>,  // Índice por nome
    indice_categoria: HashMap<String, Vec<Produto>>, // Índice por categoria
}

impl CatalogoProdutos {
    /// Cria um catálogo vazio.
    pub fn novo() -> Self {
        CatalogoProdutos {
            indice: HashMap::new(),
            indice_categoria: HashMap::new(),
        }
    }

    /// Adiciona um produto ao catálogo e atualiza os índices.
    pub fn adicionar_produto(&mut self, produto: Produto) {
        self.indice
            .entry(produto.nome.clone())
            .or_insert(Vec::new())
            .push(produto.clone());

        self.indice_categoria
            .entry(produto.categoria.clone())
            .or_insert(Vec::new())
            .push(produto);
    }

    /// Busca produtos pelo nome.
    pub fn buscar(&self, nome: &str) -> Option<&Vec<Produto>> {
        self.indice.get(nome)
    }

    /// Busca produtos pelo nome e retorna uma referência mutável.
    pub fn buscar_mut(&mut self, nome: &str) -> Option<&mut Vec<Produto>> {
        self.indice.get_mut(nome)
    }

    /// Busca produtos pela categoria.
    pub fn buscar_por_categoria(&self, categoria: &str) -> Option<&Vec<Produto>> {
        self.indice_categoria.get(categoria)
    }

    /// Retorna um iterador mutável para todos os produtos no catálogo.
    pub fn iterar_produtos_mut(&mut self) -> impl Iterator<Item = &mut Produto> {
        self.indice.values_mut().flat_map(|produtos| produtos.iter_mut())
    }
}
