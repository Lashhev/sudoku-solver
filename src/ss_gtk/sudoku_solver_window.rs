mod imp;
use gtk::gio;
use gtk::prelude::*;

gtk::glib::wrapper! {
    pub struct SudokuSolverWindow(ObjectSubclass<imp::SudokuSolverWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl SudokuSolverWindow {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        gtk::glib::Object::builder()
            .property("application", app)
            .property("default-width", 800)
            .property("default-height", 800)
            // .property("title", )
            .build()
    }
}
