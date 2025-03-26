#[cfg(test)]
mod testes {
    use mega_search_rs::produto::{Produto, CatalogoProdutos};

    #[test]
    fn teste_busca_produto() {
        // Verifica a busca de um produto pelo nome
        let mut catalogo = CatalogoProdutos::novo();
        let produto = Produto::novo("101", "Laptop", "Eletrônicos");
        catalogo.adicionar_produto(produto.clone());

        let resultados = catalogo.buscar("Laptop");
        assert!(resultados.is_some());
        assert_eq!(resultados.unwrap().len(), 1);
    }

    #[test]
    fn teste_busca_categoria() {
        // Verifica a busca de produtos por categoria
        let mut catalogo = CatalogoProdutos::novo();
        let produto = Produto::novo("102", "Mouse", "Periféricos");
        catalogo.adicionar_produto(produto.clone());

        let resultados = catalogo.buscar_por_categoria("Periféricos");
        assert!(resultados.is_some());
        assert_eq!(resultados.unwrap().len(), 1);
    }
}
