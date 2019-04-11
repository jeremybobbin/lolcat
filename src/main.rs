use std::{
    io::{
        self,
        BufReader,
        Read,
        Write,
        BufRead,
        LineWriter
    },
    env::var,
    f64::consts::PI
};


fn main() {
    let ym = var("X").ok()
        .and_then(|a| a.parse().ok())
        .unwrap_or(0.2);

    let xm = var("Y").ok()
        .and_then(|a| a.parse().ok())
        .unwrap_or(0.2);

    lolcat(io::stdin(), io::stdout(), (xm, ym));
}


// X, Y multiplier
fn lolcat<R: Read, W: Write>(reader: R, writer: W, multipliers: (f64, f64)) -> io::Result<()>  {
    let (xm, ym) = multipliers;

    let mut reader = BufReader::new(reader);
    let mut writer = LineWriter::new(writer);

    let mut y = 0;
    let mut buf = String::new();

    while let Ok(res) = reader.read_line(&mut buf) {
        if res == 0 {
            break;
        }

        if buf.ends_with('\n') {
            buf.pop();
            if buf.ends_with('\r') {
                buf.pop();
            }
        }

        for (x, c) in buf.chars().enumerate() {
            if !c.is_ascii() {
                continue;
            }
            let i = (y as f64 * ym) + (x as f64 * xm);

            let r = (((i                    ).sin() * 127.) + 128.) as u8;
            let g = (((i + ((2. * PI) / 3.) ).sin() * 127.) + 128.) as u8;
            let b = (((i + ((4. * PI) / 3.) ).sin() * 127.) + 128.) as u8;

            write!(&mut writer, "\x1B[38;2;{};{};{}m{}", r, g, b, c)?;
        }
        write!(&mut writer, "\n")?;
        
        y += 1;
        buf.clear();
    }
    Ok(())
}
