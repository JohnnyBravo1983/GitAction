Integrasjon av Rust og Python for håndtering av DataFrames
Dette prosjektet kombinerer Rust og Python for å håndtere numeriske data i DataFrames, ved hjelp av PyO3 og Polars. Funksjonaliteten eksponeres som et Python-modul, og testing er automatisert via GitHub Actions.

Hovedfunksjonalitet
Rust-funksjonen sum_numeric_columns
Tar inn en Polars DataFrame

Summerer verdiene i alle kolonner som inneholder numeriske data

Ignorerer None-verdier (manglende data)

Eksponeres som en Python-modul via PyO3

Python-testing
Bruker pytest til å teste:

Gyldige tall

Tomme data

Ugyldige input

Automatisk testkjøring via CI

GitHub Actions
Bygger Rust-prosjektet

Kjører Rust-tester med cargo test

Bygger og installerer Python-modulen med maturin

Kjører Python-tester med pytest

 Prosjektstruktur
bash
Kopier
Rediger
GitAction/
├── .github/
│   └── workflows/
│       └── ci.yml             # GitHub Actions workflow
├── src/
│   └── lib.rs                 # Rust-funksjon
├── Cargo.toml                 # Rust metadata og avhengigheter
├── test_rust_dataframe_utils.py  # Python-tests
├── .gitignore
▶ Hvordan bruke prosjektet
1. Bygg og installer Rust-modulen lokalt
bash
Kopier
Rediger
python -m venv .venv
source .venv/bin/activate   # På Linux/Mac
.venv\Scripts\activate      # På Windows

pip install maturin
python -m maturin develop
2. Kjør Python-tester
bash
Kopier
Rediger
pip install pytest
pytest test_rust_dataframe_utils.py
Automatisert testing med GitHub Actions
Ved hver commit eller pull request:

Rust bygges og testes

Python-modulen bygges og testes med pytest

🧪 Eksempel på bruk
python
Kopier
Rediger
import rust_dataframe_utils

data = [
    [1.0, 2.0, 3.0],  # Kolonne 1
    [4.0, 5.0, 0.0]   # Kolonne 2
]

result = rust_dataframe_utils.sum_numeric_columns(data)
print(f"Summen av alle tallene i DataFrame: {result}")  # Output: 15.0
Dependencies/ Avhengigheter
Rust
polars = "0.45.1"

pyo3 = { version = "0.21", features = ["extension-module"] }

pyo3-arrow = "0.3"

Python
pytest

maturin

 Forbedringsmuligheter
Flere tester for komplekse datasett

Ytelsesoptimalisering for store DataFrames

Bedre logging og feilhåndtering

Lisens
Velg og legg til en passende lisens (f.eks. MIT, Apache 2.0)

Kort oppsummert
Dette prosjektet demonstrerer:

Språkintegrasjon mellom Rust og Python

Praktisk bruk av CI/CD med GitHub Actions

Testing og modulbygging i et flerplattform-miljø
