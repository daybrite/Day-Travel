# App-owned text. EUR stays EUR: language switching never converts the trip budget.
app_title = Day Travel
app_subtitle = Une petite escapade
language = Langue
lang_en = English
lang_fr = Français
lang_ar = العربية
trip_eyebrow = Malte · Votre prochain voyage
trip_title = Un week-end à La Valette
trip_description = Le port, des rues ensoleillées et le temps de flâner.
trip_date = { DATETIME($date, dateStyle: "long") }
trip_duration = Trois nuits · Du vendredi au lundi
itinerary = Au fil de la journée
itinerary_date = { DATETIME($date, dateStyle: "full") }
time = { DATETIME($time, timeStyle: "short") }
ferry_title = Traversée du port en ferry
ferry_detail = Commencez sur l’eau, face à la ville qui domine le rivage.
cafe_title = Café et pastizzi tout chauds
cafe_detail = Un feuilleté et une table au calme dans la vieille ville.
sunset_title = Une soirée à Upper Barrakka
sunset_detail = Admirez les bateaux depuis les jardins avant de partir dîner.
walking_distance = { NUMBER($distance, maximumFractionDigits: 1) } km à pied
weather = { NUMBER($degrees) } °C · Ciel dégagé
budget = Le budget du séjour
money = { NUMBER($amount, minimumFractionDigits: 2, maximumFractionDigits: 2) } €
budget_caption = Pour tout le séjour, en euros
budget_progress = { NUMBER($fraction, style: "percent") } du budget prévu
travelers = { $count ->
    [one] Une personne
   *[other] { NUMBER($count) } personnes
    }
per_person = { NUMBER($amount, minimumFractionDigits: 2, maximumFractionDigits: 2) } € par personne
remove_traveler = Retirer une personne
add_traveler = Ajouter une personne
packing = Avant de partir
passport = Passeport
camera = Appareil photo
jacket = Veste légère
packed = { $count ->
    [one] Un objet sur trois dans la valise
   *[other] { NUMBER($count) } objets sur trois dans la valise
    }
demo_note = Créé avec Day · Changez de langue, ajustez le nombre de personnes ou redimensionnez la fenêtre.
restart_direction = Relancez l’application dans la langue choisie pour changer le sens de lecture.
lang_de = Deutsch
lang_ja = 日本語
lang_zh = 简体中文
lang_ko = 한국어
use_dark = Mode sombre
use_light = Mode clair
