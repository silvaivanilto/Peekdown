use serde::Serialize;

#[derive(Serialize)]
pub struct Translations {
    pub new_tooltip: &'static str,
    pub open_tooltip: &'static str,
    pub save_tooltip: &'static str,
    pub toggle_tooltip: &'static str,
    pub split_tooltip: &'static str,
    pub outline_tooltip: &'static str,
    pub toggle_theme_tooltip: &'static str,
    pub minimize_tooltip: &'static str,
    pub maximize_tooltip: &'static str,
    pub close_tooltip: &'static str,
    pub find_placeholder: &'static str,
    pub find_prev_tooltip: &'static str,
    pub find_next_tooltip: &'static str,
    pub find_close_tooltip: &'static str,
    pub editor_placeholder: &'static str,
    pub drop_message: &'static str,
    pub untitled: &'static str,
    pub mode_edit: &'static str,
    pub mode_preview: &'static str,
    pub mode_split: &'static str,
    pub saved_status: &'static str,
    pub error_prefix: &'static str,
    pub word_singular: &'static str,
    pub word_plural: &'static str,
    pub recent_files: &'static str,
    pub no_headings: &'static str,
    pub no_results: &'static str,
    pub of_text: &'static str,
    pub confirm_close_all: &'static str,
    pub confirm_close_tab: &'static str,
    pub window_title_prefix: &'static str,
    pub stdin_label: &'static str,
    pub failed_open_file: &'static str,
    pub failed_save: &'static str,
}

const EN: Translations = Translations {
    new_tooltip: "New (Ctrl+N)",
    open_tooltip: "Open (Ctrl+O)",
    save_tooltip: "Save (Ctrl+S)",
    toggle_tooltip: "Toggle Preview (Ctrl+E)",
    split_tooltip: "Split View (Ctrl+\\)",
    outline_tooltip: "Outline (Ctrl+Shift+O)",
    toggle_theme_tooltip: "Toggle Theme",
    minimize_tooltip: "Minimize",
    maximize_tooltip: "Maximize",
    close_tooltip: "Close",
    find_placeholder: "Find...",
    find_prev_tooltip: "Previous (Shift+Enter)",
    find_next_tooltip: "Next (Enter)",
    find_close_tooltip: "Close (Escape)",
    editor_placeholder: "Start writing markdown...",
    drop_message: "Drop to open",
    untitled: "Untitled",
    mode_edit: "EDIT",
    mode_preview: "PREVIEW",
    mode_split: "SPLIT",
    saved_status: "Saved",
    error_prefix: "Error: ",
    word_singular: " word",
    word_plural: " words",
    recent_files: "Recent Files",
    no_headings: "No headings",
    no_results: "No results",
    of_text: " of ",
    confirm_close_all: "You have unsaved changes. Close anyway?",
    confirm_close_tab: "Unsaved changes in \"{filename}\". Close anyway?",
    window_title_prefix: "Peekdown - ",
    stdin_label: "stdin",
    failed_open_file: "Failed to open file: ",
    failed_save: "Failed to save: ",
};

const PT_BR: Translations = Translations {
    new_tooltip: "Novo (Ctrl+N)",
    open_tooltip: "Abrir (Ctrl+O)",
    save_tooltip: "Salvar (Ctrl+S)",
    toggle_tooltip: "Alternar Visualização (Ctrl+E)",
    split_tooltip: "Visão Dividida (Ctrl+\\)",
    outline_tooltip: "Sumário (Ctrl+Shift+O)",
    toggle_theme_tooltip: "Alternar Tema",
    minimize_tooltip: "Minimizar",
    maximize_tooltip: "Maximizar",
    close_tooltip: "Fechar",
    find_placeholder: "Buscar...",
    find_prev_tooltip: "Anterior (Shift+Enter)",
    find_next_tooltip: "Próximo (Enter)",
    find_close_tooltip: "Fechar (Escape)",
    editor_placeholder: "Comece a escrever markdown...",
    drop_message: "Arraste para abrir",
    untitled: "Sem título",
    mode_edit: "EDIÇÃO",
    mode_preview: "VISUALIZAÇÃO",
    mode_split: "DIVIDIDO",
    saved_status: "Salvo",
    error_prefix: "Erro: ",
    word_singular: " palavra",
    word_plural: " palavras",
    recent_files: "Arquivos Recentes",
    no_headings: "Nenhum cabeçalho",
    no_results: "Nenhum resultado",
    of_text: " de ",
    confirm_close_all: "Você tem alterações não salvas. Fechar mesmo assim?",
    confirm_close_tab: "Alterações não salvas em \"{filename}\". Fechar mesmo assim?",
    window_title_prefix: "Peekdown - ",
    stdin_label: "entrada padrão",
    failed_open_file: "Falha ao abrir arquivo: ",
    failed_save: "Falha ao salvar: ",
};

#[cfg(windows)]
extern "system" {
    fn GetUserDefaultUILanguage() -> u16;
}

pub fn detect_locale() -> &'static str {
    #[cfg(windows)]
    {
        let lang_id = unsafe { GetUserDefaultUILanguage() };
        if lang_id == 0x0416 {
            return "pt-BR";
        }
    }
    if let Ok(lang) = std::env::var("LANG") {
        if lang.starts_with("pt_BR") || lang.starts_with("pt") {
            return "pt-BR";
        }
    }
    "en"
}

pub fn get_translations(locale: &str) -> &'static Translations {
    match locale {
        "pt-BR" => &PT_BR,
        _ => &EN,
    }
}
