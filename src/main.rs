use core::panic;

use dioxus::prelude::*;
use rand::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

enum Piece {
    WPawn,
    WLRook,
    WLKnight,
    WLBishop,
    WQueen,
    WKing,
    WRBishop,
    WRKnight,
    WRRook,
    BPawn,
    BLRook,
    BLKnight,
    BLBishop,
    BQueen,
    BKing,
    BRBishop,
    BRKnight,
    BRRook,
}

impl Piece {
    fn svg(&self) -> Asset {
        match self {
            Piece::WPawn => asset!("/assets/pieces/wP.svg"),
            Piece::WLRook => asset!("/assets/pieces/wR.svg"),
            Piece::WLKnight => asset!("/assets/pieces/wN.svg"),
            Piece::WLBishop => asset!("/assets/pieces/wB.svg"),
            Piece::WQueen => asset!("/assets/pieces/wQ.svg"),
            Piece::WKing => asset!("/assets/pieces/wK.svg"),
            Piece::WRBishop => asset!("/assets/pieces/wB.svg"),
            Piece::WRKnight => asset!("/assets/pieces/wN.svg"),
            Piece::WRRook => asset!("/assets/pieces/wR.svg"),
            Piece::BPawn => asset!("/assets/pieces/bP.svg"),
            Piece::BLRook => asset!("/assets/pieces/bR.svg"),
            Piece::BLKnight => asset!("/assets/pieces/bN.svg"),
            Piece::BLBishop => asset!("/assets/pieces/bB.svg"),
            Piece::BQueen => asset!("/assets/pieces/bQ.svg"),
            Piece::BKing => asset!("/assets/pieces/bK.svg"),
            Piece::BRBishop => asset!("/assets/pieces/bB.svg"),
            Piece::BRKnight => asset!("/assets/pieces/bN.svg"),
            Piece::BRRook => asset!("/assets/pieces/bR.svg"),
        }
    }
}

enum State {
    Rook,
    King,
    Done,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    let mut rng = rand::rng();
    let white_pieces: Vec<Piece> = vec![
        Piece::WLRook,
        Piece::WLKnight,
        Piece::WLBishop,
        Piece::WQueen,
        Piece::WKing,
        Piece::WRBishop,
        Piece::WRKnight,
        Piece::WRRook,
    ];
    let black_pieces: Vec<Piece> = vec![
        Piece::BLRook,
        Piece::BLKnight,
        Piece::BLBishop,
        Piece::BQueen,
        Piece::BKing,
        Piece::BRBishop,
        Piece::BRKnight,
        Piece::BRRook,
    ];
    let mut pieces: Signal<Vec<usize>> = use_signal(|| vec![0,1,2,3,4,5,6,7,8]);

    let reset = move |_| {
        let mut pieces = pieces.write();
        *pieces = vec![0,1,2,3,4,5,6,7,8];
    };
    let generate = move |_| {
        let mut pieces = pieces.write();
        let mut state = State::Rook;
        let mut gen: Vec<usize> = vec![];
        for _ in 0..8 {
            let mut number = usize::MAX;
            let mut cond = true;
            while cond {
                number = (rng.random::<f32>()*8.) as usize;
                if number > 7 {number = 7;}
                if !gen.contains(&number) {
                    match state {
                        State::Rook => {
                            if number != 4 && number != 7 {
                                cond = false;
                                if number == 0 {
                                    state = State::King;
                                }
                            }
                        },
                        State::King => {
                            if number != 7 {
                                cond = false;
                                if number == 4 {
                                    state = State::Done;
                                }
                            }
                        },
                        State::Done => {
                            cond = false;
                        },
                    }
                }
            }
            if number == usize::MAX {
                panic!("Something went wrong");
            }
            gen.push(number);
        }
        *pieces = gen;
    };
    
    rsx! {
        div {
            id: "hero",
            h1 {
                id: "title",
                "Freestyle Chess Generator"
            }
            div {
                id: "board",
                svg {
                    view_box: "0 0 800 800",
                    height: "100%",
                    width: "auto",

                    for row in 0..8 {
                        for col in 0..8 {
                            rect {
                                x: "{col * 100}",
                                y: "{row * 100}",
                                width: "100",
                                height: "100",
                                fill: if (row + col) % 2 == 0 { "#f0d9b5" } else { "#b58863" }
                            }
                        }
                    }
                    for col in 0..8 {
                        image {
                            href: black_pieces[pieces.read()[col]].svg(),
                            x: "{col * 100}",
                            y: "0",
                            width: "100",
                            height: "100",
                        }
                        image {
                            href: Piece::BPawn.svg(),
                            x: "{col * 100}",
                            y: "100",
                            width: "100",
                            height: "100",
                        }
                        image {
                            href: Piece::WPawn.svg(),
                            x: "{col * 100}",
                            y: "600",
                            width: "100",
                            height: "100",
                        }
                        image {
                            href: white_pieces[pieces.read()[col]].svg(),
                            x: "{col * 100}",
                            y: "700",
                            width: "100",
                            height: "100",
                        }
                    }
                }
            }
            div {
                id: "buttons",
                button {
                    onclick: reset,
                    "Reset"
                }
                button {
                    onclick: generate,
                    "Generate"
                }
            }
            footer {
                "Chess pieces by sadsnake1, licensed under CC BY-NC-SA 4.0"
            }
        }
    }
}
