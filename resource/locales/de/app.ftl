# App-owned text. The budget remains in EUR in every locale.
lang_en = English
lang_fr = Français
lang_ar = العربية
lang_de = Deutsch
lang_ja = 日本語
lang_zh = 简体中文
lang_ko = 한국어
trip_date = { DATETIME($date, dateStyle: "long") }
itinerary_date = { DATETIME($date, dateStyle: "full") }
time = { DATETIME($time, timeStyle: "short") }
app_title = Day Travel
app_subtitle = Zeit für eine kleine Auszeit
language = Sprache
trip_eyebrow = Malta · Deine nächste Reise
trip_title = Ein Wochenende in Valletta
trip_description = Hafenblick, sonnige Gassen und Zeit zum Entdecken.
trip_duration = Drei Nächte · Freitag bis Montag
itinerary = Ein Tag unterwegs
ferry_title = Mit der Fähre durch den Hafen
ferry_detail = Vom Wasser aus siehst du die Stadt über dem Ufer aufragen.
cafe_title = Kaffee und warme Pastizzi
cafe_detail = Blätterteiggebäck und ein ruhiger Tisch in der Altstadt.
sunset_title = Ein Abend in den Upper Barrakka Gardens
sunset_detail = Schau den Booten vom Garten aus zu, bevor es zum Abendessen geht.
walking_distance = { NUMBER($km, maximumFractionDigits: 1) } km zu Fuß
weather = { NUMBER($degrees) } °C · Klarer Himmel
budget = Das Reisebudget
money = { NUMBER($amount, minimumFractionDigits: 2, maximumFractionDigits: 2) } €
budget_caption = Für die ganze Reise, in Euro
budget_progress = { NUMBER($fraction, style: "percent") } des Budgets verplant
travelers = { $count ->
    [one] Eine Person
   *[other] { NUMBER($count) } Personen
    }
per_person = { NUMBER($amount, minimumFractionDigits: 2, maximumFractionDigits: 2) } € pro Person
remove_traveler = Eine Person entfernen
add_traveler = Eine Person hinzufügen
packing = Vor der Abreise
passport = Reisepass
camera = Kamera
jacket = Leichte Jacke
packed = { $count ->
    [one] Einer von drei Gegenständen eingepackt
   *[other] { NUMBER($count) } von drei Gegenständen eingepackt
    }
demo_note = Mit Day erstellt · Wechsle die Sprache, ändere die Gruppengröße oder passe die Fenstergröße an.
restart_direction = Starte die App in der gewählten Sprache neu, um die Leserichtung zu ändern.
use_dark = Dunkler Modus
use_light = Heller Modus
