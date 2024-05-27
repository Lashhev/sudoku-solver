use crate::ss_gtk::sudoku_solver_window::SudokuSolverWindow;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::cell::{Ref, RefCell};

#[derive(Default)]
pub struct SudokuSolverApplication {
    top_level_window: RefCell<Option<SudokuSolverWindow>>,
}
#[glib::object_subclass]
impl ObjectSubclass for SudokuSolverApplication {
    const NAME: &'static str = "SudokuSolverApplication";
    type Type = super::SudokuSolverApplication;
    type ParentType = gtk::Application;
}

impl SudokuSolverApplication {
    pub fn top_level_window(&self) -> Ref<Option<SudokuSolverWindow>> {
        self.top_level_window.borrow()
    }
    fn add_menu(&self) {
        let menubar = {
            let file_menu = {
                let new_menu_item = gio::MenuItem::new(Some("New"), None); // TODO:
                let open_menu_item = gio::MenuItem::new(Some("Open"), None); // TODO:
                let save_menu_item = gio::MenuItem::new(Some("Save"), None); // TODO:
                let quit_menu_item = gio::MenuItem::new(Some("Quit"), Some("app.quit")); // TODO:
                let about_menu_item = gio::MenuItem::new(Some("About"), Some("app.about")); // TODO:

                let file_menu = gio::Menu::new();
                file_menu.append_item(&new_menu_item);
                file_menu.append_item(&open_menu_item);
                file_menu.append_item(&save_menu_item);
                file_menu.append_item(&quit_menu_item);
                file_menu.append_item(&about_menu_item);
                file_menu
            };
            let menubar = gio::Menu::new();
            menubar.append_submenu(Some("File"), &file_menu);

            menubar
        };

        self.obj().set_menubar(Some(&menubar));
    }
    fn add_toplevel_window(&self) {
        // We create our window at activation stage
        self.top_level_window
            .replace(Some(SudokuSolverWindow::new(&*self.obj())));
        self.top_level_window
            .borrow()
            .as_ref()
            .unwrap()
            .set_show_menubar(true);
        // let window = ApplicationWindow::new(&*self.obj());
        self.top_level_window.borrow().as_ref().unwrap().present();
    }

    // fn add_about_dialog(&self) {}
}

impl ObjectImpl for SudokuSolverApplication {}
impl ApplicationImpl for SudokuSolverApplication {
    fn activate(&self) {
        self.parent_activate();
        self.add_toplevel_window();
    }
    fn startup(&self) {
        self.parent_startup();
        self.obj().add_actions();
        self.add_menu();
    }
}
impl GtkApplicationImpl for SudokuSolverApplication {}
