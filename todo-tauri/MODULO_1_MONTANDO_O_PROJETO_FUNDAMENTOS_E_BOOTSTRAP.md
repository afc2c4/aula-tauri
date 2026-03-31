# Módulo 1: Montando o Projeto — Fundamentos, Arquitetura e Bootstrap do Tauri

Arquivos analisados:

- `todo-tauri/package.json`
- `todo-tauri/tsconfig.json`
- `todo-tauri/vite.config.ts`
- `todo-tauri/src-tauri/tauri.conf.json`
- `todo-tauri/src-tauri/Cargo.toml`
- `todo-tauri/src-tauri/src/lib.rs`

## Leitura do pedido, pontos que podem ser seguidos e ambiguidades

### O que dá para seguir com clareza

- Voltar e gerar agora o **Módulo 1**.
- Manter a continuidade didática dos módulos já existentes.
- Preservar o formato já usado:
  - trecho do código
  - explicação
  - trecho do código
  - explicação
- Salvar o material em um arquivo `.md` dentro do repositório.

### Ambiguidades práticas

1. O pedido não diz explicitamente qual arquivo deveria representar o “Módulo 1”.
   - **Decisão tomada:** tratar o Módulo 1 como a base conceitual do projeto, cobrindo os arquivos que explicam a arquitetura, o bootstrap e o fluxo de build do Tauri.

2. Os módulos 2, 3 e 4 focam arquivos únicos, mas um módulo introdutório faz mais sentido como leitura transversal entre frontend, configuração e backend.
   - **Decisão tomada:** este módulo usa múltiplos arquivos e assume o papel de introdução geral antes do mergulho em `main.rs`, `main.ts` e `index.html`.

3. O repositório contém tanto `src-tauri/src/lib.rs` quanto `src-tauri/src/main.rs`, mas os módulos seguintes já analisam `main.rs`.
   - **Decisão tomada:** usar `lib.rs` neste módulo como exemplo do template/base do Tauri e posicionar `main.rs` como evolução posterior explicada no Módulo 2.

---

## A ideia central deste módulo

Antes de entender:

- o estado no Rust,
- o `invoke` no TypeScript,
- e o contrato do DOM no HTML,

é preciso entender **que tipo de aplicação este projeto é**.

O seu projeto não é apenas:

- um site,
- nem apenas um binário Rust,
- nem apenas um frontend Vite.

Ele é a composição de três camadas:

1. **casca web**  
   HTML, CSS e TypeScript renderizam a interface;

2. **runtime desktop do Tauri**  
   o app roda em uma janela nativa e faz a mediação entre frontend e backend;

3. **núcleo nativo em Rust**  
   onde vivem comandos, plugins, configuração e, depois, o estado real da aplicação.

Esse Módulo 1 existe para responder a pergunta mais básica de todas:

> antes de olhar a lógica, como esse projeto fica de pé?

---

## O desenho arquitetural em uma frase

Seu projeto segue o modelo:

- **Vite/TypeScript** para empacotar e servir o frontend;
- **Tauri** para embutir esse frontend em uma aplicação desktop;
- **Rust** para fornecer a camada nativa e, depois, o backend com IPC.

Ou, de forma ainda mais direta:

> o frontend desenha,  
> o Tauri conecta,  
> e o Rust controla.

---

## Onde este módulo entra na sequência

A ordem didática natural agora fica assim:

1. **Módulo 1** — fundamentos, arquitetura, toolchain e bootstrap do Tauri;
2. **Módulo 2** — backend Rust, comandos IPC e estado global;
3. **Módulo 3** — frontend TypeScript, `invoke` e renderização;
4. **Módulo 4** — HTML, DOM e montagem da interface.

Sem este módulo introdutório, os demais já começam no meio da história.

---

## Leitura sequencial dos arquivos-base: trecho do código + explicação

### Trecho 1 — `package.json`

```json
{
  "name": "todo-tauri",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri"
  }
}
```

### Explicação

Esse é o ponto de entrada do lado JavaScript/TypeScript.

#### O que este arquivo diz

Ele define:

- a identidade do pacote frontend;
- o modo de módulos ES (`"type": "module"`);
- os comandos de desenvolvimento e build;
- o papel do Vite como ferramenta principal do lado web.

#### O que os scripts mostram

##### `"dev": "vite"`

Em desenvolvimento, o frontend é servido pelo Vite.

Isso significa que, antes mesmo do Tauri empacotar a aplicação final, existe uma fase em que a interface roda como aplicação web servida localmente.

##### `"build": "tsc && vite build"`

Esse script mostra algo importante:

- primeiro o TypeScript é verificado/compilado;
- depois o Vite empacota o frontend para distribuição.

Ou seja, o frontend tem seu próprio pipeline de construção, separado do Rust, embora os dois depois se encontrem no fluxo do Tauri.

##### `"tauri": "tauri"`

Esse comando delega para a CLI do Tauri.

É a ponte operacional entre o mundo Node/Vite e o mundo Rust/desktop.

---

### Trecho 2 — dependências do `package.json`

```json
"dependencies": {
  "@tauri-apps/api": "^2",
  "@tauri-apps/plugin-opener": "^2"
},
"devDependencies": {
  "@tauri-apps/cli": "^2",
  "vite": "^6.0.3",
  "typescript": "~5.6.2"
}
```

### Explicação

Aqui aparece a primeira pista de que este projeto é híbrido.

#### `@tauri-apps/api`

É a biblioteca usada pelo frontend para conversar com o runtime Tauri.

Mais à frente, no Módulo 3, é exatamente dela que vem o `invoke`.

#### `@tauri-apps/plugin-opener`

Mostra que plugins do ecossistema Tauri também podem ser consumidos.

Mesmo que ele não seja o foco principal da aplicação de tarefas, sua presença mostra que o app desktop pode ganhar capacidades nativas extras por meio de plugins.

#### `@tauri-apps/cli`

É a ferramenta de linha de comando que organiza:

- desenvolvimento;
- build;
- integração entre frontend e backend;
- empacotamento da app.

#### `vite` e `typescript`

Esses dois pacotes mostram que a interface não é HTML puro solto:

- o **TypeScript** modela e valida a lógica do frontend;
- o **Vite** cuida do bundling, do servidor de desenvolvimento e da integração com esse frontend moderno.

---

### Trecho 3 — `tsconfig.json`

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "moduleResolution": "bundler",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true
  },
  "include": ["src"]
}
```

### Explicação

Esse arquivo define como o TypeScript enxerga o frontend.

#### `target` e `module`

O código é preparado para um ambiente moderno:

- `ES2020` como alvo de linguagem;
- `ESNext` como formato de módulos.

Isso combina com o uso de:

- `import`;
- `type="module"`;
- bundling moderno via Vite.

#### `lib: ["ES2020", "DOM", "DOM.Iterable"]`

Aqui o TypeScript passa a entender que o código do frontend vive num ambiente com APIs de navegador/DOM.

Isso é importante porque o `main.ts` acessa:

- `document`;
- `window`;
- `HTMLFormElement`;
- `HTMLInputElement`;
- `HTMLUListElement`.

#### `moduleResolution: "bundler"`

Essa opção deixa claro que o projeto espera um resolvedor moderno, compatível com o fluxo do Vite.

#### `strict`, `noUnusedLocals`, `noUnusedParameters`

Essas flags mostram uma intenção de disciplina:

- mais segurança estática;
- menos código morto;
- menos margem para erro silencioso.

Ou seja, mesmo sendo um projeto didático, o frontend tenta manter padrões saudáveis de tipagem.

---

### Trecho 4 — `vite.config.ts`

```ts
import { defineConfig } from "vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;
```

### Explicação

Agora entramos no ponto em que o Vite começa a ser ajustado especificamente para o Tauri.

#### `defineConfig`

É a forma padrão de declarar a configuração do Vite.

#### `TAURI_DEV_HOST`

Essa variável é relevante porque o Tauri pode precisar coordenar:

- o host do servidor de desenvolvimento;
- a conexão da interface carregada na janela nativa;
- o HMR do frontend.

Ou seja, o Vite aqui não está sendo usado como em um site genérico; ele está sendo preparado para funcionar dentro do fluxo do Tauri.

---

### Trecho 5 — `vite.config.ts`

```ts
export default defineConfig(async () => ({
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
```

### Explicação

Esse bloco já mostra decisões práticas de integração.

#### `clearScreen: false`

Mantém mensagens de erro visíveis no terminal.

Em projetos com frontend + Rust, isso é útil porque reduz o risco de esconder erros relevantes durante o desenvolvimento.

#### `port: 1420`

O Tauri espera um endereço específico de desenvolvimento, então a porta precisa ser previsível.

#### `strictPort: true`

Se a porta estiver ocupada, o Vite falha em vez de escolher outra automaticamente.

Isso é importante porque o Tauri espera encontrar o frontend exatamente onde a configuração manda.

#### `host: host || false`

Permite ajustar o host quando o ambiente de desenvolvimento exige isso.

Mais uma vez, o arquivo mostra que o frontend está subordinado a um runtime maior: ele não roda isoladamente, mas em coordenação com a aplicação desktop.

---

### Trecho 6 — `vite.config.ts`

```ts
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
```

### Explicação

Aqui aparece um detalhe muito importante do dia a dia de desenvolvimento.

#### `hmr`

O Hot Module Replacement é configurado explicitamente para funcionar no contexto esperado pelo Tauri.

Isso reforça que a experiência de desenvolvimento deste projeto é uma composição:

- o Vite recarrega a interface;
- o Tauri hospeda essa interface numa janela desktop;
- o backend Rust roda como camada nativa por trás.

#### `ignored: ["**/src-tauri/**"]`

O Vite ignora mudanças no diretório Rust.

Isso é excelente para entender a separação de responsabilidades:

- arquivos do frontend pertencem ao ciclo de observação do Vite;
- arquivos do backend pertencem ao ecossistema Cargo/Rust;
- não é o bundler web que recompila o backend nativo.

---

### Trecho 7 — `tauri.conf.json`

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "todo-tauri",
  "version": "0.1.0",
  "identifier": "com.codespace.todo-tauri"
}
```

### Explicação

Este é um dos arquivos mais importantes para entender o projeto como aplicação desktop.

#### O que ele representa

Se o `package.json` organiza o lado Node/Vite, o `tauri.conf.json` organiza a aplicação Tauri em si.

Ele define:

- metadados da app;
- build orchestration;
- janelas;
- políticas de segurança;
- bundling.

#### `$schema`

Indica que a configuração segue o esquema da versão 2 do Tauri.

#### `productName`, `version`, `identifier`

Esses campos são metadados de aplicação real, não apenas de frontend:

- nome do produto;
- versão;
- identificador único do app.

Isso mostra que o projeto já está no terreno de software desktop empacotável.

---

### Trecho 8 — `tauri.conf.json`

```json
"build": {
  "beforeDevCommand": "npm run dev",
  "devUrl": "http://localhost:1420",
  "beforeBuildCommand": "npm run build",
  "frontendDist": "../dist"
}
```

### Explicação

Esse bloco é uma das melhores janelas para entender a costura entre frontend e Tauri.

#### `beforeDevCommand`

Antes do app desktop de desenvolvimento subir, o Tauri manda iniciar o servidor do frontend.

#### `devUrl`

Durante o desenvolvimento, a janela Tauri aponta para a URL servida pelo Vite.

Ou seja:

- o frontend continua sendo servido como app web local;
- mas ele aparece dentro da casca desktop do Tauri.

#### `beforeBuildCommand`

Antes do build final da app, o Tauri manda construir o frontend.

#### `frontendDist`

Depois do build web, o Tauri sabe onde pegar os arquivos estáticos gerados.

Esse trecho sozinho já ensina uma parte enorme da arquitetura:

> Tauri não substitui o build do frontend;  
> ele orquestra esse build e depois o incorpora.

---

### Trecho 9 — `tauri.conf.json`

```json
"app": {
  "withGlobalTauri": true,
  "windows": [
    {
      "title": "To-Do List Blindado",
      "width": 800,
      "height": 600
    }
  ],
```

### Explicação

Aqui a aplicação começa a ganhar forma como janela nativa.

#### `withGlobalTauri`

Expõe integração global do Tauri para o contexto da app.

Mesmo quando o projeto usa importações modernas no frontend, esse campo evidencia que o runtime ainda pode oferecer capacidades globais dependendo da estratégia adotada.

#### `windows`

Esse array define a janela da aplicação:

- título;
- largura;
- altura.

É aqui que o projeto deixa de parecer somente “site” e passa a parecer claramente “programa desktop com interface web embarcada”.

---

### Trecho 10 — `tauri.conf.json`

```json
  "security": {
    "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:;"
  }
}
```

### Explicação

Esse bloco é crucial do ponto de vista de segurança.

#### CSP

CSP significa **Content Security Policy**.

Ela limita de onde scripts, estilos e imagens podem ser carregados.

No seu projeto, a política tenta restringir:

- scripts a `self`;
- estilos a `self` e inline;
- imagens a `self` e `data:`.

Isso é importante porque apps desktop baseados em webview continuam precisando de fronteiras de segurança. O fato de a interface rodar dentro de um aplicativo não elimina riscos de conteúdo ativo.

Didaticamente, esse ponto conecta muito bem com o restante da arquitetura:

- o frontend não deveria ter liberdade irrestrita;
- o backend não deveria expor qualquer coisa;
- a superfície precisa ser controlada em várias camadas.

---

### Trecho 11 — `Cargo.toml`

```toml
[package]
name = "todo-tauri"
version = "0.1.0"
description = "A Tauri App"
authors = ["you"]
edition = "2021"
```

### Explicação

Agora entramos no mundo Rust.

Este bloco declara a identidade do pacote nativo.

Ele é equivalente, no lado Cargo, ao papel que o `package.json` exerce no lado Node:

- nome;
- versão;
- descrição;
- metadados básicos do projeto.

O que interessa aqui é perceber que o app tem **duas vidas em paralelo**:

- uma vida como projeto frontend;
- outra vida como crate Rust.

Tauri é justamente o sistema que faz essas duas vidas convergirem.

---

### Trecho 12 — `Cargo.toml`

```toml
[lib]
name = "todo_tauri_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }
```

### Explicação

Esse bloco mostra que o lado Rust não é um detalhe decorativo; ele é parte estruturada da aplicação.

#### `[lib]`

Define a biblioteca Rust do projeto.

Os diferentes `crate-type` mostram que o código pode ser produzido em formatos diferentes conforme a necessidade do Tauri e das plataformas.

#### `tauri-build`

É a dependência de build que ajuda o ecossistema Tauri a preparar a app durante a compilação.

Esse detalhe reforça outra ideia importante:

- o Tauri não é apenas uma biblioteca de runtime;
- ele também participa do processo de build da aplicação.

---

### Trecho 13 — `Cargo.toml`

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rand = "0.10.0"
```

### Explicação

Aqui aparecem os blocos que sustentam a camada nativa.

#### `tauri`

É a dependência principal do runtime/app desktop.

#### `tauri-plugin-opener`

Corresponde ao plugin também visto no lado JavaScript.

Isso mostra que plugins Tauri podem atravessar a arquitetura, envolvendo tanto configuração e runtime quanto, em alguns casos, integrações expostas à aplicação.

#### `serde` e `serde_json`

Esses pacotes são essenciais para serialização.

E serialização é peça-chave para IPC, porque:

- o frontend envia dados;
- o backend recebe;
- o backend responde;
- tudo isso precisa atravessar uma fronteira de representação.

Mesmo antes de entrar no Módulo 2, esse detalhe já prepara o terreno conceitual.

#### `rand`

Está presente no projeto, embora não seja peça central do fluxo mostrado nos módulos atuais.

Isso é um lembrete importante: nem toda dependência presente precisa participar do caminho principal já documentado.

---

### Trecho 14 — `src-tauri/src/lib.rs`

```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
```

### Explicação

Esse trecho é quase um fóssil didático do template padrão.

#### Por que ele importa

Mesmo que a aplicação real de tarefas tenha evoluído em `main.rs`, o `lib.rs` ainda mostra a forma mais simples possível da ideia central do Tauri:

- expor um comando Rust;
- chamar esse comando a partir do frontend;
- devolver uma resposta serializável.

Esse pequeno exemplo já contém, em miniatura, a lógica que depois cresce no Módulo 2:

- em vez de `greet`, surgem comandos reais;
- em vez de resposta simples, surgem listas de tarefas;
- em vez de exemplo estático, surge uma aplicação com estado.

---

### Trecho 15 — `src-tauri/src/lib.rs`

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Explicação

Este é o bootstrap nativo do template.

#### `tauri::Builder::default()`

Cria o builder principal da aplicação.

Ele é o ponto a partir do qual o app:

- registra plugins;
- registra comandos;
- carrega contexto;
- inicializa a execução.

#### `.plugin(...)`

Registra o plugin opener.

#### `.invoke_handler(...)`

É aqui que o comando `greet` é oficialmente exposto.

Esse é um ponto conceitual fortíssimo do Tauri:

- comandos não ficam magicamente disponíveis;
- eles precisam ser registrados explicitamente.

Isso reduz superfície de acesso e reforça a ideia de contrato explícito entre frontend e backend.

#### `.run(tauri::generate_context!())`

Inicializa a aplicação usando o contexto gerado a partir da configuração do projeto.

#### `.expect(...)`

Falha de forma explícita se a aplicação não conseguir iniciar.

---

## Como tudo isso se conecta aos módulos seguintes

Depois deste módulo, a leitura dos demais fica muito mais clara:

- o **Módulo 2** pega o builder e mostra como ele foi adaptado para carregar estado global e registrar comandos reais;
- o **Módulo 3** mostra o frontend usando `invoke` para conversar com esses comandos;
- o **Módulo 4** mostra o HTML que fornece a estrutura física para a interface renderizada.

Ou seja:

- este módulo apresenta a **infraestrutura**;
- o Módulo 2 apresenta o **núcleo nativo de negócio**;
- o Módulo 3 apresenta o **cliente visual desse núcleo**;
- o Módulo 4 apresenta o **palco estrutural da UI**.

---

## Fechamento conceitual do Módulo 1

O grande valor deste projeto, para estudo, é mostrar que um app desktop moderno pode ser entendido como um sistema em camadas:

- uma camada web para interface;
- uma camada de toolchain para build;
- uma camada nativa para execução e controle;
- uma ponte segura entre os dois mundos.

O Módulo 1, portanto, não é sobre uma feature isolada. Ele é sobre a pergunta anterior a todas as outras:

> qual é a máquina arquitetural mínima que precisa existir  
> para que HTML, TypeScript, Tauri e Rust consigam cooperar?

E a resposta, neste repositório, está distribuída justamente nesses arquivos-base:

- `package.json`
- `tsconfig.json`
- `vite.config.ts`
- `tauri.conf.json`
- `Cargo.toml`
- `src-tauri/src/lib.rs`

Eles formam a fundação sobre a qual os módulos 2, 3 e 4 passam a fazer sentido.
