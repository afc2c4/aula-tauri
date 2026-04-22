use std::sync::Mutex;
use tauri::State;

struct AppState {
    tarefas: Mutex<Vec<String>>,
}

#[tauri::command]
fn adicionar_tarefa(texto: String, state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let mut lista = state.tarefas.lock().map_err(|_| "Falha ao adquirir o lock do Mutex")?;
    
    lista.push(texto);
    
    Ok(lista.clone())
}

#[tauri::command]
fn carregar_tarefas(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let lista = state.tarefas.lock().map_err(|_| "Falha ao adquirir o lock do Mutex")?;
    Ok(lista.clone())
}

#[tauri::command]
fn remover_tarefa(indice: usize, state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let mut lista = state.tarefas.lock().map_err(|_| "Falha ao adquirir o lock do Mutex")?;

    if indice >= lista.len() {
        return Err("Indice de tarefa invalido".to_string());
    }

    lista.remove(indice);
    Ok(lista.clone())

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().
        manage(AppState {
            tarefas: Mutex::new(Vec::new()),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![adicionar_tarefa, carregar_tarefas, remover_tarefa])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
