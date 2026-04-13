#!/bin/bash

# Script d'aide pour les exercices Rust

show_menu() {
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║        Exercices Rust - Menu des tests                    ║"
    echo "╠════════════════════════════════════════════════════════════╣"
    echo "║ 1. Tous les tests                                         ║"
    echo "║ 2. Tests du module 'basics' (Chapitres 1-5)              ║"
    echo "║ 3. Tests du module 'control_flow' (Chapitres 3, 6)       ║"
    echo "║ 4. Tests du module 'collections' (Chapitre 8)            ║"
    echo "║ 5. Tests du module 'error_handling' (Chapitre 9)         ║"
    echo "║ 6. Tests du module 'traits_generics' (Chapitre 10)       ║"
    echo "║ 7. Tests du module 'text_processing' (Chapitre 8+)       ║"
    echo "║ 8. Tests d'intégration                                    ║"
    echo "║ 9. Afficher la couverture (nombre de tests)              ║"
    echo "║ 0. Quitter                                                ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
}

run_tests() {
    case $1 in
        1)
            echo "📋 Exécution de tous les tests..."
            cargo test
            ;;
        2)
            echo "📚 Tests du module 'basics'..."
            cargo test basics::tests
            ;;
        3)
            echo "🔄 Tests du module 'control_flow'..."
            cargo test control_flow::tests
            ;;
        4)
            echo "📦 Tests du module 'collections'..."
            cargo test collections::tests
            ;;
        5)
            echo "⚠️  Tests du module 'error_handling'..."
            cargo test error_handling::tests
            ;;
        6)
            echo "🎯 Tests du module 'traits_generics'..."
            cargo test traits_generics::tests
            ;;
        7)
            echo "✍️  Tests du module 'text_processing'..."
            cargo test text_processing::tests
            ;;
        8)
            echo "🔗 Tests d'intégration..."
            cargo test --test integration_test_complete
            ;;
        9)
            echo "📊 Couverture des tests:"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            cargo test --lib 2>&1 | grep "test result:"
            ;;
        0)
            echo "Au revoir! 👋"
            exit 0
            ;;
        *)
            echo "❌ Option invalide"
            ;;
    esac
}

# Menu interactif
cd "$(dirname "$0")" || exit

while true; do
    show_menu
    read -p "Choisir une option (0-9): " choice
    
    if [ "$choice" != "0" ]; then
        run_tests "$choice"
        echo ""
        read -p "Appuyer sur Entrée pour continuer..."
    else
        run_tests "$choice"
    fi
done
