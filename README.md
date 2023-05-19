# Page Rank

[Plan działania](./roadmap.md)

## Treść zadania

Uproszczony algorytm Page Rank:

- Implementacja iteracyjnej metody obliczania wartości własnych macierzy.
- Wykorzystanie do uproszczonej implementacji algorytmu Page Rank.
- Dyskusja zachowania przy różnych skalach grafów relacji.

## Zbiory danych

| Nazwa | Liczba plików | Liczba powiązań | Źródło |
| ---   | ---           | ---             | --- |
| C++ Reference | 5835 | 758340 | [Źródło](https://github.com/PeterFeicht/cppreference-doc/releases/download/v20220730/html-book-20220730.zip) |

### Generowanie zbiorów danych

Wymagania:

- Rust
- Program `unzip`

Uruchom generator odpowiadający danemu zbiorowi danych. Przykładowo:

```sh
$ ./generate-cpp-reference-dataset.sh
```

Stworzy on plik odpowiadający danemu zbiorowi (w tym przypadku `cppreference.json`).
Posiada on następującą strukturę:

```typescript
{
	// Wersja pliku, nowszy generator może stworzyć plik o nowszej wersji
	version: string;

	// Kolejne strony oraz linki na nie się składające
	pages: {
        [page: string]: {
            title: string;
            references: []string;
        };
    };
}
```
