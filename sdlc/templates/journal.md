# Journal <id>

Une entrée par étape, par itération et par porte, horodatée, la plus récente en bas.
Une décision humaine commence par « Validation humaine (<nom>) : ».

## <AAAA-MM-JJ HH:MM> · 01 Cadrer · P1
Validation humaine (<nom>) : <hypothèses tranchées, corrections>

## <AAAA-MM-JJ HH:MM> · 02 Planifier
<n> tâches ; critères couverts : C1…Cn

## <AAAA-MM-JJ HH:MM> · 03 Tests · P2
make check-red : OK
<sortie>

## <AAAA-MM-JJ HH:MM> · 04 Implémenter · itération <k>/3
Tâches : <…> · make check : VERT | ROUGE
<erreur décisive, hypothèse, ce qui change>

## <AAAA-MM-JJ HH:MM> · 05 Revue · tour <t>/2 · P4
Verdict reviewer : <…>
Validation humaine (<nom>) : <livrer | corriger | reprendre>, motif : <…>

## <AAAA-MM-JJ HH:MM> · 06 Livrer
make check : VERT · make test : VERT · B2 : <k> itérations · B3 : <t> tours · escalades : <e>
