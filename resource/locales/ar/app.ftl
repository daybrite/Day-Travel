# App-owned text. EUR stays EUR: language switching never converts the trip budget.
app_title = Day Travel
app_subtitle = استراحة قصيرة
language = اللغة
lang_en = English
lang_fr = Français
lang_ar = العربية
trip_eyebrow = مالطا · رحلتك القادمة
trip_title = عطلة في فاليتا
trip_description = إطلالات على الميناء، وشوارع مشمسة، ووقت للاستكشاف.
trip_date = { DATETIME($date, dateStyle: "long") }
trip_duration = ثلاث ليالٍ · من الجمعة إلى الاثنين
itinerary = يوم في المدينة
itinerary_date = { DATETIME($date, dateStyle: "full") }
time = { DATETIME($time, timeStyle: "short") }
ferry_title = بالعبّارة عبر الميناء
ferry_detail = ابدأ يومك على الماء، والمدينة ترتفع أمامك فوق الساحل.
cafe_title = قهوة وباستيزي ساخنة
cafe_detail = فطيرة هشة وطاولة هادئة في البلدة القديمة.
sunset_title = أمسية في حدائق باراكا العليا
sunset_detail = راقب القوارب من الحدائق، ثم انطلق لتناول العشاء.
walking_distance = { NUMBER($distance, maximumFractionDigits: 1) } كم سيرًا
weather = { NUMBER($degrees) } °م · سماء صافية
budget = ميزانية الرحلة
money = { NUMBER($amount, minimumFractionDigits: 2, maximumFractionDigits: 2) } €
budget_caption = للرحلة كاملة، باليورو
budget_progress = تم التخطيط لـ { NUMBER($fraction, style: "percent") } من الميزانية
travelers = { $count ->
    [zero] لا مسافرين
    [one] مسافر واحد
    [two] مسافران
    [few] { NUMBER($count) } مسافرين
    [many] { NUMBER($count) } مسافرًا
   *[other] { NUMBER($count) } مسافر
    }
per_person = { NUMBER($amount, minimumFractionDigits: 2, maximumFractionDigits: 2) } € للشخص
remove_traveler = إزالة مسافر
add_traveler = إضافة مسافر
packing = قبل الانطلاق
passport = جواز السفر
camera = الكاميرا
jacket = سترة خفيفة
packed = { $count ->
    [zero] لم تُحزم الأغراض بعد
    [one] غرض واحد من ثلاثة في الحقيبة
    [two] غرضان من ثلاثة في الحقيبة
    [few] { NUMBER($count) } أغراض من ثلاثة في الحقيبة
    [many] { NUMBER($count) } غرضًا في الحقيبة
   *[other] { NUMBER($count) } غرض في الحقيبة
    }
demo_note = صُنع باستخدام Day · جرّب لغة أخرى، أو غيّر عدد المسافرين، أو حجم النافذة.
restart_direction = أعد تشغيل التطبيق باللغة المختارة لتغيير اتجاه الواجهة.
lang_de = Deutsch
lang_ja = 日本語
lang_zh = 简体中文
lang_ko = 한국어
use_dark = المظهر الداكن
use_light = المظهر الفاتح
