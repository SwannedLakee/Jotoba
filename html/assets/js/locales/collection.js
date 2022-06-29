const locales = {
    "en-US": {
        "LANG_JAP": "Japanese",
        "LANG_GER": "German",
        "LANG_ENG": "English",
        "LANG_RUS": "Russian",
        "LANG_SPA": "Spanish",
        "LANG_SWE": "Swedish",
        "LANG_FRE": "French",
        "LANG_DUT": "Dutch",
        "LANG_HUN": "Hungarian",
        "LANG_SLV": "Slovenian",
        "SETTINGS_COOKIE_ACCEPT": "Thanks for helping to improve Jotoba!",
        "SETTINGS_COOKIE_REJECT": "We will no longer collect <b>any</b> data.",
        "UPLOAD_NO_INPUT": "You need to enter a URL or upload a file!",
        "RADICAL_API_UNREACHABLE": "Could not reach Radical API.",
        "SPEECH_LISTEN_YES": "Yes",
        "SPEECH_LISTEN_NO": "No",
        "SPEECH_NO_PERMISSION": "Need permissions to perform speech recognition!",
        "SPEECH_ABORT": "Speech recognition aborted.",
        "SPEECH_NO_VOICE": "No voice input received!",
        "SPEECH_NOT_SUPPORTED": "Your browser does not support speech recognition!",
        "QOL_FURI_COPIED": "furigana copied to clipboard.",
        "QOL_FURI_COPIED_ALL": "<b>full</b> furigana copied to clipboard",
        "QOL_KANJI_COPIED": "kanji copied to clipboard.",
        "QOL_KANA_COPIED": "kana copied to clipboard.",
        "QOL_SENTENCE_COPIED": "copied to clipboard.",
        "QOL_AUDIO_COPIED": "Audio URL copied to clipboard",
        "QOL_LINK_COPIED": "Link URL copied to clipboard",
    },
    "de-DE": {
        "LANG_JAP": "Japanisch",
        "LANG_GER": "Deutsch",
        "LANG_ENG": "Englisch",
        "LANG_RUS": "Russisch",
        "LANG_SPA": "Spanisch",
        "LANG_SWE": "Schwedisch",
        "LANG_FRE": "Französisch",
        "LANG_DUT": "Niederländisch",
        "LANG_HUN": "Ungarisch",
        "LANG_SLV": "Slowenisch",
        "SETTINGS_COOKIE_ACCEPT": "Vielen Dank für die Unterstützung!",
        "SETTINGS_COOKIE_REJECT": "Es werden keine Daten mehr gesammelt!",
        "UPLOAD_NO_INPUT": "Du musst entweder eine Datei hochladen oder eine URL einfügen, welche auf ein Bild zeigt!",
        "RADICAL_API_UNREACHABLE": "Konnte die Radikal-API nicht erreichen.",
        "SPEECH_LISTEN_YES": "Ja",
        "SPEECH_LISTEN_NO": "Nein",
        "SPEECH_NO_PERMISSION": "Jotoba benötigt Berechtigungen für die Spracherkennung!",
        "SPEECH_ABORT": "Spracherkennung abgebrochen.",
        "SPEECH_NO_VOICE": "Wir konnten Deine Stimme nicht hören!",
        "SPEECH_NOT_SUPPORTED": "Dein Browser unterstützt dieses Feature leider nicht!",
        "QOL_FURI_COPIED": "Furigana in Zwischenablage kopiert",
        "QOL_FURI_COPIED_ALL": "<b>Vollständiges</b> Furigana in Zwischenablage kopiert",
        "QOL_KANJI_COPIED": "Kanji in Zwischenablage kopiert.",
        "QOL_KANA_COPIED": "Kana in Zwischenablage kopiert",
        "QOL_SENTENCE_COPIED": "Text in Zwischenablage kopiert",
        "QOL_AUDIO_COPIED": "Audio URL in Zwischenablage kopiert",
        "QOL_LINK_COPIED": "Link URL in Zwischenablage kopiert",
    },
    "hu": {
        "LANG_JAP": "Japán",
        "LANG_GER": "Német",
        "LANG_ENG": "Angol",
        "LANG_RUS": "Orosz",
        "LANG_SPA": "Spanyol",
        "LANG_SWE": "Svéd",
        "LANG_FRE": "Francia",
        "LANG_DUT": "Holland",
        "LANG_HUN": "Magyar",
        "LANG_SLV": "Szlovák",
        "SPEECH_LISTEN_YES": "Igen",
        "SPEECH_LISTEN_NO": "Nem",
    },
};

// Returns the text with the given identifier from the currently selected language
function getText(identifier) {
    let lang = Cookies.get("page_lang") || "en-US";

    return locales[lang][identifier] || locales["en-US"][identifier] || identifier;
}