//! Native city browsing and trip planning, with bundled photos and an embedded world map.
use day::dark_mode;
use day::prelude::*;
use day_piece_map::{LngLat, MapStatus, MapView, Marker};
mod destinations;
day::resources!();
day::day_start!(options: window_options(), root);

const INK: Ink = Ink::system(SystemColor::Text);
const TEAL: Ink = Ink::adaptive(Color::hex(0x166C68), Color::hex(0x8DD2C5));
const MUTED: Ink = Ink::system(SystemColor::SecondaryText);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Page {
    Discover,
    City(usize),
    Trip,
    Settings,
}
impl Route for Page {
    fn key(&self) -> String {
        match self {
            Self::Discover => "discover".into(),
            Self::City(i) => format!("city-{i}"),
            Self::Trip => "trip".into(),
            Self::Settings => "settings".into(),
        }
    }
    fn title(&self) -> String {
        match self {
            // Photo pages have their own headings; keep only native Back over the artwork.
            Self::Discover | Self::City(_) => String::new(),
            Self::Trip => res::str::my_trip().format(),
            Self::Settings => res::str::preferences().format(),
        }
    }
    fn from_key(key: &str) -> Option<Self> {
        match key {
            "discover" => Some(Self::Discover),
            "trip" => Some(Self::Trip),
            "settings" => Some(Self::Settings),
            _ => key
                .strip_prefix("city-")?
                .parse::<usize>()
                .ok()
                .filter(|i| *i < destinations::COUNT)
                .map(Self::City),
        }
    }
}

// The desktop sidebar selects the last page in the same root-owned path that
// phone navigation and actions update. A binding avoids two competing state signals.
#[derive(Clone, Copy)]
struct SelectedPage(Signal<Vec<Page>>);
impl Binding<Page> for SelectedPage {
    fn read(&self) -> Page {
        self.0.get().last().copied().unwrap_or(Page::Discover)
    }
    fn peek(&self) -> Page {
        self.0
            .get_untracked()
            .last()
            .copied()
            .unwrap_or(Page::Discover)
    }
    fn write(&self, page: Page) {
        self.0.set(if page == Page::Discover {
            Vec::new()
        } else {
            vec![page]
        });
    }
}
const DESKTOP: bool = !cfg!(any(
    target_os = "ios",
    target_os = "android",
    target_env = "ohos"
));

#[derive(Clone, Copy)]
struct Draft {
    departure: Signal<DayDate>,
    nights: Signal<f64>,
    travelers: Signal<f64>,
    interest: Signal<usize>,
    ticket: Signal<usize>,
    name: Signal<String>,
    round_trip: Signal<bool>,
}
#[derive(Clone)]
struct Trip {
    city: usize,
    departure: DayDate,
    nights: f64,
    travelers: f64,
    interest: usize,
    ticket: usize,
    name: String,
    round_trip: bool,
}
impl Draft {
    fn snapshot(self, city: usize) -> Trip {
        Trip {
            city,
            departure: self.departure.get_untracked(),
            nights: self.nights.get_untracked(),
            travelers: self.travelers.get_untracked(),
            interest: self.interest.get_untracked(),
            ticket: self.ticket.get_untracked(),
            name: self.name.get_untracked(),
            round_trip: self.round_trip.get_untracked(),
        }
    }
}
fn return_date(departure: DayDate, nights: f64) -> DayDate {
    DayDate::from_epoch_days(departure.to_epoch_days() + nights.round() as i64)
}
fn interest_title(interest: usize) -> LocalizedText {
    match interest {
        0 => res::str::interest_mixed(),
        1 => res::str::interest_culture(),
        _ => res::str::interest_nature(),
    }
}
pub fn window_options() -> WindowOptions {
    let mut options = res::window_options();
    options.title_fn = Some(|| res::str::app_title().format());
    options
}
fn photo(index: usize) -> impl Piece {
    image(destinations::artwork(index))
        .aspect_ratio(16.0 / 9.0)
        .fill()
        .decorative()
        .grow(Dimension::Both)
        .height(|| {
            if day::size_class().is_some_and(|c| c.width == WidthClass::Compact) {
                220.0
            } else {
                300.0
            }
        })
        .corner_radius(0.0)
}
fn heading(text: LocalizedText) -> impl Piece {
    label(text)
        .font(Font::Title2)
        .bold()
        .color(INK)
        .a11y(|a| a.role(Role::Heading(2)))
}
fn platform_map(view: MapView) -> (AnyPiece, Memo<bool>) {
    let ready = Memo::new(move || matches!(view.status.get(), MapStatus::Ready));
    if day_piece_map_apple::support() == Support::Native {
        (day_piece_map_apple::map(view).any(), ready)
    } else if day_piece_map_google::support() == Support::Native {
        (day_piece_map_google::map(view).any(), ready)
    } else {
        let canvas = day_piece_map_canvas::CanvasMapView::with_view(view);
        let locale = day::locale().get_untracked();
        canvas.set_language(locale.split(['-', '_']).next());
        // Provider readiness means the style is usable, not that the basemap has painted.
        // Capture only after tile requests finish and at least one raster reaches the screen.
        let painted = Memo::new(move || {
            ready.get() && canvas.pending.get() == 0 && canvas.frame.get().bitmaps > 0
        });
        (
            day_piece_map_canvas::map("https://tiles.openfreemap.org/styles/liberty", canvas).any(),
            painted,
        )
    }
}
fn map_section(view: MapView) -> impl Piece {
    let (map, ready) = platform_map(view);
    column((
        column((
            heading(res::str::map_heading()),
            when(
                move || ready.get(),
                || {
                    label(res::str::map_hint())
                        .font(Font::Callout)
                        .color(MUTED)
                        .id(day::id!(map_ready))
                },
            ),
            row((
                button(res::str::map_world())
                    .bordered()
                    .action(move || {
                        view.fit_bounds(LngLat::new(-130.0, -40.0), LngLat::new(155.0, 65.0), 24.0)
                    })
                    .id(day::id!(map_world)),
                button(res::str::map_zoom_in())
                    .bordered()
                    .action(move || view.zoom_by(1.0))
                    .id(day::id!(map_zoom_in)),
                button(res::str::map_zoom_out())
                    .bordered()
                    .action(move || view.zoom_by(-1.0))
                    .id(day::id!(map_zoom_out)),
            ))
            .spacing(Space::S),
            when(
                move || matches!(view.status.get(), MapStatus::Loading),
                || label(res::str::map_loading()).font(Font::Caption),
            ),
            when(
                move || matches!(view.status.get(), MapStatus::Failed(_)),
                || label(res::str::map_unavailable()).font(Font::Caption),
            ),
        ))
        .spacing(Space::S)
        .padding(Space::M)
        .align(HAlign::Leading),
        map.height(320.0)
            .grow(Dimension::Width)
            .a11y(|a| a.label(res::str::map_heading()))
            .id(day::id!(destination_map)),
    ))
    .spacing(0.0)
    .grow(Dimension::Width)
}
fn home(path: Signal<Vec<Page>>, view: MapView, query: Signal<String>) -> impl Piece {
    scroll(
        column((
            photo(0).id(day::id!(home_photo)),
            column((
                label(res::str::collection(
                    i64::try_from(destinations::COUNT).expect("city count fits i64"),
                ))
                .font(Font::Caption)
                .color(TEAL),
                label(res::str::choose_city())
                    .font(Font::LargeTitle)
                    .bold()
                    .color(INK)
                    .a11y(|a| a.role(Role::Heading(1))),
                label(res::str::escape_caption()).color(MUTED),
                row((
                    button(res::str::my_trip())
                        .bordered()
                        .action(move || path.set(vec![Page::Trip]))
                        .id(day::id!(open_trip)),
                    button(res::str::preferences())
                        .bordered()
                        .action(move || path.set(vec![Page::Settings]))
                        .id(day::id!(open_settings)),
                ))
                .spacing(Space::S),
                when(
                    || !DESKTOP,
                    move || {
                        text_field(query)
                            .placeholder(res::str::search_cities())
                            .a11y(|a| a.label(res::str::search_cities()))
                            .grow(Dimension::Width)
                            .id(day::id!(city_search))
                    },
                ),
            ))
            .spacing(Space::S)
            .align(HAlign::Leading)
            .padding(Space::M),
            when(
                || !DESKTOP,
                move || {
                    let cities = items(
                        move || {
                            let query = query.get().trim().to_lowercase();
                            (0..destinations::COUNT)
                                .filter(|i| {
                                    query.is_empty()
                                        || destinations::title(*i)
                                            .format()
                                            .to_lowercase()
                                            .contains(&query)
                                        || destinations::info(*i)
                                            .country
                                            .to_lowercase()
                                            .contains(&query)
                                })
                                .collect::<Vec<_>>()
                        },
                        |i| *i,
                    );
                    list(cities, |slot| {
                        list_item(move || destinations::title(slot.get()).format())
                            .subtitle(move || destinations::info(slot.get()).country)
                            .id_of(move || day::id!(city_row, slot.key()))
                    })
                    .on_select(move |i| path.set(vec![Page::City(i)]))
                    .on_activate(move |i| path.set(vec![Page::City(i)]))
                    .separators()
                    .height(320.0)
                    .grow(Dimension::Width)
                    .empty(|| label(res::str::no_cities()).padding(Space::M))
                    .id(day::id!(city_list))
                },
            ),
            map_section(view),
        ))
        .spacing(0.0)
        .grow(Dimension::Width),
    )
    .id(day::id!(home_screen))
}
fn ticket_title(ticket: usize) -> LocalizedText {
    match ticket {
        0 => res::str::ticket_economy(),
        1 => res::str::ticket_business(),
        _ => res::str::ticket_first_class(),
    }
}
fn planning_controls(draft: Draft) -> impl Piece {
    form((section((
        labeled(
            res::str::departure(),
            date_picker(draft.departure)
                .compact()
                .min(DayDate::today())
                .id(day::id!(departure_date)),
        ),
        labeled(
            res::str::stay(),
            row((
                slider(draft.nights)
                    .range(2.0..=14.0)
                    .step(1.0)
                    .a11y(|a| a.label(res::str::stay()))
                    .grow(Dimension::Width)
                    .id(day::id!(stay_slider)),
                label(res::str::stay_length(draft.nights)).id(day::id!(stay_length)),
            ))
            .spacing(Space::S)
            .grow(Dimension::Width),
        ),
        labeled(
            res::str::travelers(),
            stepper(draft.travelers)
                .range(1.0..=8.0)
                .step(1.0)
                .id(day::id!(travelers_stepper)),
        ),
        labeled(
            res::str::ticket_class(),
            picker(
                draft.ticket,
                [
                    (0, res::str::ticket_economy()),
                    (1, res::str::ticket_business()),
                    (2, res::str::ticket_first_class()),
                ],
            )
            .segmented()
            .grow(Dimension::Width)
            .id(day::id!(ticket_picker)),
        ),
        labeled(
            res::str::interests(),
            picker(
                draft.interest,
                [
                    (0, res::str::interest_mixed()),
                    (1, res::str::interest_culture()),
                    (2, res::str::interest_nature()),
                ],
            )
            .menu()
            .id(day::id!(interest_picker)),
        ),
        labeled(
            res::str::round_trip(),
            toggle(draft.round_trip).id(day::id!(round_trip_toggle)),
        ),
        labeled(
            res::str::trip_name(),
            text_field(draft.name)
                .placeholder(res::str::trip_name_hint())
                .grow(Dimension::Width)
                .id(day::id!(trip_name)),
        ),
    )),))
}
fn city_page(
    city: usize,
    draft: Draft,
    saved: Signal<Option<Trip>>,
    path: Signal<Vec<Page>>,
) -> impl Piece {
    let view = MapView::new(destinations::info(city).position, 11.5);
    let (map, ready) = platform_map(view);
    let marker = view.add_marker(Marker::new(destinations::info(city).position));
    Effect::new(move || {
        view.update_marker(
            marker,
            Marker::new(destinations::info(city).position)
                .title(destinations::title(city).format()),
        );
    });
    column((
        photo(city).id(day::id!(destination_photo)),
        column((
            row((
                column((
                    label(destinations::title(city))
                        .font(Font::LargeTitle)
                        .bold()
                        .color(INK)
                        .a11y(|a| a.role(Role::Heading(1)))
                        .id(day::id!(trip_title)),
                    // Localized city nicknames and their sources are recorded in CAPTIONS.md.
                    label(destinations::tagline(city))
                        .font(Font::Callout)
                        .color(MUTED)
                        .id(day::id!(city_tagline)),
                ))
                .spacing(Space::S)
                .align(HAlign::Leading)
                .grow(Dimension::Width),
                button(res::str::save_trip())
                    .prominent()
                    .action(move || {
                        saved.set(Some(draft.snapshot(city)));
                        path.update(|p| p.push(Page::Trip));
                    })
                    .id(day::id!(save_trip)),
            ))
            .spacing(Space::M)
            .align(VAlign::Center)
            .grow(Dimension::Width),
            planning_controls(draft),
            when(
                move || matches!(view.status.get(), MapStatus::Loading),
                || label(res::str::map_loading()).font(Font::Caption),
            ),
            when(
                move || matches!(view.status.get(), MapStatus::Failed(_)),
                || label(res::str::city_map_unavailable()).font(Font::Caption),
            ),
        ))
        .spacing(Space::S)
        .align(HAlign::Leading)
        .padding(Space::M)
        .grow(Dimension::Width),
        column((map
            .grow(Dimension::Both)
            .a11y(move |a| a.label(res::str::city_map_label(destinations::title(city).format())))
            .id(day::id!(city_map)),))
        // Compensate for bottom chrome exposed by the backend. Native navigation containers
        // may still clip at their safe area; the controls above remain inset.
        .padding(|| Insets {
            bottom: if cfg!(any(target_os = "ios", target_os = "android")) {
                -day::safe_area().bottom
            } else {
                0.0
            },
            ..Insets::ZERO
        })
        // Grow the padded surface itself so the parent allocates the remaining height.
        .grow(Dimension::Both)
        .id_of(move || {
            if ready.get() {
                day::id!(city_map_ready)
            } else {
                day::id!(city_map_pending)
            }
        }),
    ))
    .spacing(0.0)
    .grow(Dimension::Both)
    .id(day::id!(travel_screen))
}
fn trip_page(saved: Signal<Option<Trip>>, path: Signal<Vec<Page>>) -> impl Piece {
    scroll(
        column((
            when(
                move || saved.get().is_some(),
                move || {
                    let trip = saved
                        .get_untracked()
                        .expect("saved trip is present in this branch");
                    let city = trip.city;
                    column((
                        photo(city).id(day::id!(saved_photo)),
                        column((
                            label(res::str::trip_saved())
                                .font(Font::Caption)
                                .color(TEAL),
                            label(destinations::title(city))
                                .font(Font::LargeTitle)
                                .bold()
                                .color(INK)
                                .id(day::id!(saved_city)),
                            label(if trip.name.trim().is_empty() {
                                res::str::trip_name_hint().format()
                            } else {
                                trip.name
                            })
                            .font(Font::Title3)
                            .id(day::id!(saved_name)),
                            label(res::str::departing_on(trip.departure.to_string()))
                                .id(day::id!(saved_departure)),
                            when(
                                move || trip.round_trip,
                                move || {
                                    label(res::str::return_on(
                                        return_date(trip.departure, trip.nights).to_string(),
                                    ))
                                    .id(day::id!(saved_return))
                                },
                            ),
                            label(res::str::stay_length(trip.nights)).id(day::id!(saved_stay)),
                            label(res::str::traveler_count(trip.travelers))
                                .id(day::id!(saved_travelers)),
                            label(interest_title(trip.interest)).id(day::id!(saved_interest)),
                            label(ticket_title(trip.ticket)).id(day::id!(saved_ticket)),
                            label(if trip.round_trip {
                                res::str::round_trip()
                            } else {
                                res::str::one_way()
                            })
                            .id(day::id!(saved_trip_type)),
                            button(res::str::edit_trip())
                                .bordered()
                                .action(move || path.set(vec![Page::City(city)]))
                                .id(day::id!(edit_trip)),
                            label(res::str::saved_note())
                                .font(Font::Footnote)
                                .color(MUTED),
                        ))
                        .spacing(Space::M)
                        .padding(Space::M)
                        .align(HAlign::Leading),
                    ))
                    .spacing(0.0)
                    .grow(Dimension::Width)
                },
            ),
            when(
                move || saved.get().is_none(),
                || {
                    label(res::str::trip_empty())
                        .font(Font::Title2)
                        .padding(Space::L)
                },
            ),
        ))
        .grow(Dimension::Width),
    )
    .padding(|| Insets {
        top: day::safe_area().top,
        ..Insets::default()
    })
    .id(day::id!(saved_screen))
}
fn settings() -> impl Piece {
    scroll(
        column((
            heading(res::str::preferences()),
            form((section((
                labeled(
                    res::str::language(),
                    picker(
                        day::locale(),
                        [
                            ("en-US".to_owned(), res::str::lang_en()),
                            ("fr".to_owned(), res::str::lang_fr()),
                            ("de".to_owned(), res::str::lang_de()),
                            ("ja".to_owned(), res::str::lang_ja()),
                            ("zh-CN".to_owned(), res::str::lang_zh()),
                            ("ko".to_owned(), res::str::lang_ko()),
                            ("ar".to_owned(), res::str::lang_ar()),
                        ],
                    )
                    .menu()
                    .id(day::id!(language_picker)),
                ),
                button(|| {
                    if dark_mode() {
                        res::str::use_light().format()
                    } else {
                        res::str::use_dark().format()
                    }
                })
                .bordered()
                .action(|| day::set_appearance(Some(!dark_mode())))
                .id(day::id!(theme_toggle)),
            )),)),
            label(res::str::restart_direction())
                .font(Font::Footnote)
                .color(MUTED),
            label(res::str::artwork_note())
                .font(Font::Footnote)
                .color(MUTED),
        ))
        .spacing(Space::L)
        .padding(Space::L)
        .align(HAlign::Leading),
    )
    .padding(|| Insets {
        top: day::safe_area().top,
        ..Insets::default()
    })
}
pub fn root() -> impl Piece {
    if day::locale().get().starts_with("en") {
        day::locale().set("en-US".to_owned());
    }
    let path = Signal::new(Vec::<Page>::new());
    let draft = Draft {
        departure: Signal::new(DayDate::from_epoch_days(
            DayDate::today().to_epoch_days() + 30,
        )),
        nights: Signal::new(4.0),
        travelers: Signal::new(2.0),
        interest: Signal::new(0),
        ticket: Signal::new(0),
        name: Signal::new(String::new()),
        round_trip: Signal::new(true),
    };
    let saved = Signal::new(None::<Trip>);
    let query = Signal::new(String::new());
    let view = MapView::new(LngLat::new(15.0, 22.0), 0.7);
    let markers: Vec<_> = (0..destinations::COUNT)
        .map(|i| view.add_marker(Marker::new(destinations::info(i).position).tag(i.to_string())))
        .collect();
    Effect::new(move || {
        let _locale = day::locale().get();
        // MapKit manages annotation collisions. Keep the canvas overview quiet;
        // reveal city names once the reader zooms in, retaining stable pin identities.
        let titles = day_piece_map_apple::support() == Support::Native || view.zoom.get() >= 3.0;
        for (i, marker) in markers.iter().enumerate() {
            view.update_marker(
                *marker,
                Marker::new(destinations::info(i).position)
                    .title(if titles {
                        destinations::title(i).format()
                    } else {
                        String::new()
                    })
                    .tag(i.to_string())
                    .color(Color::hex(0x166C68)),
            );
        }
    });
    Effect::new(move || {
        if let Some(tap) = view.tap.get()
            && let Some(marker) = tap.marker.and_then(|id| view.marker(id))
            && let Ok(city) = marker.tag.parse::<usize>()
            && city < destinations::COUNT
        {
            path.set(vec![Page::City(city)]);
        }
    });
    let destination = move |page: &Page| match page {
        Page::Discover => home(path, view, query).any(),
        Page::City(city) => city_page(*city, draft, saved, path).any(),
        Page::Trip => trip_page(saved, path).any(),
        Page::Settings => settings().any(),
    };
    if DESKTOP {
        nav(SelectedPage(path))
            .style(NavStyle::Sidebar)
            .title(res::str::app_title())
            .items((
                item(Page::Discover, res::str::discover()),
                (0..destinations::COUNT)
                    .map(|i| item(Page::City(i), destinations::title(i)))
                    .collect::<Vec<_>>(),
                item(Page::Trip, res::str::my_trip()),
                item(Page::Settings, res::str::preferences()),
            ))
            .destination(destination)
            .id(day::id!(city_sidebar))
            .any()
    } else {
        nav_stack(path, home(path, view, query))
            .immersive(|page| matches!(page, None | Some(Page::City(_))))
            .destination(destination)
            .id(day::id!(travel_stack))
            .any()
    }
}
