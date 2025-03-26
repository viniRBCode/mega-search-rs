# MegaStore - Sistema de Busca e Recomendação

Bem-vindo ao sistema de busca e recomendação da MegaStore! Este projeto foi desenvolvido para facilitar a busca por produtos e oferecer recomendações úteis, tornando a experiência de compra mais prática e agradável.

## Funcionalidades

- **Busca por produtos**: Permite encontrar produtos pelo nome ou pela categoria.
- **Recomendações de produtos**: Sugere produtos relacionados com base nas conexões criadas no sistema.

## Primeira Versão

Esta é a primeira versão do sistema, que já conta com funcionalidades básicas de busca e recomendação. Estamos animados para expandir e melhorar o projeto com base no feedback dos usuários.

## Estrutura do Projeto

O projeto está organizado em módulos para facilitar a manutenção e a expansão:

- **`src/`**: Contém o código principal do sistema.
  - `produto.rs`: Define a estrutura dos produtos e suas funcionalidades.
  - `recomendacoes.rs`: Implementa o grafo de recomendações.
  - `busca.rs`: Gerencia o catálogo de produtos e as buscas.
- **`tests/`**: Contém os testes automatizados para validar o funcionamento do sistema.

## Como Executar

Para rodar o projeto, siga os passos abaixo:

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

Se você quiser contribuir, fique à vontade para abrir uma issue ou enviar um pull request. Toda ajuda é bem-vinda!

## Licença

Este projeto está licenciado sob a [MIT License](LICENSE).
