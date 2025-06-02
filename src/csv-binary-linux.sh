#!/bin/bash

# Générer les transactions
./generate_transactions-linux

# Vérifier si binsort est installé
if ! command -v binsort &> /dev/null; then
    echo "binsort n'est pas installé. Veuillez l'installer"
    exit 1
fi

# Effectuer un binsort sur les transactions
binsort -s 23 -o 17 -l 2 out/transactions.bin out/transactions-filtered.bin

# Générer les soldes des utilisateur
./balances_by_user-linux

# Afficher les résultats
./read_balances_from_binary-linux

# Générer de nouvelles transactions
./generate_transactions-linux
binsort -s 23 -o 17 -l 2 out/transactions.bin out/transactions-filtered.bin

./update_balance_file-linux
./read_balances_from_binary-linux out/balances_updated.bin