# Módulo 4: Passo 4 — Ligando o Frontend TypeScript e Finalizando o App

Arquivo analisado: `todo-tauri/src/main.ts`

## O que você monta neste passo

Este é o passo em que a aplicação finalmente fecha o circuito completo.

Depois de:

- criar a base do projeto no Módulo 1;
- montar a estrutura HTML no Módulo 2;
- blindar o backend Rust no Módulo 3;

o `main.ts` entra para ligar tudo:

- captura os eventos do formulário;
- chama os comandos IPC;
- recebe snapshots atualizados;
- e redesenha a lista na interface.

## A ideia central deste módulo

Se o Módulo 2 montou o palco do DOM e o Módulo 3 blindou o estado no Rust, este módulo mostra **quem conecta as duas pontas e faz o app acontecer**.

No seu projeto:

- o `main.ts` é a ponte entre a interface HTML e o backend Rust;
- ele captura eventos do usuário;
- transforma esses eventos em chamadas IPC;
- recebe de volta snapshots do estado atualizado;
- redesenha a interface a partir desses dados.

Ou seja, o frontend não é a fonte da verdade. Ele é o **orquestrador visual**:

- lê o DOM,
- escuta ações,
- chama o backend,
- recebe dados,
- renderiza novamente.

Esse desenho é importante porque separa responsabilidades:

- **UI**: mostrar e coletar interação;
- **backend**: validar, mutar e guardar estado;
- **IPC**: transportar pedidos e respostas entre as camadas.

---

## O que mudou em relação a um frontend “solto”

Num frontend web comum, seria muito fácil manter tudo só em JavaScript:

- um array local;
- um `push`;
- um `splice`;
- e uma re-renderização.

Mas aqui o objetivo didático é outro: mostrar que a interface pode ser apenas cliente de uma camada nativa.

Então, em vez de:

- guardar o array no JavaScript,

o seu código faz isso:

- envia comandos com `invoke(...)`;
- recebe o estado de volta do backend;
- redesenha com base no retorno.

Esse padrão ensina uma disciplina importante:

- o frontend não “acha” o estado;
- o frontend **consulta** o estado;
- o frontend não “decide” a mutação;
- o frontend **solicita** a mutação.

---

## Leitura sequencial do arquivo: trecho do código + explicação

### Trecho 1

```ts
import { invoke } from "@tauri-apps/api/core";
```

### Explicação

Essa linha importa a função mais importante deste módulo: `invoke`.

#### O que é `invoke`

`invoke` é a API do Tauri usada no frontend para chamar comandos expostos no backend Rust.

Ela é o ponto visível da IPC no lado TypeScript.

Em vez de o frontend acessar diretamente a memória do Rust, ele faz algo conceitualmente parecido com:

1. montar uma mensagem;
2. informar o nome do comando;
3. enviar argumentos serializáveis;
4. aguardar a resposta;
5. receber dados serializados de volta.

#### Por que isso é importante

Essa linha já deixa clara a arquitetura:

- o frontend depende de um canal oficial de comunicação;
- ele não tem acesso direto ao `Vec<String>` do Rust;
- ele só enxerga comandos autorizados.

Isso reforça isolamento entre camadas e reduz acoplamento.

---

### Trecho 2

```ts
const form = document.getElementById("todo-form") as HTMLFormElement;
const input = document.getElementById("task-input") as HTMLInputElement;
const list = document.getElementById("task-list") as HTMLUListElement;
```

### Explicação

Aqui o código captura os três elementos centrais da interface:

- o formulário;
- o campo de texto;
- a lista visual das tarefas.

#### O que está acontecendo

`document.getElementById(...)` busca elementos já existentes no HTML.

Depois disso, o código usa *type assertions*:

- `HTMLFormElement`
- `HTMLInputElement`
- `HTMLUListElement`

Isso ajuda o TypeScript a entender qual API DOM estará disponível em cada variável.

#### Papel arquitetural desses três nós

- `form`: captura a intenção de adicionar uma tarefa;
- `input`: guarda o texto digitado;
- `list`: recebe a renderização da lista de tarefas.

Esse trecho é o começo da camada de apresentação: antes de falar com o backend, o frontend precisa saber **onde escutar** e **onde desenhar**.

---

### Trecho 3

```ts
// Raciocínio: Ao carregar a página (ou dar F5), busca as tarefas que estão na RAM do Rust
window.addEventListener("DOMContentLoaded", async () => {
  try {
    const tarefas = await invoke<string[]>("carregar_tarefas");
    renderizarLista(tarefas);
  } catch (err) {
    console.error("Erro ao carregar o estado nativo:", err);
  }
});
```

### Explicação

Esse bloco é fundamental porque mostra a sincronização inicial entre interface e backend.

#### `DOMContentLoaded`

Esse evento dispara quando a estrutura principal do HTML já foi carregada. Isso significa que os elementos do DOM já existem e podem ser usados com segurança.

O código espera esse momento para iniciar a conversa com o backend.

#### `invoke<string[]>("carregar_tarefas")`

Aqui o frontend chama o comando Rust `carregar_tarefas`, que foi exposto no backend no módulo anterior.

O `<string[]>` indica o tipo esperado na resposta: um array de strings.

Didaticamente, isso é ótimo porque deixa explícito que:

- há uma requisição assíncrona;
- o frontend espera uma resposta;
- a resposta precisa ter forma compatível com a renderização.

#### Por que carregar no início

Mesmo que a lista esteja vazia no começo, o padrão correto é:

- abrir a tela;
- consultar o backend;
- renderizar o estado real.

Isso evita assumir que a UI já sabe o que existe.

#### Tratamento de erro

Se a IPC falhar, o código registra o erro no console.

Isso não resolve a falha para o usuário final, mas é suficiente para ensino porque mostra que:

- comunicação entre camadas pode falhar;
- a interface precisa lidar com isso;
- chamadas assíncronas não devem assumir sucesso automático.

---

### Trecho 4

```ts
form.addEventListener("submit", async (e) => {
  e.preventDefault();
  
  const taskText = input.value.trim();
  if (!taskText) return;
```

### Explicação

Esse trecho reage ao envio do formulário para criar uma nova tarefa.

#### `submit`

Em vez de escutar diretamente um clique em botão, o código escuta o evento semântico do formulário.

Isso é bom porque:

- funciona com botão;
- funciona com Enter no input;
- preserva a semântica natural do HTML.

#### `e.preventDefault()`

Sem essa linha, o navegador tentaria submeter o formulário do jeito tradicional e poderia recarregar a página.

Como a aplicação trabalha com JavaScript e IPC, esse comportamento padrão precisa ser bloqueado.

#### `input.value.trim()`

O valor digitado é lido e espaços nas pontas são removidos.

Isso evita aceitar como tarefa algo como:

- `"   "`
- `"    estudar   "` sem limpeza mínima

#### `if (!taskText) return;`

Essa é a primeira camada de validação, no frontend.

Ela é útil para UX porque evita chamada desnecessária ao backend quando a entrada está vazia.

Mas ela **não substitui** validação do lado nativo quando a regra de negócio exigir. Em arquitetura cliente-servidor, o frontend ajuda, mas não é autoridade final.

---

### Trecho 5

```ts
  try {
    // Envia a nova tarefa e recebe a lista inteira atualizada de volta do Core
    const tarefasAtualizadas = await invoke<string[]>("adicionar_tarefa", { texto: taskText });
    renderizarLista(tarefasAtualizadas);
    input.value = "";
  } catch (erro) {
    console.error("Erro na comunicação IPC:", erro);
  }
});
```

### Explicação

Aqui acontece a mutação pedida pelo usuário.

#### O comando chamado

`invoke<string[]>("adicionar_tarefa", { texto: taskText })`

O frontend:

- informa o nome do comando;
- envia um objeto com o argumento esperado;
- aguarda a resposta do backend;
- recebe a lista já atualizada.

O nome `texto` no objeto precisa bater com o parâmetro esperado no comando Rust. Isso é importante porque a IPC trabalha com serialização e mapeamento de campos.

#### Por que renderizar a resposta e não atualizar localmente

Em vez de fazer:

- `arrayLocal.push(...)`

o código faz:

- esperar a resposta oficial do backend;
- redesenhar com base nela.

Isso tem duas vantagens grandes:

1. o frontend permanece alinhado com a fonte da verdade;
2. qualquer regra do backend já entra refletida no retorno.

#### `input.value = ""`

Depois do sucesso, o campo é limpo. Isso fecha o ciclo da interação:

- usuário envia;
- backend responde;
- interface atualiza;
- campo volta ao estado pronto para nova entrada.

#### Erro de IPC

Se a chamada falhar, o erro é registrado. Isso é coerente com o restante do arquivo: todos os pontos de comunicação com o backend são tratados de forma explícita.

---

### Trecho 6

```ts
function renderizarLista(tarefas: string[]) {
  list.innerHTML = ""; // Limpa o DOM antes de redesenhar
  tarefas.forEach((t, indice) => {
    const li = document.createElement("li");
```

### Explicação

Agora entra a função que transforma dados em interface.

#### Papel da função

`renderizarLista` recebe um array de tarefas e reconstrói visualmente a lista no DOM.

Ela é importante porque concentra a regra de apresentação em um só lugar.

#### `list.innerHTML = ""`

Antes de desenhar a nova lista, o código apaga a anterior.

Isso implementa uma estratégia simples de renderização:

- limpar tudo;
- recriar tudo.

Para listas pequenas, isso é totalmente aceitável em contexto didático e deixa o fluxo fácil de entender.

#### `tarefas.forEach((t, indice) => {`

O código percorre cada tarefa junto com seu índice.

Esse índice é especialmente importante porque, no backend, a remoção é feita por posição e não por texto. Então o frontend precisa preservar essa referência ao montar o botão de exclusão.

#### `document.createElement("li")`

Cada tarefa vira um `<li>`, isto é, um item de lista real no DOM.

---

### Trecho 7

```ts
    const texto = document.createElement("span");
    texto.textContent = t;
```

### Explicação

Esse bloco cria o nó que exibe o conteúdo textual da tarefa.

#### Por que `textContent`

`textContent` é uma escolha correta porque insere texto como texto, não como HTML interpretável.

Isso ajuda a evitar problemas de injeção de markup acidental, por exemplo se alguém digitasse algo como:

- `<b>oi</b>`
- `<script>alert(1)</script>`

Com `textContent`, esse conteúdo aparece como texto literal na tela.

Esse detalhe é pequeno, mas importante do ponto de vista de segurança.

---

### Trecho 8

```ts
    const botaoExcluir = document.createElement("button");
    botaoExcluir.type = "button";
    botaoExcluir.textContent = "X";
    botaoExcluir.className = "delete-task";
```

### Explicação

Aqui o código cria o botão que remove tarefas.

#### `type = "button"`

Isso evita que o botão se comporte como botão de submissão de formulário por padrão.

Como ele vive dentro de uma interface com formulário, explicitar o tipo é uma decisão correta e defensiva.

#### `textContent = "X"`

Define o rótulo visual do botão.

#### `className = "delete-task"`

Conecta o elemento à camada de estilo.

Isso mostra a divisão clássica:

- TypeScript cria e conecta comportamento;
- CSS decide aparência.

---

### Trecho 9

```ts
    botaoExcluir.addEventListener("click", async () => {
      try {
        const tarefasAtualizadas = await invoke<string[]>("remover_tarefa", { indice });
        renderizarLista(tarefasAtualizadas);
      } catch (erro) {
        console.error("Erro ao remover tarefa:", erro);
      }
    });
```

### Explicação

Esse trecho fecha o ciclo de CRUD mínimo da aplicação.

#### O que acontece no clique

Quando o usuário clica no botão:

- o frontend chama `remover_tarefa`;
- envia o `indice` correspondente;
- aguarda a lista atualizada;
- renderiza de novo.

#### Relação com o backend

Esse `indice` é exatamente o contrato que o Rust espera.

Perceba como frontend e backend precisam concordar em três dimensões:

- nome do comando;
- nome do argumento;
- tipo do argumento.

Se houver divergência, a IPC quebra.

#### Re-renderização após remoção

Mais uma vez, a UI não tenta “adivinhar” o novo estado localmente. Ela prefere:

- pedir a operação;
- receber a verdade atualizada;
- redesenhar.

Esse padrão é consistente com toda a arquitetura do projeto.

---

### Trecho 10

```ts
    li.append(texto, botaoExcluir);
    list.appendChild(li);
  });
}
```

### Explicação

Este é o fechamento da renderização.

#### `li.append(texto, botaoExcluir)`

O item da lista recebe dois filhos:

- o texto da tarefa;
- o botão de exclusão.

#### `list.appendChild(li)`

Depois disso, o item entra no DOM visível.

O resultado final é uma interface reconstruída inteiramente a partir do array recebido do backend.

Esse detalhe é central:

- a renderização depende dos dados recebidos;
- os dados recebidos vêm do Rust;
- logo, a UI espelha o estado nativo, e não o contrário.

---

## Fechamento conceitual do Módulo 4

O `main.ts` ensina muito mais do que manipulação de DOM. Ele mostra, de forma pequena e direta, o fluxo completo de uma interface cliente de um core nativo:

- a interface captura eventos do usuário;
- cada evento relevante vira uma chamada `invoke(...)`;
- o backend responde com dados serializados;
- a UI redesenha a tela com base nesse retorno;
- o frontend não possui o estado autoritativo;
- o backend continua sendo a fonte da verdade.

Em resumo:

> o usuário interage;  
> o DOM dispara evento;  
> o TypeScript traduz isso em IPC;  
> o Rust processa;  
> o resultado volta;  
> e a interface se reconstrói em cima da resposta.

Esse é o ponto em que a aplicação deixa de ser apenas “JavaScript manipulando HTML” e passa a ser uma UI que opera como cliente de um sistema com fronteira nativa bem definida.
