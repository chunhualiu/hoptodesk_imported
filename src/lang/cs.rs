lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Stav"),
        ("Your Desktop", "Vaše plocha"),
        ("desk_tip", "Pomocí tohoto identifikátoru a hesla můžete přistupovat ke své ploše."),
        ("Password", "Heslo"),
        ("Ready", "Připraveno"),
        ("Established", "Navázáno"),
        ("connecting_status", "Připojování se k HopToDesk síti…"),
        ("connecting_status_short", "Připojování..."),
        ("Enable Service", "Povolit službu"),
        ("Start Service", "Spustit službu"),
        ("Service is running", "Služba je spuštěná"),
        ("Service is not running", "Služba není spuštěná"),
        ("not_ready_status", "Nepřipraveno. Zkontrolujte své připojení."),
		("not_ready_status_short", "Nepřipraveno"),
        ("Control Remote Desktop", "Ovládat vzdálenou plochu"),
        ("Transfer File", "Přenos souborů"),
        ("Connect", "Připojit"),
        ("Recent sessions", "Nedávné relace"),
        ("Address book", "Adresář kontaktů"),
        ("Confirmation", "Potvrzení"),
        ("TCP tunneling", "TCP tunelování"),
        ("Remove", "Odebrat"),
        ("Refresh random password", "Vytvořit nové náhodné heslo"),
        ("Set your own password", "Nastavte si své vlastní heslo"),
        ("Enable keyboard/mouse", "Povolit klávesnici/myš"),
        ("Enable clipboard", "Povolit schránku"),
        ("Enable file transfer", "Povolit přenos souborů"),
        ("Enable TCP tunneling", "Povolit TCP tunelování"),
        ("IP Whitelisting", "Povolování pouze z daných IP adres"),
        ("ID/Relay Server", "ID/předávací server"),
        ("Import server config", "Importovat konfiguraci serveru"),
        ("Export Server Config", "Exportovat konfiguraci serveru"),
        ("Import server configuration successfully", "Konfigurace serveru úspěšně importována"),
        ("Export server configuration successfully", "Konfigurace serveru úspěšně exportována"),
        ("Invalid server configuration", "Neplatná konfigurace serveru"),
        ("Clipboard is empty", "Schránka je prázdná"),
        ("Stop service", "Zastavit službu"),
        ("Change ID", "Změnit ID"),
        ("Your new ID", "Váše nové ID"),
        ("length %min% to %max%", "délka mezi %min% a %max%"),
        ("starts with a letter", "začíná písmenem"),
        ("allowed characters", "povolené znaky"),
        ("id_change_tip", "Použít je možné pouze znaky a-z, A-Z, 0-9 a _ (podtržítko). Dále je třeba aby začínalo písmenem a-z, A-Z. Délka mezi 6 a 16 znaky."),
        ("Website", "Webové stránky"),
        ("About", "O aplikaci"),
        ("Privacy Statement", "Prohlášení o ochraně osobních údajů"),
        ("Mute", "Ztlumit zvuk"),
        ("Build Date", "Datum sestavení"),
        ("Version", "Verze"),
        ("Home", "Domů"),
        ("Audio Input", "Vstup zvuku"),
        ("Enhancements", "Vylepšení"),
        ("Hardware Codec", "Hardwarový kodek"),
        ("Adaptive bitrate", "Adaptivní datový tok"),
        ("ID Server", "ID Server"),
        ("Relay Server", "Předávací server"),
        ("API Server", "API Server"),
        ("invalid_http", "Je třeba, aby začínalo na http:// nebo https://"),
        ("Invalid IP", "Neplatná IP"),
        ("Invalid format", "Neplatný formát"),
        ("server_not_support", "Server zatím nepodporuje"),
        ("Not available", "Není k dispozici"),
        ("Too frequent", "Příliš časté"),
        ("Cancel", "Storno"),
        ("Skip", "Přeskočit"),
        ("Close", "Zavřít"),
        ("Retry", "Zkusit znovu"),
        ("OK", "OK"),
        ("Password Required", "Vyžadováno heslo"),
        ("Please enter your password", "Zadejte své heslo"),
        ("Remember password", "Zapamatovat heslo"),
        ("Wrong Password", "Nesprávné heslo"),
        ("Do you want to enter again?", "Chcete se znovu připojit?"),
        ("Connection Error", "Chyba spojení"),
        ("Error", "Chyba"),
        ("Connection lost", "Resetováno protějškem"),
        ("Connecting...", "Připojování..."),
        ("Connection in progress. Please wait.", "Probíhá připojování, vyčkejte prosím."),
        ("Please try 1 minute later", "Zkuste to prosím o 1 minutu později"),
        ("Login Error", "Chyba přihlášení se"),
        ("Successful", "Úspěšné"),
        ("Connected, waiting for image...", "Připojeno, čeká se na obraz..."),
        ("Name", "Název"),
        ("Type", "Typ"),
        ("Modified", "Změněno"),
        ("Size", "Velikost"),
        ("Show Hidden Files", "Zobrazit skryté soubory"),
        ("Receive", "Přijmout"),
        ("Send", "Odeslat"),
        ("Refresh File", "Znovu načíst soubor"),
        ("Local", "Místní"),
        ("Remote", "Vzdálené"),
        ("Remote Computer", "Vzdálený počítač"),
        ("Local Computer", "Místní počítač"),
        ("Confirm Delete", "Potvrdit smazání"),
        ("Delete", "Smazat"),
        ("Properties", "Vlastnosti"),
        ("Multi Select", "Vícenásobný výběr"),
        ("Select All", "Vybrat vše"),
        ("Unselect All", "Zrušit výběr všech"),
        ("Empty Directory", "Prázdná složka"),
        ("Not an empty directory", "Neprázdná složka"),
        ("Are you sure you want to delete this file?", "Opravdu chcete tento soubor vymazat?"),
        ("Are you sure you want to delete this empty directory?", "Opravdu chcete tuto prázdnou složku smazat?"),
        ("Are you sure you want to delete the file of this directory?", "Opravdu chcete vymazat soubor z této složky?"),
        ("Do this for all conflicts", "Naložit takto se všemi konflikty"),
        ("This is irreversible!", "Toto nelze vzít zpět"),
        ("Deleting", "Mazání"),
        ("files", "soubory"),
        ("Waiting", "Čeká se"),
        ("Finished", "Dokončeno"),
        ("Speed", "Rychlost"),
        ("Custom Image Quality", "Uživatelsky určená kvalita obrazu"),
        ("Privacy mode", "Režim ochrany soukromí"),
        ("Block user input", "Blokovat vstupní zařízení uživatele"),
        ("Unblock user input", "Odblokovat vstupní zařízení uživatele"),
        ("Adjust Window", "Přizpůsobit velikost okna"),
        ("Original", "Původní"),
        ("Shrink", "Oříznout"),
        ("Stretch", "Roztáhnout"),
        ("Scrollbar", "Posuvník"),
        ("ScrollAuto", "Automatické rolování"),
        ("Good image quality", "Dobrá kvalita obrazu"),
        ("Balanced", "Vyvážená"),
        ("Optimize reaction time", "Optimalizovat reakční dobu"),
        ("Custom", "Vlastní"),
        ("Show remote cursor", "Zobrazit vzdálený kurzor"),
        ("Show quality monitor", "Zobrazit monitor kvality"),
        ("Disable clipboard", "Vypnout schránku"),
        ("Lock after session end", "Po ukončení relace zamknout plochu"),
        ("Insert", "Vložit"),
        ("Insert Lock", "Zamknout"),
        ("Refresh", "Načíst znovu"),
        ("ID does not exist", "Toto ID neexistuje"),
        ("Failed to connect to rendezvous server", "Nepodařilo se připojit ke zprostředkovávajícímu serveru"),
        ("Please try later", "Zkuste to později"),
        ("Remote desktop is offline", "Vzdálená plocha není připojená ke službě"),
        ("Key mismatch", "Neshoda klíčů"),
        ("Timeout", "Překročen časový limit pro navázání spojení"),
        ("Failed to connect to relay server", "Nepodařilo se připojit k předávacímu (relay) serveru"),
        ("Unable to connect to the remote partner.", "Nelze se připojit ke vzdálenému partnerovi."),
        ("Failed to connect via relay server", "Nepodařilo se připojit prostřednictvím předávacímu (relay) serveru"),
        ("Failed to make direct connection to remote desktop", "Nepodařilo s navázat přímé připojení ke vzdálené ploše"),
        ("Set Password", "Nastavit heslo"),
        ("OS Password", "Heslo do operačního systému"),
        ("install_tip", "Pro nejlepší výkon dokončete úplnou instalaci."),
        ("Click to upgrade", "Aktualizovat"),
        ("Click to download", "Stáhnout"),
        ("Click to update", "Aktualizovat"),
        ("Configure", "Nastavit"),
        ("config_acc", "Aby bylo možné na dálku ovládat vaši plochu, je třeba aplikaci HopToDesk udělit oprávnění pro \"Zpřístupnění pro hendikepované\"."),
        ("config_screen", "Aby bylo možné přistupovat k vaší ploše na dálku, je třeba aplikaci HopToDesk udělit oprávnění pro \"Nahrávání obsahu obrazovky\"."),
        ("Installing ...", "Instaluje se ..."),
        ("Install", "Nainstalovat"),
        ("Installation", "Instalace"),
        ("Installation Path", "Umístění instalace"),
        ("Create start menu shortcuts", "Vytvořit zástupce v nabídce Start"),
        ("Create desktop icon", "Vytvořit ikonu na ploše"),
        ("agreement_tip", "Spuštěním instalace přijímáte licenční ujednání."),
        ("Accept and Install", "Přijmout a nainstalovat"),
        ("End-user license agreement", "Licencenční ujednání s koncovým uživatelem"),
        ("Generating ...", "Vytváření ..."),
        ("A new update is available.", "Máte nainstalovanou starší verzi"),
        ("not_close_tcp_tip", "Po dobu, po kterou tunel potřebujete, nezavírejte toto okno"),
        ("Listening ...", "Očekávaní spojení ..."),
        ("Remote Host", "Vzdálený hostitel"),
        ("Remote Port", "Vzdálený port"),
        ("Action", "Akce"),
        ("Add", "Přidat"),
        ("Local Port", "Místní port"),
        ("Local Address", "Místní adresa"),
        ("Change Local Port", "Změnit místní port"),
        ("setup_server_tip", "Rychlejší připojení získáte vytvořením si svého vlastního serveru"),
        ("Too short, at least 6 characters.", "Příliš krátké, alespoň 6 znaků."),
        ("The confirmation is not identical.", "Kontrolní zadání se neshoduje."),
        ("Permissions", "Oprávnění"),
        ("Accept", "Přijmout"),
        ("Dismiss", "Zahodit"),
        ("Disconnect", "Odpojit"),
        ("Enable file copy and paste", "Povolit kopírování a vkládání souborů"),
        ("Connected", "Připojeno"),
        ("Direct and encrypted connection", "Přímé a šifrované spojení"),
        ("Relayed and encrypted connection", "Předávané (relay) a šifrované spojení"),
        ("Direct and unencrypted connection", "Přímé a nešifrované spojení"),
        ("Relayed and unencrypted connection", "Předávané (relay) a nešifrované spojení"),
        ("Enter Remote ID", "Zadejte ID protistrany"),
        ("Enter your password", "Zadejte své heslo"),
        ("Logging in...", "Přihlašování..."),
        ("Enable RDP session sharing", "Povolit sdílení relace RDP"),
        ("Auto Login", "Automatické přihlášení"),
        ("Enable Direct IP Access", "Povolit přímý přístup k IP"),
        ("Rename", "Přejmenovat"),
        ("Space", "Mezera"),
        ("Create Desktop Shortcut", "Vytvořit zástupce na ploše"),
        ("Change Path", "Změnit umístění"),
        ("Create Folder", "Vytvořit složku"),
        ("Please enter the folder name", "Zadejte název pro složku"),
        ("Disable Wayland", "Vypněte Wayland"),
        ("Warning", "Upozornění"),
        ("Login screen using Wayland is not supported.", "Přihlašovací obrazovka pomocí systému Wayland není podporována"),
        ("Reboot required", "Je vyžadován restart"),
        ("Unsupported display server", "Nepodporovaný zobrazovací server"),
        ("x11 expected", "očekávaný x11"),
        ("Port", "Port"),
        ("Settings", "Nastavení"),
        ("Username", "Uživatelské jméno"),
        ("Invalid port", "Neplatné číslo portu"),
        ("The remote partner has closed the session.", "Ručně ukončeno protějškem"),
        ("Enable remote configuration modification", "Povolit vzdálenou úpravu konfigurace"),
        ("Run without install", "Spustit bez instalace"),
        ("Connect via relay", "Připojení přes předávací server"),
        ("Always connect via relay", "Vždy se připojovat prostřednictvím předávacího serveru"),
        ("whitelist_tip", "Přístup je umožněn pouze z IP adres, nacházejících se na seznamu povolených"),
        ("Login", "Přihlásit se"),
        ("Verify", "Ověřit"),
        ("Remember me", "Zapamatovat si"),
        ("Trust this device", "Důvěřovat tomuto zařízení"),
        ("Verification code", "Ověřovací kód"),
        ("verification_tip", "Na registrovanou e-mailovou adresu byl zaslán ověřovací kód, zadejte jej a pokračujte v přihlašování."),
        ("Logout", "Odhlásit se"),
        ("Tags", "Štítky"),
        ("Search ID", "Hledat ID"),
        ("whitelist_sep", "Oddělené čárkou, středníkem, mezerami, nebo novým řádkem."),
        ("Add ID", "Přidat ID"),
        ("Add Tag", "Přidat štítek"),
        ("Unselect all tags", "Zrušit výběr všech štítků"),
        ("Network error", "Chyba sítě"),
        ("Username missed", "Chybí uživatelské jméno"),
        ("Password missed", "Chybí heslo"),
        ("Wrong credentials", "Nesprávné přihlašovací údaje"),
        ("The verification code is incorrect or has expired", "Ověřovací kód je nesprávný, nebo jeho platnost vypršela"),
        ("Edit Tag", "Upravit štítek"),
        ("Forget Password", "Přestat si pamatovat heslo"),
        ("Favorites", "Oblíbené"),
        ("Add to Favorites", "Přidat do oblíbených"),
        ("Remove from Favorites", "Odebrat z oblíbených"),
        ("Empty", "Prázdné"),
        ("Invalid folder name", "Neplatný název složky"),
        ("SOCKS5 Proxy", "SOCKS5 proxy"),
        ("Hostname", "Název hostitele"),
        ("Discovered", "Objeveno"),
        ("install_daemon_tip", "Pokud má být spouštěno při startu systému, je třeba nainstalovat systémovou službu."),
        ("Remote ID", "Vzdálené ID"),
        ("Paste", "Vložit"),
        ("Paste here?", "Vložit zde?"),
        ("Are you sure to close the connection?", "Opravdu chcete spojení uzavřít?"),
        ("Download new version", "Stáhnout novou verzi"),
        ("Touch mode", "Režim dotyku"),
        ("Mouse mode", "Režim myši"),
        ("One-Finger Tap", "Klepnutí jedním prstem"),
        ("Left Mouse", "Levé tlačítko myši"),
        ("One-Long Tap", "Jedno dlouhé klepnutí"),
        ("Two-Finger Tap", "Klepnutí dvěma prsty"),
        ("Right Mouse", "Pravé tlačítko myši"),
        ("One-Finger Move", "Přesouvání jedním prstem"),
        ("Double Tap & Move", "Dvojité klepnutí a přesun"),
        ("Mouse Drag", "Přetažení myší"),
        ("Three-Finger vertically", "Třemi prsty svisle"),
        ("Mouse Wheel", "Kolečko myši"),
        ("Two-Finger Move", "Posun dvěma prsty"),
        ("Canvas Move", "Posun zobrazení"),
        ("Pinch to Zoom", "Přiblížíte roztažením dvěma prsty"),
        ("Canvas Zoom", "Přiblížení zobrazení"),
        ("Reset canvas", "Vrátit měřtko zobrazení na výchozí"),
        ("No permission of file transfer", "Žádné oprávnění k přenosu souborů"),
        ("Note", "Poznámka"),
        ("Connection", "Připojení"),
        ("Share Screen", "Sdílet obrazovku"),
        ("Chat", "Chat"),
        ("Total", "Celkem"),
        ("items", "Položek"),
        ("Selected", "Vybráno"),
        ("Screen Capture", "Zachytávání obrazovky"),
        ("Input Control", "Ovládání vstupních zařízení"),
        ("Audio Capture", "Zachytávání zvuku"),
        ("File Connection", "Souborové spojení"),
        ("Screen Connection", "Spojení obrazovky"),
        ("Do you accept?", "Přijímáte?"),
        ("Open System Setting", "Otevřít nastavení systému"),
        ("How to get Android input permission?", "Jak v systému Android získat oprávnění pro vstupní zařízení?"),
        ("android_input_permission_tip1", "Aby vzdálené zařízení mohlo ovládat vaše Android zařízení prostřednictví myši či dotyků, je třeba povolit, aby HopToDesk mohlo používat službu „Zpřístupnění hendikepovaným“."),
        ("android_input_permission_tip2", "Přejděte na následující stránku nastavení systému, najděte a přejděte do [Nainstalované služby] a zapněte službu [HopToDesk vstup]."),
        ("android_new_connection_tip", "Obdržen nový požadavek na řízení zařízení, který chce ovládat vaše stávající zařízení."),
        ("android_service_will_start_tip", "Zapnutí „Zachytávání obsahu obrazovky“ automaticky spustí službu, což umožní ostatním zařízením žádat o připojení k vašemu zařízení."),
        ("android_stop_service_tip", "Zastavení služby automaticky ukončí veškerá navázaná spojení."),
        ("android_version_audio_tip", "Vámi nyní používaná verze systému Android nepodporuje zachytávání zvuku – přejděte na Android 10, nebo novější."),
        ("android_start_service_tip", "Klepnutím na možnost [Spustit službu], nebo povolením oprávnění [Snímání obrazovky] spustíte službu sdílení obrazovky."),
        ("android_permission_may_not_change_tip", "Oprávnění pro navázaná připojení lze změnit až po opětovném připojení."),
        ("Account", "Účet"),
        ("Overwrite", "Přepsat"),
        ("This file exists, skip or overwrite this file?", "Tento soubor existuje, přeskočit, nebo přepsat tento soubor?"),
        ("Quit", "Ukončit"),
        ("Help", "Nápověda"),
        ("Failed", "Nepodařilo se"),
        ("Succeeded", "Úspěšný"),
        ("Someone turned on privacy mode, exiting.", "Někdo zapne režim ochrany soukromí, ukončete ho"),
        ("Unsupported", "Nepodporováno"),
        ("Peer denied", "Protistana odmítnula"),
        ("Please install plugins", "Nainstalujte si prosím pluginy"),
        ("The peer has exited from Privacy Mode.", "Ukončení protistrany"),
        ("Failed to turn off", "Nepodařilo se vypnout"),
        ("Turned off", "Vypnutý"),
        ("Language", "Jazyk"),
        ("Keep HopToDesk background service", "Zachovat službu HopToDesk na pozadí"),
        ("Ignore Battery Optimizations", "Ignorovat optimalizaci baterie"),
        ("android_open_battery_optimizations_tip", "Pokud chcete tuto funkci zakázat, přejděte na další stránku nastavení aplikace HopToDesk, najděte a zadejte [Baterie], zrušte zaškrtnutí [Neomezeno]."),
        ("Start on Boot", "Spustit při startu systému"),
        ("Start the screen sharing service on boot, requires special permissions", "Spuštění služby sdílení obrazovky při spuštění systému, vyžaduje zvláštní oprávnění"),
        ("Connection not allowed", "Připojení není povoleno"),
        ("Legacy mode", "Režim Legacy"),
        ("Map mode", "Režim mapování"),
        ("Translate mode", "Režim překladu"),
        ("Use permanent password", "Použít trvalé heslo"),
        ("Use both passwords", "Použít obě hesla"),
        ("Set permanent password", "Nastavit trvalé heslo"),
        ("Enable Remote Restart", "Povolit vzdálené restartování"),
        ("Restart Remote Device", "Restartovat vzdálené zařízení"),
        ("Are you sure you want to restart", "Jste si jisti, že chcete restartovat"),
        ("Restarting Remote Device", "Restartování vzdáleného zařízení"),
        ("remote_restarting_tip", "Vzdálené zařízení se restartuje, zavřete prosím toto okno a po chvíli se znovu připojte pomocí trvalého hesla."),
        ("Copied", "Zkopírováno"),
        ("Exit Fullscreen", "Ukončit celou obrazovku"),
        ("Fullscreen", "Celá obrazovka"),
        ("Mobile Actions", "Mobilní akce"),
        ("Select Monitor", "Vybrat monitor"),
        ("Control Actions", "Ovládací akce"),
        ("Display Settings", "Nastavení obrazovky"),
        ("Ratio", "Poměr"),
        ("Image Quality", "Kvalita obrazu"),
        ("Scroll Style", "Štýl posúvania"),
        ("Show Toolbar", "Zobrazit panel nástrojů"),
        ("Hide Toolbar", "Skrýt panel nástrojů"),
        ("Direct Connection", "Přímé spojení"),
        ("Relay Connection", "Připojení předávací server"),
        ("Secure Connection", "Zabezpečené připojení"),
        ("Insecure Connection", "Nezabezpečené připojení"),
        ("Scale original", "Originální měřítko"),
        ("Scale adaptive", "Adaptivní měřítko"),
        ("General", "Obecné"),
        ("Security", "Zabezpečení"),
        ("Theme", "Motiv"),
        ("Dark Theme", "Tmavý motiv"),
        ("Light Theme", "Světlý motiv"),
        ("Dark", "Tmavý"),
        ("Light", "Světlý"),
        ("Follow System", "Podle systému"),
        ("Enable hardware codec", "Povolit hardwarový kodek"),
        ("Unlock Security Settings", "Odemknout nastavení zabezpečení"),
        ("Enable Audio", "Povolit zvuk"),
        ("Unlock Network Settings", "Odemknout nastavení sítě"),
        ("Server", "Server"),
        ("Direct IP Access", "Přímý IP přístup"),
        ("Proxy", "Proxy"),
        ("Apply", "Použít"),
        ("Disconnect all devices?", "Odpojit všechna zařízení?"),
        ("Clear", "Smazat"),
        ("Audio Input Device", "Vstupní zvukové zařízení"),
        ("Use IP Whitelisting", "Použít bílou listinu IP"),
        ("Network", "Síť"),
        ("Pin Toolbar", "Připnout panel nástrojů"),
        ("Unpin Toolbar", "Odepnout panel nástrojů"),
        ("Recording", "Nahrávání"),
        ("Directory", "Adresář"),
        ("Automatically record incoming sessions", "Automaticky nahrávat příchozí relace"),
        ("Change", "Změnit"),
        ("Start session recording", "Spustit záznam relace"),
        ("Stop session recording", "Zastavit záznam relace"),
        ("Enable Recording Session", "Povolit nahrávání relace"),
        ("Enable LAN Discovery", "Povolit zjišťování sítě LAN"),
        ("Deny LAN Discovery", "Zakázat zjišťování sítě LAN"),
        ("Write a message", "Napsat zprávu"),
        ("Prompt", "Výzva"),
        ("Please wait for confirmation of UAC...", "Počkejte prosím na potvrzení UAC..."),
        ("elevated_foreground_window_tip", "Aktuální okno vzdálené plochy vyžaduje vyšší oprávnění, takže dočasně nemůže používat myš a klávesnici. Můžete vzdáleného uživatele požádat, aby aktuální okno minimalizoval, nebo kliknout na tlačítko pro zvýšení v okně správy připojení. Chcete-li se tomuto problému vyhnout, doporučujeme nainstalovat software na vzdálené zařízení."),
        ("Disconnected", "Odpojeno"),
        ("Other", "Jiné"),
        ("Confirm before closing multiple tabs", "Potvrdit před zavřením více karet"),
        ("Keyboard Settings", "Nastavení klávesnice"),
        ("Full Access", "Úplný přístup"),
        ("Screen Share", "Sdílení obrazovky"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland vyžaduje Ubuntu 21.04, nebo vyšší verzi."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland vyžaduje vyšší verzi linuxové distribuce. Zkuste prosím X11 desktop, nebo změňte OS."),
        ("JumpLink", "JumpLink"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Vyberte prosím obrazovku, kterou chcete sdílet (Ovládejte na straně protistrany)."),
        ("Show HopToDesk", "Zobrazit HopToDesk"),
        ("This PC", "Tento počítač"),
        ("or", "nebo"),
        ("Continue with", "Pokračovat s"),
        ("Elevate", "Zvýšit"),
        ("Zoom cursor", "Kurzor přiblížení"),
        ("Accept sessions via password", "Přijímat relace pomocí hesla"),
        ("Accept sessions via click", "Přijímat relace kliknutím"),
        ("Accept sessions via both", "Přijímat relace prostřednictvím obou"),
        ("Please wait for the remote side to accept your session request...", "Počkejte prosím, až vzdálená strana přijme váš požadavek na relaci..."),
        ("One-time Password", "Jednorázové heslo"),
        ("Use one-time password", "Použít jednorázové heslo"),
        ("One-time password length", "Délka jednorázového hesla"),
        ("Request access to your device", "Žádost o přístup k vašemu zařízení"),
        ("Hide connection management window", "Skrýt okno správy připojení"),
        ("hide_cm_tip", "Povolit skrývání pouze v případě, že přijímáte relace pomocí hesla a používáte trvalé heslo."),
        ("wayland_experiment_tip", "Podpora Waylandu je v experimentální fázi, pokud potřebujete bezobslužný přístup, použijte prosím X11."),
        ("Right click to select tabs", "Výběr karet kliknutím pravým tlačítkem myši"),
        ("Skipped", "Vynecháno"),
        ("Add to Address Book", "Přidat do adresáře"),
        ("Group", "Skupina"),
        ("Search", "Vyhledávání"),
        ("Closed manually by web console", "Uzavřeno ručně pomocí webové konzole"),
        ("Local keyboard type", "Typ místní klávesnice"),
        ("Select local keyboard type", "Výběr typu místní klávesnice"),
        ("software_render_tip", "Pokud používáte grafickou kartu Nvidia v systému Linux a vzdálené okno se po připojení ihned zavře, může vám pomoci přepnutí na open-source ovladač Nouveau a volba softwarového vykreslování. Je nutný restart softwaru."),
        ("Always use software rendering", "Vždy použít softwarové vykreslování"),
        ("config_input", "Chcete-li ovládat vzdálenou plochu pomocí klávesnice, musíte udělit oprávnění HopToDesk \"Sledování vstupu\"."),
        ("config_microphone", "Abyste mohli mluvit na dálku, musíte udělit oprávnění HopToDesk \"Nahrávat zvuk\"."),
        ("request_elevation_tip", "Můžete také požádat o zvýšení, pokud je někdo na vzdálené straně."),
        ("Wait", "Počkejte"),
        ("Elevation Error", "Chyba navýšení"),
        ("Ask the remote user for authentication", "Požádat vzdáleného uživatele o ověření"),
        ("Choose this if the remote account is administrator", "Tuto možnost vyberte, pokud je vzdálený účet správce"),
        ("Transmit the username and password of administrator", "Přenos uživatelského jména a hesla správce"),
        ("still_click_uac_tip", "Stále vyžaduje, aby vzdálený uživatel kliknul na OK v okně UAC spuštěného HopToDesk."),
        ("Request Elevation", "Žádost o navýšení"),
        ("wait_accept_uac_tip", "Počkejte, až vzdálený uživatel přijme dialogové okno UAC."),
        ("Elevate successfully", "Úspěšné navýšení"),
        ("uppercase", "velká písmena"),
        ("lowercase", "malá písmena"),
        ("digit", "číslice"),
        ("special character", "speciální znak"),
        ("length>=8", "délka>=8"),
        ("Weak", "Slabé"),
        ("Medium", "Střední"),
        ("Strong", "Silné"),
        ("Switch Sides", "Přepínání stran"),
        ("Please confirm if you want to share your desktop?", "Potvrďte prosím, zda chcete sdílet svou plochu?"),
        ("Display", "Obrazovka"),
        ("Default View Style", "Výchozí styl zobrazení"),
        ("Default Scroll Style", "Výchozí styl rolování"),
        ("Default Image Quality", "Výchozí kvalita obrazu"),
        ("Default Codec", "Výchozí kodek"),
        ("Bitrate", "Datový tok"),
        ("FPS", "FPS"),
        ("Auto", "Auto"),
        ("Other Default Options", "Ostatní výchozí možnosti"),
        ("Voice call", "Hlasové volání"),
        ("Text chat", "Textový chat"),
        ("Stop voice call", "Zastavit hlasové volání"),
        ("relay_hint_tip", "Přímé připojení nemusí být možné, můžete se zkusit připojit přes předávací server. Pokud navíc chcete při prvním pokusu použít předávací server, můžete k ID přidat příponu \"/r\", nebo v kartě posledních relací vybrat možnost \"Vždy se připojovat přes bránu\", pokud existuje."),
        ("Reconnect", "Znovu připojit"),
        ("Codec", "Kodek"),
        ("Resolution", "Rozlišení"),
        ("No transfers in progress", "Žádné probíhající přenosy"),
        ("Set one-time password length", "Nastavení délky jednorázového hesla"),
        ("RDP Settings", "Nastavení RDP"),
        ("Sort by", "Seřadit podle"),
        ("New Connection", "Nové připojení"),
        ("Restore", "Obnovit"),
        ("Minimize", "Minimalizovat"),
        ("Maximize", "Maximalizovat"),
        ("Your Device", "Vaše zařízení"),
        ("empty_recent_tip", "Zde se zobrazí nedávné relace."),
        ("empty_favorite_tip", "Zde se zobrazí oblíbení vrstevníci."),
        ("empty_lan_tip", "Zde se objeví objevení vrstevníci."),
        ("empty_address_book_tip", "Ve vašem adresáři nejsou aktuálně uvedeni žádní kolegové."),
        ("eg: admin", "např. admin"),
        ("Empty Username", "Prázdné uživatelské jméno"),
        ("Empty Password", "Prázdné heslo"),
        ("Me", "Já"),
        ("identical_file_tip", "Tento soubor je totožný se souborem partnera."),
        ("show_monitors_tip", "Zobrazit monitory na panelu nástrojů"),
        ("View Mode", "Režim zobrazení"),
        ("login_linux_tip", "Chcete-li povolit relaci plochy X, musíte se přihlásit ke vzdálenému účtu systému Linux."),
        ("verify_rustdesk_password_tip", "Ověření hesla HopToDesk"),
        ("remember_account_tip", "Zapamatovat si tento účet"),
        ("os_account_desk_tip", "Tento účet se používá k přihlášení do vzdáleného operačního systému a k povolení relace plochy v režimu headless."),
        ("OS Account", "Účet operačního systému"),
        ("another_user_login_title_tip", "Další uživatel je již přihlášen"),
        ("another_user_login_text_tip", "Odpojit"),
        ("xorg_not_found_title_tip", "Xorg nebyl nalezen"),
        ("xorg_not_found_text_tip", "Prosím, nainstalujte Xorg"),
        ("no_desktop_title_tip", "Není k dispozici žádná plocha"),
        ("no_desktop_text_tip", "Nainstalujte si prosím prostředí GNOME"),
        ("No need to elevate", "Není třeba navýšení"),
        ("System Sound", "Systémový zvuk"),
        ("Default", "Výchozí"),
        ("New RDP", "Nové RDP"),
        ("Fingerprint", "Otisk"),
        ("Copy Fingerprint", "Kopírovat otisk"),
        ("no fingerprints", "žádný otisk"),
        ("Select a peer", "Výběr protistrany"),
        ("Select peers", "Vybrat protistrany"),
        ("Plugins", "Pluginy"),
        ("Uninstall", "Odinstalovat"),
        ("Update", "Aktualizovat"),
        ("Enable", "Povolit"),
        ("Disable", "Zakázat"),
        ("Options", "Možnosti"),
        ("resolution_original_tip", "Původní rozlišení"),
        ("resolution_fit_local_tip", "Přizpůsobit místní rozlišení"),
        ("resolution_custom_tip", "Vlastní rozlišení"),
        ("Collapse toolbar", "Sbalit panel nástrojů"),
        ("Accept and Elevate", "Přijmout navýšení"),
        ("accept_and_elevate_btn_tooltip", "Přijměte připojení a zvyšte oprávnění UAC."),
        ("clipboard_wait_response_timeout_tip", "Vypršel čas čekání odpovědi na kopii."),
        ("Incoming connection", "Příchozí připojení"),
        ("Outgoing connection", "Odchozí připojení"),
        ("Exit", "Ukončit"),
        ("Open", "Otevřít"),
        ("logout_tip", "Opravdu se chcete odhlásit?"),
        ("Service", "Služba"),
        ("Start", "Spustit"),
        ("Stop", "Zastavit"),
        ("exceed_max_devices", "Dosáhli jste maximálního počtu spravovaných zařízení."),
        ("Sync with recent sessions", "Synchronizace s posledními relacemi"),
        ("Sort tags", "Seřadit štítky"),
        ("Open connection in new tab", "Otevřít připojení na nové kartě"),
        ("Move tab to new window", "Přesunout kartu do nového okna"),
        ("Can not be empty", "Nemůže být prázdné"),
        ("Already exists", "Již existuje"),
        ("Change Password", "Změnit heslo"),
        ("Refresh Password", "Obnovit heslo"),
        ("ID", "ID"),
        ("Grid View", "Mřížka"),
        ("List View", "Seznam"),
        ("Select", "Vybrat"),
        ("Toggle Tags", "Přepnout štítky"),
        ("pull_ab_failed_tip", "Nepodařilo se obnovit adresář"),
        ("push_ab_failed_tip", "Nepodařilo se synchronizovat adresář se serverem"),
        ("synced_peer_readded_tip", "Zařízení, která byla přítomna v posledních relacích, budou synchronizována zpět do adresáře."),
        ("Change Color", "Změna barvy"),
        ("Primary Color", "Základní barva"),
        ("HSV Color", "HSV barva"),
        ("Installation Successful!", "Instalace úspěšná!"),
        ("Installation failed!", "Instalace se nezdařila!"),
        ("Reverse mouse wheel", "Reverzní kolečko myši"),
        ("{} sessions", "{} sezení"),
        ("Don't show again", "Znovu se neukázat"),
        ("I Agree", "Souhlasím"),
        ("Decline", "Odmítnout"),
        ("Timeout in minutes", "Časový limit v minutách"),
        ("auto_disconnect_option_tip", "Automatické ukončení příchozích relací při nečinnosti uživatele"),
        ("Connection failed due to inactivity", "Připojení se nezdařilo z důvodu nečinnosti"),
        ("Check for software update on startup", "Kontrola aktualizace softwaru při spuštění"),
        ("pull_group_failed_tip", "Nepodařilo se obnovit skupinu"),
        ("Filter by intersection", "Filtrovat podle průsečíku"),
        ("Remove wallpaper during incoming sessions", "Odstranit tapetu během příchozích relací"),
        ("Test", "Test"),
        ("display_is_plugged_out_msg", "Obrazovka je odpojena, přepněte na první obrazovku."),
        ("No displays", "Žádné obrazovky"),
        ("Open in new window", "Otevřít v novém okně"),
        ("Show displays as individual windows", "Zobrazit obrazovky jako jednotlivá okna"),
        ("Use all my displays for the remote session", "Použít všechny mé obrazovky pro vzdálenou relaci"),
        ("selinux_tip", "Na vašem zařízení je povolen SELinux, což může bránit správnému běhu HopToDesk jako řízené strany."),
        ("Change view", "Změnit pohled"),
        ("Big tiles", "Velké dlaždice"),
        ("Small tiles", "Malé dlaždice"),
        ("List", "Seznam"),
        ("Virtual display", "Virtuální obrazovka"),
        ("Plug out all", "Odpojit všechny"),
        ("True color (4:4:4)", "Skutečné barvy (4:4:4)"),
        ("Enable blocking user input", "Povolit blokování uživatelského vstupu"),
        ("id_input_tip", "Můžete zadat ID, přímou IP adresu nebo doménu s portem (<doména>:<port>).\nPokud chcete přistupovat k zařízení na jiném serveru, připojte adresu serveru (<id>@<adresa_serveru>?key=<hodnota_klíče>), například,\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nPokud chcete přistupovat k zařízení na veřejném serveru, zadejte \"<id>@public\", klíč není pro veřejný server potřeba."),
        ("privacy_mode_impl_mag_tip", "Režim 1"),
        ("privacy_mode_impl_virtual_display_tip", "Režim 2"),
        ("Privacy Mode started.", "Vstup do režimu soukromí"),
        ("Privacy Mode turned off.", "Ukončit režim soukromí"),
        ("idd_not_support_under_win10_2004_tip", "Ovladač nepřímého zobrazení není podporován. Je vyžadován systém Windows 10, verze 2004 nebo novější."),
        ("switch_display_elevated_connections_tip", "Přepnutí na jinou než primární obrazovku není podporováno ve zvýšeném režimu, pokud existuje více připojení. Pokud chcete ovládat více obrazovek, zkuste to po instalaci znovu."),
        ("input_source_1_tip", "Vstupní zdroj 1"),
        ("input_source_2_tip", "Vstupní zdroj 2"),
        ("Swap control-command key", "Prohození klávesy control-command"),
        ("swap-left-right-mouse", "Prohodit levé a pravé tlačítko myši"),
        ("2FA code", "2FA kód"),
        ("More", "Více"),
        ("enable-2fa-title", "Povolit Dvoufaktorové Ověřování"),
        ("enable-2fa-desc", "Otevřete aplikaci pro ověřování, jako je Google Authenticator, a naskenujte níže uvedený QR kód."),
        ("wrong-2fa-code", "Neplatný kód: zkontrolujte prosím čas vašeho ověřovacího zařízení."),
        ("enter-2fa-title", "Nastavení Dvoufaktorové Autentizace"),
        ("Email verification code must be 6 characters.", "E-mailový ověřovací kód musí mít 6 znaků."),
        ("2FA code must be 6 digits.", "Kód 2FA musí mít 6 číslic."),
        ("Multiple Windows sessions found", "Bylo nalezeno více relací Windows"),
        ("Please select the session you want to connect to", "Vyberte relaci, ke které se chcete připojit"),
        ("outgoing_only_desk_tip", "Toto je přizpůsobená edice.\nMůžete se připojit k jiným zařízením, ale jiná zařízení se k vašemu zařízení připojit nemohou."),
        ("preset_password_warning", "Tato upravená edice je dodávána s přednastaveným heslem. Každý, kdo zná toto heslo, může získat plnou kontrolu nad vaším zařízením. Pokud jste to nečekali, okamžitě software odinstalujte."),
        ("Security Alert", "Bezbečnostní Výstraha"),
        ("My address book", "Můj adresář"),
        ("Personal", "Osobní"),
        ("Owner", "Majitel"),
        ("Set shared password", "Nastavit sdílené heslo"),
        ("Exist in", "Existuje v"),
        ("Read-only", "Pouze ke čtení"),
        ("Read/Write", "Režim čtení/zápisu"),
        ("Full Control", "Plná kontrola"),
        ("share_warning_tip", "Výše uvedená pole jsou sdílená a viditelná pro ostatní."),
        ("Everyone", "Každý"),
        ("ab_web_console_tip", "Více na webové konzoli"),
        ("allow-only-conn-window-open-tip", "Povolit připojení pouze v případě, že je otevřené okno HopToDesk"),
        ("no_need_privacy_mode_no_physical_displays_tip", "Žádné fyzické displeje, není třeba používat režim soukromí."),
        ("Follow remote cursor", "Sledovat dálkový kurzor"),
        ("Follow remote window focus", "Sledovat zaměření vzdáleného okna"),
        ("default_proxy_tip", "Výchozí protokol a port jsou SOCKS5 a 1080"),
        ("no_audio_input_device_tip", "Nebylo nalezeno žádné vstupní zvukové zařízení."),
        ("Incoming", "Příchozí"),
        ("Outgoing", "Odchozí"),
        ("Clear Wayland screen selection", "Vymazat výběr obrazovky Wayland"),
        ("clear_Wayland_screen_selection_tip", "Po vymazání výběru obrazovky můžete znovu vybrat obrazovku, kterou chcete sdílet."),
        ("confirm_clear_Wayland_screen_selection_tip", "Opravdu chcete vymazat výběr obrazovky Wayland?"),
        ("android_new_voice_call_tip", "Byl přijat nový požadavek na hlasové volání. Pokud hovor přijmete, přepne se zvuk na hlasovou komunikaci."),
        ("texture_render_tip", "Použít vykreslování textur, aby byly obrázky hladší."),
        ("Use texture rendering", "Použít vykreslování textur"),
        ("Floating window", "Plovoucí okno"),
        ("floating_window_tip", "Pomáhá udržovat službu HopToDesk na pozadí"),
        ("Keep screen on", "Ponechat obrazovku zapnutou"),
        ("Never", "Nikdy"),
        ("During controlled", "Během řízeného"),
        ("During service is on", "Během služby je v provozu"),
        ("Capture screen using DirectX", "Snímání obrazovky pomocí DirectX"),
        ("Back", "Zpět"),
        ("Apps", "Aplikace"),
        ("Volume up", "Zvýšit hlasitost"),
        ("Volume down", "Snížit hlasitost"),
        ("Power", "Napájení"),
        ("Telegram bot", "Telegram bot"),
        ("enable-bot-tip", "Pokud tuto funkci povolíte, můžete od svého bota obdržet kód 2FA. Může také fungovat jako oznámení o připojení."),
        ("enable-bot-desc", "1, Otevřete chat s @BotFather.\n2, Pošlete příkaz \"/newbot\". Po dokončení tohoto kroku obdržíte token.\n3, Spusťte chat s nově vytvořeným botem. Pro jeho aktivaci odešlete zprávu začínající lomítkem vpřed (\"/\"), například \"/hello\".\n"),
        ("cancel-2fa-confirm-tip", "Jste si jisti, že chcete zrušit 2FA?"),
        ("cancel-bot-confirm-tip", "Jste si jisti, že chcete zrušit bota Telegramu?"),
		("Enable Wake On LAN", "Zapnout Wake On LAN"),
        ("Enable 2FA", "Povolit 2FA"),
        ("You will need to confirm the 2FA on the secondary device with you when trying to connect to this desktop.", "Při pokusu o připojení k této ploše budete muset potvrdit 2FA na sekundárním zařízení."),		
		("Choose Network", "Vyberte Síť"),
		("Your Security Code", "Váš bezpečnostní Kód"),
		("ID (Click to Copy)", "ID (kliknutím zkopírujete)"),		
		("Password (Click to Copy)", "Heslo (kliknutím zkopírujete)"),
		("Unattended Access", "Bezobslužný Přístup"),				
    ].iter().cloned().collect();
}
