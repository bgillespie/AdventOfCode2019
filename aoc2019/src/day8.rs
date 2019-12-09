use crate::common::*;

const WIDTH : usize = 25;
const HEIGHT: usize = 6;

struct Sif {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Sif {
    fn new(width: usize, height: usize, data: Vec<u8>) -> Self {
        Self { width, height, data }
    }

    #[inline]
    fn layer_size(&self) -> usize {
        self.width * self.height
    }

    #[inline]
    fn layer_offset(&self, layer_idx: usize) -> usize {
        layer_idx * self.layer_size()
    }

    #[inline]
    fn num_layers(&self) -> usize {
        self.data.len() / self.layer_size()
    }

    #[inline]
    fn layer_data_ref(&self, layer_idx: usize) -> &[u8] {
        &self.data[
            self.layer_offset(layer_idx)
            ..self.layer_offset(layer_idx + 1)
        ]
    }

    /// Count pixels of a certain value in a layer
    #[inline]
    fn count_pixel_eq(&self, layer_idx: usize, pixel_val: u8) -> usize {
        self.layer_data_ref(layer_idx)
            .iter()
            .filter(|k| **k == pixel_val)
            .count()
    }

    fn render(&self) -> String {
        let refs = 
            (0..self.num_layers())
            .into_iter()
            .map(|i| self.layer_data_ref(i))
            .collect::<Vec<&[u8]>>();
        let mut rendered: Vec<u8> = Vec::with_capacity(self.layer_size());
        for i in 0..self.layer_size() {
            rendered.push(
                refs
                    .iter()
                    .fold(2, |acc, p| if acc == 2 {p[i]} else {acc}));
        }
        rendered
            .chunks(self.width)
            .map(|row| row.iter()
                        .map(|u| if *u == 1 {"1"} else {" "})
                        .collect::<Vec<&str>>()
                        .join("")
            )
            .collect::<Vec<String>>().join("\n")
    }
}

pub fn run() {
    let image_data = 
        file_to_string("day/8/input")
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|i| i.to_string().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
    let sif = Sif::new(WIDTH, HEIGHT, image_data);
    let num_layers = sif.num_layers();
    println!(
        "Layer size {} data size {} num layers {}",
        sif.width * sif.height, sif.data.len(), num_layers
    );
    
    let layer_least_0s =
        (0..num_layers)
            .into_iter()
            .map(|i| (i, sif.count_pixel_eq(i, 0)) )
            .fold((0, 1000000000), |memo, next|
                if next.1 < memo.1 { next } else { memo }
            ).0;
    println!("Index of layer with least zeroes {:?}", layer_least_0s);
    let layers_1s = sif.count_pixel_eq(layer_least_0s, 1);
    let layers_2s = sif.count_pixel_eq(layer_least_0s, 2);
    println!("{}", layers_1s * layers_2s);

    println!("{}", sif.render());
}
