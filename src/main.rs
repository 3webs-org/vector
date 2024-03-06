use adw::prelude::*;
use webkit6::prelude::*;

use adw::{Application, ApplicationWindow, HeaderBar};
use gtk4::*;
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
        header.pack_start(&back_button);
        let forward_button = Button::builder()
            .icon_name("go-next-symbolic")
            .build();
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
