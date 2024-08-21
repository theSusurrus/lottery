# Loteria Gralniowa

Projekt ma na celu automatyzowanie losowania graczy na FNM organizowanych przez Gralnię.
Lista graczy pochodzi z listy HTML generowanej przez Eventlink.

# Użytkowanie

Konfiguracja aplikacji odbywa się przez plik `config.toml`.
W nim przekazujemy:
-   ścieżka z której zaciągane są dane do losowania

# Architektura

Aplikacja składa się z dwóch części

## Backend Rust

### Config

Konfiguracja zaczytywana jest z pliku `config.toml`.
Jeśli konfiguracja nie powiedzie się z jakiegokolwiek powodu, program panikuje.

### Name Provider

#### Provider

Trait Provider umożliwia łatwą wymianę parsera imion. Jedyne wymaganie to implementacja funkcji `fn get_names(&self) -> Result<Vec<String>, std::io::Error>`.

#### HTML Provider

W tej chwili jedyny napisany provider to parser HTML z Eventlink `src/names/html.rs`.
Przykładowy plik znajduje się w `src/test_names.htm`. To według niego został napisany parser.

Parser ma trywialne zadanie: znaleźć marker początkowy i zacząć dodawać linijki tekstu zanim nie znajdzie markera końcowego.

Jeśli z jakiegokolwiek powodu lista jest krótsza niż 2, plik się nie otworzy, etc. parsowanie nie powiedzie się i do klienta zostanie zwrócona wiadomość o błędzie.
