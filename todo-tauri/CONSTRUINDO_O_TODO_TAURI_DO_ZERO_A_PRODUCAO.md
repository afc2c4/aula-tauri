# Construindo o Todo-Tauri: Do Zero à Produção

Este guia consolida os quatro módulos do projeto `todo-tauri` em uma trilha única, linear e progressiva.

A ideia é reconstruir o raciocínio do app na ordem certa:

1. entender o ecossistema;
2. configurar o build e o manifesto do aplicativo;
3. montar o backend nativo em Rust;
4. ligar a interface ao backend;
5. compilar o projeto final.

---

## Passo 1: O Bootstrap do Ecossistema (Extraído do Módulo 1)

Antes de falar de tarefas, botões e renderização, é preciso entender que este projeto nasce da cooperação entre três camadas:

- **frontend web** com Vite + TypeScript;
- **runtime desktop** com Tauri;
- **backend nativo** com Rust.

O começo do projeto aparece no `package.json`:

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
  },
  "dependencies": {
    "@tauri-apps/api": "^2",
    "@tauri-apps/plugin-opener": "^2"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2",
    "vite": "^6.0.3",
    "typescript": "~5.6.2"
  }
}
```

- `vite` organiza o ciclo do frontend em desenvolvimento e build.
- `typescript` dá tipagem ao código que vai manipular DOM e IPC.
- `@tauri-apps/api` entrega ao frontend a ponte oficial para conversar com o lado Rust.
- `@tauri-apps/cli` é o elo operacional entre Node, Vite e Tauri.
- `npm run dev` sobe a camada web.
- `npm run build` valida TypeScript e gera o bundle do frontend.
- `npm run tauri ...` delega o restante para o ecossistema Tauri.

Em seguida, o `tsconfig.json` mostra que esse frontend foi preparado para viver num ambiente moderno de navegador embutido:

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

- `DOM` e `DOM.Iterable` deixam explícito que o frontend vai manipular a interface do navegador.
- `moduleResolution: "bundler"` alinha o TypeScript com o fluxo do Vite.
- `strict: true` ajuda a reduzir ambiguidades entre frontend e backend.
- isso é importante porque o Tauri depende de contratos bem definidos entre nomes, tipos e payloads.

No template original do Tauri, o núcleo nativo começa assim:

```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- esse template prova a ideia central do Tauri: **o frontend não chama funções Rust arbitrárias; ele chama comandos expostos**.
- `#[tauri::command]` transforma uma função Rust em um ponto de entrada IPC.
- `.invoke_handler(...)` registra explicitamente quais comandos podem ser acessados.
- `.run(...)` sobe o runtime desktop.
- aqui ainda não existe regra de negócio real, mas já existe a forma da arquitetura que o projeto final vai aproveitar.

Ao final deste passo, o ponto central é:

> o projeto ainda não está “pronto”,  
> mas a máquina que permite HTML, TypeScript, Tauri e Rust cooperarem já está montada.

---

## Passo 2: Configuração de Build e Manifesto (Extraído do Módulo 2)

Com o ecossistema entendido, o próximo passo é definir como o aplicativo é montado, servido e empacotado.

No `vite.config.ts`, o frontend é preparado para conversar corretamente com o Tauri:

```ts
import { defineConfig } from "vite";

export default defineConfig({
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
```

- `port: 1420` fixa a porta usada pelo frontend durante o desenvolvimento.
- `strictPort: true` evita que o Vite “escape” para outra porta e quebre o contrato com o Tauri.
- `ignored: ["**/src-tauri/**"]` evita recarga desnecessária quando arquivos nativos mudam.
- isso mostra que o app desktop depende de uma coordenação fina entre bundler web e runtime nativo.

No `src-tauri/tauri.conf.json`, o Tauri aprende como controlar esse frontend:

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "todo-tauri",
  "version": "0.1.0",
  "identifier": "com.todo-tauri.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist",
    "devUrl": "http://localhost:1420"
  },
  "app": {
    "windows": [
      {
        "title": "todo-tauri",
        "width": 800,
        "height": 600
      }
    ],
    "security": {
      "csp": null
    }
  }
}
```

- `beforeDevCommand` diz ao Tauri qual comando precisa rodar antes da app abrir em modo dev.
- `beforeBuildCommand` garante que o frontend seja empacotado antes do build final.
- `frontendDist` aponta para a saída do Vite em produção.
- `devUrl` aponta para o servidor local do Vite durante desenvolvimento.
- a seção `windows` define o contêiner nativo em que a UI web será exibida.
- esse arquivo funciona como o **manifesto operacional** do aplicativo desktop.

O `Cargo.toml` fecha a configuração do lado Rust:

```toml
[package]
name = "todo-tauri"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rand = "0.9"
```

- `tauri-build` participa da geração e preparação do aplicativo.
- `serde` e `serde_json` são essenciais para serialização entre frontend e backend.
- em Tauri, a IPC só funciona bem porque os dados atravessam a fronteira em formato serializável.
- o Rust não entra só como “linguagem auxiliar”; ele entra como parte estrutural do build e da execução.

Neste ponto, o aluno já consegue responder:

- quem sobe o frontend em desenvolvimento;
- quem gera o bundle web;
- quem embute esse bundle numa janela desktop;
- e onde o app declara sua identidade e seu fluxo de compilação.

---

## Passo 3: Arquitetura do Backend em Rust (Extraído do Módulo 3)

Agora que o projeto já tem base e pipeline, o próximo passo é criar a camada que guarda o estado real da aplicação.

O backend em `src-tauri/src/main.rs` começa com a definição do estado global:

```rust
use std::sync::Mutex;
use tauri::State;

struct AppState {
    tarefas: Mutex<Vec<String>>,
}
```

- `Vec<String>` guarda a lista de tarefas em memória.
- `Mutex<Vec<String>>` protege essa lista contra acessos concorrentes inseguros.
- isso é importante porque, no lado nativo, múltiplas threads podem tentar tocar no mesmo recurso.
- o Rust obriga uma estratégia explícita de sincronização em vez de permitir mutação compartilhada sem controle.
- o estado, aqui, é **volátil**: vive na RAM enquanto o processo estiver aberto.

O primeiro comando expõe a adição de tarefas:

```rust
#[tauri::command]
fn adicionar_tarefa(texto: String, state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let mut lista = state.tarefas.lock().map_err(|_| "Falha ao adquirir o lock do Mutex")?;
    lista.push(texto);
    Ok(lista.clone())
}
```

- `#[tauri::command]` publica a função como ponto de entrada IPC.
- `State<'_, AppState>` injeta o estado gerenciado pela aplicação.
- `.lock()` adquire a chave do `Mutex` antes de tocar na lista.
- o lock existe para serializar o acesso à seção crítica.
- `Ok(lista.clone())` devolve ao frontend um **snapshot serializável**, não uma referência viva da memória interna.

O carregamento inicial do estado segue a mesma lógica:

```rust
#[tauri::command]
fn carregar_tarefas(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let lista = state.tarefas.lock().map_err(|_| "Falha ao adquirir o lock do Mutex")?;
    Ok(lista.clone())
}
```

- o frontend não assume nada sobre o estado; ele consulta o backend.
- isso mantém a autoridade do dado no lado Rust.
- mesmo em leitura, o acesso continua disciplinado via `Mutex`.
- o clone devolvido é a fotografia segura do estado naquele instante.

A remoção por índice completa a API mínima do backend:

```rust
#[tauri::command]
fn remover_tarefa(indice: usize, state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let mut lista = state.tarefas.lock().map_err(|_| "Falha ao adquirir o lock do Mutex")?;

    if indice >= lista.len() {
        return Err("Indice de tarefa invalido".to_string());
    }

    lista.remove(indice);
    Ok(lista.clone())
}
```

- remover por índice evita ambiguidade quando existem textos duplicados.
- a validação defensiva impede acesso inválido vindo do frontend.
- o backend continua sendo a camada que decide se uma mutação é aceitável.
- isso reforça um princípio importante: **o frontend pede; o backend valida**.

Por fim, o `main` liga tudo:

```rust
fn main() {
    tauri::Builder::default()
        .manage(AppState {
            tarefas: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            adicionar_tarefa,
            carregar_tarefas,
            remover_tarefa
        ])
        .run(tauri::generate_context!())
        .expect("Erro fatal: Falha ao iniciar o Tauri");
}
```

- `.manage(...)` registra uma instância central de estado global.
- `.invoke_handler(...)` forma a lista branca dos comandos acessíveis pela UI.
- `.run(...)` sobe a aplicação com esse estado e esses comandos já registrados.
- o resultado arquitetural é simples, mas forte:
  - a UI não toca a memória do Rust diretamente;
  - o estado fica concentrado;
  - a sincronização é explícita;
  - e a superfície exposta é mínima.

Ao final deste passo, o app já possui:

- um núcleo nativo;
- estado em RAM;
- sincronização com `Mutex`;
- e uma API IPC pequena e controlada.

---

## Passo 4: Interface e Integração no Frontend (Extraído do Módulo 4)

Com o backend pronto, a interface agora precisa de duas coisas:

1. uma estrutura HTML estável;
2. um frontend TypeScript que conecte DOM e IPC.

O `index.html` fornece o contrato físico da tela:

```html
<body>
  <div class="container">
    <h1>Tarefas</h1>
    <form id="todo-form">
      <input type="text" id="task-input" placeholder="O que precisa ser feito?" required />
      <button type="submit">Adicionar</button>
    </form>
    <ul id="task-list"></ul>
  </div>
  <script type="module" src="/src/main.ts"></script>
</body>
```

- `todo-form`, `task-input` e `task-list` são o contrato entre HTML e TypeScript.
- o formulário captura a intenção de adicionar tarefas.
- o `input` recebe o texto bruto do usuário.
- o `ul` começa vazio porque o estado verdadeiro não mora no HTML.
- o script de módulo entrega o controle para o `main.ts`.

No `main.ts`, o primeiro passo é capturar os elementos:

```ts
import { invoke } from "@tauri-apps/api/core";

const form = document.getElementById("todo-form") as HTMLFormElement;
const input = document.getElementById("task-input") as HTMLInputElement;
const list = document.getElementById("task-list") as HTMLUListElement;
```

- `invoke` é a ponte oficial do frontend para os comandos Rust.
- as type assertions ajudam o TypeScript a saber qual API DOM está disponível.
- esse trecho só funciona porque o HTML definiu `id`s estáveis no passo anterior.
- aqui fica clara a divisão de papéis:
  - HTML fornece nós;
  - TypeScript fornece comportamento;
  - Rust fornece autoridade sobre o estado.

Ao carregar a página, a UI consulta o estado nativo:

```ts
window.addEventListener("DOMContentLoaded", async () => {
  try {
    const tarefas = await invoke<string[]>("carregar_tarefas");
    renderizarLista(tarefas);
  } catch (err) {
    console.error("Erro ao carregar o estado nativo:", err);
  }
});
```

- `DOMContentLoaded` garante que o DOM já existe antes do uso.
- `invoke<string[]>("carregar_tarefas")` chama exatamente o comando registrado no backend.
- o frontend não inventa o estado; ele pede ao Rust a lista atual.
- isso mantém a UI como cliente do core nativo.

Quando o usuário envia o formulário, o frontend pede a mutação:

```ts
form.addEventListener("submit", async (e) => {
  e.preventDefault();

  const taskText = input.value.trim();
  if (!taskText) return;

  try {
    const tarefasAtualizadas = await invoke<string[]>("adicionar_tarefa", { texto: taskText });
    renderizarLista(tarefasAtualizadas);
    input.value = "";
  } catch (erro) {
    console.error("Erro na comunicação IPC:", erro);
  }
});
```

- `preventDefault()` impede o comportamento padrão do formulário.
- `trim()` remove espaços inúteis antes do envio.
- a validação no frontend melhora a UX, mas não substitui a autoridade do backend.
- `invoke(..., { texto: taskText })` precisa casar com a assinatura Rust.
- a resposta do backend já volta como nova fonte para renderização.

A função de renderização reconstrói a lista a partir do snapshot recebido:

```ts
function renderizarLista(tarefas: string[]) {
  list.innerHTML = "";

  tarefas.forEach((t, indice) => {
    const li = document.createElement("li");
    const texto = document.createElement("span");
    texto.textContent = t;

    const botaoExcluir = document.createElement("button");
    botaoExcluir.type = "button";
    botaoExcluir.textContent = "X";

    botaoExcluir.addEventListener("click", async () => {
      try {
        const tarefasAtualizadas = await invoke<string[]>("remover_tarefa", { indice });
        renderizarLista(tarefasAtualizadas);
      } catch (erro) {
        console.error("Erro ao remover tarefa:", erro);
      }
    });

    li.append(texto, botaoExcluir);
    list.appendChild(li);
  });
}
```

- `list.innerHTML = ""` limpa a renderização anterior antes de redesenhar.
- a UI é reconstruída a partir do retorno do backend, não de um estado local autoritativo.
- `textContent` é mais seguro que injetar HTML arbitrário.
- o botão de exclusão usa o `indice` do snapshot atual para pedir a remoção.
- o fluxo completo agora é:
  - usuário interage;
  - o DOM dispara evento;
  - o TypeScript chama IPC;
  - o Rust decide e devolve;
  - a interface é redesenhada.

Ao final deste passo, o projeto já virou um aplicativo funcional:

- o HTML define o palco;
- o TypeScript orquestra a interação;
- o Rust controla o estado;
- e o Tauri conecta tudo numa janela desktop.

---

## Passo 5: Compilação e Build Final

Com todas as camadas ligadas, o fechamento natural do projeto é a compilação final.

Primeiro, o frontend precisa ser empacotado:

```bash
npm run build
```

- `tsc` valida e compila o TypeScript.
- `vite build` gera o bundle final da interface.
- o resultado vai para a pasta indicada em `frontendDist`.
- esse passo transforma a UI em artefato estático pronto para ser embutido.

Depois, o Tauri pode empacotar a aplicação desktop:

```bash
npm run tauri build
```

- o Tauri executa o build completo da aplicação.
- o frontend já compilado é embutido no app nativo.
- o Rust compila o backend e o runtime necessário.
- o resultado final é um binário desktop adequado ao sistema operacional.

Em termos de arquitetura, esse encerramento é importante porque mostra que:

- o frontend e o backend não são builds independentes no produto final;
- o Tauri orquestra a junção dos dois mundos;
- a aplicação distribuída não é “um site com Rust ao lado”, mas um app desktop coeso.

Se você olhar o projeto de ponta a ponta, o fluxo final fica assim:

```text
Bootstrap do ecossistema
→ configuração de build e manifesto
→ backend Rust com estado e IPC
→ frontend TypeScript integrando DOM e comandos
→ build final da aplicação desktop
```

Esse é o caminho completo do `todo-tauri`:

- começar pela infraestrutura;
- estruturar o ciclo de build;
- dar autoridade ao backend;
- conectar a UI ao core nativo;
- e empacotar tudo como aplicação final.
