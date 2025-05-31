use std::process::exit;
use gtk4::cairo;
use zip::write::SimpleFileOptions;
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::str::FromStr;
use std::vec;
use poppler::Document as PopplerDocument;
use lopdf::{Document, Object, ObjectId, Bookmark};
use colored::Colorize;
use crate::app::gui::app;

pub fn cli_handler(args:&mut Vec<String>){
    let command = args[0].as_str();
    let mut option_enc = false;
    match command{
        "merge" | "m" => {
            println!("Merging the given pdf in the given order ");
            let mut name = "merged_documents.pdf";
            let mut documents = Vec::new();
            let mut doc;
            let mut pdf_name;
            
            for i in 1..args.len(){
                pdf_name = &args[i];
                if option_enc{
                    option_enc = false;
                    continue;
                }
                if pdf_name == "-o"{
                    name = args[i+1].as_str();
                    option_enc = true;
                    continue;
                }
                doc = match Document::load(pdf_name.clone()).as_mut(){
                    Ok(e) =>e,
                    Err(err) => {println!("Error while trying to load the pdf {pdf_name}:\n {err}");exit(1)},
                }.clone();
                documents.push(doc);
            }
            let _ = merge(documents,name);
        },
        "help" |"h" |"?" => help(args),
        "compress" =>{let _ = compress(args);},
        "get" | "g" => {let _ = get_page(args);},
        "delete" |"del" |"d" =>{let _ = del_page(args);},
        "split" |"s" => {let _ = split(args,false);},
        "app" => app(),
        "reorganize" |"swap"|"sw" |"reorg" |"ro" => {let _ = reorganize(args);},
        _ => print!("No command was recognized. type yapm help to get all the commands")
    }
}


pub fn split(args:&mut Vec<String>,gui:bool) -> Result<(),String>{
    //Doc req
    if !gui{
            
        let document;
        let _pdf_name;
        let mut files:Vec<String> = Vec::new();
        (document,_pdf_name) = load_doc_pop(args);

        let n_pages = document.n_pages();
        for page_nb in 0..n_pages {
                let path = format!("page_{}.pdf", page_nb + 1);
                files.push(path.clone());
                let surface = cairo::PdfSurface::new(595.0, 842.0, path)
                    .expect("Erreur surface");

                let cr = cairo::Context::new(&surface).unwrap();
                let page = document.page(page_nb).expect("Erreur page");
                page.render_for_printing(&cr);
                cr.show_page().expect("Erreur show_page");
                surface.finish();
            }
        

        let mut name = "splitted_document.zip".to_string();
        for i in 0..args.len(){
            if args[i] == "-o"{
                if i +1 > args.len() - 1{
                    println!("Missing the argument after -o");
                    exit(1);
                }
                name = args[i+1].clone();
                if ! name.contains(".zip"){
                    name = format!("{name}.zip");
                }
            }
        }
        let archive_path = name.as_str();

        let archive = PathBuf::from_str(archive_path).unwrap();
        let existing_zip = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(archive)
            .unwrap();

        let mut append_zip = zip::ZipWriter::new(existing_zip);
        let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

        for file in &files {
            append_zip
                .start_file(PathBuf::from(file).to_string_lossy(), options)
                .unwrap();

            let mut f = File::open(file).unwrap();
            let _ = std::io::copy(&mut f, &mut append_zip);
        }

        delete_files(files);

        match append_zip.finish() {
                Ok(_)=> return Ok(()),
                Err(err) => return Err(err.to_string()),
        }
            
    } else {
        //args is the list of path to pdf to split + the name of the archive if given
        let mut files:Vec<String> = Vec::new();
        
        //Depending on if we have multiple files to split we create sub dir in our tmp
        if args.len() == 2 { //only one file to split we do not create a sub dir 
            let zip_name = args.remove(0);
             //getting the pdf name
            let pdf_name = { 
                let tmp:Vec<&str> = args[0].split("/").collect();  
                let pdf_name = tmp[tmp.len() -1 ];
                pdf_name.replace(".pdf", "")
            };
            let doc = PopplerDocument::from_file(&format!("file://{}", args[0]), Some(""))
                .expect("Impossible d'ouvrir le PDF");

            let n_pages = doc.n_pages();
            for page_nb in 0..n_pages {
                let path = format!("/usr/share/yapm/tmp/{pdf_name}_page_{}.pdf", page_nb + 1);
                files.push(path.clone());
                let surface = cairo::PdfSurface::new(595.0, 842.0, path)
                    .expect("Erreur surface");

                let cr = cairo::Context::new(&surface).unwrap();
                let page = doc.page(page_nb).expect("Erreur page");
                page.render_for_printing(&cr);
                cr.show_page().expect("Erreur show_page");
                surface.finish();
                }
        
                
            let archive = PathBuf::from_str(&zip_name).unwrap();
            let existing_zip = match OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(archive)
                {
                    Ok(zip) => zip,
                    Err(err) => return Err(err.to_string()),
                };

            let mut append_zip = zip::ZipWriter::new(existing_zip);
            let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

            for file in &files {
                //extract the name only 
                let file_name:String = {
                    let tmp:Vec<&str> = file.split("/").collect();
                    tmp[tmp.len() -1 ].to_string()
                };
                append_zip
                    .start_file(PathBuf::from(file_name).to_string_lossy(), options)
                    .unwrap();

                let mut f = File::open(file).unwrap();
                let _ = std::io::copy(&mut f, &mut append_zip);
            }

            delete_files(files);

            match append_zip.finish() {
                Ok(_)=> return Ok(()),
                Err(err) => return Err(err.to_string()),
            }
            
        } else {// Multiple file to split -> create subdir before extraction + delete dir after zip is done
            let zip_name = args.remove(0);
            let archive = PathBuf::from_str(&zip_name).unwrap();
            let existing_zip = match OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(archive){
                    Ok(zip) => zip,
                    Err(err) => return Err(err.to_string()),
                };
                

            let mut append_zip = zip::ZipWriter::new(existing_zip);
            let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

            for i in  0..args.len(){
                let mut files_dir:Vec<String> = Vec::new();
                //getting the pdf name
                let pdf_name = { 
                    let tmp:Vec<&str> = args[i].split("/").collect();  
                    let pdf_name = tmp[tmp.len() -1 ];
                    pdf_name.replace(".pdf", "")
                };
                let doc = PopplerDocument::from_file(&format!("file://{}", args[i]), Some(""))
                    .expect("Impossible d'ouvrir le PDF");

                let n_pages = doc.n_pages();
                for page_nb in 0..n_pages {
                    let path = format!("/usr/share/yapm/tmp/{pdf_name}_page_{}.pdf", page_nb + 1);
                    files.push(path.clone());
                    files_dir.push(path.clone());
                    let surface = cairo::PdfSurface::new(595.0, 842.0, path)
                        .expect("Erreur surface");

                    let cr = cairo::Context::new(&surface).unwrap();
                    let page = doc.page(page_nb).expect("Erreur page");
                    page.render_for_printing(&cr);
                    cr.show_page().expect("Erreur show_page");
                    surface.finish();
                }
                for file in &files_dir {
                    //extract the name only 
                    let file_name:String = {
                        let tmp:Vec<&str> = file.split("/").collect();
                        format!("{pdf_name}/{}",tmp[tmp.len() -1 ].to_string())
                    };
                    append_zip
                        .start_file(PathBuf::from(file_name).to_string_lossy(), options)
                        .unwrap();

                    let mut f = File::open(file).unwrap();
                    let _ = std::io::copy(&mut f, &mut append_zip);
                }
                files_dir.clear();
            }

            delete_files(files);

            match append_zip.finish() {
                Ok(_)=> return Ok(()),
                Err(err) => return Err(err.to_string()),
            }
            
        }
    }

}

pub fn merge(documents:Vec<Document>,name: &str) -> Result<(),String> {
    // Define a starting `max_id` (will be used as start index for object_ids).
    let mut max_id = 1;
    let mut pagenum = 1;
    // Collect all Documents Objects grouped by a map
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut document = Document::with_version("1.5");

    for mut doc in documents {
        let mut first = false;
        doc.renumber_objects_with(max_id);

        max_id = doc.max_id + 1;

        documents_pages.extend(
            doc
                    .get_pages()
                    .into_iter()
                    .map(|(_, object_id)| {
                        if !first {
                            let bookmark = Bookmark::new(String::from(format!("Page_{}", pagenum)), [0.0, 0.0, 1.0], 0, object_id);
                            document.add_bookmark(bookmark, None);
                            first = true;
                            pagenum += 1;
                        }

                        (
                            object_id,
                            doc.get_object(object_id).unwrap().to_owned(),
                        )
                    })
                    .collect::<BTreeMap<ObjectId, Object>>(),
        );
        documents_objects.extend(doc.objects);
    }

    // "Catalog" and "Pages" are mandatory.
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    // Process all objects except "Page" type
    for (object_id, object) in documents_objects.iter() {
        // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects.
        // All other objects should be collected and inserted into the main Document.
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                // Collect a first "Catalog" object and use it for the future "Pages".
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object {
                        id
                    } else {
                        *object_id
                    },
                    object.clone(),
                ));
            }
            b"Pages" => {
                // Collect and update a first "Pages" object and use it for the future "Catalog"
                // We have also to merge all dictionaries of the old and the new "Pages" object
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref object)) = pages_object {
                        if let Ok(old_dictionary) = object.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }

                    pages_object = Some((
                        if let Some((id, _)) = pages_object {
                            id
                        } else {
                            *object_id
                        },
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            b"Page" => {}     // Ignored, processed later and separately
            b"Outlines" => {} // Ignored, not supported yet
            b"Outline" => {}  // Ignored, not supported yet
            _ => {
                document.objects.insert(*object_id, object.clone());
            }
        }
    }

    // If no "Pages" object found, abort.
    if pages_object.is_none() {
        println!("Pages root not found.");
    }

    // Iterate over all "Page" objects and collect into the parent "Pages" created before
    for (object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", pages_object.as_ref().unwrap().0);

            document
                    .objects
                    .insert(*object_id, Object::Dictionary(dictionary));
        }
    }

    // If no "Catalog" found, abort.
    if catalog_object.is_none() {
        println!("Catalog root not found.");
    }

    let catalog_object = catalog_object.unwrap();
    let pages_object = pages_object.unwrap();

    // Build a new "Pages" with updated fields
    if let Ok(dictionary) = pages_object.1.as_dict() {
        let mut dictionary = dictionary.clone();

        // Set new pages count
        dictionary.set("Count", documents_pages.len() as u32);

        // Set new "Kids" list (collected from documents pages) for "Pages"
        dictionary.set(
            "Kids",
            documents_pages
                    .into_iter()
                    .map(|(object_id, _)| Object::Reference(object_id))
                    .collect::<Vec<_>>(),
        );

        document
                .objects
                .insert(pages_object.0, Object::Dictionary(dictionary));
    }

    // Build a new "Catalog" with updated fields
    if let Ok(dictionary) = catalog_object.1.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", pages_object.0);
        dictionary.remove(b"Outlines"); // Outlines not supported in merged PDFs

        document
                .objects
                .insert(catalog_object.0, Object::Dictionary(dictionary));
    }

    document.trailer.set("Root", catalog_object.0);

    // Update the max internal ID as wasn't updated before due to direct objects insertion
    document.max_id = document.objects.len() as u32;

    // Reorder all new Document objects
    document.renumber_objects();

    // Set any Bookmarks to the First child if they are not set to a page
    document.adjust_zero_pages();

    // Set all bookmarks to the PDF Object tree then set the Outlines to the Bookmark content map.
    if let Some(n) = document.build_outline() {
        if let Ok(Object::Dictionary(dict)) = document.get_object_mut(catalog_object.0) {
            dict.set("Outlines", Object::Reference(n));
        }
    }

    document.compress();

    match document.save(name){
        Ok(_) => Ok(()),
        Err(err)=> Err(err.to_string()),
    }
    

}

pub fn reorganize(args:&mut Vec<String>)-> Result<(),String>{
    let document;
    let pdf_name;
    (document,pdf_name)  = load_doc_lop(args);
    let count = document.get_pages().len();
    let mut documents: Vec<Document> = Vec::new();
    let mut files : Vec<String> = Vec::new();

    if args.len() > 1{
        if args[0] == "-s"{
            if  args.len() != 3{
                println!("Missing arguments after -s\n\t[USAGE] yapm reorganize -s <page one> <page two> <pdf file>");
            }
            else {
                //Extract every pages
                for i in 1..=count{
                    let mut  tmp:Vec<u32> = Vec::new();
                    let mut doc = document.clone();
                    for j in 1..=count{
                        if j!=i {
                            tmp.push(j as u32)
                        }
                    }
                    doc.delete_pages(&tmp);
                    let name = format!("tmp_{i}_{}",pdf_name);
                    doc.save(&name).unwrap();
                    documents.push(Document::load(&name).unwrap());
                    files.push(name);
                    tmp.clear();
                }

                documents.swap(args[1].parse::<usize>().unwrap() -1 , args[2].parse::<usize>().unwrap() -1 );
                let name = format!("swapped_{pdf_name}");
                let _ = merge(documents, name.as_str());
                delete_files(files);
            }
            
            
        }
    }
    //Option -s 1 2 => swap page 1 and 2 / By default give the full possibility 
    //OR full swap ...

    Ok(())
}

pub fn del_page(args:&mut Vec<String>)-> Result<(),String>{
    let  doc;
    let  pdf_name;
    (doc,pdf_name )= load_doc_pop(args);
    let mut pages_to_del:Vec<i32> = vec![];
    for i in 0..args.len(){
        pages_to_del.push(args[i].parse::<i32>().unwrap())
    }

    let total = doc.n_pages();
    let mut files:Vec<String> = Vec::new();

    for i in 0..total {
        let page_number = i + 1;
        if pages_to_del.contains(&page_number) {
            continue;
        }

        let filename = format!("page_{}.pdf", page_number);
        files.push(filename.clone());
        let surface = cairo::PdfSurface::new(595.0, 842.0, &filename).unwrap();
        let cr = cairo::Context::new(&surface).unwrap();
        let page = doc.page(i ).unwrap();
        page.render_for_printing(&cr);

        match cr.show_page() {
            Ok(_) => (),
            Err(err) => return Err(err.to_string()),
        };
        surface.finish();
    }

    let mut documents:Vec<Document> = Vec::new();
    for file in &files{
        documents.push(Document::load(file).unwrap())
    }

    let name = format!("modified_{}",pdf_name);
    delete_files(files);
    match merge(documents, &name){
        Ok(_) => Ok(()),
        Err(err)=> Err(err),
    }

}

pub fn get_page(args:&mut Vec<String>)-> Result<(),String>{
    let  document;
    let  pdf_name;
    (document,pdf_name)  = load_doc_pop(args);
    let count = document.n_pages();
    let page_to_get = args[0].parse::<i32>().unwrap();
    if page_to_get > count {
        println!("The {} page is out of range. The pdf file have {} page(s)",page_to_get,count);
        exit(1);
    }
    

    let filename =  format!("Page_{page_to_get}_{}",pdf_name);
    let surface = match cairo::PdfSurface::new(595.0, 842.0, &filename){
        Ok(surface) => surface,
        Err(err) => return Err(err.to_string()),
    };
    let cr = match cairo::Context::new(&surface){
        Ok(cr) => cr,
        Err(err) => return Err(err.to_string()),
    };
    let page  = match  document.page(page_to_get -1 ){
        Some(e) => e,
        None => return  Err("The page was not found in the given pdf".to_string()),
    };
    page.render_for_printing(&cr);

    match cr.show_page(){
        Ok(_) => (),
        Err(err) => return Err(err.to_string()),
    };
    surface.finish();
   
   Ok(())
}

fn compress( args: &mut Vec<String>) -> Result<(),String>{
    let mut doc ;
    let _pdf_name;
    (doc,_pdf_name)= load_doc_lop(args);
    doc.compress();
    match  doc.save("compressed.pdf") {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }

}


fn help(args:&mut Vec<String>){
    if args.len() == 1 {
        compress_help(false);
        merge_help(false);
        get_help(false);
        del_help(false);
        split_help(false);

    } else if args.len() == 2 {
        let arg = args[1].as_str();
        match arg{
            "compress" => compress_help(true),
            "merge" => merge_help(true),
            "get" => get_help(true),
            "delete" => del_help(true),
            "split" => split_help(true),
            _ => println!("The given command was not recognize.\n[USAGE] yapm {} (<cmd>)","help".blue()),
        }
    } else {
        println!("Too many arguments for help\n[USAGE] yapm {} (<cmd>)","help".blue())
    }
    
}

pub fn compress_help(full:bool){
    if full{
        println!("- {} : compress the given pdf file (Do not expect a good compression)\n\t[USAGE] yapm compress <pdf file>","compress".blue());
    }else {
        println!("- {} : compress the given pdf file","compress".blue());
    }
}

fn merge_help(full:bool){
    if full{
        println!("- {} : merge the given pdf file in the order you submitted them\n\t[USAGE] yapm merge <pdffile1> ... <pdffileN>\n\t\tPotential options: -o output name","merge".blue());
    }else {
        println!("- {} : merge the given pdf files","merge".blue());
    }
}

fn get_help(full:bool){
    if full{
        println!("- {} : get the n page of the given pdf file\n\t[USAGE] yapm get <pdf file> <page>","get".blue());
    }else {
        println!("- {} : get the n page of the given pdf file","get".blue());
    }
}

fn del_help(full:bool){
    if full{
        println!("- {} : del the n page of the given pdf file\n\t[USAGE] yapm del <pdf file> <page number 1> .. <page number n>","delete".blue());
    }else {
        println!("- {} : del the n page of the given pdf file","delete".blue());
    }
}

fn split_help(full:bool){
    if full{
        println!("- {} : split the given pdf file into a zip of the pages\n\t[USAGE] yapm split <pdf file>\n\t\tPotential options: -o output name","split".blue());
    }else {
        println!("- {} : split the given pdf file into a zip of the pages","split".blue());
    }
}

fn load_doc_pop(args: &mut Vec<String>) ->( PopplerDocument,String){
    //Removing the command from the arfs list 
    args.remove(0);
    let mut index = 0;
    let mut name: String = String::new();
    //Found the .pdf file in the list of the arguments
    for i in 0..args.len(){
        if args[i].contains(".pdf"){
            name = args[i].clone();
            index = i;
        }
    }
    let pdf_name = args.remove(index);
    let binding = std::env::current_dir().unwrap();
    let working_dir = binding.to_str().unwrap();
    let document = PopplerDocument::from_file(&format!("file://{}/{}",working_dir, name), Some(""));
        
    match document{
        Ok(doc) => (doc,pdf_name),
        Err(err) => {println!("Error when trying to load the pdf {}\n{}",&name.blue(),err); exit(1)}
    }
}

fn load_doc_lop(args: &mut Vec<String>) ->( Document,String){
    //Removing the command from the arfs list 
    args.remove(0);
    let mut index = 0;
    let mut name: String = String::new();
    //Found the .pdf file in the list of the arguments
    for i in 0..args.len(){
        if args[i].contains(".pdf"){
            name = args[i].clone();
            index = i;
        }
    }
    let pdf_name = args.remove(index);
    let document = Document::load(&name);
    match document{
        Ok(doc) => (doc,pdf_name),
        Err(err) => {println!("Error when trying to load the pdf {}\n{}",&name.blue(),err); exit(1)}
    }

}

fn delete_files(files: Vec<String>){
    for file in files{
       let _ =  fs::remove_file(file);
    }
}