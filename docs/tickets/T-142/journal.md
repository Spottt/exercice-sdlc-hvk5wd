# Journal T-142

Trace horodatée du cycle, la plus récente en bas. Presque tout s'y écrit sans l'agent :

| Entrée | Qui l'écrit |
| --- | --- |
| `03 Tests · P2` : résultat de `check-red`, une ligne par test | `make check-red` |
| `04 Implémenter · itération k/3` : VERT ou ROUGE, étape et erreurs | `make check` |
| `Diagnostic :` sous une itération rouge | l'agent |
| `05 Revue · tour t/2` : verdict du reviewer | l'agent |
| `P1`, `P4`, `escalade` : « Validation humaine (nom git) : … » | l'humain, `make valider P=… MSG="…"` |
| `06 Livrer · bilan` : totaux du cycle | `make bilan` |

## 2026-11-15 17:14 · P1 · validation humaine
Validation humaine (Enzo Olmos) : hypothèses 1 à 8 validées

## 2026-11-15 17:14 · 02 Planifier
3 tâches ; critères couverts : C1…C8

## 2026-11-15 17:15 · 03 Tests · P2
make check-red : OK
test le_salon_et_ses_stands_sont_listes ... ok
test un_stand_ne_peut_etre_reserve_qu_une_fois ... ok
test reserver_un_stand_calcule_un_acompte_de_30_pour_cent ... ok
test les_demandes_invalides_sont_refusees ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test c7_une_reservation_inconnue_renvoie_404 ... FAILED
test c1_changer_pour_un_stand_plus_cher_libere_l_ancien_et_monte_l_acompte ... FAILED
test c5_un_stand_deja_reserve_par_un_autre_est_refuse_et_rien_ne_change ... FAILED
test c6_un_autre_nom_que_celui_de_la_reservation_est_refuse ... FAILED
test c2_changer_pour_un_stand_moins_cher_baisse_l_acompte_arrondi_au_centime_inferieur ... FAILED
test c8_un_stand_inconnu_renvoie_404 ... FAILED
test c4_le_changement_est_refuse_a_j_moins_6_et_la_reservation_reste_inchangee ... FAILED
test c3_le_changement_est_accepte_a_j_moins_7 ... FAILED
test result: FAILED. 0 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
check-red OK : nouveaux tests rouges, anciens verts
