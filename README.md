Integrasjon av Rust og Python for håndtering av DataFrames


Dette prosjektet er en kombinasjon av Rust og Python, hvor vi bruker PyO3 og Polars for å lage en funksjon som summerer alle tallene i en DataFrame. Prosjektet inkluderer tester både i Rust og Python, og alt er automatisert gjennom GitHub Actions.
________________________________________
Hovedfunksjonalitet
•	Rust-funksjonen sum_numeric_columns:
o	Tar inn en Polars DataFrame.
o	Summerer tallene i alle kolonner som inneholder numeriske verdier.
o	Ignorerer None-verdier (mangler data).
o	Eksponeres som en Python-modul ved hjelp av PyO3.
•	Python-testing:
o	Tester funksjonen med gyldige tall, tomme data, og ugyldige data.
o	Automatisert med pytest.
•	GitHub Actions:
o	Bygger Rust-prosjektet.
o	Kjører Rust-tester med cargo test.
o	Bygger og installerer Rust-modulen i et Python-miljø.
o	Kjører Python-tester med pytest.
________________________________________
Prosjektstruktur
GitAction/
├── .github/
│   └── workflows/
│       └── ci.yml               # GitHub Actions arbeidsflyt
├── src/
│   └── lib.rs                   # Rust-koden for funksjonen
├── Cargo.toml                   # Rust-prosjektets avhengigheter og metadata
├── test_rust_dataframe_utils.py # Python-testskript
├── .gitignore                   # Filer og mapper som ikke legges til i Git
________________________________________
Hvordan bruke prosjektet
1. Bygg og installer Rust-modulen lokalt
1.	Aktiver et virtuelt miljø: 
2.	python -m venv .venv
3.	source .venv/bin/activate  # På Linux/Mac
4.	.venv\Scripts\activate     # På Windows
5.	Installer maturin: 
6.	pip install maturin
7.	Bygg og installer modulen: 
8.	python -m maturin develop
2. Kjør Python-testene
1.	Installer pytest: 
2.	pip install pytest
3.	Kjør testene: 
4.	pytest test_rust_dataframe_utils.py
________________________________________
Automatisert testing med GitHub Actions
•	Hver gang du committer eller åpner en pull request, kjører GitHub Actions: 
1.	Bygging og testing av Rust-koden.
2.	Bygging av Rust-modulen og kjøring av Python-tester.
________________________________________
Eksempel på bruk
Her er et Python-eksempel som viser hvordan du bruker modulen:
import rust_dataframe_utils

# Data som skal summeres
data = [
    [1.0, 2.0, 3.0],  # Kolonne 1
    [4.0, 5.0, 0.0]   # Kolonne 2
]

# Kjør funksjonen
result = rust_dataframe_utils.sum_numeric_columns(data)
print(f"Summen av alle tallene i DataFrame: {result}")  # Output: 15.0
________________________________________
Avhengigheter
Rust:
•	polars = "0.45.1"
•	pyo3 = { version = "0.21", features = ["extension-module"] }
•	pyo3-arrow = "0.3"
Python:
•	pytest
•	maturin
________________________________________
Forbedringsmuligheter
1.	Legge til flere tester for mer komplekse data.
2.	Optimalisere funksjonen for store DataFrames.
3.	Legge til mer detaljert logging.
________________________________________
Lisens
Legg til en passende lisens her (f.eks., MIT, Apache 2.0).
________________________________________

