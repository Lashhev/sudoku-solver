use gtk::{glib, prelude::*};
use sudoku_solver::ss_gtk::sudoku_solver_app::SudokuSolverApplication;

fn main() -> glib::ExitCode {
    let app = SudokuSolverApplication::default();
    app.run()
}
