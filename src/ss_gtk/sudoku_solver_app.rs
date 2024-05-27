mod imp;

use gtk::prelude::*;
use gtk::{
    gdk, gio,
    glib::{self, subclass::types::*},
};
static LOGO_SVG: &[u8] = include_bytes!("../../resources/about_icon.png");
glib::wrapper! {
    pub struct SudokuSolverApplication(ObjectSubclass<imp::SudokuSolverApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for SudokuSolverApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", "org.gtk_rs.sudoku_solver")
            .build()
    }
}

impl SudokuSolverApplication {
    fn add_actions(&self) {
        let callback = move |app: &Self, _: &gio::SimpleAction, _: Option<&gtk::glib::Variant>| {
            let transient_for = app.imp().top_level_window();
            let program_name = "Sudoku Solver";
            let bytes = glib::Bytes::from_static(LOGO_SVG);
            let logo = gdk::Texture::from_bytes(&bytes).expect("gtk-rs.svg to load");
            let dialog = gtk::AboutDialog::builder()
                .transient_for(transient_for.as_ref().unwrap())
                .modal(true)
                .program_name(program_name)
                .version("0.1.0")
                .website("https://github.com/Lashhev/sudoku_solver_lib")
                .license_type(gtk::License::Bsd)
                .authors(["Andrew Lashchev"])
                .logo(&logo)
                .build();

            dialog.present();
        };
        let about = gio::ActionEntry::builder("about")
            .activate(callback)
            .build();

        let quit = gio::ActionEntry::builder("quit")
            .activate(
                |app: &Self, _: &gio::SimpleAction, _: Option<&gtk::glib::Variant>| app.quit(),
            )
            .build();
        self.add_action_entries([about, quit]);
    }
}
