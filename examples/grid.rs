extern crate orbtk;

use std::cell::Cell;

use orbtk::{Button, Grid, Label, Window, Rect};
use orbtk::traits::{Click, Place, Text};

fn main() {
    let mut window = Window::new(Rect::new(100, 100, 450, 400), "Grid example");

    let grid = Grid::new();
    grid.spacing(8, 8)
        .position(10, 55);
    window.add(&grid);

    let btn_insert = Button::new();
    {
        let _grid = grid.clone();
        btn_insert.size(11 + 15*8 + 11, 35)
            .position(10, 10)
            .text("Insert elements")
            .text_offset(11, 11)
            .on_click(move |_, _| {
                let label = Label::new();
                label.size(72, 16).text("Element 1");
                _grid.insert(0, 0, &label);

                let label = Label::new();
                label.size(72, 16).text("Element 2");
                _grid.insert(1, 0, &label);

                let label = Label::new();
                label.size(72, 16).text("Element 3");
                _grid.insert(0, 1, &label);

                let label = Label::new();
                label.size(72, 16).text("Element 4");
                _grid.insert(1, 1, &label);
            });
        window.add(&btn_insert);
    }

    let btn_clear = Button::new();

    {
        let _grid = grid.clone();

        btn_clear
            .size(11 + 14 * 8 + 11, 35)
            .position(10 + btn_insert.rect.get().width as i32 + 10, 10)
            .text("Clear elements")
            .text_offset(11, 11)
            .on_click(move |_, _| {
                _grid.clear();
            });
        window.add(&btn_clear);
    }

    {
        let _grid = grid.clone();
        let btn_remove = Button::new();
        btn_remove
            .size(11 + 14 * 8 + 11, 35)
            .position(btn_clear.rect.get().x + btn_clear.rect.get().width as i32 + 10, 10)
            .text("Remove element")
            .text_offset(11, 11)
            .on_click(move |_, _| _grid.remove(1, 0));
        window.add(&btn_remove);
    }

    let column_grid = Grid::new();
    column_grid.spacing(8, 8)
        .position(10, 255)
        .columns(4);
    window.add(&column_grid);

    let btn_add = Button::new();
    {
        let _grid = column_grid.clone();
        let element_counter = Cell::new(0);

        btn_add.size(11 + 12*8 + 11, 35)
            .position(10, 200)
            .text("Add elements")
            .text_offset(11, 11)
            .on_click(move |_, _| {
                let _element_counter = element_counter.get();
                let label = Label::new();
                label.size(80, 16).text(format!("Element {}",_element_counter));
                _grid.add(&label);
                element_counter.set(_element_counter + 1);
            });
        window.add(&btn_add);
    }

    window.exec();
}



