use slint::Model;
use sudoku_solver_lib::{SudokuGrid, SudokuSolver};
use slint::{ModelRc, VecModel};
use slint::SharedString;
use rfd::AsyncFileDialog;
use futures::executor::block_on;

slint::slint!{
 import {LineEdit, VerticalBox, HorizontalBox, GridBox, AboutSlint, Button} from "std-widgets.slint";
 

 component ControlButton inherits Rectangle {
    in-out property <string> text <=> txt.text;
    in property <brush> pressed-color;
    in property <brush> hover-color;
    callback clicked <=> touch.clicked;
    // border-radius: root.height / 2;
    border-width: 1px;
    border-color: root.background.darker(25%);
    background: touch.pressed ? pressed-color.with-alpha(0.5) : touch.has-hover ? hover-color :  pressed-color;
    height: 50px;
    txt := Text {
        color: touch.pressed ? #fff : #eee;
        font-size: 24px;
    }
    touch := TouchArea { }
}

component IOButtons inherits Rectangle {
    in-out property <string> text <=> txt.text;
    
    callback clicked <=> touch.clicked;
    height: 40px;
    width: 40px;
    border-width: 1px;
    txt := Text {
        color: touch.pressed ? #fff : #eee;
        font-size: 24px;
    }
    touch := TouchArea { }
}


export component ControlPannel inherits HorizontalBox {
    callback solve-btn-clicked <=> solve-btn.clicked;
    callback clear-btn-clicked <=> clear-btn.clicked;
    solve-btn := ControlButton{
        text: "Solve";
        pressed-color: Colors.green;
        hover-color: Colors.green.darker(25%);
    }
    clear-btn := ControlButton{
        text: "Clear";
        pressed-color: Colors.red;
        hover-color: Colors.red.darker(50%);
    }
}

export component SudokuCell inherits LineEdit {
    in property <length> cell-size: 40px; 
    max-width: root.cell-size;
    max-height: root.cell-size;
    min-width: root.cell-size;
    min-height: root.cell-size;
    // width: root.cell-size;
    // height: root.cell-size;
    font-size: 24px;
    input-type: InputType.decimal; 
    placeholder-text: "0";
    text: "";
    horizontal-alignment: TextHorizontalAlignment.center;
}


export component SudokuPlate inherits GridLayout {
    in property <length> cell-size: 70px;
    spacing: 0px;
    public function get-field() -> [string] {
        return [c00.text, c01.text, c02.text, c10.text, c11.text, c12.text, c20.text, c21.text, c22.text];
    }
    public function set-field(values: [string]) {
        c00.text = values[0];
        c01.text = values[1];
        c02.text = values[2];
        c10.text = values[3];
        c11.text = values[4];
        c12.text = values[5];
        c20.text = values[6];
        c21.text = values[7];
        c22.text = values[8];
    }

    Row{
        c00 := SudokuCell{cell-size: root.cell-size;}
        c01 := SudokuCell{cell-size: root.cell-size;}
        c02 := SudokuCell{cell-size: root.cell-size;}
    }
    Row{
        c10 := SudokuCell{cell-size: root.cell-size;}
        c11 := SudokuCell{cell-size: root.cell-size;}
        c12 := SudokuCell{cell-size: root.cell-size;}
    }
    Row{
       c20 := SudokuCell{cell-size: root.cell-size;}
       c21 := SudokuCell{cell-size: root.cell-size;}
       c22 := SudokuCell{cell-size: root.cell-size;}
    }
}

export component SudokuField inherits GridBox {
    spacing: 5px;
    public function get-field() -> [[string]] {
        return [p00.get-field(), p01.get-field(), p02.get-field(), p10.get-field(), p11.get-field(), p12.get-field(), p20.get-field(), p21.get-field(), p22.get-field()];
    }
    public function set-field(values: [[string]]) {
        p00.set-field(values[0]);
        p01.set-field(values[1]);
        p02.set-field(values[2]);
        p10.set-field(values[3]);
        p11.set-field(values[4]);
        p12.set-field(values[5]);
        p20.set-field(values[6]);
        p21.set-field(values[7]);
        p22.set-field(values[8]);
    }
    Row{
        p00 := SudokuPlate {}
        p01 := SudokuPlate{}
        p02 := SudokuPlate {}
    }
    Row{
        p10 := SudokuPlate {}
        p11 := SudokuPlate{}
        p12 := SudokuPlate {}
    }
    Row{
        p20 := SudokuPlate {}
        p21 := SudokuPlate{}
        p22 := SudokuPlate {}
    }
}

export component LineStatus inherits LineEdit{
    text: "";
    font-size: 24px;
    height: 40px;
    width: 500px;
    input-type: InputType.decimal; 
    horizontal-alignment: TextHorizontalAlignment.center;
}

export component MainWindow inherits Window {
    title: "Sudoku Solver";
    callback solve-btn-clicked;
    callback load-from-file;
    callback save-to-file;
    property<[[string]]> zeros: [["", "", "", "", "", "", "", "", ""],
                                 ["", "", "", "", "", "", "", "", ""],
                                 ["", "", "", "", "", "", "", "", ""],
                                 ["", "", "", "", "", "", "", "", ""],
                                 ["", "", "", "", "", "", "", "", ""],
                                 ["", "", "", "", "", "", "", "", ""],
                                 ["", "", "", "", "", "", "", "", ""],
                                 ["", "", "", "", "", "", "", "", ""],
                                 ["", "", "", "", "", "", "", "", ""]];

    public function print_message(m: string) {status-line.text = m}

    public function get-field() -> [[string]] {
        return sudoku-field.get-field();
    }
    public function set-fields(fields: [[string]])
    {
        sudoku-field.set-field(fields)
    }

    VerticalBox {
        HorizontalBox {
            status-line := LineStatus {
                enabled: false;
                text: "Input your puzzle";
            }
            load-button := IOButtons {
                
                Image {
                    source: @image-url("resources/download.png");
                    image-fit: cover;
                    width: 100%;
                    height: 100%;

                }
                text: "";
                clicked => {root.load-from-file();}
            }
            save-button := IOButtons {
                
                Image {
                    source: @image-url("resources/diskette.png");
                    image-fit: cover;
                    width: 100%;
                    height: 100%;
                }
                text: "";
                clicked => {root.save-to-file();}
            }
        }
    
    sudoku-field := SudokuField {}
    control-pannel := HorizontalBox {
        solve-btn := ControlButton{
            text: "Solve";
            pressed-color: Colors.green;
            hover-color: Colors.green.darker(25%);
            clicked => {root.solve-btn-clicked();}
        }
        clear-btn := ControlButton{
            text: "Clear";
            pressed-color: Colors.red;
            hover-color: Colors.red.darker(50%);
            clicked => {
                root.set-fields(root.zeros);
                status-line.text = "Input your puzzle";
            }
        }
    }
    }
    
    
}
}


fn import_field_model(model: &ModelRc<ModelRc<SharedString>>) -> SudokuGrid
{
    let mut grid = SudokuGrid::default();
    let plates: Vec<ModelRc<SharedString>> = model.iter().collect();
    for plate_row in 0..3
    {
        for plate_col in 0..3
        {
            let plate: Vec<SharedString> = plates[plate_row*3+plate_col].iter().collect();
            for cell_row in 0..3
            {
                for cell_col in 0..3
                {
                    let value = plate[cell_row*3+cell_col].to_string().parse::<u8>();
                    if value.is_ok()
                    {
                        grid[(plate_row*3+cell_row, plate_col*3 + cell_col)] = value.unwrap();
                    }
                    else {
                        grid[(plate_row*3+cell_row, plate_col*3 + cell_col)] = 0;
                    }
                    
                }
            }
        }
    }
    grid.transpose()
}

fn export_field_model(grid: &SudokuGrid) -> ModelRc<ModelRc<SharedString>>
{
    let gridt = grid.transpose();
    // let plates: Vec<ModelRc<SharedString>> = model.iter().collect();
    
    let mut model_vec =  Vec::<ModelRc<SharedString>>::new();
    for plate_row in 0..3
    {
        for plate_col in 0..3
        {
            
            let mut cells = Vec::<String>::new();
            for cell_row in 0..3
            {
                for cell_col in 0..3
                {
                    cells.push(gridt[(plate_row*3+cell_row, plate_col*3 + cell_col)].to_string());
                }
            }
            let my_vec : Vec<SharedString> = cells.into_iter().map(Into::into).collect();
            let plate: ModelRc<SharedString> = ModelRc::<SharedString>::new(VecModel::from(my_vec));

            model_vec.push(plate);
        }
    }
    let model = ModelRc::<ModelRc<SharedString>>::new(VecModel::from(model_vec));
    model
}

fn main() {
    use slint::SharedString;
    let app = MainWindow::new().expect("Failed to create window");
    let weak1 = app.as_weak();
    
    app.on_solve_btn_clicked( move || {
        let app2 = weak1.upgrade().unwrap();
        let sudoku_data = app2.invoke_get_field();
        let grid = import_field_model(&sudoku_data);
        if grid == SudokuGrid::zeros()
        {
            app2.invoke_print_message(SharedString::from("Failed to solve the puzzle"));
            return;
        }
        let solution = SudokuSolver::solve(&grid);
        if solution.is_some()
        {
            let res = export_field_model(solution.as_ref().unwrap());
            app2.invoke_print_message(SharedString::from("Success"));
            app2.invoke_set_fields(res.clone());
        }
        else {
            app2.invoke_print_message(SharedString::from("Failed to solve the puzzle"));
        }
    });

    let weak2 = app.as_weak();
    app.on_load_from_file( move || {
        let app2 = weak2.upgrade().unwrap();
        let _future = async {
            let file = AsyncFileDialog::new()
                .add_filter("Json", &["json"])
                .set_directory("/home")
                .pick_file()
                .await;
            if file.is_some(){
                let data = file.as_ref().unwrap().read().await;
                let res: Result<SudokuGrid, serde_json::Error> = serde_json::from_slice(&data);
                if res.is_ok(){
                    let grid = export_field_model(&res.unwrap());
                    app2.invoke_set_fields(grid.clone());
                }
                
            }
            
        };
        block_on(_future);
    });

    let weak3 = app.as_weak();
    app.on_save_to_file( move || {
        let _future = async {
            let file = AsyncFileDialog::new()
                .add_filter("Json", &["json"])
                .set_directory("/home")
                .save_file()
                .await;
            if file.is_some(){
                let app2 = weak3.upgrade().unwrap();
                let sudoku_data = app2.invoke_get_field();
                let grid = import_field_model(&sudoku_data);
                let data = serde_json::to_vec(&grid).expect("Failed to parse");
                let res = file.as_ref().unwrap().write(&data).await;

                if res.is_err(){
                    app2.invoke_print_message(SharedString::from(res.err().unwrap().to_string()));
                }

            }  
        };
        block_on(_future);
    });
    let _ = app.run();
}
