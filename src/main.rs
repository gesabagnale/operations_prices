//#![windows_subsystem="windows"]
use fltk::{app, window, button::Button, prelude::*, input::*, menu::Choice};
use fltk::group::Pack;
use fltk::group::Scroll;
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let app = app::App::default();
    let _ = MyWindow::new();
    app.run().unwrap();
}

const WIDGET_HEIGHT: i32 = 25;

struct Adder {}

impl Adder {
    fn new() -> (Self, Vec::<(Choice, Input, Input, Input)>) {
        fn widgets(x: i32, y: i32, count_lines: i32) -> Vec<(Choice, Input, Input, Input)> {
            let mut lines = Vec::<(Choice, Input, Input, Input)>::new();
            for i in 0..count_lines {
                let choice = Choice::default()
                    .with_size(250, WIDGET_HEIGHT).with_pos(x, y + i*30);
                let count = Input::default()
                    .with_size(130, WIDGET_HEIGHT).with_pos(x + 260, y + i*30);
                let price = Input::default()
                    .with_size(130, WIDGET_HEIGHT).with_pos(x + 400, y + i*30);
                let all = Input::default()
                    .with_size(130, WIDGET_HEIGHT).with_pos(x + 540, y + i*30);
                lines.push((choice, count, price, all));
            }
            lines
        }

        let lines = widgets(10, 30, 30);
        let mut btn = Button::new(160, 200, 80, 30, "Hello");
        btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);

        (Adder {}, lines)
    }
}

struct MyWindow {}

impl MyWindow {
    fn new() -> Self {
        let mut win = window::Window::default()
            .with_size(720, 620)
            .with_label("")
            .center_screen();
        let label_pack = Pack::new(200,24,300,40,"Наименование операции                        Количество           Стоимость за ед.              Всего");
        label_pack.end();
        let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
        widget_theme.apply();

        let scroll = Scroll::new(10,30,709,580,"");

        let (_, lines) = Adder::new(); // Вектор виджетов
        scroll.end();

        let mut vec_names = Vec::<Choice>::new();
        let mut vec_counts = Vec::<Input>::new();
        let mut vec_prices = Vec::<Input>::new();
        let mut vec_all = Vec::<Input>::new();

        for (n, c, p, a) in lines {
            vec_names.push(n); vec_counts.push(c);
            vec_prices.push(p); vec_all.push(a);
        }

        let path = "data.txt";
        let f = File::open(path);

        let f = match f {
            Ok(file) => file,
            Err(error) => panic!("Не удалось открыть файл: {:?}", error),
        };

        let (names, _counts, _prices) = read_data_for_choice(f);
        add_to_choice(names, &vec_names);

        win.end();
        win.show();

        MyWindow {}
    }
}

fn read_data_for_choice(input: File) -> (Vec<String>, Vec<i32>, Vec<f64>) {
    // Вектора для хранения исходных данных
    let mut names: Vec<String> = Vec::with_capacity(30);
    let mut counts: Vec<i32> = Vec::with_capacity(30);
    let mut prices: Vec<f64> = Vec::with_capacity(30);

    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let l = &line.unwrap();
        let v: Vec<&str> = l.split(|c| c == ';').collect();

        names.push(v[0].to_string());

        match v[1].trim().parse::<i32>() {
            Ok(n) => counts.push(n),
            Err(_) => println!("Не удалось распарсить {}", v[1]),
        }

        match v[2].trim().parse::<f64>() {
            Ok(n) => prices.push(n),
            Err(_) => println!("Не удалось распарсить {}", v[2]),
        }
    }
    assert!(names.len() == counts.len() && counts.len() == prices.len(), "Некорректные данные");

    (names, counts, prices)
}

fn add_to_choice(
    v: Vec<String>,
    ch: &Vec::<Choice>
) {
    for name in v {
        let ch_c = ch.clone();
        for mut choice in ch_c {
            choice.add_choice(&name);
        }
    }
}



