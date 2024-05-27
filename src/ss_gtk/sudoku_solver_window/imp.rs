use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::option::Option;
// use sudoku_solver_lib::SudokuGrid;

type SudokuCell = Rc<gtk::Entry>;
type SudokuPlate = Vec<Vec<SudokuCell>>;//[[SudokuCell;3];3];
type SudokuField = Vec<Vec<SudokuPlate>>;

// fn is_non_ascii_digit(c: char) -> bool {
//     !c.is_ascii_digit()
// }
// fn entry_disallow(entry: &gtk::Entry, pattern: fn(char) -> bool) {
//     entry.connect_insert_text(move |entry, text, position| {
//         if text.contains(pattern) {
//             glib::signal::signal_stop_emission_by_name(entry, "insert-text");
//             entry.insert_text(&text.replace(pattern, ""), position);
//         }
//     });
// }


#[derive(Default)]
pub struct SudokuSolverWindow {
    sudoku_field : RefCell<Option<SudokuField>>
}

#[glib::object_subclass]
impl ObjectSubclass for SudokuSolverWindow {
    const NAME: &'static str = "SudokuSolverWindow";
    type Type = super::SudokuSolverWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl SudokuSolverWindow {
    fn create_upper_pannel(&self) -> gtk::Box {
        let table = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        table
    }

    // fn sudoku_field2sudoku_grid(&self) -> SudokuGrid
    // {
    //     let mut grid = SudokuGrid::default();
    //     for row_index in 0..3
    //     {
    //         for col_index in 0..3
    //         {
    //             for i in 0..3
    //             {
    //                 for j in 0..3
    //                 {
    //                     let value = self.sudoku_field.borrow().as_ref().unwrap()[row_index][col_index][i][j].text().to_string().parse::<u8>().expect("Failed to parse");
    //                     grid[(i*row_index, j*col_index)] = value;
    //                 }
    //             }
    //         }   
    //     }
    //     grid
    // }

    // fn sudoku_grid2sudoku_field(&self, grid: &SudokuGrid)
    // {
    //     for row_index in 0..3
    //     {
    //         for col_index in 0..3
    //         {
    //             for i in 0..3
    //             {
    //                 for j in 0..3
    //                 {
    //                     self.sudoku_field.borrow_mut().as_ref().unwrap()[row_index][col_index][i][j].set_text(grid[(i*row_index, j*col_index)].to_string().as_str());
    //                 }
    //             }
    //         }   
    //     }
    // }

    fn create_3x3_plate(&self) -> (SudokuPlate, gtk::Box)
    {
        let mut plate: SudokuPlate = SudokuPlate::new();
        // let mut sudoku_plate = SudokuPlate::default();
        let col_layout = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .homogeneous(true)
            .build();
        for _row_index in 0..3{
            let row_layout = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .homogeneous(true)
                .build();
            let mut row = Vec::<SudokuCell>::new();
            for _col_index in 0..3{
                let lbl1 = gtk::Entry::builder().halign(gtk::Align::Center)
                    // .height_request(20)
                    // .width_request(20)
                    // .width_chars(30)
                    // .max_width_chars(30)
                    .max_width_chars(2)
                    .max_length(1)
                    .input_purpose(gtk::InputPurpose::Digits)
                    .build();
                // lbl1.set_property("width", 30);
                // lbl1.set_property("height", 30);
                // entry_disallow(&lbl1, is_non_ascii_digit);
                row.push(Rc::new(lbl1.clone()));
                row_layout.append(&lbl1);  
            }
            plate.push(row);
            col_layout.append(&row_layout);
        }
        (plate, col_layout)
    }

    fn create_field(&self) -> gtk::Box {
        let mut sudoku_field = SudokuField::new();
        let lay = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .homogeneous(true)
            .build();
        let field_layout = gtk::Grid::builder()
            .orientation(gtk::Orientation::Horizontal)
            .row_spacing(2)
            .column_spacing(2)
            .row_homogeneous(true)
            .column_homogeneous(true)
            // .valign(gtk::Align::Center)
            // .halign(gtk::Align::Center)
            .build();
        for row in 0..3{
            let mut row_plates = Vec::<SudokuPlate>::new();
            for col in 0..3{
                let (plate, rect_layout) = self.create_3x3_plate();
                field_layout.attach(&rect_layout, col, row, 1, 1);
                row_plates.push(plate);
            }
            sudoku_field.push(row_plates);
        }
        *self.sudoku_field.borrow_mut() = Some(sudoku_field);
        
        // lay.append(&gtk::Box::new(gtk::Orientation::Horizontal, 0));
        lay.append(&field_layout); 
        // lay.append(&gtk::Box::new(gtk::Orientation::Horizontal, 0));
        lay
    }

    fn create_lower_pannel(&self) -> gtk::Box {
        let lower_pannel_layout = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(12)
            .halign(gtk::Align::End)
            .valign(gtk::Align::End)
            .homogeneous(false)
            .build();
        let close_button_label = "Close";
        let clear_button_label = "Clear";
        let solve_button_label = "Solve";
        let close_btn = gtk::Button::builder().label(close_button_label).build();
        let solve_btn = gtk::Button::builder().label(solve_button_label).build();
        let clear_btn = gtk::Button::builder().label(clear_button_label).build();
        // solve_btn.connect_clicked(clone!(@weak window => move |_| window.destroy()));
        lower_pannel_layout.append(&solve_btn);
        lower_pannel_layout.append(&clear_btn);
        lower_pannel_layout.append(&close_btn);
        lower_pannel_layout
    }

    // fn build_ui(&self) {}
}

impl ObjectImpl for SudokuSolverWindow {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().set_title(Some("Sudoku Solver"));
        let global_layout = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .valign(gtk::Align::Fill)
            .build();
        self.obj().set_child(Some(&global_layout));
        let upper_pannel_layout = self.create_upper_pannel();
        let central_field_layout = self.create_field();
        let lower_pannel_layout = self.create_lower_pannel();
        global_layout.append(&upper_pannel_layout);
        global_layout.append(&central_field_layout);
        global_layout.append(&lower_pannel_layout);
        // global_layout.set_homogeneous(true);
        
        // self.obj().set_show_menubar(true);
    }
}
impl WidgetImpl for SudokuSolverWindow {}
impl WindowImpl for SudokuSolverWindow {}
impl ApplicationWindowImpl for SudokuSolverWindow {}
