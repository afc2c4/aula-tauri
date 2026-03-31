#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::State;

// 1. Definimos a estrutura do nosso Estado Global
struct AppState {
    // O Mutex garante que apenas uma thread escreva/leia por vez na RAM
    tarefas: Mutex<Vec<String>>,
}

// 2. Comando para adicionar tarefa no estado global
#[tauri::command]
fn adicionar_tarefa(texto: String, state: State<'_, AppState>) -> Result<Vec<String>, String> {
    // Tentamos adquirir o lock (a chave) do Mutex
    // Se outra thread estiver usando, esta thread dorme até a chave ser liberada
    let mut lista = state.tarefas.lock().map_err(|_| "Falha ao adquirir o lock do Mutex")?;
    
    lista.push(texto);
    
    // Retornamos um clone da lista atualizada para o Frontend via IPC
    Ok(lista.clone())
}

// 3. Comando para carregar a lista inicial quando a UI abre
#[tauri::command]
fn carregar_tarefas(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let lista = state.tarefas.lock().map_err(|_| "Falha ao adquirir o lock do Mutex")?;
    Ok(lista.clone())
}

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