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

// enum State {
//     Rook,
//     King,
//     Done,
// }

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

        let mut gen = vec![usize::MAX; 8];

        // bishops on opposite colors
        let left_bishop = [0, 2, 4, 6]
            .choose(&mut rng)
            .copied()
            .unwrap();
        let right_bishop = [1, 3, 5, 7]
            .choose(&mut rng)
            .copied()
            .unwrap();

        gen[left_bishop] = 2;
        gen[right_bishop] = 5;

        // queen
        let mut empty: Vec<usize> =
            (0..8).filter(|&i| gen[i] == usize::MAX).collect();

        let queen = *empty.choose(&mut rng).unwrap();
        gen[queen] = 3;

        // knights
        empty.retain(|&i| i != queen);
        empty.shuffle(&mut rng);

        gen[empty[0]] = 1;
        gen[empty[1]] = 6;

        // remaining squares are rook, king, rook from left to right
        let mut remaining: Vec<usize> =
            (0..8).filter(|&i| gen[i] == usize::MAX).collect();

        remaining.sort();

        gen[remaining[0]] = 0; // left rook
        gen[remaining[1]] = 4; // king
        gen[remaining[2]] = 7; // right rook

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
