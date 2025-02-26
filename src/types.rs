
pub fn mmtopx(mm: f64) -> f64 {
    // TODO(hualet): use actual dpi?
    let dpi = 97.0;
    return mm * dpi / 25.4;
}

pub mod st {
    pub type Array = Vec<i32>;
}

pub mod ct {
    use crate::types::st;
    use crate::types::mmtopx;

    pub struct PageArea {
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
    }

    // Boundary="10.30 30.30 0.30 22"
    #[derive(Debug)]
    pub struct Box {
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
    }

    #[derive(Debug)]
    pub struct Color {
        pub value: st::Array,
        pub alpha: i32,
    }

    // implement string to PageArea
    impl From<String> for PageArea {
        fn from(value: String) -> Self {
            let parts: Vec<f64> = value.split_whitespace().map(|s| s.parse().unwrap()).collect();
            PageArea {
                x: parts[0],
                y: parts[1],
                width: parts[2],
                height: parts[3],
            }
        }
    }

    impl PageArea {
        pub fn to_pixel(&self) -> PageArea {
            PageArea {
                x: mmtopx(self.x),
                y: mmtopx(self.y),
                width: mmtopx(self.width),
                height: mmtopx(self.height),
            }
        }
    }

    // implement string to Box
    impl From<String> for Box {
        fn from(value: String) -> Self {
            let parts: Vec<f64> = value.split_whitespace().map(|s| s.parse().unwrap()).collect();
            Box {
                x: parts[0],
                y: parts[1],
                width: parts[2],
                height: parts[3],
            }
        }
    }


    impl Box {
        pub fn to_pixel(&self) -> Box {
            Box {
                x: mmtopx(self.x),
                y: mmtopx(self.y),
                width: mmtopx(self.width),
                height: mmtopx(self.height),
            }
        }
    }

    // implement string to Color
    impl From<String> for Color {
        fn from(value: String) -> Self {
            let parts: Vec<i32> = value.split_whitespace().map(|s| s.parse().unwrap()).collect();
            Color {
                value: parts.clone(),
                alpha: 255,
            }
        }
    }

}