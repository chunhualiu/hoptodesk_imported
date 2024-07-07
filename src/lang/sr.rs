lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Vaša radna površina"),
        ("desk_tip", "Vašoj radnoj površini se može pristupiti ovim ID i lozinkom."),
        ("Password", "Lozinka"),
        ("Ready", "Spremno"),
        ("Established", "Uspostavljeno"),
        ("connecting_status", "Spajanje na HopToDesk mrežu..."),
        ("Enable Service", "Dozvoli servis"),
        ("Start Service", "Pokreni servis"),
        ("Service is running", "Servis je pokrenut"),
        ("Service is not running", "Servis nije pokrenut"),
        ("not_ready_status", "Nije spremno. Proverite konekciju."),
        ("Control Remote Desktop", "Upravljanje udaljenom radnom površinom"),
        ("Transfer File", "Prenos fajla"),
        ("Connect", "Spajanje"),
        ("Recent Sessions", "Poslednje sesije"),
        ("Address Book", "Adresar"),
        ("Confirmation", "Potvrda"),
        ("TCP Tunneling", "TCP tunel"),
        ("Remove", "Ukloni"),
        ("Refresh random password", "Osveži slučajnu lozinku"),
        ("Set your own password", "Postavi lozinku"),
        ("Enable Keyboard/Mouse", "Dozvoli tastaturu/miša"),
        ("Enable Clipboard", "Dozvoli clipboard"),
        ("Enable File Transfer", "Dozvoli prenos fajlova"),
        ("Enable TCP Tunneling", "Dozvoli TCP tunel"),
        ("IP Whitelisting", "IP pouzdana lista"),
        ("ID/Relay Server", "ID/Posredni server"),
        ("Import Server Config", "Import server konfiguracije"),
        ("Export Server Config", "Eksport server konfiguracije"),
        ("Import server configuration successfully", "Import server konfiguracije uspešan"),
        ("Export server configuration successfully", "Eksport server konfiguracije uspešan"),
        ("Invalid server configuration", "Pogrešna konfiguracija servera"),
        ("Clipboard is empty", "Clipboard je prazan"),
        ("Stop service", "Stopiraj servis"),
        ("Change ID", "Promeni ID"),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Dozvoljeni su samo a-z, A-Z, 0-9 i _ (donja crta) znakovi. Prvi znak mora biti slovo a-z, A-Z. Dužina je od 6 do 16."),
        ("Website", "Web sajt"),
        ("About", "O programu"),
        ("Privacy Statement", ""),
        ("Mute", "Utišaj"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Audio ulaz"),
        ("Enhancements", "Proširenja"),
        ("Hardware Codec", "Hardverski kodek"),
        ("Adaptive Bitrate", "Prilagodljiva gustina podataka"),
        ("ID Server", "ID server"),
        ("Relay Server", "Posredni server"),
        ("API Server", "API server"),
        ("invalid_http", "mora početi sa http:// ili https://"),
        ("Invalid IP", "Nevažeća IP"),
        ("Invalid format", "Pogrešan format"),
        ("server_not_support", "Server još uvek ne podržava"),
        ("Not available", "Nije dostupno"),
        ("Too frequent", "Previše često"),
        ("Cancel", "Otkaži"),
        ("Skip", "Preskoči"),
        ("Close", "Zatvori"),
        ("Retry", "Ponovi"),
        ("OK", "Ok"),
        ("Password Required", "Potrebna lozinka"),
        ("Please enter your password", "Molimo unesite svoju lozinku"),
        ("Remember password", "Zapamti lozinku"),
        ("Wrong Password", "Pogrešna lozinka"),
        ("Do you want to enter again?", "Želite li da unesete ponovo?"),
        ("Connection Error", "Greška u konekciji"),
        ("Error", "Greška"),
        ("Connection lost", "Prekinuto sa druge strane"),
        ("Connecting...", "Povezivanje..."),
        ("Connection in progress. Please wait.", "Povezivanje u toku. Molimo sačekajte."),
        ("Please try 1 minute later", "Pokušajte minut kasnije"),
        ("Login Error", "Greška u prijavljivanju"),
        ("Successful", "Uspešno"),
        ("Connected, waiting for image...", "Spojeno, sačekajte sliku..."),
        ("Name", "Ime"),
        ("Type", "Tip"),
        ("Modified", "Izmenjeno"),
        ("Size", "Veličina"),
        ("Show Hidden Files", "Prikaži skrivene datoteke"),
        ("Receive", "Prijem"),
        ("Send", "Slanje"),
        ("Refresh File", "Osveži datoteku"),
        ("Local", "Lokalno"),
        ("Remote", "Udaljeno"),
        ("Remote Computer", "Udaljeni računar"),
        ("Local Computer", "Lokalni računar"),
        ("Confirm Delete", "Potvrdite brisanje"),
        ("Delete", "Brisanje"),
        ("Properties", "Osobine"),
        ("Multi Select", "Višestruko selektovanje"),
        ("Select All", "Selektuj sve"),
        ("Unselect All", "Deselektuj sve"),
        ("Empty Directory", "Prazan direktorijum"),
        ("Not an empty directory", "Nije prazan direktorijum"),
        ("Are you sure you want to delete this file?", "Da li ste sigurni da želite da obrišete ovu datoteku?"),
        ("Are you sure you want to delete this empty directory?", "Da li ste sigurni da želite da obrišete ovaj prazan direktorijum?"),
        ("Are you sure you want to delete the file of this directory?", "Da li ste sigurni da želite da obrišete datoteku ovog direktorijuma?"),
        ("Do this for all conflicts", "Uradi ovo za sve konflikte"),
        ("This is irreversible!", "Ovo je nepovratno"),
        ("Deleting", "Brisanje"),
        ("files", "datoteke"),
        ("Waiting", "Čekanje"),
        ("Finished", "Završeno"),
        ("Speed", "Brzina"),
        ("Custom Image Quality", "Korisnički kvalitet slike"),
        ("Privacy mode", "Mod privatnosti"),
        ("Block user input", "Blokiraj korisnikov unos"),
        ("Unblock user input", "Odblokiraj korisnikov unos"),
        ("Adjust Window", "Podesi prozor"),
        ("Original", "Original"),
        ("Shrink", "Skupi"),
        ("Stretch", "Raširi"),
        ("Scrollbar", "Skrol linija"),
        ("ScrollAuto", "Auto skrol"),
        ("Good image quality", "Dobar kvalitet slike"),
        ("Balanced", "Balansirano"),
        ("Optimize reaction time", "Optimizuj vreme reakcije"),
        ("Custom", "Korisnički"),
        ("Show remote cursor", "Prikaži udaljeni kursor"),
        ("Show quality monitor", "Prikaži monitor kvaliteta"),
        ("Disable clipboard", "Zabrani clipboard"),
        ("Lock after session end", "Zaključaj po završetku sesije"),
        ("Insert", "Umetni"),
        ("Insert Lock", "Zaključaj umetanje"),
        ("Refresh", "Osveži"),
        ("ID does not exist", "ID ne postoji"),
        ("Please try later", "Molimo pokušajte kasnije"),
        ("Remote desktop is offline", "Udaljeni ekran je isključen"),
        ("Key mismatch", "Pogrešan ključ"),
        ("Timeout", "Isteklo vreme"),
        ("Failed to connect to relay server", "Greška u spajanju na posredni server"),
        ("Failed to connect to signal server", "Greška u spajanju preko servera za povezivanje"),
        ("Failed to connect via relay server", "Greška u spajanju preko posrednog servera"),
        ("Failed to make direct connection to remote desktop", "Greška u direktnom spajanju na udaljenu radnu površinu"),
        ("Set Password", "Postavi lozinku"),
        ("OS Password", "OS lozinka"),
        ("install_tip", "Za najbolju izvedbu, kompletnu instalaciju."),
        ("Click to upgrade", "Klik za nadogradnju"),
        ("Click to download", "Klik za preuzimanje"),
        ("Click to update", "Klik za ažuriranje"),
        ("Configure", "Konfigurisanje"),
        ("config_acc", "Da biste daljinski kontrolisali radnu površinu, HopToDesk-u treba da dodelite \"Accessibility\" prava."),
        ("config_screen", "Da biste daljinski pristupili radnoj površini, HopToDesk-u treba da dodelite \"Screen Recording\" prava."),
        ("Installing ...", "Instaliranje..."),
        ("Install", "Instaliraj"),
        ("Installation", "Instalacija"),
        ("Installation Path", "Putanja za instalaciju"),
        ("Create start menu shortcuts", "Kreiraj prečice u meniju"),
        ("Create desktop icon", "Kreiraj ikonicu na radnoj površini"),
        ("agreement_tip", "Pokretanjem instalacije prihvatate ugovor o licenciranju."),
        ("Accept and Install", "Prihvati i instaliraj"),
        ("End-user license agreement", "Ugovor sa krajnjim korisnikom"),
        ("Generating ...", "Generisanje..."),
        ("Your installation is lower version.", "Vaša instalacija je niže verzije"),
        ("not_close_tcp_tip", "Ne zatvarajte ovaj prozor dok koristite tunel"),
        ("Listening ...", "Na slušanju..."),
        ("Remote Host", "Adresa udaljenog uređaja"),
        ("Remote Port", "Udaljeni port"),
        ("Action", "Akcija"),
        ("Add", "Dodaj"),
        ("Local Port", "Lokalni port"),
        ("Local Address", "Lokalna adresa"),
        ("Change Local Port", "Promeni lokalni port"),
        ("setup_server_tip", "Za brže spajanje, molimo da koristite svoj server"),
        ("Too short, at least 6 characters.", "Prekratko, najmanje 6 znakova."),
        ("The confirmation is not identical.", "Potvrda nije identična"),
        ("Permissions", "Dozvole"),
        ("Accept", "Prihvati"),
        ("Dismiss", "Odbaci"),
        ("Disconnect", "Raskini konekciju"),
        ("Enable file copy and paste", "Dozvoli kopiranje i lepljenje fajlova"),
        ("Connected", "Spojeno"),
        ("Direct and encrypted connection", "Direktna i kriptovana konekcija"),
        ("Relayed and encrypted connection", "Posredna i kriptovana konekcija"),
        ("Direct and unencrypted connection", "Direktna i nekriptovana konekcija"),
        ("Relayed and unencrypted connection", "Posredna i nekriptovana konekcija"),
        ("Enter Remote ID", "Unesite ID udaljenog uređaja"),
        ("Enter your password", "Unesite svoju lozinku"),
        ("Logging in...", "Prijava..."),
        ("Enable RDP session sharing", "Dozvoli deljenje RDP sesije"),
        ("Auto Login", "Auto prijavljivanje (Važeće samo ako ste postavili \"Lock after session end\")"),
        ("Enable Direct IP Access", "Dozvoli direktan pristup preko IP"),
        ("Rename", "Preimenuj"),
        ("Space", "Prazno"),
        ("Create Desktop Shortcut", "Kreiraj prečicu na radnoj površini"),
        ("Change Path", "Promeni putanju"),
        ("Create Folder", "Kreiraj direktorijum"),
        ("Please enter the folder name", "Unesite ime direktorijuma"),
        ("Disable Wayland", "Popravi ga"),
        ("Warning", "Upozorenje"),
        ("Login screen using Wayland is not supported.", "Ekran za prijavu koji koristi Wayland nije podržan"),
        ("Reboot required", "Potreban je restart"),
        ("Unsupported display server ", "Nepodržan server za prikaz"),
        ("x11 expected", "x11 očekivan"),
        ("Port", "Port"),
        ("Settings", "Postavke"),
        ("Username", "Korisničko ime"),
        ("Invalid port", "Pogrešan port"),
        ("The remote partner has closed the session.", "Udaljeni partner je zatvorio sesiju."),
        ("Enable remote configuration modification", "Dozvoli modifikaciju udaljene konfiguracije"),
        ("Run without install", "Pokreni bez instalacije"),
        ("Always connected via relay", "Uvek spojne preko posrednika"),
        ("Always connect via relay", "Uvek se spoj preko posrednika"),
        ("whitelist_tip", "Samo dozvoljene IP mi mogu pristupiti"),
        ("Login", "Prijava"),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", "Odjava"),
        ("Tags", "Oznake"),
        ("Search ID", "Traži ID"),
        ("whitelist_sep", "Odvojeno zarezima, tačka zarezima, praznim mestima ili novim redovima"),
        ("Add ID", "Dodaj ID"),
        ("Add Tag", "Dodaj oznaku"),
        ("Unselect all tags", "Odselektuj sve oznake"),
        ("Network error", "Greška na mreži"),
        ("Username missed", "Korisničko ime promašeno"),
        ("Password missed", "Lozinka promašena"),
        ("Wrong credentials", "Pogrešno korisničko ime ili lozinka"),
        ("Edit Tag", "Izmeni oznaku"),
        ("Forget Password", "Zaboravi lozinku"),
        ("Favorites", "Favoriti"),
        ("Add to Favorites", "Dodaj u favorite"),
        ("Remove from Favorites", "Izbaci iz favorita"),
        ("Empty", "Prazno"),
        ("Invalid folder name", "Pogrešno ime direktorijuma"),
        ("SOCKS5 Proxy", "SOCKS5 proksi"),
        ("Hostname", "Ime uređaja"),
        ("Discovered", "Otkriveno"),
        ("install_daemon_tip", "Za pokretanje pri startu sistema, treba da instalirate sistemski servis."),
        ("Remote ID", "Udaljeni ID"),
        ("Paste", "Nalepi"),
        ("Paste here?", "Nalepi ovde?"),
        ("Are you sure to close the connection?", "Da li ste sigurni da zatvarate konekciju?"),
        ("Download new version", "Preuzmi novu verziju"),
        ("Touch mode", "Mod na dodir"),
        ("Mouse mode", "Miš mod"),
        ("One-Finger Tap", "Pritisak jednim prstom"),
        ("Left Mouse", "Levi miš"),
        ("One-Long Tap", "Dugi pritisak"),
        ("Two-Finger Tap", "Pritisak sa dva prsta"),
        ("Right Mouse", "Desni miš"),
        ("One-Finger Move", "Pomeranje jednim prstom"),
        ("Double Tap & Move", "Dupli pritisak i pomeranje"),
        ("Mouse Drag", "Prevlačenje mišem"),
        ("Three-Finger vertically", "Sa tri prsta vertikalno"),
        ("Mouse Wheel", "Točkić miša"),
        ("Two-Finger Move", "Pomeranje sa dva prsta"),
        ("Canvas Move", "Pomeranje pozadine"),
        ("Pinch to Zoom", "Stisnite za zumiranje"),
        ("Canvas Zoom", "Zumiranje pozadine"),
        ("Reset canvas", "Resetuj pozadinu"),
        ("No permission of file transfer", "Nemate pravo prenosa datoteka"),
        ("Note", "Primedba"),
        ("Connection", "Konekcija"),
        ("Share Screen", "Podeli ekran"),
        ("Chat", "Dopisivanje"),
        ("Total", "Ukupno"),
        ("items", "stavki"),
        ("Selected", "Izabrano"),
        ("Screen Capture", "Snimanje ekrana"),
        ("Input Control", "Kontrola unosa"),
        ("Audio Capture", "Snimanje zvuka"),
        ("File Connection", "Spajanje preko datoteke"),
        ("Screen Connection", "Podeli konekciju"),
        ("Do you accept?", "Prihvatate?"),
        ("Open System Setting", "Postavke otvorenog sistema"),
        ("How to get Android input permission?", "Kako dobiti pristup za Android unos?"),
        ("android_input_permission_tip1", "Da bi daljinski uređaj kontrolisao vaš Android uređaj preko miša ili na dodir, treba da dozvolite HopToDesk-u da koristi \"Accessibility\" servis."),
        ("android_input_permission_tip2", "Molimo pređite na sledeću stranicu sistemskih podešavanja, pronađite i unesite [Installed Services], uključite [HopToDesk Input] servis."),
        ("android_new_connection_tip", "Primljen je novi zahtev za upravljanje, koji želi da upravlja ovim vašim uređajem."),
        ("android_service_will_start_tip", "Uključenje \"Screen Capture\" automatski će pokrenuti servis, dozvoljavajući drugim uređajima da zahtevaju spajanje na vaš uređaj."),
        ("android_stop_service_tip", "Zatvaranje servisa automatski će zatvoriti sve uspostavljene konekcije."),
        ("android_version_audio_tip", "Tekuća Android verzija ne podržava audio snimanje, molimo nadogradite na Android 10 ili veći."),
        ("android_start_service_tip", "Kliknite [Start Screen Share] ili OPEN [Screen Capture] dozvolu da pokrenete servis deljenja ekrana."),
        ("Account", "Nalog"),
        ("Overwrite", "Prepiši preko"),
        ("This file exists, skip or overwrite this file?", "Ova datoteka postoji, preskoči ili prepiši preko?"),
        ("Quit", "Izlaz"),
        ("server_not_support", "Server još uvek ne podržava"),
        ("Help", "Pomoć"),
        ("Failed", "Greška"),
        ("Succeeded", "Uspešno"),
        ("Someone turned on privacy mode, exiting.", "Neko je uključio mod privatnosti, izlaz."),
        ("Unsupported", "Nepodržano"),
        ("Peer denied", "Klijent zabranjen"),
        ("Please install plugins", "Molimo instalirajte dodatke"),
        ("The peer has exited from Privacy Mode.", "Klijent izašao"),
        ("Failed to turn off", "Greška kod isključenja"),
        ("Turned off", "Isključeno"),
        ("Language", "Jezik"),
        ("Keep HopToDesk background service", "Zadrži HopToDesk kao pozadinski servis"),
        ("Ignore Battery Optimizations", "Zanemari optimizacije baterije"),
        ("android_open_battery_optimizations_tip", "Ako želite da onemogućite ovu funkciju, molimo idite na sledeću stranicu za podešavanje HopToDesk aplikacije, pronađite i uđite u [Battery], isključite [Unrestricted]"),
        ("Connection not allowed", "Konekcija nije dozvoljena"),
        ("Legacy mode", "Zastareli mod"),
        ("Map mode", "Mod mapiranja"),
        ("Translate mode", "Mod prevođenja"),
        ("Use permanent password", "Koristi trajnu lozinku"),
        ("Use both passwords", "Koristi obe lozinke"),
        ("Set permanent password", "Postavi trajnu lozinku"),
        ("Enable Remote Restart", "Omogući daljinsko restartovanje"),
        ("Restart Remote Device", "Restartuj daljinski uređaj"),
        ("Are you sure you want to restart", "Da li ste sigurni da želite restart"),
        ("Restarting Remote Device", "Restartovanje daljinskog uređaja"),
        ("remote_restarting_tip", "Udaljeni uređaj se restartuje, molimo zatvorite ovu poruku i ponovo se kasnije povežite trajnom šifrom"),
        ("Are you sure to close the connection?", "Da li ste sigurni da želite da zatvorite konekciju?"),
        ("Copied", "Kopirano"),
        ("Exit Fullscreen", "Napusti mod celog ekrana"),
        ("Fullscreen", "Mod celog ekrana"),
        ("Mobile Actions", "Mobilne akcije"),
        ("Select Monitor", "Izbor monitora"),
        ("Control Actions", "Upravljačke akcije"),
        ("Display Settings", "Postavke prikaza"),
        ("Ratio", "Odnos"),
        ("Image Quality", "Kvalitet slike"),
        ("Scroll Style", "Stil skrolovanja"),
        ("Show Menubar", "Prikaži meni"),
        ("Hide Menubar", "Sakrij meni"),
        ("Direct Connection", "Direktna konekcija"),
        ("Relay Connection", "Posredna konekcija"),
        ("Secure Connection", "Bezbedna konekcija"),
        ("Insecure Connection", "Nebezbedna konekcija"),
        ("Scale original", "Skaliraj original"),
        ("Scale adaptive", "Adaptivno skaliranje"),
        ("General", "Uopšteno"),
        ("Security", "Bezbednost"),
        ("Theme", "Tema"),
        ("Dark Theme", "Tamna tema"),
        ("Light Theme", ""),
        ("Dark", "Tamno"),
        ("Light", "Svetlo"),
        ("Follow System", "Prema sistemu"),
        ("Enable hardware codec", "Omogući hardverski kodek"),
        ("Unlock Security Settings", "Otključaj postavke bezbednosti"),
        ("Enable Audio", "Dozvoli zvuk"),
        ("Unlock Network Settings", "Otključaj postavke mreže"),
        ("Server", "Server"),
        ("Direct IP Access", "Direktan IP pristup"),
        ("Proxy", "Proksi"),
        ("Apply", "Primeni"),
        ("Disconnect all devices?", "Otkači sve uređaju?"),
        ("Clear", "Obriši"),
        ("Audio Input Device", "Uređaj za ulaz zvuka"),
        ("Use IP Whitelisting", "Koristi listu pouzdanih IP"),
        ("Network", "Mreža"),
        ("Pin menubar", "Zakači meni"),
        ("Unpin menubar", "Otkači meni"),
        ("Recording", "Snimanje"),
        ("Directory", "Direktorijum"),
        ("Automatically record incoming sessions", "Automatski snimaj dolazne sesije"),
        ("Change", "Promeni"),
        ("Start session recording", "Započni snimanje sesije"),
        ("Stop session recording", "Zaustavi snimanje sesije"),
        ("Enable Recording Session", "Omogući snimanje sesije"),
        ("Enable LAN Discovery", "Omogući LAN otkrivanje"),
        ("Deny LAN Discovery", "Zabrani LAN otkrivanje"),
        ("Write a message", "Napiši poruku"),
        ("Prompt", "Prompt"),
        ("Please wait for confirmation of UAC...", "Molimo sačekajte UAC potvrdu..."),
        ("elevated_foreground_window_tip", "Tekući prozor udaljene radne površine zahteva veću privilegiju za rad, tako da trenutno nije moguće koristiti miša i tastaturu. Možete zahtevati od udaljenog korisnika da minimizira aktivni prozor, ili kliknuti na taster za podizanje privilegija u prozoru za rad sa konekcijom. Da biste prevazišli ovaj problem, preporučljivo je da instalirate softver na udaljeni uređaj."),
        ("Disconnected", "Odspojeno"),
        ("Other", "Ostalo"),
        ("Confirm before closing multiple tabs", "Potvrda pre zatvaranja više kartica"),
        ("Keyboard Settings", "Postavke tastature"),
        ("Full Access", "Pun pristup"),
        ("Screen Share", "Deljenje ekrana"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland zahteva Ubuntu 21.04 ili veću verziju"),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland zahteva veću verziju Linux distribucije. Molimo pokušajte X11 ili promenite OS."),
        ("JumpLink", "Vidi"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Molimo izaberite ekran koji će biti podeljen (Za rad na klijent strani)"),
        ("Show HopToDesk", "Prikazi HopToDesk"),
        ("This PC", "Ovaj PC"),
        ("or", "ili"),
        ("Continue with", "Nastavi sa"),
        ("Elevate", "Izdigni"),
        ("Zoom cursor", "Zumiraj kursor"),
        ("Accept sessions via password", "Prihvati sesije preko lozinke"),
        ("Accept sessions via click", "Prihvati sesije preko klika"),
        ("Accept sessions via both", "Prihvati sesije preko oboje"),
        ("Please wait for the remote side to accept your session request...", "Molimo sačekajte da udaljena strana prihvati vaš zahtev za sesijom..."),
        ("One-time Password", "Jednokratna lozinka"),
        ("Use one-time password", "Koristi jednokratnu lozinku"),
        ("One-time password length", "Dužina jednokratne lozinke"),
        ("Request access to your device", "Zahtev za pristup vašem uređaju"),
        ("Hide connection management window", "Sakrij prozor za uređivanje konekcije"),
        ("hide_cm_tip", "Skrivanje dozvoljeno samo prihvatanjem sesije preko lozinke i korišćenjem trajne lozinke"),
        ("wayland_experiment_tip", "Wayland eksperiment savet"),
        ("Right click to select tabs", "Desni klik za izbor kartica"),
        ("Skipped", ""),
        ("Add to Address Book", "Dodaj u adresar"),
        ("Group", "Grupa"),
        ("Search", "Pretraga"),
        ("Closed manually by web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", ""),
        ("Always use software rendering", ""),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", ""),
        ("Elevation Error", ""),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", ""),
        ("Request Elevation", ""),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
        ("Switch Sides", ""),
        ("Please confirm if you want to share your desktop?", ""),
        ("Display", ""),
        ("Default View Style", ""),
        ("Default Scroll Style", ""),
        ("Default Image Quality", ""),
        ("Default Codec", ""),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", ""),
        ("Voice call", ""),
        ("Text chat", ""),
        ("Stop voice call", ""),
        ("relay_hint_tip", ""),
        ("Reconnect", ""),
        ("Codec", ""),
        ("Resolution", ""),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("RDP Settings", ""),
        ("Sort by", ""),
        ("New Connection", ""),
        ("Restore", ""),
        ("Minimize", ""),
        ("Maximize", ""),
        ("Your Device", ""),
        ("empty_recent_tip", ""),
        ("empty_favorite_tip", ""),
        ("empty_lan_tip", ""),
        ("empty_address_book_tip", ""),
        ("eg: admin", ""),
        ("Empty Username", ""),
        ("Empty Password", ""),
        ("Me", ""),
        ("identical_file_tip", ""),
        ("show_monitors_tip", ""),
        ("View Mode", ""),
        ("login_linux_tip", ""),
        ("verify_rustdesk_password_tip", ""),
        ("remember_account_tip", ""),
        ("os_account_desk_tip", ""),
        ("OS Account", ""),
        ("another_user_login_title_tip", ""),
        ("another_user_login_text_tip", ""),
        ("xorg_not_found_title_tip", ""),
        ("xorg_not_found_text_tip", ""),
        ("no_desktop_title_tip", ""),
        ("no_desktop_text_tip", ""),
        ("No need to elevate", ""),
        ("System Sound", ""),
        ("Default", ""),
        ("New RDP", ""),
        ("Fingerprint", ""),
        ("Copy Fingerprint", ""),
        ("no fingerprints", ""),
        ("Select a peer", ""),
        ("Select peers", ""),
        ("Plugins", ""),
        ("Uninstall", ""),
        ("Update", ""),
        ("Enable", ""),
        ("Disable", ""),
        ("Options", ""),
        ("resolution_original_tip", ""),
        ("resolution_fit_local_tip", ""),
        ("resolution_custom_tip", ""),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", ""),
        ("accept_and_elevate_btn_tooltip", ""),
        ("clipboard_wait_response_timeout_tip", ""),
        ("Incoming connection", ""),
        ("Outgoing connection", ""),
        ("Exit", ""),
        ("Open", ""),
        ("logout_tip", ""),
        ("Service", ""),
        ("Start", ""),
        ("Stop", ""),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Open connection in new tab", ""),
        ("Move tab to new window", ""),
        ("Can not be empty", ""),
        ("Already exists", ""),
        ("Change Password", ""),
        ("Refresh Password", ""),
        ("ID", ""),
        ("Grid View", ""),
        ("List View", ""),
        ("Select", ""),
        ("Toggle Tags", ""),
        ("pull_ab_failed_tip", ""),
        ("push_ab_failed_tip", ""),
        ("synced_peer_readded_tip", ""),
        ("Change Color", ""),
        ("Primary Color", ""),
        ("HSV Color", ""),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", ""),
        ("Change view", ""),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", ""),
        ("Virtual display", ""),
        ("Plug out all", ""),
        ("True color (4:4:4)", ""),
        ("Enable blocking user input", ""),
        ("id_input_tip", ""),
        ("privacy_mode_impl_mag_tip", ""),
        ("privacy_mode_impl_virtual_display_tip", ""),
        ("Privacy Mode started.", ""),
        ("Privacy Mode turned off.", ""),
        ("idd_not_support_under_win10_2004_tip", "Ne podržava ispod Win10 2004"),
        ("switch_display_elevated_connections_tip", "Prebacite prikaz povišenih konekcija"),
        ("input_source_1_tip", "Izvor ulaza 1"),
        ("input_source_2_tip", "Izvor ulaza 2"),
        ("Swap control-command key", "Zamijenite kontrolnu-komandu tipku"),
        ("swap-left-right-mouse", "Zamijenite lijevu-desnu tipku miša"),
        ("2FA code", "2FA kod"),
        ("More", "Više"),
        ("enable-2fa-title", "Uključite 2FA"),
        ("enable-2fa-desc", "Opis uključivanja 2FA"),
        ("wrong-2fa-code", "Pogrešan 2FA kod"),
        ("enter-2fa-title", "Unesite 2FA kod"),
        ("Email verification code must be 6 characters.", "Kod za verifikaciju emaila mora imati 6 karaktera."),
        ("2FA code must be 6 digits.", "2FA kod mora imati 6 cifara."),
        ("Multiple Windows sessions found", "Pronađeno je više Windows sesija"),
        ("Please select the session you want to connect to", "Molimo vas izaberite sesiju na koju se želite povezati"),
        ("powered_by_me", "pokreće ga ja"),
        ("outgoing_only_desk_tip", "Samo izlazni stol"),
        ("preset_password_warning", "Upozorenje o unaprijed postavljenoj lozinci"),
        ("Security Alert", "Sigurnosno Upozorenje"),
        ("My address book", "Moj adresar"),
        ("Personal", "Lično"),
        ("Owner", "Vlasnik"),
        ("Set shared password", "Postavite zajedničku lozinku"),
        ("Exist in", "Postoji u"),
        ("Read-only", "Samo za čitanje"),
        ("Read/Write", "Čitanje/Pisanje"),
        ("Full Control", "Puna Kontrola"),
        ("share_warning_tip", "Upozorenje o dijeljenju"),
        ("Everyone", "Svi"),
        ("ab_web_console_tip", "web konzola"),
        ("allow-only-conn-window-open-tip", "Dozvolite samo otvaranje prozora za povezivanje"),
        ("no_need_privacy_mode_no_physical_displays_tip", "Nije potreban privatni način rada bez fizičkih prikaza"),
        ("Follow remote cursor", "Pratite udaljeni kursor"),
        ("Follow remote window focus", "Pratite fokus udaljenog prozora"),
        ("default_proxy_tip", "podrazumijevani proxy"),
        ("no_audio_input_device_tip", "nema uređaja za unos zvuka"),
        ("Incoming", "Dolazno"),
        ("Outgoing", "Odlazno"),
        ("Clear Wayland screen selection", "Očistite izbor ekrana Wayland"),
        ("clear_Wayland_screen_selection_tip", "Savjet za čišćenje izbora ekrana Wayland"),
        ("confirm_clear_Wayland_screen_selection_tip", "Potvrdite čišćenje izbora ekrana Wayland"),
        ("android_new_voice_call_tip", "novi glasovni poziv za Android"),
        ("texture_render_tip", "teksturni prikaz"),
        ("Use texture rendering", "Koristite teksturni prikaz"),
        ("Floating window", "Plutajući prozor"),
        ("floating_window_tip", "Savjet za plutajući prozor"),
        ("Keep screen on", "Ostavite ekran uključen"),
        ("Never", "Nikad"),
        ("During controlled", "Tokom kontrole"),
        ("During service is on", "Tokom usluge"),
        ("Capture screen using DirectX", "Zabilježite ekran koristeći DirectX"),
        ("Your Security Code", "Vaš Sigurnosni Kod"),
        ("Enable 2FA", "Uključite 2FA"),
        ("Enable Wake On LAN", "Aktivirati buđenje na LAN"),
        ("You will need to confirm the 2FA on the secondary device with you when trying to connect to this desktop.", "Moraćete da potvrdite 2FA na sekundarnom uređaju sa vama kada pokušate da se povežete sa ovim stolom."),
        ("Choose Network", "Izaberi Mrežu"),
        ("HopToDesk Network (Default)", "HopToDesk Mreža (predeterminado)"),
        ("Custom Network Settings", "Podešavanje Mreže"),
        ("Custom Network Error", "Greška u Vlastitoj Mreži"),
        ("The custom network URL provided was not valid, please try again.", "Vlastiti URL nije valjan, molim vas pokušajte ponovo."),
        ("Your Security Code", "Vaš Sigurnosni Kod"),
        ("ID (Click to Copy)", "ID (Kliknite u kopiju)"),
        ("Password (Click to Copy)", "Lozinka (Kliknite u kopiju)"),
        ("Unattended Access", "Pristup bez Nadzora")		
        ].iter().cloned().collect();
}
