#![allow(unused)]

use candle_core::{Device, Result, Tensor};
use candle_nn::encoding::one_hot;
use image;
use std::collections::{HashMap, HashSet};

const CPUNKS_PATH: &str = "../cpunks-10k/cpunks/images/training/";
const CPUNKS_TOTAL: u16 = 10000;
// const CPUNKS_TOTAL: u16 = 10;

pub fn get_punk_tensor(p_id: u16) -> Result<(Tensor, usize, usize)> {
    let path = format!("{}punk{:0>4}.png", CPUNKS_PATH, p_id);
    let img = image::open(path).unwrap();
    let (height, width) = (img.height() as usize, img.width() as usize);
    let img = img.to_rgba8();
    let data = img.into_raw();
    let data = Tensor::from_vec(data, (height, width, 4), &Device::Cpu)?.permute((2, 0, 1))?;
    Ok((data, height, width))
}

pub fn one_image_colors(img: &Tensor) -> HashSet<(u8, u8, u8, u8)> {
    let img = img.reshape((4, ())).unwrap();
    let mut colors = HashSet::new();
    let r = img.get(0).unwrap();
    let g = img.get(1).unwrap();
    let b = img.get(2).unwrap();
    let a = img.get(3).unwrap();

    for i in 0..576 {
        let color = (
            r.get(i).unwrap().to_scalar::<u8>().unwrap(),
            g.get(i).unwrap().to_scalar::<u8>().unwrap(),
            b.get(i).unwrap().to_scalar::<u8>().unwrap(),
            a.get(i).unwrap().to_scalar::<u8>().unwrap(),
        );
        colors.insert(color);
    }
    colors
}

pub fn many_image_colors(ts: &Vec<Tensor>) -> HashSet<(u8, u8, u8, u8)> {
    let mut all_colors = HashSet::new();
    for t in ts {
        let img = t.reshape((4, ())).unwrap();
        let colors = one_image_colors(&img);
        all_colors.extend(&colors);
    }
    all_colors
}

pub fn get_all_punks() -> Vec<Tensor> {
    let mut ts: Vec<Tensor> = Vec::with_capacity(CPUNKS_TOTAL as usize);
    for i in 0..CPUNKS_TOTAL {
        if let Ok((t, _, _)) = get_punk_tensor(i) {
            ts.push(t);
        }
    }
    ts
}

//struct ColorMapper {
//    oh_to_colors: HashMap<Tensor, (u8, u8, u8, u8)>,
//    colors_to_oh: HashMap<(u8, u8, u8, u8), Tensor>,
//}
//
//impl From<HashSet<(u8, u8, u8, u8)>> for ColorMapper {
//    fn from(colors: HashSet<(u8, u8, u8, u8)>) -> Self {
//        println!("hey");
//        let mut mapper = ColorMapper { oh_to_colors: HashMap::new(), colors_to_oh: HashMap::new() };
//        let colors_vec = colors
//            .into_iter()
//            .map(|cs| Tensor::new(&[cs.0, cs.1, cs.2, cs.3], &Device::Cpu).unwrap())
//            .collect::<Vec<Tensor>>();
//        // let colors_tensor = Tensor::new(&color_vec, &Device::Cpu).unwrap();
//        let indices = Tensor::from_vec(colors_vec, (4, colors.len()), &Device::Cpu).unwrap();
//        let one_hots = one_hot(indices, colors.len(), 1u8, 0u8).unwrap();
//        println!("{:?}", one_hots.shape());
//        mapper
//    }
//}

fn main() {
    let ts = get_all_punks();
    let colors = many_image_colors(&ts);
    println!("{}", colors.len());
    //let _mapper = ColorMapper::from(colors);
}
