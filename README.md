# To-Do Rust Server

Um servidor web simples para uma lista de tarefas (To-Do list) construído em Rust, demonstrando uma arquitetura em camadas e o uso de tecnologias modernas do ecossistema Rust.

## ✨ Funcionalidades

- Criação de novas tarefas.
- Listagem de todas as tarefas existentes.
- API RESTful.

## 🛠️ Tecnologias Utilizadas

- **Linguagem:** [Rust](https://www.rust-lang.org/)
- **Framework Web:** [Actix Web](https://actix.rs/)
- **ORM:** [SeaORM](https://www.sea-ql.org/SeaORM/)
- **Banco de Dados:** [PostgreSQL](https://www.postgresql.org/)
- **Runtime Assíncrono:** [Tokio](https://tokio.rs/)
- **Migrações de Banco de Dados:** [sea-orm-migration](https://www.sea-ql.org/SeaORM/docs/migration/introduction)
- **Containerização:** [Docker](https://www.docker.com/) & [Docker Compose](https://docs.docker.com/compose/) para o ambiente de banco de dados.
- **Configuração:** Variáveis de ambiente gerenciadas com o crate `dotenv`.

## 🏗️ Arquitetura do Projeto

O projeto segue uma arquitetura em camadas (Layered Architecture), inspirada em princípios da Clean Architecture, para separar as responsabilidades e promover um baixo acoplamento entre os componentes.

A estrutura de diretórios principal é:

```
.
├── migration/      # Crate separado para migrações de banco de dados
└── src/
    ├── app/        # Lógica de negócio e regras da aplicação (camada de domínio/aplicação)
    │   ├── entities/
    │   ├── repositories/
    │   └── use_cases/
    ├── infra/      # Detalhes de implementação (camada de infraestrutura)
    │   ├── database/
    │   └── http/
    └── main.rs     # Ponto de entrada, configuração do servidor
```

### Camadas

1. **Domínio/Aplicação (`src/app`)**:

   - **`entities`**: Contém as estruturas de dados centrais e a lógica de negócio (ex: `Task`). Não depende de nenhuma outra camada.
   - **`repositories`**: Define os `traits` (interfaces) para a persistência de dados. Abstrai os detalhes do banco de dados, definindo os contratos que a camada de infraestrutura deve implementar.
   - **`use_cases`**: Orquestra o fluxo de dados e implementa as regras de negócio específicas da aplicação. Utiliza os `traits` de repositório para interagir com os dados e opera sobre as entidades.

2. **Infraestrutura (`src/infra`)**:
   - **`http`**: Contém os controladores (handlers do Actix Web) que expõem a API. Recebe as requisições HTTP, as traduz para chamadas nos `use_cases` e formata as respostas.
   - **`database`**: Fornece a implementação concreta dos `traits` de repositório definidos na camada de aplicação, utilizando SeaORM. Também gerencia a configuração e a conexão com o banco de dados.

### Fluxo de Dependência

O fluxo de dependência é unidirecional, apontando para o centro (a camada de aplicação), o que garante que a lógica de negócio permaneça independente dos detalhes de implementação.

`Controller (infra)` → `Use Case (app)` → `Repository Trait (app)` ← `Repository Impl (infra)`

## 🚀 Como Executar

### Pré-requisitos

- Rust
- Docker e Docker Compose

### Passos

1. **Clonar o repositório:**

   ```sh
   git clone https://github.com/DanielZanad/todo-rust-server.git
   cd todo-rust-server
   ```

2. **Configurar variáveis de ambiente:**
   Crie um arquivo `.env` na raiz do projeto, baseado no `.env-example`. Para o ambiente de desenvolvimento com Docker, você pode usar:

   ```env
   DATABASE_USER=docker
   DATABASE_PASSWORD=docker
   DATABASE_NAME=todo
   DATABASE_HOST=localhost
   ```

3. **Iniciar o banco de dados:**
   Execute o Docker Compose para iniciar o container do PostgreSQL.

   ```sh
   docker-compose up -d
   ```

4. **Executar as migrações:**
   Navegue até o diretório de migração e execute o CLI para aplicar as migrações no banco de dados.

   ```sh
   cd migration
   cargo run
   cd ..
   ```

5. **Iniciar o servidor:**
   Volte para a raiz do projeto e inicie o servidor Actix.

   ```sh
   cargo run
   ```

O servidor estará rodando em `http://127.0.0.1:3000`.

## 🔌 API Endpoints

- **`GET /`**

  - Retorna uma mensagem de boas-vindas.
  - Resposta: `hey there`

- **`GET /tasks`**

  - Lista todas as tarefas.

- **`POST /task`**

  - Cria uma nova tarefa.
  - Corpo da Requisição (JSON):

        ```json
        {
          "name": "Nova Tarefa",
          "content": "Conteúdo da nova tarefa",
          "state": "A Fazer"
        }
        ```

  - Resposta: `200 OK`
