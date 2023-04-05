Termin: 28.04

## Minimum

Uproszczony algorytm Page Rank z następującym planem działania:

- R (do 7.04 włącznie): Przygotowanie zbioru danych
	- Ekstrakcja linków i powiązanie ich wzajemnie
- D: (do 11.04)Implementacja iteracyjnej metody obliczania wartości własnych macierzy. Wykorzystanie do uproszczonej implementacji algorytmu Page Rank.
	- Implementacja silnie inspirowana: https://allendowney.github.io/DSIRP/pagerank.html
- G: Dyskusja zachowania przy różnych skalach grafów relacji (zależność zbieżności algorytmu od wielkości grafu - liczby linków)
- DGR: Zapoznanie się z papierem co narobił nam problemów:
	- https://raw.githubusercontent.com/emintham/Papers/master/Google/Page%2CBrin%2CMotwani%2CWinograd-%20The%20PageRank%20Citation%20Ranking%3A%20Bringing%20Order%20to%20the%20Web.pdf


## Rozbudowa

- D (do 07.04): API skryptu pythonowego do wykorzystania przez przeglądarkę
- D: (do 16.04)interfejs do wyszukiwania w przeglądarce
- R (do 16.04 włącznie): TF-IDF zintegrowany z PageRankiem (pełen system wyszukiwania)


## Endpointy zaszkicowane
- POST `/api/search?q="xd"` <- Pełen system wyszukiwania
	- zwróć powiązane dokumenty z daną frazą wyszukiwania uwzględniając TF-IDF oraz PageRank

- POST `/api/relevant?link="foo/bar"` <- PageRank
	- zwróć powiązane dokumenty wyłącznie na podstawie algorytmu PageRank

- GET `/api/links`
	- zwróć wszystkie linki jakie mamy w systemie



