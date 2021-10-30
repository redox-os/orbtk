use orbtk::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MyStruct {
    name: String,
    ext: String,
    type_s: String,
    domain: String,
}

type TableData = Vec<MyStruct>;

#[derive(Default, AsAny)]
struct MainViewState;

impl State for MainViewState {
    fn init(&mut self, _reg: &mut Registry, ctx: &mut Context) {
        let table_data = vec![
            MyStruct {
                name: "Rust".to_string(),
                ext: ".rs".to_string(),
                type_s: "static".to_string(),
                domain: "System programming, general purpose".to_string(),
            },
            MyStruct {
                name: "Javascript".to_string(),
                ext: ".js".to_string(),
                type_s: "dynamic".to_string(),
                domain: "Web, mobile".to_string(),
            },
            MyStruct {
                name: "PHP".to_string(),
                ext: ".php".to_string(),
                type_s: "dynamic".to_string(),
                domain: "Web backend".to_string(),
            },
            MyStruct {
                name: "Java".to_string(),
                ext: ".java".to_string(),
                type_s: "static".to_string(),
                domain: "General purpose, mobile, web backend, fintech".to_string(),
            },
            MyStruct {
                name: "C++".to_string(),
                ext: ".cc".to_string(),
                type_s: "static".to_string(),
                domain: "System programming, Object oriented".to_string(),
            },
            MyStruct {
                name: "C".to_string(),
                ext: ".c".to_string(),
                type_s: "static".to_string(),
                domain: "Programming, ABI Stack".to_string(),
            },
        ];
        ctx.widget().set::<TableData>("data", table_data);
        ctx.child_from_index(0).set::<usize>("row_count", 6);
    }
}

widget!(MainView<MainViewState>{
    data: TableData
});

impl Template for MainView {
    fn template(self, id: Entity, build_context: &mut BuildContext) -> Self {
        self.name("MainView").child(
            TableView::new()
                .column("Name", "col_name")
                .column("File extension", "col_ext")
                .column("Type system", "col_ts")
                .column("Domain", "col_domain")
                .data_source(id.0)
                .row_builder(move |build_context, row_index: usize, row_cells| {
                    let row = build_context
                        .get_widget(id)
                        .get::<TableData>("data")
                        .get(row_index)
                        .unwrap()
                        .clone();

                    row_cells.push(
                        Container::new()
                            .child(
                                TextBlock::new()
                                    .font_size(12.0)
                                    .text(row.name)
                                    .build(build_context),
                            )
                            .build(build_context),
                    );
                    row_cells.push(
                        Container::new()
                            .child(
                                TextBlock::new()
                                    .font_size(12.0)
                                    .text(row.ext)
                                    .build(build_context),
                            )
                            .build(build_context),
                    );
                    row_cells.push(
                        Container::new()
                            .child(
                                TextBlock::new()
                                    .font_size(12.0)
                                    .text(row.type_s)
                                    .build(build_context),
                            )
                            .build(build_context),
                    );
                    row_cells.push(
                        Container::new()
                            .child(
                                TextBlock::new()
                                    .font_size(12.0)
                                    .text(row.domain)
                                    .build(build_context),
                            )
                            .build(build_context),
                    );
                })
                .on_sort(
                    |sort_predicate, sort_direction, data_source, ctx| match sort_predicate {
                        "col_name" => match sort_direction {
                            TableSortDirection::Ascending => {
                                ctx.get_widget(data_source)
                                    .get_mut::<TableData>("data")
                                    .sort_by(|a, b| a.name.cmp(&b.name));
                            }
                            TableSortDirection::Descending => {
                                ctx.get_widget(data_source)
                                    .get_mut::<TableData>("data")
                                    .sort_by(|a, b| b.name.cmp(&a.name));
                            }
                        },
                        "col_ext" => match sort_direction {
                            TableSortDirection::Ascending => {
                                ctx.get_widget(data_source)
                                    .get_mut::<TableData>("data")
                                    .sort_by(|a, b| a.ext.cmp(&b.ext));
                            }
                            TableSortDirection::Descending => {
                                ctx.get_widget(data_source)
                                    .get_mut::<TableData>("data")
                                    .sort_by(|a, b| b.ext.cmp(&a.ext));
                            }
                        },
                        "col_ts" => match sort_direction {
                            TableSortDirection::Ascending => {
                                ctx.get_widget(data_source)
                                    .get_mut::<TableData>("data")
                                    .sort_by(|a, b| a.type_s.cmp(&b.type_s));
                            }
                            TableSortDirection::Descending => {
                                ctx.get_widget(data_source)
                                    .get_mut::<TableData>("data")
                                    .sort_by(|a, b| b.type_s.cmp(&a.type_s));
                            }
                        },
                        "col_domain" => match sort_direction {
                            TableSortDirection::Ascending => {
                                ctx.get_widget(data_source)
                                    .get_mut::<TableData>("data")
                                    .sort_by(|a, b| a.domain.cmp(&b.domain));
                            }
                            TableSortDirection::Descending => {
                                ctx.get_widget(data_source)
                                    .get_mut::<TableData>("data")
                                    .sort_by(|a, b| b.domain.cmp(&a.domain));
                            }
                        },
                        _ => {
                            println!("no match");
                        }
                    },
                )
                .build(build_context),
        )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .h_align("center")
                .title("TableView example")
                .position((100.0, 100.0))
                .size(1075.0, 615.0)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
