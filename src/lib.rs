//! A local screenshot demo. Trip data is fixed so captures can be compared across locales.
use day::dark_mode;
use day::prelude::*;

day::resources!();
day::day_start!(options: window_options(), root);

/// Resolve the window title through the app’s localization catalog.
pub fn window_options() -> WindowOptions {
    let mut options = res::window_options();
    options.title_fn = Some(|| res::str::app_title().format());
    options
}

// Reactive colors follow the toolkit appearance without rebuilding form state.
fn ink() -> Color {
    if dark_mode() {
        Color::hex(0xE8F0EB)
    } else {
        Color::hex(0x243D42)
    }
}
fn teal() -> Color {
    if dark_mode() {
        Color::hex(0x8DD2C5)
    } else {
        Color::hex(0x166C68)
    }
}
fn muted() -> Color {
    if dark_mode() {
        Color::hex(0xB1C3C0)
    } else {
        Color::hex(0x617577)
    }
}
fn paper() -> Color {
    if dark_mode() {
        Color::hex(0x142326)
    } else {
        Color::hex(0xF5F3EC)
    }
}
fn sand() -> Color {
    if dark_mode() {
        Color::hex(0xC6BEA9)
    } else {
        Color::hex(0xEAE5D8)
    }
}
fn surface() -> Color {
    if dark_mode() {
        Color::hex(0x213337)
    } else {
        Color::WHITE
    }
}
fn budget_surface() -> Color {
    if dark_mode() {
        Color::hex(0x233F3E)
    } else {
        Color::hex(0xE8F0EC)
    }
}
fn weather_surface() -> Color {
    if dark_mode() {
        Color::hex(0x2B4340)
    } else {
        Color::hex(0xE9F2EB)
    }
}

fn heading<M>(text: impl IntoText<M>) -> impl Piece {
    label(text)
        .font(Font::Title3)
        .weight(FontWeight::Semibold)
        .color(ink)
        .a11y(|a| a.role(Role::Heading(2)))
}

// XAML cannot convert gradient SVGs to native paths. Its current glyph fallback pads
// to a 256px square, which shrinks this wide illustration. Ship full-size, correctly
// proportioned PNGs for Windows until that fallback supports illustration viewports.
// The SVGs remain the authoring source; scripts/render_windows_art.sh regenerates PNGs.
fn harbour_art(compact: bool) -> impl Piece {
    #[cfg(target_os = "windows")]
    let art = image(move || {
        let name = match (compact, dark_mode()) {
            (true, true) => res::images::valletta_detail_dark_windows,
            (true, false) => res::images::valletta_detail_windows,
            (false, true) => res::images::valletta_dark_windows,
            (false, false) => res::images::valletta_windows,
        };
        day::ImageSource::from(name)
    });
    #[cfg(not(target_os = "windows"))]
    let art = vector(move || match (compact, dark_mode()) {
        (true, true) => res::vectors::valletta_detail_dark,
        (true, false) => res::vectors::valletta_detail,
        (false, true) => res::vectors::valletta_dark,
        (false, false) => res::vectors::valletta,
    });
    art.decorative()
        .aspect_ratio(if compact {
            720.0 / 280.0
        } else {
            1200.0 / 280.0
        })
        .grow()
}

/// Keep the same state while resizing or changing language. All app text and illustrations
/// use generated resource accessors. Locale tags and fixed ISO dates are data, not UI strings.
pub fn root() -> impl Piece {
    // The sample offers US English. Keep the base `en` catalog as the fallback,
    // while giving number/date formatting and the picker an explicit regional tag.
    if day::locale().get().starts_with("en") {
        day::locale().set("en-US".to_owned());
    }
    let travelers = Signal::new(2i64);
    let launched_rtl = is_rtl();
    let passport = Signal::new(true);
    let camera = Signal::new(true);
    let jacket = Signal::new(false);
    scroll(
        column((
            adaptive((
                row((
                    vector(res::vectors::compass).decorative().frame(36.0, 36.0),
                    column((
                        label(res::str::app_title())
                            .font(Font::Title2)
                            .weight(FontWeight::Bold)
                            .color(ink),
                        label(res::str::app_subtitle())
                            .font(Font::Caption)
                            .color(muted),
                    ))
                    .align(HAlign::Leading)
                    .spacing(Space::XS),
                ))
                .spacing(Space::S)
                .grow(),
                row((
                    column((
                        label(res::str::language()).font(Font::Caption).color(muted),
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
                        .a11y(|a| a.label(res::str::language()))
                        .id(day::id!(language_picker)),
                    ))
                    .align(HAlign::Leading)
                    .spacing(Space::XS),
                    when(
                        || capability(Cap::Appearance) != Support::Unsupported,
                        || {
                            button(move || {
                                if dark_mode() {
                                    res::str::use_light().format()
                                } else {
                                    res::str::use_dark().format()
                                }
                            })
                            .icon(|| {
                                if dark_mode() {
                                    Symbol::Light
                                } else {
                                    Symbol::Dark
                                }
                            })
                            .bordered()
                            .icon_only()
                            .action(|| day::set_appearance(Some(!dark_mode())))
                            .id(day::id!(theme_toggle))
                        },
                    ),
                ))
                .spacing(Space::S),
            ))
            .spacing(Space::M),
            column((
                // Rebuild only the decorative artwork at the compact breakpoint. Form state stays mounted.
                switch(
                    move || day::size_class().is_some_and(|c| c.width == WidthClass::Compact),
                    |compact| harbour_art(*compact),
                ),
                adaptive((
                    column((
                        label(res::str::trip_eyebrow())
                            .font(Font::Caption)
                            .weight(FontWeight::Semibold)
                            .color(teal),
                        label(res::str::trip_title())
                            .font(Font::LargeTitle)
                            .weight(FontWeight::Bold)
                            .color(ink)
                            .a11y(|a| a.role(Role::Heading(1)))
                            .id(day::id!(trip_title)),
                        label(res::str::trip_description()).color(muted),
                    ))
                    .align(HAlign::Leading)
                    .spacing(Space::S)
                    .grow(),
                    column((
                        label(res::str::trip_date("2027-06-18"))
                            .font(Font::Headline)
                            .color(ink)
                            .id(day::id!(trip_date)),
                        label(res::str::trip_duration())
                            .font(Font::Callout)
                            .color(muted),
                    ))
                    .align(HAlign::Leading)
                    .spacing(Space::S),
                ))
                .spacing(Space::L)
                .padding(Space::L),
            ))
            .align(HAlign::Leading)
            .background(surface)
            .corner_radius(18.0),
            adaptive((
                column((
                    heading(res::str::itinerary()),
                    label(res::str::itinerary_date("2027-06-19"))
                        .font(Font::Callout)
                        .color(muted),
                    itinerary_item(
                        res::vectors::ferry,
                        res::str::time("09:30"),
                        res::str::ferry_title(),
                        res::str::ferry_detail(),
                    ),
                    itinerary_item(
                        res::vectors::cup,
                        res::str::time("11:00"),
                        res::str::cafe_title(),
                        res::str::cafe_detail(),
                    ),
                    itinerary_item(
                        res::vectors::sunset,
                        res::str::time("18:45"),
                        res::str::sunset_title(),
                        res::str::sunset_detail(),
                    ),
                    row((
                        label(|| {
                            let us = day::locale().get().starts_with("en");
                            res::str::walking_distance(display_distance(6.4, us)).format()
                        })
                        .id(day::id!(walking_distance))
                        .font(Font::Callout)
                        .color(teal),
                        spacer(),
                        label(|| {
                            let us = day::locale().get().starts_with("en");
                            res::str::weather(display_temperature(24.0, us)).format()
                        })
                        .id(day::id!(weather))
                        .font(Font::Callout)
                        .color(teal),
                    ))
                    .fit(RowFit::Wrap { run_spacing: 8.0 })
                    .spacing(Space::M)
                    .padding(Space::M)
                    .background(weather_surface)
                    .corner_radius(10.0),
                ))
                .align(HAlign::Leading)
                .spacing(Space::M)
                .padding(Space::L)
                .background(surface)
                .corner_radius(16.0)
                .grow(),
                column((
                    column((
                        heading(res::str::budget()),
                        label(res::str::money(1284.50))
                            .font(Font::LargeTitle)
                            .weight(FontWeight::Semibold)
                            .color(teal)
                            .id(day::id!(budget_total)),
                        label(res::str::budget_caption())
                            .font(Font::Callout)
                            .color(muted),
                        progress(0.72),
                        label(res::str::budget_progress(0.72))
                            .font(Font::Caption)
                            .color(muted)
                            .id(day::id!(budget_progress)),
                        row((
                            column((
                                label(res::str::travelers(travelers))
                                    .font(Font::Headline)
                                    .color(ink)
                                    .id(day::id!(traveler_count)),
                                label(move || {
                                    res::str::per_person(1284.50 / travelers.get() as f64).format()
                                })
                                .font(Font::Caption)
                                .color(muted)
                                .id(day::id!(per_person)),
                            ))
                            .align(HAlign::Leading)
                            .spacing(Space::XS)
                            .grow(),
                            button(res::str::remove_traveler())
                                .icon(Symbol::Remove)
                                .icon_only()
                                .bordered()
                                .enabled(move || travelers.get() > 1)
                                .action(move || travelers.update(|n| *n = (*n - 1).max(1)))
                                .id(day::id!(remove_traveler)),
                            button(res::str::add_traveler())
                                .icon(Symbol::Add)
                                .icon_only()
                                .bordered()
                                .enabled(move || travelers.get() < 8)
                                .action(move || travelers.update(|n| *n = (*n + 1).min(8)))
                                .id(day::id!(add_traveler)),
                        ))
                        .spacing(Space::S),
                    ))
                    .align(HAlign::Leading)
                    .spacing(Space::S)
                    .padding(Space::L)
                    .background(budget_surface)
                    .corner_radius(16.0),
                    column((
                        heading(res::str::packing()),
                        row((
                            label(res::str::passport()).color(ink),
                            spacer(),
                            toggle(passport)
                                .a11y(|a| a.label(res::str::passport()))
                                .id(day::id!(passport)),
                        )),
                        row((
                            label(res::str::camera()).color(ink),
                            spacer(),
                            toggle(camera)
                                .a11y(|a| a.label(res::str::camera()))
                                .id(day::id!(camera)),
                        )),
                        row((
                            label(res::str::jacket()).color(ink),
                            spacer(),
                            toggle(jacket)
                                .a11y(|a| a.label(res::str::jacket()))
                                .id(day::id!(jacket)),
                        )),
                        label(move || {
                            res::str::packed(
                                i64::from(passport.get())
                                    + i64::from(camera.get())
                                    + i64::from(jacket.get()),
                            )
                            .format()
                        })
                        .font(Font::Caption)
                        .color(muted)
                        .id(day::id!(packing_count)),
                    ))
                    .align(HAlign::Leading)
                    .spacing(Space::S)
                    .padding(Space::L)
                    .background(surface)
                    .corner_radius(16.0),
                ))
                .align(HAlign::Leading)
                .spacing(Space::M)
                .grow(),
            ))
            .align(VAlign::Top)
            .stack_at(WidthClass::Compact)
            .spacing(Space::M),
            when(
                move || day::locale().get().starts_with("ar") != launched_rtl,
                || {
                    label(res::str::restart_direction())
                        .font(Font::Callout)
                        .color(teal)
                        .id(day::id!(direction_note))
                },
            ),
            label(res::str::demo_note())
                .id(day::id!(demo_note))
                .font(Font::Caption)
                .color(muted),
        ))
        .align(HAlign::Leading)
        .spacing(Space::L)
        .padding(Space::L)
        .max_width(1120.0)
        .grow(),
    )
    .background(paper)
    .id(day::id!(travel_screen))
}

fn itinerary_item<A, B, C>(
    icon: VectorName,
    time: impl IntoText<A>,
    title: impl IntoText<B>,
    detail: impl IntoText<C>,
) -> impl Piece {
    row((
        vector(icon)
            .decorative()
            .frame(42.0, 42.0)
            .padding(Space::S)
            .background(sand)
            .corner_radius(12.0),
        column((
            label(time)
                .font(Font::Caption)
                .weight(FontWeight::Semibold)
                .color(teal),
            label(title).font(Font::Headline).color(ink),
            label(detail).font(Font::Callout).color(muted),
        ))
        .align(HAlign::Leading)
        .spacing(Space::XS)
        .grow(),
    ))
    .spacing(Space::M)
    .align(VAlign::Top)
}

// Store trip data in metric units; convert only for the sample's US English view.
fn display_distance(kilometers: f64, us: bool) -> f64 {
    if us {
        kilometers / 1.609_344
    } else {
        kilometers
    }
}

fn display_temperature(celsius: f64, us: bool) -> f64 {
    if us {
        celsius * 9.0 / 5.0 + 32.0
    } else {
        celsius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn us_units_convert_the_same_trip_data() {
        assert!((display_distance(6.4, true) - 3.976_775_630).abs() < 0.000_001);
        assert!((display_temperature(24.0, true) - 75.2).abs() < 0.000_001);
        assert_eq!(display_temperature(0.0, true), 32.0);
        assert_eq!(display_distance(6.4, false), 6.4);
        assert_eq!(display_temperature(24.0, false), 24.0);
    }
}
