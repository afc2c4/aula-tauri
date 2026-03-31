# Módulo 1 — O Frontend Agnóstico

## Visão geral

Neste primeiro módulo, a ideia central é mostrar que a interface foi construída como uma camada **agnóstica ao sistema operacional** e **quase agnóstica ao backend**. O HTML desenha a estrutura, o CSS define a aparência e o TypeScript atua como cola entre DOM e IPC. Até aqui, o frontend não sabe nada sobre `Mutex`, `State`, heap do processo Rust ou detalhes de threads nativas. Ele só conhece:

1. elementos do DOM;
2. eventos do usuário;
3. um contrato de chamada assíncrona que depois será ligado ao backend.

Essa separação é importante do ponto de vista de Sistemas Operacionais:

- a **WebView** roda como uma sandbox de interface, com seu próprio motor de rendering e loop de eventos;
- o **core Rust** roda como o processo nativo da aplicação, com acesso controlado a memória e recursos do SO;
- a conversa entre os dois lados não é feita por ponteiros compartilhados, mas por **mensagens**.

Em outras palavras: o frontend desenha, coleta intenção do usuário e renderiza estado; o backend detém a autoridade sobre o dado.

---

## 1. `index.html`

### A Arquitetura

O `index.html` foi reduzido a uma interface bem direta: um contêiner, um formulário e uma lista. Isso é didaticamente forte porque expõe o que realmente importa:

- um campo de entrada (`input`) para capturar intenção;
- um `form` para modelar a ação de envio;
- uma `ul` para representar visualmente um conjunto de tarefas.

Do ponto de vista de arquitetura, esse arquivo é a **casca declarativa da UI**. Ele não contém regra de negócio, não persiste nada e não tenta “gerenciar estado” sozinho. Isso evita que o estado fique espalhado entre HTML, atributos soltos e scripts inline complexos.

Em termos de fundamentos:

- o navegador/WebView mantém uma **árvore DOM em memória**;
- cada elemento (`form`, `input`, `ul`) vira uma estrutura interna manipulada pelo motor da WebView;
- quando o usuário digita ou clica, o motor dispara eventos no loop de eventos da UI;
- o TypeScript captura esses eventos e decide se deve pedir alguma operação ao backend.

Isso é um desenho clássico de isolamento: a interface não toca diretamente a memória do processo Rust. Não existe um ponteiro JS apontando para `Vec<String>` do backend. O canal entre os dois mundos é indireto e controlado.

### A Diferença em relação ao template vazio do Tauri

Comparando com o template padrão:

- saem os logos do Vite/Tauri;
- sai a tela de “greet”/saudação de exemplo;
- entra uma interface funcional de lista de tarefas;
- o HTML passa a representar um caso de uso real, e não só uma demo de bootstrap.

Outra diferença importante: o estilo principal da tela foi colocado **inline** no próprio `index.html`, o que torna o arquivo mais autocontido para estudo inicial. Para uma aula, isso ajuda porque o aluno enxerga estrutura e aparência juntas. Em contrapartida, o `src/styles.css` fica como artefato do template original, o que também é um excelente gancho pedagógico para discutir “código vivo” versus “sobras de scaffolding”.

### O Arquivo Completo

Arquivo: `/home/runner/work/aula-tauri/aula-tauri/todo-tauri/index.html`

Observação didática: o bloco abaixo reproduz **literalmente** o arquivo atual, inclusive a referência externa a jQuery, que deve ser lida em aula como um vestígio/anti-exemplo de endurecimento de superfície.

```html
<!DOCTYPE html>
<html lang="pt-BR">

<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>To-Do List Blindado</title>
  <style>
    body {
      font-family: sans-serif;
      background: #1e1e1e;
      color: #fff;
      padding: 2rem;
    }

    .container {
      max-width: 600px;
      margin: 0 auto;
    }

    input,
    button {
      padding: 0.5rem;
      font-size: 1rem;
    }

    ul {
      list-style: none;
      padding: 0;
    }

    li {
      background: #333;
      margin: 0.5rem 0;
      padding: 1rem;
      display: flex;
      align-items: center;
      justify-content: space-between;
    }

    li button {
      background: #b42318;
      color: #fff;
      border: 0;
      border-radius: 6px;
      padding: 0.35rem 0.75rem;
      cursor: pointer;
    }
  </style>
</head>

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
  <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>
</body>

</html>
```

### Leitura crítica para aula

Há um detalhe excelente para discutir em sala: existe um `<script>` externo de jQuery no final do arquivo, mas o código da aplicação não usa jQuery. Além disso, ele aparece depois do `main.ts`, o que reforça que não participa do fluxo real da aplicação. Isso mostra três coisas:

1. nem tudo que está no HTML participa da arquitetura de fato;
2. dependência não utilizada aumenta superfície mental e pode aumentar superfície de ataque;
3. em Tauri, a política de segurança depois pode impedir esse carregamento externo.

Então, para a aula, a leitura correta não é “o app depende de jQuery”, mas sim “o repositório ainda carrega um vestígio que idealmente deveria ser removido para reduzir ruído e superfície de ataque”.

Ou seja: o HTML já serve para introduzir o princípio de **mínimo privilégio** antes mesmo de falar de backend.

---

## 2. `src/styles.css`

### A Arquitetura

Esse arquivo é interessante justamente porque ele **não está dirigindo a UI atual**. Em um projeto de engenharia real, isso acontece muito: o scaffold gera uma base visual, mas a aplicação evolui para outro desenho e parte do CSS original fica para trás.

Didaticamente, isso ajuda a explicar:

- o que veio do template;
- o que foi realmente adotado no produto final;
- por que é importante distinguir “arquivo existente” de “arquivo efetivamente carregado”.

No estado atual do projeto, o estilo dominante da página está no bloco `<style>` do `index.html`. Como o `main.ts` não importa `./styles.css`, esse CSS não participa do bundle carregado pelo entrypoint atual do frontend.

Do ponto de vista de fundamentos, isso significa que:

- esse arquivo existe no sistema de arquivos;
- mas não entra no grafo de dependências do bundler do jeito que o código está hoje;
- portanto, não afeta a renderização final da UI em produção, salvo se for importado explicitamente depois.

É um ótimo exemplo para mostrar a diferença entre **presença física no disco** e **presença lógica na execução**.

### A Diferença em relação ao template vazio do Tauri

Aqui a diferença é quase invertida: este arquivo está praticamente com a cara do template original. Ele preserva:

- classes de logo;
- layout centralizado do exemplo padrão;
- estilos para input e button do exemplo de saudação;
- modo claro/escuro do scaffold.

Ou seja, a principal diferença não é uma transformação interna do arquivo, mas sim o fato de que ele deixou de ser o centro visual da aplicação. Isso é valioso para a aula porque mostra uma trilha real de evolução: o desenvolvedor começou com o template, personalizou o HTML e o TypeScript, e o CSS padrão virou um remanescente.

### O Arquivo Completo

Arquivo: `/home/runner/work/aula-tauri/aula-tauri/todo-tauri/src/styles.css`

Observação didática: o bloco abaixo foi mantido **literalmente** para preservar o estado real do repositório, inclusive detalhes de formatação herdados do scaffold.

```css
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.typescript:hover {
  filter: drop-shadow(0 0 2em #2d79c7);
}
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
```

### Leitura crítica para aula

Esse arquivo é perfeito para introduzir um conceito de engenharia muito importante: **nem todo arquivo do repositório participa do caminho crítico de execução**.

Isso conversa com Sistemas Operacionais de forma interessante:

- o arquivo está persistido em disco;
- o bundler decide se ele será incorporado ao artefato final;
- só o que entra no bundle impacta a memória do processo da UI em runtime.

Portanto, “estar no projeto” não é o mesmo que “estar carregado no processo”.

---

## 3. `src/main.ts` — a casca de orquestração

### A Arquitetura

O `main.ts` é o primeiro ponto em que a UI ganha comportamento. Ainda assim, ele continua sendo uma camada fina. A palavra-chave aqui é **orquestração**:

- captura referências do DOM;
- escuta eventos;
- chama o backend de forma assíncrona;
- redesenha a lista com o estado recebido.

Isso é importante porque evita que o frontend “invente” um estado paralelo. Em vez de manter uma cópia autoritativa local e sincronizar depois, ele pede a lista ao backend e renderiza o resultado devolvido. Isso reduz inconsistência.

Pelo prisma de fundamentos:

- `window.addEventListener("DOMContentLoaded", ...)` registra um callback no loop de eventos da WebView;
- o `submit` do formulário evita o comportamento padrão do navegador (`preventDefault`) para não causar reload;
- cada `await invoke(...)` suspende a continuação lógica da função sem travar a thread de interface;
- `renderizarLista` reconstrói a árvore visual da lista no DOM.

Repare no isolamento: o frontend não manipula endereços de memória do Rust, não compartilha lock, não acessa `Vec<String>` diretamente. Ele recebe **cópias serializadas** do estado. Isso é muito mais seguro do que tentar compartilhar memória entre contextos heterogêneos.

### A Diferença em relação ao template vazio do Tauri

No template padrão, o `main.ts` normalmente:

- importa o CSS;
- busca um input de saudação;
- chama um comando simples como `greet`;
- escreve uma resposta em um elemento da página.

No seu código, isso mudou para um fluxo de aplicação real:

- as referências do DOM agora apontam para `todo-form`, `task-input` e `task-list`;
- no carregamento inicial, a UI pede o estado atual ao core;
- no submit, envia uma nova tarefa;
- ao clicar em excluir, pede a remoção pelo índice;
- a tela sempre é re-renderizada a partir do retorno do backend.

Isto é: o arquivo saiu de uma demo de “ping” e virou uma casca de cliente para um pequeno sistema de gerenciamento de estado.

### O Arquivo Completo

Arquivo: `/home/runner/work/aula-tauri/aula-tauri/todo-tauri/src/main.ts`

```ts
import { invoke } from "@tauri-apps/api/core";

const form = document.getElementById("todo-form") as HTMLFormElement;
const input = document.getElementById("task-input") as HTMLInputElement;
const list = document.getElementById("task-list") as HTMLUListElement;

// Raciocínio: Ao carregar a página (ou dar F5), busca as tarefas que estão na RAM do Rust
window.addEventListener("DOMContentLoaded", async () => {
  try {
    const tarefas = await invoke<string[]>("carregar_tarefas");
    renderizarLista(tarefas);
  } catch (err) {
    console.error("Erro ao carregar o estado nativo:", err);
  }
});

form.addEventListener("submit", async (e) => {
  e.preventDefault();
  
  const taskText = input.value.trim();
  if (!taskText) return;

  try {
    // Envia a nova tarefa e recebe a lista inteira atualizada de volta do Core
    const tarefasAtualizadas = await invoke<string[]>("adicionar_tarefa", { texto: taskText });
    renderizarLista(tarefasAtualizadas);
    input.value = "";
  } catch (erro) {
    console.error("Erro na comunicação IPC:", erro);
  }
});

function renderizarLista(tarefas: string[]) {
  list.innerHTML = ""; // Limpa o DOM antes de redesenhar
  tarefas.forEach((t, indice) => {
    const li = document.createElement("li");

    const texto = document.createElement("span");
    texto.textContent = t;

    const botaoExcluir = document.createElement("button");
    botaoExcluir.type = "button";
    botaoExcluir.textContent = "X";
    botaoExcluir.className = "delete-task";
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

### Leitura crítica para aula

Esse arquivo já antecipa conceitos que serão aprofundados no Módulo 2 e no Módulo 3:

- o frontend não confia em si mesmo como fonte da verdade;
- a UI pede ao backend a lista “oficial”;
- cada ação do usuário vira uma mensagem;
- a atualização da tela ocorre depois da resposta do core.

Também vale destacar dois detalhes de engenharia para leitura crítica:

- `list.innerHTML = ""` prioriza clareza didática sobre micro-otimização; para uma lista pequena e um tutorial introdutório, isso deixa o redesenho fácil de explicar, embora APIs como `replaceChildren()` sejam alternativas mais refinadas;
- `botaoExcluir.className = "delete-task"` funciona como marcador semântico, mesmo sem uma regra CSS correspondente no estado atual do projeto.

Do ponto de vista conceitual, isso lembra bastante um cliente falando com um servidor — com a diferença de que, aqui, o “servidor” está no mesmo aplicativo, isolado por uma ponte IPC interna.

---

## Fechamento do Módulo 1

O grande ensinamento deste módulo é que o seu frontend foi construído como uma camada **simples, imperativa e desacoplada do estado nativo**:

- o HTML define a anatomia da tela;
- o CSS original do template revela a origem do scaffold e o que sobrou dele;
- o TypeScript liga eventos da WebView ao backend sem acessar memória nativa diretamente.

Para uma aula de fundamentos, isso é excelente porque permite explicar:

- separação entre interface e núcleo;
- diferença entre DOM em memória da WebView e estado em RAM do processo Rust;
- por que a comunicação entre camadas deve ocorrer por mensagens e contratos, não por compartilhamento bruto de memória;
- como um app desktop moderno ainda herda princípios clássicos de isolamento de processos.

No próximo módulo, o foco natural é abrir o `src-tauri/src/main.rs` e mostrar como o backend nativo recebe essas mensagens, serializa respostas e protege o estado em RAM com `Mutex`.
