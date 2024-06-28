use std::{env::args, fs::{self, create_dir_all, File}, io::copy, path::Path};

use zip::ZipArchive;

fn main() {
    std::process::exit(unzipper());
}

fn unzipper()->i32{
    let args:Vec<_>=args().collect();

    if args.len()<2{
        println!("Please provide the zip file.");
        return 1;
    }
    let file_name=Path::new(&args[1]);
    let file=File::open(&file_name).unwrap();


    let mut archive=ZipArchive::new(file).unwrap();

    for i in 0..archive.len(){
        let mut file=archive.by_index(i).unwrap();
        let out_path=match file.enclosed_name() {
            Some(path)=>path.to_owned(),
            None=>continue
        };

        {
            let comment=file.comment();
            if !comment.is_empty(){
                println!("File {} comment: {}",i,comment);
            }
        }
        if (*file.name()).ends_with('/'){
            println!("File {} Extracted to \"{}\"",i,out_path.display());
            create_dir_all(&out_path).unwrap();
        }
        else{
            println!(
                "File {} extracted to \"{:?}\" ({} bytes)",
                i,out_path.display(),file.size()
            );
            if let Some(p)=out_path.parent(){
                if !p.exists(){
                    create_dir_all(&p).unwrap();
                }
            }
            let mut outfile=File::create(&out_path).unwrap();
            copy(&mut file, &mut outfile).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode)=file.unix_mode(){
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0
}
