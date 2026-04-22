import { invoke } from "@tauri-apps/api/core";

const form = document.getElementById("todo-form") as HTMLFormElement;
const input = document.getElementById("task-input") as HTMLInputElement;
const list = document.getElementById("task-list") as HTMLUListElement;

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
    const tarefasAtualizadas = await invoke<string[]>("adicionar_tarefa", { texto: taskText });
    renderizarLista(tarefasAtualizadas);
    input.value = "";
  } catch (erro) {
    console.error("Erro na comunicação IPC:", erro);
  }
});

function renderizarLista(tarefas: string[]) {
  list.innerHTML = ""; 
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
