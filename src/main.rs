#![feature(map_first_last)]


use image::{open, Rgba, ImageError};
use std::collections::BTreeSet;
use std::time::SystemTime;
use std::io::Write;
use std::env::args;

fn main() {
    
    let mut entrada = String::new();
    
    if args().len() == 1 {
        print!("caminho da imagem: ");
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut entrada).expect("erro ao ler entrada");
        entrada = entrada.trim().to_string();

    }

    else if args().len() == 2 {
        entrada = args().collect::<Vec<String>>()[1].clone();

    }
    else {
        panic!("Número de argumentos inválido!!!")
    }


    let res: f64 = removedor_de_fundos(&entrada).unwrap() as f64 / 1000.0;
    println!("Tempo de execução: {}ms", res);
    println!("Aperte enter para sair!");
    std::io::stdin().read_line(&mut entrada).unwrap();

}


fn removedor_de_fundos(path: &String) -> Result<u128, ImageError> {

    let tempo = SystemTime::now();

    let mut img = open(path)?.to_rgba();

    let (mut img_x, mut img_y) = img.dimensions();
    img_x = img_x - 1;
    img_y = img_y - 1;

    let mut checar: BTreeSet<(u32, u32)> = BTreeSet::new();
    let mut tmp: (u32, u32);
    let alvo = Rgba([0, 0, 0, 0]);

    {
        for a in 0..img_x + 1 {
            if branco(img.get_pixel(a, 0)) {
                checar.insert((a, 0));
            }
            if branco(img.get_pixel(a, img_y)) {
                checar.insert((a, img_y));
            }
        }

        for a in 0..img_y + 1 {
            if branco(img.get_pixel(0, a)) {
                checar.insert((0, a));
            }
            if branco(img.get_pixel(img_x, a)) {
                checar.insert((img_x, a));
            }
        }
    }

    while !checar.is_empty() {
        tmp = checar.pop_first().unwrap();

        for a in pegar_lados(tmp, img_x, img_y) {
            if branco(img.get_pixel(a.0 as u32, a.1 as u32)) {
                checar.insert(a);
            }
        }

        img.put_pixel(tmp.0 as u32, tmp.1 as u32, alvo)
    }

    img.save("./saida.png")?;

    match tempo.elapsed() {

        Ok(elapsed) => return Ok(elapsed.as_millis()),
        _ => panic!("erro ao retornar tempo de execução"),
    }

}

fn pegar_lados(pos: (u32, u32), tamx: u32, tamy: u32) -> Vec<(u32, u32)> {
    let mut saida: Vec<(u32, u32)> = vec![];

    if pos.0 >= 1 {
        saida.push(((pos.0) - 1, pos.1));
    }
    if ((pos.0) + 1) <= tamx {
        saida.push(((pos.0) + 1, pos.1));
    }
    if pos.1 >= 1 {
        saida.push((pos.0, (pos.1) - 1));
    }
    if ((pos.1) + 1) <= tamy {
        saida.push((pos.0, (pos.1) + 1));
    }

    return saida;
}

fn branco(valor: &Rgba<u8>) -> bool {
    let tmp = vec![valor[0], valor[1], valor[2], valor[3]];

    if tmp.iter().all(|&x| x >= 250) {
        return true;
    }
    return false;
}
