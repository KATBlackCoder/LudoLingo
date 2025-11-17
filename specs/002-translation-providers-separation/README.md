# Spécification 002: Séparation des Providers de Traduction

## Vue d'ensemble

Cette spécification documente la migration de l'architecture de traduction pour séparer complètement Ollama (local) et RunPod (online).

## Documents

- **[spec.md](spec.md)**: Spécification complète de la fonctionnalité avec user stories et requirements techniques
- **[plan.md](plan.md)**: Plan d'implémentation détaillé avec phases et structure de code
- **[tasks.md](tasks.md)**: Liste détaillée des tâches à accomplir par phase

## Objectifs

1. **Nettoyer `translation/ollama`** pour être 100% local uniquement avec `ollama-rs`
2. **Créer `translation/runpod`** pour gérer uniquement les connexions RunPod avec `reqwest`
3. **Mettre à jour le frontend** pour permettre un choix clair entre les deux providers

## Phases d'implémentation

1. **Phase 1**: Backend - Nettoyage Ollama (Local uniquement)
2. **Phase 2**: Backend - Création RunPod
3. **Phase 3**: Backend - Coordination
4. **Phase 4**: Frontend - Settings
5. **Phase 5**: Frontend - Stores et Composants
6. **Phase 6**: Tests et Validation

## Dependencies

### Backend
- `ollama-rs = "0.3.2"` (existant)
- `reqwest = "0.11"` (nouveau)

### Frontend
- Aucune nouvelle dépendance nécessaire

## Migration

Les settings existants seront automatiquement migrés:
- `mode: 'local'` → `provider: 'ollama'`
- `mode: 'online'` → `provider: 'runpod'`
- Si `endpoint` RunPod existe, extraction automatique du POD_ID depuis l'URL

## Configuration RunPod

Pour simplifier la configuration, l'utilisateur entre uniquement le **POD_ID** (ex: `xedezhzb9la3ye`). L'URL est construite automatiquement selon le format RunPod:
- Format: `https://{pod_id}-11434.proxy.runpod.net`
- Port standard Ollama: `11434`

## Statut

- **Créé**: 2025-01-XX
- **Statut**: Draft
- **Branch**: `002-translation-providers-separation`

