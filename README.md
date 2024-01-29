# Loteria Gralniowa

Projekt ma na celu automatyzowanie losowania graczy na FNM organizowanych przez Gralnię.
Lista graczy pochodzi z listy HTML generowanej przez Eventlink i jest przekazywana przez serwer do aplikacji web hostowanej przez ten sam serwer.

# Użytkowanie

Konfiguracja serwera odbywa się przez plik `config.toml`.
W nim przekazujemy:
-   adres i port na którym nassłuchiwać będzie serwer
-   ścieżka z której zaciągane są dane do losowania
-   folder z hostowanymi plikami, domyślnie `host/`
-   strona startowa hostowana pod ścieżką `/`

W większości przypadków należy ustawić adres i port interfejsu sieciowego widocznego z sieci LAN z której będzie dostępowana aplikacja. Zalecane jest ustawienie na komputerze serwującym loterię statycznego adresu lub nazwy DNS, w celu ustawienia niezmieniającej się zakładki na urządzeniu z której będzie prowadzona loteria - smartofonie, tablecie, etc.

Następnie należy otworzyć aplikację na urządzeniu klienckim. Zakładając że w `config.toml` ustaliliśmy adres `192.168.0.1` i port `3000`, URL strony będzie równy `http://192.168.0.1:3000/`.

# Instalacja

TBD

# Architektura

Aplikacja składa się z dwóch części

## Backend Rust

### Config

Konfiguracja zaczytywana jest z pliku `config.toml`.
Jeśli konfiguracja nie powiedzie się z jakiegokolwiek powodu, program panikuje.

### HTTP Service

Serwis HTTP zaczyna nasłuchiwać po poprawnym skonfigurowaniu programu.
Serwis wspiera tylko metody GET, wszystkie ścieżki są względne skonfigurowanemu folderowi.

Dodatkowa ścieżka `/names` zwraca obecny stan listy graczy uczestniczących w losowaniu.

Query `lottery=new` powoduje przeczytanie pliku HTML zawierającego listę graczy, aktualizacji listy graczy w pamięci i przegenerowaniu pliku `<host>/names.json`. Ten plik jest zaczytywany przez frontend w celu importu listy graczy. Wszystko to dzieje się przed wysłaniem odpowiedzi do klienta.

### Name Provider

#### Provider

Trait Provider umożliwia łatwą wymianę parsera imion. Jedyne wymaganie to implementacja funkcji `fn get_names(&self) -> Result<Vec<String>, std::io::Error>`.

#### HTML Provider

W tej chwili jedyny napisany provider to parser HTML z Eventlink `src/names/html.rs`.
Przykładowy plik znajduje się w `src/test_names.htm`. To według niego został napisany parser.

Parser ma trywialne zadanie: znaleźć marker początkowy i zacząć dodawać linijki tekstu zanim nie znajdzie markera końcowego.

Jeśli z jakiegokolwiek powodu lista jest krótsza niż 2, plik się nie otworzy, etc. parsowanie nie powiedzie się i do klienta zostanie zwrócona wiadomość o błędzie.

## Frontend JS

TBD
