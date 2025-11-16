# Quickstart: LudoLingo Game Localization

**Time to first localization**: ~15 minutes
**Prerequisites**: Ollama installed and running locally

## 1. Installation & Setup

### Prerequisites
- **Ollama**: Install from [ollama.ai](https://ollama.ai) and start the service
- **Language Models**: Pull the French model: `ollama pull llama2:13b` (or your preferred model)

### Application Setup
```bash
# Clone and setup the project
git clone <repository-url>
cd ludolingo
pnpm install
pnpm tauri dev  # Launch development mode
```

## 2. First Project Creation

1. **Launch LudoLingo**
   - Open the application
   - Click "Nouveau Projet"

2. **Configure Project**
   ```
   Nom: Mon Premier Jeu
   Langue source: ja (Japonais)
   Langue cible: fr (Français)
   Moteur: rpgmaker (RPG Maker MV/MZ)
   ```

3. **Save Project**
   - Click "Créer"
   - Project appears in the project list

## 3. Game File Scanning

1. **Select Project**
   - Click on your project in the list

2. **Start Scan**
   - Click "Scanner un dossier"
   - Select the game folder (containing data/, www/, etc.)
   - Check "Scan récursif" for complete scan

3. **Monitor Progress**
   - Watch real-time progress
   - View extracted files and text count
   - Check for any scan errors

## 4. Review Extracted Texts

1. **Navigate to Translations**
   - Go to "Translations" tab
   - View extracted texts by category:
     - Dialogues
     - Termes système
     - Objets
     - Compétences

2. **Filter and Search**
   - Use filters: status, type, texte source
   - Search specific terms
   - Sort by length or context

## 5. Batch Translation

1. **Select Texts**
   - Check multiple entries (1-100 recommended)
   - Or select entire categories

2. **Start Translation**
   - Click "Traduire par lots"
   - Choose batch name (optional)
   - Enable "Utiliser le glossary" if available

3. **Monitor Progress**
   - Real-time progress bar
   - View current processing text
   - Pause/cancel if needed

## 6. Glossary Management

1. **Add Terms Manually**
   - Go to "Glossary" tab
   - Click "Ajouter un terme"
   - Enter source and translation

2. **Extract from Translations**
   - Select a translation with a recurring term
   - Click "Extraire vers glossary"
   - Choose category and save

3. **Apply Glossary**
   - Select translations needing the term
   - Click "Appliquer le glossary"
   - Review automatic applications

## 7. Quality Review

1. **Review Translations**
   - Go through translated texts
   - Edit manually if needed
   - Mark as "reviewed" when satisfied

2. **Check Consistency**
   - Use glossary search
   - Verify term usage across files
   - Adjust translations for consistency

## 8. Final Injection

1. **Validate Before Injection**
   - Click "Valider l'injection"
   - Review summary and warnings
   - Fix any untranslated texts

2. **Start Injection**
   - Click "Injecter les traductions"
   - Automatic backup is created
   - Monitor progress

3. **Test the Game**
   - Launch the modified game
   - Verify translations in-game
   - Check for any display issues

## 9. Backup & Recovery

1. **Automatic Backups**
   - Created before each injection
   - Stored in project folder

2. **Manual Restore**
   - Go to "Paramètres" > "Sauvegardes"
   - Select backup to restore
   - Confirm restoration

## Troubleshooting

### Common Issues

**Ollama not responding:**
```bash
# Check Ollama status
ollama list
ollama serve  # Start if not running
```

**Scan fails on certain files:**
- Check file permissions
- Verify file is not corrupted
- Some encrypted files may not be scannable

**Injection creates invalid files:**
- Restore from backup
- Check for special characters in translations
- Validate file encoding

**Memory issues with large games:**
- Process in smaller batches
- Close other applications
- Check available RAM (minimum 8GB recommended)

### Performance Tips

- **Batch size**: 20-50 texts for optimal speed
- **Glossary first**: Build glossary before large translations
- **Regular saves**: Auto-save prevents data loss
- **File validation**: Always validate before injection

## Next Steps

- Explore advanced glossary features
- Set up multiple projects
- Customize translation workflows
- Integrate with external translation tools

For detailed documentation, see the project wiki.
