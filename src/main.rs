// Copyright 2024 3WEBS LLC
// SPDX-License-Identifier: GPL-3.0-or-later

use adw::prelude::*;
use webkit6::prelude::*;

use adw::{Application, ApplicationWindow, HeaderBar};
use gtk4::*;
use gdk::Display;
use glib::clone;
use url::Url;
use webkit6::WebView;

use lazy_static::lazy_static;
use toml::Value;

lazy_static! {
    static ref CARGO_MANIFEST: Value = {
        // Include and parse the Cargo.toml file at compile time.
        let cargo_toml_str = include_str!("../Cargo.toml");
        toml::from_str(cargo_toml_str).expect("Failed to parse Cargo.toml")
    };
}

fn main() {
    let application = Application::builder()
        .application_id(CARGO_MANIFEST["package"]["metadata"]["application_id"].as_str().unwrap())
        .build();

    application.connect_activate(|app| {
        // Create a new window
        let window = ApplicationWindow::builder()
            .application(app)
            .title(CARGO_MANIFEST["package"]["metadata"]["human_readable_name"].as_str().unwrap())
            .show_menubar(true)
            .maximized(true)
            .build();

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);

        // Create a progress bar to show loading progress
        let progress_bar = ProgressBar::builder()
            .show_text(true)
            .build();
        progress_bar.set_hexpand(true);
        progress_bar.set_vexpand(false);
        progress_bar.set_show_text(false);
        progress_bar.set_margin_top(0);
        progress_bar.set_margin_bottom(0);
        progress_bar.set_margin_start(0);
        progress_bar.set_margin_end(0);

        // Initialize the webview right away
        let webview = WebView::new();
        webview.set_hexpand(true);
        webview.set_vexpand(true);

        // And set webview settings
        let settings = webkit6::prelude::WebViewExt::settings(&webview).unwrap();

        // Static settings
        settings.set_allow_file_access_from_file_urls(true);
        settings.set_allow_modal_dialogs(true);
        settings.set_allow_top_navigation_to_data_urls(true);
        settings.set_allow_universal_access_from_file_urls(true);
        settings.set_auto_load_images(true);
        settings.set_default_charset("utf-8");
        settings.set_disable_web_security(false);
        settings.set_draw_compositing_indicators(false);
        settings.set_enable_back_forward_navigation_gestures(false);
        settings.set_enable_caret_browsing(false);
        settings.set_enable_developer_extras(true);
        settings.set_enable_dns_prefetching(true);
        settings.set_enable_encrypted_media(true);
        settings.set_enable_fullscreen(true);
        settings.set_enable_html5_database(true);
        settings.set_enable_html5_local_storage(true);
        settings.set_enable_hyperlink_auditing(false); // Disable for privacy
        settings.set_enable_javascript(true);
        settings.set_enable_javascript_markup(true);
        settings.set_enable_media(true);
        settings.set_enable_media_capabilities(true);
        settings.set_enable_media_stream(true);
        settings.set_enable_mediasource(true);
        settings.set_enable_mock_capture_devices(false); // Is there any good reason to enable this?
        settings.set_enable_page_cache(true);
        settings.set_enable_resizable_text_areas(false); // This has always been a pet peeve of mine
        settings.set_enable_site_specific_quirks(false);
        settings.set_enable_smooth_scrolling(true); // TODO: This should be configurable
        settings.set_enable_spatial_navigation(true);
        settings.set_enable_tabs_to_links(true);
        settings.set_enable_webaudio(true);
        settings.set_enable_webgl(true);
        settings.set_enable_webrtc(true);
        settings.set_enable_write_console_messages_to_stdout(false); // TODO: This should be configurable
        settings.set_hardware_acceleration_policy(webkit6::HardwareAccelerationPolicy::Always); // TODO: This should be configurable.
        settings.set_javascript_can_access_clipboard(false); // No good reason to enable this for now.
        settings.set_javascript_can_open_windows_automatically(false); // TODO: This should be configurable
        settings.set_media_playback_allows_inline(true);
        settings.set_media_playback_requires_user_gesture(true); // TODO: This should be configurable
        settings.set_print_backgrounds(true);
        settings.set_zoom_text_only(false); // TODO: This should be configurable
        settings.set_user_agent_with_application_details(
            Some(CARGO_MANIFEST["package"]["metadata"]["human_readable_name"].as_str().unwrap()),
            Some(CARGO_MANIFEST["package"]["version"].as_str().unwrap())
        );

        // Get font data from the system and set those as default
        let gtk_settings = Settings::for_display(&Display::default().unwrap());
        let font = gtk_settings.gtk_font_name();
        settings.set_cursive_font_family("serif"); // System default
        settings.set_fantasy_font_family("serif"); // System default
        settings.set_monospace_font_family("monospace"); // System default
        settings.set_pictograph_font_family("serif"); // System default
        settings.set_sans_serif_font_family("sans-serif"); // System default
        settings.set_serif_font_family("serif"); // System default
        if font.is_some() {
            // Font size is last part of the string
            let mut font_split = font.as_ref().unwrap().split(' ').collect::<Vec<&str>>();
            let font_size = font_split.pop().unwrap().parse::<u32>().unwrap();
            let font_family = font_split.join(" ");
            settings.set_default_font_family(&font_family);
            settings.set_default_font_size(font_size);
            settings.set_default_monospace_font_size(font_size);
        } else {
            settings.set_default_font_family("sans-serif"); // System default
            settings.set_default_font_size(16); // System default
            settings.set_default_monospace_font_size(16); // System default
        }

        webview.set_settings(&settings);

        // Header
        let header = HeaderBar::builder()
            .can_focus(true)
            .build();
        let url_bar = Entry::builder()
            .activates_default(true)
            .placeholder_text("Enter URL")
            .max_width_chars(256)
            .height_request(48)
            .can_focus(true)
            .focusable(true)
            .focus_on_click(true)
            .xalign(0.5)
            .build();
        
        // Make the URL bar behave like a combined search and address bar
        let url_bar_focus_controller = EventControllerFocus::new();
        url_bar_focus_controller.connect_enter(clone!(@weak url_bar, @weak webview => move |_| {
            let uri = webview.uri();
            url_bar.set_xalign(0.0);
            if let Some(uri) = uri {
                url_bar.set_text(&uri);
            }
        }));
        url_bar_focus_controller.connect_leave(clone!(@weak url_bar => move |_| {
            url_bar.set_xalign(0.5);
            url_bar.set_text("");
        }));
        url_bar.connect_activate(clone!(@weak webview => move |entry| {
            let text = entry.text();
            let url = Url::parse(&text);
            if let Ok(url) = url {
                if url.scheme() == "http" || url.scheme() == "https" {
                    webview.load_uri(&text);
                } else {
                    webview.load_uri(&format!("https://duckduckgo.com/?q={}", text));
                }
            } else {
                webview.load_uri(&format!("https://duckduckgo.com/?q={}", text));
            }
            webview.grab_focus();
        }));
        webview.connect_uri_notify(clone!(@weak url_bar => move |webview| {
            let uri = webview.uri();
            if let Some(uri) = uri {
                if uri != "" {
                    let url = Url::parse(&uri);
                    match url {
                        Ok(url) => {
                            url_bar.set_placeholder_text(Some(url.host_str().unwrap_or("")));
                        },
                        Err(_) => {
                            url_bar.set_placeholder_text(Some("Invalid URL"));
                        }
                    }
                }
            }
        }));
        webview.connect_title_notify(clone!(@weak window, @weak url_bar => move |webview| {
            let title = webview.title();
            if let Some(title) = title {
                if title != "" {
                    url_bar.set_placeholder_text(Some(&title));
                    window.set_title(Some(title.as_str()));
                }
            }
        }));

        // Search bar functionality
        url_bar.connect_notify(Some("text"), move |url_bar, _| {
            let text = url_bar.text();
            if text == "" {
                url_bar.set_icon_from_icon_name(EntryIconPosition::Primary, Some(""));
                return;
            }
            let url = Url::parse(&text);
            match url {
                Ok(url) => {
                    if url.scheme() == "http" || url.scheme() == "https" {
                        url_bar.set_icon_from_icon_name(EntryIconPosition::Primary, Some("network-server-symbolic"));
                    } else {
                        url_bar.set_icon_from_icon_name(EntryIconPosition::Primary, Some("system-search-symbolic"));
                    }
                },
                Err(_) => {
                    url_bar.set_icon_from_icon_name(EntryIconPosition::Primary, Some("system-search-symbolic"));
                }
            }
        });

        // Add back, forward, and refresh buttons
        url_bar.add_controller(url_bar_focus_controller);
        header.set_title_widget(Some(&url_bar));
        let back_button = Button::builder()
            .icon_name("go-previous-symbolic")
            .build();
        back_button.connect_clicked(clone!(@weak webview => move |_| {
            webview.go_back();
        }));
        webview.connect_load_changed(clone!(@weak back_button, @weak webview => move |_, _| {
            // Hide back button if there is no history
            if webview.can_go_back() {
                back_button.set_sensitive(true);
            } else {
                back_button.set_sensitive(false);
            }
        }));
        header.pack_start(&back_button);
        let forward_button = Button::builder()
            .icon_name("go-next-symbolic")
            .build();
        forward_button.connect_clicked(clone!(@weak webview => move |_| {
            webview.go_forward();
        }));
        webview.connect_load_changed(clone!(@weak forward_button, @weak webview => move |_, _| {
            // Hide forward button if there is no history
            if webview.can_go_forward() {
                forward_button.set_sensitive(true);
            } else {
                forward_button.set_sensitive(false);
            }
        }));
        header.pack_start(&forward_button);
        let refresh_button = Button::builder()
            .icon_name("view-refresh-symbolic")
            .build();
        refresh_button.connect_clicked(clone!(@weak webview => move |_| {
            webview.reload();
        }));
        header.pack_start(&refresh_button);

        // Progress bar manager
        webview.connect_load_changed(clone!(@weak progress_bar => move |_, load_event| {
            match load_event {
                webkit6::LoadEvent::Started => {
                    progress_bar.set_fraction(0.0);
                    progress_bar.show();
                },
                webkit6::LoadEvent::Redirected => {
                    progress_bar.set_fraction(0.5);
                },
                webkit6::LoadEvent::Committed => {
                    progress_bar.set_fraction(0.75);
                },
                webkit6::LoadEvent::Finished => {
                    progress_bar.set_fraction(0.0);
                    progress_bar.hide();
                },
                _ => {}
            }
        }));

        // Create the menu button and corresponding actions
        let menu = gio::Menu::new();
        menu.append(Some("About"), Some("app.about"));
        menu.append(Some("Quit"), Some("app.quit"));
        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(clone!(@weak window => move |_, _| {
            let about = adw::AboutWindow::builder()
                .transient_for(&window)
                .application_name(CARGO_MANIFEST["package"]["metadata"]["human_readable_name"].as_str().unwrap())
                .version(CARGO_MANIFEST["package"]["version"].as_str().unwrap())
                .website(CARGO_MANIFEST["package"]["homepage"].as_str().unwrap())
                .issue_url(CARGO_MANIFEST["package"]["metadata"]["issue_url"].as_str().unwrap())
                .developer_name(CARGO_MANIFEST["package"]["metadata"]["developer_name"].as_str().unwrap())
                .copyright(CARGO_MANIFEST["package"]["metadata"]["copyright"].as_str().unwrap())
                .license_type(gtk4::License::Gpl30)
                .developers(CARGO_MANIFEST["package"]["authors"].as_array().unwrap().iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>())
                .build();
            about.add_legal_section(
                "gtk-rs-core",
                Some("<a href=\"https://crates.io/crates/glib-build-tools\">Crate</a> <a href=\"https://github.com/gtk-rs/gtk-rs-core\">Source</a>"),
                gtk4::License::MitX11,
                Some("")
            );
            about.add_legal_section(
                "gtk4-rs",
                Some("<a href=\"https://crates.io/crates/gtk4\">Crate</a> <a href=\"https://github.com/gtk-rs/gtk4-rs\">Source</a>"),
                gtk4::License::MitX11,
                Some("")
            );
            about.add_legal_section(
                "libadwaita-rs",
                Some("<a href=\"https://crates.io/crates/libadwaita\">Crate</a> <a href=\"https://gitlab.gnome.org/World/Rust/libadwaita-rs\">Source</a>"),
                gtk4::License::MitX11,
                Some("")
            );
            about.add_legal_section(
                "webkit6-rs",
                Some("<a href=\"https://crates.io/crates/webkit6\"> Crate</a> <a href=\"https://gitlab.gnome.org/World/Rust/webkit6-rs\">Source</a>"),
                gtk4::License::MitX11,
                Some("")
            );
            about.add_legal_section(
                "rust-url",
                Some("<a href=\"https://crates.io/crates/url\">Crate</a> <a href=\"https://github.com/servo/rust-url\">Source</a>"),
                gtk4::License::MitX11, // OR Apache-2.0
                Some("")
            );
            about.present();
        }));
        app.add_action(&about_action);
        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(clone!(@weak window => move |_, _| {
            window.close();
        }));
        app.add_action(&quit_action);
        let menu_button = MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .menu_model(&menu)
            .build();
        header.pack_end(&menu_button);
        content.append(&header);

        // Add the progress bar to the content
        content.append(&progress_bar);

        // Set progress bar to show loading progress
        progress_bar.set_fraction(0.5);

        // Add the webview to the content
        webview.load_uri("https://example.com");
        content.append(&webview);

        // Start
        window.set_content(Some(&content));
        window.present();
    });

    application.run();
}
