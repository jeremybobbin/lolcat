use std::{
    io::{
        self,
        BufReader,
        BufWriter,
        Read,
        Write,
        BufRead,
        LineWriter
    },
    env::var,
    f64::consts::PI
};


fn main() {
    let mut stdin  = io::stdin();
    let mut stdout = io::stdout();

    let ym = var("X").ok()
        .and_then(|a| a.parse().ok())
        .unwrap_or(0.2);

    let xm = var("Y").ok()
        .and_then(|a| a.parse().ok())
        .unwrap_or(0.2);

    lolcat(stdin, stdout.lock(), (xm, ym))
        .unwrap();
}


// X, Y multiplier
fn lolcat<R: Read, W: Write>(reader: R, writer: W, multipliers: (f64, f64)) -> io::Result<()>  {
    let (xm, ym) = multipliers;

    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    let lines = reader.lines()
        .filter_map(Result::ok)
        .enumerate();

    for (y, line) in lines {
        let y = y as f64 * ym;
        for (x, c) in line.chars().enumerate() {
            let i = y + (x as f64 * xm);

            let r = (((i                    ).sin() * 127.) + 128.) as u8;
            let g = (((i + ((2. * PI) / 3.) ).sin() * 127.) + 128.) as u8;
            let b = (((i + ((4. * PI) / 3.) ).sin() * 127.) + 128.) as u8;
            write!(&mut writer, "\x1B[38;2;{};{};{}m{}", r, g, b, c);
        }
        write!(&mut writer, "\n");
    }
    writer.flush()?;
    Ok(())
}
