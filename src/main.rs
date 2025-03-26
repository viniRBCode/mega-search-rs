mod produto;
mod busca; // Módulo para funcionalidades de busca
mod recomendacoes; // Módulo para funcionalidades de recomendação

pub use produto::Produto;  // Reexporta a estrutura Produto
pub use busca::CatalogoProdutos;  // Reexporta a estrutura CatalogoProdutos
pub use recomendacoes::GrafoProdutos;  // Reexporta a estrutura GrafoProdutos

// Função principal da aplicação
fn main() {
    // Inicializa o catálogo de produtos e o grafo de recomendações
    let mut catalogo = CatalogoProdutos::novo();
    let mut grafo = GrafoProdutos::novo();

    let produto1 = Produto::novo("101", "Laptop", "Eletrônicos");
    let produto2 = Produto::novo("102", "Mouse", "Periféricos");

    catalogo.adicionar_produto(produto1.clone());
    catalogo.adicionar_produto(produto2.clone());

    grafo.adicionar_produto(produto1.clone());
    grafo.adicionar_produto(produto2.clone());

    // Realiza uma busca no catálogo pelo nome do produto
    println!("🔍 Buscando 'Laptop'...");
    match catalogo.buscar("Laptop") {
        Some(produtos) => println!("Produto encontrado: {:?}", produtos),
        None => println!("Produto não encontrado"),
    }

    // Cria uma relação entre dois produtos no grafo
    grafo.adicionar_relacao("101", "102");

    // Sincroniza os produtos relacionados no catálogo com base no grafo
    grafo.sincronizar_relacionados(&mut catalogo);

    // Exibe recomendações de produtos com base em um produto específico
    println!("🛒 Recomendações para 'Laptop'...");
    match grafo.recomendar("101") {
        Some(recomendados) => println!("Produtos recomendados: {:?}", recomendados),
        None => println!("Nenhuma recomendação encontrada"),
    }

    // Exibe os produtos relacionados diretamente no produto buscado
    if let Some(produto) = catalogo.buscar("Laptop").and_then(|p| p.first()) {
        println!("Produtos relacionados ao '{}': {:?}", produto.nome, produto.relacionados);
    } else {
        println!("Produto 'Laptop' não encontrado no catálogo.");
    }
}
