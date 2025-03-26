# MegaStore - Sistema de Busca e Recomendação

Bem-vindo ao sistema de busca e recomendação da MegaStore! Este projeto foi desenvolvido para facilitar a busca por produtos e oferecer recomendações úteis, tornando a experiência de compra mais prática e agradável.

## Funcionalidades

- **Busca por produtos**: Encontre produtos pelo nome ou pela categoria.
- **Recomendações de produtos**: Receba sugestões de produtos relacionados com base nas conexões do sistema.
- **Exibição de nomes relacionados**: Visualize os nomes dos produtos relacionados ao realizar uma busca, tornando as recomendações mais intuitivas.

## Primeira Versão

Esta é a versão inicial do sistema, que já inclui funcionalidades básicas de busca e recomendação. Estamos entusiasmados para expandir e melhorar o projeto com base no feedback dos usuários.

## Estrutura do Projeto

O projeto está organizado em módulos para facilitar a manutenção e a expansão:

- **`src/`**: Contém o código principal do sistema.
  - `produto.rs`: Define a estrutura e funcionalidades dos produtos.
  - `recomendacoes.rs`: Implementa o grafo de recomendações.
  - `busca.rs`: Gerencia o catálogo de produtos e as buscas.
- **`tests/`**: Inclui testes automatizados para validar o funcionamento do sistema.

## Como Executar

Siga os passos abaixo para rodar o projeto:

1. Clone o repositório:
   ```sh
   git clone https://github.com/seu-usuario/mega-search-rs.git
   cd mega-search-rs
   ```

2. Compile o projeto:
   ```sh
   cargo build
   ```

3. Execute os testes:
   ```sh
   cargo test
   ```

4. Rode a aplicação:
   ```sh
   cargo run
   ```

## Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir uma issue ou enviar um pull request. Toda ajuda é muito apreciada.

## Licença

Este projeto está licenciado sob a [MIT License](LICENSE).
