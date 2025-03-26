use mega_search_rs::recomendacoes::GrafoProdutos;
use mega_search_rs::produto::Produto;

#[cfg(test)]
mod testes {
    use super::{GrafoProdutos, Produto};

    #[test]
    fn teste_recomendacao() {
        // Cria um grafo de produtos e adiciona dois produtos com uma relação entre eles
        let mut grafo = GrafoProdutos::novo();
        let produto_a = Produto::novo("101", "Produto A", "Categoria X");
        let produto_b = Produto::novo("102", "Produto B", "Categoria Y");

        grafo.adicionar_produto(produto_a);
        grafo.adicionar_produto(produto_b);
        grafo.adicionar_relacao("101", "102");

        // Verifica se as recomendações estão corretas
        assert!(grafo.recomendar("101").is_some());
        assert_eq!(grafo.recomendar("101").unwrap(), &vec!["102".to_string()]);
    }

    #[test]
    fn teste_adicionar_produto() {
        // Testa a adição de um produto ao grafo
        let mut grafo = GrafoProdutos::novo();
        let produto = Produto::novo("101", "Produto A", "Categoria X");

        grafo.adicionar_produto(produto);

        // Verifica se o produto foi adicionado corretamente
        let produto_adicionado = grafo.obter_produto("101").unwrap();
        assert_eq!(produto_adicionado.id, "101");
    }

    #[test]
    fn teste_atualizacao_relacionados() {
        // Testa a atualização dos produtos relacionados no grafo
        let mut grafo = GrafoProdutos::novo();
        let produto_a = Produto::novo("101", "Produto A", "Categoria X");
        let produto_b = Produto::novo("102", "Produto B", "Categoria Y");

        grafo.adicionar_produto(produto_a);
        grafo.adicionar_produto(produto_b);
        grafo.adicionar_relacao("101", "102");

        // Verifica se os produtos relacionados foram atualizados corretamente
        let produto_a = grafo.obter_produto("101").unwrap();
        let produto_b = grafo.obter_produto("102").unwrap();

        assert_eq!(produto_a.relacionados, vec!["102"]);
        assert_eq!(produto_b.relacionados, vec!["101"]);
    }
}
