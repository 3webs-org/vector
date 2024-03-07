use adw::prelude::*;
use webkit6::prelude::*;

use adw::{Application, ApplicationWindow, HeaderBar};
use gtk4::*;
use gdk::Display;
use glib::clone;
use url::Url;
use webkit6::WebView;

fn main() {
    let application = Application::builder()
        .application_id("org.3webs.vanadium")
        .build();

    application.connect_activate(|app| {
        // Create a new window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Vanadium")
            .show_menubar(true)
            .maximized(true)
            .build();

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);

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
        settings.set_enable_offline_web_application_cache(true);
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

        // Vanadium user agent
        let vanadium_version = &env!("CARGO_PKG_VERSION");
        settings.set_user_agent_with_application_details("Vanadium", vanadium_version);

        // Get font data from the system and set those as default
        let gtk_settings = Settings::for_display(&Display::default().unwrap());
        let font = gtk_settings.gtk_font_name();
        settings.set_cursive_font_family("serif"); // System default
        settings.set_fantasy_font_family("serif"); // System default
        settings.set_monospace_font_family("monospace"); // System default
        settings.set_pictograph_font_family("serif"); // System default
        settings.set_sans_serif_font_family("sans-serif"); // System default
        settings.set_serif_font_family("serif"); // System default
        if font != None {
            // Font size is last part of the string
            let mut font_split = font.as_ref().unwrap().split(" ").collect::<Vec<&str>>();
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
        let url_bar = SearchEntry::builder()
            .activates_default(true)
            .placeholder_text("Enter URL")
            .max_width_chars(256)
            .height_request(48)
            .can_focus(true)
            .focusable(true)
            .focus_on_click(true)
            .xalign(0.5)
            .build();
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
            let uri = entry.text();
            webview.load_uri(&uri);
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
        let settings_button = Button::builder()
            .icon_name("open-menu-symbolic")
            .build();
        settings_button.set_sensitive(false); // Not implemented yet
        header.pack_end(&settings_button);
        content.append(&header);

        // Add the webview to the content
        webview.load_uri("https://example.com");
        content.append(&webview);

        // Start
        window.set_content(Some(&content));
        window.present();
    });

    application.run();
}
