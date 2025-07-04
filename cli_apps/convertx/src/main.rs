use structopt::StructOpt;


const FEET_IN_METER: f64 = 3.28084;
const INCHES_IN_METER: f64 = 39.3701;
const KG_IN_LB: f64 = 2.20462;
const OZ_IN_KG: f64 = 35.274;
const BPS_IN_MBPS: f64 = 1_000_000.0;
const KELVIN_OFFSET: f64 = 273.15;


#[derive(StructOpt,Debug)]
#[structopt(name="convertx",about="Multi-purpose unit converter")]
enum Cli {
    /// Convert byte values
    Bytes {
        /// Number of bytes
        num:u64,
        /// Convert bytes to megabytes
        #[structopt(short,long)]
        megabytes:bool,
        /// Convert bytes to human-readable
        #[structopt(short = "h", long = "human-readable")]
        human_readable: bool,
    }
}


// Utils fxn
fn bytes_to_mb(num_bytes:u64) -> f64 {
    num_bytes as f64/ (1024.0 * 1024.0)
}

fn bytes_to_human_readable(num_bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut idx = 0;
    let mut n = num_bytes as f64;
    while n >= 1024.0 && idx < units.len() - 1 {
        n /= 1024.0;
        idx += 1;
    }
    format!("{:.2} {}", n, units[idx])
}

fn main() {
    let cli = Cli::from_args();
    match cli {
        Cli::Bytes { num, megabytes, human_readable } =>{
            if megabytes {
                println!("{} bytes = {:.2} MB", num, bytes_to_mb(num));
            } else if human_readable {
                println!("{} bytes = {}", num, bytes_to_human_readable(num));
            } else {
                println!("Please specify --megabytes or --human-readable. See --help.");
            }
        
        }
    }
   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_mb(){
        assert_eq!(bytes_to_mb(1048576), 1.0);
        assert!((bytes_to_mb(2097152) - 2.0).abs() < 1e-8);
    }

     #[test]
    fn test_bytes_to_human_readable() {
        assert_eq!(bytes_to_human_readable(1023), "1023.00 B");
        assert_eq!(bytes_to_human_readable(1024), "1.00 KB");
        assert_eq!(bytes_to_human_readable(1048576), "1.00 MB");
    }

}