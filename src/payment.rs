#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Payment {
    Cash,
    Swish,
    Paypal,
}

impl Default for Payment {
    fn default() -> Self {
        Self::Swish
    }
}

impl From<Payment> for String {
    fn from(p: Payment) -> String {
        String::from(match p {
            Payment::Swish => "Swish",
            Payment::Cash => "Cash",
            Payment::Paypal => "PayPal",
        })
    }
}

impl TryFrom<String> for Payment {
    type Error = crate::error::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "cash" => Ok(Self::Cash),
            "swish" => Ok(Self::Swish),
            "paypal" => Ok(Self::Paypal),
            _ => Err("Invalid Payment Method")?,
        }
    }
}

use iced::{widget::canvas::*, Color, Point, Rectangle, Vector};
impl<M> Program<M> for Payment {
    fn draw(&self, bounds: Rectangle<f32>, _cursor: Cursor) -> Vec<Geometry> {
        match *self {
            Self::Swish => {
                let mut frame = Frame::new(bounds.size());

                frame.scale(frame.width().min(frame.height()) / 420.0);

                let right_swirl = Path::new(|path| {
                    path.move_to(Point {
                        x: 0.0,
                        y: 350.04439,
                    });
                    path.bezier_curve_to(
                        Point {
                            x: 84.2884683,
                            y: 390.356195,
                        },
                        Point {
                            x: 188.31202,
                            y: 370.40599,
                        },
                        Point {
                            x: 251.156312,
                            y: 295.526341,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 325.643824,
                            y: 206.7744,
                        },
                        Point {
                            x: 314.074361,
                            y: 74.4625171,
                        },
                        Point {
                            x: 225.315044,
                            y: -5.68434189e-14,
                        },
                    );
                    path.line_to(Point {
                        x: 166.309346,
                        y: 70.306361,
                    });
                    path.bezier_curve_to(
                        Point {
                            x: 235.651961,
                            y: 128.480254,
                        },
                        Point {
                            x: 244.690771,
                            y: 231.848605,
                        },
                        Point {
                            x: 186.49762,
                            y: 301.186302,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 140.096429,
                            y: 356.473463,
                        },
                        Point {
                            x: 63.6990439,
                            y: 374.933385,
                        },
                        Point {
                            x: 0.0,
                            y: 350.04439,
                        },
                    );
                });

                let left_swirl = Path::new(|path| {
                    path.move_to(Point {
                        x: 300.254951,
                        y: 20.4289088,
                    });
                    path.bezier_curve_to(
                        Point {
                            x: 215.966893,
                            y: -19.8828961,
                        },
                        Point {
                            x: 111.943342,
                            y: 0.0668990353,
                        },
                        Point {
                            x: 49.0990489,
                            y: 74.9465478,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: -25.3884633,
                            y: 163.698899,
                        },
                        Point {
                            x: -13.8189999,
                            y: 296.010782,
                        },
                        Point {
                            x: 74.9399074,
                            y: 370.473299,
                        },
                    );
                    path.line_to(Point {
                        x: 133.946015,
                        y: 300.166938,
                    });
                    path.bezier_curve_to(
                        Point {
                            x: 64.6029903,
                            y: 241.993045,
                        },
                        Point {
                            x: 55.5645903,
                            y: 138.624694,
                        },
                        Point {
                            x: 113.757742,
                            y: 69.2865868,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 160.158932,
                            y: 13.9994259,
                        },
                        Point {
                            x: 236.556317,
                            y: -4.46049609,
                        },
                        Point {
                            x: 300.254951,
                            y: 20.4289088,
                        },
                    );
                });

                frame.fill(&left_swirl, Color::BLACK);
                frame.translate(Vector {
                    x: 119.332186,
                    y: 49.114212,
                });
                frame.fill(&right_swirl, Color::BLACK);

                vec![frame.into_geometry()]
            }
            Self::Paypal => {
                let mut frame = Frame::new(bounds.size());

                frame.scale(frame.width().min(frame.height()) / 225.0);
                //frame.translate(Vector { x: 0.0, y: 225.0 });

                let path1 = Path::new(|path| {
                    path.move_to(Point { x: 57.8, y: 15.0 });
                    path.bezier_curve_to(
                        Point { x: 56.3, y: 16.1 },
                        Point { x: 54.9, y: 17.8 },
                        Point {
                            x: 54.599999999999994,
                            y: 18.8,
                        },
                    );
                    path.bezier_curve_to(
                        Point { x: 54.3, y: 19.7 },
                        Point {
                            x: 48.199999999999996,
                            y: 57.599999999999994,
                        },
                        Point {
                            x: 40.99999999999999,
                            y: 103.0,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 29.099999999999994,
                            y: 178.8,
                        },
                        Point {
                            x: 28.199999999999992,
                            y: 185.6,
                        },
                        Point {
                            x: 29.599999999999994,
                            y: 187.7,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 31.099999999999994,
                            y: 190.0,
                        },
                        Point {
                            x: 31.199999999999996,
                            y: 190.0,
                        },
                        Point {
                            x: 49.3,
                            y: 189.79999999999998,
                        },
                    );
                    path.line_to(Point {
                        x: 67.5,
                        y: 189.49999999999997,
                    });
                    path.line_to(Point {
                        x: 72.3,
                        y: 158.99999999999997,
                    });
                    path.bezier_curve_to(
                        Point {
                            x: 75.0,
                            y: 142.19999999999996,
                        },
                        Point {
                            x: 77.7,
                            y: 127.39999999999998,
                        },
                        Point {
                            x: 78.3,
                            y: 126.09999999999997,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 79.0,
                            y: 124.79999999999997,
                        },
                        Point {
                            x: 80.8,
                            y: 122.69999999999996,
                        },
                        Point {
                            x: 82.3,
                            y: 121.59999999999997,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 84.89999999999999,
                            y: 119.69999999999996,
                        },
                        Point {
                            x: 87.0,
                            y: 119.39999999999996,
                        },
                        Point {
                            x: 105.8,
                            y: 118.79999999999997,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 128.9,
                            y: 118.09999999999997,
                        },
                        Point {
                            x: 134.2,
                            y: 117.09999999999997,
                        },
                        Point {
                            x: 145.5,
                            y: 111.19999999999997,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 159.0,
                            y: 104.09999999999998,
                        },
                        Point {
                            x: 168.1,
                            y: 91.99999999999997,
                        },
                        Point {
                            x: 173.5,
                            y: 73.49999999999997,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 177.2,
                            y: 61.09999999999997,
                        },
                        Point {
                            x: 177.4,
                            y: 47.199999999999974,
                        },
                        Point {
                            x: 174.0,
                            y: 38.49999999999997,
                        },
                    );
                    path.bezier_curve_to(
                        Point { x: 169.8, y: 27.8 },
                        Point { x: 159.8, y: 19.9 },
                        Point { x: 144.5, y: 15.4 },
                    );
                    path.bezier_curve_to(
                        Point { x: 139.4, y: 13.9 },
                        Point { x: 132.9, y: 13.6 },
                        Point { x: 99.5, y: 13.3 },
                    );
                    path.bezier_curve_to(
                        Point { x: 61.1, y: 12.9 },
                        Point { x: 60.4, y: 12.9 },
                        Point { x: 57.8, y: 15.0 },
                    );
                    path.close();
                });

                let path2 = Path::new(|path| {
                    path.move_to(Point {
                        x: 183.40014,
                        y: 69.7,
                    });
                    path.bezier_curve_to(
                        Point {
                            x: 178.30014,
                            y: 93.1,
                        },
                        Point {
                            x: 166.10012,
                            y: 110.5,
                        },
                        Point {
                            x: 148.90008999999998,
                            y: 118.9,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 135.20007999999999,
                            y: 125.60000000000001,
                        },
                        Point {
                            x: 122.20005999999998,
                            y: 128.0,
                        },
                        Point {
                            x: 99.80003499999998,
                            y: 128.0,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 87.40001899999999,
                            y: 128.0,
                        },
                        Point {
                            x: 86.90001799999997,
                            y: 128.1,
                        },
                        Point {
                            x: 86.00001699999999,
                            y: 130.2,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 85.50001599999999,
                            y: 131.5,
                        },
                        Point {
                            x: 82.60001199999999,
                            y: 148.5,
                        },
                        Point {
                            x: 79.50000799999998,
                            y: 168.0,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 76.400004,
                            y: 187.5,
                        },
                        Point { x: 73.7, y: 204.7 },
                        Point { x: 73.4, y: 206.1 },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 72.39999900000001,
                            y: 211.4,
                        },
                        Point {
                            x: 74.400001,
                            y: 212.0,
                        },
                        Point {
                            x: 92.400025,
                            y: 212.0,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 107.10004,
                            y: 212.0,
                        },
                        Point {
                            x: 108.80005,
                            y: 211.8,
                        },
                        Point {
                            x: 110.70005,
                            y: 210.0,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 113.20006000000001,
                            y: 207.8,
                        },
                        Point {
                            x: 113.10006,
                            y: 208.2,
                        },
                        Point {
                            x: 117.00005,
                            y: 183.4,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 120.10006,
                            y: 163.5,
                        },
                        Point {
                            x: 120.90006,
                            y: 160.1,
                        },
                        Point {
                            x: 123.10006,
                            y: 158.3,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 123.90007,
                            y: 157.8,
                        },
                        Point {
                            x: 129.50008,
                            y: 157.0,
                        },
                        Point {
                            x: 135.50008,
                            y: 156.70000000000002,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 154.60011,
                            y: 155.60000000000002,
                        },
                        Point {
                            x: 167.70012,
                            y: 151.10000000000002,
                        },
                        Point {
                            x: 177.30014,
                            y: 142.20000000000002,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 193.00016,
                            y: 127.60000000000002,
                        },
                        Point {
                            x: 200.80016,
                            y: 99.30000000000001,
                        },
                        Point {
                            x: 194.40016,
                            y: 79.90000000000002,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 193.00016,
                            y: 75.40000000000002,
                        },
                        Point {
                            x: 186.80015,
                            y: 67.00000000000001,
                        },
                        Point {
                            x: 184.90014,
                            y: 67.00000000000001,
                        },
                    );
                    path.bezier_curve_to(
                        Point {
                            x: 184.40014,
                            y: 67.00000000000001,
                        },
                        Point {
                            x: 183.80014,
                            y: 68.20000000000002,
                        },
                        Point {
                            x: 183.40014,
                            y: 69.70000000000002,
                        },
                    );
                    path.close();
                });

                frame.fill(&path1, Color::BLACK);
                frame.fill(&path2, Color::BLACK);
                vec![frame.into_geometry()]
            }
            Self::Cash => vec![],
        }
    }
}
