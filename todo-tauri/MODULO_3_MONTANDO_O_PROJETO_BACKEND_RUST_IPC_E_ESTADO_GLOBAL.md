# Módulo 3: Passo 3 — Blindando o Backend Rust com IPC e Estado Global

Arquivo analisado: `todo-tauri/src-tauri/src/main.rs`

## O que você monta neste passo

Com a base do projeto pronta e a estrutura da interface já definida, agora entra a parte que dá autoridade real ao app: o backend Rust.

Neste passo, você monta a camada nativa que:

- registra os comandos que a UI poderá chamar;
- guarda o estado autoritativo da aplicação;
- sincroniza o acesso concorrente à lista de tarefas;
- e transforma o app em algo além de “uma tela com formulário”.

Em termos de passo a passo:

> o Módulo 2 desenhou onde a interface existe;  
> este módulo define onde o estado verdadeiro mora.

## A arquitetura deste passo

O desenho do seu backend segue uma ideia muito sólida para ensino: o frontend não manipula estado crítico; ele só envia comandos. O estado verdadeiro vive no lado Rust, na memória do processo nativo, e é protegido por tipos e sincronização.

Em termos de fundamentos:

- o **frontend** roda como camada de interface e interação;
- o **backend Rust** é a camada que guarda e controla o estado;
- a comunicação entre os dois lados acontece por **IPC mediada pelo Tauri**;
- o estado global é centralizado com `app.manage()` e recuperado por `State<'_, AppState>`;
- o acesso concorrente ao vetor de tarefas é serializado por `Mutex<Vec<String>>`.

Essa arquitetura é boa para iniciantes porque mostra, sem banco de dados e sem frameworks pesados, quatro ideias centrais de Sistemas Operacionais e engenharia de software:

1. **isolamento entre camadas**  
   o JavaScript não toca diretamente a memória do Rust;

2. **compartilhamento controlado de estado**  
   o dado vive em um ponto central, e não espalhado pela UI;

3. **sincronização explícita**  
   várias threads podem tentar acessar o mesmo recurso, então alguém precisa arbitrar a entrada;

4. **superfície mínima de ataque**  
   o frontend só consegue executar o que foi explicitamente exposto como comando.

---

## O que muda em relação ao template vazio do Tauri

Saindo do template para o seu código final, a mudança é profunda:

- o template padrão costuma expor apenas um comando de demonstração, como `greet`;
- o seu código expõe uma **API nativa mínima**, mas real:
  - `adicionar_tarefa`
  - `carregar_tarefas`
  - `remover_tarefa`
- o template normalmente não possui **estado global compartilhado protegido por mutex**;
- o seu backend já introduz **RAM como fonte de verdade**, sem banco, sem arquivo, sem SQLite;
- o template costuma só provar que “JS chama Rust”;
- o seu projeto prova algo mais importante: **JS pede, Rust decide, Rust sincroniza e Rust devolve um snapshot consistente**.

Há ainda um detalhe interessante do repositório: o arquivo `todo-tauri/src-tauri/src/lib.rs` ainda preserva a casca do template com o comando `greet`, enquanto o `main.rs` concentra a implementação real da aplicação de tarefas. Isso é didaticamente útil porque mostra a transição do esqueleto inicial para o backend final.

---

## Leitura sequencial do arquivo: trecho do código + explicação

### Trecho 1

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

### Explicação

Essa linha não mexe na lógica de tarefas, mas interfere no comportamento do processo quando a aplicação roda no Windows.

- `cfg_attr` aplica um atributo condicionalmente;
- `not(debug_assertions)` significa “quando não estamos em build de debug”, ou seja, normalmente em release;
- `windows_subsystem = "windows"` evita abrir um console junto com a aplicação gráfica.

Em aula, isso é um bom ponto para reforçar que um programa desktop não é só algoritmo: ele também conversa com convenções do sistema operacional e com o modelo de subsistema do executável.

---

### Trecho 2

```rust
use std::sync::Mutex;
use tauri::State;
```

### Explicação

Aqui já aparece o coração do módulo.

#### `Mutex`

`Mutex` vem de **mutual exclusion**. Ele é um mecanismo de sincronização para proteger um recurso compartilhado.

No seu projeto, o recurso compartilhado é a lista de tarefas na RAM. Sem esse mecanismo, duas execuções concorrentes poderiam:

- ler estado ao mesmo tempo;
- escrever ao mesmo tempo;
- intercalar operações de forma imprevisível;
- produzir inconsistência lógica;
- ou cair em condições clássicas de concorrência.

Em termos de SO, pense no `Mutex` como uma **chave do banheiro**: só uma thread entra por vez na seção crítica.

#### `State`

`State` é a forma tipada com que o Tauri entrega ao comando Rust uma referência ao estado gerenciado pela aplicação. Em vez de usar variável global solta ou ponteiros crus, o framework injeta esse estado de forma controlada.

Isso é importante porque:

- reduz acoplamento;
- evita truques inseguros;
- deixa explícito, na assinatura da função, que ela depende de estado compartilhado.

---

### Trecho 3

```rust
// 1. Definimos a estrutura do nosso Estado Global
struct AppState {
    // O Mutex garante que apenas uma thread escreva/leia por vez na RAM
    tarefas: Mutex<Vec<String>>,
}
```

### Explicação

Este é o núcleo do backend.

#### Por que encapsular em `AppState`

Em vez de registrar um `Vec<String>` solto, o seu código cria uma estrutura dedicada:

- isso agrupa responsabilidade;
- isso deixa espaço para crescer depois;
- isso torna o estado nomeado e semântico.

Hoje o estado tem só `tarefas`, mas amanhã poderia ter:

- contador de operações,
- preferências de UI,
- estatísticas,
- sessão do usuário,
- cache,
- conexões.

#### Por que `Mutex<Vec<String>>`

O vetor guarda as tarefas em memória. O `Mutex` envolve o vetor porque o vetor, sozinho, **não é seguro para mutação concorrente compartilhada**.

Sem `Mutex`, duas chamadas quase simultâneas poderiam tentar:

- fazer `push` ao mesmo tempo;
- remover e ler ao mesmo tempo;
- clonar enquanto outra thread altera a capacidade interna do vetor.

O Rust não deixa esse tipo de acesso compartilhado mutável passar impune. O compilador obriga o programador a escolher uma estratégia de sincronização quando há compartilhamento entre threads.

#### Onde está a RAM aqui

`Vec<String>` mora na heap do processo Rust. Isso significa:

- os elementos são alocados dinamicamente;
- a lista inteira vive enquanto o processo existir;
- ao fechar a aplicação, esse estado desaparece;
- não há persistência em disco.

Logo, sua aplicação tem **estado volátil**: consistente em execução, mas efêmero.

---

### Trecho 4

```rust
// 2. Comando para adicionar tarefa no estado global
#[tauri::command]
fn adicionar_tarefa(texto: String, state: State<'_, AppState>) -> Result<Vec<String>, String> {
```

### Explicação

Aqui começa a ponte IPC propriamente dita.

#### `#[tauri::command]`

Essa macro marca a função como um comando que pode ser chamado a partir do frontend via `invoke`.

Ela funciona como uma autorização explícita: o frontend **não chama qualquer função Rust arbitrária**. Ele só chama o que o backend decidiu publicar.

Isso é uma decisão forte de segurança:

- reduz a superfície exposta;
- evita acoplamento implícito;
- força o backend a declarar sua API pública.

#### Assinatura da função

`texto: String`  
é o dado enviado pelo frontend. O texto chega serializado do lado JavaScript, cruza a fronteira IPC, e o Tauri o desserializa para o tipo Rust esperado.

`state: State<'_, AppState>`  
é a injeção do estado global gerenciado pelo Tauri. O lifetime `'_` indica que essa referência emprestada é válida dentro do escopo da execução do comando.

`Result<Vec<String>, String>`  
é excelente do ponto de vista didático:

- no sucesso, devolve a lista atualizada;
- no erro, devolve uma mensagem serializável.

Isso mostra que IPC não é só chamada remota; é também **contrato de dados** entre dois mundos com modelos de execução diferentes.

---

### Trecho 5

```rust
    // Tentamos adquirir o lock (a chave) do Mutex
    // Se outra thread estiver usando, esta thread dorme até a chave ser liberada
    let mut lista = state.tarefas.lock().map_err(|_| "Falha ao adquirir o lock do Mutex")?;
```

### Explicação

Este é o ponto mais importante de concorrência do arquivo.

#### O que `lock()` faz

Ao chamar `lock()`, o código pede acesso exclusivo à seção crítica.

Se ninguém estiver usando o mutex:

- a thread entra imediatamente;
- recebe um guard;
- e pode manipular a lista.

Se outra thread já estiver com o lock:

- a thread atual espera;
- ela não ganha acesso simultâneo;
- o recurso continua protegido.

#### O que é `lista`

`lista` não é um `Vec<String>` independente. Ela é um guard que dá acesso mutável ao vetor protegido. Enquanto esse guard existir:

- o lock permanece segurado;
- outras threads não entram nessa região crítica.

#### Por que isso evita data races

Uma **data race** acontece quando múltiplas threads acessam a mesma memória ao mesmo tempo e pelo menos uma escreve, sem sincronização adequada.

Aqui isso não acontece porque:

- o vetor compartilhado está atrás de um `Mutex`;
- o acesso mutável depende de adquirir o lock;
- o Rust força esse protocolo pelo tipo.

Não é só convenção. É parte do modelo de segurança de memória da linguagem.

#### O `map_err(...)` e o `?`

O lock pode falhar se o mutex estiver envenenado (`poisoned`), por exemplo se alguma thread entrou em pânico enquanto segurava o lock. O seu código converte esse erro interno em uma string amigável e a propaga com `?`.

Isso é pedagogicamente bom porque ensina que:

- sincronização também falha;
- falha de sincronização precisa ser tratada;
- IPC precisa serializar erro de forma entendível.

---

### Trecho 6

```rust
    lista.push(texto);
    
    // Retornamos um clone da lista atualizada para o Frontend via IPC
    Ok(lista.clone())
}
```

### Explicação

Depois de adquirir o lock, a mutação é simples: `push(texto)`.

Mas o detalhe mais importante vem no retorno.

#### Por que retornar `clone()`

O frontend não recebe uma referência para a memória interna do Rust. Isso seria inviável e inseguro entre fronteiras de processo/camada.

Em vez disso, o backend:

1. atualiza o estado interno;
2. cria uma cópia serializável;
3. devolve essa cópia pela IPC.

Esse desenho reforça uma ideia central:

- **o backend retém a posse do estado real**;
- o frontend recebe apenas uma representação transportável.

Isso evita:

- vazamento de alias mutável;
- compartilhamento indevido de memória;
- dependência do frontend sobre layout interno da estrutura.

Quando a função termina, `lista` sai de escopo e o lock é liberado automaticamente. Esse padrão é um exemplo elegante de **RAII** em Rust: o recurso é adquirido e liberado conforme o tempo de vida do valor guardião.

---

### Trecho 7

```rust
// 3. Comando para carregar a lista inicial quando a UI abre
#[tauri::command]
fn carregar_tarefas(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let lista = state.tarefas.lock().map_err(|_| "Falha ao adquirir o lock do Mutex")?;
    Ok(lista.clone())
}
```

### Explicação

Este comando serve para sincronizar a interface com o estado do backend quando a UI inicia.

Arquiteturalmente, isso é muito importante: o HTML inicial não é a fonte da verdade. A fonte da verdade está no estado nativo, então a UI precisa pedir o snapshot atual.

#### O que ele faz

- recebe o `State`;
- adquire o lock;
- clona o vetor;
- envia a cópia para o frontend.

#### O que ele não faz

- não persiste em disco;
- não toca banco;
- não recalcula nada complexo;
- não dá ao frontend acesso direto ao vetor interno.

Esse comando também mostra bem o papel do `Mutex` em leitura. Mesmo para ler, o código entra pelo mesmo portão de sincronização, garantindo consistência do snapshot retornado.

---

### Trecho 8

```rust
// 4. Remove uma tarefa pelo indice para suportar textos duplicados
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

### Explicação

Esse terceiro comando adiciona dois temas muito bons para aula: **identidade lógica** e **validação defensiva**.

#### Remover por índice

O comentário já revela a intenção correta: remover por índice evita ambiguidade quando há textos duplicados.

Exemplo:

- `["estudar", "estudar", "descansar"]`

Se a remoção fosse por texto, qual `"estudar"` sairia? Ao usar `indice`, o contrato fica objetivo.

#### Validação de fronteira

Este trecho é essencial:

```rust
if indice >= lista.len() {
    return Err("Indice de tarefa invalido".to_string());
}
```

O frontend não é confiável por definição. Mesmo que a UI atual só envie índices válidos, a camada nativa precisa validar a entrada recebida pela IPC.

Esse é um princípio importante de segurança:

- toda entrada externa deve ser considerada potencialmente inválida;
- a checagem deve ocorrer no lado de autoridade;
- o backend não deve confiar cegamente no cliente.

#### Segurança de memória

Depois da validação, `lista.remove(indice)` é seguro. Sem a checagem, haveria erro por acesso fora do intervalo lógico do vetor. Aqui o código protege a integridade da operação antes da mutação.

#### Consistência do retorno

Assim como nos outros comandos, o backend retorna um clone da lista inteira. Isso simplifica o frontend, que não precisa recalcular diferenças nem manter estruturas paralelas.

---

### Trecho 9

```rust
fn main() {
    tauri::Builder::default()
        // 5. Injetamos o estado inicial na "mochila" do Tauri ao dar o boot
        .manage(AppState {
            tarefas: Mutex::new(Vec::new()),
        })
        // Registramos os novos comandos IPC
        .invoke_handler(tauri::generate_handler![adicionar_tarefa, carregar_tarefas, remover_tarefa])
        .run(tauri::generate_context!())
        .expect("Erro fatal: Falha ao iniciar o Tauri");
}
```

### Explicação

Aqui o backend é realmente montado.

#### `tauri::Builder::default()`

O builder monta a aplicação desktop, configurando a infraestrutura necessária para janela, runtime, contexto e registro de comandos.

#### `.manage(AppState { ... })`

Este é o ponto em que o estado global entra na aplicação.

Ao chamar `.manage(...)`, você entrega ao Tauri uma instância única de `AppState`, que fica disponível para os comandos via `State<'_, AppState>`.

O conteúdo inicial é:

```rust
tarefas: Mutex::new(Vec::new())
```

Ou seja:

- o vetor começa vazio;
- o estado nasce na RAM;
- o recurso já nasce protegido por mutex;
- a aplicação inteira passa a compartilhar esse mesmo ponto central de verdade.

#### `.invoke_handler(...)`

Esse trecho registra quais funções podem ser chamadas pelo frontend:

- `adicionar_tarefa`
- `carregar_tarefas`
- `remover_tarefa`

Esse registro é a lista branca da IPC. Se um comando não estiver registrado aqui, o frontend não o invoca por nome.

Isso conversa diretamente com o modelo de segurança:

- exposição explícita;
- superfície mínima;
- controle central do que atravessa a fronteira.

#### `.run(tauri::generate_context!())`

Aqui a aplicação entra em execução. A partir desse ponto:

- o processo sobe;
- o runtime do Tauri inicializa;
- os comandos passam a responder à UI;
- o estado gerenciado fica vivo enquanto o processo existir.

#### `.expect(...)`

Se a aplicação falhar ao subir, o processo aborta com uma mensagem clara. Para aula, isso é um bom lugar para mostrar a diferença entre:

- erro operacional recuperável dentro de um comando;
- falha fatal de inicialização da aplicação.

---

## Fechamento conceitual do Módulo 3

O `main.rs` mostra uma arquitetura pequena, mas extremamente rica para ensinar fundamentos:

- **IPC**: o frontend conversa com o backend por comandos nomeados;
- **estado em RAM**: não há SQLite, não há arquivo, não há persistência;
- **estado global controlado**: `manage()` registra uma instância central;
- **sincronização**: `Mutex` serializa o acesso ao vetor;
- **segurança de memória**: o frontend nunca recebe referência direta à estrutura interna;
- **segurança de entrada**: o backend valida índice antes de remover;
- **anti-data-race por construção**: o modelo de tipos do Rust obriga disciplina no acesso compartilhado.

Em linguagem de sala de aula:

> o frontend pede;  
> o Tauri transporta;  
> o Rust valida;  
> o Mutex arbitra;  
> a RAM guarda;  
> e o backend devolve um snapshot seguro.

Esse é o passo exato em que a aplicação deixa de ser “uma página com botão” e passa a ser um sistema com fronteira, autoridade, sincronização e política de acesso ao estado.
