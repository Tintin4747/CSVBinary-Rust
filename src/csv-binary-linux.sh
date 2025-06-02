#!/bin/bash

# Fonction utilitaire pour mesurer la durée
time_step() {
    local label="$1"
    shift
    start=$(date +%s)
    local start
    "$@"
    local status=$?
    end=$(date +%s)
    local end
    local duration=$((end - start))
    echo "Durée pour $label : ${duration}s"
    return $status
}

# Générer les transactions
time_step "Génération des transactions (1)" ./generate_transactions-linux

# Vérifier si binsort est installé
if ! command -v binsort &> /dev/null; then
    echo "binsort n'est pas installé. Veuillez l'installer"
    exit 1
fi

# Effectuer un binsort sur les transactions
binsort -s 23 -o 17 -l 2 out/transactions.bin out/transactions-filtered.bin

# Générer les soldes des utilisateur
time_step "Génération des soldes utilisateurs" ./balance_by_user-linux

# Afficher les résultats
./read_balances_from_binary-linux

# Générer de nouvelles transactions
time_step "Génération des transactions (2)" ./generate_transactions-linux
binsort -s 23 -o 17 -l 2 out/transactions.bin out/transactions-filtered.bin

time_step "Mise à jour des soldes utilisateurs" ./update_balance_file-linux
./read_balances_from_binary-linux out/balances_updated.bin