# App-owned text. EUR stays EUR: language switching never converts the trip budget.
app_title = Day Travel
app_subtitle = A little time away
language = Language
lang_en = English
lang_fr = Français
lang_ar = العربية
trip_eyebrow = Malta · Your next trip
trip_title = A weekend in Valletta
trip_description = Harbour views, sunlit streets, and time to explore.
trip_date = { DATETIME($date, dateStyle: "long") }
trip_duration = Three nights · Friday to Monday
itinerary = A day out
itinerary_date = { DATETIME($date, dateStyle: "full") }
time = { DATETIME($time, timeStyle: "short") }
ferry_title = A ferry across the harbour
ferry_detail = Start on the water, with the city rising above the shore.
cafe_title = Coffee & warm pastizzi
cafe_detail = A flaky pastry and a quiet table in the old town.
sunset_title = An evening at Upper Barrakka
sunset_detail = Watch the boats from the gardens, then head out for dinner.
walking_distance = { NUMBER($km, maximumFractionDigits: 1) } km on foot
weather = { NUMBER($degrees) } °C · Clear skies
budget = The trip budget
money = €{ NUMBER($amount, minimumFractionDigits: 2, maximumFractionDigits: 2) }
budget_caption = For the whole trip, in euros
budget_progress = { NUMBER($fraction, style: "percent") } of the budget planned
travelers = { $count ->
    [one] One traveler
   *[other] { NUMBER($count) } travelers
    }
per_person = €{ NUMBER($amount, minimumFractionDigits: 2, maximumFractionDigits: 2) } per person
remove_traveler = Remove a traveler
add_traveler = Add a traveler
packing = Before you go
passport = Passport
camera = Camera
jacket = Light jacket
packed = { $count ->
    [one] One of three things packed
   *[other] { NUMBER($count) } of three things packed
    }
demo_note = Made with Day · Try another language, change the group size, or resize the window.
restart_direction = Restart in the selected language to change the layout direction.
lang_de = Deutsch
lang_ja = 日本語
lang_zh = 简体中文
lang_ko = 한국어
use_dark = Dark mode
use_light = Light mode
