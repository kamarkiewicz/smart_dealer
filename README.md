smart_dealer.
=============

Projekt polegający na stworzeniu bazy danych i aplikacji użytkowej dla handlarza bronią.


TODO:
- [27 maja] model konceptualny (czyli diagram E-R)
- [27 maja] model fizyczny (skrypt SQL definiujący bazę danych)



Warunki zaliczenia projektu
===========================

Przypominam, że tematem projektu jest "Handlarz Bronią". Wg harmonogramu za 2 tygodnie należy oddać model konceptualny (czyli diagram E-R) i model fizyczny (skrypt SQL definiujący bazę danych, czyli typy, tabele, sekwencje, funkcje, wyzwalacze , perspektywy itp.). Najprawdopodobniej na tym etapie jeszcze nie będziecie wiedzieli jakie funkcje czy wyzwalacze napisać, więc wystarczy podać tylko deklaracje tych funkcji (gdzieś w komentarzu z opisem, co mniej-więcej będą one kiedyś robić). Typowy projekt ma posiadać ok. 6-8 nietrywialnych tabel (czyli bez tabel, których jedyną rolą jest implementacja relacji wiele-do-wielu), kilka funkcji/wyzwalaczy i może jakąś pomocniczą perspektywę. Na dzień dzisiejszy to wszystko co chcę Wam przekazać.


Wymagania
=========
* Git
* Rust nightly
* Postgres 9.x
* Postgis

Instalacja
==========

Najprostrzym sposobem na instalację Rust nightly jest wydanie polecenia:
```curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly```
Najnowsze źródła do mojego projektu są dostępne po wydaniu polecenia:
```git clone https://github.com/kamarkiewicz/smart_dealer.git```
W katalogu projektu wpisz `cargo run --release` w celu automatycznego jego zbudowania i uruchomienia.


Przydatne linki
===============

Hierarchia katalogów zaczerpnięta z:
http://www.terlici.com/2014/08/25/best-practices-express-structure.html
