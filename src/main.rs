#![windows_subsystem = "windows"]

mod sa_core;

use eframe::egui::{self, CentralPanel};
use egui::{
    containers::Frame,
    Button, Color32, Context, FontData, FontDefinitions,
    FontFamily::{self, Proportional},
    FontId, Key, Margin, Rounding, ScrollArea, TextEdit,
    TextStyle::*,
    TopBottomPanel, Vec2, Visuals,
};
use sa_core::{
    sa_export, sa_generate, SAGeneratorOptions, SAOperation, SAProblemInputBuffer, SAValidity,
};

#[derive(Default)]
struct SAGui {
    status: String,

    buffer_space_addition: SAProblemInputBuffer,
    buffer_space_subtraction: SAProblemInputBuffer,
    buffer_space_multiplication: SAProblemInputBuffer,
    buffer_space_division: SAProblemInputBuffer,

    addition: SAGeneratorOptions,
    subtraction: SAGeneratorOptions,
    multiplication: SAGeneratorOptions,
    division: SAGeneratorOptions,
}

impl SAGui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        setup_text_styles(&cc.egui_ctx);

        Self {
            status: "Ready".to_owned(),

            buffer_space_addition: SAProblemInputBuffer::new(),
            buffer_space_subtraction: SAProblemInputBuffer::new(),
            buffer_space_multiplication: SAProblemInputBuffer::new(),
            buffer_space_division: SAProblemInputBuffer::new(),

            addition: SAGeneratorOptions {
                problems_num: 10i32,
                problems_floor: 2i32,
                problems_ceiling: 250i32,
                allow_negatives: false,
                operation: SAOperation::Addition,
            },
            subtraction: SAGeneratorOptions {
                problems_num: 10i32,
                problems_floor: 2i32,
                problems_ceiling: 250i32,
                allow_negatives: false,
                operation: SAOperation::Subtraction,
            },
            multiplication: SAGeneratorOptions {
                problems_num: 10i32,
                problems_floor: 2i32,
                problems_ceiling: 250i32,
                allow_negatives: false,
                operation: SAOperation::Multiplication,
            },
            division: SAGeneratorOptions {
                problems_num: 10i32,
                problems_floor: 2i32,
                problems_ceiling: 250i32,
                allow_negatives: false,
                operation: SAOperation::Division,
            },
        }
    }
}

impl eframe::App for SAGui {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals {
            override_text_color: Some(Color32::from_rgb(220, 220, 220)),
            ..Visuals::default()
        });

        TopBottomPanel::top("SA_TopPanel_Main")
            .frame(Frame {
                outer_margin: Margin {
                    left: 12.0,
                    right: 12.0,
                    top: 12.0,
                    bottom: 12.0,
                },
                ..Frame::default()
            })
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.heading("SimAri");
                ui.label("A Simple Arithmetic Generator");
            });

        TopBottomPanel::bottom("SA_BottomPanel_Main")
            .frame(Frame {
                outer_margin: Margin {
                    left: 12.0,
                    right: 12.0,
                    top: 12.0,
                    bottom: 12.0,
                },
                ..Frame::default()
            })
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.label("Made by ParasiteDelta");
                ui.label(
                    "Check the Github repo for updates, issues, or comments @ParasiteDelta/simari",
                );
            });

        CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(1.6);
            ScrollArea::vertical().show(ui, |ui| {
                ui.set_width(ctx.screen_rect().width());
                ui.add_space(16.0);
                ui.label(format!("Status: {0}", &self.status));
                ui.add_space(16.0);
                ui.label("Hello, and welcome to SimAri!");
                ui.label("Please select an operation from below:");
                ui.add_space(16.0);

                //---===---===---===---===---===
                //Segment: Addition Generation
                ui.collapsing("Generate Addition Problems", |ui| {
                    let top_spacing = 8.0;
                    let bottom_spacing = 8.0;

                    //Section: Set Amount of Generated Problems
                    ui.add_space(top_spacing);
                    ui.label("The number of addition problems to generate (default: 10)");
                    let addition_resp_num_of = ui.add(TextEdit::singleline(
                        &mut self.buffer_space_addition.num_of_problems,
                    ));

                    if addition_resp_num_of.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter))
                    {
                        println!(
                            "Addition Num Of Problems Input: {0}",
                            &self.buffer_space_addition.num_of_problems
                        );
                        match check_input_num(&self.buffer_space_addition.num_of_problems) {
                            SAValidity::ValidNumber(n) => {
                                println!("Conversion successful, valid number!");
                                self.addition.problems_num = n;
                            }
                            SAValidity::InvalidNumber => {
                                println!("Conversion unsuccessful, invalid number!");
                            }
                        }
                    }

                    //Section: Set Generation Floor
                    ui.add_space(top_spacing);
                    ui.label("The lowest possible number to generate (default: 2)");
                    let addition_resp_floor =
                        ui.add(TextEdit::singleline(&mut self.buffer_space_addition.floor));

                    if addition_resp_floor.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                        println!(
                            "Addition Floor Input: {0}",
                            &self.buffer_space_addition.floor
                        );
                        match check_input_num(&self.buffer_space_addition.floor) {
                            SAValidity::ValidNumber(n) => {
                                println!("Conversion successful, valid number!");
                                self.addition.problems_floor = n;
                            }
                            SAValidity::InvalidNumber => {
                                println!("Conversion unsuccessful, invalid number!");
                            }
                        }
                    }

                    //Section: Set Generation Ceiling
                    ui.add_space(top_spacing);
                    ui.label("The highest possible number to generate (default: 250)");
                    let addition_resp_ceiling = ui.add(TextEdit::singleline(
                        &mut self.buffer_space_addition.ceiling,
                    ));

                    if addition_resp_ceiling.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter))
                    {
                        println!(
                            "Addition Ceiling Input: {0}",
                            &self.buffer_space_addition.ceiling
                        );
                        match check_input_num(&self.buffer_space_addition.ceiling) {
                            SAValidity::ValidNumber(n) => {
                                println!("Conversion successful, valid number!");
                                self.addition.problems_ceiling = n;
                            }
                            SAValidity::InvalidNumber => {
                                println!("Conversion unsuccessful, invalid number!");
                            }
                        }
                    }

                    //Section: Allow Negatives
                    ui.add_space(top_spacing);
                    ui.label("Whether we allow negative numbers to be generated (default: false)");
                    ui.checkbox(&mut self.addition.allow_negatives, "Allow Negative Numbers");

                    ui.add_space(bottom_spacing);

                    //Section: Generate Button
                    if ui.add(generate_button("Generate Problems")).clicked() {
                        self.status =
                            "Generating addition problems with parameters, standby...".to_owned();
                        let generated_package = sa_generate(&self.addition);
                        self.status = "Generated! Pushing to file...".to_owned();

                        match sa_export(generated_package) {
                            Ok((sfn, ifn)) => {
                                self.status = format!("Success!\nExported to files:\n{sfn}\n{ifn}")
                            }
                            Err(e) => {
                                self.status =
                                    format!("Error! Could not export to files!\nRaw error: {e}")
                            }
                        }
                    }

                    ui.add_space(bottom_spacing);
                });

                //---===---===---===---===---===
                //Segment: Subtraction Generation
                ui.collapsing("Generate Subtraction Problems", |ui| {
                    let top_spacing = 8.0;
                    let bottom_spacing = 8.0;

                    //Section: Set Amount of Generated Problems
                    ui.add_space(top_spacing);
                    ui.label("The number of subtraction problems to generate (default: 10)");
                    let subtraction_resp_num_of = ui.add(TextEdit::singleline(
                        &mut self.buffer_space_subtraction.num_of_problems,
                    ));

                    if subtraction_resp_num_of.lost_focus()
                        && ui.input(|i| i.key_pressed(Key::Enter))
                    {
                        println!(
                            "Subtraction Num Of Problems Input: {0}",
                            &self.buffer_space_subtraction.num_of_problems
                        );
                        match check_input_num(&self.buffer_space_subtraction.num_of_problems) {
                            SAValidity::ValidNumber(n) => {
                                println!("Conversion successful, valid number!");
                                self.subtraction.problems_num = n;
                            }
                            SAValidity::InvalidNumber => {
                                println!("Conversion unsuccessful, invalid number!");
                            }
                        }
                    }

                    //Section: Set Generation Floor
                    ui.add_space(top_spacing);
                    ui.label("The lowest possible number to generate (default: 2)");
                    let subtraction_resp_floor = ui.add(TextEdit::singleline(
                        &mut self.buffer_space_subtraction.floor,
                    ));

                    if subtraction_resp_floor.lost_focus()
                        && ui.input(|i| i.key_pressed(Key::Enter))
                    {
                        println!(
                            "Subtraction Floor Input: {0}",
                            &self.buffer_space_subtraction.floor
                        );
                        match check_input_num(&self.buffer_space_subtraction.floor) {
                            SAValidity::ValidNumber(n) => {
                                println!("Conversion successful, valid number!");
                                self.subtraction.problems_floor = n;
                            }
                            SAValidity::InvalidNumber => {
                                println!("Conversion unsuccessful, invalid number!");
                            }
                        }
                    }

                    //Section: Set Generation Ceiling
                    ui.add_space(top_spacing);
                    ui.label("The highest possible number to generate (default: 250)");
                    let subtraction_resp_ceiling = ui.add(TextEdit::singleline(
                        &mut self.buffer_space_subtraction.ceiling,
                    ));

                    if subtraction_resp_ceiling.lost_focus()
                        && ui.input(|i| i.key_pressed(Key::Enter))
                    {
                        println!(
                            "Subtraction Ceiling Input: {0}",
                            &self.buffer_space_subtraction.ceiling
                        );
                        match check_input_num(&self.buffer_space_subtraction.ceiling) {
                            SAValidity::ValidNumber(n) => {
                                println!("Conversion successful, valid number!");
                                self.subtraction.problems_ceiling = n;
                            }
                            SAValidity::InvalidNumber => {
                                println!("Conversion unsuccessful, invalid number!");
                            }
                        }
                    }

                    //Section: Allow Negatives
                    ui.add_space(top_spacing);
                    ui.label("Whether we allow negative numbers to be generated (default: false)");
                    ui.checkbox(
                        &mut self.subtraction.allow_negatives,
                        "Allow Negative Numbers",
                    );

                    ui.add_space(bottom_spacing);

                    //Section: Generate Button
                    if ui.add(generate_button("Generate Problems")).clicked() {
                        self.status = "Generating subtraction problems with parameters, standby..."
                            .to_owned();
                        let generated_package = sa_generate(&self.subtraction);
                        self.status = "Generated! Pushing to file...".to_owned();

                        match sa_export(generated_package) {
                            Ok((sfn, ifn)) => {
                                self.status = format!("Success!\nExported to files:\n{sfn}\n{ifn}")
                            }
                            Err(e) => {
                                self.status =
                                    format!("Error! Could not export to files!\nRaw error: {e}")
                            }
                        }
                    }

                    ui.add_space(bottom_spacing);
                });

                //---===---===---===---===---===
                //Segment: Multiplication Generation
                ui.collapsing("Generate Multiplication Problems", |ui| {
                    let top_spacing = 8.0;
                    let bottom_spacing = 8.0;

                    //Section: Set Amount of Generated Problems
                    ui.add_space(top_spacing);
                    ui.label("The number of multiplication problems to generate (default: 10)");
                    let multiplication_resp_num_of = ui.add(TextEdit::singleline(
                        &mut self.buffer_space_multiplication.num_of_problems,
                    ));

                    if multiplication_resp_num_of.lost_focus()
                        && ui.input(|i| i.key_pressed(Key::Enter))
                    {
                        println!(
                            "Multiplication Num Of Problems Input: {0}",
                            &self.buffer_space_multiplication.num_of_problems
                        );
                        match check_input_num(&self.buffer_space_multiplication.num_of_problems) {
                            SAValidity::ValidNumber(n) => {
                                println!("Conversion successful, valid number!");
                                self.multiplication.problems_num = n;
                            }
                            SAValidity::InvalidNumber => {
                                println!("Conversion unsuccessful, invalid number!");
                            }
                        }
                    }

                    //Section: Set Generation Floor
                    ui.add_space(top_spacing);
                    ui.label("The lowest possible number to generate (default: 2)");
                    let multiplication_resp_floor = ui.add(TextEdit::singleline(
                        &mut self.buffer_space_multiplication.floor,
                    ));

                    if multiplication_resp_floor.lost_focus()
                        && ui.input(|i| i.key_pressed(Key::Enter))
                    {
                        println!(
                            "Multiplication Floor Input: {0}",
                            &self.buffer_space_multiplication.floor
                        );
                        match check_input_num(&self.buffer_space_multiplication.floor) {
                            SAValidity::ValidNumber(n) => {
                                println!("Conversion successful, valid number!");
                                self.multiplication.problems_floor = n;
                            }
                            SAValidity::InvalidNumber => {
                                println!("Conversion unsuccessful, invalid number!");
                            }
                        }
                    }

                    //Section: Set Generation Ceiling
                    ui.add_space(top_spacing);
                    ui.label("The highest possible number to generate (default: 250)");
                    let multiplication_resp_ceiling = ui.add(TextEdit::singleline(
                        &mut self.buffer_space_multiplication.ceiling,
                    ));

                    if multiplication_resp_ceiling.lost_focus()
                        && ui.input(|i| i.key_pressed(Key::Enter))
                    {
                        println!(
                            "multiplication Ceiling Input: {0}",
                            &self.buffer_space_multiplication.ceiling
                        );
                        match check_input_num(&self.buffer_space_multiplication.ceiling) {
                            SAValidity::ValidNumber(n) => {
                                println!("Conversion successful, valid number!");
                                self.multiplication.problems_ceiling = n;
                            }
                            SAValidity::InvalidNumber => {
                                println!("Conversion unsuccessful, invalid number!");
                            }
                        }
                    }

                    //Section: Allow Negatives
                    ui.add_space(top_spacing);
                    ui.label("Whether we allow negative numbers to be generated (default: false)");
                    ui.checkbox(
                        &mut self.multiplication.allow_negatives,
                        "Allow Negative Numbers",
                    );

                    ui.add_space(bottom_spacing);

                    //Section: Generate Button
                    if ui.add(generate_button("Generate Problems")).clicked() {
                        self.status =
                            "Generating multiplication problems with parameters, standby..."
                                .to_owned();
                        let generated_package = sa_generate(&self.multiplication);
                        self.status = "Generated! Pushing to file...".to_owned();

                        match sa_export(generated_package) {
                            Ok((sfn, ifn)) => {
                                self.status = format!("Success!\nExported to files:\n{sfn}\n{ifn}")
                            }
                            Err(e) => {
                                self.status =
                                    format!("Error! Could not export to files!\nRaw error: {e}")
                            }
                        }
                    }

                    ui.add_space(bottom_spacing);
                });

                //---===---===---===---===---===
                //Segment: Division Generation
                ui.collapsing("Generate Division Problems", |ui| {
                    let top_spacing = 8.0;
                    let bottom_spacing = 8.0;

                    //Section: Set Amount of Generated Problems
                    ui.add_space(top_spacing);
                    ui.label("The number of division problems to generate (default: 10)");
                    let division_resp_num_of = ui.add(TextEdit::singleline(
                        &mut self.buffer_space_division.num_of_problems,
                    ));

                    if division_resp_num_of.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter))
                    {
                        println!(
                            "Division Num Of Problems Input: {0}",
                            &self.buffer_space_division.num_of_problems
                        );
                        match check_input_num(&self.buffer_space_division.num_of_problems) {
                            SAValidity::ValidNumber(n) => {
                                println!("Conversion successful, valid number!");
                                self.division.problems_num = n;
                            }
                            SAValidity::InvalidNumber => {
                                println!("Conversion unsuccessful, invalid number!");
                            }
                        }
                    }

                    //Section: Set Generation Floor
                    ui.add_space(top_spacing);
                    ui.label("The lowest possible number to generate (default: 2)");
                    let division_resp_floor =
                        ui.add(TextEdit::singleline(&mut self.buffer_space_division.floor));

                    if division_resp_floor.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                        println!(
                            "Division Floor Input: {0}",
                            &self.buffer_space_division.floor
                        );
                        match check_input_num(&self.buffer_space_division.floor) {
                            SAValidity::ValidNumber(n) => {
                                println!("Conversion successful, valid number!");
                                self.division.problems_floor = n;
                            }
                            SAValidity::InvalidNumber => {
                                println!("Conversion unsuccessful, invalid number!");
                            }
                        }
                    }

                    //Section: Set Generation Ceiling
                    ui.add_space(top_spacing);
                    ui.label("The highest possible number to generate (default: 250)");
                    let division_resp_ceiling = ui.add(TextEdit::singleline(
                        &mut self.buffer_space_division.ceiling,
                    ));

                    if division_resp_ceiling.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter))
                    {
                        println!(
                            "Division Ceiling Input: {0}",
                            &self.buffer_space_division.ceiling
                        );
                        match check_input_num(&self.buffer_space_division.ceiling) {
                            SAValidity::ValidNumber(n) => {
                                println!("Conversion successful, valid number!");
                                self.division.problems_ceiling = n;
                            }
                            SAValidity::InvalidNumber => {
                                println!("Conversion unsuccessful, invalid number!");
                            }
                        }
                    }

                    //Section: Allow Negatives
                    ui.add_space(top_spacing);
                    ui.label("Whether we allow negative numbers to be generated (default: false)");
                    ui.checkbox(&mut self.division.allow_negatives, "Allow Negative Numbers");

                    ui.add_space(bottom_spacing);

                    //Section: Generate Button
                    if ui.add(generate_button("Generate Problems")).clicked() {
                        self.status =
                            "Generating division problems with parameters, standby...".to_owned();
                        let generated_package = sa_generate(&self.division);
                        self.status = "Generated! Pushing to file...".to_owned();

                        match sa_export(generated_package) {
                            Ok((sfn, ifn)) => {
                                self.status = format!("Success!\nExported to files:\n{sfn}\n{ifn}")
                            }
                            Err(e) => {
                                self.status =
                                    format!("Error! Could not export to files!\nRaw error: {e}")
                            }
                        }
                    }

                    ui.add_space(bottom_spacing);
                });

                ui.add_space(16.0);

                if ui.add(generate_button("Exit SimAri")).clicked() {
                    std::process::exit(0);
                }
            });
        });
    }
}

fn check_input_num<S1: AsRef<str>>(input: S1) -> SAValidity {
    match input.as_ref().parse::<i32>() {
        Ok(n) => SAValidity::ValidNumber(n),
        Err(_) => SAValidity::InvalidNumber,
    }
}

fn generate_button(label: &str) -> Button {
    Button::new(format!("{label}"))
        .min_size(Vec2 { x: 48.0, y: 24.0 })
        .rounding(Rounding {
            nw: 0.0,
            ne: 6.0,
            sw: 6.0,
            se: 0.0,
        })
}

fn setup_custom_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "Tektur".to_owned(),
        FontData::from_static(include_bytes!("./res/tektur.ttf")),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "Tektur".to_owned());

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("Tektur".to_owned());

    ctx.set_fonts(fonts);
}

fn setup_text_styles(ctx: &Context) {
    let mut style = (*ctx.style()).clone();

    style.text_styles = [
        (Heading, FontId::new(24.0, Proportional)),
        (Body, FontId::new(10.0, Proportional)),
        (Button, FontId::new(10.0, Proportional)),
        (Small, FontId::new(6.0, Proportional)),
    ]
    .into();

    ctx.set_style(style);
}

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_active(true)
            .with_min_inner_size([960.0, 720.0])
            .with_max_inner_size([1920.0, 1080.0]),
        follow_system_theme: false,
        ..Default::default()
    };
    let _ = eframe::run_native(
        "SimAri",
        native_options,
        Box::new(|cc| Box::new(SAGui::new(cc))),
    );
}
