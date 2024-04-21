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
        ("connecting_status_short", "Spajanje..."),
        ("Enable service", "Dopusti servis"),
        ("Start service", "Pokreni servis"),
        ("Service is running", "Servis je pokrenut"),
        ("Service is not running", "Servis nije pokrenut"),
        ("not_ready_status", "Nije spremno. Provjerite vezu."),
		("not_ready_status_short", "Nije spremno"),        
        ("Control Remote Desktop", "Upravljanje udaljenom radnom površinom"),
        ("Transfer file", "Prijenos datoteke"),
        ("Connect", "Spajanje"),
        ("Recent sessions", "Nedavne sesije"),
        ("Address book", "Adresar"),
        ("Confirmation", "Potvrda"),
        ("TCP tunneling", "TCP tunel"),
        ("Remove", "Ukloni"),
        ("Refresh random password", "Osvježi slučajnu lozinku"),
        ("Set your own password", "Postavi lozinku"),
        ("Enable keyboard/mouse", "Dopusti tipkovnicu/miša"),
        ("Enable clipboard", "Dopusti međuspremnik"),
        ("Enable file transfer", "Dopusti prijenos datoteka"),
        ("Enable TCP tunneling", "Dopusti TCP tunel"),
        ("IP Whitelisting", "IP pouzdana lista"),
        ("ID/Relay Server", "ID/Posredni poslužitelj"),
        ("Import server config", "Uvoz konfiguracije poslužitelja"),
        ("Export Server Config", "Izvoz konfiguracije poslužitelja"),
        ("Import server configuration successfully", "Uvoz konfiguracije poslužitelja uspješan"),
        ("Export server configuration successfully", "Izvoz konfiguracije poslužitelja uspješan"),
        ("Invalid server configuration", "Pogrešna konfiguracija poslužitelja"),
        ("Clipboard is empty", "Međuspremnik je prazan"),
        ("Stop service", "Zaustavi servis"),
        ("Change ID", "Promijeni ID"),
        ("Your new ID", "Vaš novi ID"),
        ("length %min% to %max%", "duljina %min% do %max%"),
        ("starts with a letter", "Počinje slovom"),
        ("allowed characters", "Dopušteni znakovi"),
        ("id_change_tip", "Dopušteni su samo a-z, A-Z, 0-9 i _ (donja crta) znakovi. Prvi znak mora biti slovo a-z, A-Z. Duljina je od 6 do 16."),
        ("Website", "Web stranica"),
        ("About", "O programu"),
        ("Slogan_tip", "Stvoren srcem u ovom kaotičnom svijetu!"),
        ("Privacy Statement", "Izjava o privatnosti"),
        ("Mute", "Utišaj"),
        ("Build Date", "Datum izrade"),
        ("Version", "Verzija"),
        ("Home", "Početno"),
        ("Audio Input", "Audio ulaz"),
        ("Enhancements", "Proširenja"),
        ("Hardware Codec", "Hardverski kodek"),
        ("Adaptive bitrate", "Prilagodljiva gustoća podataka"),
        ("ID Server", "ID poslužitelja"),
        ("Relay Server", "Posredni poslužitelj"),
        ("API Server", "API poslužitelj"),
        ("invalid_http", "Treba početi sa http:// ili https://"),
        ("Invalid IP", "Nevažeća IP"),
        ("Invalid format", "Pogrešan format"),
        ("server_not_support", "Poslužitelj još uvijek ne podržava"),
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
        ("Do you want to enter again?", "Želite li ponovo unijeti lozinku?"),
        ("Connection Error", "Greška u spajanju"),
        ("Error", "Greška"),
        ("Connection lost", "Prekinuto sa druge strane"),
        ("Connecting...", "Povezivanje..."),
        ("Connection in progress. Please wait.", "Povezivanje u tijeku. Molimo pričekajte."),
        ("Please try 1 minute later", "Pokušajte minutu kasnije"),
        ("Login Error", "Greška kod prijave"),
        ("Successful", "Uspješno"),
        ("Connected, waiting for image...", "Spojeno, pričekajte sliku..."),
        ("Name", "Ime"),
        ("Type", "Vrsta"),
        ("Modified", "Izmijenjeno"),
        ("Size", "Veličina"),
        ("Show Hidden Files", "Prikaži skrivene datoteke"),
        ("Receive", "Prijem"),
        ("Send", "Slanje"),
        ("Refresh File", "Osvježi datoteku"),
        ("Local", "Lokalno"),
        ("Remote", "Udaljeno"),
        ("Remote Computer", "Udaljeno računalo"),
        ("Local Computer", "Lokalno računalo"),
        ("Confirm Delete", "Potvrdite brisanje"),
        ("Delete", "Brisanje"),
        ("Properties", "Svojstva"),
        ("Multi Select", "Višestruki odabir"),
        ("Select All", "Odaberi sve"),
        ("Unselect All", "Poništi odabir"),
        ("Empty Directory", "Prazna mapa"),
        ("Not an empty directory", "Nije prazna mapa"),
        ("Are you sure you want to delete this file?", "Jeste sigurni da želite obrisati ovu datoteku?"),
        ("Are you sure you want to delete this empty directory?", "Jeste sigurni da želite obrisati ovu praznu mapu?"),
        ("Are you sure you want to delete the file of this directory?", "Jeste sigurni da želite obrisati datoteku u ovoj mapi?"),
        ("Do this for all conflicts", "Učinite to za sve sukobe"),
        ("This is irreversible!", "Ovo je nepovratno"),
        ("Deleting", "Brisanje"),
        ("files", "datoteke"),
        ("Waiting", "Čekanje"),
        ("Finished", "Završeno"),
        ("Speed", "Brzina"),
        ("Custom Image Quality", "Korisnička kvaliteta slike"),
        ("Privacy mode", "Način privatnosti"),
        ("Block user input", "Blokiraj korisnikov unos"),
        ("Unblock user input", "Odblokiraj korisnikov unos"),
        ("Adjust Window", "Podesi prozor"),
        ("Original", "Izvornik"),
        ("Shrink", "Skupi"),
        ("Stretch", "Raširi"),
        ("Scrollbar", "Linija pomaka"),
        ("ScrollAuto", "Autom. pomak"),
        ("Good image quality", "Dobra kvaliteta slike"),
        ("Balanced", "Balansirano"),
        ("Optimize reaction time", "Optimizirano vrijeme reakcije"),
        ("Custom", "Korisničko"),
        ("Show remote cursor", "Prikaži udaljeni kursor"),
        ("Show quality monitor", "Prikaži kvalitetu monitora"),
        ("Disable clipboard", "Zabrani međuspremnik"),
        ("Lock after session end", "Zaključaj po završetku sesije"),
        ("Insert", "Umetni"),
        ("Insert Lock", "Zaključaj umetanje"),
        ("Refresh", "Osvježi"),
        ("ID does not exist", "ID ne postoji"),
        ("Failed to connect to rendezvous server", "Greška u spajanju na poslužitelj za povezivanje"),
        ("Please try later", "Molimo pokušajte kasnije"),
        ("Remote desktop is offline", "Udaljeni zaslon je isključen"),
        ("Key mismatch", "Pogrešan ključ"),
        ("Timeout", "Isteklo vrijeme"),
        ("Failed to connect to relay server", "Greška u spajanju na posredni poslužitelj"),
        ("Failed to connect via rendezvous server", "Greška u spajanju preko poslužitelja za povezivanje"),
        ("Failed to connect via relay server", "Greška u spajanju preko posrednog poslužitelja"),
        ("Failed to make direct connection to remote desktop", "Greška u direktnom spajanju na udaljenu radnu površinu"),
        ("Set Password", "Postavi lozinku"),
        ("OS Password", "Lozinka OS-a"),
        ("install_tip", "Za najbolje performanse dovršite potpunu instalaciju."),
        ("Click to upgrade", "Klik za nadogradnju"),
        ("Click to download", "Klik za preuzimanje"),
        ("Click to update", "Klik za ažuriranje"),
        ("Configure", "Konfiguracija"),
        ("config_acc", "Da biste daljinski kontrolirali radnu površinu, HopToDesk-u trebate dodijeliti prava za \"Pristupačnost\"."),
        ("config_screen", "Da biste daljinski pristupili radnoj površini, HopToDesk-u trebate dodijeliti prava za \"Snimanje zaslona\"."),
        ("Installing ...", "Instaliranje..."),
        ("Install", "Instaliraj"),
        ("Installation", "Instalacija"),
        ("Installation Path", "Putanja za instalaciju"),
        ("Create start menu shortcuts", "Stvori prečace u izborniku"),
        ("Create desktop icon", "Stvori ikonu na radnoj površini"),
        ("agreement_tip", "Pokretanjem instalacije prihvaćate ugovor o licenciranju."),
        ("Accept and Install", "Prihvati i instaliraj"),
        ("End-user license agreement", "Ugovor sa krajnjim korisnikom"),
        ("Generating ...", "Generiranje..."),
        ("Your installation is lower version.", "Vaša instalacija je niže verzije"),
        ("not_close_tcp_tip", "Ne zatvarajte ovaj prozor dok koristite tunel"),
        ("Listening ...", "Na slušanju..."),
        ("Remote Host", "Adresa udaljenog uređaja"),
        ("Remote Port", "Udaljeni port"),
        ("Action", "Postupak"),
        ("Add", "Dodaj"),
        ("Local Port", "Lokalni port"),
        ("Local Address", "Lokalna adresa"),
        ("Change Local Port", "Promijeni lokalni port"),
        ("setup_server_tip", "Za brže spajanje, molimo da koristite vlastiti poslužitelj"),
        ("Too short, at least 6 characters.", "Prekratko, najmanje 6 znakova."),
        ("The confirmation is not identical.", "Potvrda nije identična"),
        ("Permissions", "Dopuštenja"),
        ("Accept", "Prihvati"),
        ("Dismiss", "Odbaci"),
        ("Disconnect", "Prekini vezu"),
        ("Enable file copy and paste", "Dopusti kopiranje i lijepljenje datoteka"),
        ("Connected", "Spojeno"),
        ("Direct and encrypted connection", "Izravna i kriptirana veza"),
        ("Relayed and encrypted connection", "Posredna i kriptirana veza"),
        ("Direct and unencrypted connection", "Izravna i nekriptirana veza"),
        ("Relayed and unencrypted connection", "Posredna i nekriptirana veza"),
        ("Enter Remote ID", "Unesite ID udaljenog uređaja"),
        ("Enter your password", "Unesite svoju lozinku"),
        ("Logging in...", "Prijava..."),
        ("Enable RDP session sharing", "Dopusti dijeljenje RDP sesije"),
        ("Auto Login", "Autom. prijava (Vrijedi samo ako je postavljeno \"Zaključavanje nakon završetka sesije\")"),
        ("Enable direct IP access", "Dopusti izravan pristup preko IP adrese"),
        ("Rename", "Preimenuj"),
        ("Space", "Prazno"),
        ("Create desktop shortcut", "Stvori prečac na radnoj površini"),
        ("Change Path", "Promijeni putanju"),
        ("Create Folder", "Svori mapu"),
        ("Please enter the folder name", "Unesite ime mape"),
        ("Fix it", "Popravi"),
        ("Warning", "Upozorenje"),
        ("Login screen using Wayland is not supported", "Zaslon za prijavu koji koristi Wayland nije podržan"),
        ("Reboot required", "Potrebano je ponovno pokretanje"),
        ("Unsupported display server", "Nepodržani poslužitelj za prikaz"),
        ("x11 expected", "x11 očekivan"),
        ("Port", "Port"),
        ("Settings", "Postavke"),
        ("Username", "Korisničko ime"),
        ("Invalid port", "Pogrešan port"),
        ("Closed manually by the peer", "Klijent ručno prekinuo vezu"),
        ("Enable remote configuration modification", "Dopusti izmjenu udaljene konfiguracije"),
        ("Run without install", "Pokreni bez instalacije"),
        ("Connect via relay", "Povezivanje preko relejnog poslužitelja"),
        ("Always connect via relay", "Uvek se poveži preko relejnog poslužitelja"),
        ("whitelist_tip", "Mogu mi pristupiti samo dozvoljene IP adrese"),
        ("Login", "Prijava"),
        ("Verify", "Potvrdi"),
        ("Remember me", "Zapamti me"),
        ("Trust this device", "Vjeruj ovom uređaju"),
        ("Verification code", "Kontrolni kôd"),
        ("verification_tip", "Kontrolni kôd je poslan na registriranu adresu e-pošte, unesite ga i nastavite s prijavom."),
        ("Logout", "Odjava"),
        ("Tags", "Oznake"),
        ("Search ID", "Traži ID"),
        ("whitelist_sep", "Odvojeno zarezima, točka zarezima, praznim mjestima ili novim redovima"),
        ("Add ID", "Dodaj ID"),
        ("Add Tag", "Dodaj oznaku"),
        ("Unselect all tags", "Odznači sve oznake"),
        ("Network error", "Greška na mreži"),
        ("Username missed", "Korisničko ime propušteno"),
        ("Password missed", "Lozinka propuštena"),
        ("Wrong credentials", "Pogrešno korisničko ime ili lozinka"),
        ("The verification code is incorrect or has expired", "Kôd za provjeru nije točan ili je istekao"),
        ("Edit Tag", "Izmjenite oznaku"),
        ("Forget Password", "Zaboravi lozinku"),
        ("Favorites", "Favoriti"),
        ("Add to Favorites", "Dodaj u favorite"),
        ("Remove from Favorites", "Ukloni iz favorita"),
        ("Empty", "Prazno"),
        ("Invalid folder name", "Nevažeći naziv mape"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Discovered", "Otkriveno"),
        ("install_daemon_tip", "Servis sustava mora biti instaliran ako se želi pokrenuti pri pokretanju sustava."),
        ("Remote ID", "Udaljeni ID"),
        ("Paste", "Zalijepi"),
        ("Paste here?", "Zalijepi ovdje?"),
        ("Are you sure to close the connection?", "Jeste li sigurni da želite zatvoriti vezu?"),
        ("Download new version", "Preuzmi novu verziju"),
        ("Touch mode", "Način rada na dodir"),
        ("Mouse mode", "Način rada miša"),
        ("One-Finger Tap", "Dodir jednim prstom"),
        ("Left Mouse", "Lijeva tipka miša"),
        ("One-Long Tap", "Jedan dugi dodir"),
        ("Two-Finger Tap", "Dodir s dva prsta"),
        ("Right Mouse", "Desna tipka miša"),
        ("One-Finger Move", "Pomak jednim prstom"),
        ("Double Tap & Move", "Dupli dodir i pomak"),
        ("Mouse Drag", "Povlačenje mišem"),
        ("Three-Finger vertically", "Sa tri prsta okomito"),
        ("Mouse Wheel", "Kotačić miša"),
        ("Two-Finger Move", "Pomak s dva prsta"),
        ("Canvas Move", "Pomak pozadine"),
        ("Pinch to Zoom", "Stisnite prste za zumiranje"),
        ("Canvas Zoom", "Zumiranje pozadine"),
        ("Reset canvas", "Resetiraj pozadinu"),
        ("No permission of file transfer", "Nemate pravo prijenosa datoteka"),
        ("Note", "Bilješka"),
        ("Connection", "Povezivanje"),
        ("Share Screen", "Podijeli zaslon"),
        ("Chat", "Dopisivanje"),
        ("Total", "Ukupno"),
        ("items", "stavki"),
        ("Selected", "Odabrano"),
        ("Screen Capture", "Snimanje zaslona"),
        ("Input Control", "Kontrola unosa"),
        ("Audio Capture", "Snimanje zvuka"),
        ("File Connection", "Spajanje preko datoteke"),
        ("Screen Connection", "Podijelite vezu"),
        ("Do you accept?", "Prihvaćate li?"),
        ("Open System Setting", "Postavke otvorenog sustava"),
        ("How to get Android input permission?", "Kako dobiti pristup za unos na Androidu?"),
        ("android_input_permission_tip1", "Da bi ste daljinski uređaj kontrolirali vašim Android uređajem preko miša ili na dodir, trebate dopustiti HopToDesk-u da koristi servis \"Pristupačnost\"."),
        ("android_input_permission_tip2", "Molimo prijeđite na sljedeću stranicu podešavanja sustava, pronađite i unesite [Instalirani servisi], uključite servis [HopToDesk Input]."),
        ("android_new_connection_tip", "Primljen je novi zahtjev za upravljanje, koji želi upravljati vašim uređajem."),
        ("android_service_will_start_tip", "Omogućavanje \"Snimanje zaslona\" automatski će pokrenuti servis, dopuštajući drugim uređajima da zahtjevaju spajanje na vaš uređaj."),
        ("android_stop_service_tip", "Zatvaranje servisa automatski će zatvoriti sve uspostavljene veze."),
        ("android_version_audio_tip", "Trenutna Android verzija ne podržava audio snimanje, molimo nadogradite na Android 10 ili veći."),
        ("android_start_service_tip", "Pritisnite [Pokreni servis] ili omogućite dopuštenje [Snimanje zaslona] za pokretanje usluge dijeljenja zaslona."),
        ("android_permission_may_not_change_tip", "Dopuštenja za uspostavljene veze mogu se promijeniti tek nakon ponovnog povezivanja."),
        ("Account", "Račun"),
        ("Overwrite", "Prepiši"),
        ("This file exists, skip or overwrite this file?", "Ova datoteka postoji, preskoči ju ili prepiši?"),
        ("Quit", "Izlaz"),
        ("Help", "Pomoć"),
        ("Failed", "Neuspješno"),
        ("Succeeded", "Uspešno"),
        ("Someone turns on privacy mode, exit", "Netko je uključio način privatnosti, izlaz."),
        ("Unsupported", "Nepodržano"),
        ("Peer denied", "Klijent zabranjen"),
        ("Please install plugins", "Molimo instalirajte dodatke"),
        ("Peer exit", "Klijent je izašao"),
        ("Failed to turn off", "Greška kod isključenja"),
        ("Turned off", "Isključeno"),
        ("Language", "Jezik"),
        ("Keep HopToDesk background service", "Zadrži HopToDesk kao pozadinski servis"),
        ("Ignore Battery Optimizations", "Zanemari optimizaciju baterije"),
        ("android_open_battery_optimizations_tip", "Ako želite onemogućiti ovu funkciju, molimo idite na sljedeću stranicu za podešavanje HopToDesk aplikacije, pronađite i uđite u [Baterija], onemogućite [Neograničeno]"),
        ("Start on boot", "Pokreni pri pokretanju sustava"),
        ("Start the screen sharing service on boot, requires special permissions", "Za pokretanje usluge dijeljenja zaslona pri pokretanju sustava potrebna su posebna dopuštenja"),
        ("Connection not allowed", "Veza nije dopuštena"),
        ("Legacy mode", "Naslijeđeni način"),
        ("Map mode", "Način mapiranja"),
        ("Translate mode", "Način prevođenja"),
        ("Use permanent password", "Koristi trajnu lozinku"),
        ("Use both passwords", "Koristi obje lozinke"),
        ("Set permanent password", "Postavi trajnu lozinku"),
        ("Enable remote restart", "Omogući daljinsko ponovno pokretanje"),
        ("Restart remote device", "Ponovno pokreni daljinski uređaj"),
        ("Are you sure you want to restart", "Jeste li sigurni da želite ponovno pokretanje"),
        ("Restarting remote device", "Ponovno pokretanje daljinskog uređaja"),
        ("remote_restarting_tip", "Udaljeni uređaj se ponovno pokreće, molimo zatvorite ovu poruku i ponovo se kasnije povežite trajnom lozinkom"),
        ("Copied", "Kopirano"),
        ("Exit Fullscreen", "Izlaz iz cijelog zaslona"),
        ("Fullscreen", "Cijeli zaslon"),
        ("Mobile Actions", "Mobilne akcije"),
        ("Select Monitor", "Odabir monitora"),
        ("Control Actions", "Kontrolni postupci"),
        ("Display Settings", "Postavke prikaza"),
        ("Ratio", "Odnos"),
        ("Image Quality", "Kvaliteta slike"),
        ("Scroll Style", "Način pomaka"),
        ("Show Toolbar", "Prikaži alatnu traku"),
        ("Hide Toolbar", "Sakrij alatnu traku"),
        ("Direct Connection", "Izravna veza"),
        ("Relay Connection", "Posredna veza"),
        ("Secure Connection", "Sigurna veza"),
        ("Insecure Connection", "Nesigurna veza"),
        ("Scale original", "Skaliraj izvornik"),
        ("Scale adaptive", "Prilagođeno skaliranje"),
        ("General", "Općenito"),
        ("Security", "Sigurnost"),
        ("Theme", "Tema"),
        ("Dark Theme", "Tamna tema"),
        ("Light Theme", "Svjetla tema"),
        ("Dark", "Tamna"),
        ("Light", "Svjetla"),
        ("Follow System", "Tema sutava"),
        ("Enable hardware codec", "Omogući hardverski kodek"),
        ("Unlock Security Settings", "Otključaj postavke sigurnosti"),
        ("Enable audio", "Dopusti zvuk"),
        ("Unlock Network Settings", "Otključaj postavke mreže"),
        ("Server", "Poslužitelj"),
        ("Direct IP Access", "Izravan IP pristup"),
        ("Proxy", "Proxy"),
        ("Apply", "Primijeni"),
        ("Disconnect all devices?", "Odspojiti sve uređaje?"),
        ("Clear", "Obriši"),
        ("Audio Input Device", "Uređaj za ulaz zvuka"),
        ("Use IP Whitelisting", "Koristi popis pouzdanih IP adresa"),
        ("Network", "Mreža"),
        ("Pin Toolbar", "Prikači alatnu traku"),
        ("Unpin Toolbar", "Otkvači alatnu traku"),
        ("Recording", "Snimanje"),
        ("Directory", "Mapa"),
        ("Automatically record incoming sessions", "Automatski snimi dolazne sesije"),
        ("Change", "Promijeni"),
        ("Start session recording", "Započni snimanje sesije"),
        ("Stop session recording", "Zaustavi snimanje sesije"),
        ("Enable recording session", "Omogući snimanje sesije"),
        ("Enable LAN discovery", "Omogući LAN otkrivanje"),
        ("Deny LAN discovery", "Onemogući LAN otkrivanje"),
        ("Write a message", "Napiši poruku"),
        ("Prompt", "Spremno"),
        ("Please wait for confirmation of UAC...", "Molimo pričekajte potvrdu UAC-a..."),
        ("elevated_foreground_window_tip", "Tekući prozor udaljene radne površine zahtijeva veće privilegije za rad, tako da trenutno nije moguće koristiti miša i tipkovnicu. Možete zatražiti od udaljenog korisnika da minimizira aktivni prozor, ili kliknuti gumb za povećanje privilegija u prozoru za upravljanje vezom. Kako biste izbjegli ovaj problem, preporučujemo da instalirate softver na udaljeni uređaj."),
        ("Disconnected", "Odspojeno"),
        ("Other", "Ostalo"),
        ("Confirm before closing multiple tabs", "Potvrda prije zatvaranja više kartica"),
        ("Keyboard Settings", "Postavke tipkovnice"),
        ("Full Access", "Potpuni pristup"),
        ("Screen Share", "Dijeljenje zaslona"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland zahtijeva Ubuntu verziju 21.04 ili višu"),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland zahtijeva višu verziju Linux distribucije. Molimo isprobjate X11 ili promijenite OS."),
        ("JumpLink", "Vidi"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Molimo odaberite zaslon koji će biti podijeljen (Za rad na strani klijenta)"),
        ("Show HopToDesk", "Prikaži HopToDesk"),
        ("This PC", "Ovo računalo"),
        ("or", "ili"),
        ("Continue with", "Nastavi sa"),
        ("Elevate", "Izdigni"),
        ("Zoom cursor", "Zumiraj kursor"),
        ("Accept sessions via password", "Prihvati sesije preko lozinke"),
        ("Accept sessions via click", "Prihvati sesije preko klika"),
        ("Accept sessions via both", "Prihvati sesije preko oboje"),
        ("Please wait for the remote side to accept your session request...", "Molimo pričekajte da udaljena strana prihvati vaš zahtjev za sesijom..."),
        ("One-time Password", "Jednokratna lozinka"),
        ("Use one-time password", "Koristi jednokratnu lozinku"),
        ("One-time password length", "Duljina jednokratne lozinke"),
        ("Request access to your device", "Zahtjev za pristup vašem uređaju"),
        ("Hide connection management window", "Sakrij prozor za uređivanje veze"),
        ("hide_cm_tip", "Skrivanje dozvoljeno samo prihvaćanjem sesije preko lozinke i korištenjem trajne lozinke"),
        ("wayland_experiment_tip", "Podrška za Wayland je eksperimentalna, ako trebate pristup bez nadzora, koristite X11."),
        ("Right click to select tabs", "Desni klik za odabir kartica"),
        ("Skipped", "Preskočeno"),
        ("Add to address book", "Dodaj u adresar"),
        ("Group", "Grupa"),
        ("Search", "Pretraga"),
        ("Closed manually by web console", "Zatvoreno ručno pomoću web konzole"),
        ("Local keyboard type", "Vrsta lokalne tipkovnice"),
        ("Select local keyboard type", "Odabir lokalne vrste tipkovnice"),
        ("software_render_tip", "Ako koristite Nvidia grafičku karticu na Linuxu i udaljeni prozor se zatvori odmah nakon povezivanja, prebacivanje na Nouveau upravljački program otvorenog kôda i odabir softverskog renderiranja može pomoći. Potrebno je ponovno pokretanje."),
        ("Always use software rendering", "Uvijek koristite softversko renderiranje"),
        ("config_input", "Za upravljanje udaljenom radnom površinom pomoću tipkovnice, morate dodijeliti HopToDesku dopuštenje \"Nadzor unosa\"."),
        ("config_microphone", "Da biste razgovarali na daljinu, morate dopustiti HopToDesku \"Snimanje zvuka\"."),
        ("request_elevation_tip", "Također možete tražiti podizanje ako je netko na drugoj strani."),
        ("Wait", "Pričekaj"),
        ("Elevation Error", "Pogreška povećanja"),
        ("Ask the remote user for authentication", "Pitajte udaljenog korisnika za autentifikaciju"),
        ("Choose this if the remote account is administrator", "Odaberite ovu opciju ako je udaljeni račun administrator"),
        ("Transmit the username and password of administrator", "Prijenos administratorskog korisničkog imena i lozinke"),
        ("still_click_uac_tip", "Još uvijek zahtijeva da udaljeni korisnik klikne OK u UAC prozoru pokrenutog HopToDeska."),
        ("Request Elevation", "Zahtjev za podizanje"),
        ("wait_accept_uac_tip", "Pričekajte da udaljeni korisnik prihvati UAC dijaloški okvir."),
        ("Elevate successfully", "Uspješno podizanje"),
        ("uppercase", "velika slova"),
        ("lowercase", "mala slova"),
        ("digit", "brojka"),
        ("special character", "poseban znak"),
        ("length>=8", "duljina>=8"),
        ("Weak", "Slabo"),
        ("Medium", "Srednje"),
        ("Strong", "Jako"),
        ("Switch Sides", "Promjena strane"),
        ("Please confirm if you want to share your desktop?", "Potvrdite ako želite dijeliti svoju radnu površinu?"),
        ("Display", "Zaslon"),
        ("Default View Style", "Zadani način prikaza"),
        ("Default Scroll Style", "Zadani način pomaka"),
        ("Default Image Quality", "Zadana kvaliteta slike"),
        ("Default Codec", "Izlazni kodek"),
        ("Bitrate", "Tok podataka"),
        ("FPS", "FPS"),
        ("Auto", "Autom."),
        ("Other Default Options", "Ostale zadane opcije"),
        ("Voice call", "Glasovni poziv"),
        ("Text chat", "Tekstni chat"),
        ("Stop voice call", "Zaustavi glasovni poziv"),
        ("relay_hint_tip", "Izravna veza možda neće biti moguća, možete se pokušati povezati preko relejnog poslužitelja. Osim toga, ako želite koristiti poslužitelj za prosljeđivanje u prvom pokušaju, možete dodati sufiks ID-u \"/r\", ili u kartici nedavnih sesija odaberite opciju \"Uvijek se poveži preko pristupnika\", ako postoji."),
        ("Reconnect", "Ponovno se spojite"),
        ("Codec", "Kodek"),
        ("Resolution", "Razlika"),
        ("No transfers in progress", "Nema prijenosa u tijeku"),
        ("Set one-time password length", "Postavljanje duljine jednokratne lozinke"),
        ("RDP Settings", "Postavljanje RDP-a"),
        ("Sort by", "Poredaj po"),
        ("New Connection", "Nova veza"),
        ("Restore", "Vratiti"),
        ("Minimize", "Smanjiti"),
        ("Maximize", "Povećati"),
        ("Your Device", "Vaš uređaj"),
        ("empty_recent_tip", "Nema nedavne sesije!\nVrijeme je da zakažete novu."),
        ("empty_favorite_tip", "Još nemate nijednog omiljenog partnera?\nPronađite nekoga s kim se možete povezati i dodajte ga u svoje favorite!"),
        ("empty_lan_tip", "Ali ne, izgleda da još nismo otkrili niti jednu drugu stranu."),
        ("empty_address_book_tip", "Izgleda da trenutno nemate nijednog kolege navedenog u svom imeniku."),
        ("eg: admin", "napr. admin"),
        ("Empty Username", "Prazno korisničko ime"),
        ("Empty Password", "Prazna lozinka"),
        ("Me", "Ja"),
        ("identical_file_tip", "Ova je datoteka identična partnerskoj datoteci."),
        ("show_monitors_tip", "Prikažite monitore na alatnoj traci"),
        ("View Mode", "Način prikaza"),
        ("login_linux_tip", "Da biste omogućili sesiju X radne površine, morate se prijaviti na udaljeni Linux račun."),
        ("verify_rustdesk_password_tip", "Provjera lozinke za HopToDesk"),
        ("remember_account_tip", "Zapamti ovaj račun"),
        ("os_account_desk_tip", "Ovaj se račun koristi za prijavu na udaljeni operativni sustav i za omogućavanje sesije radne površine u bezglavom načinu rada."),
        ("OS Account", "Račun operativnog sustava"),
        ("another_user_login_title_tip", "Drugi korisnik je već prijavljen"),
        ("another_user_login_text_tip", "Prekini vezu"),
        ("xorg_not_found_title_tip", "Xorg nije pronađen"),
        ("xorg_not_found_text_tip", "Molimo instalirajte Xorg"),
        ("no_desktop_title_tip", "Nema dostupne radne površine"),
        ("no_desktop_text_tip", "Molimo instalirajte GNOME"),
        ("No need to elevate", "Nije potrebno povećanje"),
        ("System Sound", "Zvuk sustava"),
        ("Default", "Zadano"),
        ("New RDP", "Novi RDP"),
        ("Fingerprint", "Otisak"),
        ("Copy Fingerprint", "Kopirat otisak"),
        ("no fingerprints", "nema otiska"),
        ("Select a peer", "Izbor druge strane"),
        ("Select peers", "Odaberite druge strane"),
        ("Plugins", "Dodaci"),
        ("Uninstall", "Deinstaliraj"),
        ("Update", "Ažuriraj"),
        ("Enable", "Dopustiti"),
        ("Disable", "Zabraniti"),
        ("Options", "Mogućnosti"),
        ("resolution_original_tip", "Izvorna rezolucija"),
        ("resolution_fit_local_tip", "Podesite lokalnu rezoluciju"),
        ("resolution_custom_tip", "Prilagođena rezolucija"),
        ("Collapse toolbar", "Sažmi alatnu traku"),
        ("Accept and Elevate", "Prihvati povećanje"),
        ("accept_and_elevate_btn_tooltip", "Prihvatite vezu i povećajte UAC dopuštenja."),
        ("clipboard_wait_response_timeout_tip", "Isteklo je vrijeme čekanja na kopiju odgovora."),
        ("Incoming connection", "Dolazna veza"),
        ("Outgoing connection", "Odlazna veza"),
        ("Exit", "Izlaz"),
        ("Open", "Otvori"),
        ("logout_tip", "Jeste li sigurni da se želite odjaviti?"),
        ("Service", "Servis"),
        ("Start", "Pokreni"),
        ("Stop", "Zaustavi"),
        ("exceed_max_devices", "Dosegli ste najveći broj upravljanih uređaja."),
        ("Sync with recent sessions", "Sinkronizacija s nedavnim sesijama"),
        ("Sort tags", "Razvrstaj oznake"),
        ("Open connection in new tab", "Otvorite vezu u novoj kartici"),
        ("Move tab to new window", "Premjesti karticu u novi prozor"),
        ("Can not be empty", "Ne može biti prazno"),
        ("Already exists", "Već postoji"),
        ("Change Password", "Promjena lozinke"),
        ("Refresh Password", "Poništavanje lozinke"),
        ("ID", "ID"),
        ("Grid View", "Mreža"),
        ("List View", "Imenik"),
        ("Select", "Odaberi"),
        ("Toggle Tags", "Promijenite oznake"),
        ("pull_ab_failed_tip", "Nije uspjelo vraćanje imenika"),
        ("push_ab_failed_tip", "Sinkronizacija imenika s poslužiteljem nije uspjela"),
        ("synced_peer_readded_tip", "Uređaji koji su bili prisutni u posljednjim sesijama sinkronizirat će se natrag u imenik."),
        ("Change Color", "Promjena boje"),
        ("Primary Color", "Osnovna boja"),
        ("HSV Color", "HSV boja"),
        ("Installation Successful!", "Instalacija uspjela!"),
        ("Installation failed!", "Instalacija nije uspjela!"),
        ("Reverse mouse wheel", "Obrnuti kotačić miša"),
        ("{} sessions", "{} sesija"),
        ("Don't show again", "Ne prikazuj opet"),
        ("I Agree", "Slažem se"),
        ("Decline", "Ne slažem se"),
        ("Timeout in minutes", "Istek u minutama"),
        ("auto_disconnect_option_tip", "Automatsko prekidanje dolaznih veza kada je korisnik neaktivan"),
        ("Connection failed due to inactivity", "Povezivanje nije uspjelo zbog neaktivnosti"),
        ("Check for software update on startup", "Provjera ažuriranja softvera pri pokretanju"),
        ("pull_group_failed_tip", "Vraćanje grupe nije uspjelo"),
        ("Filter by intersection", "Filtriraj po prosjeku"),
        ("Remove wallpaper during incoming sessions", "Uklonite pozadinu tijekom dolaznih sesija"),
        ("Test", "Test"),
        ("display_is_plugged_out_msg", "Zaslon je isključen, prijeđite na prvi zaslon."),
        ("No displays", "Nema zaslona"),
        ("elevated_switch_display_msg", "Prijeđite na primarni zaslon jer više zaslona nije podržano u povišenom načinu rada."),
        ("Open in new window", "Otvori u novom prozoru"),
        ("Show displays as individual windows", "Prikaži zaslone kao pojedinačne prozore"),
        ("Use all my displays for the remote session", "Koristi sve moje zaslone za udaljenu sesiju"),
        ("selinux_tip", "Na vašem uređaju je omogućen SELinux, što može spriječiti HopToDesk da pravilno radi kao upravljana strana."),
        ("Change view", "Promjena prikaza"),
        ("Big tiles", "Velike pločice"),
        ("Small tiles", "Male pločice"),
        ("List", "Imenik"),
        ("Virtual display", "Virtualni zaslon"),
        ("Plug out all", "Odspojite sve"),
        ("True color (4:4:4)", "Stvarne boje (4:4:4)"),
        ("Enable blocking user input", "Omogući blokiranje korisničkog unosa"),
        ("id_input_tip", "Možete unijeti ID, izravnu IP adresu ili domenu s portom (<domena>:<port>).\nAko želite pristupiti uređaju na drugom poslužitelju, povežite adresu poslužitelja (<id>@<adresa_poslužitelja>?kljuć=<vrijednost_ključa>), naprimjer,\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nAko želite pristupiti uređaju na javnom poslužitelju, unesite \"<id>@public\", ključ nije potreban za javni poslužitelj."),
        ("privacy_mode_impl_mag_tip", "Način 1"),
        ("privacy_mode_impl_virtual_display_tip", "Način 2"),
        ("Privacy Mode started.", "Uđite u način privatnosti"),
        ("Privacy Mode turned off.", "Izađi iz načina privatnosti"),
        ("idd_not_support_under_win10_2004_tip", "Neizravni upravljački program za zaslon nije podržan. Potreban je Windows 10 verzija 2004 ili novija."),
        ("switch_display_elevated_connections_tip", "Prebacivanje na zaslon koji nije primarni nije podržan u povišenom načinu rada kada postoji više veza. Ako želite kontrolirati više zaslona, pokušajte ponovno nakon instalacije."),
        ("input_source_1_tip", "Ulazni izvor 1"),
        ("input_source_2_tip", "Ulazni izvor 2"),
        ("capture_display_elevated_connections_tip", "Skeniranje na više zaslona nije podržano u korisničkom načinu rada s povišenim pravima. Ako želite kontrolirati više zaslona, pokušajte ponovno nakon instalacije."),
        ("Swap control-command key", "Zamjena tipki control-command"),
        ("swap-left-right-mouse", "Zamjena lijeve i desne tipke miša"),
        ("2FA code", "2FA kôd"),
        ("More", "Više"),
        ("enable-2fa-title", "Omogući dvofaktorsku autentifikaciju"),
        ("enable-2fa-desc", "Postavite svoj autentifikator. Na telefonu ili računalu možete koristiti aplikaciju za autentifikaciju kao što su Authy, Microsoft ili Google Authenticator.\n\nSkenirajte QR kôd pomoću aplikacije i unesite kôd koji aplikacija prikazuje za aktiviranje dvofaktorske autentifikacije."),
        ("wrong-2fa-code", "Kôd se ne može provjeriti. Provjerite jesu li kôd i postavke lokalnog vremena točni"),
        ("enter-2fa-title", "Dvofaktorska autentifikacija"),
        ("Email verification code must be 6 characters.", "Kôd za provjeru e-pošte mora imati 6 znakova."),
        ("2FA code must be 6 digits.", "2FA kôd mora imati 6 znamenki."),
        ("Multiple Windows sessions found", "Pronađeno je više Windows sesija"),
        ("Please select the session you want to connect to", "Odaberite sesiju kojoj se želite pridružiti"),
        ("powered_by_me", "Pokreće HopToDesk"),
        ("outgoing_only_desk_tip", "Ovo je prilagođeno izdanje.\nMožete se povezati s drugim uređajima, ali se drugi uređaji ne mogu povezati s vašim uređajem."),
        ("preset_password_warning", "Ovo modificirano izdanje dolazi s unaprijed postavljenom lozinkom. Svatko tko zna ovu lozinku može dobiti potpunu kontrolu nad vašim uređajem. Ako to niste očekivali, odmah deinstalirajte softver."),
        ("Security Alert", "Sigurnosno upozorenje"),
        ("My address book", "Moj adresar"),
        ("Personal", "Osobni"),
        ("Owner", "Vlasnik"),
        ("Set shared password", "Postavite zajedničku lozinku"),
        ("Exist in", "Postoji u"),
        ("Read-only", "Samo za čitanje"),
        ("Read/Write", "Način čitanja/pisanja"),
        ("Full Control", "Potpuna kontrola"),
        ("share_warning_tip", "Gornja polja su podijeljena i vidljiva drugima."),
        ("Everyone", "Svatko"),
        ("ab_web_console_tip", "Više na web konzoli"),
        ("allow-only-conn-window-open-tip", ""),
		("Enable Wake On LAN", "Omogući Wake On LAN"),
		("Enable 2FA", "Omogućite 2FA"),
        ("You will need to confirm the 2FA on the secondary device with you when trying to connect to this desktop.", "Morat ćete potvrditi 2FA na sekundarnom uređaju kada se pokušavate povezati s ovom radnom površinom."),		
		("Choose Network", "Odaberite Mreža"),
		("Enter your custom network settings URL:", "Unesite URL prilagođenih mrežnih postavki:"),
		("HopToDesk Network (Default)", "HopToDesk mreža (zadano)"),
		("Custom Network Settings", "Prilagođene mrežne postavke"),
		("Custom Network Error", "Prilagođena mrežna pogreška"),
		("The custom network URL provided was not valid, please try again.", "Navedeni URL prilagođene mreže nije važeći, pokušajte ponovno."),								
		("Your Security Code", "Vaš sigurnosni kod"),
		("ID (Click to Copy)", "ID (kliknite za kopiranje)"),		
		("Password (Click to Copy)", "Lozinka (kliknite za kopiranje)"),
		("Unattended Access", "Pristup bez nadzora"),        
    ].iter().cloned().collect();
}
